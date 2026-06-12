//! Remote monitoring via SSH.
//!
//! Captures [`Snapshot`]s from a remote host by running `syslenz --export -`
//! over SSH. The remote syslenz instance serializes a snapshot as JSON to
//! stdout; we deserialize it here.
//!
//! If the remote host does not have syslenz installed the functions return an
//! error with a descriptive message suggesting installation.

use crate::proc::Snapshot;

use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// ---------------------------------------------------------------------------
// BL-072: SSH ControlMaster support
// ---------------------------------------------------------------------------

/// Return the directory used for SSH ControlMaster sockets.
/// Creates it if missing (mode 0700 for security).
fn control_socket_dir() -> PathBuf {
    let dir = if let Ok(runtime) = std::env::var("XDG_RUNTIME_DIR") {
        PathBuf::from(runtime).join("syslenz-ssh")
    } else if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".cache").join("syslenz-ssh")
    } else {
        PathBuf::from("/tmp/syslenz-ssh")
    };
    if !dir.exists() {
        let _ = std::fs::create_dir_all(&dir);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700));
        }
    }
    dir
}

/// Return the ControlPath for a given host. The socket is placed under
/// [`control_socket_dir`] using the host string (sanitized).
fn control_path_for(host: &str) -> PathBuf {
    let sanitized: String = host
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '.' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect();
    control_socket_dir().join(format!("ctl-{}", sanitized))
}

/// Build common SSH arguments that enable ControlMaster socket reuse.
fn ssh_control_args(host: &str) -> Vec<String> {
    let ctl = control_path_for(host);
    let ctl_str = ctl.to_string_lossy().to_string();
    vec![
        "-o".to_string(),
        format!("ControlPath={}", ctl_str),
        "-o".to_string(),
        "ControlMaster=auto".to_string(),
        "-o".to_string(),
        "ControlPersist=60".to_string(),
    ]
}

/// Capture a single [`Snapshot`] from a remote host via SSH.
///
/// Runs `syslenz --export -` on `host` (where `-` means "write JSON to
/// stdout").  The SSH connection inherits the caller's SSH agent / config so
/// key-based auth works transparently.
///
/// # Errors
///
/// Returns an error if:
/// - The SSH process cannot be spawned (e.g. `ssh` not on `PATH`).
/// - The remote command exits with a non-zero status (most likely syslenz is
///   not installed).
/// - The output cannot be parsed as a valid `Snapshot`.
pub fn capture_remote(host: &str) -> anyhow::Result<Snapshot> {
    let ctl_args = ssh_control_args(host);
    let mut cmd = Command::new("ssh");
    cmd
        // Disable pseudo-TTY allocation — we only want stdout.
        .arg("-T")
        // Batch mode: never prompt for passwords (rely on keys / agent).
        .arg("-o")
        .arg("BatchMode=yes")
        // Fail fast on connection issues.
        .arg("-o")
        .arg("ConnectTimeout=10");
    // BL-072: ControlMaster args for persistent connections
    for arg in &ctl_args {
        cmd.arg(arg);
    }
    let mut child = cmd
        .arg(host)
        .arg("syslenz")
        .arg("--export")
        .arg("-")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| anyhow::anyhow!("failed to spawn ssh process: {e}"))?;

    let mut stdout_buf = String::new();
    if let Some(ref mut stdout) = child.stdout {
        stdout.read_to_string(&mut stdout_buf)?;
    }

    let status = child.wait()?;

    if !status.success() {
        let mut stderr_buf = String::new();
        if let Some(ref mut stderr) = child.stderr {
            stderr.read_to_string(&mut stderr_buf)?;
        }

        // Provide a helpful message depending on the exit code.
        let hint = if stderr_buf.contains("not found")
            || stderr_buf.contains("No such file")
            || stderr_buf.contains("command not found")
        {
            format!(
                "It looks like syslenz is not installed on '{host}'. \
                 Install it on the remote machine and make sure it is on the PATH."
            )
        } else {
            format!(
                "SSH command failed on '{host}' (exit {}): {}",
                status.code().unwrap_or(-1),
                stderr_buf.trim()
            )
        };

        anyhow::bail!(hint);
    }

    let mut snapshot: Snapshot = serde_json::from_str(&stdout_buf).map_err(|e| {
        anyhow::anyhow!(
            "failed to parse snapshot JSON from '{host}': {e}\n\
             (first 200 bytes of output: {:?})",
            &stdout_buf[..stdout_buf.len().min(200)]
        )
    })?;
    snapshot.sanitize();

    Ok(snapshot)
}

/// Stream [`Snapshot`]s from a remote host at a fixed interval.
///
/// Spawns a background thread that calls [`capture_remote`] every
/// `interval_ms` milliseconds and sends each snapshot through the returned
/// channel.  The thread exits when the receiver is dropped or when an
/// unrecoverable error occurs (the error is sent as the thread's join result).
///
/// Transient failures (e.g. a single SSH timeout) are silently skipped so that
/// the stream is resilient to momentary network blips.  After
/// `MAX_CONSECUTIVE_FAILURES` consecutive failures the thread gives up and
/// exits.
pub fn stream_remote(host: &str, interval_ms: u64) -> anyhow::Result<mpsc::Receiver<Snapshot>> {
    const MAX_CONSECUTIVE_FAILURES: u32 = 5;

    let host = host.to_owned();
    let (tx, rx) = mpsc::channel();

    thread::Builder::new()
        .name(format!("remote-stream-{host}"))
        .spawn(move || {
            let interval = Duration::from_millis(interval_ms);
            let mut consecutive_failures: u32 = 0;

            loop {
                match capture_remote(&host) {
                    Ok(snapshot) => {
                        consecutive_failures = 0;
                        // If the receiver has been dropped, stop streaming.
                        if tx.send(snapshot).is_err() {
                            break;
                        }
                    }
                    Err(_) => {
                        consecutive_failures += 1;
                        if consecutive_failures >= MAX_CONSECUTIVE_FAILURES {
                            // Too many failures in a row — give up.
                            break;
                        }
                    }
                }

                thread::sleep(interval);
            }
        })?;

    Ok(rx)
}

/// Capture a single [`Snapshot`] from a Docker container via `docker exec`.
///
/// The container must have syslenz installed. Runs
/// `docker exec <container> syslenz --export -`.
pub fn capture_docker(container: &str) -> anyhow::Result<Snapshot> {
    let mut child = Command::new("docker")
        .arg("exec")
        .arg(container)
        .arg("syslenz")
        .arg("--export")
        .arg("-")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| anyhow::anyhow!("failed to run docker exec: {e}"))?;

    let mut stdout_buf = String::new();
    if let Some(ref mut stdout) = child.stdout {
        stdout.read_to_string(&mut stdout_buf)?;
    }

    let status = child.wait()?;

    if !status.success() {
        let mut stderr_buf = String::new();
        if let Some(ref mut stderr) = child.stderr {
            stderr.read_to_string(&mut stderr_buf)?;
        }

        let hint = if stderr_buf.contains("not found") || stderr_buf.contains("No such file") {
            format!(
                "syslenz is not installed in container '{container}'. \
                 Install it: docker exec {container} sh -c 'curl -L <url> -o /usr/local/bin/syslenz && chmod +x /usr/local/bin/syslenz'"
            )
        } else if stderr_buf.contains("No such container")
            || stderr_buf.contains("no such container")
        {
            format!("Container '{container}' not found. Check `docker ps`.")
        } else {
            format!(
                "docker exec failed on '{container}' (exit {}): {}",
                status.code().unwrap_or(-1),
                stderr_buf.trim()
            )
        };
        anyhow::bail!(hint);
    }

    let mut snapshot: Snapshot = serde_json::from_str(&stdout_buf).map_err(|e| {
        anyhow::anyhow!(
            "failed to parse snapshot JSON from container '{container}': {e}\n\
             (first 200 bytes: {:?})",
            &stdout_buf[..stdout_buf.len().min(200)]
        )
    })?;
    snapshot.sanitize();

    Ok(snapshot)
}

/// Stream [`Snapshot`]s from a Docker container at a fixed interval.
pub fn stream_docker(
    container: &str,
    interval_ms: u64,
) -> anyhow::Result<mpsc::Receiver<Snapshot>> {
    const MAX_CONSECUTIVE_FAILURES: u32 = 5;

    let container = container.to_owned();
    let (tx, rx) = mpsc::channel();

    thread::Builder::new()
        .name(format!("docker-stream-{container}"))
        .spawn(move || {
            let interval = Duration::from_millis(interval_ms);
            let mut consecutive_failures: u32 = 0;

            loop {
                match capture_docker(&container) {
                    Ok(snapshot) => {
                        consecutive_failures = 0;
                        if tx.send(snapshot).is_err() {
                            break;
                        }
                    }
                    Err(_) => {
                        consecutive_failures += 1;
                        if consecutive_failures >= MAX_CONSECUTIVE_FAILURES {
                            break;
                        }
                    }
                }
                thread::sleep(interval);
            }
        })?;

    Ok(rx)
}

/// Capture a snapshot from a syslenz TCP server (`--serve` mode).
pub fn capture_tcp(addr: &str) -> anyhow::Result<Snapshot> {
    use std::io::Write;
    use std::net::TcpStream;

    let mut stream = TcpStream::connect(addr)
        .map_err(|e| anyhow::anyhow!("failed to connect to syslenz server at {addr}: {e}"))?;
    stream.set_read_timeout(Some(Duration::from_secs(10)))?;

    // Send request
    stream.write_all(b"SNAPSHOT\n")?;

    // Read JSON response
    let mut buf = String::new();
    stream.read_to_string(&mut buf)?;

    let mut snapshot: Snapshot = serde_json::from_str(&buf)
        .map_err(|e| anyhow::anyhow!("failed to parse snapshot from {addr}: {e}"))?;
    snapshot.sanitize();

    Ok(snapshot)
}

/// Stream snapshots from a syslenz TCP server.
pub fn stream_tcp(addr: &str, interval_ms: u64) -> anyhow::Result<mpsc::Receiver<Snapshot>> {
    const MAX_CONSECUTIVE_FAILURES: u32 = 5;

    let addr = addr.to_owned();
    let (tx, rx) = mpsc::channel();

    thread::Builder::new()
        .name(format!("tcp-stream-{addr}"))
        .spawn(move || {
            let interval = Duration::from_millis(interval_ms);
            let mut consecutive_failures: u32 = 0;

            loop {
                match capture_tcp(&addr) {
                    Ok(snapshot) => {
                        consecutive_failures = 0;
                        if tx.send(snapshot).is_err() {
                            break;
                        }
                    }
                    Err(_) => {
                        consecutive_failures += 1;
                        if consecutive_failures >= MAX_CONSECUTIVE_FAILURES {
                            break;
                        }
                    }
                }
                thread::sleep(interval);
            }
        })?;

    Ok(rx)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Smoke-test: calling capture_remote with an invalid host should return an
    /// error rather than panicking.
    #[test]
    fn capture_remote_bad_host_returns_error() {
        let result = capture_remote("__nonexistent_host_for_test__");
        assert!(result.is_err());
    }
}

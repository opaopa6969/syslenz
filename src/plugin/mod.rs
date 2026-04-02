use crate::proc::ProcEntry;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::Duration;

/// Discover and run all plugins, returning their entries keyed as "plugin/<name>".
pub fn load_plugins() -> BTreeMap<String, ProcEntry> {
    let mut entries = BTreeMap::new();
    let plugin_dir = plugin_directory();

    // Create the plugins directory if it doesn't exist
    if !plugin_dir.exists() {
        let _ = std::fs::create_dir_all(&plugin_dir);
    }

    let read_dir = match std::fs::read_dir(&plugin_dir) {
        Ok(rd) => rd,
        Err(_) => return entries,
    };

    for dir_entry in read_dir.flatten() {
        let path = dir_entry.path();
        if !path.is_file() {
            continue;
        }

        // Skip non-executable files on unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(meta) = path.metadata() {
                if meta.permissions().mode() & 0o111 == 0 {
                    continue;
                }
            } else {
                continue;
            }
        }

        match run_plugin(&path) {
            Ok((key, entry)) => {
                entries.insert(key, entry);
            }
            Err(e) => {
                eprintln!(
                    "[syslenz] plugin {:?} skipped: {}",
                    path.file_name().unwrap_or_default(),
                    e
                );
            }
        }
    }

    entries
}

fn plugin_directory() -> PathBuf {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(xdg).join("syslenz").join("plugins")
    } else if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home)
            .join(".config")
            .join("syslenz")
            .join("plugins")
    } else {
        PathBuf::from("/tmp/syslenz/plugins")
    }
}

fn run_plugin(path: &std::path::Path) -> anyhow::Result<(String, ProcEntry)> {
    use std::process::{Command, Stdio};

    let child = Command::new(path)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Timeout: 5 seconds
    let output = wait_with_timeout(child, Duration::from_secs(5))?;

    if !output.status.success() {
        anyhow::bail!("exited with status {}", output.status);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let entry: ProcEntry = serde_json::from_str(&stdout)?;

    // Use filename (without extension) as entry key, prefixed with "plugin/"
    let name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");
    let key = format!("plugin/{}", name);

    Ok((key, entry))
}

/// Wait for a child process with a timeout. Kills the process if it exceeds the deadline.
fn wait_with_timeout(
    mut child: std::process::Child,
    timeout: Duration,
) -> anyhow::Result<std::process::Output> {
    let start = std::time::Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                // Process exited, collect output
                let stdout = child
                    .stdout
                    .take()
                    .map(|mut s| {
                        let mut buf = Vec::new();
                        std::io::Read::read_to_end(&mut s, &mut buf).unwrap_or(0);
                        buf
                    })
                    .unwrap_or_default();
                let stderr = child
                    .stderr
                    .take()
                    .map(|mut s| {
                        let mut buf = Vec::new();
                        std::io::Read::read_to_end(&mut s, &mut buf).unwrap_or(0);
                        buf
                    })
                    .unwrap_or_default();
                return Ok(std::process::Output {
                    status,
                    stdout,
                    stderr,
                });
            }
            Ok(None) => {
                // Still running
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    anyhow::bail!("plugin timed out after {:?}", timeout);
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(e) => {
                anyhow::bail!("failed to wait on plugin: {}", e);
            }
        }
    }
}

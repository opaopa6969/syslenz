//! TCP server mode for syslenz.
//!
//! Runs a lightweight TCP server that responds to `SNAPSHOT\n` requests
//! with JSON snapshots. Designed for running inside Docker containers
//! where SSH is not available.
//!
//! Usage: `syslenz --serve [bind_addr]` (default: `127.0.0.1:9100`)

use crate::proc::Snapshot;
use crate::prometheus;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

/// Run the TCP snapshot server.
pub fn run_server(bind_addr: &str) -> anyhow::Result<()> {
    let listener = TcpListener::bind(bind_addr)?;
    eprintln!("syslenz: serving on {}", bind_addr);
    eprintln!("Connect with: syslenz --connect {}", bind_addr);

    serve_connections(listener);

    Ok(())
}

/// Accept connections and hand each one to its own thread so a slow or
/// idle client (e.g. a long-lived `--connect` TUI session) cannot stall
/// requests from other clients. Each connection is independently
/// stateless (`Snapshot::capture()` per request), so no shared state
/// needs synchronization.
fn serve_connections(listener: TcpListener) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || handle_connection(stream));
            }
            Err(e) => {
                eprintln!("  accept error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let peer = stream.peer_addr().ok();
    let reader = match stream.try_clone() {
        Ok(s) => BufReader::new(s),
        Err(_) => return,
    };

    for line in reader.lines() {
        match line {
            Ok(cmd) => {
                let cmd = cmd.trim();
                if cmd == "SNAPSHOT" || cmd == "METRICS" {
                    match Snapshot::capture() {
                        Ok(snap) => {
                            if cmd == "METRICS" {
                                let prom = prometheus::format_prometheus(&snap);
                                let _ = stream.write_all(prom.as_bytes());
                            } else {
                                let json = serde_json::to_string(&snap)
                                    .unwrap_or_else(|_| "{}".to_string());
                                let _ = stream.write_all(json.as_bytes());
                                let _ = stream.write_all(b"\n");
                            }
                            let _ = stream.flush();
                        }
                        Err(e) => {
                            let err = format!("{{\"error\":\"{}\"}}\n", e);
                            let _ = stream.write_all(err.as_bytes());
                        }
                    }
                    break; // One request per connection (simple protocol)
                } else if cmd == "QUIT" || cmd.is_empty() {
                    break;
                }
            }
            Err(_) => break,
        }
    }

    if let Some(peer) = peer {
        eprintln!("  client disconnected: {}", peer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use std::time::Duration;

    /// Regression test for the single-threaded accept loop: a client that
    /// connects without ever sending a command (or closing) used to block
    /// the server from servicing any other client, since connections were
    /// handled one at a time inline in the accept loop.
    #[test]
    fn slow_client_does_not_block_other_clients() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let addr = listener.local_addr().expect("local_addr");
        std::thread::spawn(move || serve_connections(listener));

        // Slow/idle client: connects, sends nothing, never closes.
        let _slow = TcpStream::connect(addr).expect("slow client connect");

        // A second client should still get a prompt response.
        let mut fast = TcpStream::connect(addr).expect("fast client connect");
        fast.set_read_timeout(Some(Duration::from_secs(5)))
            .expect("set_read_timeout");
        fast.write_all(b"SNAPSHOT\n").expect("write SNAPSHOT");

        let mut buf = [0u8; 1024];
        let n = fast
            .read(&mut buf)
            .expect("fast client should receive a response promptly");
        assert!(n > 0, "expected a non-empty snapshot response");
        assert!(
            String::from_utf8_lossy(&buf[..n]).contains('{'),
            "expected JSON snapshot response"
        );
    }
}

//! TCP server mode for syslenz.
//!
//! Runs a lightweight TCP server that responds to `SNAPSHOT\n` requests
//! with JSON snapshots. Designed for running inside Docker containers
//! where SSH is not available.
//!
//! Usage: `syslenz --serve [bind_addr]` (default: `0.0.0.0:9100`)

use crate::proc::Snapshot;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

/// Run the TCP snapshot server.
pub fn run_server(bind_addr: &str) -> anyhow::Result<()> {
    let listener = TcpListener::bind(bind_addr)?;
    eprintln!("syslenz: serving on {}", bind_addr);
    eprintln!("Connect with: syslenz --connect {}", bind_addr);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // Handle each connection in the same thread (simple & lightweight)
                let peer = stream.peer_addr().ok();
                let reader = BufReader::new(stream.try_clone()?);

                for line in reader.lines() {
                    match line {
                        Ok(cmd) => {
                            let cmd = cmd.trim();
                            if cmd == "SNAPSHOT" {
                                match Snapshot::capture() {
                                    Ok(snap) => {
                                        let json = serde_json::to_string(&snap)
                                            .unwrap_or_else(|_| "{}".to_string());
                                        let _ = stream.write_all(json.as_bytes());
                                        let _ = stream.write_all(b"\n");
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
            Err(e) => {
                eprintln!("  accept error: {}", e);
            }
        }
    }

    Ok(())
}

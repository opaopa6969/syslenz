//! Prometheus metrics export.
//!
//! Formats all numeric fields from a [`Snapshot`] in Prometheus text exposition
//! format, and optionally serves them over HTTP on a `/metrics` endpoint.

use crate::proc::{FieldValue, Snapshot};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

/// Sanitize a name component for use in a Prometheus metric name.
/// Replaces `/` and `-` with `_`.
fn sanitize(s: &str) -> String {
    s.replace('/', "_").replace('-', "_")
}

/// Format all numeric fields from a snapshot in Prometheus text exposition format.
pub fn format_prometheus(snapshot: &Snapshot) -> String {
    let mut out = String::new();

    for (source_key, entry) in &snapshot.entries {
        let source = sanitize(source_key);

        for field in &entry.fields {
            let value = match &field.value {
                FieldValue::Bytes(b) => Some(format!("{}", b)),
                FieldValue::Integer(i) => Some(format!("{}", i)),
                FieldValue::Float(f) => Some(format!("{}", f)),
                FieldValue::Duration(secs) => Some(format!("{}", secs)),
                FieldValue::Text(_) | FieldValue::Table(_) => None,
            };

            if let Some(val) = value {
                let metric_name = format!("syslenz_{}_{}", source, sanitize(&field.name));
                out.push_str(&format!("# HELP {} {}\n", metric_name, field.description));
                out.push_str(&format!("# TYPE {} gauge\n", metric_name));
                out.push_str(&format!("{} {}\n", metric_name, val));
            }
        }
    }

    out
}

/// Run a standalone HTTP server that serves Prometheus metrics on `/metrics`.
///
/// This is a minimal HTTP server using only `std::net::TcpListener`.
/// It captures a fresh snapshot for every scrape request.
pub fn run_prometheus_server(bind_addr: &str) -> anyhow::Result<()> {
    let listener = TcpListener::bind(bind_addr)?;
    eprintln!(
        "syslenz: Prometheus metrics endpoint on http://{}/metrics",
        bind_addr
    );

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let reader = BufReader::new(stream.try_clone()?);

                // Read the HTTP request line
                let mut is_metrics_request = false;
                for line in reader.lines() {
                    match line {
                        Ok(line) => {
                            if line.starts_with("GET /metrics") {
                                is_metrics_request = true;
                            }
                            // Empty line signals end of HTTP headers
                            if line.is_empty() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }

                if is_metrics_request {
                    match Snapshot::capture() {
                        Ok(snap) => {
                            let body = format_prometheus(&snap);
                            let response = format!(
                                "HTTP/1.1 200 OK\r\n\
                                 Content-Type: text/plain; version=0.0.4; charset=utf-8\r\n\
                                 Content-Length: {}\r\n\
                                 Connection: close\r\n\
                                 \r\n\
                                 {}",
                                body.len(),
                                body
                            );
                            let _ = stream.write_all(response.as_bytes());
                        }
                        Err(e) => {
                            let body = format!("# error capturing snapshot: {}\n", e);
                            let response = format!(
                                "HTTP/1.1 500 Internal Server Error\r\n\
                                 Content-Type: text/plain\r\n\
                                 Content-Length: {}\r\n\
                                 Connection: close\r\n\
                                 \r\n\
                                 {}",
                                body.len(),
                                body
                            );
                            let _ = stream.write_all(response.as_bytes());
                        }
                    }
                } else {
                    let body = "404 Not Found\n";
                    let response = format!(
                        "HTTP/1.1 404 Not Found\r\n\
                         Content-Type: text/plain\r\n\
                         Content-Length: {}\r\n\
                         Connection: close\r\n\
                         \r\n\
                         {}",
                        body.len(),
                        body
                    );
                    let _ = stream.write_all(response.as_bytes());
                }

                let _ = stream.flush();
            }
            Err(e) => {
                eprintln!("  accept error: {}", e);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proc::{Field, FieldValue, ProcEntry, Snapshot};
    use std::collections::BTreeMap;
    use std::time::SystemTime;

    fn make_snapshot(fields: Vec<(&str, &str, FieldValue, &str)>) -> Snapshot {
        let mut entries = BTreeMap::new();
        for (source_key, field_name, value, desc) in fields {
            let entry = entries
                .entry(source_key.to_string())
                .or_insert_with(|| ProcEntry {
                    source: format!("/proc/{}", source_key),
                    fields: Vec::new(),
                });
            entry.fields.push(Field {
                name: field_name.to_string(),
                value,
                unit: None,
                description: desc.to_string(),
            });
        }
        Snapshot {
            timestamp: SystemTime::now(),
            entries,
        }
    }

    #[test]
    fn format_bytes_field() {
        let snap = make_snapshot(vec![(
            "meminfo",
            "MemTotal",
            FieldValue::Bytes(67100098560),
            "Total usable RAM",
        )]);
        let output = format_prometheus(&snap);
        assert!(output.contains("# HELP syslenz_meminfo_MemTotal Total usable RAM"));
        assert!(output.contains("# TYPE syslenz_meminfo_MemTotal gauge"));
        assert!(output.contains("syslenz_meminfo_MemTotal 67100098560"));
    }

    #[test]
    fn format_float_field() {
        let snap = make_snapshot(vec![(
            "loadavg",
            "load_1min",
            FieldValue::Float(2.11),
            "1-minute load average",
        )]);
        let output = format_prometheus(&snap);
        assert!(output.contains("syslenz_loadavg_load_1min 2.11"));
    }

    #[test]
    fn format_duration_field() {
        let snap = make_snapshot(vec![(
            "uptime",
            "uptime",
            FieldValue::Duration(86400.5),
            "System uptime",
        )]);
        let output = format_prometheus(&snap);
        assert!(output.contains("syslenz_uptime_uptime 86400.5"));
    }

    #[test]
    fn skips_text_and_table() {
        let snap = make_snapshot(vec![
            (
                "version",
                "version_string",
                FieldValue::Text("Linux 6.1".into()),
                "Kernel version",
            ),
            (
                "processes",
                "procs",
                FieldValue::Table(vec![vec!["a".into()]]),
                "Process list",
            ),
        ]);
        let output = format_prometheus(&snap);
        assert!(!output.contains("syslenz_version_version_string"));
        assert!(!output.contains("syslenz_processes_procs"));
    }

    #[test]
    fn sanitizes_slashes_and_dashes() {
        let snap = make_snapshot(vec![
            (
                "net/dev",
                "rx_bytes",
                FieldValue::Bytes(1234),
                "Received bytes",
            ),
            (
                "file-nr",
                "allocated",
                FieldValue::Integer(5000),
                "Allocated FDs",
            ),
        ]);
        let output = format_prometheus(&snap);
        assert!(output.contains("syslenz_net_dev_rx_bytes 1234"));
        assert!(output.contains("syslenz_file_nr_allocated 5000"));
    }
}

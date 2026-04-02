use crate::proc::{Field, FieldValue, ProcEntry};
use std::process::Command;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let output = Command::new("ss").arg("-s").output()?;

    if !output.status.success() {
        anyhow::bail!("ss -s failed");
    }

    let content = String::from_utf8_lossy(&output.stdout);

    let mut tcp_established: i64 = 0;
    let mut tcp_timewait: i64 = 0;
    let mut tcp_orphaned: i64 = 0;
    let mut tcp_closed: i64 = 0;
    let mut udp_count: i64 = 0;

    for line in content.lines() {
        let line = line.trim();

        // TCP line: "TCP:   10 (estab 5, closed 2, orphaned 0, timewait 1)"
        if line.starts_with("TCP:") {
            // Extract values from parenthesized section
            if let Some(paren_start) = line.find('(') {
                let inner = &line[paren_start..];
                tcp_established = extract_named_value(inner, "estab");
                tcp_closed = extract_named_value(inner, "closed");
                tcp_orphaned = extract_named_value(inner, "orphaned");
                tcp_timewait = extract_named_value(inner, "timewait");
            }
        }

        // UDP line: "UDP:   8 (..."
        if line.starts_with("UDP:") {
            if let Some(count_str) = line.split_whitespace().nth(1) {
                udp_count = count_str.parse().unwrap_or(0);
            }
        }
    }

    Ok(ProcEntry {
        source: "ss -s".into(),
        fields: vec![
            Field {
                name: "tcp_established".into(),
                value: FieldValue::Integer(tcp_established),
                unit: None,
                description: "Number of established TCP connections".into(),
            },
            Field {
                name: "tcp_timewait".into(),
                value: FieldValue::Integer(tcp_timewait),
                unit: None,
                description: "Number of TCP connections in TIME_WAIT state".into(),
            },
            Field {
                name: "tcp_orphaned".into(),
                value: FieldValue::Integer(tcp_orphaned),
                unit: None,
                description: "Number of orphaned TCP connections (no owning process)".into(),
            },
            Field {
                name: "tcp_closed".into(),
                value: FieldValue::Integer(tcp_closed),
                unit: None,
                description: "Number of closed TCP connections".into(),
            },
            Field {
                name: "udp_count".into(),
                value: FieldValue::Integer(udp_count),
                unit: None,
                description: "Total number of UDP sockets".into(),
            },
        ],
    })
}

fn extract_named_value(s: &str, name: &str) -> i64 {
    s.split(name)
        .nth(1)
        .and_then(|after| {
            after
                .trim_start()
                .split(|c: char| !c.is_ascii_digit())
                .next()
        })
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}

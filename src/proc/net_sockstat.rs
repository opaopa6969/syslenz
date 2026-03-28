use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/net/sockstat")?;
    let mut fields = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }
        let category = parts[0].trim_end_matches(':');
        // Parse key-value pairs: category: key1 val1 key2 val2 ...
        let mut i = 1;
        while i + 1 < parts.len() {
            let key = parts[i];
            let val: i64 = parts[i + 1].parse().unwrap_or(0);
            let description = describe_sockstat_field(category, key);
            fields.push(Field {
                name: format!("{}_{}", category, key),
                value: FieldValue::Integer(val),
                unit: None,
                description,
            });
            i += 2;
        }
    }

    Ok(ProcEntry {
        source: "/proc/net/sockstat".into(),
        fields,
    })
}

fn describe_sockstat_field(category: &str, key: &str) -> String {
    match (category, key) {
        ("sockets", "used") => "Total number of sockets in use across all protocols".into(),
        ("TCP", "inuse") => "TCP sockets currently in use".into(),
        ("TCP", "orphan") => "TCP sockets with no owning process (orphaned)".into(),
        ("TCP", "tw") => "TCP sockets in TIME-WAIT state".into(),
        ("TCP", "alloc") => "TCP sockets allocated (all states)".into(),
        ("TCP", "mem") => "Pages of memory allocated for TCP socket buffers".into(),
        ("UDP", "inuse") => "UDP sockets currently in use".into(),
        ("UDP", "mem") => "Pages of memory allocated for UDP socket buffers".into(),
        ("UDPLITE", "inuse") => "UDP-Lite sockets currently in use".into(),
        ("RAW", "inuse") => "Raw sockets currently in use".into(),
        ("FRAG", "inuse") => "IP fragment reassembly entries currently active".into(),
        ("FRAG", "memory") => "Bytes of memory used for IP fragment reassembly".into(),
        _ => format!("{}: {}", category, key),
    }
}

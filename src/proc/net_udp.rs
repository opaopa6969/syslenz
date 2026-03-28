use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/net/udp")?;
    let mut rows = Vec::new();

    for (i, line) in content.lines().enumerate() {
        if i == 0 {
            continue; // skip header
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
            let local = parse_addr(parts[1]);
            let remote = parse_addr(parts[2]);
            let state = parts[3].to_string();
            let uid = parts.get(7).unwrap_or(&"?").to_string();
            rows.push(vec![local, remote, state, uid]);
        }
    }

    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/net/udp".into(),
        fields: vec![
            Field {
                name: "socket_count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Number of UDP sockets".into(),
            },
            Field {
                name: "sockets".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "UDP sockets (local_addr, remote_addr, state, uid)".into(),
            },
        ],
    })
}

fn parse_addr(hex_addr: &str) -> String {
    let parts: Vec<&str> = hex_addr.split(':').collect();
    if parts.len() != 2 {
        return hex_addr.to_string();
    }
    let ip_hex = parts[0];
    let port = u16::from_str_radix(parts[1], 16).unwrap_or(0);

    if ip_hex.len() == 8 {
        let ip = u32::from_str_radix(ip_hex, 16).unwrap_or(0);
        let a = ip & 0xFF;
        let b = (ip >> 8) & 0xFF;
        let c = (ip >> 16) & 0xFF;
        let d = (ip >> 24) & 0xFF;
        format!("{}.{}.{}.{}:{}", a, b, c, d, port)
    } else {
        format!("[{}]:{}", ip_hex, port)
    }
}

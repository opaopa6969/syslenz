use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let mut rows = Vec::new();

    for path in &["/proc/net/tcp", "/proc/net/tcp6"] {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let proto = if *path == "/proc/net/tcp" {
            "tcp"
        } else {
            "tcp6"
        };
        parse_proto_content(proto, &content, &mut rows);
    }

    build_entry(rows)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let mut rows = Vec::new();
    parse_proto_content("tcp", content, &mut rows);
    build_entry(rows)
}

fn parse_proto_content(proto: &str, content: &str, rows: &mut Vec<Vec<String>>) {
    for (i, line) in content.lines().enumerate() {
        if i == 0 {
            continue; // skip header
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
            let local = parse_addr(parts[1]);
            let remote = parse_addr(parts[2]);
            let state = decode_tcp_state(parts[3]);
            let uid = parts.get(7).unwrap_or(&"?").to_string();
            rows.push(vec![proto.to_string(), local, remote, state, uid]);
        }
    }
}

fn build_entry(rows: Vec<Vec<String>>) -> anyhow::Result<ProcEntry> {
    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/net/tcp".into(),
        fields: vec![
            Field {
                name: "connection_count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Number of TCP connections (v4 + v6)".into(),
            },
            Field {
                name: "connections".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "TCP connections (proto, local_addr, remote_addr, state, uid)".into(),
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
        // IPv4
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

fn decode_tcp_state(hex: &str) -> String {
    match hex {
        "01" => "ESTABLISHED".into(),
        "02" => "SYN_SENT".into(),
        "03" => "SYN_RECV".into(),
        "04" => "FIN_WAIT1".into(),
        "05" => "FIN_WAIT2".into(),
        "06" => "TIME_WAIT".into(),
        "07" => "CLOSE".into(),
        "08" => "CLOSE_WAIT".into(),
        "09" => "LAST_ACK".into(),
        "0A" => "LISTEN".into(),
        "0B" => "CLOSING".into(),
        _ => hex.to_string(),
    }
}

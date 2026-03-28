use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/net/unix")?;
    let mut rows = Vec::new();

    for (i, line) in content.lines().enumerate() {
        if i == 0 {
            continue; // skip header
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        // Format: Num RefCount Protocol Flags Type St Inode [Path]
        if parts.len() >= 7 {
            let refcount = parts[1].to_string();
            let sock_type = parts[4].to_string();
            let state = parts[5].to_string();
            let inode = parts[6].to_string();
            let path = if parts.len() >= 8 { parts[7].to_string() } else { String::new() };
            rows.push(vec![refcount, sock_type, state, inode, path]);
        }
    }

    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/net/unix".into(),
        fields: vec![
            Field {
                name: "socket_count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Number of Unix domain sockets".into(),
            },
            Field {
                name: "sockets".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Unix domain sockets (refcount, type, state, inode, path)".into(),
            },
        ],
    })
}

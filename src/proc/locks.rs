use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/locks")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let mut rows = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // Format: id: type mode lock_type pid major:minor:inode start end
        if parts.len() >= 8 {
            let lock_type = parts[1].to_string();   // POSIX or FLOCK
            let mode = parts[3].to_string();         // READ or WRITE
            let pid = parts[4].to_string();
            let inode_info = parts[5].to_string();
            let range_start = parts.get(6).unwrap_or(&"?").to_string();
            let range_end = parts.get(7).unwrap_or(&"?").to_string();
            rows.push(vec![lock_type, mode, pid, inode_info, range_start, range_end]);
        }
    }

    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/locks".into(),
        fields: vec![
            Field {
                name: "lock_count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Number of active file locks".into(),
            },
            Field {
                name: "locks".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Active file locks (type, mode, pid, inode, start, end)".into(),
            },
        ],
    })
}

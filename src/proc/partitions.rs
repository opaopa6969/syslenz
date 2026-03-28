use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/partitions")?;
    let mut rows = Vec::new();

    for line in content.lines().skip(2) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
            let blocks: u64 = parts[2].parse().unwrap_or(0);
            rows.push(vec![
                parts[3].to_string(),              // name
                format_size(blocks * 1024),         // size (blocks are 1KB)
                parts[0].to_string(),               // major
                parts[1].to_string(),               // minor
            ]);
        }
    }

    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/partitions".into(),
        fields: vec![
            Field {
                name: "count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Number of partitions".into(),
            },
            Field {
                name: "partitions".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Partitions (name, size, major, minor)".into(),
            },
        ],
    })
}

fn format_size(bytes: u64) -> String {
    const GIB: u64 = 1024 * 1024 * 1024;
    const MIB: u64 = 1024 * 1024;
    if bytes >= GIB {
        format!("{:.1} GiB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.1} MiB", bytes as f64 / MIB as f64)
    } else {
        format!("{} KiB", bytes / 1024)
    }
}

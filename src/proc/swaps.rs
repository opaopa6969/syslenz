use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/swaps")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let mut rows = Vec::new();
    let mut total_size: u64 = 0;
    let mut total_used: u64 = 0;

    for line in content.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 {
            let size: u64 = parts[2].parse().unwrap_or(0);
            let used: u64 = parts[3].parse().unwrap_or(0);
            total_size += size;
            total_used += used;
            rows.push(vec![
                parts[0].to_string(), // filename
                parts[1].to_string(), // type
                format_kb(size),      // size
                format_kb(used),      // used
                parts[4].to_string(), // priority
            ]);
        }
    }

    Ok(ProcEntry {
        source: "/proc/swaps".into(),
        fields: vec![
            Field {
                name: "total_size".into(),
                value: FieldValue::Bytes(total_size * 1024),
                unit: Some("kB".into()),
                description: "Total swap space".into(),
            },
            Field {
                name: "total_used".into(),
                value: FieldValue::Bytes(total_used * 1024),
                unit: Some("kB".into()),
                description: "Used swap space".into(),
            },
            Field {
                name: "usage_pct".into(),
                value: FieldValue::Float(if total_size > 0 {
                    total_used as f64 / total_size as f64 * 100.0
                } else {
                    0.0
                }),
                unit: Some("%".into()),
                description: "Swap usage percentage".into(),
            },
            Field {
                name: "swap_areas".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Swap areas (filename, type, size, used, priority)".into(),
            },
        ],
    })
}

fn format_kb(kb: u64) -> String {
    let bytes = kb * 1024;
    const GIB: u64 = 1024 * 1024 * 1024;
    const MIB: u64 = 1024 * 1024;
    if bytes >= GIB {
        format!("{:.1} GiB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.1} MiB", bytes as f64 / MIB as f64)
    } else {
        format!("{} KiB", kb)
    }
}

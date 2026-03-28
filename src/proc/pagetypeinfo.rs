use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/pagetypeinfo")?;
    let mut rows = Vec::new();
    let mut in_free_pages = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("Free pages count per migrate type") {
            in_free_pages = true;
            continue;
        }
        if trimmed.starts_with("Number of blocks type") {
            in_free_pages = false;
            continue;
        }
        if !in_free_pages {
            continue;
        }
        // Skip sub-headers
        if trimmed.starts_with("Node") && trimmed.contains("type") {
            continue;
        }
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 5 {
            // Node X, zone ZoneName, type TypeName count0 count1 ...
            let row: Vec<String> = parts.iter().map(|s| s.trim_end_matches(',').to_string()).collect();
            rows.push(row);
        }
    }

    let row_count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/pagetypeinfo".into(),
        fields: vec![
            Field {
                name: "entry_count".into(),
                value: FieldValue::Integer(row_count),
                unit: None,
                description: "Number of pagetype info entries".into(),
            },
            Field {
                name: "entries".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Page type info (node, zone, type, free counts per order)".into(),
            },
        ],
    })
}

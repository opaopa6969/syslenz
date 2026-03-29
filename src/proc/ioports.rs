use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/ioports")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let mut rows = Vec::new();

    for line in content.lines() {
        if let Some((range, desc)) = line.split_once(':') {
            rows.push(vec![
                range.trim().to_string(),
                desc.trim().to_string(),
            ]);
        }
    }

    let region_count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/ioports".into(),
        fields: vec![
            Field {
                name: "region_count".into(),
                value: FieldValue::Integer(region_count),
                unit: None,
                description: "Number of I/O port regions".into(),
            },
            Field {
                name: "regions".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "I/O port regions (port_range, description)".into(),
            },
        ],
    })
}

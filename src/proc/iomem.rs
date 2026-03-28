use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/iomem")?;
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
        source: "/proc/iomem".into(),
        fields: vec![
            Field {
                name: "region_count".into(),
                value: FieldValue::Integer(region_count),
                unit: None,
                description: "Number of I/O memory regions".into(),
            },
            Field {
                name: "regions".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "I/O memory map regions (address_range, description)".into(),
            },
        ],
    })
}

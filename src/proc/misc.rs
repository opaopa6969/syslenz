use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/misc")?;
    let mut rows = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            rows.push(vec![
                parts[0].to_string(), // minor number
                parts[1].to_string(), // device name
            ]);
        }
    }

    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/misc".into(),
        fields: vec![
            Field {
                name: "device_count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Number of misc devices".into(),
            },
            Field {
                name: "devices".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Misc devices (minor_number, name)".into(),
            },
        ],
    })
}

use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/devices")?;
    let mut rows = Vec::new();
    let mut current_type = String::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.ends_with("devices:") {
            current_type = trimmed.trim_end_matches(" devices:").to_string();
            continue;
        }
        // Format: "major name"
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 2 {
            rows.push(vec![
                current_type.clone(),
                parts[0].to_string(), // major number
                parts[1].to_string(), // device name
            ]);
        }
    }

    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/devices".into(),
        fields: vec![
            Field {
                name: "device_count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Number of registered devices".into(),
            },
            Field {
                name: "devices".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Registered devices (type, major, name)".into(),
            },
        ],
    })
}

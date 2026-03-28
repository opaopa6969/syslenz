use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/consoles")?;
    let mut rows = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let name = parts[0].to_string();
            let flags = parts[1..].join(" ");
            rows.push(vec![name, flags]);
        }
    }

    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/consoles".into(),
        fields: vec![
            Field {
                name: "console_count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Number of registered consoles".into(),
            },
            Field {
                name: "consoles".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Registered console devices (name, flags)".into(),
            },
        ],
    })
}

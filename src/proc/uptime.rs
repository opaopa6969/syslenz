use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/uptime")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let parts: Vec<&str> = content.trim().split_whitespace().collect();
    if parts.len() < 2 {
        anyhow::bail!(
            "invalid uptime format: expected 2 fields, got {}",
            parts.len()
        );
    }

    let uptime: f64 = parts[0].parse()?;
    let idle: f64 = parts[1].parse()?;

    Ok(ProcEntry {
        source: "/proc/uptime".into(),
        fields: vec![
            Field {
                name: "uptime".into(),
                value: FieldValue::Duration(uptime),
                unit: Some("seconds".into()),
                description: "System uptime".into(),
            },
            Field {
                name: "idle".into(),
                value: FieldValue::Duration(idle),
                unit: Some("seconds".into()),
                description: "Total idle time (sum of all CPUs)".into(),
            },
            Field {
                name: "idle_pct".into(),
                value: FieldValue::Float(idle / uptime * 100.0),
                unit: Some("%".into()),
                description: "Idle percentage (idle / uptime)".into(),
            },
        ],
    })
}

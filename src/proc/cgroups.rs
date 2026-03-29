use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/cgroups")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let mut rows = Vec::new();

    for line in content.lines() {
        if line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
            rows.push(vec![
                parts[0].to_string(), // subsys_name
                parts[1].to_string(), // hierarchy
                parts[2].to_string(), // num_cgroups
                parts[3].to_string(), // enabled
            ]);
        }
    }

    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/cgroups".into(),
        fields: vec![
            Field {
                name: "controller_count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Number of cgroup controllers".into(),
            },
            Field {
                name: "controllers".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Cgroup controllers (name, hierarchy, num_cgroups, enabled)".into(),
            },
        ],
    })
}

use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/modules")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let mut rows = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // Format: name size refcount deps state address
        if parts.len() >= 4 {
            let name = parts[0].to_string();
            let size = parts[1].to_string();
            let refcount = parts[2].to_string();
            let deps = parts[3].to_string();
            let state = parts.get(4).unwrap_or(&"").to_string();
            rows.push(vec![name, size, refcount, deps, state]);
        }
    }

    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/modules".into(),
        fields: vec![
            Field {
                name: "module_count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Number of loaded kernel modules".into(),
            },
            Field {
                name: "modules".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Loaded kernel modules (name, size, refcount, dependencies, state)"
                    .into(),
            },
        ],
    })
}

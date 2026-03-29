use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/net/wireless")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let mut rows = Vec::new();

    for (i, line) in content.lines().enumerate() {
        if i < 2 {
            continue; // skip two header lines
        }
        let line = line.replace('.', " ");
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
            let iface = parts[0].trim_end_matches(':').to_string();
            let status = parts[1].to_string();
            let quality_link = parts.get(2).unwrap_or(&"0").to_string();
            let quality_level = parts.get(3).unwrap_or(&"0").to_string();
            let quality_noise = parts.get(4).unwrap_or(&"0").to_string();
            rows.push(vec![iface, status, quality_link, quality_level, quality_noise]);
        }
    }

    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/net/wireless".into(),
        fields: vec![
            Field {
                name: "interface_count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Number of wireless interfaces".into(),
            },
            Field {
                name: "interfaces".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Wireless interfaces (iface, status, link, level, noise)".into(),
            },
        ],
    })
}

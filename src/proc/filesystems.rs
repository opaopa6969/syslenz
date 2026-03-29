use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/filesystems")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let mut rows = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let (nodev, fsname) = if trimmed.starts_with("nodev") {
            ("yes".to_string(), trimmed.trim_start_matches("nodev").trim().to_string())
        } else {
            ("no".to_string(), trimmed.to_string())
        };
        rows.push(vec![fsname, nodev]);
    }

    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/filesystems".into(),
        fields: vec![
            Field {
                name: "filesystem_count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Number of supported filesystems".into(),
            },
            Field {
                name: "filesystems".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Supported filesystems (name, nodev)".into(),
            },
        ],
    })
}

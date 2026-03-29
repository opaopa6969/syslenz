use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/dma")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let mut rows = Vec::new();

    for line in content.lines() {
        if let Some((channel, name)) = line.split_once(':') {
            rows.push(vec![
                channel.trim().to_string(),
                name.trim().to_string(),
            ]);
        }
    }

    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/dma".into(),
        fields: vec![
            Field {
                name: "channel_count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Number of DMA channels in use".into(),
            },
            Field {
                name: "channels".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "DMA channels (channel_number, device_name)".into(),
            },
        ],
    })
}

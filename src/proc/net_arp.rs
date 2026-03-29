use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/net/arp")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let mut rows = Vec::new();

    for (i, line) in content.lines().enumerate() {
        if i == 0 {
            continue; // skip header
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        // Format: IP address, HW type, Flags, HW address, Mask, Device
        if parts.len() >= 6 {
            rows.push(vec![
                parts[0].to_string(), // IP address
                parts[3].to_string(), // HW address
                parts[2].to_string(), // Flags
                parts[5].to_string(), // Device
            ]);
        }
    }

    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/net/arp".into(),
        fields: vec![
            Field {
                name: "entry_count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Number of ARP table entries".into(),
            },
            Field {
                name: "entries".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "ARP table entries (ip, hw_addr, flags, device)".into(),
            },
        ],
    })
}

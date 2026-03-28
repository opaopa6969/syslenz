use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/buddyinfo")?;
    let mut rows = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // Format: Node X, zone   ZoneName   count0 count1 count2 ...
        if parts.len() >= 5 {
            let node = parts.get(1).unwrap_or(&"?").trim_end_matches(',').to_string();
            let zone = parts.get(3).unwrap_or(&"?").to_string();
            let counts: Vec<String> = parts[4..].iter().map(|s| s.to_string()).collect();
            let mut row = vec![node, zone];
            row.extend(counts);
            rows.push(row);
        }
    }

    let zone_count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/buddyinfo".into(),
        fields: vec![
            Field {
                name: "zone_count".into(),
                value: FieldValue::Integer(zone_count),
                unit: None,
                description: "Number of memory zones".into(),
            },
            Field {
                name: "zones".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Memory fragmentation per zone (node, zone, free chunks per order 0-10)".into(),
            },
        ],
    })
}

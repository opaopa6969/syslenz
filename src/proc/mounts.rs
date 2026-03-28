use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/mounts")?;
    let mut rows = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.splitn(6, ' ').collect();
        if parts.len() >= 4 {
            rows.push(vec![
                parts[0].to_string(), // device
                parts[1].to_string(), // mountpoint
                parts[2].to_string(), // fstype
                parts[3].to_string(), // options
            ]);
        }
    }

    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/mounts".into(),
        fields: vec![
            Field {
                name: "count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Number of mounted filesystems".into(),
            },
            Field {
                name: "mounts".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Mounted filesystems (device, mountpoint, fstype, options)".into(),
            },
        ],
    })
}

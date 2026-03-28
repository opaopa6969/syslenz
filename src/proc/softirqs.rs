use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/softirqs")?;
    let mut rows = Vec::new();

    for (i, line) in content.lines().enumerate() {
        if i == 0 {
            // Skip header line
            continue;
        }
        if let Some((name, counts)) = line.split_once(':') {
            let name = name.trim().to_string();
            let total: u64 = counts
                .split_whitespace()
                .filter_map(|s| s.parse::<u64>().ok())
                .sum();
            rows.push(vec![name, total.to_string()]);
        }
    }

    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/softirqs".into(),
        fields: vec![
            Field {
                name: "softirq_count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Number of softirq types".into(),
            },
            Field {
                name: "softirqs".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Softirq counters (name, total_count across all CPUs)".into(),
            },
        ],
    })
}

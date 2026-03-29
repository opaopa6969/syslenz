use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/schedstat")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let mut rows = Vec::new();
    let mut sched_version = String::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        if parts[0] == "version" {
            sched_version = parts.get(1).unwrap_or(&"?").to_string();
            continue;
        }
        if parts[0].starts_with("cpu") {
            // cpu<N> fields: yld_count sched_count sched_goidle ttwu_count ttwu_local ...
            let cpu = parts[0].to_string();
            let values: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
            let mut row = vec![cpu];
            row.extend(values);
            rows.push(row);
        }
    }

    let mut fields = vec![
        Field {
            name: "version".into(),
            value: FieldValue::Text(sched_version),
            unit: None,
            description: "Scheduler statistics version".into(),
        },
        Field {
            name: "cpu_count".into(),
            value: FieldValue::Integer(rows.len() as i64),
            unit: None,
            description: "Number of CPUs with scheduler stats".into(),
        },
    ];

    fields.push(Field {
        name: "cpu_stats".into(),
        value: FieldValue::Table(rows),
        unit: None,
        description: "Per-CPU scheduler stats (cpu, yld_count, sched_count, sched_goidle, ttwu_count, ...)".into(),
    });

    Ok(ProcEntry {
        source: "/proc/schedstat".into(),
        fields,
    })
}

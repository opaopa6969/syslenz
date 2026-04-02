use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/timer_list")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let mut fields = Vec::new();
    let mut timer_count: i64 = 0;
    let mut clock_count: i64 = 0;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("Timer List Version:") {
            let version = trimmed
                .split_once(':')
                .map(|(_, v)| v.trim().to_string())
                .unwrap_or_default();
            fields.push(Field {
                name: "version".into(),
                value: FieldValue::Text(version),
                unit: None,
                description: "Timer list version".into(),
            });
        } else if trimmed.starts_with("now at") {
            let now = trimmed.split_whitespace().last().unwrap_or("0").to_string();
            fields.push(Field {
                name: "now".into(),
                value: FieldValue::Text(now),
                unit: Some("nsec".into()),
                description: "Current time in nanoseconds".into(),
            });
        } else if trimmed.starts_with("clock") {
            clock_count += 1;
        } else if trimmed.starts_with("#") && trimmed.contains(':') {
            timer_count += 1;
        }
    }

    fields.push(Field {
        name: "clock_count".into(),
        value: FieldValue::Integer(clock_count),
        unit: None,
        description: "Number of clock event devices".into(),
    });

    fields.push(Field {
        name: "timer_count".into(),
        value: FieldValue::Integer(timer_count),
        unit: None,
        description: "Number of pending timers".into(),
    });

    Ok(ProcEntry {
        source: "/proc/timer_list".into(),
        fields,
    })
}

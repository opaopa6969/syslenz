use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/net/sockstat")?;
    let mut fields = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }
        let category = parts[0].trim_end_matches(':');
        // Parse key-value pairs: category: key1 val1 key2 val2 ...
        let mut i = 1;
        while i + 1 < parts.len() {
            let key = parts[i];
            let val: i64 = parts[i + 1].parse().unwrap_or(0);
            fields.push(Field {
                name: format!("{}_{}", category, key),
                value: FieldValue::Integer(val),
                unit: None,
                description: format!("{} {}", category, key),
            });
            i += 2;
        }
    }

    Ok(ProcEntry {
        source: "/proc/net/sockstat".into(),
        fields,
    })
}

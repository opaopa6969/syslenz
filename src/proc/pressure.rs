use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let mut fields = Vec::new();

    for resource in &["cpu", "io", "memory"] {
        let path = format!("/proc/pressure/{}", resource);
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            let level = parts[0]; // "some" or "full"
            for part in &parts[1..] {
                if let Some((key, val)) = part.split_once('=') {
                    let field_name = format!("{}_{}_{}",resource, level, key);
                    let description = format!("{} {} pressure: {}", resource, level, key);
                    if key == "total" {
                        let v: i64 = val.parse().unwrap_or(0);
                        fields.push(Field {
                            name: field_name,
                            value: FieldValue::Integer(v),
                            unit: Some("us".into()),
                            description,
                        });
                    } else {
                        let v: f64 = val.parse().unwrap_or(0.0);
                        fields.push(Field {
                            name: field_name,
                            value: FieldValue::Float(v),
                            unit: Some("%".into()),
                            description,
                        });
                    }
                }
            }
        }
    }

    Ok(ProcEntry {
        source: "/proc/pressure/{cpu,io,memory}".into(),
        fields,
    })
}

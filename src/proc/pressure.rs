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
        parse_resource_content(resource, &content, &mut fields);
    }

    Ok(ProcEntry {
        source: "/proc/pressure/{cpu,io,memory}".into(),
        fields,
    })
}

/// Parse a single PSI resource file content (cpu, io, or memory).
pub fn parse_resource_content(resource: &str, content: &str, fields: &mut Vec<Field>) {
    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        let level = parts[0]; // "some" or "full"
        for part in &parts[1..] {
            if let Some((key, val)) = part.split_once('=') {
                let field_name = format!("{}_{}_{}", resource, level, key);
                let description = describe_psi_field(resource, level, key);
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

/// Parse all three resources from a combined map of content strings.
pub fn parse_content(contents: &[(&str, &str)]) -> anyhow::Result<ProcEntry> {
    let mut fields = Vec::new();
    for (resource, content) in contents {
        parse_resource_content(resource, content, &mut fields);
    }
    Ok(ProcEntry {
        source: "/proc/pressure/{cpu,io,memory}".into(),
        fields,
    })
}

fn describe_psi_field(resource: &str, level: &str, key: &str) -> String {
    let resource_desc = match resource {
        "cpu" => "CPU",
        "io" => "I/O",
        "memory" => "memory",
        other => other,
    };
    let level_desc = match level {
        "some" => "at least one task stalled on",
        "full" => "all non-idle tasks stalled on",
        other => other,
    };
    match key {
        "avg10" => format!("Pct of time {} {} (10s avg)", level_desc, resource_desc),
        "avg60" => format!("Pct of time {} {} (60s avg)", level_desc, resource_desc),
        "avg300" => format!("Pct of time {} {} (5min avg)", level_desc, resource_desc),
        "total" => format!("Total us {} {} since boot", level_desc, resource_desc),
        _ => format!("{} {} pressure: {}", resource_desc, level, key),
    }
}

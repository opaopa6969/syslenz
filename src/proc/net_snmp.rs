use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/net/snmp")?;
    let mut fields = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    // /proc/net/snmp has pairs of lines: header then values
    let mut i = 0;
    while i + 1 < lines.len() {
        let header_parts: Vec<&str> = lines[i].split_whitespace().collect();
        let value_parts: Vec<&str> = lines[i + 1].split_whitespace().collect();

        if header_parts.len() == value_parts.len() && header_parts.len() >= 2 {
            let category = header_parts[0].trim_end_matches(':');
            for j in 1..header_parts.len() {
                let val: i64 = value_parts[j].parse().unwrap_or(0);
                fields.push(Field {
                    name: format!("{}_{}", category, header_parts[j]),
                    value: FieldValue::Integer(val),
                    unit: None,
                    description: format!("{} {} counter", category, header_parts[j]),
                });
            }
        }
        i += 2;
    }

    Ok(ProcEntry {
        source: "/proc/net/snmp".into(),
        fields,
    })
}

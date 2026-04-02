use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/slabinfo")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let mut rows = Vec::new();

    for (i, line) in content.lines().enumerate() {
        if i < 2 {
            continue; // skip version and header lines
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        // Format: name active_objs num_objs objsize objperslab pagesperslab ...
        if parts.len() >= 6 {
            rows.push(vec![
                parts[0].to_string(), // cache name
                parts[1].to_string(), // active_objs
                parts[2].to_string(), // num_objs
                parts[3].to_string(), // objsize
                parts[4].to_string(), // objperslab
                parts[5].to_string(), // pagesperslab
            ]);
        }
    }

    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/slabinfo".into(),
        fields: vec![
            Field {
                name: "cache_count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Number of slab caches".into(),
            },
            Field {
                name: "caches".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description:
                    "Slab caches (name, active_objs, num_objs, objsize, objperslab, pagesperslab)"
                        .into(),
            },
        ],
    })
}

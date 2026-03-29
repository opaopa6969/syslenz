use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/crypto")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let mut rows = Vec::new();
    let mut name = String::new();
    let mut driver = String::new();
    let mut module = String::new();
    let mut algo_type = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            if !name.is_empty() {
                rows.push(vec![
                    name.clone(),
                    algo_type.clone(),
                    driver.clone(),
                    module.clone(),
                ]);
                name.clear();
                driver.clear();
                module.clear();
                algo_type.clear();
            }
            continue;
        }
        if let Some((key, val)) = line.split_once(':') {
            let key = key.trim();
            let val = val.trim().to_string();
            match key {
                "name" => name = val,
                "driver" => driver = val,
                "module" => module = val,
                "type" => algo_type = val,
                _ => {}
            }
        }
    }
    // Push last entry
    if !name.is_empty() {
        rows.push(vec![name, algo_type, driver, module]);
    }

    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/crypto".into(),
        fields: vec![
            Field {
                name: "algorithm_count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Number of registered crypto algorithms".into(),
            },
            Field {
                name: "algorithms".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Crypto algorithms (name, type, driver, module)".into(),
            },
        ],
    })
}

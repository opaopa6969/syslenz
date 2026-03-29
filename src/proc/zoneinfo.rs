use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/zoneinfo")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let mut rows = Vec::new();
    let mut current_zone = String::new();
    let mut free_pages: i64 = 0;
    let mut min_pages: i64 = 0;
    let mut low_pages: i64 = 0;
    let mut high_pages: i64 = 0;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("Node") {
            // Save previous zone
            if !current_zone.is_empty() {
                rows.push(vec![
                    current_zone.clone(),
                    free_pages.to_string(),
                    min_pages.to_string(),
                    low_pages.to_string(),
                    high_pages.to_string(),
                ]);
            }
            current_zone = trimmed.to_string();
            free_pages = 0;
            min_pages = 0;
            low_pages = 0;
            high_pages = 0;
            continue;
        }
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 2 {
            let val: i64 = parts[1].parse().unwrap_or(0);
            match parts[0] {
                "pages" => { /* header, skip */ }
                "free" => free_pages = val,
                "min" => min_pages = val,
                "low" => low_pages = val,
                "high" => high_pages = val,
                _ => {}
            }
        }
    }
    // Push last zone
    if !current_zone.is_empty() {
        rows.push(vec![
            current_zone,
            free_pages.to_string(),
            min_pages.to_string(),
            low_pages.to_string(),
            high_pages.to_string(),
        ]);
    }

    let zone_count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/zoneinfo".into(),
        fields: vec![
            Field {
                name: "zone_count".into(),
                value: FieldValue::Integer(zone_count),
                unit: None,
                description: "Number of memory zones".into(),
            },
            Field {
                name: "zones".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Memory zones (zone, free, min, low, high pages)".into(),
            },
        ],
    })
}

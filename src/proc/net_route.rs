use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/net/route")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let mut rows = Vec::new();

    for (i, line) in content.lines().enumerate() {
        if i == 0 {
            continue; // skip header
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        // Format: Iface Destination Gateway Flags RefCnt Use Metric Mask MTU Window IRTT
        if parts.len() >= 8 {
            let iface = parts[0].to_string();
            let dest = hex_to_ipv4(parts[1]);
            let gateway = hex_to_ipv4(parts[2]);
            let flags = parts[3].to_string();
            let mask = hex_to_ipv4(parts[7]);
            let metric = parts.get(6).unwrap_or(&"0").to_string();
            rows.push(vec![iface, dest, gateway, mask, flags, metric]);
        }
    }

    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/net/route".into(),
        fields: vec![
            Field {
                name: "route_count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Number of routing table entries".into(),
            },
            Field {
                name: "routes".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Routing table (iface, destination, gateway, mask, flags, metric)"
                    .into(),
            },
        ],
    })
}

fn hex_to_ipv4(hex: &str) -> String {
    let val = u32::from_str_radix(hex, 16).unwrap_or(0);
    let a = val & 0xFF;
    let b = (val >> 8) & 0xFF;
    let c = (val >> 16) & 0xFF;
    let d = (val >> 24) & 0xFF;
    format!("{}.{}.{}.{}", a, b, c, d)
}

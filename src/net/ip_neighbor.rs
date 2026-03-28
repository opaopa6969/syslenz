use crate::proc::{Field, FieldValue, ProcEntry};
use std::process::Command;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let output = Command::new("ip")
        .args(["neighbor", "show"])
        .output()?;

    if !output.status.success() {
        anyhow::bail!("ip neighbor show failed");
    }

    let content = String::from_utf8_lossy(&output.stdout);
    let mut rows = Vec::new();
    let mut failed_count: i64 = 0;

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let ip = parts[0].to_string();

        let get_val = |key: &str| -> String {
            parts.iter()
                .position(|&p| p == key)
                .and_then(|i| parts.get(i + 1))
                .map(|s| s.to_string())
                .unwrap_or_default()
        };

        let device = get_val("dev");
        let lladdr = get_val("lladdr");

        // State is typically the last token
        let state = parts.last()
            .map(|s| s.to_string())
            .unwrap_or_default();

        if state == "FAILED" {
            failed_count += 1;
        }

        rows.push(vec![ip, device, lladdr, state]);
    }

    let neighbor_count = rows.len() as i64;

    Ok(ProcEntry {
        source: "ip neighbor show".into(),
        fields: vec![
            Field {
                name: "neighbor_count".into(),
                value: FieldValue::Integer(neighbor_count),
                unit: None,
                description: "Total number of ARP/NDP neighbor entries".into(),
            },
            Field {
                name: "failed_count".into(),
                value: FieldValue::Integer(failed_count),
                unit: None,
                description: "Number of neighbors in FAILED state (unreachable)".into(),
            },
            Field {
                name: "neighbors".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Neighbor table: IP, Device, LLAddr (MAC), State".into(),
            },
        ],
    })
}

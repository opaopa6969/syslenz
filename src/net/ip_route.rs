use crate::proc::{Field, FieldValue, ProcEntry};
use std::process::Command;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let output = Command::new("ip")
        .args(["route", "show"])
        .output()?;

    if !output.status.success() {
        anyhow::bail!("ip route show failed");
    }

    let content = String::from_utf8_lossy(&output.stdout);
    let mut rows = Vec::new();
    let mut default_gateway = String::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let destination = parts[0].to_string();

        let get_val = |key: &str| -> String {
            parts.iter()
                .position(|&p| p == key)
                .and_then(|i| parts.get(i + 1))
                .map(|s| s.to_string())
                .unwrap_or_default()
        };

        let gateway = get_val("via");
        let device = get_val("dev");
        let protocol = get_val("proto");
        let scope = get_val("scope");
        let metric = get_val("metric");

        if destination == "default" && !gateway.is_empty() {
            default_gateway = gateway.clone();
        }

        rows.push(vec![
            destination,
            gateway,
            device,
            protocol,
            scope,
            metric,
        ]);
    }

    let route_count = rows.len() as i64;

    Ok(ProcEntry {
        source: "ip route show".into(),
        fields: vec![
            Field {
                name: "default_gateway".into(),
                value: FieldValue::Text(if default_gateway.is_empty() {
                    "(none)".into()
                } else {
                    default_gateway
                }),
                unit: None,
                description: "Default gateway IP address".into(),
            },
            Field {
                name: "route_count".into(),
                value: FieldValue::Integer(route_count),
                unit: None,
                description: "Total number of routing entries".into(),
            },
            Field {
                name: "routes".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Routing table: Destination, Gateway, Device, Protocol, Scope, Metric".into(),
            },
        ],
    })
}

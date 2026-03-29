use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/net/dev")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let mut rows = Vec::new();
    let mut total_rx: u64 = 0;
    let mut total_tx: u64 = 0;

    for line in content.lines().skip(2) {
        let Some((iface, stats)) = line.split_once(':') else { continue };
        let iface = iface.trim();
        let nums: Vec<u64> = stats
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if nums.len() >= 10 {
            let rx_bytes = nums[0];
            let rx_packets = nums[1];
            let tx_bytes = nums[8];
            let tx_packets = nums[9];

            total_rx += rx_bytes;
            total_tx += tx_bytes;

            rows.push(vec![
                iface.to_string(),
                format_bytes(rx_bytes),
                rx_packets.to_string(),
                format_bytes(tx_bytes),
                tx_packets.to_string(),
            ]);
        }
    }

    Ok(ProcEntry {
        source: "/proc/net/dev".into(),
        fields: vec![
            Field {
                name: "total_rx".into(),
                value: FieldValue::Bytes(total_rx),
                unit: Some("bytes".into()),
                description: "Total received across all interfaces".into(),
            },
            Field {
                name: "total_tx".into(),
                value: FieldValue::Bytes(total_tx),
                unit: Some("bytes".into()),
                description: "Total transmitted across all interfaces".into(),
            },
            Field {
                name: "interface_count".into(),
                value: FieldValue::Integer(rows.len() as i64),
                unit: None,
                description: "Number of network interfaces".into(),
            },
            Field {
                name: "interfaces".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Network interfaces (name, RX bytes, RX pkts, TX bytes, TX pkts)".into(),
            },
        ],
    })
}

fn format_bytes(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = 1024 * KIB;
    const GIB: u64 = 1024 * MIB;
    if bytes >= GIB {
        format!("{:.2} GiB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.1} MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.1} KiB", bytes as f64 / KIB as f64)
    } else {
        format!("{} B", bytes)
    }
}

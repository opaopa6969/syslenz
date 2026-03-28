use crate::proc::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    // Read conntrack max from sysctl
    let conntrack_max = fs::read_to_string("/proc/sys/net/nf_conntrack_max")
        .ok()
        .and_then(|s| s.trim().parse::<i64>().ok());

    // Try to get current count from /proc/sys/net/netfilter/nf_conntrack_count
    let conntrack_count = fs::read_to_string("/proc/sys/net/netfilter/nf_conntrack_count")
        .ok()
        .and_then(|s| s.trim().parse::<i64>().ok());

    // If neither is available, try `conntrack -C` as fallback
    let conntrack_count = match conntrack_count {
        Some(c) => Some(c),
        None => {
            std::process::Command::new("conntrack")
                .arg("-C")
                .output()
                .ok()
                .and_then(|o| {
                    if o.status.success() {
                        String::from_utf8_lossy(&o.stdout)
                            .trim()
                            .parse::<i64>()
                            .ok()
                    } else {
                        None
                    }
                })
        }
    };

    // If we have neither count nor max, connection tracking is not available
    if conntrack_count.is_none() && conntrack_max.is_none() {
        anyhow::bail!("Connection tracking not available (no nf_conntrack module loaded)");
    }

    let count = conntrack_count.unwrap_or(0);
    let max = conntrack_max.unwrap_or(0);
    let usage_pct = if max > 0 {
        (count as f64 / max as f64) * 100.0
    } else {
        0.0
    };

    Ok(ProcEntry {
        source: "/proc/sys/net/nf_conntrack".into(),
        fields: vec![
            Field {
                name: "conntrack_count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Current number of tracked connections".into(),
            },
            Field {
                name: "conntrack_max".into(),
                value: FieldValue::Integer(max),
                unit: None,
                description: "Maximum number of tracked connections (nf_conntrack_max)".into(),
            },
            Field {
                name: "usage_pct".into(),
                value: FieldValue::Float(usage_pct),
                unit: Some("%".into()),
                description: "Connection tracking table usage percentage".into(),
            },
        ],
    })
}

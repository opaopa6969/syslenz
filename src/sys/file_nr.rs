use crate::proc::{Field, FieldValue, ProcEntry};
use std::fs;

/// Parse system-wide file descriptor usage from /proc/sys/fs/file-nr.
///
/// Format: "allocated\tunused\tmax"
/// - allocated: number of file handles currently allocated
/// - unused: number of allocated but unused file handles
/// - max: maximum number of file handles the kernel will allocate
pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/sys/fs/file-nr")?;
    let parts: Vec<&str> = content.trim().split_whitespace().collect();

    if parts.len() < 3 {
        anyhow::bail!("unexpected format in /proc/sys/fs/file-nr");
    }

    let allocated: i64 = parts[0].parse()?;
    let unused: i64 = parts[1].parse()?;
    let max: i64 = parts[2].parse()?;

    let usage_pct = if max > 0 {
        ((allocated - unused) as f64 / max as f64) * 100.0
    } else {
        0.0
    };

    let fields = vec![
        Field {
            name: "fd_allocated".into(),
            value: FieldValue::Integer(allocated),
            unit: None,
            description: "Number of file handles currently allocated by the kernel".into(),
        },
        Field {
            name: "fd_unused".into(),
            value: FieldValue::Integer(unused),
            unit: None,
            description: "Number of allocated but unused file handles".into(),
        },
        Field {
            name: "fd_max".into(),
            value: FieldValue::Integer(max),
            unit: None,
            description: "Maximum number of file handles the kernel will allocate".into(),
        },
        Field {
            name: "fd_usage_pct".into(),
            value: FieldValue::Float(usage_pct),
            unit: Some("%".into()),
            description: "File descriptor usage percentage: (allocated - unused) / max * 100"
                .into(),
        },
    ];

    Ok(ProcEntry {
        source: "/proc/sys/fs/file-nr".into(),
        fields,
    })
}

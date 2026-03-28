use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/diskstats")?;
    let mut rows = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 14 { continue; }

        let name = parts[2];
        // Skip partition numbers (e.g., sda1) — only show whole devices and interesting ones
        let reads_completed: u64 = parts[3].parse().unwrap_or(0);
        let writes_completed: u64 = parts[7].parse().unwrap_or(0);
        let sectors_read: u64 = parts[5].parse().unwrap_or(0);
        let sectors_written: u64 = parts[9].parse().unwrap_or(0);
        let io_in_progress: u64 = parts[11].parse().unwrap_or(0);

        // Skip devices with zero activity
        if reads_completed == 0 && writes_completed == 0 { continue; }

        // Sectors are typically 512 bytes
        let bytes_read = sectors_read * 512;
        let bytes_written = sectors_written * 512;

        rows.push(vec![
            name.to_string(),
            reads_completed.to_string(),
            format_bytes(bytes_read),
            writes_completed.to_string(),
            format_bytes(bytes_written),
            io_in_progress.to_string(),
        ]);
    }

    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/diskstats".into(),
        fields: vec![
            Field {
                name: "active_devices".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Devices with I/O activity".into(),
            },
            Field {
                name: "devices".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Disks (name, reads, read bytes, writes, written bytes, in-flight)".into(),
            },
        ],
    })
}

fn format_bytes(bytes: u64) -> String {
    const GIB: u64 = 1024 * 1024 * 1024;
    const MIB: u64 = 1024 * 1024;
    const KIB: u64 = 1024;
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

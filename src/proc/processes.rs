use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let mut rows = Vec::new();

    let entries = fs::read_dir("/proc")?;
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        // Only numeric directories (PIDs)
        if !name_str.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }

        let pid = name_str.to_string();
        let base = format!("/proc/{}", pid);

        // Read comm (process name)
        let comm = fs::read_to_string(format!("{}/comm", base))
            .map(|s| s.trim().to_string())
            .unwrap_or_default();

        // Read status for key fields
        let mut state = String::new();
        let mut vm_rss = 0u64;
        let mut threads = 0i64;
        let mut uid = String::new();

        if let Ok(status) = fs::read_to_string(format!("{}/status", base)) {
            for line in status.lines() {
                if let Some((key, val)) = line.split_once(':') {
                    match key.trim() {
                        "State" => state = val.trim().to_string(),
                        "VmRSS" => {
                            let parts: Vec<&str> = val.trim().split_whitespace().collect();
                            vm_rss = parts
                                .first()
                                .and_then(|v| v.parse::<u64>().ok())
                                .unwrap_or(0)
                                * 1024; // kB to bytes
                        }
                        "Threads" => threads = val.trim().parse().unwrap_or(0),
                        "Uid" => {
                            uid = val
                                .trim()
                                .split_whitespace()
                                .next()
                                .unwrap_or("")
                                .to_string();
                        }
                        _ => {}
                    }
                }
            }
        }

        if comm.is_empty() {
            continue;
        }

        rows.push(vec![
            pid,
            comm,
            state,
            format_bytes(vm_rss),
            threads.to_string(),
            uid,
        ]);
    }

    // Sort by PID numerically
    rows.sort_by(|a, b| {
        a[0].parse::<u64>()
            .unwrap_or(0)
            .cmp(&b[0].parse::<u64>().unwrap_or(0))
    });

    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/[pid]/*".into(),
        fields: vec![
            Field {
                name: "process_count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Total number of processes".into(),
            },
            Field {
                name: "processes".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Processes (PID, name, state, RSS, threads, UID)".into(),
            },
        ],
    })
}

fn format_bytes(bytes: u64) -> String {
    const MIB: u64 = 1024 * 1024;
    const KIB: u64 = 1024;
    if bytes >= MIB {
        format!("{:.1} MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.1} KiB", bytes as f64 / KIB as f64)
    } else if bytes == 0 {
        "-".to_string()
    } else {
        format!("{} B", bytes)
    }
}

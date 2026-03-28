use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/loadavg")?;
    let parts: Vec<&str> = content.trim().split_whitespace().collect();

    let load1: f64 = parts[0].parse()?;
    let load5: f64 = parts[1].parse()?;
    let load15: f64 = parts[2].parse()?;

    let sched_parts: Vec<&str> = parts[3].split('/').collect();
    let running: i64 = sched_parts[0].parse()?;
    let total: i64 = sched_parts[1].parse()?;
    let last_pid: i64 = parts[4].parse()?;

    Ok(ProcEntry {
        source: "/proc/loadavg".into(),
        fields: vec![
            Field {
                name: "load_1min".into(),
                value: FieldValue::Float(load1),
                unit: None,
                description: "1-minute load average".into(),
            },
            Field {
                name: "load_5min".into(),
                value: FieldValue::Float(load5),
                unit: None,
                description: "5-minute load average".into(),
            },
            Field {
                name: "load_15min".into(),
                value: FieldValue::Float(load15),
                unit: None,
                description: "15-minute load average".into(),
            },
            Field {
                name: "running_threads".into(),
                value: FieldValue::Integer(running),
                unit: None,
                description: "Currently running/runnable threads".into(),
            },
            Field {
                name: "total_threads".into(),
                value: FieldValue::Integer(total),
                unit: None,
                description: "Total threads in system".into(),
            },
            Field {
                name: "last_pid".into(),
                value: FieldValue::Integer(last_pid),
                unit: None,
                description: "Last PID assigned".into(),
            },
        ],
    })
}

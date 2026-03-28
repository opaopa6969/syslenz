use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/stat")?;
    let mut fields = Vec::new();

    for line in content.lines() {
        if line.starts_with("cpu ") {
            let parts: Vec<u64> = line[4..]
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            if parts.len() >= 7 {
                let user = parts[0];
                let nice = parts[1];
                let system = parts[2];
                let idle = parts[3];
                let iowait = parts.get(4).copied().unwrap_or(0);
                let irq = parts.get(5).copied().unwrap_or(0);
                let softirq = parts.get(6).copied().unwrap_or(0);
                let steal = parts.get(7).copied().unwrap_or(0);

                let total = user + nice + system + idle + iowait + irq + softirq + steal;
                let busy = total - idle - iowait;

                fields.push(Field {
                    name: "cpu_user".into(),
                    value: FieldValue::Integer(user as i64),
                    unit: Some("jiffies".into()),
                    description: "Time in user mode".into(),
                });
                fields.push(Field {
                    name: "cpu_system".into(),
                    value: FieldValue::Integer(system as i64),
                    unit: Some("jiffies".into()),
                    description: "Time in kernel mode".into(),
                });
                fields.push(Field {
                    name: "cpu_idle".into(),
                    value: FieldValue::Integer(idle as i64),
                    unit: Some("jiffies".into()),
                    description: "Idle time".into(),
                });
                fields.push(Field {
                    name: "cpu_iowait".into(),
                    value: FieldValue::Integer(iowait as i64),
                    unit: Some("jiffies".into()),
                    description: "I/O wait time".into(),
                });
                fields.push(Field {
                    name: "cpu_usage_pct".into(),
                    value: FieldValue::Float(if total > 0 { busy as f64 / total as f64 * 100.0 } else { 0.0 }),
                    unit: Some("%".into()),
                    description: "Overall CPU usage (cumulative)".into(),
                });
            }
        } else if line.starts_with("processes ") {
            let val: i64 = line[10..].trim().parse().unwrap_or(0);
            fields.push(Field {
                name: "forks_total".into(),
                value: FieldValue::Integer(val),
                unit: None,
                description: "Total forks since boot".into(),
            });
        } else if line.starts_with("procs_running ") {
            let val: i64 = line[14..].trim().parse().unwrap_or(0);
            fields.push(Field {
                name: "procs_running".into(),
                value: FieldValue::Integer(val),
                unit: None,
                description: "Processes currently running".into(),
            });
        } else if line.starts_with("procs_blocked ") {
            let val: i64 = line[14..].trim().parse().unwrap_or(0);
            fields.push(Field {
                name: "procs_blocked".into(),
                value: FieldValue::Integer(val),
                unit: None,
                description: "Processes blocked on I/O".into(),
            });
        } else if line.starts_with("ctxt ") {
            let val: i64 = line[5..].trim().parse().unwrap_or(0);
            fields.push(Field {
                name: "context_switches".into(),
                value: FieldValue::Integer(val),
                unit: None,
                description: "Total context switches since boot".into(),
            });
        }
    }

    Ok(ProcEntry {
        source: "/proc/stat".into(),
        fields,
    })
}

use crate::proc::{Field, FieldValue, ProcEntry};
use std::process::Command;

/// Parse GPU metrics from nvidia-smi.
///
/// Queries GPU name, temperature, utilization, memory, fan speed, and power.
/// Gracefully returns Err if nvidia-smi is not found (AMD/Intel GPUs: skip).
pub fn parse() -> anyhow::Result<ProcEntry> {
    let output = Command::new("nvidia-smi")
        .args([
            "--query-gpu=name,temperature.gpu,utilization.gpu,utilization.memory,memory.total,memory.used,memory.free,fan.speed,power.draw,power.limit",
            "--format=csv,noheader,nounits",
        ])
        .output()
        .map_err(|e| anyhow::anyhow!("nvidia-smi not found or failed to execute: {}", e))?;

    if !output.status.success() {
        anyhow::bail!("nvidia-smi returned non-zero exit code");
    }

    let content = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();

    if lines.is_empty() {
        anyhow::bail!("nvidia-smi returned no GPU data");
    }

    let mut fields = Vec::new();
    let mut table_rows: Vec<Vec<String>> = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let cols: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        if cols.len() < 10 {
            continue;
        }

        let gpu_name = cols[0].to_string();
        let gpu_temp = parse_f64(cols[1]);
        let gpu_util = parse_f64(cols[2]);
        let mem_util = parse_f64(cols[3]);
        let mem_total_mib = parse_f64(cols[4]);
        let mem_used_mib = parse_f64(cols[5]);
        let mem_free_mib = parse_f64(cols[6]);
        let fan_speed = parse_f64(cols[7]);
        let power_draw = parse_f64(cols[8]);
        let power_limit = parse_f64(cols[9]);

        // Convert MiB to bytes
        let mib: f64 = 1024.0 * 1024.0;
        let mem_total = (mem_total_mib * mib) as u64;
        let mem_used = (mem_used_mib * mib) as u64;
        let mem_free = (mem_free_mib * mib) as u64;

        table_rows.push(vec![
            format!("{}", idx),
            gpu_name.clone(),
            format!("{:.0}", gpu_temp),
            format!("{:.1}", gpu_util),
            format!("{:.1}", mem_util),
            format!("{}", mem_total),
            format!("{}", mem_used),
            format!("{}", mem_free),
            format!("{:.0}", fan_speed),
            format!("{:.1}", power_draw),
            format!("{:.1}", power_limit),
        ]);

        // For GPU 0 (or the only GPU), emit summary fields
        if idx == 0 {
            fields.push(Field {
                name: "gpu_name".into(),
                value: FieldValue::Text(gpu_name),
                unit: None,
                description: "GPU model name".into(),
            });
            fields.push(Field {
                name: "gpu_temp".into(),
                value: FieldValue::Float(gpu_temp),
                unit: Some("°C".into()),
                description: "GPU temperature".into(),
            });
            fields.push(Field {
                name: "gpu_util".into(),
                value: FieldValue::Float(gpu_util),
                unit: Some("%".into()),
                description: "GPU utilization percentage".into(),
            });
            fields.push(Field {
                name: "mem_util".into(),
                value: FieldValue::Float(mem_util),
                unit: Some("%".into()),
                description: "GPU memory utilization percentage".into(),
            });
            fields.push(Field {
                name: "mem_total".into(),
                value: FieldValue::Bytes(mem_total),
                unit: Some("bytes".into()),
                description: "Total GPU memory".into(),
            });
            fields.push(Field {
                name: "mem_used".into(),
                value: FieldValue::Bytes(mem_used),
                unit: Some("bytes".into()),
                description: "Used GPU memory".into(),
            });
            fields.push(Field {
                name: "mem_free".into(),
                value: FieldValue::Bytes(mem_free),
                unit: Some("bytes".into()),
                description: "Free GPU memory".into(),
            });
            fields.push(Field {
                name: "fan_speed".into(),
                value: FieldValue::Float(fan_speed),
                unit: Some("%".into()),
                description: "GPU fan speed percentage".into(),
            });
            fields.push(Field {
                name: "power_draw".into(),
                value: FieldValue::Float(power_draw),
                unit: Some("W".into()),
                description: "Current GPU power draw".into(),
            });
            fields.push(Field {
                name: "power_limit".into(),
                value: FieldValue::Float(power_limit),
                unit: Some("W".into()),
                description: "GPU power limit".into(),
            });
        }
    }

    // If multiple GPUs, add a table with per-GPU rows
    if lines.len() > 1 {
        fields.push(Field {
            name: "gpus".into(),
            value: FieldValue::Table(table_rows),
            unit: None,
            description: "Per-GPU details: idx, name, temp(°C), gpu_util(%), mem_util(%), mem_total(B), mem_used(B), mem_free(B), fan(%), power_draw(W), power_limit(W)".into(),
        });
    }

    Ok(ProcEntry {
        source: "nvidia-smi".into(),
        fields,
    })
}

fn parse_f64(s: &str) -> f64 {
    s.trim()
        .replace("[Not Supported]", "0")
        .replace("[N/A]", "0")
        .parse::<f64>()
        .unwrap_or(0.0)
}

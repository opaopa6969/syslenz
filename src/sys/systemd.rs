use crate::proc::{Field, FieldValue, ProcEntry};
use std::process::Command;

/// Parse systemd service status from systemctl.
///
/// Lists all services, counts running/failed, and captures the system state.
/// Gracefully returns Err if systemctl is not found.
pub fn parse() -> anyhow::Result<ProcEntry> {
    // Get overall system state
    let system_state = get_system_state();

    // List all service units
    let output = Command::new("systemctl")
        .args(["list-units", "--type=service", "--all", "--no-pager", "--plain", "--no-legend"])
        .output()
        .map_err(|e| anyhow::anyhow!("systemctl not found or failed to execute: {}", e))?;

    if !output.status.success() {
        anyhow::bail!("systemctl list-units failed");
    }

    let content = String::from_utf8_lossy(&output.stdout);
    let mut fields = Vec::new();

    let mut service_count: i64 = 0;
    let mut running_count: i64 = 0;
    let mut failed_count: i64 = 0;
    let mut failed_rows: Vec<Vec<String>> = Vec::new();
    let mut all_rows: Vec<Vec<String>> = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Format: UNIT LOAD ACTIVE SUB DESCRIPTION...
        let parts: Vec<&str> = line.splitn(5, char::is_whitespace).collect();
        if parts.len() < 4 {
            continue;
        }

        // Filter out non-whitespace splits
        let cols: Vec<&str> = parts.iter().map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        if cols.len() < 4 {
            continue;
        }

        let unit = cols[0];
        let load = cols[1];
        let active = cols[2];
        let sub = cols[3];
        let description = if cols.len() > 4 { cols[4] } else { "" };

        service_count += 1;

        if sub == "running" {
            running_count += 1;
        }

        if active == "failed" {
            failed_count += 1;
            failed_rows.push(vec![
                unit.to_string(),
                load.to_string(),
                active.to_string(),
                sub.to_string(),
                description.to_string(),
            ]);
        }

        all_rows.push(vec![
            unit.to_string(),
            load.to_string(),
            active.to_string(),
            sub.to_string(),
            description.to_string(),
        ]);
    }

    fields.push(Field {
        name: "system_state".into(),
        value: FieldValue::Text(system_state),
        unit: None,
        description: "Overall systemd system state (running, degraded, maintenance)".into(),
    });

    fields.push(Field {
        name: "service_count".into(),
        value: FieldValue::Integer(service_count),
        unit: None,
        description: "Total number of systemd services".into(),
    });

    fields.push(Field {
        name: "running_count".into(),
        value: FieldValue::Integer(running_count),
        unit: None,
        description: "Number of running services".into(),
    });

    fields.push(Field {
        name: "failed_count".into(),
        value: FieldValue::Integer(failed_count),
        unit: None,
        description: "Number of failed services".into(),
    });

    fields.push(Field {
        name: "failed_services".into(),
        value: FieldValue::Table(failed_rows),
        unit: None,
        description: "Failed services: unit, load, active, sub, description".into(),
    });

    fields.push(Field {
        name: "all_services".into(),
        value: FieldValue::Table(all_rows),
        unit: None,
        description: "All services: unit, load, active, sub, description".into(),
    });

    Ok(ProcEntry {
        source: "systemctl".into(),
        fields,
    })
}

fn get_system_state() -> String {
    Command::new("systemctl")
        .arg("is-system-running")
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "unknown".into())
}

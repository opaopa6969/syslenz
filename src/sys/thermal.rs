use crate::proc::{Field, FieldValue, ProcEntry};
use std::fs;
use std::path::Path;

/// Parse CPU/GPU temperature from /sys/class/thermal/thermal_zone*/temp.
///
/// Each thermal zone directory contains a `temp` file (millidegrees Celsius)
/// and a `type` file (zone name like "x86_pkg_temp", "acpitz", etc.).
pub fn parse() -> anyhow::Result<ProcEntry> {
    let thermal_base = Path::new("/sys/class/thermal");
    let mut fields = Vec::new();
    let mut max_temp: f64 = 0.0;
    let mut max_zone = String::new();

    if !thermal_base.exists() {
        anyhow::bail!("/sys/class/thermal does not exist");
    }

    let mut entries: Vec<_> = fs::read_dir(thermal_base)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_str()
                .is_some_and(|n| n.starts_with("thermal_zone"))
        })
        .collect();

    // Sort by name for consistent ordering
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let zone_path = entry.path();

        let temp_path = zone_path.join("temp");
        let type_path = zone_path.join("type");

        let temp_str = match fs::read_to_string(&temp_path) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let zone_type = fs::read_to_string(&type_path)
            .unwrap_or_else(|_| "unknown".into())
            .trim()
            .to_string();

        let millideg: f64 = match temp_str.trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let degrees = millideg / 1000.0;

        fields.push(Field {
            name: zone_type.clone(),
            value: FieldValue::Float(degrees),
            unit: Some("°C".into()),
            description: format!("Temperature of thermal zone '{}'", zone_type),
        });

        if degrees > max_temp {
            max_temp = degrees;
            max_zone = zone_type;
        }
    }

    if fields.is_empty() {
        anyhow::bail!("No thermal zones found");
    }

    fields.push(Field {
        name: "max_temp".into(),
        value: FieldValue::Float(max_temp),
        unit: Some("°C".into()),
        description: format!("Highest temperature across all zones (from '{}')", max_zone),
    });

    Ok(ProcEntry {
        source: "/sys/class/thermal".into(),
        fields,
    })
}

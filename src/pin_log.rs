//! Feature 2 of docs/features/planned/pin-recall-and-selection-emit.md:
//! append-only JSONL logging of pinned items (`--log`).

use crate::pins::Pin;
use crate::proc::Snapshot;
use anyhow::Result;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

/// Append one JSON line per pinned item found in `snapshot` to `path`,
/// creating the file if needed. Pins whose source/field is absent from the
/// snapshot are skipped (the process being watched may not be running yet —
/// same tolerance as the TUI's greyed-out pin display). Returns the number
/// of lines written.
pub fn append_pin_values(path: &Path, pins: &[Pin], snapshot: &Snapshot) -> Result<usize> {
    let snapshot_json = serde_json::to_value(snapshot)?;
    let ts = snapshot_json
        .get("timestamp")
        .and_then(|v| v.as_str())
        .unwrap_or_default();

    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    let mut written = 0usize;

    for pin in pins {
        let Some(entry) = snapshot.entries.get(&pin.source) else {
            continue;
        };
        let fields = match &pin.field {
            Some(name) => entry.fields.iter().filter(|f| &f.name == name).collect(),
            None => entry.fields.iter().collect::<Vec<_>>(),
        };
        for field in fields {
            let line = serde_json::json!({
                "ts": ts,
                "host": pin.host,
                "source": pin.source,
                "field": field.name,
                "value": field.value,
            });
            writeln!(file, "{}", line)?;
            written += 1;
        }
    }

    Ok(written)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proc::{Field, FieldValue, ProcEntry};
    use std::collections::BTreeMap;
    use std::time::SystemTime;

    fn make_snapshot() -> Snapshot {
        let mut entries = BTreeMap::new();
        entries.insert(
            "meminfo".to_string(),
            ProcEntry {
                source: "/proc/meminfo".to_string(),
                fields: vec![
                    Field {
                        name: "MemAvailable".to_string(),
                        value: FieldValue::Bytes(8_217_034_752),
                        unit: Some("kB".to_string()),
                        description: "Available memory".to_string(),
                    },
                    Field {
                        name: "MemTotal".to_string(),
                        value: FieldValue::Bytes(16_000_000_000),
                        unit: Some("kB".to_string()),
                        description: "Total memory".to_string(),
                    },
                ],
            },
        );
        entries.insert(
            "loadavg".to_string(),
            ProcEntry {
                source: "/proc/loadavg".to_string(),
                fields: vec![Field {
                    name: "load_1min".to_string(),
                    value: FieldValue::Float(0.5),
                    unit: None,
                    description: "1-minute load average".to_string(),
                }],
            },
        );
        Snapshot {
            timestamp: SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_000),
            entries,
            alerts: Vec::new(),
        }
    }

    #[test]
    fn logs_pinned_field_only() {
        let tmp = tempfile::TempDir::new().unwrap();
        let path = tmp.path().join("pins.jsonl");
        let pins = vec![Pin {
            source: "meminfo".to_string(),
            field: Some("MemAvailable".to_string()),
            host: String::new(),
        }];
        let snapshot = make_snapshot();
        let written = append_pin_values(&path, &pins, &snapshot).unwrap();
        assert_eq!(written, 1);

        let contents = std::fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = contents.lines().collect();
        assert_eq!(lines.len(), 1);
        let parsed: serde_json::Value = serde_json::from_str(lines[0]).unwrap();
        assert_eq!(parsed["source"], "meminfo");
        assert_eq!(parsed["field"], "MemAvailable");
        assert_eq!(parsed["value"]["Bytes"], 8_217_034_752_u64);
        assert_eq!(parsed["host"], "");
        assert!(parsed["ts"].as_str().unwrap().starts_with("2023-"));
    }

    #[test]
    fn logs_whole_source_when_field_omitted() {
        let tmp = tempfile::TempDir::new().unwrap();
        let path = tmp.path().join("pins.jsonl");
        let pins = vec![Pin {
            source: "meminfo".to_string(),
            field: None,
            host: String::new(),
        }];
        let snapshot = make_snapshot();
        let written = append_pin_values(&path, &pins, &snapshot).unwrap();
        assert_eq!(written, 2);
    }

    #[test]
    fn skips_pin_absent_from_snapshot() {
        let tmp = tempfile::TempDir::new().unwrap();
        let path = tmp.path().join("pins.jsonl");
        let pins = vec![Pin {
            source: "jvm".to_string(),
            field: Some("heap_used".to_string()),
            host: String::new(),
        }];
        let snapshot = make_snapshot();
        let written = append_pin_values(&path, &pins, &snapshot).unwrap();
        assert_eq!(written, 0);
        assert!(!path.exists() || std::fs::read_to_string(&path).unwrap().is_empty());
    }

    #[test]
    fn appends_across_multiple_calls() {
        let tmp = tempfile::TempDir::new().unwrap();
        let path = tmp.path().join("pins.jsonl");
        let pins = vec![Pin {
            source: "loadavg".to_string(),
            field: Some("load_1min".to_string()),
            host: String::new(),
        }];
        let snapshot = make_snapshot();
        append_pin_values(&path, &pins, &snapshot).unwrap();
        append_pin_values(&path, &pins, &snapshot).unwrap();
        let contents = std::fs::read_to_string(&path).unwrap();
        assert_eq!(contents.lines().count(), 2);
    }
}

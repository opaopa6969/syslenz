use std::path::Path;
use anyhow::Result;
use crate::proc::Snapshot;

/// Write a single snapshot as pretty-printed JSON.
pub fn export_snapshot(snapshot: &Snapshot, path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(snapshot)?;
    std::fs::write(path, json)?;
    Ok(())
}

/// Write multiple snapshots as a JSON array.
pub fn export_series(snapshots: &[Snapshot], path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(snapshots)?;
    std::fs::write(path, json)?;
    Ok(())
}

/// Read a single snapshot from a JSON file.
pub fn import_snapshot(path: &Path) -> Result<Snapshot> {
    let data = std::fs::read_to_string(path)?;
    let snapshot: Snapshot = serde_json::from_str(&data)?;
    Ok(snapshot)
}

/// Read multiple snapshots from a JSON file (expects a JSON array).
pub fn import_series(path: &Path) -> Result<Vec<Snapshot>> {
    let data = std::fs::read_to_string(path)?;
    let snapshots: Vec<Snapshot> = serde_json::from_str(&data)?;
    Ok(snapshots)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proc::{ProcEntry, Field, FieldValue, Snapshot};
    use std::collections::BTreeMap;
    use std::time::SystemTime;

    fn make_test_snapshot() -> Snapshot {
        let mut entries = BTreeMap::new();
        entries.insert("meminfo".into(), ProcEntry {
            source: "/proc/meminfo".into(),
            fields: vec![
                Field {
                    name: "MemTotal".into(),
                    value: FieldValue::Bytes(16 * 1024 * 1024 * 1024),
                    unit: Some("kB".into()),
                    description: "Total usable RAM".into(),
                },
                Field {
                    name: "MemFree".into(),
                    value: FieldValue::Bytes(8 * 1024 * 1024 * 1024),
                    unit: Some("kB".into()),
                    description: "Free memory".into(),
                },
            ],
        });
        entries.insert("loadavg".into(), ProcEntry {
            source: "/proc/loadavg".into(),
            fields: vec![
                Field {
                    name: "load_1min".into(),
                    value: FieldValue::Float(0.50),
                    unit: None,
                    description: "1-minute load average".into(),
                },
            ],
        });
        entries.insert("uptime".into(), ProcEntry {
            source: "/proc/uptime".into(),
            fields: vec![
                Field {
                    name: "uptime".into(),
                    value: FieldValue::Duration(86400.0),
                    unit: Some("seconds".into()),
                    description: "System uptime".into(),
                },
            ],
        });
        Snapshot {
            timestamp: SystemTime::now(),
            entries,
        }
    }

    // T3: export_snapshot -> import_snapshot round-trip
    #[test]
    fn snapshot_export_import_roundtrip() {
        let original = make_test_snapshot();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.json");

        export_snapshot(&original, &path).unwrap();
        let restored = import_snapshot(&path).unwrap();

        assert_eq!(original.entries.len(), restored.entries.len());
        for (key, entry) in &original.entries {
            let restored_entry = restored.entries.get(key)
                .unwrap_or_else(|| panic!("Missing key '{}' in restored snapshot", key));
            assert_eq!(entry.fields.len(), restored_entry.fields.len(),
                "Field count mismatch for source '{}'", key);
            for (f1, f2) in entry.fields.iter().zip(restored_entry.fields.iter()) {
                assert_eq!(f1.name, f2.name);
                assert_eq!(f1.description, f2.description);
                assert_eq!(f1.value.display(), f2.value.display());
            }
        }
        // Verify timestamp survives round-trip
        assert_eq!(original.timestamp, restored.timestamp);
    }

    // T4: export_series -> import_series round-trip
    #[test]
    fn series_export_import_roundtrip() {
        let s1 = make_test_snapshot();
        let s2 = make_test_snapshot();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("series.json");

        export_series(&[s1, s2], &path).unwrap();
        let restored = import_series(&path).unwrap();

        assert_eq!(2, restored.len(), "Expected 2 snapshots in restored series");
        for snap in &restored {
            assert_eq!(3, snap.entries.len(), "Each snapshot should have 3 entries");
        }
    }
}

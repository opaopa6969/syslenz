// P-A5: History JSONL writer
// Periodically appends Snapshot data as JSONL to date-partitioned files.
// To integrate: add `mod history;` in main.rs, call `history.write_if_due(&snapshot)` in the run() loop.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime};

use anyhow::Result;

use crate::proc::Snapshot;

/// Configuration for the history writer, populated from [history] in config.toml.
#[derive(Debug, Clone)]
pub struct HistoryConfig {
    pub enabled: bool,
    pub interval_secs: u64,
    pub retention_days: u32,
    pub path: Option<PathBuf>,
}

impl Default for HistoryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_secs: 60,
            retention_days: 7,
            path: None,
        }
    }
}

/// Writes snapshots as JSONL to date-partitioned files.
pub struct HistoryWriter {
    dir: PathBuf,
    interval: Duration,
    retention_days: u32,
    last_write: Option<Instant>,
}

impl HistoryWriter {
    /// Create a new HistoryWriter from config.
    pub fn new(config: &HistoryConfig) -> Self {
        let dir = config.path.clone().unwrap_or_else(default_history_dir);
        Self {
            dir,
            interval: Duration::from_secs(config.interval_secs),
            retention_days: config.retention_days,
            last_write: None,
        }
    }

    /// Write the snapshot if the configured interval has elapsed since the last write.
    /// Returns Ok(()) if skipped (interval not yet elapsed) or written successfully.
    pub fn write_if_due(&mut self, snapshot: &Snapshot) -> Result<()> {
        if let Some(last) = self.last_write {
            if last.elapsed() < self.interval {
                return Ok(());
            }
        }

        self.write_snapshot(snapshot)?;
        self.last_write = Some(Instant::now());
        Ok(())
    }

    /// Append a single snapshot as one JSON line to the date-partitioned file.
    fn write_snapshot(&self, snapshot: &Snapshot) -> Result<()> {
        fs::create_dir_all(&self.dir)?;

        let filename = self.filename_for(snapshot.timestamp);
        let path = self.dir.join(filename);

        let line = serde_json::to_string(snapshot)?;

        let mut file = OpenOptions::new().create(true).append(true).open(&path)?;
        writeln!(file, "{}", line)?;

        Ok(())
    }

    /// Remove JSONL files older than retention_days.
    pub fn cleanup(&self) -> Result<()> {
        let entries = match fs::read_dir(&self.dir) {
            Ok(e) => e,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(e) => return Err(e.into()),
        };

        let cutoff = SystemTime::now()
            .checked_sub(Duration::from_secs(u64::from(self.retention_days) * 86400))
            .unwrap_or(SystemTime::UNIX_EPOCH);

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            // Only consider .jsonl files
            if path.extension().and_then(|e| e.to_str()) != Some("jsonl") {
                continue;
            }

            // Use file modification time for age check
            let metadata = fs::metadata(&path)?;
            let modified = metadata.modified()?;

            if modified < cutoff {
                fs::remove_file(&path)?;
            }
        }

        Ok(())
    }

    /// Generate a filename like "2026-03-29.jsonl" for the given timestamp.
    fn filename_for(&self, time: SystemTime) -> String {
        let secs = time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let days = secs / 86400;
        let (year, month, day) = days_to_ymd(days as i64);
        format!("{:04}-{:02}-{:02}.jsonl", year, month, day)
    }
}

/// G20-9: Append an alert event to `alerts.jsonl` in the history directory.
/// Each line is a JSON object with timestamp, severity, source, field, value, message, host, status.
pub fn append_alert_event(
    dir: &std::path::Path,
    event: &crate::alert::AlertEvent,
    host: &str,
) -> Result<()> {
    fs::create_dir_all(dir)?;
    let path = dir.join("alerts.jsonl");

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let status = if event.firing { "firing" } else { "resolved" };

    let entry = serde_json::json!({
        "timestamp": now,
        "severity": event.severity,
        "source": event.source,
        "field": event.field,
        "value": event.current_value,
        "message": event.message,
        "host": host,
        "status": status,
    });

    let mut file = OpenOptions::new().create(true).append(true).open(&path)?;
    writeln!(file, "{}", entry)?;

    Ok(())
}

/// Default alert history directory: same as snapshot history directory.
/// Public so that alert.rs can call record_alert_history with this path.
pub fn default_alert_history_dir() -> PathBuf {
    default_history_dir()
}

/// Default history directory: ~/.local/share/syslenz/history
fn default_history_dir() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home)
            .join(".local")
            .join("share")
            .join("syslenz")
            .join("history")
    } else {
        PathBuf::from("/tmp/syslenz/history")
    }
}

/// Convert days since Unix epoch to (year, month, day).
/// Ported from the existing chrono_lite logic in proc/mod.rs.
fn days_to_ymd(days: i64) -> (i64, u32, u32) {
    // Algorithm from http://howardhinnant.github.io/date_algorithms.html
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use tempfile::TempDir;

    fn make_snapshot() -> Snapshot {
        Snapshot {
            timestamp: SystemTime::now(),
            entries: BTreeMap::new(),
            alerts: Vec::new(),
        }
    }

    fn make_writer(
        dir: &std::path::Path,
        interval_secs: u64,
        retention_days: u32,
    ) -> HistoryWriter {
        HistoryWriter {
            dir: dir.to_path_buf(),
            interval: Duration::from_secs(interval_secs),
            retention_days,
            last_write: None,
        }
    }

    #[test]
    fn write_if_due_skips_when_interval_not_elapsed() {
        let tmp = TempDir::new().unwrap();
        let mut writer = make_writer(tmp.path(), 3600, 7); // 1 hour interval

        let snap = make_snapshot();
        // First write should succeed
        writer.write_if_due(&snap).unwrap();
        assert!(writer.last_write.is_some());

        // Second write should be skipped (interval not elapsed)
        writer.write_if_due(&snap).unwrap();

        // File should have exactly 1 line (not 2)
        let files: Vec<_> = fs::read_dir(tmp.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        assert_eq!(files.len(), 1);
        let content = fs::read_to_string(files[0].path()).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(
            lines.len(),
            1,
            "Should have only 1 line since interval hasn't elapsed"
        );
    }

    #[test]
    fn write_if_due_writes_when_interval_elapsed() {
        let tmp = TempDir::new().unwrap();
        let mut writer = make_writer(tmp.path(), 0, 7); // 0-second interval => always write

        let snap = make_snapshot();
        writer.write_if_due(&snap).unwrap();
        writer.write_if_due(&snap).unwrap();

        // Should have 2 lines in the file
        let files: Vec<_> = fs::read_dir(tmp.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        assert_eq!(files.len(), 1);
        let content = fs::read_to_string(files[0].path()).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 2, "Should have 2 lines since interval is 0");

        // Each line should be valid JSON
        for line in &lines {
            let _: serde_json::Value = serde_json::from_str(line).unwrap();
        }
    }

    #[test]
    fn cleanup_removes_old_files() {
        let tmp = TempDir::new().unwrap();

        // Create a "recent" file (mtime = now)
        let recent = tmp.path().join("recent.jsonl");
        fs::write(&recent, "{}\n").unwrap();

        // Create an "old" file and backdate its mtime to epoch via libc::utime
        let old_file = tmp.path().join("old.jsonl");
        fs::write(&old_file, "{}\n").unwrap();
        let path_cstr = std::ffi::CString::new(old_file.to_str().unwrap()).unwrap();
        let times = libc::utimbuf {
            actime: 0,
            modtime: 0,
        };
        unsafe {
            libc::utime(path_cstr.as_ptr(), &times);
        }

        let writer = make_writer(tmp.path(), 60, 1);
        writer.cleanup().unwrap();

        // old.jsonl should be removed (mtime=epoch is > 1 day old)
        assert!(!old_file.exists(), "Old file should be removed");
        // recent file should still exist
        assert!(recent.exists(), "Recent file should still exist");
    }

    #[test]
    fn cleanup_on_missing_dir_is_ok() {
        let writer = make_writer(
            std::path::Path::new("/tmp/syslenz_nonexistent_test_dir"),
            60,
            7,
        );
        // Should not error when directory doesn't exist
        writer.cleanup().unwrap();
    }

    #[test]
    fn filename_for_produces_correct_date() {
        let tmp = TempDir::new().unwrap();
        let writer = make_writer(tmp.path(), 60, 7);

        // 2025-03-29 00:00:00 UTC = 1743206400 seconds since epoch
        let time = SystemTime::UNIX_EPOCH + Duration::from_secs(1743206400);
        let name = writer.filename_for(time);
        assert_eq!(name, "2025-03-29.jsonl");
    }

    // G20-9: append_alert_event writes to alerts.jsonl
    #[test]
    fn append_alert_event_writes_jsonl() {
        let tmp = TempDir::new().unwrap();
        let event = crate::alert::AlertEvent {
            rule_index: 0,
            source: "meminfo".to_string(),
            field: "MemAvailable".to_string(),
            severity: "critical".to_string(),
            message: "Memory low".to_string(),
            current_value: "100000".to_string(),
            firing: true,
        };

        append_alert_event(tmp.path(), &event, "myhost").unwrap();

        let path = tmp.path().join("alerts.jsonl");
        assert!(path.exists());
        let content = fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 1);

        let entry: serde_json::Value = serde_json::from_str(lines[0]).unwrap();
        assert_eq!(entry["host"], "myhost");
        assert_eq!(entry["severity"], "critical");
        assert_eq!(entry["source"], "meminfo");
        assert_eq!(entry["field"], "MemAvailable");
        assert_eq!(entry["value"], "100000");
        assert_eq!(entry["message"], "Memory low");
        assert_eq!(entry["status"], "firing");
        assert!(entry["timestamp"].is_number());
    }

    // G20-9: append_alert_event records resolved status
    #[test]
    fn append_alert_event_resolved() {
        let tmp = TempDir::new().unwrap();
        let event = crate::alert::AlertEvent {
            rule_index: 0,
            source: "loadavg".to_string(),
            field: "load_1min".to_string(),
            severity: "warning".to_string(),
            message: "Load normal".to_string(),
            current_value: "2.0".to_string(),
            firing: false,
        };

        append_alert_event(tmp.path(), &event, "server1").unwrap();

        let path = tmp.path().join("alerts.jsonl");
        let content = fs::read_to_string(&path).unwrap();
        let entry: serde_json::Value = serde_json::from_str(content.trim()).unwrap();
        assert_eq!(entry["status"], "resolved");
        assert_eq!(entry["host"], "server1");
    }

    // G20-9: multiple appends accumulate in alerts.jsonl
    #[test]
    fn append_alert_event_appends_multiple() {
        let tmp = TempDir::new().unwrap();
        let event1 = crate::alert::AlertEvent {
            rule_index: 0,
            source: "meminfo".to_string(),
            field: "MemAvailable".to_string(),
            severity: "critical".to_string(),
            message: "Firing".to_string(),
            current_value: "100".to_string(),
            firing: true,
        };
        let event2 = crate::alert::AlertEvent {
            rule_index: 0,
            source: "meminfo".to_string(),
            field: "MemAvailable".to_string(),
            severity: "critical".to_string(),
            message: "Resolved".to_string(),
            current_value: "900000".to_string(),
            firing: false,
        };

        append_alert_event(tmp.path(), &event1, "host1").unwrap();
        append_alert_event(tmp.path(), &event2, "host1").unwrap();

        let path = tmp.path().join("alerts.jsonl");
        let content = fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 2);
    }
}

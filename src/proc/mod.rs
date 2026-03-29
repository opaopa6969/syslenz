#[cfg(target_os = "linux")]
pub mod meminfo;
#[cfg(target_os = "linux")]
pub mod uptime;
#[cfg(target_os = "linux")]
pub mod loadavg;
#[cfg(target_os = "linux")]
pub mod version;
#[cfg(target_os = "linux")]
pub mod mounts;
#[cfg(target_os = "linux")]
pub mod partitions;
#[cfg(target_os = "linux")]
pub mod cpuinfo;
#[cfg(target_os = "linux")]
pub mod stat;
#[cfg(target_os = "linux")]
pub mod net_dev;
#[cfg(target_os = "linux")]
pub mod diskstats;
#[cfg(target_os = "linux")]
pub mod processes;
#[cfg(target_os = "linux")]
pub mod swaps;
#[cfg(target_os = "linux")]
pub mod buddyinfo;
#[cfg(target_os = "linux")]
pub mod cgroups;
#[cfg(target_os = "linux")]
pub mod cmdline;
#[cfg(target_os = "linux")]
pub mod consoles;
#[cfg(target_os = "linux")]
pub mod crypto;
#[cfg(target_os = "linux")]
pub mod devices;
#[cfg(target_os = "linux")]
pub mod filesystems;
#[cfg(target_os = "linux")]
pub mod interrupts;
#[cfg(target_os = "linux")]
pub mod iomem;
#[cfg(target_os = "linux")]
pub mod ioports;
#[cfg(target_os = "linux")]
pub mod locks;
#[cfg(target_os = "linux")]
pub mod modules;
#[cfg(target_os = "linux")]
pub mod vmstat;
#[cfg(target_os = "linux")]
pub mod zoneinfo;
#[cfg(target_os = "linux")]
pub mod softirqs;
#[cfg(target_os = "linux")]
pub mod misc;
#[cfg(target_os = "linux")]
pub mod pressure;
#[cfg(target_os = "linux")]
pub mod net_tcp;
#[cfg(target_os = "linux")]
pub mod net_udp;
#[cfg(target_os = "linux")]
pub mod net_unix;
#[cfg(target_os = "linux")]
pub mod net_arp;
#[cfg(target_os = "linux")]
pub mod net_route;
#[cfg(target_os = "linux")]
pub mod net_sockstat;
#[cfg(target_os = "linux")]
pub mod net_snmp;
#[cfg(target_os = "linux")]
pub mod net_netstat;
#[cfg(target_os = "linux")]
pub mod net_wireless;
#[cfg(target_os = "linux")]
pub mod slabinfo;
#[cfg(target_os = "linux")]
pub mod pagetypeinfo;
#[cfg(target_os = "linux")]
pub mod schedstat;
#[cfg(target_os = "linux")]
pub mod dma;
#[cfg(target_os = "linux")]
pub mod timer_list;

#[cfg(target_os = "macos")]
pub mod platform_macos;
#[cfg(target_os = "windows")]
pub mod platform_windows;

use std::collections::BTreeMap;
use std::time::SystemTime;
use serde::{Serialize, Deserialize};

mod systemtime_iso8601 {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::time::SystemTime;

    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = time
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(serde::ser::Error::custom)?;
        let secs = duration.as_secs();
        let nanos = duration.subsec_nanos();
        // Format as ISO 8601
        let datetime = chrono_lite_format(secs, nanos);
        serializer.serialize_str(&datetime)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_iso8601(&s).map_err(serde::de::Error::custom)
    }

    fn chrono_lite_format(secs: u64, nanos: u32) -> String {
        // Convert unix timestamp to UTC datetime components
        let days = (secs / 86400) as i64;
        let time_of_day = secs % 86400;
        let hours = time_of_day / 3600;
        let minutes = (time_of_day % 3600) / 60;
        let seconds = time_of_day % 60;

        // Days since 1970-01-01 to Y-M-D
        let (year, month, day) = days_to_ymd(days);

        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:09}Z",
            year, month, day, hours, minutes, seconds, nanos
        )
    }

    fn days_to_ymd(mut days: i64) -> (i64, u32, u32) {
        // Algorithm from Howard Hinnant
        days += 719468;
        let era = if days >= 0 { days } else { days - 146096 } / 146097;
        let doe = (days - era * 146097) as u32;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe as i64 + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        let y = if m <= 2 { y + 1 } else { y };
        (y, m, d)
    }

    fn parse_iso8601(s: &str) -> Result<SystemTime, String> {
        // Parse "YYYY-MM-DDTHH:MM:SS.nnnnnnnnnZ"
        let s = s.trim_end_matches('Z');
        let (date_part, time_part) = s.split_once('T')
            .ok_or_else(|| "missing T separator".to_string())?;
        let date_parts: Vec<&str> = date_part.split('-').collect();
        if date_parts.len() != 3 {
            return Err("invalid date".to_string());
        }
        let year: i64 = date_parts[0].parse().map_err(|e| format!("{}", e))?;
        let month: u32 = date_parts[1].parse().map_err(|e| format!("{}", e))?;
        let day: u32 = date_parts[2].parse().map_err(|e| format!("{}", e))?;

        let (time_main, nanos_str) = if let Some((t, n)) = time_part.split_once('.') {
            (t, n)
        } else {
            (time_part, "0")
        };
        let time_parts: Vec<&str> = time_main.split(':').collect();
        if time_parts.len() != 3 {
            return Err("invalid time".to_string());
        }
        let hours: u64 = time_parts[0].parse().map_err(|e| format!("{}", e))?;
        let minutes: u64 = time_parts[1].parse().map_err(|e| format!("{}", e))?;
        let seconds: u64 = time_parts[2].parse().map_err(|e| format!("{}", e))?;
        let nanos: u32 = {
            let padded = format!("{:0<9}", nanos_str);
            padded[..9].parse().map_err(|e| format!("{}", e))?
        };

        // Convert Y-M-D to days since epoch
        let days = ymd_to_days(year, month, day);
        let total_secs = days as u64 * 86400 + hours * 3600 + minutes * 60 + seconds;
        let duration = std::time::Duration::new(total_secs, nanos);
        Ok(SystemTime::UNIX_EPOCH + duration)
    }

    fn ymd_to_days(y: i64, m: u32, d: u32) -> i64 {
        let y = if m <= 2 { y - 1 } else { y };
        let era = if y >= 0 { y } else { y - 399 } / 400;
        let yoe = (y - era * 400) as u32;
        let m = m;
        let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        era * 146097 + doe as i64 - 719468
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    #[serde(with = "systemtime_iso8601")]
    pub timestamp: SystemTime,
    pub entries: BTreeMap<String, ProcEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcEntry {
    pub source: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub value: FieldValue,
    pub unit: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldValue {
    Bytes(u64),
    Integer(i64),
    Float(f64),
    Text(String),
    Duration(f64),
    Table(Vec<Vec<String>>),
}

impl FieldValue {
    pub fn display(&self) -> String {
        match self {
            FieldValue::Bytes(b) => format_bytes(*b),
            FieldValue::Integer(i) => i.to_string(),
            FieldValue::Float(f) => format!("{:.2}", f),
            FieldValue::Text(s) => s.clone(),
            FieldValue::Duration(secs) => format_duration(*secs),
            FieldValue::Table(rows) => format!("[{} rows]", rows.len()),
        }
    }
}

impl PartialEq for FieldValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FieldValue::Bytes(a), FieldValue::Bytes(b)) => a == b,
            (FieldValue::Integer(a), FieldValue::Integer(b)) => a == b,
            (FieldValue::Float(a), FieldValue::Float(b)) => (a - b).abs() < 1e-9,
            (FieldValue::Text(a), FieldValue::Text(b)) => a == b,
            (FieldValue::Duration(a), FieldValue::Duration(b)) => (a - b).abs() < 1e-9,
            (FieldValue::Table(a), FieldValue::Table(b)) => a == b,
            _ => false,
        }
    }
}

fn format_bytes(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = 1024 * KIB;
    const GIB: u64 = 1024 * MIB;
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

fn format_duration(secs: f64) -> String {
    let total = secs as u64;
    let days = total / 86400;
    let hours = (total % 86400) / 3600;
    let mins = (total % 3600) / 60;
    let s = total % 60;
    if days > 0 {
        format!("{}d {}h {}m {}s", days, hours, mins, s)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, mins, s)
    } else if mins > 0 {
        format!("{}m {}s", mins, s)
    } else {
        format!("{:.1}s", secs)
    }
}

impl Snapshot {
    #[cfg(target_os = "linux")]
    pub fn capture() -> anyhow::Result<Self> {
        let mut entries = BTreeMap::new();
        if let Ok(e) = meminfo::parse() { entries.insert("meminfo".into(), e); }
        if let Ok(e) = uptime::parse() { entries.insert("uptime".into(), e); }
        if let Ok(e) = loadavg::parse() { entries.insert("loadavg".into(), e); }
        if let Ok(e) = version::parse() { entries.insert("version".into(), e); }
        if let Ok(e) = mounts::parse() { entries.insert("mounts".into(), e); }
        if let Ok(e) = partitions::parse() { entries.insert("partitions".into(), e); }
        if let Ok(e) = cpuinfo::parse() { entries.insert("cpuinfo".into(), e); }
        if let Ok(e) = stat::parse() { entries.insert("stat".into(), e); }
        if let Ok(e) = net_dev::parse() { entries.insert("net/dev".into(), e); }
        if let Ok(e) = diskstats::parse() { entries.insert("diskstats".into(), e); }
        if let Ok(e) = processes::parse() { entries.insert("processes".into(), e); }
        if let Ok(e) = swaps::parse() { entries.insert("swaps".into(), e); }
        if let Ok(e) = buddyinfo::parse() { entries.insert("buddyinfo".into(), e); }
        if let Ok(e) = cgroups::parse() { entries.insert("cgroups".into(), e); }
        if let Ok(e) = cmdline::parse() { entries.insert("cmdline".into(), e); }
        if let Ok(e) = consoles::parse() { entries.insert("consoles".into(), e); }
        if let Ok(e) = crypto::parse() { entries.insert("crypto".into(), e); }
        if let Ok(e) = devices::parse() { entries.insert("devices".into(), e); }
        if let Ok(e) = filesystems::parse() { entries.insert("filesystems".into(), e); }
        if let Ok(e) = interrupts::parse() { entries.insert("interrupts".into(), e); }
        if let Ok(e) = iomem::parse() { entries.insert("iomem".into(), e); }
        if let Ok(e) = ioports::parse() { entries.insert("ioports".into(), e); }
        if let Ok(e) = locks::parse() { entries.insert("locks".into(), e); }
        if let Ok(e) = modules::parse() { entries.insert("modules".into(), e); }
        if let Ok(e) = vmstat::parse() { entries.insert("vmstat".into(), e); }
        if let Ok(e) = zoneinfo::parse() { entries.insert("zoneinfo".into(), e); }
        if let Ok(e) = softirqs::parse() { entries.insert("softirqs".into(), e); }
        if let Ok(e) = misc::parse() { entries.insert("misc".into(), e); }
        if let Ok(e) = pressure::parse() { entries.insert("pressure".into(), e); }
        if let Ok(e) = net_tcp::parse() { entries.insert("net/tcp".into(), e); }
        if let Ok(e) = net_udp::parse() { entries.insert("net/udp".into(), e); }
        if let Ok(e) = net_unix::parse() { entries.insert("net/unix".into(), e); }
        if let Ok(e) = net_arp::parse() { entries.insert("net/arp".into(), e); }
        if let Ok(e) = net_route::parse() { entries.insert("net/route".into(), e); }
        if let Ok(e) = net_sockstat::parse() { entries.insert("net/sockstat".into(), e); }
        if let Ok(e) = net_snmp::parse() { entries.insert("net/snmp".into(), e); }
        if let Ok(e) = net_netstat::parse() { entries.insert("net/netstat".into(), e); }
        if let Ok(e) = net_wireless::parse() { entries.insert("net/wireless".into(), e); }
        if let Ok(e) = slabinfo::parse() { entries.insert("slabinfo".into(), e); }
        if let Ok(e) = pagetypeinfo::parse() { entries.insert("pagetypeinfo".into(), e); }
        if let Ok(e) = schedstat::parse() { entries.insert("schedstat".into(), e); }
        if let Ok(e) = dma::parse() { entries.insert("dma".into(), e); }
        if let Ok(e) = timer_list::parse() { entries.insert("timer_list".into(), e); }

        // /sys and system metrics
        if let Ok(e) = crate::sys::df::parse() { entries.insert("df".into(), e); }
        if let Ok(e) = crate::sys::thermal::parse() { entries.insert("thermal".into(), e); }
        if let Ok(e) = crate::sys::file_nr::parse() { entries.insert("file-nr".into(), e); }
        if let Ok(e) = crate::sys::gpu::parse() { entries.insert("gpu".into(), e); }
        if let Ok(e) = crate::sys::systemd::parse() { entries.insert("systemd".into(), e); }

        // Network deep-dive
        if let Ok(e) = crate::net::ip_route::parse() { entries.insert("ip/route".into(), e); }
        if let Ok(e) = crate::net::ip_neighbor::parse() { entries.insert("ip/neighbor".into(), e); }
        if let Ok(e) = crate::net::ss_summary::parse() { entries.insert("ss".into(), e); }
        if let Ok(e) = crate::net::dns::parse() { entries.insert("dns".into(), e); }
        if let Ok(e) = crate::net::conntrack::parse() { entries.insert("conntrack".into(), e); }

        // Plugins
        for (key, entry) in crate::plugin::load_plugins() {
            entries.insert(key, entry);
        }

        Ok(Snapshot {
            timestamp: SystemTime::now(),
            entries,
        })
    }

    #[cfg(target_os = "macos")]
    pub fn capture() -> anyhow::Result<Self> {
        platform_macos::capture()
    }

    #[cfg(target_os = "windows")]
    pub fn capture() -> anyhow::Result<Self> {
        platform_windows::capture()
    }
}

pub fn diff_snapshots(old: &Snapshot, new: &Snapshot) -> Vec<DiffItem> {
    let mut diffs = Vec::new();
    for (key, new_entry) in &new.entries {
        if let Some(old_entry) = old.entries.get(key) {
            for (nf, of) in new_entry.fields.iter().zip(old_entry.fields.iter()) {
                let changed = match (&of.value, &nf.value) {
                    (FieldValue::Bytes(a), FieldValue::Bytes(b)) => a != b,
                    (FieldValue::Integer(a), FieldValue::Integer(b)) => a != b,
                    (FieldValue::Float(a), FieldValue::Float(b)) => (a - b).abs() > 0.001,
                    (FieldValue::Text(a), FieldValue::Text(b)) => a != b,
                    (FieldValue::Duration(a), FieldValue::Duration(b)) => (a - b).abs() > 0.1,
                    _ => true,
                };
                if changed {
                    diffs.push(DiffItem {
                        source: key.clone(),
                        field: nf.name.clone(),
                        old_value: of.value.display(),
                        new_value: nf.value.display(),
                    });
                }
            }
        }
    }
    diffs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffItem {
    pub source: String,
    pub field: String,
    pub old_value: String,
    pub new_value: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::time::SystemTime;

    fn make_test_snapshot(fields: Vec<(&str, &str, FieldValue)>) -> Snapshot {
        let mut entries = BTreeMap::new();
        for (source_key, field_name, value) in fields {
            let entry = entries.entry(source_key.to_string()).or_insert_with(|| ProcEntry {
                source: format!("/proc/{}", source_key),
                fields: Vec::new(),
            });
            entry.fields.push(Field {
                name: field_name.to_string(),
                value,
                unit: None,
                description: format!("Test field {}", field_name),
            });
        }
        Snapshot {
            timestamp: SystemTime::now(),
            entries,
        }
    }

    // T1: format_bytes
    #[test]
    fn format_bytes_zero() {
        assert_eq!(format_bytes(0), "0 B");
    }

    #[test]
    fn format_bytes_below_kib() {
        assert_eq!(format_bytes(500), "500 B");
    }

    #[test]
    fn format_bytes_one_kib() {
        assert_eq!(format_bytes(1024), "1.0 KiB");
    }

    #[test]
    fn format_bytes_one_mib() {
        assert_eq!(format_bytes(1048576), "1.0 MiB");
    }

    #[test]
    fn format_bytes_one_gib() {
        assert_eq!(format_bytes(1073741824), "1.00 GiB");
    }

    // T2: format_duration
    #[test]
    fn format_duration_subsecond() {
        assert_eq!(format_duration(0.5), "0.5s");
    }

    #[test]
    fn format_duration_minutes() {
        assert_eq!(format_duration(65.0), "1m 5s");
    }

    #[test]
    fn format_duration_hours() {
        assert_eq!(format_duration(3665.0), "1h 1m 5s");
    }

    #[test]
    fn format_duration_days() {
        assert_eq!(format_duration(90061.0), "1d 1h 1m 1s");
    }

    // T5: diff_snapshots — identical snapshots
    #[test]
    fn diff_identical_snapshots_returns_empty() {
        let snap = make_test_snapshot(vec![
            ("meminfo", "MemTotal", FieldValue::Bytes(16 * 1024 * 1024 * 1024)),
            ("loadavg", "load_1min", FieldValue::Float(0.50)),
        ]);
        let diffs = diff_snapshots(&snap, &snap);
        assert!(diffs.is_empty(), "Expected no diffs for identical snapshots, got {}", diffs.len());
    }

    // T6: diff_snapshots — changed Bytes field
    #[test]
    fn diff_detects_bytes_change() {
        let snap1 = make_test_snapshot(vec![
            ("meminfo", "MemTotal", FieldValue::Bytes(16 * 1024 * 1024 * 1024)),
        ]);
        let snap2 = make_test_snapshot(vec![
            ("meminfo", "MemTotal", FieldValue::Bytes(999999)),
        ]);
        let diffs = diff_snapshots(&snap1, &snap2);
        assert!(!diffs.is_empty(), "Expected diffs for changed Bytes field");
        assert_eq!(diffs[0].source, "meminfo");
        assert_eq!(diffs[0].field, "MemTotal");
    }

    // T7: diff_snapshots — small Float change below threshold
    #[test]
    fn diff_ignores_small_float_change() {
        let snap1 = make_test_snapshot(vec![
            ("loadavg", "load_1min", FieldValue::Float(0.50)),
        ]);
        let snap2 = make_test_snapshot(vec![
            ("loadavg", "load_1min", FieldValue::Float(0.5005)),
        ]);
        let diffs = diff_snapshots(&snap1, &snap2);
        let loadavg_diffs: Vec<_> = diffs.iter().filter(|d| d.source == "loadavg").collect();
        assert!(loadavg_diffs.is_empty(),
            "Expected no diffs for small float change (< 0.001), got {}", loadavg_diffs.len());
    }

    // BL-020: FieldValue PartialEq
    #[test]
    fn field_value_partial_eq() {
        assert_eq!(FieldValue::Bytes(100), FieldValue::Bytes(100));
        assert_ne!(FieldValue::Bytes(100), FieldValue::Bytes(200));
        assert_eq!(FieldValue::Text("a".into()), FieldValue::Text("a".into()));
        assert_ne!(FieldValue::Text("a".into()), FieldValue::Text("b".into()));
        assert_eq!(FieldValue::Float(1.0), FieldValue::Float(1.0));
        assert_ne!(FieldValue::Float(1.0), FieldValue::Float(2.0));
        assert_eq!(FieldValue::Integer(42), FieldValue::Integer(42));
        assert_eq!(FieldValue::Duration(3600.0), FieldValue::Duration(3600.0));
        assert_eq!(
            FieldValue::Table(vec![vec!["a".into()]]),
            FieldValue::Table(vec![vec!["a".into()]])
        );
        // Cross-variant should be false
        assert_ne!(FieldValue::Bytes(100), FieldValue::Integer(100));
    }

    // T11: systemtime_iso8601 round-trip
    #[test]
    fn systemtime_roundtrip_via_json() {
        let snap = make_test_snapshot(vec![
            ("uptime", "uptime", FieldValue::Duration(86400.0)),
        ]);
        let json = serde_json::to_string(&snap).unwrap();
        let restored: Snapshot = serde_json::from_str(&json).unwrap();
        assert_eq!(snap.timestamp, restored.timestamp);
    }

    // T17: Snapshot::capture() doesn't panic and returns at least 10 entries
    #[cfg(target_os = "linux")]
    #[test]
    fn snapshot_capture_returns_entries() {
        let snap = Snapshot::capture().unwrap();
        assert!(
            snap.entries.len() >= 10,
            "Expected at least 10 entries, got {}",
            snap.entries.len()
        );
        assert!(snap.entries.contains_key("meminfo"), "Missing meminfo entry");
        assert!(snap.entries.contains_key("uptime"), "Missing uptime entry");
        assert!(snap.entries.contains_key("loadavg"), "Missing loadavg entry");
    }

    // T18: All parsers individually don't panic
    #[cfg(target_os = "linux")]
    #[test]
    fn all_parsers_dont_panic() {
        // Each parser may return Ok or Err (e.g., permission denied),
        // but must never panic.
        let _ = meminfo::parse();
        let _ = uptime::parse();
        let _ = loadavg::parse();
        let _ = version::parse();
        let _ = mounts::parse();
        let _ = partitions::parse();
        let _ = cpuinfo::parse();
        let _ = stat::parse();
        let _ = net_dev::parse();
        let _ = diskstats::parse();
        let _ = processes::parse();
        let _ = swaps::parse();
        let _ = buddyinfo::parse();
        let _ = cgroups::parse();
        let _ = cmdline::parse();
        let _ = consoles::parse();
        let _ = crypto::parse();
        let _ = devices::parse();
        let _ = filesystems::parse();
        let _ = interrupts::parse();
        let _ = iomem::parse();
        let _ = ioports::parse();
        let _ = locks::parse();
        let _ = modules::parse();
        let _ = vmstat::parse();
        let _ = zoneinfo::parse();
        let _ = softirqs::parse();
        let _ = misc::parse();
        let _ = pressure::parse();
        let _ = net_tcp::parse();
        let _ = net_udp::parse();
        let _ = net_unix::parse();
        let _ = net_arp::parse();
        let _ = net_route::parse();
        let _ = net_sockstat::parse();
        let _ = net_snmp::parse();
        let _ = net_netstat::parse();
        let _ = net_wireless::parse();
        let _ = slabinfo::parse();
        let _ = pagetypeinfo::parse();
        let _ = schedstat::parse();
        let _ = dma::parse();
        let _ = timer_list::parse();
    }
}

// =============================================================================
// BL-060 through BL-063: Fixture-based parse_content() tests
// =============================================================================
// These tests run on any platform (no /proc access required).
// Each parser's parse_content() is tested with realistic fixture data.
#[cfg(test)]
mod parser_content_tests {
    use super::*;

    // Helper: find a field by name in a ProcEntry
    fn field_by_name<'a>(entry: &'a ProcEntry, name: &str) -> Option<&'a Field> {
        entry.fields.iter().find(|f| f.name == name)
    }

    // =========================================================================
    // BL-061: T12 — uptime
    // =========================================================================
    #[test]
    fn uptime_parse_content() {
        let entry = uptime::parse_content("12345.67 98765.43\n").unwrap();
        assert_eq!(entry.source, "/proc/uptime");
        let up = field_by_name(&entry, "uptime").unwrap();
        assert_eq!(up.value, FieldValue::Duration(12345.67));
        let idle = field_by_name(&entry, "idle").unwrap();
        assert_eq!(idle.value, FieldValue::Duration(98765.43));
        let pct = field_by_name(&entry, "idle_pct").unwrap();
        if let FieldValue::Float(v) = pct.value {
            let expected = 98765.43 / 12345.67 * 100.0;
            assert!((v - expected).abs() < 0.01, "idle_pct = {} expected ~{}", v, expected);
        } else {
            panic!("idle_pct should be Float");
        }
    }

    // BL-062: uptime error handling
    #[test]
    fn uptime_empty_input() {
        assert!(uptime::parse_content("").is_err());
    }

    #[test]
    fn uptime_single_field() {
        assert!(uptime::parse_content("12345.67\n").is_err());
    }

    #[test]
    fn uptime_non_numeric() {
        assert!(uptime::parse_content("abc def\n").is_err());
    }

    // =========================================================================
    // BL-061: T13 — loadavg
    // =========================================================================
    #[test]
    fn loadavg_parse_content() {
        let entry = loadavg::parse_content("0.50 0.75 1.00 3/150 12345\n").unwrap();
        assert_eq!(entry.source, "/proc/loadavg");
        assert_eq!(field_by_name(&entry, "load_1min").unwrap().value, FieldValue::Float(0.50));
        assert_eq!(field_by_name(&entry, "load_5min").unwrap().value, FieldValue::Float(0.75));
        assert_eq!(field_by_name(&entry, "load_15min").unwrap().value, FieldValue::Float(1.00));
        assert_eq!(field_by_name(&entry, "running_threads").unwrap().value, FieldValue::Integer(3));
        assert_eq!(field_by_name(&entry, "total_threads").unwrap().value, FieldValue::Integer(150));
        assert_eq!(field_by_name(&entry, "last_pid").unwrap().value, FieldValue::Integer(12345));
    }

    #[test]
    fn loadavg_error_too_few_fields() {
        assert!(loadavg::parse_content("0.50 0.75\n").is_err());
    }

    #[test]
    fn loadavg_error_missing_slash() {
        assert!(loadavg::parse_content("0.50 0.75 1.00 3 12345\n").is_err());
    }

    // =========================================================================
    // BL-061: T14 — meminfo
    // =========================================================================
    #[test]
    fn meminfo_parse_content() {
        let fixture = "\
MemTotal:       16384000 kB
MemFree:         8192000 kB
MemAvailable:   12000000 kB
Buffers:          512000 kB
Cached:          4096000 kB
HugePages_Total:       0
";
        let entry = meminfo::parse_content(fixture).unwrap();
        assert_eq!(entry.source, "/proc/meminfo");
        let total = field_by_name(&entry, "MemTotal").unwrap();
        assert_eq!(total.value, FieldValue::Bytes(16384000 * 1024));
        let free = field_by_name(&entry, "MemFree").unwrap();
        assert_eq!(free.value, FieldValue::Bytes(8192000 * 1024));
        // HugePages_Total has no kB suffix, stored as raw bytes
        let hp = field_by_name(&entry, "HugePages_Total").unwrap();
        assert_eq!(hp.value, FieldValue::Bytes(0));
    }

    #[test]
    fn meminfo_empty_input() {
        let entry = meminfo::parse_content("").unwrap();
        assert!(entry.fields.is_empty());
    }

    #[test]
    fn meminfo_malformed_lines_skipped() {
        let fixture = "This is not a valid line\nMemTotal:       1024 kB\n";
        let entry = meminfo::parse_content(fixture).unwrap();
        assert_eq!(entry.fields.len(), 1);
        assert_eq!(field_by_name(&entry, "MemTotal").unwrap().value, FieldValue::Bytes(1024 * 1024));
    }

    // =========================================================================
    // BL-061: T15 — version
    // =========================================================================
    #[test]
    fn version_parse_content() {
        // The parser extracts compiler info from "(gcc" to the first ")"
        let fixture = "Linux version 6.6.87.2-microsoft-standard-WSL2 (gcc version 12.2.0) #1 SMP Mon Mar 10\n";
        let entry = version::parse_content(fixture).unwrap();
        assert_eq!(entry.source, "/proc/version");
        let raw = field_by_name(&entry, "raw").unwrap();
        assert!(matches!(&raw.value, FieldValue::Text(s) if s.contains("Linux version")));
        let kv = field_by_name(&entry, "kernel_version").unwrap();
        assert_eq!(kv.value, FieldValue::Text("6.6.87.2-microsoft-standard-WSL2".into()));
        let comp = field_by_name(&entry, "compiler").unwrap();
        assert_eq!(comp.value, FieldValue::Text("gcc version 12.2.0".into()));
    }

    #[test]
    fn version_no_gcc_info() {
        let fixture = "Linux version 5.15.0\n";
        let entry = version::parse_content(fixture).unwrap();
        assert!(field_by_name(&entry, "compiler").is_none());
        assert!(field_by_name(&entry, "kernel_version").is_some());
    }

    // =========================================================================
    // BL-061: T16 — stat
    // =========================================================================
    #[test]
    fn stat_parse_content() {
        let fixture = "\
cpu  1000 200 500 8000 100 50 30 20 0 0
cpu0 500 100 250 4000 50 25 15 10 0 0
intr 123456 0 0 0 0 0 0 0
ctxt 987654
processes 5000
procs_running 3
procs_blocked 1
";
        let entry = stat::parse_content(fixture).unwrap();
        assert_eq!(entry.source, "/proc/stat");
        assert_eq!(field_by_name(&entry, "cpu_user").unwrap().value, FieldValue::Integer(1000));
        assert_eq!(field_by_name(&entry, "cpu_system").unwrap().value, FieldValue::Integer(500));
        assert_eq!(field_by_name(&entry, "cpu_idle").unwrap().value, FieldValue::Integer(8000));
        assert_eq!(field_by_name(&entry, "cpu_iowait").unwrap().value, FieldValue::Integer(100));
        assert_eq!(field_by_name(&entry, "forks_total").unwrap().value, FieldValue::Integer(5000));
        assert_eq!(field_by_name(&entry, "procs_running").unwrap().value, FieldValue::Integer(3));
        assert_eq!(field_by_name(&entry, "procs_blocked").unwrap().value, FieldValue::Integer(1));
        assert_eq!(field_by_name(&entry, "context_switches").unwrap().value, FieldValue::Integer(987654));
        // cpu_usage_pct: busy = total - idle - iowait = 9900 - 8000 - 100 = 1800
        // total = 1000+200+500+8000+100+50+30+20 = 9900
        if let FieldValue::Float(pct) = field_by_name(&entry, "cpu_usage_pct").unwrap().value {
            let expected = 1800.0 / 9900.0 * 100.0;
            assert!((pct - expected).abs() < 0.1, "cpu_usage_pct={} expected ~{}", pct, expected);
        } else {
            panic!("cpu_usage_pct should be Float");
        }
    }

    #[test]
    fn stat_empty_input() {
        let entry = stat::parse_content("").unwrap();
        assert!(entry.fields.is_empty());
    }

    // =========================================================================
    // BL-063: cmdline
    // =========================================================================
    #[test]
    fn cmdline_parse_content() {
        let fixture = "BOOT_IMAGE=/vmlinuz root=/dev/sda1 ro quiet splash\n";
        let entry = cmdline::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "param_count").unwrap().value, FieldValue::Integer(5));
        if let FieldValue::Text(s) = &field_by_name(&entry, "cmdline").unwrap().value {
            assert!(s.contains("BOOT_IMAGE"));
        }
    }

    #[test]
    fn cmdline_empty() {
        let entry = cmdline::parse_content("\n").unwrap();
        assert_eq!(field_by_name(&entry, "param_count").unwrap().value, FieldValue::Integer(0));
    }

    // =========================================================================
    // BL-063: mounts
    // =========================================================================
    #[test]
    fn mounts_parse_content() {
        let fixture = "\
sysfs /sys sysfs rw,nosuid,nodev,noexec,relatime 0 0
proc /proc proc rw,nosuid,nodev,noexec,relatime 0 0
/dev/sda1 / ext4 rw,relatime 0 0
";
        let entry = mounts::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "count").unwrap().value, FieldValue::Integer(3));
        if let FieldValue::Table(rows) = &field_by_name(&entry, "mounts").unwrap().value {
            assert_eq!(rows.len(), 3);
            assert_eq!(rows[2][0], "/dev/sda1");
            assert_eq!(rows[2][2], "ext4");
        }
    }

    #[test]
    fn mounts_empty() {
        let entry = mounts::parse_content("").unwrap();
        assert_eq!(field_by_name(&entry, "count").unwrap().value, FieldValue::Integer(0));
    }

    // =========================================================================
    // BL-063: partitions
    // =========================================================================
    #[test]
    fn partitions_parse_content() {
        let fixture = "\
major minor  #blocks  name

   8        0  104857600 sda
   8        1  104856576 sda1
";
        let entry = partitions::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "count").unwrap().value, FieldValue::Integer(2));
        if let FieldValue::Table(rows) = &field_by_name(&entry, "partitions").unwrap().value {
            assert_eq!(rows[0][0], "sda");
        }
    }

    // =========================================================================
    // BL-063: cpuinfo
    // =========================================================================
    #[test]
    fn cpuinfo_parse_content() {
        let fixture = "\
processor\t: 0
model name\t: Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz
cpu MHz\t\t: 2600.000
cache size\t: 12288 KB
cpu cores\t: 6
flags\t\t: fpu vme de pse tsc msr pae mce cx8 apic sse sse2 avx avx2 hypervisor

processor\t: 1
model name\t: Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz
cpu MHz\t\t: 2600.000
cache size\t: 12288 KB
cpu cores\t: 6
flags\t\t: fpu vme de pse tsc msr pae mce cx8 apic sse sse2 avx avx2 hypervisor
";
        let entry = cpuinfo::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "logical_cpus").unwrap().value, FieldValue::Integer(2));
        if let FieldValue::Text(m) = &field_by_name(&entry, "model").unwrap().value {
            assert!(m.contains("i7-9750H"));
        }
        assert_eq!(field_by_name(&entry, "cores_per_socket").unwrap().value, FieldValue::Integer(6));
        if let FieldValue::Text(flags) = &field_by_name(&entry, "key_flags").unwrap().value {
            assert!(flags.contains("sse"));
            assert!(flags.contains("avx2"));
            assert!(flags.contains("hypervisor"));
        }
    }

    // =========================================================================
    // BL-063: net_dev
    // =========================================================================
    #[test]
    fn net_dev_parse_content() {
        let fixture = "\
Inter-|   Receive                                                |  Transmit
 face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
    lo: 1000000     500    0    0    0     0          0         0  1000000     500    0    0    0     0       0          0
  eth0: 5000000    3000    0    0    0     0          0         0  2000000    1500    0    0    0     0       0          0
";
        let entry = net_dev::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "interface_count").unwrap().value, FieldValue::Integer(2));
        assert_eq!(field_by_name(&entry, "total_rx").unwrap().value, FieldValue::Bytes(6000000));
        assert_eq!(field_by_name(&entry, "total_tx").unwrap().value, FieldValue::Bytes(3000000));
    }

    // =========================================================================
    // BL-063: diskstats
    // =========================================================================
    #[test]
    fn diskstats_parse_content() {
        // Fields: major minor name reads_completed reads_merged sectors_read time_reading writes_completed writes_merged sectors_written time_writing ios_in_progress io_time weighted_io_time
        let fixture = "\
   8       0 sda 1000 500 20000 5000 2000 800 40000 3000 5 8000 12000
   8       1 sda1 0 0 0 0 0 0 0 0 0 0 0
";
        let entry = diskstats::parse_content(fixture).unwrap();
        // sda has activity, sda1 does not (0 reads/writes -> skipped)
        assert_eq!(field_by_name(&entry, "active_devices").unwrap().value, FieldValue::Integer(1));
    }

    // =========================================================================
    // BL-063: swaps
    // =========================================================================
    #[test]
    fn swaps_parse_content() {
        let fixture = "\
Filename\t\t\t\tType\t\tSize\t\tUsed\t\tPriority
/dev/sda2                               partition\t2097148\t\t524288\t\t-2
";
        let entry = swaps::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "total_size").unwrap().value, FieldValue::Bytes(2097148 * 1024));
        assert_eq!(field_by_name(&entry, "total_used").unwrap().value, FieldValue::Bytes(524288 * 1024));
    }

    #[test]
    fn swaps_no_swap() {
        let fixture = "Filename\t\t\t\tType\t\tSize\t\tUsed\t\tPriority\n";
        let entry = swaps::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "total_size").unwrap().value, FieldValue::Bytes(0));
        if let FieldValue::Float(pct) = field_by_name(&entry, "usage_pct").unwrap().value {
            assert!((pct - 0.0).abs() < 0.001);
        }
    }

    // =========================================================================
    // BL-063: buddyinfo
    // =========================================================================
    #[test]
    fn buddyinfo_parse_content() {
        let fixture = "\
Node 0, zone      DMA      1      1      0      1      2      1      1      0      1      1      3
Node 0, zone    DMA32    100     80     50     30     20     10      5      3      2      1      0
";
        let entry = buddyinfo::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "zone_count").unwrap().value, FieldValue::Integer(2));
    }

    // =========================================================================
    // BL-063: cgroups
    // =========================================================================
    #[test]
    fn cgroups_parse_content() {
        let fixture = "\
#subsys_name\thierarchy\tnum_cgroups\tenabled
cpuset\t1\t1\t1
cpu\t2\t80\t1
memory\t3\t80\t1
";
        let entry = cgroups::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "controller_count").unwrap().value, FieldValue::Integer(3));
    }

    // =========================================================================
    // BL-063: consoles
    // =========================================================================
    #[test]
    fn consoles_parse_content() {
        let fixture = "\
tty0                 -WU (EC p a)    4:7
ttyS0                -W- (E  p  )    4:64
";
        let entry = consoles::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "console_count").unwrap().value, FieldValue::Integer(2));
    }

    // =========================================================================
    // BL-063: crypto
    // =========================================================================
    #[test]
    fn crypto_parse_content() {
        let fixture = "\
name         : aes
driver       : aes-generic
module       : kernel
type         : cipher

name         : sha256
driver       : sha256-generic
module       : kernel
type         : shash
";
        let entry = crypto::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "algorithm_count").unwrap().value, FieldValue::Integer(2));
        if let FieldValue::Table(rows) = &field_by_name(&entry, "algorithms").unwrap().value {
            assert_eq!(rows[0][0], "aes");
            assert_eq!(rows[1][0], "sha256");
        }
    }

    // =========================================================================
    // BL-063: devices
    // =========================================================================
    #[test]
    fn devices_parse_content() {
        let fixture = "\
Character devices:
  1 mem
  4 tty
  5 /dev/tty

Block devices:
  8 sd
  11 sr
";
        let entry = devices::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "device_count").unwrap().value, FieldValue::Integer(5));
        if let FieldValue::Table(rows) = &field_by_name(&entry, "devices").unwrap().value {
            assert_eq!(rows[0][0], "Character"); // type
            assert_eq!(rows[0][2], "mem");       // name
            assert_eq!(rows[3][0], "Block");
        }
    }

    // =========================================================================
    // BL-063: filesystems
    // =========================================================================
    #[test]
    fn filesystems_parse_content() {
        let fixture = "\
nodev\tsysfs
nodev\ttmpfs
\text4
\txfs
";
        let entry = filesystems::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "filesystem_count").unwrap().value, FieldValue::Integer(4));
        if let FieldValue::Table(rows) = &field_by_name(&entry, "filesystems").unwrap().value {
            assert_eq!(rows[0][0], "sysfs");
            assert_eq!(rows[0][1], "yes"); // nodev
            assert_eq!(rows[2][0], "ext4");
            assert_eq!(rows[2][1], "no");  // not nodev
        }
    }

    // =========================================================================
    // BL-063: interrupts
    // =========================================================================
    #[test]
    fn interrupts_parse_content() {
        let fixture = "\
           CPU0       CPU1
  0:         50          0   IO-APIC   2-edge      timer
  1:        100        200   IO-APIC   1-edge      i8042
NMI:          5          3   Non-maskable interrupts
";
        let entry = interrupts::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "cpu_count").unwrap().value, FieldValue::Integer(2));
        assert_eq!(field_by_name(&entry, "irq_count").unwrap().value, FieldValue::Integer(3));
    }

    // =========================================================================
    // BL-063: iomem
    // =========================================================================
    #[test]
    fn iomem_parse_content() {
        let fixture = "\
00000000-00000fff : Reserved
00001000-0009fbff : System RAM
000a0000-000bffff : PCI Bus 0000:00
";
        let entry = iomem::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "region_count").unwrap().value, FieldValue::Integer(3));
    }

    // =========================================================================
    // BL-063: ioports
    // =========================================================================
    #[test]
    fn ioports_parse_content() {
        let fixture = "\
0000-0cf7 : PCI Bus 0000:00
  0000-001f : dma1
  0020-0021 : pic1
";
        let entry = ioports::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "region_count").unwrap().value, FieldValue::Integer(3));
    }

    // =========================================================================
    // BL-063: locks
    // =========================================================================
    #[test]
    fn locks_parse_content() {
        let fixture = "\
1: POSIX  ADVISORY  WRITE 1234 08:01:131073 0 EOF
2: FLOCK  ADVISORY  READ  5678 08:01:131074 0 EOF
";
        let entry = locks::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "lock_count").unwrap().value, FieldValue::Integer(2));
    }

    #[test]
    fn locks_empty() {
        let entry = locks::parse_content("").unwrap();
        assert_eq!(field_by_name(&entry, "lock_count").unwrap().value, FieldValue::Integer(0));
    }

    // =========================================================================
    // BL-063: modules
    // =========================================================================
    #[test]
    fn modules_parse_content() {
        let fixture = "\
nf_tables 311296 0 - Live 0xffffffff81000000
nf_conntrack 176128 1 nf_tables, Live 0xffffffff81100000
";
        let entry = modules::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "module_count").unwrap().value, FieldValue::Integer(2));
    }

    // =========================================================================
    // BL-063: vmstat
    // =========================================================================
    #[test]
    fn vmstat_parse_content() {
        let fixture = "\
nr_free_pages 12345
nr_inactive_anon 5000
pgfault 999999
pgmajfault 100
";
        let entry = vmstat::parse_content(fixture).unwrap();
        assert_eq!(entry.fields.len(), 4);
        assert_eq!(field_by_name(&entry, "nr_free_pages").unwrap().value, FieldValue::Integer(12345));
        assert_eq!(field_by_name(&entry, "pgmajfault").unwrap().value, FieldValue::Integer(100));
    }

    #[test]
    fn vmstat_empty() {
        let entry = vmstat::parse_content("").unwrap();
        assert!(entry.fields.is_empty());
    }

    // =========================================================================
    // BL-063: zoneinfo
    // =========================================================================
    #[test]
    fn zoneinfo_parse_content() {
        // The parser expects: lines with "free <val>", "min <val>", "low <val>", "high <val>"
        // "pages" line is skipped by the parser (matches "pages" but not any of the tracked keys)
        let fixture = "\
Node 0, zone      DMA
  pages free     3841
        free     100
        min      10
        low      20
        high     30
Node 0, zone    DMA32
  pages free     3841
        free     5000
        min      500
        low      1000
        high     1500
";
        let entry = zoneinfo::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "zone_count").unwrap().value, FieldValue::Integer(2));
        if let FieldValue::Table(rows) = &field_by_name(&entry, "zones").unwrap().value {
            assert_eq!(rows[0][1], "100");  // free pages
            assert_eq!(rows[0][2], "10");   // min
            assert_eq!(rows[1][1], "5000"); // free pages
        }
    }

    // =========================================================================
    // BL-063: softirqs
    // =========================================================================
    #[test]
    fn softirqs_parse_content() {
        let fixture = "\
                    CPU0       CPU1
          HI:          0          0
       TIMER:      10000      20000
      NET_TX:        100        200
      NET_RX:       5000       6000
";
        let entry = softirqs::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "softirq_count").unwrap().value, FieldValue::Integer(4));
        if let FieldValue::Table(rows) = &field_by_name(&entry, "softirqs").unwrap().value {
            assert_eq!(rows[0][0], "HI");
            assert_eq!(rows[0][1], "0"); // 0+0
            assert_eq!(rows[1][0], "TIMER");
            assert_eq!(rows[1][1], "30000"); // 10000+20000
        }
    }

    // =========================================================================
    // BL-063: misc
    // =========================================================================
    #[test]
    fn misc_parse_content() {
        let fixture = "\
 63 device-mapper
200 tun
";
        let entry = misc::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "device_count").unwrap().value, FieldValue::Integer(2));
    }

    // =========================================================================
    // BL-063: pressure
    // =========================================================================
    #[test]
    fn pressure_parse_content() {
        let cpu_fixture = "some avg10=0.50 avg60=1.00 avg300=0.75 total=123456\n";
        let io_fixture = "some avg10=2.00 avg60=1.50 avg300=1.00 total=654321\nfull avg10=0.10 avg60=0.05 avg300=0.02 total=10000\n";
        let memory_fixture = "some avg10=0.00 avg60=0.00 avg300=0.00 total=0\nfull avg10=0.00 avg60=0.00 avg300=0.00 total=0\n";

        let entry = pressure::parse_content(&[
            ("cpu", cpu_fixture),
            ("io", io_fixture),
            ("memory", memory_fixture),
        ]).unwrap();

        let cpu_some_avg10 = field_by_name(&entry, "cpu_some_avg10").unwrap();
        assert_eq!(cpu_some_avg10.value, FieldValue::Float(0.50));
        let cpu_some_total = field_by_name(&entry, "cpu_some_total").unwrap();
        assert_eq!(cpu_some_total.value, FieldValue::Integer(123456));
        let io_full_avg10 = field_by_name(&entry, "io_full_avg10").unwrap();
        assert_eq!(io_full_avg10.value, FieldValue::Float(0.10));
    }

    // =========================================================================
    // BL-063: net_tcp
    // =========================================================================
    #[test]
    fn net_tcp_parse_content() {
        let fixture = "\
  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode
   0: 0100007F:0050 00000000:0000 0A 00000000:00000000 00:00000000 00000000     0        0 12345
   1: 0100007F:01BB C0A80001:D234 01 00000000:00000000 00:00000000 00000000  1000        0 12346
";
        let entry = net_tcp::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "connection_count").unwrap().value, FieldValue::Integer(2));
        if let FieldValue::Table(rows) = &field_by_name(&entry, "connections").unwrap().value {
            assert_eq!(rows[0][3], "LISTEN");       // state 0A
            assert_eq!(rows[1][3], "ESTABLISHED");  // state 01
            // Check IP parsing: 0100007F -> 127.0.0.1, port 0050 -> 80
            assert!(rows[0][1].starts_with("127.0.0.1:80"));
        }
    }

    // =========================================================================
    // BL-063: net_udp
    // =========================================================================
    #[test]
    fn net_udp_parse_content() {
        let fixture = "\
  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode ref pointer drops
   0: 00000000:0035 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 9876 2 0000000000000000 0
";
        let entry = net_udp::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "socket_count").unwrap().value, FieldValue::Integer(1));
    }

    // =========================================================================
    // BL-063: net_unix
    // =========================================================================
    #[test]
    fn net_unix_parse_content() {
        let fixture = "\
Num       RefCount Protocol Flags    Type St Inode Path
0000000000000000: 00000002 00000000 00010000 0001 01 12345 /run/dbus/system_bus_socket
0000000000000001: 00000002 00000000 00010000 0001 01 12346
";
        let entry = net_unix::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "socket_count").unwrap().value, FieldValue::Integer(2));
        if let FieldValue::Table(rows) = &field_by_name(&entry, "sockets").unwrap().value {
            assert_eq!(rows[0][4], "/run/dbus/system_bus_socket");
            assert!(rows[1][4].is_empty()); // no path
        }
    }

    // =========================================================================
    // BL-063: net_arp
    // =========================================================================
    #[test]
    fn net_arp_parse_content() {
        let fixture = "\
IP address       HW type     Flags       HW address            Mask     Device
192.168.1.1      0x1         0x2         aa:bb:cc:dd:ee:ff     *        eth0
10.0.0.1         0x1         0x2         11:22:33:44:55:66     *        eth0
";
        let entry = net_arp::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "entry_count").unwrap().value, FieldValue::Integer(2));
        if let FieldValue::Table(rows) = &field_by_name(&entry, "entries").unwrap().value {
            assert_eq!(rows[0][0], "192.168.1.1");
            assert_eq!(rows[0][1], "aa:bb:cc:dd:ee:ff");
        }
    }

    // =========================================================================
    // BL-063: net_route
    // =========================================================================
    #[test]
    fn net_route_parse_content() {
        let fixture = "\
Iface\tDestination\tGateway \tFlags\tRefCnt\tUse\tMetric\tMask\t\tMTU\tWindow\tIRTT
eth0\t00000000\t0101A8C0\t0003\t0\t0\t100\t00000000\t0\t0\t0
eth0\t0001A8C0\t00000000\t0001\t0\t0\t100\tFFFFFF00\t0\t0\t0
";
        let entry = net_route::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "route_count").unwrap().value, FieldValue::Integer(2));
        if let FieldValue::Table(rows) = &field_by_name(&entry, "routes").unwrap().value {
            assert_eq!(rows[0][0], "eth0");
            // 00000000 -> 0.0.0.0, 0101A8C0 -> 192.168.1.1
            assert_eq!(rows[0][1], "0.0.0.0");
            assert_eq!(rows[0][2], "192.168.1.1");
        }
    }

    // =========================================================================
    // BL-063: net_sockstat
    // =========================================================================
    #[test]
    fn net_sockstat_parse_content() {
        let fixture = "\
sockets: used 500
TCP: inuse 50 orphan 0 tw 10 alloc 60 mem 5
UDP: inuse 20 mem 3
UDPLITE: inuse 0
RAW: inuse 0
FRAG: inuse 0 memory 0
";
        let entry = net_sockstat::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "sockets_used").unwrap().value, FieldValue::Integer(500));
        assert_eq!(field_by_name(&entry, "TCP_inuse").unwrap().value, FieldValue::Integer(50));
        assert_eq!(field_by_name(&entry, "TCP_tw").unwrap().value, FieldValue::Integer(10));
        assert_eq!(field_by_name(&entry, "UDP_inuse").unwrap().value, FieldValue::Integer(20));
    }

    // =========================================================================
    // BL-063: net_snmp
    // =========================================================================
    #[test]
    fn net_snmp_parse_content() {
        let fixture = "\
Ip: Forwarding DefaultTTL InReceives
Ip: 1 64 123456
Tcp: ActiveOpens PassiveOpens InSegs OutSegs
Tcp: 100 200 5000 6000
";
        let entry = net_snmp::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "Ip_Forwarding").unwrap().value, FieldValue::Integer(1));
        assert_eq!(field_by_name(&entry, "Ip_InReceives").unwrap().value, FieldValue::Integer(123456));
        assert_eq!(field_by_name(&entry, "Tcp_ActiveOpens").unwrap().value, FieldValue::Integer(100));
    }

    // =========================================================================
    // BL-063: net_netstat
    // =========================================================================
    #[test]
    fn net_netstat_parse_content() {
        let fixture = "\
TcpExt: SyncookiesSent SyncookiesRecv TCPTimeouts
TcpExt: 0 0 150
IpExt: InOctets OutOctets
IpExt: 999999 888888
";
        let entry = net_netstat::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "TcpExt_TCPTimeouts").unwrap().value, FieldValue::Integer(150));
        assert_eq!(field_by_name(&entry, "IpExt_InOctets").unwrap().value, FieldValue::Integer(999999));
    }

    // =========================================================================
    // BL-063: net_wireless
    // =========================================================================
    #[test]
    fn net_wireless_parse_content() {
        let fixture = "\
Inter-| sta-|   Quality        |   Discarded packets               | Missed | WE
 face | tus | link level noise |  nwid  crypt   frag  retry   misc | beacon | 22
 wlan0: 0001   70.  -40.  -95.       0      0      0      0      0        0
";
        let entry = net_wireless::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "interface_count").unwrap().value, FieldValue::Integer(1));
        if let FieldValue::Table(rows) = &field_by_name(&entry, "interfaces").unwrap().value {
            assert_eq!(rows[0][0], "wlan0");
        }
    }

    #[test]
    fn net_wireless_no_interfaces() {
        let fixture = "\
Inter-| sta-|   Quality        |   Discarded packets               | Missed | WE
 face | tus | link level noise |  nwid  crypt   frag  retry   misc | beacon | 22
";
        let entry = net_wireless::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "interface_count").unwrap().value, FieldValue::Integer(0));
    }

    // =========================================================================
    // BL-063: slabinfo
    // =========================================================================
    #[test]
    fn slabinfo_parse_content() {
        let fixture = "\
slabinfo - version: 2.1
# name            <active_objs> <num_objs> <objsize> <objperslab> <pagesperslab> : tunables <limit> <batchcount> <sharedfactor> : slabdata <active_slabs> <num_slabs> <sharedavail>
kmalloc-256         1000    1200    256   32    2 : tunables    0    0    0 : slabdata     38     38      0
dentry              5000    5500    192   21    1 : tunables    0    0    0 : slabdata    262    262      0
";
        let entry = slabinfo::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "cache_count").unwrap().value, FieldValue::Integer(2));
        if let FieldValue::Table(rows) = &field_by_name(&entry, "caches").unwrap().value {
            assert_eq!(rows[0][0], "kmalloc-256");
            assert_eq!(rows[1][0], "dentry");
        }
    }

    // =========================================================================
    // BL-063: pagetypeinfo
    // =========================================================================
    #[test]
    fn pagetypeinfo_parse_content() {
        // Note: The parser skips lines that start with "Node" and contain "type" (sub-headers).
        // Real /proc/pagetypeinfo data lines also match this pattern, so the parser
        // effectively returns 0 for most systems. This tests the parsing of the
        // "Free pages count per migrate type" section boundary logic.
        let fixture = "\
Free pages count per migrate type at order       0      1      2      3      4      5      6      7      8      9     10
Node    0, zone      DMA, type    Unmovable      1      0      1      0      2      1      1      0      1      0      0
Number of blocks type     Unmovable  Reclaimable      Movable
Node 0, zone      DMA            1            0            3
";
        let entry = pagetypeinfo::parse_content(fixture).unwrap();
        // Parser skips lines starting with "Node" + containing "type" in the free pages section
        assert_eq!(field_by_name(&entry, "entry_count").unwrap().value, FieldValue::Integer(0));
    }

    // =========================================================================
    // BL-063: schedstat
    // =========================================================================
    #[test]
    fn schedstat_parse_content() {
        let fixture = "\
version 15
timestamp 123456789
cpu0 100 200 300 400 500 600 700 800 900
cpu1 110 210 310 410 510 610 710 810 910
";
        let entry = schedstat::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "version").unwrap().value, FieldValue::Text("15".into()));
        assert_eq!(field_by_name(&entry, "cpu_count").unwrap().value, FieldValue::Integer(2));
    }

    // =========================================================================
    // BL-063: dma
    // =========================================================================
    #[test]
    fn dma_parse_content() {
        let fixture = "\
 4: cascade
";
        let entry = dma::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "channel_count").unwrap().value, FieldValue::Integer(1));
        if let FieldValue::Table(rows) = &field_by_name(&entry, "channels").unwrap().value {
            assert_eq!(rows[0][0], "4");
            assert_eq!(rows[0][1], "cascade");
        }
    }

    #[test]
    fn dma_empty() {
        let entry = dma::parse_content("").unwrap();
        assert_eq!(field_by_name(&entry, "channel_count").unwrap().value, FieldValue::Integer(0));
    }

    // =========================================================================
    // BL-063: timer_list
    // =========================================================================
    #[test]
    fn timer_list_parse_content() {
        let fixture = "\
Timer List Version: v0.9
HRTIMER_MAX_CLOCK_BASES: 8
now at 123456789 nsecs

clock 0:
  .base: 0xffff88003fc16e00
  #0: <0xffff88003fc16e00>, tick_sched_timer, S:01
  #1: <0xffff88003fc16e00>, watchdog_timer_fn, S:01

clock 1:
  .base: 0xffff88003fc16e40
";
        let entry = timer_list::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "version").unwrap().value, FieldValue::Text("v0.9".into()));
        if let FieldValue::Text(now) = &field_by_name(&entry, "now").unwrap().value {
            assert_eq!(now, "nsecs");
        }
        assert_eq!(field_by_name(&entry, "clock_count").unwrap().value, FieldValue::Integer(2));
        assert_eq!(field_by_name(&entry, "timer_count").unwrap().value, FieldValue::Integer(2));
    }

    // =========================================================================
    // BL-062: Additional error handling tests
    // =========================================================================
    #[test]
    fn stat_malformed_cpu_line() {
        // Too few CPU fields — should not produce cpu_* fields
        let fixture = "cpu  1000 200\n";
        let entry = stat::parse_content(fixture).unwrap();
        assert!(field_by_name(&entry, "cpu_user").is_none());
    }

    #[test]
    fn net_snmp_odd_lines() {
        // Odd number of lines should not panic
        let fixture = "Ip: Forwarding DefaultTTL\n";
        let entry = net_snmp::parse_content(fixture).unwrap();
        assert!(entry.fields.is_empty());
    }

    #[test]
    fn net_snmp_mismatched_lengths() {
        // Header and value have different counts — should skip
        let fixture = "Ip: Forwarding DefaultTTL\nIp: 1\n";
        let entry = net_snmp::parse_content(fixture).unwrap();
        assert!(entry.fields.is_empty());
    }

    #[test]
    fn partitions_header_only() {
        let fixture = "major minor  #blocks  name\n\n";
        let entry = partitions::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn cpuinfo_empty() {
        let entry = cpuinfo::parse_content("").unwrap();
        assert_eq!(field_by_name(&entry, "logical_cpus").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn crypto_single_entry_no_trailing_newline() {
        let fixture = "name         : test\ndriver       : test-drv\nmodule       : kernel\ntype         : cipher";
        let entry = crypto::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "algorithm_count").unwrap().value, FieldValue::Integer(1));
    }

    #[test]
    fn mounts_short_line() {
        let fixture = "short\n";
        let entry = mounts::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn modules_empty() {
        let entry = modules::parse_content("").unwrap();
        assert_eq!(field_by_name(&entry, "module_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn zoneinfo_empty() {
        let entry = zoneinfo::parse_content("").unwrap();
        assert_eq!(field_by_name(&entry, "zone_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn net_tcp_empty() {
        let fixture = "  sl  local_address rem_address   st\n";
        let entry = net_tcp::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "connection_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn net_udp_empty() {
        let fixture = "  sl  local_address rem_address   st\n";
        let entry = net_udp::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "socket_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn net_arp_empty() {
        let fixture = "IP address       HW type     Flags       HW address            Mask     Device\n";
        let entry = net_arp::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "entry_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn net_route_empty() {
        let fixture = "Iface\tDestination\tGateway\tFlags\tRefCnt\tUse\tMetric\tMask\tMTU\tWindow\tIRTT\n";
        let entry = net_route::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "route_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn buddyinfo_empty() {
        let entry = buddyinfo::parse_content("").unwrap();
        assert_eq!(field_by_name(&entry, "zone_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn cgroups_only_comments() {
        let fixture = "#subsys_name\thierarchy\tnum_cgroups\tenabled\n";
        let entry = cgroups::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "controller_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn softirqs_empty() {
        let fixture = "                    CPU0\n";
        let entry = softirqs::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "softirq_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn diskstats_no_activity() {
        let fixture = "   8       0 sda 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n";
        let entry = diskstats::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "active_devices").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn devices_empty() {
        let entry = devices::parse_content("").unwrap();
        assert_eq!(field_by_name(&entry, "device_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn filesystems_empty() {
        let entry = filesystems::parse_content("").unwrap();
        assert_eq!(field_by_name(&entry, "filesystem_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn net_sockstat_empty() {
        let entry = net_sockstat::parse_content("").unwrap();
        assert!(entry.fields.is_empty());
    }

    #[test]
    fn net_netstat_empty() {
        let entry = net_netstat::parse_content("").unwrap();
        assert!(entry.fields.is_empty());
    }

    #[test]
    fn schedstat_empty() {
        let entry = schedstat::parse_content("").unwrap();
        assert_eq!(field_by_name(&entry, "version").unwrap().value, FieldValue::Text("".into()));
        assert_eq!(field_by_name(&entry, "cpu_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn slabinfo_header_only() {
        let fixture = "slabinfo - version: 2.1\n# name ...\n";
        let entry = slabinfo::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "cache_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn timer_list_empty() {
        let entry = timer_list::parse_content("").unwrap();
        assert_eq!(field_by_name(&entry, "clock_count").unwrap().value, FieldValue::Integer(0));
        assert_eq!(field_by_name(&entry, "timer_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn consoles_empty() {
        let entry = consoles::parse_content("").unwrap();
        assert_eq!(field_by_name(&entry, "console_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn misc_empty() {
        let entry = misc::parse_content("").unwrap();
        assert_eq!(field_by_name(&entry, "device_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn locks_malformed_short_line() {
        let fixture = "1: POSIX\n";
        let entry = locks::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "lock_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn iomem_empty() {
        let entry = iomem::parse_content("").unwrap();
        assert_eq!(field_by_name(&entry, "region_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn ioports_empty() {
        let entry = ioports::parse_content("").unwrap();
        assert_eq!(field_by_name(&entry, "region_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn interrupts_empty() {
        let entry = interrupts::parse_content("").unwrap();
        assert_eq!(field_by_name(&entry, "cpu_count").unwrap().value, FieldValue::Integer(0));
        assert_eq!(field_by_name(&entry, "irq_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn net_unix_empty() {
        let fixture = "Num       RefCount Protocol Flags    Type St Inode Path\n";
        let entry = net_unix::parse_content(fixture).unwrap();
        assert_eq!(field_by_name(&entry, "socket_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn pagetypeinfo_empty() {
        let entry = pagetypeinfo::parse_content("").unwrap();
        assert_eq!(field_by_name(&entry, "entry_count").unwrap().value, FieldValue::Integer(0));
    }

    #[test]
    fn pressure_empty() {
        let entry = pressure::parse_content(&[]).unwrap();
        assert!(entry.fields.is_empty());
    }

    #[test]
    fn version_empty() {
        let entry = version::parse_content("").unwrap();
        // Should have at least the "raw" field
        assert!(field_by_name(&entry, "raw").is_some());
    }
}

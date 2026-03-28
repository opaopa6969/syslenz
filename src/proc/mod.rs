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

//! macOS system information via sysctl and system commands.
//!
//! Provides a subset of the metrics available on Linux, using macOS-native APIs.

use super::{Field, FieldValue, ProcEntry, Snapshot};
use std::collections::BTreeMap;
use std::process::Command;
use std::time::SystemTime;

pub fn capture() -> anyhow::Result<Snapshot> {
    let mut entries = BTreeMap::new();

    if let Ok(e) = parse_meminfo() { entries.insert("meminfo".into(), e); }
    if let Ok(e) = parse_uptime() { entries.insert("uptime".into(), e); }
    if let Ok(e) = parse_loadavg() { entries.insert("loadavg".into(), e); }
    if let Ok(e) = parse_cpuinfo() { entries.insert("cpuinfo".into(), e); }
    if let Ok(e) = parse_version() { entries.insert("version".into(), e); }
    if let Ok(e) = parse_mounts() { entries.insert("mounts".into(), e); }
    if let Ok(e) = parse_net_dev() { entries.insert("net/dev".into(), e); }
    if let Ok(e) = parse_processes() { entries.insert("processes".into(), e); }
    if let Ok(e) = parse_diskstats() { entries.insert("diskstats".into(), e); }
    if let Ok(e) = parse_df() { entries.insert("df".into(), e); }
    if let Ok(e) = parse_thermal() { entries.insert("thermal".into(), e); }
    if let Ok(e) = parse_battery() { entries.insert("battery".into(), e); }
    if let Ok(e) = parse_fd() { entries.insert("file-nr".into(), e); }
    if let Ok(e) = parse_top_summary() { entries.insert("top_summary".into(), e); }

    Ok(Snapshot {
        timestamp: SystemTime::now(),
        entries,
    })
}

fn sysctl_u64(name: &str) -> anyhow::Result<u64> {
    let output = Command::new("sysctl").arg("-n").arg(name).output()?;
    let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(s.parse()?)
}

fn sysctl_string(name: &str) -> anyhow::Result<String> {
    let output = Command::new("sysctl").arg("-n").arg(name).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn parse_meminfo() -> anyhow::Result<ProcEntry> {
    let page_size = sysctl_u64("hw.pagesize")?;
    let total = sysctl_u64("hw.memsize")?;

    // vm_stat for detailed memory breakdown
    let output = Command::new("vm_stat").output()?;
    let vm_stat = String::from_utf8_lossy(&output.stdout);

    let mut free_pages: u64 = 0;
    let mut active_pages: u64 = 0;
    let mut inactive_pages: u64 = 0;
    let mut wired_pages: u64 = 0;
    let mut compressed_pages: u64 = 0;

    for line in vm_stat.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 { continue; }
        let val: u64 = parts[1].trim().trim_end_matches('.').parse().unwrap_or(0);
        match parts[0].trim() {
            "Pages free" => free_pages = val,
            "Pages active" => active_pages = val,
            "Pages inactive" => inactive_pages = val,
            "Pages wired down" => wired_pages = val,
            "Pages occupied by compressor" => compressed_pages = val,
            _ => {}
        }
    }

    let free = free_pages * page_size;
    let active = active_pages * page_size;
    let inactive = inactive_pages * page_size;
    let wired = wired_pages * page_size;
    let compressed = compressed_pages * page_size;
    let available = free + inactive;

    Ok(ProcEntry {
        source: "sysctl hw.memsize + vm_stat".into(),
        fields: vec![
            Field { name: "MemTotal".into(), value: FieldValue::Bytes(total), unit: None, description: "Total physical memory".into() },
            Field { name: "MemFree".into(), value: FieldValue::Bytes(free), unit: None, description: "Free memory (not used at all)".into() },
            Field { name: "MemAvailable".into(), value: FieldValue::Bytes(available), unit: None, description: "Available memory (free + reclaimable)".into() },
            Field { name: "Active".into(), value: FieldValue::Bytes(active), unit: None, description: "Recently used memory".into() },
            Field { name: "Inactive".into(), value: FieldValue::Bytes(inactive), unit: None, description: "Not recently used, reclaimable".into() },
            Field { name: "Wired".into(), value: FieldValue::Bytes(wired), unit: None, description: "Memory that cannot be paged out".into() },
            Field { name: "Compressed".into(), value: FieldValue::Bytes(compressed), unit: None, description: "Memory compressed by the compressor".into() },
        ],
    })
}

fn parse_uptime() -> anyhow::Result<ProcEntry> {
    let boot_str = sysctl_string("kern.boottime")?;
    // Format: "{ sec = 1234567890, usec = 0 } ..."
    let sec_val = boot_str
        .split("sec = ").nth(1)
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.trim().parse::<u64>().ok())
        .unwrap_or(0);

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let uptime_secs = now.saturating_sub(sec_val) as f64;

    Ok(ProcEntry {
        source: "sysctl kern.boottime".into(),
        fields: vec![
            Field { name: "uptime".into(), value: FieldValue::Duration(uptime_secs), unit: Some("seconds".into()), description: "Time since boot".into() },
        ],
    })
}

fn parse_loadavg() -> anyhow::Result<ProcEntry> {
    let load_str = sysctl_string("vm.loadavg")?;
    // Format: "{ 1.23 2.34 3.45 }"
    let nums: Vec<f64> = load_str
        .trim_matches(|c: char| c == '{' || c == '}' || c.is_whitespace())
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    Ok(ProcEntry {
        source: "sysctl vm.loadavg".into(),
        fields: vec![
            Field { name: "load1".into(), value: FieldValue::Float(*nums.first().unwrap_or(&0.0)), unit: None, description: "1-minute load average".into() },
            Field { name: "load5".into(), value: FieldValue::Float(*nums.get(1).unwrap_or(&0.0)), unit: None, description: "5-minute load average".into() },
            Field { name: "load15".into(), value: FieldValue::Float(*nums.get(2).unwrap_or(&0.0)), unit: None, description: "15-minute load average".into() },
        ],
    })
}

fn parse_cpuinfo() -> anyhow::Result<ProcEntry> {
    let brand = sysctl_string("machdep.cpu.brand_string").unwrap_or_default();
    let cores = sysctl_u64("hw.physicalcpu").unwrap_or(0);
    let logical = sysctl_u64("hw.logicalcpu").unwrap_or(0);
    let freq = sysctl_u64("hw.cpufrequency").unwrap_or(0);

    Ok(ProcEntry {
        source: "sysctl machdep.cpu".into(),
        fields: vec![
            Field { name: "model_name".into(), value: FieldValue::Text(brand), unit: None, description: "CPU model name".into() },
            Field { name: "physical_cores".into(), value: FieldValue::Integer(cores as i64), unit: None, description: "Physical CPU cores".into() },
            Field { name: "logical_cores".into(), value: FieldValue::Integer(logical as i64), unit: None, description: "Logical CPU cores (with HT)".into() },
            Field { name: "frequency".into(), value: FieldValue::Bytes(freq), unit: Some("Hz".into()), description: "CPU frequency".into() },
        ],
    })
}

fn parse_version() -> anyhow::Result<ProcEntry> {
    let version = sysctl_string("kern.osrelease")?;
    let os_type = sysctl_string("kern.ostype").unwrap_or_default();
    let os_version = sysctl_string("kern.osproductversion").unwrap_or_default();

    Ok(ProcEntry {
        source: "sysctl kern.osrelease".into(),
        fields: vec![
            Field { name: "os_type".into(), value: FieldValue::Text(os_type), unit: None, description: "Operating system type".into() },
            Field { name: "os_release".into(), value: FieldValue::Text(version), unit: None, description: "Kernel release version".into() },
            Field { name: "os_version".into(), value: FieldValue::Text(os_version), unit: None, description: "macOS product version".into() },
        ],
    })
}

fn parse_mounts() -> anyhow::Result<ProcEntry> {
    let output = Command::new("mount").output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let rows: Vec<Vec<String>> = text.lines().filter_map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 {
            Some(vec![
                parts[0].to_string(),
                parts[2].to_string(),
                parts[4].trim_matches(|c| c == '(' || c == ')' || c == ',').to_string(),
            ])
        } else {
            None
        }
    }).collect();

    Ok(ProcEntry {
        source: "mount".into(),
        fields: vec![
            Field { name: "mounts".into(), value: FieldValue::Table(rows), unit: None, description: "Mounted filesystems".into() },
        ],
    })
}

fn parse_net_dev() -> anyhow::Result<ProcEntry> {
    let output = Command::new("netstat").arg("-ibn").output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let rows: Vec<Vec<String>> = text.lines().skip(1).filter_map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 7 {
            Some(vec![
                parts[0].to_string(),  // Interface
                parts[4].to_string(),  // Ibytes
                parts[3].to_string(),  // Ipkts
                parts[6].to_string(),  // Obytes
                parts[5].to_string(),  // Opkts
            ])
        } else {
            None
        }
    }).collect();

    Ok(ProcEntry {
        source: "netstat -ibn".into(),
        fields: vec![
            Field { name: "interfaces".into(), value: FieldValue::Table(rows), unit: None, description: "Network interface statistics".into() },
        ],
    })
}

fn parse_processes() -> anyhow::Result<ProcEntry> {
    let output = Command::new("ps")
        .args(["-eo", "pid,comm,state,rss,nlwp,uid", "--no-headers"])
        .output()
        .or_else(|_| {
            // macOS ps syntax differs
            Command::new("ps").args(["-eo", "pid,comm,stat,rss,uid"]).output()
        })?;
    let text = String::from_utf8_lossy(&output.stdout);
    let rows: Vec<Vec<String>> = text.lines().skip(1).filter_map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 {
            Some(parts.iter().map(|s| s.to_string()).collect())
        } else {
            None
        }
    }).collect();

    Ok(ProcEntry {
        source: "ps -eo".into(),
        fields: vec![
            Field { name: "processes".into(), value: FieldValue::Table(rows), unit: None, description: "Running processes".into() },
        ],
    })
}

fn parse_df() -> anyhow::Result<ProcEntry> {
    let output = Command::new("df").arg("-k").output()?;
    let text = String::from_utf8_lossy(&output.stdout);

    let mut fields = Vec::new();
    let mut table_rows: Vec<Vec<String>> = Vec::new();
    let mut root_use_pct: Option<f64> = None;

    for line in text.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 {
            continue;
        }
        let filesystem = parts[0].to_string();
        let size_kb: u64 = parts[1].parse().unwrap_or(0);
        let used_kb: u64 = parts[2].parse().unwrap_or(0);
        let avail_kb: u64 = parts[3].parse().unwrap_or(0);
        let use_pct_str = parts[4].to_string();
        let mounted_on = parts[5..].join(" ");

        table_rows.push(vec![
            filesystem,
            format_size_kb(size_kb),
            format_size_kb(used_kb),
            format_size_kb(avail_kb),
            use_pct_str.clone(),
            mounted_on.clone(),
        ]);

        if mounted_on == "/" {
            if let Some(pct_str) = use_pct_str.strip_suffix('%') {
                root_use_pct = pct_str.parse::<f64>().ok();
            }
        }
    }

    fields.push(Field {
        name: "filesystems".into(),
        value: FieldValue::Table(table_rows),
        unit: None,
        description: "Filesystem usage table: Filesystem, Size, Used, Available, Use%, MountedOn".into(),
    });

    if let Some(pct) = root_use_pct {
        fields.push(Field {
            name: "root_use_pct".into(),
            value: FieldValue::Float(pct),
            unit: Some("%".into()),
            description: "Root filesystem usage percentage".into(),
        });
    }

    Ok(ProcEntry {
        source: "df -k".into(),
        fields,
    })
}

fn format_size_kb(kb: u64) -> String {
    let bytes = kb * 1024;
    const KIB: u64 = 1024;
    const MIB: u64 = 1024 * KIB;
    const GIB: u64 = 1024 * MIB;
    const TIB: u64 = 1024 * GIB;
    if bytes >= TIB {
        format!("{:.1}T", bytes as f64 / TIB as f64)
    } else if bytes >= GIB {
        format!("{:.1}G", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.1}M", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.1}K", bytes as f64 / KIB as f64)
    } else {
        format!("{}B", bytes)
    }
}

fn parse_thermal() -> anyhow::Result<ProcEntry> {
    let mut fields = Vec::new();

    // Try sysctl for thermal throttling level
    if let Ok(level_str) = sysctl_string("machdep.xcpm.cpu_thermal_level") {
        let level: i64 = level_str.parse().unwrap_or(0);
        fields.push(Field {
            name: "thermal_level".into(),
            value: FieldValue::Integer(level),
            unit: None,
            description: "CPU thermal throttling level (0 = nominal)".into(),
        });
    }

    // Try reading SMC CPU temperature via a broadly-available approach
    // `osx-cpu-temp` is a popular open-source tool; try it as optional
    if let Ok(output) = Command::new("osx-cpu-temp").output() {
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            // Output format: "65.0°C" or "65.0 °C"
            if let Some(temp_str) = text.trim().split('°').next() {
                if let Ok(temp) = temp_str.trim().parse::<f64>() {
                    fields.push(Field {
                        name: "cpu_temp".into(),
                        value: FieldValue::Float(temp),
                        unit: Some("°C".into()),
                        description: "CPU temperature from SMC sensor".into(),
                    });
                }
            }
        }
    }

    if fields.is_empty() {
        anyhow::bail!("No thermal information available");
    }

    Ok(ProcEntry {
        source: "sysctl machdep.xcpm / osx-cpu-temp".into(),
        fields,
    })
}

fn parse_battery() -> anyhow::Result<ProcEntry> {
    let output = Command::new("pmset").args(["-g", "batt"]).output()?;
    if !output.status.success() {
        anyhow::bail!("pmset failed");
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let mut fields = Vec::new();

    // Second line typically looks like:
    // -InternalBattery-0 (id=...)	85%; charging; 1:23 remaining present: true
    for line in text.lines().skip(1) {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Extract percentage: find "NN%"
        if let Some(pct_pos) = line.find('%') {
            // Walk backwards from '%' to find start of number
            let before = &line[..pct_pos];
            let num_start = before.rfind(|c: char| !c.is_ascii_digit() && c != '.')
                .map(|i| i + 1)
                .unwrap_or(0);
            if let Ok(pct) = before[num_start..].parse::<f64>() {
                fields.push(Field {
                    name: "battery_pct".into(),
                    value: FieldValue::Float(pct),
                    unit: Some("%".into()),
                    description: "Current battery charge percentage".into(),
                });
            }
        }

        // Extract status: charging, discharging, charged, finishing charge, etc.
        let status = if line.contains("charging;") && !line.contains("discharging") && !line.contains("not charging") {
            "charging"
        } else if line.contains("discharging") {
            "discharging"
        } else if line.contains("charged") {
            "charged"
        } else if line.contains("not charging") {
            "not charging"
        } else {
            "unknown"
        };
        fields.push(Field {
            name: "charging".into(),
            value: FieldValue::Text(status.into()),
            unit: None,
            description: "Battery charging state".into(),
        });

        // Extract time remaining if present (e.g. "1:23 remaining" or "(no estimate)")
        if let Some(pos) = line.find("remaining") {
            let before = line[..pos].trim();
            if let Some(time_str) = before.rsplit(';').next() {
                let time_str = time_str.trim();
                if !time_str.is_empty() {
                    fields.push(Field {
                        name: "time_remaining".into(),
                        value: FieldValue::Text(time_str.into()),
                        unit: None,
                        description: "Estimated time remaining on battery".into(),
                    });
                }
            }
        } else if line.contains("(no estimate)") {
            fields.push(Field {
                name: "time_remaining".into(),
                value: FieldValue::Text("(no estimate)".into()),
                unit: None,
                description: "Estimated time remaining on battery".into(),
            });
        }

        // Only parse the first battery line
        break;
    }

    if fields.is_empty() {
        anyhow::bail!("Could not parse battery info from pmset");
    }

    Ok(ProcEntry {
        source: "pmset -g batt".into(),
        fields,
    })
}

fn parse_fd() -> anyhow::Result<ProcEntry> {
    let max_str = sysctl_string("kern.maxfiles")?;
    let current_str = sysctl_string("kern.num_files")?;

    let fd_max: i64 = max_str.parse()?;
    let fd_current: i64 = current_str.parse()?;

    let usage_pct = if fd_max > 0 {
        (fd_current as f64 / fd_max as f64) * 100.0
    } else {
        0.0
    };

    Ok(ProcEntry {
        source: "sysctl kern.maxfiles / kern.num_files".into(),
        fields: vec![
            Field { name: "fd_max".into(), value: FieldValue::Integer(fd_max), unit: None, description: "Maximum number of file descriptors".into() },
            Field { name: "fd_current".into(), value: FieldValue::Integer(fd_current), unit: None, description: "Currently open file descriptors".into() },
            Field { name: "fd_usage_pct".into(), value: FieldValue::Float(usage_pct), unit: Some("%".into()), description: "File descriptor usage percentage".into() },
        ],
    })
}

fn parse_top_summary() -> anyhow::Result<ProcEntry> {
    let output = Command::new("top").args(["-l", "1", "-n", "0"]).output()?;
    if !output.status.success() {
        anyhow::bail!("top command failed");
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let mut fields = Vec::new();

    for line in text.lines() {
        let line = line.trim();

        // "Processes: 432 total, 3 running, 429 sleeping, 1823 threads"
        if line.starts_with("Processes:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            for (i, part) in parts.iter().enumerate() {
                match *part {
                    "total," | "total" => {
                        if i > 0 {
                            if let Ok(n) = parts[i - 1].parse::<i64>() {
                                fields.push(Field {
                                    name: "process_count".into(),
                                    value: FieldValue::Integer(n),
                                    unit: None,
                                    description: "Total number of processes".into(),
                                });
                            }
                        }
                    }
                    "running," | "running" => {
                        if i > 0 {
                            if let Ok(n) = parts[i - 1].parse::<i64>() {
                                fields.push(Field {
                                    name: "running_count".into(),
                                    value: FieldValue::Integer(n),
                                    unit: None,
                                    description: "Number of running processes".into(),
                                });
                            }
                        }
                    }
                    "threads" | "threads." => {
                        if i > 0 {
                            if let Ok(n) = parts[i - 1].parse::<i64>() {
                                fields.push(Field {
                                    name: "thread_count".into(),
                                    value: FieldValue::Integer(n),
                                    unit: None,
                                    description: "Total number of threads".into(),
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // "CPU usage: 5.26% user, 10.52% sys, 84.21% idle"
        if line.starts_with("CPU usage:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            for (i, part) in parts.iter().enumerate() {
                match *part {
                    "user," | "user" => {
                        if i > 0 {
                            if let Ok(v) = parts[i - 1].trim_end_matches('%').parse::<f64>() {
                                fields.push(Field {
                                    name: "cpu_user".into(),
                                    value: FieldValue::Float(v),
                                    unit: Some("%".into()),
                                    description: "CPU time spent in user mode".into(),
                                });
                            }
                        }
                    }
                    "sys," | "sys" => {
                        if i > 0 {
                            if let Ok(v) = parts[i - 1].trim_end_matches('%').parse::<f64>() {
                                fields.push(Field {
                                    name: "cpu_sys".into(),
                                    value: FieldValue::Float(v),
                                    unit: Some("%".into()),
                                    description: "CPU time spent in system mode".into(),
                                });
                            }
                        }
                    }
                    "idle" | "idle." => {
                        if i > 0 {
                            if let Ok(v) = parts[i - 1].trim_end_matches('%').parse::<f64>() {
                                fields.push(Field {
                                    name: "cpu_idle".into(),
                                    value: FieldValue::Float(v),
                                    unit: Some("%".into()),
                                    description: "CPU time idle".into(),
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    if fields.is_empty() {
        anyhow::bail!("Could not parse top summary output");
    }

    Ok(ProcEntry {
        source: "top -l 1 -n 0".into(),
        fields,
    })
}

fn parse_diskstats() -> anyhow::Result<ProcEntry> {
    let output = Command::new("iostat").arg("-d").output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let rows: Vec<Vec<String>> = text.lines().skip(2).filter_map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            Some(parts.iter().map(|s| s.to_string()).collect())
        } else {
            None
        }
    }).collect();

    Ok(ProcEntry {
        source: "iostat -d".into(),
        fields: vec![
            Field { name: "disks".into(), value: FieldValue::Table(rows), unit: None, description: "Disk I/O statistics".into() },
        ],
    })
}

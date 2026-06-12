//! macOS system information via sysctl and system commands.
//!
//! Provides a subset of the metrics available on Linux, using macOS-native APIs.

use super::{Field, FieldValue, ProcEntry, Snapshot};
use std::collections::BTreeMap;
use std::process::Command;
use std::time::SystemTime;

pub fn capture() -> anyhow::Result<Snapshot> {
    let mut entries = BTreeMap::new();

    if let Ok(e) = parse_meminfo() {
        entries.insert("meminfo".into(), e);
    }
    if let Ok(e) = parse_uptime() {
        entries.insert("uptime".into(), e);
    }
    if let Ok(e) = parse_loadavg() {
        entries.insert("loadavg".into(), e);
    }
    if let Ok(e) = parse_cpuinfo() {
        entries.insert("cpuinfo".into(), e);
    }
    if let Ok(e) = parse_version() {
        entries.insert("version".into(), e);
    }
    if let Ok(e) = parse_mounts() {
        entries.insert("mounts".into(), e);
    }
    if let Ok(e) = parse_net_dev() {
        entries.insert("net/dev".into(), e);
    }
    if let Ok(e) = parse_processes() {
        entries.insert("processes".into(), e);
    }
    if let Ok(e) = parse_diskstats() {
        entries.insert("diskstats".into(), e);
    }
    if let Ok(e) = parse_df() {
        entries.insert("df".into(), e);
    }
    if let Ok(e) = parse_thermal() {
        entries.insert("thermal".into(), e);
    }
    if let Ok(e) = parse_battery() {
        entries.insert("battery".into(), e);
    }
    if let Ok(e) = parse_fd() {
        entries.insert("file-nr".into(), e);
    }
    if let Ok(e) = parse_top_summary() {
        entries.insert("top_summary".into(), e);
    }
    if let Ok(e) = parse_network_connections() {
        entries.insert("net/connections".into(), e);
    }
    if let Ok(e) = parse_launchd_services() {
        entries.insert("launchd".into(), e);
    }
    if let Ok(e) = parse_diskutil() {
        entries.insert("diskutil".into(), e);
    }
    if let Ok(e) = parse_network_config() {
        entries.insert("net/config".into(), e);
    }
    if let Ok(e) = parse_system_profile() {
        entries.insert("system_profile".into(), e);
    }
    if let Ok(e) = parse_open_files() {
        entries.insert("open_files".into(), e);
    }
    if let Ok(e) = parse_dns_config() {
        entries.insert("dns".into(), e);
    }
    if let Ok(e) = parse_software_update() {
        entries.insert("software_update".into(), e);
    }
    if let Ok(e) = parse_power_management() {
        entries.insert("power_management".into(), e);
    }
    if let Ok(e) = parse_kernel_extensions() {
        entries.insert("kernel_extensions".into(), e);
    }

    Ok(Snapshot {
        timestamp: SystemTime::now(),
        entries,
        alerts: Vec::new(),
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
        if parts.len() != 2 {
            continue;
        }
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
            Field {
                name: "MemTotal".into(),
                value: FieldValue::Bytes(total),
                unit: None,
                description: "Total physical memory".into(),
            },
            Field {
                name: "MemFree".into(),
                value: FieldValue::Bytes(free),
                unit: None,
                description: "Free memory (not used at all)".into(),
            },
            Field {
                name: "MemAvailable".into(),
                value: FieldValue::Bytes(available),
                unit: None,
                description: "Available memory (free + reclaimable)".into(),
            },
            Field {
                name: "Active".into(),
                value: FieldValue::Bytes(active),
                unit: None,
                description: "Recently used memory".into(),
            },
            Field {
                name: "Inactive".into(),
                value: FieldValue::Bytes(inactive),
                unit: None,
                description: "Not recently used, reclaimable".into(),
            },
            Field {
                name: "Wired".into(),
                value: FieldValue::Bytes(wired),
                unit: None,
                description: "Memory that cannot be paged out".into(),
            },
            Field {
                name: "Compressed".into(),
                value: FieldValue::Bytes(compressed),
                unit: None,
                description: "Memory compressed by the compressor".into(),
            },
        ],
    })
}

fn parse_uptime() -> anyhow::Result<ProcEntry> {
    let boot_str = sysctl_string("kern.boottime")?;
    // Format: "{ sec = 1234567890, usec = 0 } ..."
    let sec_val = boot_str
        .split("sec = ")
        .nth(1)
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
        fields: vec![Field {
            name: "uptime".into(),
            value: FieldValue::Duration(uptime_secs),
            unit: Some("seconds".into()),
            description: "Time since boot".into(),
        }],
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
            Field {
                name: "load1".into(),
                value: FieldValue::Float(*nums.first().unwrap_or(&0.0)),
                unit: None,
                description: "1-minute load average".into(),
            },
            Field {
                name: "load5".into(),
                value: FieldValue::Float(*nums.get(1).unwrap_or(&0.0)),
                unit: None,
                description: "5-minute load average".into(),
            },
            Field {
                name: "load15".into(),
                value: FieldValue::Float(*nums.get(2).unwrap_or(&0.0)),
                unit: None,
                description: "15-minute load average".into(),
            },
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
            Field {
                name: "model_name".into(),
                value: FieldValue::Text(brand),
                unit: None,
                description: "CPU model name".into(),
            },
            Field {
                name: "physical_cores".into(),
                value: FieldValue::Integer(cores as i64),
                unit: None,
                description: "Physical CPU cores".into(),
            },
            Field {
                name: "logical_cores".into(),
                value: FieldValue::Integer(logical as i64),
                unit: None,
                description: "Logical CPU cores (with HT)".into(),
            },
            Field {
                name: "frequency".into(),
                value: FieldValue::Bytes(freq),
                unit: Some("Hz".into()),
                description: "CPU frequency".into(),
            },
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
            Field {
                name: "os_type".into(),
                value: FieldValue::Text(os_type),
                unit: None,
                description: "Operating system type".into(),
            },
            Field {
                name: "os_release".into(),
                value: FieldValue::Text(version),
                unit: None,
                description: "Kernel release version".into(),
            },
            Field {
                name: "os_version".into(),
                value: FieldValue::Text(os_version),
                unit: None,
                description: "macOS product version".into(),
            },
        ],
    })
}

fn parse_mounts() -> anyhow::Result<ProcEntry> {
    let output = Command::new("mount").output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let rows: Vec<Vec<String>> = text
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                Some(vec![
                    parts[0].to_string(),
                    parts[2].to_string(),
                    parts[4]
                        .trim_matches(|c| c == '(' || c == ')' || c == ',')
                        .to_string(),
                ])
            } else {
                None
            }
        })
        .collect();

    Ok(ProcEntry {
        source: "mount".into(),
        fields: vec![Field {
            name: "mounts".into(),
            value: FieldValue::Table(rows),
            unit: None,
            description: "Mounted filesystems".into(),
        }],
    })
}

fn parse_net_dev() -> anyhow::Result<ProcEntry> {
    let output = Command::new("netstat").arg("-ibn").output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let rows: Vec<Vec<String>> = text
        .lines()
        .skip(1)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 7 {
                Some(vec![
                    parts[0].to_string(), // Interface
                    parts[4].to_string(), // Ibytes
                    parts[3].to_string(), // Ipkts
                    parts[6].to_string(), // Obytes
                    parts[5].to_string(), // Opkts
                ])
            } else {
                None
            }
        })
        .collect();

    Ok(ProcEntry {
        source: "netstat -ibn".into(),
        fields: vec![Field {
            name: "interfaces".into(),
            value: FieldValue::Table(rows),
            unit: None,
            description: "Network interface statistics".into(),
        }],
    })
}

fn parse_processes() -> anyhow::Result<ProcEntry> {
    let output = Command::new("ps")
        .args(["-eo", "pid,comm,state,rss,nlwp,uid", "--no-headers"])
        .output()
        .or_else(|_| {
            // macOS ps syntax differs
            Command::new("ps")
                .args(["-eo", "pid,comm,stat,rss,uid"])
                .output()
        })?;
    let text = String::from_utf8_lossy(&output.stdout);
    let rows: Vec<Vec<String>> = text
        .lines()
        .skip(1)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                Some(parts.iter().map(|s| s.to_string()).collect())
            } else {
                None
            }
        })
        .collect();

    Ok(ProcEntry {
        source: "ps -eo".into(),
        fields: vec![Field {
            name: "processes".into(),
            value: FieldValue::Table(rows),
            unit: None,
            description: "Running processes".into(),
        }],
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
        description: "Filesystem usage table: Filesystem, Size, Used, Available, Use%, MountedOn"
            .into(),
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
            let num_start = before
                .rfind(|c: char| !c.is_ascii_digit() && c != '.')
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
        let status = if line.contains("charging;")
            && !line.contains("discharging")
            && !line.contains("not charging")
        {
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
            Field {
                name: "fd_max".into(),
                value: FieldValue::Integer(fd_max),
                unit: None,
                description: "Maximum number of file descriptors".into(),
            },
            Field {
                name: "fd_current".into(),
                value: FieldValue::Integer(fd_current),
                unit: None,
                description: "Currently open file descriptors".into(),
            },
            Field {
                name: "fd_usage_pct".into(),
                value: FieldValue::Float(usage_pct),
                unit: Some("%".into()),
                description: "File descriptor usage percentage".into(),
            },
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
    let rows: Vec<Vec<String>> = text
        .lines()
        .skip(2)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                Some(parts.iter().map(|s| s.to_string()).collect())
            } else {
                None
            }
        })
        .collect();

    Ok(ProcEntry {
        source: "iostat -d".into(),
        fields: vec![Field {
            name: "disks".into(),
            value: FieldValue::Table(rows),
            unit: None,
            description: "Disk I/O statistics".into(),
        }],
    })
}

fn parse_network_connections() -> anyhow::Result<ProcEntry> {
    let output = Command::new("netstat").args(["-an"]).output()?;
    if !output.status.success() {
        anyhow::bail!("netstat -an failed");
    }
    let text = String::from_utf8_lossy(&output.stdout);

    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut established: i64 = 0;
    let mut time_wait: i64 = 0;
    let mut close_wait: i64 = 0;
    let mut listen: i64 = 0;

    for line in text.lines().skip(2) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }
        let proto = parts[0];
        // Only include tcp/udp lines
        if !proto.starts_with("tcp") && !proto.starts_with("udp") {
            continue;
        }

        let local_addr = parts.get(3).unwrap_or(&"").to_string();
        let remote_addr = parts.get(4).unwrap_or(&"*.*").to_string();
        let state = if proto.starts_with("udp") {
            String::new()
        } else {
            parts.get(5).unwrap_or(&"").to_string()
        };

        match state.as_str() {
            "ESTABLISHED" => established += 1,
            "TIME_WAIT" => time_wait += 1,
            "CLOSE_WAIT" => close_wait += 1,
            "LISTEN" => listen += 1,
            _ => {}
        }

        rows.push(vec![proto.to_string(), local_addr, remote_addr, state]);
    }

    Ok(ProcEntry {
        source: "netstat -an".into(),
        fields: vec![
            Field {
                name: "connections".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Network connections: Protocol, LocalAddr, RemoteAddr, State".into(),
            },
            Field {
                name: "established".into(),
                value: FieldValue::Integer(established),
                unit: None,
                description: "Number of established connections".into(),
            },
            Field {
                name: "time_wait".into(),
                value: FieldValue::Integer(time_wait),
                unit: None,
                description: "Number of connections in TIME_WAIT state".into(),
            },
            Field {
                name: "close_wait".into(),
                value: FieldValue::Integer(close_wait),
                unit: None,
                description: "Number of connections in CLOSE_WAIT state".into(),
            },
            Field {
                name: "listen".into(),
                value: FieldValue::Integer(listen),
                unit: None,
                description: "Number of listening sockets".into(),
            },
        ],
    })
}

fn parse_launchd_services() -> anyhow::Result<ProcEntry> {
    let output = Command::new("launchctl").arg("list").output()?;
    if !output.status.success() {
        anyhow::bail!("launchctl list failed");
    }
    let text = String::from_utf8_lossy(&output.stdout);

    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut running_count: i64 = 0;
    let mut error_count: i64 = 0;

    for line in text.lines().skip(1) {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 3 {
            continue;
        }
        let pid = parts[0].trim().to_string();
        let status = parts[1].trim().to_string();
        let label = parts[2].trim().to_string();

        if pid != "-" && !pid.is_empty() {
            running_count += 1;
        }
        if let Ok(code) = status.parse::<i64>() {
            if code != 0 {
                error_count += 1;
            }
        }

        rows.push(vec![pid, status, label]);
    }

    Ok(ProcEntry {
        source: "launchctl list".into(),
        fields: vec![
            Field {
                name: "services".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Launchd services: PID, Status, Label".into(),
            },
            Field {
                name: "running_count".into(),
                value: FieldValue::Integer(running_count),
                unit: None,
                description: "Number of services with a running PID".into(),
            },
            Field {
                name: "error_count".into(),
                value: FieldValue::Integer(error_count),
                unit: None,
                description: "Number of services with non-zero exit status".into(),
            },
        ],
    })
}

fn parse_diskutil() -> anyhow::Result<ProcEntry> {
    let output = Command::new("diskutil").arg("list").output()?;
    if !output.status.success() {
        anyhow::bail!("diskutil list failed");
    }
    let text = String::from_utf8_lossy(&output.stdout);

    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut current_device = String::new();

    for line in text.lines() {
        let trimmed = line.trim();
        // Header lines like "/dev/disk0 (internal, physical):"
        if trimmed.starts_with("/dev/") {
            current_device = trimmed.trim_end_matches(':').to_string();
            continue;
        }
        // Partition lines like "   0:      GUID_partition_scheme    *500.1 GB   disk0"
        // or "   1:                        EFI EFI  209.7 MB   disk0s1"
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = trimmed.splitn(2, ':').collect();
        if parts.len() < 2 {
            continue;
        }
        let rest = parts[1].trim();
        // Try to extract type, name, size from the rest
        // Format varies; grab what we can from the whitespace-separated tokens
        let tokens: Vec<&str> = rest.split_whitespace().collect();
        if tokens.len() >= 3 {
            let disk_type = tokens[0].to_string();
            // Size is typically near the end, look for a pattern like "500.1" followed by "GB"/"MB"/"TB"/"KB"
            let mut size_str = String::new();
            let mut name_parts: Vec<&str> = Vec::new();
            let mut i = 1;
            while i < tokens.len() {
                if i + 1 < tokens.len() {
                    let maybe_unit = tokens[i + 1].trim_start_matches('*');
                    if matches!(maybe_unit, "KB" | "MB" | "GB" | "TB" | "B") {
                        size_str = format!("{} {}", tokens[i].trim_start_matches('*'), maybe_unit);
                        i += 2;
                        continue;
                    }
                }
                // Check if current token starts with * (size marker)
                if tokens[i].starts_with('*') && i + 1 < tokens.len() {
                    let maybe_unit = tokens[i + 1];
                    if matches!(maybe_unit, "KB" | "MB" | "GB" | "TB" | "B") {
                        size_str = format!("{} {}", tokens[i].trim_start_matches('*'), maybe_unit);
                        i += 2;
                        continue;
                    }
                }
                name_parts.push(tokens[i]);
                i += 1;
            }
            let name = name_parts.join(" ");
            rows.push(vec![current_device.clone(), disk_type, size_str, name]);
        }
    }

    Ok(ProcEntry {
        source: "diskutil list".into(),
        fields: vec![Field {
            name: "volumes".into(),
            value: FieldValue::Table(rows),
            unit: None,
            description: "Disk volumes: Device, Type, Size, Name".into(),
        }],
    })
}

fn parse_network_config() -> anyhow::Result<ProcEntry> {
    let hw_output = Command::new("networksetup")
        .arg("-listallhardwareports")
        .output()?;
    if !hw_output.status.success() {
        anyhow::bail!("networksetup -listallhardwareports failed");
    }
    let hw_text = String::from_utf8_lossy(&hw_output.stdout);

    // Also get ifconfig for addresses and status
    let if_output = Command::new("ifconfig").output()?;
    let if_text = String::from_utf8_lossy(&if_output.stdout);

    // Parse ifconfig into a map of device -> (address, status)
    let mut if_map: std::collections::HashMap<String, (String, String)> =
        std::collections::HashMap::new();
    let mut current_iface = String::new();
    let mut current_addr = String::new();
    let mut current_status = String::new();
    for line in if_text.lines() {
        if !line.starts_with('\t') && !line.starts_with(' ') {
            // Save previous interface
            if !current_iface.is_empty() {
                if_map.insert(
                    current_iface.clone(),
                    (current_addr.clone(), current_status.clone()),
                );
            }
            current_iface = line.split(':').next().unwrap_or("").to_string();
            current_addr = String::new();
            current_status = String::new();
        }
        let trimmed = line.trim();
        if trimmed.starts_with("inet ") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 && current_addr.is_empty() {
                current_addr = parts[1].to_string();
            }
        }
        if trimmed.starts_with("status:") {
            current_status = trimmed.trim_start_matches("status:").trim().to_string();
        }
    }
    if !current_iface.is_empty() {
        if_map.insert(current_iface, (current_addr, current_status));
    }

    // Parse networksetup output into rows
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut port = String::new();
    let mut device = String::new();
    for line in hw_text.lines() {
        let trimmed = line.trim();
        if let Some(p) = trimmed.strip_prefix("Hardware Port: ") {
            port = p.to_string();
        } else if let Some(d) = trimmed.strip_prefix("Device: ") {
            device = d.to_string();
        } else if trimmed.is_empty()
            || trimmed.starts_with("VLAN")
            || trimmed.starts_with("Ethernet")
        {
            if !port.is_empty() && !device.is_empty() {
                let (addr, status) = if_map.get(&device).cloned().unwrap_or_default();
                rows.push(vec![port.clone(), device.clone(), addr, status]);
            }
            port.clear();
            device.clear();
        }
    }
    // Handle last entry if file does not end with blank line
    if !port.is_empty() && !device.is_empty() {
        let (addr, status) = if_map.get(&device).cloned().unwrap_or_default();
        rows.push(vec![port, device, addr, status]);
    }

    Ok(ProcEntry {
        source: "networksetup -listallhardwareports + ifconfig".into(),
        fields: vec![Field {
            name: "ports".into(),
            value: FieldValue::Table(rows),
            unit: None,
            description: "Network hardware ports: Port, Device, Address, Status".into(),
        }],
    })
}

fn parse_system_profile() -> anyhow::Result<ProcEntry> {
    let output = Command::new("system_profiler")
        .args(["SPHardwareDataType", "-detailLevel", "mini"])
        .output()?;
    if !output.status.success() {
        anyhow::bail!("system_profiler failed");
    }
    let text = String::from_utf8_lossy(&output.stdout);

    let mut model_name = String::new();
    let mut model_id = String::new();
    let mut chip = String::new();
    let mut memory = String::new();
    let mut serial_number = String::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if let Some((key, val)) = trimmed.split_once(':') {
            let key = key.trim();
            let val = val.trim().to_string();
            match key {
                "Model Name" => model_name = val,
                "Model Identifier" => model_id = val,
                "Chip" | "Processor Name" => chip = val,
                "Memory" | "Total Number of Cores" => {
                    if key == "Memory" {
                        memory = val;
                    }
                }
                "Serial Number (system)" | "Serial Number" => serial_number = val,
                _ => {}
            }
        }
    }

    if model_name.is_empty() && chip.is_empty() {
        anyhow::bail!("Could not parse system profile data");
    }

    Ok(ProcEntry {
        source: "system_profiler SPHardwareDataType".into(),
        fields: vec![
            Field {
                name: "model_name".into(),
                value: FieldValue::Text(model_name),
                unit: None,
                description: "Mac model name".into(),
            },
            Field {
                name: "model_id".into(),
                value: FieldValue::Text(model_id),
                unit: None,
                description: "Mac model identifier".into(),
            },
            Field {
                name: "chip".into(),
                value: FieldValue::Text(chip),
                unit: None,
                description: "Processor or Apple Silicon chip".into(),
            },
            Field {
                name: "memory".into(),
                value: FieldValue::Text(memory),
                unit: None,
                description: "Total installed memory".into(),
            },
            Field {
                name: "serial_number".into(),
                value: FieldValue::Text(serial_number),
                unit: None,
                description: "System serial number".into(),
            },
        ],
    })
}

fn parse_open_files() -> anyhow::Result<ProcEntry> {
    let fd_max: i64 = sysctl_string("kern.maxfiles")?.parse()?;
    let fd_current: i64 = sysctl_string("kern.num_files")?.parse()?;
    let fd_max_per_proc: i64 = sysctl_string("kern.maxfilesperproc")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let usage_pct = if fd_max > 0 {
        (fd_current as f64 / fd_max as f64) * 100.0
    } else {
        0.0
    };

    let mut fields = vec![
        Field {
            name: "fd_max".into(),
            value: FieldValue::Integer(fd_max),
            unit: None,
            description: "System-wide maximum file descriptors".into(),
        },
        Field {
            name: "fd_current".into(),
            value: FieldValue::Integer(fd_current),
            unit: None,
            description: "Currently open file descriptors system-wide".into(),
        },
        Field {
            name: "fd_max_per_proc".into(),
            value: FieldValue::Integer(fd_max_per_proc),
            unit: None,
            description: "Maximum file descriptors per process".into(),
        },
        Field {
            name: "fd_usage_pct".into(),
            value: FieldValue::Float(usage_pct),
            unit: Some("%".into()),
            description: "File descriptor usage percentage".into(),
        },
    ];

    // Try to get per-process top consumers via lsof
    if let Ok(output) = Command::new("lsof").args(["-n", "-P"]).output() {
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            let mut proc_counts: std::collections::HashMap<String, i64> =
                std::collections::HashMap::new();
            let mut total_open: i64 = 0;
            for line in text.lines().skip(1) {
                total_open += 1;
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(cmd) = parts.first() {
                    *proc_counts.entry(cmd.to_string()).or_insert(0) += 1;
                }
            }

            fields.push(Field {
                name: "total_open_files".into(),
                value: FieldValue::Integer(total_open),
                unit: None,
                description: "Total open files reported by lsof".into(),
            });

            // Top 10 consumers
            let mut top: Vec<(String, i64)> = proc_counts.into_iter().collect();
            top.sort_by(|a, b| b.1.cmp(&a.1));
            let top_rows: Vec<Vec<String>> = top
                .into_iter()
                .take(10)
                .map(|(cmd, count)| vec![cmd, count.to_string()])
                .collect();

            fields.push(Field {
                name: "top_consumers".into(),
                value: FieldValue::Table(top_rows),
                unit: None,
                description: "Top 10 processes by open file count: Command, Count".into(),
            });
        }
    }

    Ok(ProcEntry {
        source: "sysctl kern.maxfiles + lsof".into(),
        fields,
    })
}

fn parse_dns_config() -> anyhow::Result<ProcEntry> {
    let output = Command::new("scutil").arg("--dns").output()?;
    if !output.status.success() {
        anyhow::bail!("scutil --dns failed");
    }
    let text = String::from_utf8_lossy(&output.stdout);

    let mut nameservers: Vec<Vec<String>> = Vec::new();
    let mut search_domains: Vec<String> = Vec::new();
    let mut resolver_count: i64 = 0;
    let mut seen_ns: std::collections::HashSet<String> = std::collections::HashSet::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("resolver #") {
            resolver_count += 1;
        }
        if trimmed.starts_with("nameserver[") {
            if let Some(addr) = trimmed.split(':').last() {
                let addr = addr.trim().to_string();
                if !addr.is_empty() && seen_ns.insert(addr.clone()) {
                    nameservers.push(vec![addr]);
                }
            }
        }
        if trimmed.starts_with("search domain[") {
            if let Some(domain) = trimmed.split(':').last() {
                let domain = domain.trim().to_string();
                if !domain.is_empty() && !search_domains.contains(&domain) {
                    search_domains.push(domain);
                }
            }
        }
    }

    Ok(ProcEntry {
        source: "scutil --dns".into(),
        fields: vec![
            Field {
                name: "nameservers".into(),
                value: FieldValue::Table(nameservers),
                unit: None,
                description: "Configured DNS nameservers".into(),
            },
            Field {
                name: "search_domains".into(),
                value: FieldValue::Text(search_domains.join(", ")),
                unit: None,
                description: "DNS search domains".into(),
            },
            Field {
                name: "resolver_count".into(),
                value: FieldValue::Integer(resolver_count),
                unit: None,
                description: "Number of DNS resolver configurations".into(),
            },
        ],
    })
}

fn parse_software_update() -> anyhow::Result<ProcEntry> {
    let output = Command::new("softwareupdate").arg("-l").output()?;
    let text = String::from_utf8_lossy(&output.stderr).to_string()
        + &String::from_utf8_lossy(&output.stdout);

    let mut update_names: Vec<String> = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        // Lines starting with "* Label:" or "* " indicate an available update
        if let Some(label) = trimmed.strip_prefix("* Label: ") {
            update_names.push(label.trim().to_string());
        } else if let Some(label) = trimmed.strip_prefix("* ") {
            if !label.is_empty() {
                update_names.push(label.trim().to_string());
            }
        }
    }

    let count = update_names.len() as i64;
    let list_text = if update_names.is_empty() {
        "No updates available".to_string()
    } else {
        update_names.join("; ")
    };

    Ok(ProcEntry {
        source: "softwareupdate -l".into(),
        fields: vec![
            Field {
                name: "updates_available".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Number of available software updates".into(),
            },
            Field {
                name: "update_list".into(),
                value: FieldValue::Text(list_text),
                unit: None,
                description: "List of available software updates".into(),
            },
        ],
    })
}

fn parse_power_management() -> anyhow::Result<ProcEntry> {
    let output = Command::new("pmset").args(["-g"]).output()?;
    if !output.status.success() {
        anyhow::bail!("pmset -g failed");
    }
    let text = String::from_utf8_lossy(&output.stdout);

    let mut sleep_setting = String::new();
    let mut display_sleep = String::new();
    let mut disk_sleep = String::new();
    let mut power_source = String::new();

    for line in text.lines() {
        let trimmed = line.trim();
        // "Currently drawing from 'AC Power'" or "'Battery Power'"
        if trimmed.starts_with("Currently drawing from") {
            power_source = trimmed.split('\'').nth(1).unwrap_or("unknown").to_string();
            continue;
        }
        // Key-value lines like " sleep               10 (sleep prevented by ...)" or " sleep  1"
        if let Some((key, val)) = trimmed.split_once(char::is_whitespace) {
            let key = key.trim();
            let val = val.trim();
            // Extract numeric or first token from val
            let first_token = val.split_whitespace().next().unwrap_or(val);
            match key {
                "sleep" => sleep_setting = first_token.to_string(),
                "displaysleep" => display_sleep = first_token.to_string(),
                "disksleep" => disk_sleep = first_token.to_string(),
                _ => {}
            }
        }
    }

    Ok(ProcEntry {
        source: "pmset -g".into(),
        fields: vec![
            Field {
                name: "power_source".into(),
                value: FieldValue::Text(power_source),
                unit: None,
                description: "Current power source (AC Power or Battery Power)".into(),
            },
            Field {
                name: "sleep_setting".into(),
                value: FieldValue::Text(sleep_setting),
                unit: Some("minutes".into()),
                description: "System sleep timeout in minutes (0 = never)".into(),
            },
            Field {
                name: "display_sleep".into(),
                value: FieldValue::Text(display_sleep),
                unit: Some("minutes".into()),
                description: "Display sleep timeout in minutes".into(),
            },
            Field {
                name: "disk_sleep".into(),
                value: FieldValue::Text(disk_sleep),
                unit: Some("minutes".into()),
                description: "Disk sleep timeout in minutes".into(),
            },
        ],
    })
}

fn parse_kernel_extensions() -> anyhow::Result<ProcEntry> {
    let output = Command::new("kextstat").output()?;
    if !output.status.success() {
        anyhow::bail!("kextstat failed");
    }
    let text = String::from_utf8_lossy(&output.stdout);

    let mut rows: Vec<Vec<String>> = Vec::new();

    for line in text.lines().skip(1) {
        // Skip Apple kexts
        if line.contains("com.apple") {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        // kextstat columns: Index Refs Address Size Wired Name (Version) ...
        if parts.len() >= 6 {
            let name = parts[5].to_string();
            let version = parts
                .get(6)
                .unwrap_or(&"")
                .trim_matches(|c| c == '(' || c == ')')
                .to_string();
            let size = parts[3].to_string();
            rows.push(vec![name, version, size]);
        }
    }

    let kext_count = rows.len() as i64;

    Ok(ProcEntry {
        source: "kextstat".into(),
        fields: vec![
            Field {
                name: "third_party_kexts".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Third-party kernel extensions: Name, Version, Size".into(),
            },
            Field {
                name: "kext_count".into(),
                value: FieldValue::Integer(kext_count),
                unit: None,
                description: "Number of loaded third-party kernel extensions".into(),
            },
        ],
    })
}

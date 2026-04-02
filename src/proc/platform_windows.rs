//! Windows system information via WMIC / PowerShell commands.
//!
//! Provides a subset of the metrics available on Linux, using Windows-native tools.

use super::{Field, FieldValue, ProcEntry, Snapshot};
use std::collections::BTreeMap;
use std::process::Command;
use std::time::SystemTime;

pub fn capture() -> anyhow::Result<Snapshot> {
    let mut entries = BTreeMap::new();

    if let Ok(e) = parse_meminfo() {
        entries.insert("meminfo".into(), e);
    }
    if let Ok(e) = parse_cpuinfo() {
        entries.insert("cpuinfo".into(), e);
    }
    if let Ok(e) = parse_version() {
        entries.insert("version".into(), e);
    }
    if let Ok(e) = parse_uptime() {
        entries.insert("uptime".into(), e);
    }
    if let Ok(e) = parse_processes() {
        entries.insert("processes".into(), e);
    }
    if let Ok(e) = parse_net_dev() {
        entries.insert("net/dev".into(), e);
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
    if let Ok(e) = parse_services() {
        entries.insert("services".into(), e);
    }
    if let Ok(e) = parse_fd() {
        entries.insert("file-nr".into(), e);
    }
    if let Ok(e) = parse_eventlog() {
        entries.insert("eventlog".into(), e);
    }
    if let Ok(e) = parse_tcp_connections() {
        entries.insert("tcp_connections".into(), e);
    }
    if let Ok(e) = parse_udp_endpoints() {
        entries.insert("udp_endpoints".into(), e);
    }
    if let Ok(e) = parse_perf_cpu() {
        entries.insert("perf_cpu".into(), e);
    }
    if let Ok(e) = parse_perf_memory() {
        entries.insert("perf_memory".into(), e);
    }
    if let Ok(e) = parse_perf_disk() {
        entries.insert("perf_disk".into(), e);
    }
    if let Ok(e) = parse_handles() {
        entries.insert("handles".into(), e);
    }
    if let Ok(e) = parse_hotfix() {
        entries.insert("hotfix".into(), e);
    }
    if let Ok(e) = parse_scheduled_tasks() {
        entries.insert("scheduled_tasks".into(), e);
    }
    if let Ok(e) = parse_volumes() {
        entries.insert("volumes".into(), e);
    }
    if let Ok(e) = parse_dns_cache() {
        entries.insert("dns_cache".into(), e);
    }
    if let Ok(e) = parse_firewall() {
        entries.insert("firewall".into(), e);
    }

    Ok(Snapshot {
        timestamp: SystemTime::now(),
        entries,
    })
}

fn wmic_query(class: &str, fields: &[&str]) -> anyhow::Result<String> {
    let output = Command::new("wmic")
        .arg(class)
        .arg("get")
        .arg(fields.join(","))
        .arg("/format:csv")
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn powershell(script: &str) -> anyhow::Result<String> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", script])
        .output()?;
    if !output.status.success() {
        anyhow::bail!(
            "PowerShell error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn parse_meminfo() -> anyhow::Result<ProcEntry> {
    let total = powershell("(Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory")?
        .parse::<u64>()
        .unwrap_or(0);
    let free = powershell("(Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory")?
        .parse::<u64>()
        .unwrap_or(0)
        * 1024; // FreePhysicalMemory is in KB

    Ok(ProcEntry {
        source: "Win32_OperatingSystem".into(),
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
                description: "Free physical memory".into(),
            },
            Field {
                name: "MemUsed".into(),
                value: FieldValue::Bytes(total.saturating_sub(free)),
                unit: None,
                description: "Used physical memory".into(),
            },
        ],
    })
}

fn parse_cpuinfo() -> anyhow::Result<ProcEntry> {
    let name = powershell("(Get-CimInstance Win32_Processor).Name")?;
    let cores = powershell("(Get-CimInstance Win32_Processor).NumberOfCores")?
        .parse::<i64>()
        .unwrap_or(0);
    let logical = powershell("(Get-CimInstance Win32_Processor).NumberOfLogicalProcessors")?
        .parse::<i64>()
        .unwrap_or(0);
    let freq = powershell("(Get-CimInstance Win32_Processor).MaxClockSpeed")?
        .parse::<u64>()
        .unwrap_or(0)
        * 1_000_000; // MHz to Hz

    Ok(ProcEntry {
        source: "Win32_Processor".into(),
        fields: vec![
            Field {
                name: "model_name".into(),
                value: FieldValue::Text(name),
                unit: None,
                description: "CPU model name".into(),
            },
            Field {
                name: "physical_cores".into(),
                value: FieldValue::Integer(cores),
                unit: None,
                description: "Physical CPU cores".into(),
            },
            Field {
                name: "logical_cores".into(),
                value: FieldValue::Integer(logical),
                unit: None,
                description: "Logical CPU cores".into(),
            },
            Field {
                name: "frequency".into(),
                value: FieldValue::Bytes(freq),
                unit: Some("Hz".into()),
                description: "Max CPU frequency".into(),
            },
        ],
    })
}

fn parse_version() -> anyhow::Result<ProcEntry> {
    let caption = powershell("(Get-CimInstance Win32_OperatingSystem).Caption")?;
    let version = powershell("(Get-CimInstance Win32_OperatingSystem).Version")?;
    let build = powershell("(Get-CimInstance Win32_OperatingSystem).BuildNumber")?;

    Ok(ProcEntry {
        source: "Win32_OperatingSystem".into(),
        fields: vec![
            Field {
                name: "os_name".into(),
                value: FieldValue::Text(caption),
                unit: None,
                description: "Windows edition".into(),
            },
            Field {
                name: "os_version".into(),
                value: FieldValue::Text(version),
                unit: None,
                description: "OS version number".into(),
            },
            Field {
                name: "build_number".into(),
                value: FieldValue::Text(build),
                unit: None,
                description: "OS build number".into(),
            },
        ],
    })
}

fn parse_uptime() -> anyhow::Result<ProcEntry> {
    let boot_str =
        powershell("(Get-CimInstance Win32_OperatingSystem).LastBootUpTime.ToString('o')")?;
    // Parse ISO 8601 datetime to compute uptime
    // Fallback: use the tick count
    let ticks = powershell("[Environment]::TickCount64")?
        .parse::<u64>()
        .unwrap_or(0);
    let uptime_secs = (ticks / 1000) as f64;

    Ok(ProcEntry {
        source: "Win32_OperatingSystem".into(),
        fields: vec![
            Field {
                name: "uptime".into(),
                value: FieldValue::Duration(uptime_secs),
                unit: Some("seconds".into()),
                description: "Time since boot".into(),
            },
            Field {
                name: "last_boot".into(),
                value: FieldValue::Text(boot_str),
                unit: None,
                description: "Last boot time".into(),
            },
        ],
    })
}

fn parse_processes() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "Get-Process | Select-Object Id,ProcessName,WorkingSet64,Threads | \
         ForEach-Object { \"$($_.Id)|$($_.ProcessName)|$($_.WorkingSet64)|$($_.Threads.Count)\" }",
    )?;

    let rows: Vec<Vec<String>> = output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 4 {
                Some(parts.iter().map(|s| s.to_string()).collect())
            } else {
                None
            }
        })
        .collect();

    Ok(ProcEntry {
        source: "Get-Process".into(),
        fields: vec![Field {
            name: "processes".into(),
            value: FieldValue::Table(rows),
            unit: None,
            description: "Running processes".into(),
        }],
    })
}

fn parse_net_dev() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "Get-NetAdapterStatistics | ForEach-Object { \
         \"$($_.Name)|$($_.ReceivedBytes)|$($_.ReceivedUnicastPackets)|$($_.SentBytes)|$($_.SentUnicastPackets)\" }",
    )?;

    let rows: Vec<Vec<String>> = output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 5 {
                Some(parts.iter().map(|s| s.to_string()).collect())
            } else {
                None
            }
        })
        .collect();

    Ok(ProcEntry {
        source: "Get-NetAdapterStatistics".into(),
        fields: vec![Field {
            name: "interfaces".into(),
            value: FieldValue::Table(rows),
            unit: None,
            description: "Network interface statistics".into(),
        }],
    })
}

fn parse_df() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "Get-PSDrive -PSProvider FileSystem | ForEach-Object { \
         \"$($_.Name)|$($_.Used)|$($_.Free)|$($_.Used + $_.Free)\" }",
    )?;

    let mut fields = Vec::new();
    let mut table_rows: Vec<Vec<String>> = Vec::new();
    let mut c_use_pct: Option<f64> = None;

    for line in output.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 4 {
            continue;
        }
        let name = parts[0].trim().to_string();
        let used: u64 = parts[1].trim().parse().unwrap_or(0);
        let free: u64 = parts[2].trim().parse().unwrap_or(0);
        let total: u64 = parts[3].trim().parse().unwrap_or(0);

        // Skip drives with zero total (e.g. unmapped drives)
        if total == 0 {
            continue;
        }

        let use_pct = (used as f64 / total as f64) * 100.0;

        table_rows.push(vec![
            format!("{}:", name),
            format_size_bytes(total),
            format_size_bytes(used),
            format_size_bytes(free),
            format!("{:.1}%", use_pct),
        ]);

        if name == "C" {
            c_use_pct = Some(use_pct);
        }
    }

    fields.push(Field {
        name: "filesystems".into(),
        value: FieldValue::Table(table_rows),
        unit: None,
        description: "Filesystem usage table: Drive, Total, Used, Free, Use%".into(),
    });

    if let Some(pct) = c_use_pct {
        fields.push(Field {
            name: "root_use_pct".into(),
            value: FieldValue::Float(pct),
            unit: Some("%".into()),
            description: "C: drive usage percentage".into(),
        });
    }

    Ok(ProcEntry {
        source: "Get-PSDrive".into(),
        fields,
    })
}

fn format_size_bytes(bytes: u64) -> String {
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
    // MSAcpi_ThermalZoneTemperature requires admin; try it and bail gracefully
    let output = powershell(
        "Get-CimInstance MSAcpi_ThermalZoneTemperature -Namespace root/WMI -ErrorAction Stop | \
         ForEach-Object { \"$($_.InstanceName)|$($_.CurrentTemperature)\" }",
    )?;

    let mut fields = Vec::new();
    let mut max_temp: f64 = f64::MIN;

    for line in output.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 2 {
            continue;
        }
        let zone_name = parts[0].trim().to_string();
        // Temperature is in tenths of Kelvin
        let tenths_k: f64 = parts[1].trim().parse().unwrap_or(0.0);
        let celsius = (tenths_k / 10.0) - 273.15;

        fields.push(Field {
            name: zone_name.clone(),
            value: FieldValue::Float(celsius),
            unit: Some("°C".into()),
            description: format!("Temperature of thermal zone '{}'", zone_name),
        });

        if celsius > max_temp {
            max_temp = celsius;
        }
    }

    if fields.is_empty() {
        anyhow::bail!("No thermal information available (may require administrator privileges)");
    }

    fields.push(Field {
        name: "max_temp".into(),
        value: FieldValue::Float(max_temp),
        unit: Some("°C".into()),
        description: "Highest temperature across all thermal zones".into(),
    });

    Ok(ProcEntry {
        source: "MSAcpi_ThermalZoneTemperature".into(),
        fields,
    })
}

fn parse_battery() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "Get-CimInstance Win32_Battery | ForEach-Object { \
         \"$($_.EstimatedChargeRemaining)|$($_.BatteryStatus)|$($_.EstimatedRunTime)\" }",
    )?;

    if output.is_empty() {
        anyhow::bail!("No battery detected");
    }

    // Take the first battery line
    let line = output.lines().next().unwrap_or("");
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 3 {
        anyhow::bail!("Unexpected battery output format");
    }

    let pct: f64 = parts[0].trim().parse().unwrap_or(0.0);
    let status_code: u16 = parts[1].trim().parse().unwrap_or(0);
    let runtime_minutes: f64 = parts[2].trim().parse().unwrap_or(0.0);

    // BatteryStatus: 1=discharging, 2=AC/charging, 3=fully charged, 4=low, 5=critical
    let status = match status_code {
        1 => "discharging",
        2 => "charging",
        3 => "fully charged",
        4 => "low",
        5 => "critical",
        6 => "charging",      // Charging
        7 => "charging high", // Charging and High
        8 => "low",           // Low
        9 => "critical",      // Critical
        _ => "unknown",
    };

    let runtime_secs = runtime_minutes * 60.0;

    Ok(ProcEntry {
        source: "Win32_Battery".into(),
        fields: vec![
            Field {
                name: "battery_pct".into(),
                value: FieldValue::Float(pct),
                unit: Some("%".into()),
                description: "Current battery charge percentage".into(),
            },
            Field {
                name: "status".into(),
                value: FieldValue::Text(status.into()),
                unit: None,
                description: "Battery status".into(),
            },
            Field {
                name: "estimated_runtime".into(),
                value: FieldValue::Duration(runtime_secs),
                unit: Some("seconds".into()),
                description: "Estimated battery runtime remaining".into(),
            },
        ],
    })
}

fn parse_services() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "Get-Service | Where-Object {$_.Status -ne 'Stopped'} | \
         ForEach-Object { \"$($_.Status)|$($_.Name)|$($_.DisplayName)\" }",
    )?;

    let mut table_rows: Vec<Vec<String>> = Vec::new();
    let mut failed_count: i64 = 0;

    for line in output.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 3 {
            continue;
        }
        let status = parts[0].trim().to_string();
        let name = parts[1].trim().to_string();
        let display_name = parts[2].trim().to_string();

        // Count services that are not Running (e.g. StartPending, StopPending, Paused)
        if status != "Running" {
            failed_count += 1;
        }

        table_rows.push(vec![status, name, display_name]);
    }

    Ok(ProcEntry {
        source: "Get-Service".into(),
        fields: vec![
            Field {
                name: "services".into(),
                value: FieldValue::Table(table_rows),
                unit: None,
                description: "Non-stopped services: Status, Name, DisplayName".into(),
            },
            Field {
                name: "non_running_count".into(),
                value: FieldValue::Integer(failed_count),
                unit: None,
                description: "Count of non-stopped services not in Running state".into(),
            },
        ],
    })
}

fn parse_fd() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "$p = Get-CimInstance Win32_Process; \
         $sum = ($p | Measure-Object HandleCount -Sum).Sum; \
         $count = $p.Count; \
         \"$sum|$count\"",
    )?;

    let parts: Vec<&str> = output.split('|').collect();
    if parts.len() < 2 {
        anyhow::bail!("Unexpected handle count output");
    }

    let total_handles: i64 = parts[0].trim().parse().unwrap_or(0);
    let process_count: i64 = parts[1].trim().parse().unwrap_or(0);

    Ok(ProcEntry {
        source: "Win32_Process HandleCount".into(),
        fields: vec![
            Field {
                name: "total_handles".into(),
                value: FieldValue::Integer(total_handles),
                unit: None,
                description: "Total open handles across all processes".into(),
            },
            Field {
                name: "process_count".into(),
                value: FieldValue::Integer(process_count),
                unit: None,
                description: "Number of processes".into(),
            },
        ],
    })
}

fn parse_eventlog() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "Get-EventLog -LogName System -EntryType Error -Newest 10 -ErrorAction Stop | \
         ForEach-Object { \"$($_.TimeGenerated.ToString('yyyy-MM-dd HH:mm:ss'))|$($_.Source)|$($_.Message -replace '[\\r\\n]+', ' ' -replace '\\|', '-')\" }",
    )?;

    let mut table_rows: Vec<Vec<String>> = Vec::new();

    for line in output.lines() {
        let parts: Vec<&str> = line.splitn(3, '|').collect();
        if parts.len() < 3 {
            continue;
        }
        let time = parts[0].trim().to_string();
        let source = parts[1].trim().to_string();
        let message = parts[2].trim().to_string();
        // Truncate long messages
        let message = if message.len() > 200 {
            format!("{}...", &message[..200])
        } else {
            message
        };
        table_rows.push(vec![time, source, message]);
    }

    if table_rows.is_empty() {
        anyhow::bail!("No recent error events in System log");
    }

    Ok(ProcEntry {
        source: "Get-EventLog System".into(),
        fields: vec![Field {
            name: "recent_errors".into(),
            value: FieldValue::Table(table_rows),
            unit: None,
            description: "Recent System event log errors: Time, Source, Message".into(),
        }],
    })
}

fn parse_diskstats() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "Get-PhysicalDisk | ForEach-Object { \
         \"$($_.FriendlyName)|$($_.Size)|$($_.MediaType)|$($_.HealthStatus)\" }",
    )?;

    let rows: Vec<Vec<String>> = output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 4 {
                Some(parts.iter().map(|s| s.to_string()).collect())
            } else {
                None
            }
        })
        .collect();

    Ok(ProcEntry {
        source: "Get-PhysicalDisk".into(),
        fields: vec![Field {
            name: "disks".into(),
            value: FieldValue::Table(rows),
            unit: None,
            description: "Physical disk information".into(),
        }],
    })
}

fn parse_tcp_connections() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "Get-NetTCPConnection | Select LocalAddress,LocalPort,RemoteAddress,RemotePort,State | \
         ForEach-Object { \"$($_.LocalAddress)|$($_.LocalPort)|$($_.RemoteAddress)|$($_.RemotePort)|$($_.State)\" }",
    )?;

    if output.is_empty() {
        anyhow::bail!("No TCP connection data available");
    }

    let mut table_rows: Vec<Vec<String>> = Vec::new();
    let mut established: i64 = 0;
    let mut time_wait: i64 = 0;
    let mut close_wait: i64 = 0;
    let mut listen: i64 = 0;

    for line in output.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 5 {
            continue;
        }
        let state = parts[4].trim();
        match state {
            "Established" => established += 1,
            "TimeWait" => time_wait += 1,
            "CloseWait" => close_wait += 1,
            "Listen" => listen += 1,
            _ => {}
        }
        table_rows.push(parts.iter().map(|s| s.trim().to_string()).collect());
    }

    Ok(ProcEntry {
        source: "Get-NetTCPConnection".into(),
        fields: vec![
            Field {
                name: "connections".into(),
                value: FieldValue::Table(table_rows),
                unit: None,
                description:
                    "TCP connections: LocalAddress, LocalPort, RemoteAddress, RemotePort, State"
                        .into(),
            },
            Field {
                name: "established".into(),
                value: FieldValue::Integer(established),
                unit: None,
                description: "Number of established TCP connections".into(),
            },
            Field {
                name: "time_wait".into(),
                value: FieldValue::Integer(time_wait),
                unit: None,
                description: "Number of TCP connections in TimeWait state".into(),
            },
            Field {
                name: "close_wait".into(),
                value: FieldValue::Integer(close_wait),
                unit: None,
                description: "Number of TCP connections in CloseWait state".into(),
            },
            Field {
                name: "listen".into(),
                value: FieldValue::Integer(listen),
                unit: None,
                description: "Number of TCP connections in Listen state".into(),
            },
        ],
    })
}

fn parse_udp_endpoints() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "Get-NetUDPEndpoint | Select LocalAddress,LocalPort | \
         ForEach-Object { \"$($_.LocalAddress)|$($_.LocalPort)\" }",
    )?;

    if output.is_empty() {
        anyhow::bail!("No UDP endpoint data available");
    }

    let rows: Vec<Vec<String>> = output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 2 {
                Some(parts.iter().map(|s| s.trim().to_string()).collect())
            } else {
                None
            }
        })
        .collect();

    Ok(ProcEntry {
        source: "Get-NetUDPEndpoint".into(),
        fields: vec![Field {
            name: "endpoints".into(),
            value: FieldValue::Table(rows),
            unit: None,
            description: "UDP endpoints: LocalAddress, LocalPort".into(),
        }],
    })
}

fn parse_perf_cpu() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "$c = Get-Counter '\\Processor(_Total)\\% Processor Time','\\Processor(_Total)\\% User Time',\
         '\\Processor(_Total)\\% Privileged Time','\\Processor(_Total)\\% Idle Time'; \
         $s = $c.CounterSamples; \
         \"$($s[0].CookedValue)|$($s[1].CookedValue)|$($s[2].CookedValue)|$($s[3].CookedValue)\"",
    )?;

    let parts: Vec<&str> = output.split('|').collect();
    if parts.len() < 4 {
        anyhow::bail!("Unexpected CPU performance counter output");
    }

    let total: f64 = parts[0].trim().parse().unwrap_or(0.0);
    let user: f64 = parts[1].trim().parse().unwrap_or(0.0);
    let system: f64 = parts[2].trim().parse().unwrap_or(0.0);
    let idle: f64 = parts[3].trim().parse().unwrap_or(0.0);

    Ok(ProcEntry {
        source: "Get-Counter Processor".into(),
        fields: vec![
            Field {
                name: "cpu_total_pct".into(),
                value: FieldValue::Float(total),
                unit: Some("%".into()),
                description: "Total CPU utilization percentage".into(),
            },
            Field {
                name: "cpu_user_pct".into(),
                value: FieldValue::Float(user),
                unit: Some("%".into()),
                description: "CPU time spent in user mode".into(),
            },
            Field {
                name: "cpu_system_pct".into(),
                value: FieldValue::Float(system),
                unit: Some("%".into()),
                description: "CPU time spent in privileged (kernel) mode".into(),
            },
            Field {
                name: "cpu_idle_pct".into(),
                value: FieldValue::Float(idle),
                unit: Some("%".into()),
                description: "CPU idle time percentage".into(),
            },
        ],
    })
}

fn parse_perf_memory() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "$c = Get-Counter '\\Memory\\Available MBytes','\\Memory\\Pages/sec',\
         '\\Memory\\Page Faults/sec','\\Memory\\Cache Bytes'; \
         $s = $c.CounterSamples; \
         \"$($s[0].CookedValue)|$($s[1].CookedValue)|$($s[2].CookedValue)|$($s[3].CookedValue)\"",
    )?;

    let parts: Vec<&str> = output.split('|').collect();
    if parts.len() < 4 {
        anyhow::bail!("Unexpected memory performance counter output");
    }

    let available_mb: f64 = parts[0].trim().parse().unwrap_or(0.0);
    let pages_per_sec: f64 = parts[1].trim().parse().unwrap_or(0.0);
    let page_faults: f64 = parts[2].trim().parse().unwrap_or(0.0);
    let cache_bytes: f64 = parts[3].trim().parse().unwrap_or(0.0);

    Ok(ProcEntry {
        source: "Get-Counter Memory".into(),
        fields: vec![
            Field {
                name: "available_mb".into(),
                value: FieldValue::Float(available_mb),
                unit: Some("MB".into()),
                description: "Available physical memory in megabytes".into(),
            },
            Field {
                name: "pages_per_sec".into(),
                value: FieldValue::Float(pages_per_sec),
                unit: Some("pages/sec".into()),
                description: "Rate of pages read from or written to disk for virtual memory".into(),
            },
            Field {
                name: "page_faults_per_sec".into(),
                value: FieldValue::Float(page_faults),
                unit: Some("faults/sec".into()),
                description: "Rate of page faults including both hard and soft faults".into(),
            },
            Field {
                name: "cache_bytes".into(),
                value: FieldValue::Float(cache_bytes),
                unit: Some("bytes".into()),
                description: "Size of the file system cache in bytes".into(),
            },
        ],
    })
}

fn parse_perf_disk() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "$c = Get-Counter '\\PhysicalDisk(_Total)\\Avg. Disk Queue Length',\
         '\\PhysicalDisk(_Total)\\% Disk Time',\
         '\\PhysicalDisk(_Total)\\Disk Read Bytes/sec',\
         '\\PhysicalDisk(_Total)\\Disk Write Bytes/sec'; \
         $s = $c.CounterSamples; \
         \"$($s[0].CookedValue)|$($s[1].CookedValue)|$($s[2].CookedValue)|$($s[3].CookedValue)\"",
    )?;

    let parts: Vec<&str> = output.split('|').collect();
    if parts.len() < 4 {
        anyhow::bail!("Unexpected disk performance counter output");
    }

    let queue_length: f64 = parts[0].trim().parse().unwrap_or(0.0);
    let disk_time_pct: f64 = parts[1].trim().parse().unwrap_or(0.0);
    let read_bytes: f64 = parts[2].trim().parse().unwrap_or(0.0);
    let write_bytes: f64 = parts[3].trim().parse().unwrap_or(0.0);

    Ok(ProcEntry {
        source: "Get-Counter PhysicalDisk".into(),
        fields: vec![
            Field {
                name: "avg_queue_length".into(),
                value: FieldValue::Float(queue_length),
                unit: None,
                description: "Average number of queued disk I/O requests".into(),
            },
            Field {
                name: "disk_time_pct".into(),
                value: FieldValue::Float(disk_time_pct),
                unit: Some("%".into()),
                description: "Percentage of time the disk is busy servicing requests".into(),
            },
            Field {
                name: "read_bytes_per_sec".into(),
                value: FieldValue::Float(read_bytes),
                unit: Some("bytes/sec".into()),
                description: "Disk read throughput in bytes per second".into(),
            },
            Field {
                name: "write_bytes_per_sec".into(),
                value: FieldValue::Float(write_bytes),
                unit: Some("bytes/sec".into()),
                description: "Disk write throughput in bytes per second".into(),
            },
        ],
    })
}

fn parse_handles() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "Get-Process | Sort-Object HandleCount -Descending | Select-Object -First 20 | \
         ForEach-Object { \"$($_.Name)|$($_.Id)|$($_.HandleCount)\" }",
    )?;

    if output.is_empty() {
        anyhow::bail!("No handle data available");
    }

    let mut table_rows: Vec<Vec<String>> = Vec::new();
    let mut total_handles: i64 = 0;

    for line in output.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 3 {
            continue;
        }
        let handle_count: i64 = parts[2].trim().parse().unwrap_or(0);
        total_handles += handle_count;
        table_rows.push(parts.iter().map(|s| s.trim().to_string()).collect());
    }

    // Query the system handle limit from the registry
    let limit_str = powershell(
        "(Get-ItemProperty -Path 'HKLM:\\SYSTEM\\CurrentControlSet\\Control\\Session Manager' \
         -Name 'HandleLimit' -ErrorAction SilentlyContinue).HandleLimit",
    )
    .unwrap_or_default();
    let system_handle_limit: i64 = limit_str.trim().parse().unwrap_or(16_777_216); // Default Windows limit

    Ok(ProcEntry {
        source: "Get-Process Handles".into(),
        fields: vec![
            Field {
                name: "top_handle_consumers".into(),
                value: FieldValue::Table(table_rows),
                unit: None,
                description: "Top 20 processes by handle count: Name, PID, HandleCount".into(),
            },
            Field {
                name: "total_handles".into(),
                value: FieldValue::Integer(total_handles),
                unit: None,
                description: "Sum of handles held by top 20 processes".into(),
            },
            Field {
                name: "system_handle_limit".into(),
                value: FieldValue::Integer(system_handle_limit),
                unit: None,
                description: "System-wide maximum handle count".into(),
            },
        ],
    })
}

fn parse_hotfix() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "Get-HotFix | Select HotFixID,InstalledOn,Description | Sort InstalledOn -Descending | \
         ForEach-Object { \"$($_.HotFixID)|$($_.InstalledOn)|$($_.Description)\" }",
    )?;

    if output.is_empty() {
        anyhow::bail!("No hotfix information available");
    }

    let rows: Vec<Vec<String>> = output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 3 {
                Some(parts.iter().map(|s| s.trim().to_string()).collect())
            } else {
                None
            }
        })
        .collect();

    Ok(ProcEntry {
        source: "Get-HotFix".into(),
        fields: vec![Field {
            name: "hotfixes".into(),
            value: FieldValue::Table(rows),
            unit: None,
            description: "Installed Windows updates: HotFixID, InstalledOn, Description".into(),
        }],
    })
}

fn parse_scheduled_tasks() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "Get-ScheduledTask | Where-Object {$_.State -ne 'Disabled'} | \
         ForEach-Object { \"$($_.TaskName)|$($_.State)|$($_.LastRunTime)\" }",
    )?;

    if output.is_empty() {
        anyhow::bail!("No scheduled task data available");
    }

    let mut table_rows: Vec<Vec<String>> = Vec::new();
    let mut running_count: i64 = 0;
    let mut ready_count: i64 = 0;

    for line in output.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 3 {
            continue;
        }
        let state = parts[1].trim();
        match state {
            "Running" => running_count += 1,
            "Ready" => ready_count += 1,
            _ => {}
        }
        table_rows.push(parts.iter().map(|s| s.trim().to_string()).collect());
    }

    Ok(ProcEntry {
        source: "Get-ScheduledTask".into(),
        fields: vec![
            Field {
                name: "tasks".into(),
                value: FieldValue::Table(table_rows),
                unit: None,
                description: "Active scheduled tasks: TaskName, State, LastRunTime".into(),
            },
            Field {
                name: "running_count".into(),
                value: FieldValue::Integer(running_count),
                unit: None,
                description: "Number of currently running scheduled tasks".into(),
            },
            Field {
                name: "ready_count".into(),
                value: FieldValue::Integer(ready_count),
                unit: None,
                description: "Number of scheduled tasks in Ready state".into(),
            },
        ],
    })
}

fn parse_volumes() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "Get-Volume | Where-Object { $_.DriveLetter } | \
         ForEach-Object { \"$($_.DriveLetter)|$($_.FileSystemLabel)|$($_.FileSystem)|$($_.Size)|$($_.SizeRemaining)|$($_.HealthStatus)\" }",
    )?;

    if output.is_empty() {
        anyhow::bail!("No volume data available");
    }

    let rows: Vec<Vec<String>> = output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 6 {
                Some(parts.iter().map(|s| s.trim().to_string()).collect())
            } else {
                None
            }
        })
        .collect();

    Ok(ProcEntry {
        source: "Get-Volume".into(),
        fields: vec![Field {
            name: "volumes".into(),
            value: FieldValue::Table(rows),
            unit: None,
            description:
                "Volume details: DriveLetter, Label, FileSystem, Size, SizeRemaining, HealthStatus"
                    .into(),
        }],
    })
}

fn parse_dns_cache() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "Get-DnsClientCache | Select Name,Type,Data | Select-Object -First 50 | \
         ForEach-Object { \"$($_.Name)|$($_.Type)|$($_.Data)\" }",
    )?;

    if output.is_empty() {
        anyhow::bail!("DNS client cache is empty or not accessible");
    }

    let rows: Vec<Vec<String>> = output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 3 {
                Some(parts.iter().map(|s| s.trim().to_string()).collect())
            } else {
                None
            }
        })
        .collect();

    Ok(ProcEntry {
        source: "Get-DnsClientCache".into(),
        fields: vec![Field {
            name: "dns_entries".into(),
            value: FieldValue::Table(rows),
            unit: None,
            description: "Cached DNS entries: Name, Type, Data".into(),
        }],
    })
}

fn parse_firewall() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "Get-NetFirewallRule -Enabled True | Select DisplayName,Direction,Action | \
         Select-Object -First 30 | \
         ForEach-Object { \"$($_.DisplayName)|$($_.Direction)|$($_.Action)\" }",
    )?;

    if output.is_empty() {
        anyhow::bail!("No firewall rule data available (may require administrator privileges)");
    }

    let mut table_rows: Vec<Vec<String>> = Vec::new();

    for line in output.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 3 {
            continue;
        }
        table_rows.push(parts.iter().map(|s| s.trim().to_string()).collect());
    }

    // Get total count of enabled rules separately for the summary field
    let count_str = powershell("(Get-NetFirewallRule -Enabled True | Measure-Object).Count")
        .unwrap_or_default();
    let rule_count: i64 = count_str.trim().parse().unwrap_or(0);

    Ok(ProcEntry {
        source: "Get-NetFirewallRule".into(),
        fields: vec![
            Field {
                name: "rules".into(),
                value: FieldValue::Table(table_rows),
                unit: None,
                description: "Enabled firewall rules (first 30): DisplayName, Direction, Action"
                    .into(),
            },
            Field {
                name: "rule_count".into(),
                value: FieldValue::Integer(rule_count),
                unit: None,
                description: "Total number of enabled firewall rules".into(),
            },
        ],
    })
}

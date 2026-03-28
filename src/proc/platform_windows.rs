//! Windows system information via WMIC / PowerShell commands.
//!
//! Provides a subset of the metrics available on Linux, using Windows-native tools.

use super::{Field, FieldValue, ProcEntry, Snapshot};
use std::collections::BTreeMap;
use std::process::Command;
use std::time::SystemTime;

pub fn capture() -> anyhow::Result<Snapshot> {
    let mut entries = BTreeMap::new();

    if let Ok(e) = parse_meminfo() { entries.insert("meminfo".into(), e); }
    if let Ok(e) = parse_cpuinfo() { entries.insert("cpuinfo".into(), e); }
    if let Ok(e) = parse_version() { entries.insert("version".into(), e); }
    if let Ok(e) = parse_uptime() { entries.insert("uptime".into(), e); }
    if let Ok(e) = parse_processes() { entries.insert("processes".into(), e); }
    if let Ok(e) = parse_net_dev() { entries.insert("net/dev".into(), e); }
    if let Ok(e) = parse_diskstats() { entries.insert("diskstats".into(), e); }
    if let Ok(e) = parse_df() { entries.insert("df".into(), e); }
    if let Ok(e) = parse_thermal() { entries.insert("thermal".into(), e); }
    if let Ok(e) = parse_battery() { entries.insert("battery".into(), e); }
    if let Ok(e) = parse_services() { entries.insert("services".into(), e); }
    if let Ok(e) = parse_fd() { entries.insert("file-nr".into(), e); }
    if let Ok(e) = parse_eventlog() { entries.insert("eventlog".into(), e); }

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
        anyhow::bail!("PowerShell error: {}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn parse_meminfo() -> anyhow::Result<ProcEntry> {
    let total = powershell("(Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory")?
        .parse::<u64>().unwrap_or(0);
    let free = powershell("(Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory")?
        .parse::<u64>().unwrap_or(0) * 1024; // FreePhysicalMemory is in KB

    Ok(ProcEntry {
        source: "Win32_OperatingSystem".into(),
        fields: vec![
            Field { name: "MemTotal".into(), value: FieldValue::Bytes(total), unit: None, description: "Total physical memory".into() },
            Field { name: "MemFree".into(), value: FieldValue::Bytes(free), unit: None, description: "Free physical memory".into() },
            Field { name: "MemUsed".into(), value: FieldValue::Bytes(total.saturating_sub(free)), unit: None, description: "Used physical memory".into() },
        ],
    })
}

fn parse_cpuinfo() -> anyhow::Result<ProcEntry> {
    let name = powershell("(Get-CimInstance Win32_Processor).Name")?;
    let cores = powershell("(Get-CimInstance Win32_Processor).NumberOfCores")?
        .parse::<i64>().unwrap_or(0);
    let logical = powershell("(Get-CimInstance Win32_Processor).NumberOfLogicalProcessors")?
        .parse::<i64>().unwrap_or(0);
    let freq = powershell("(Get-CimInstance Win32_Processor).MaxClockSpeed")?
        .parse::<u64>().unwrap_or(0) * 1_000_000; // MHz to Hz

    Ok(ProcEntry {
        source: "Win32_Processor".into(),
        fields: vec![
            Field { name: "model_name".into(), value: FieldValue::Text(name), unit: None, description: "CPU model name".into() },
            Field { name: "physical_cores".into(), value: FieldValue::Integer(cores), unit: None, description: "Physical CPU cores".into() },
            Field { name: "logical_cores".into(), value: FieldValue::Integer(logical), unit: None, description: "Logical CPU cores".into() },
            Field { name: "frequency".into(), value: FieldValue::Bytes(freq), unit: Some("Hz".into()), description: "Max CPU frequency".into() },
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
            Field { name: "os_name".into(), value: FieldValue::Text(caption), unit: None, description: "Windows edition".into() },
            Field { name: "os_version".into(), value: FieldValue::Text(version), unit: None, description: "OS version number".into() },
            Field { name: "build_number".into(), value: FieldValue::Text(build), unit: None, description: "OS build number".into() },
        ],
    })
}

fn parse_uptime() -> anyhow::Result<ProcEntry> {
    let boot_str = powershell(
        "(Get-CimInstance Win32_OperatingSystem).LastBootUpTime.ToString('o')"
    )?;
    // Parse ISO 8601 datetime to compute uptime
    // Fallback: use the tick count
    let ticks = powershell("[Environment]::TickCount64")?
        .parse::<u64>().unwrap_or(0);
    let uptime_secs = (ticks / 1000) as f64;

    Ok(ProcEntry {
        source: "Win32_OperatingSystem".into(),
        fields: vec![
            Field { name: "uptime".into(), value: FieldValue::Duration(uptime_secs), unit: Some("seconds".into()), description: "Time since boot".into() },
            Field { name: "last_boot".into(), value: FieldValue::Text(boot_str), unit: None, description: "Last boot time".into() },
        ],
    })
}

fn parse_processes() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "Get-Process | Select-Object Id,ProcessName,WorkingSet64,Threads | \
         ForEach-Object { \"$($_.Id)|$($_.ProcessName)|$($_.WorkingSet64)|$($_.Threads.Count)\" }"
    )?;

    let rows: Vec<Vec<String>> = output.lines().filter_map(|line| {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 4 {
            Some(parts.iter().map(|s| s.to_string()).collect())
        } else {
            None
        }
    }).collect();

    Ok(ProcEntry {
        source: "Get-Process".into(),
        fields: vec![
            Field { name: "processes".into(), value: FieldValue::Table(rows), unit: None, description: "Running processes".into() },
        ],
    })
}

fn parse_net_dev() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "Get-NetAdapterStatistics | ForEach-Object { \
         \"$($_.Name)|$($_.ReceivedBytes)|$($_.ReceivedUnicastPackets)|$($_.SentBytes)|$($_.SentUnicastPackets)\" }"
    )?;

    let rows: Vec<Vec<String>> = output.lines().filter_map(|line| {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 5 {
            Some(parts.iter().map(|s| s.to_string()).collect())
        } else {
            None
        }
    }).collect();

    Ok(ProcEntry {
        source: "Get-NetAdapterStatistics".into(),
        fields: vec![
            Field { name: "interfaces".into(), value: FieldValue::Table(rows), unit: None, description: "Network interface statistics".into() },
        ],
    })
}

fn parse_df() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "Get-PSDrive -PSProvider FileSystem | ForEach-Object { \
         \"$($_.Name)|$($_.Used)|$($_.Free)|$($_.Used + $_.Free)\" }"
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
         ForEach-Object { \"$($_.InstanceName)|$($_.CurrentTemperature)\" }"
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
         \"$($_.EstimatedChargeRemaining)|$($_.BatteryStatus)|$($_.EstimatedRunTime)\" }"
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
        6 => "charging",       // Charging
        7 => "charging high",  // Charging and High
        8 => "low",            // Low
        9 => "critical",       // Critical
        _ => "unknown",
    };

    let runtime_secs = runtime_minutes * 60.0;

    Ok(ProcEntry {
        source: "Win32_Battery".into(),
        fields: vec![
            Field { name: "battery_pct".into(), value: FieldValue::Float(pct), unit: Some("%".into()), description: "Current battery charge percentage".into() },
            Field { name: "status".into(), value: FieldValue::Text(status.into()), unit: None, description: "Battery status".into() },
            Field { name: "estimated_runtime".into(), value: FieldValue::Duration(runtime_secs), unit: Some("seconds".into()), description: "Estimated battery runtime remaining".into() },
        ],
    })
}

fn parse_services() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "Get-Service | Where-Object {$_.Status -ne 'Stopped'} | \
         ForEach-Object { \"$($_.Status)|$($_.Name)|$($_.DisplayName)\" }"
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
            Field { name: "services".into(), value: FieldValue::Table(table_rows), unit: None, description: "Non-stopped services: Status, Name, DisplayName".into() },
            Field { name: "non_running_count".into(), value: FieldValue::Integer(failed_count), unit: None, description: "Count of non-stopped services not in Running state".into() },
        ],
    })
}

fn parse_fd() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "$p = Get-CimInstance Win32_Process; \
         $sum = ($p | Measure-Object HandleCount -Sum).Sum; \
         $count = $p.Count; \
         \"$sum|$count\""
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
            Field { name: "total_handles".into(), value: FieldValue::Integer(total_handles), unit: None, description: "Total open handles across all processes".into() },
            Field { name: "process_count".into(), value: FieldValue::Integer(process_count), unit: None, description: "Number of processes".into() },
        ],
    })
}

fn parse_eventlog() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "Get-EventLog -LogName System -EntryType Error -Newest 10 -ErrorAction Stop | \
         ForEach-Object { \"$($_.TimeGenerated.ToString('yyyy-MM-dd HH:mm:ss'))|$($_.Source)|$($_.Message -replace '[\\r\\n]+', ' ' -replace '\\|', '-')\" }"
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
        fields: vec![
            Field { name: "recent_errors".into(), value: FieldValue::Table(table_rows), unit: None, description: "Recent System event log errors: Time, Source, Message".into() },
        ],
    })
}

fn parse_diskstats() -> anyhow::Result<ProcEntry> {
    let output = powershell(
        "Get-PhysicalDisk | ForEach-Object { \
         \"$($_.FriendlyName)|$($_.Size)|$($_.MediaType)|$($_.HealthStatus)\" }"
    )?;

    let rows: Vec<Vec<String>> = output.lines().filter_map(|line| {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 4 {
            Some(parts.iter().map(|s| s.to_string()).collect())
        } else {
            None
        }
    }).collect();

    Ok(ProcEntry {
        source: "Get-PhysicalDisk".into(),
        fields: vec![
            Field { name: "disks".into(), value: FieldValue::Table(rows), unit: None, description: "Physical disk information".into() },
        ],
    })
}

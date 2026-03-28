use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/meminfo")?;
    let mut fields = Vec::new();

    for line in content.lines() {
        let Some((key, rest)) = line.split_once(':') else { continue };
        let parts: Vec<&str> = rest.trim().split_whitespace().collect();
        if parts.is_empty() { continue; }

        let value: u64 = match parts[0].parse() {
            Ok(v) => v,
            Err(_) => {
                fields.push(Field {
                    name: key.trim().to_string(),
                    value: FieldValue::Text(rest.trim().to_string()),
                    unit: None,
                    description: describe_meminfo_field(key.trim()),
                });
                continue;
            }
        };

        let has_kb = parts.get(1).is_some_and(|u| *u == "kB");
        let bytes = if has_kb { value * 1024 } else { value };

        fields.push(Field {
            name: key.trim().to_string(),
            value: FieldValue::Bytes(bytes),
            unit: Some(if has_kb { "kB".into() } else { "".into() }),
            description: describe_meminfo_field(key.trim()),
        });
    }

    Ok(ProcEntry {
        source: "/proc/meminfo".into(),
        fields,
    })
}

fn describe_meminfo_field(name: &str) -> String {
    match name {
        "MemTotal" => "Total usable RAM".into(),
        "MemFree" => "Free memory (not used at all)".into(),
        "MemAvailable" => "Available memory for new processes".into(),
        "Buffers" => "Memory used by kernel buffers".into(),
        "Cached" => "Page cache memory".into(),
        "SwapTotal" => "Total swap space".into(),
        "SwapFree" => "Free swap space".into(),
        "SwapCached" => "Swap cached in RAM".into(),
        "Active" => "Recently used memory".into(),
        "Inactive" => "Not recently used memory".into(),
        "Dirty" => "Memory waiting to be written to disk".into(),
        "Writeback" => "Memory being written to disk".into(),
        "Shmem" => "Shared memory (tmpfs etc)".into(),
        "Slab" => "Kernel slab allocator memory".into(),
        "SReclaimable" => "Reclaimable slab memory".into(),
        "SUnreclaim" => "Unreclaimable slab memory".into(),
        "HugePages_Total" => "Total huge pages".into(),
        "HugePages_Free" => "Free huge pages".into(),
        "Hugepagesize" => "Size of each huge page".into(),
        _ => String::new(),
    }
}

use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/cpuinfo")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let mut fields = Vec::new();

    let mut cpu_count = 0;
    let mut model_name = String::new();
    let mut mhz = 0.0f64;
    let mut cache_size = String::new();
    let mut cores_per_socket = 0i64;
    let mut flags = String::new();

    for line in content.lines() {
        let Some((key, val)) = line.split_once(':') else { continue };
        let key = key.trim();
        let val = val.trim();

        match key {
            "processor" => cpu_count += 1,
            "model name" if model_name.is_empty() => model_name = val.to_string(),
            "cpu MHz" => { if let Ok(v) = val.parse::<f64>() { mhz = v; } }
            "cache size" if cache_size.is_empty() => cache_size = val.to_string(),
            "cpu cores" if cores_per_socket == 0 => {
                cores_per_socket = val.parse().unwrap_or(0);
            }
            "flags" if flags.is_empty() => flags = val.to_string(),
            _ => {}
        }
    }

    fields.push(Field {
        name: "logical_cpus".into(),
        value: FieldValue::Integer(cpu_count),
        unit: None,
        description: "Number of logical CPUs (threads)".into(),
    });
    fields.push(Field {
        name: "model".into(),
        value: FieldValue::Text(model_name),
        unit: None,
        description: "CPU model name".into(),
    });
    fields.push(Field {
        name: "frequency".into(),
        value: FieldValue::Float(mhz),
        unit: Some("MHz".into()),
        description: "Current CPU frequency".into(),
    });
    fields.push(Field {
        name: "cache_size".into(),
        value: FieldValue::Text(cache_size),
        unit: None,
        description: "L2/L3 cache size".into(),
    });
    fields.push(Field {
        name: "cores_per_socket".into(),
        value: FieldValue::Integer(cores_per_socket),
        unit: None,
        description: "Physical cores per socket".into(),
    });

    // Parse key flags
    let key_flags: Vec<&str> = flags
        .split_whitespace()
        .filter(|f| matches!(*f,
            "sse" | "sse2" | "sse3" | "ssse3" | "sse4_1" | "sse4_2" |
            "avx" | "avx2" | "avx512f" | "aes" | "vmx" | "svm" |
            "hypervisor" | "ht" | "lm" | "nx"
        ))
        .collect();

    fields.push(Field {
        name: "key_flags".into(),
        value: FieldValue::Text(key_flags.join(" ")),
        unit: None,
        description: "Key CPU flags (SSE/AVX/VT/HT)".into(),
    });

    Ok(ProcEntry {
        source: "/proc/cpuinfo".into(),
        fields,
    })
}

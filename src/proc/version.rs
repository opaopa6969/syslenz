use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/version")?;
    let text = content.trim();

    let mut fields = vec![
        Field {
            name: "raw".into(),
            value: FieldValue::Text(text.to_string()),
            unit: None,
            description: "Full kernel version string".into(),
        },
    ];

    // Extract kernel version (e.g., "Linux version 6.6.87")
    if let Some(ver_start) = text.find("version ") {
        let ver = &text[ver_start + 8..];
        let kernel_ver = ver.split_whitespace().next().unwrap_or(ver);
        fields.push(Field {
            name: "kernel_version".into(),
            value: FieldValue::Text(kernel_ver.to_string()),
            unit: None,
            description: "Kernel version".into(),
        });
    }

    // Extract compiler info
    if let Some(gcc_start) = text.find("(gcc") {
        if let Some(gcc_end) = text[gcc_start..].find(')') {
            let gcc_info = &text[gcc_start + 1..gcc_start + gcc_end];
            fields.push(Field {
                name: "compiler".into(),
                value: FieldValue::Text(gcc_info.to_string()),
                unit: None,
                description: "Compiler used to build kernel".into(),
            });
        }
    }

    Ok(ProcEntry {
        source: "/proc/version".into(),
        fields,
    })
}

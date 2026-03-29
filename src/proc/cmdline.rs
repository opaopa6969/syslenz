use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/cmdline")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let cmdline = content.trim().to_string();
    let param_count = cmdline.split_whitespace().count() as i64;

    Ok(ProcEntry {
        source: "/proc/cmdline".into(),
        fields: vec![
            Field {
                name: "cmdline".into(),
                value: FieldValue::Text(cmdline),
                unit: None,
                description: "Kernel boot command line parameters".into(),
            },
            Field {
                name: "param_count".into(),
                value: FieldValue::Integer(param_count),
                unit: None,
                description: "Number of kernel boot parameters".into(),
            },
        ],
    })
}

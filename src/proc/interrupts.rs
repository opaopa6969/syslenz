use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/interrupts")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let mut rows = Vec::new();
    let mut cpu_count: i64 = 0;

    for (i, line) in content.lines().enumerate() {
        if i == 0 {
            // Header line: "           CPU0       CPU1 ..."
            cpu_count = line.split_whitespace().count() as i64;
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        let irq = parts[0].trim_end_matches(':').to_string();
        // Sum counts across all CPUs
        let mut total: u64 = 0;
        let mut desc_parts = Vec::new();
        for p in &parts[1..] {
            if let Ok(n) = p.parse::<u64>() {
                total += n;
            } else {
                desc_parts.push(*p);
            }
        }
        let description = desc_parts.join(" ");
        rows.push(vec![irq, total.to_string(), description]);
    }

    let irq_count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/interrupts".into(),
        fields: vec![
            Field {
                name: "cpu_count".into(),
                value: FieldValue::Integer(cpu_count),
                unit: None,
                description: "Number of CPUs".into(),
            },
            Field {
                name: "irq_count".into(),
                value: FieldValue::Integer(irq_count),
                unit: None,
                description: "Number of IRQ lines".into(),
            },
            Field {
                name: "interrupts".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Interrupt counters (irq, total_count, description)".into(),
            },
        ],
    })
}

use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/vmstat")?;
    let mut fields = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let name = parts[0].to_string();
            let value: i64 = parts[1].parse().unwrap_or(0);
            let description = describe_vmstat_field(&name);
            fields.push(Field {
                name,
                value: FieldValue::Integer(value),
                unit: None,
                description,
            });
        }
    }

    Ok(ProcEntry {
        source: "/proc/vmstat".into(),
        fields,
    })
}

fn describe_vmstat_field(name: &str) -> String {
    match name {
        "pgfault" => "Total page faults".into(),
        "pgmajfault" => "Major page faults".into(),
        "pgpgin" => "Pages paged in from disk".into(),
        "pgpgout" => "Pages paged out to disk".into(),
        "pswpin" => "Pages swapped in".into(),
        "pswpout" => "Pages swapped out".into(),
        "nr_free_pages" => "Free pages".into(),
        "nr_active_anon" => "Active anonymous pages".into(),
        "nr_inactive_anon" => "Inactive anonymous pages".into(),
        "nr_active_file" => "Active file-backed pages".into(),
        "nr_inactive_file" => "Inactive file-backed pages".into(),
        "nr_dirty" => "Dirty pages".into(),
        "nr_writeback" => "Pages under writeback".into(),
        "nr_slab_reclaimable" => "Reclaimable slab pages".into(),
        "nr_slab_unreclaimable" => "Unreclaimable slab pages".into(),
        "oom_kill" => "OOM killer invocations".into(),
        _ => String::new(),
    }
}

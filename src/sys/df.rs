use crate::proc::{Field, FieldValue, ProcEntry};
use std::fs;

/// Parse disk filesystem usage via statvfs() syscall.
///
/// Reads /proc/mounts to discover mount points, then calls statvfs() on each
/// real filesystem to get usage statistics. Pseudo-filesystems are filtered out.
pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/mounts")?;
    let mut fields = Vec::new();

    let pseudo_fs = [
        "proc", "sysfs", "tmpfs", "devpts", "devtmpfs", "cgroup", "cgroup2",
        "pstore", "securityfs", "debugfs", "tracefs", "hugetlbfs", "mqueue",
        "fusectl", "configfs", "binfmt_misc", "autofs", "efivarfs", "bpf",
        "nsfs", "ramfs", "rpc_pipefs", "nfsd", "overlay",
    ];

    let mut table_rows: Vec<Vec<String>> = Vec::new();
    // Track root filesystem stats for convenience fields
    let mut root_total: u64 = 0;
    let mut root_used: u64 = 0;
    let mut root_available: u64 = 0;

    let mut seen_mounts = std::collections::HashSet::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }
        let device = parts[0];
        let mount_point = parts[1];
        let fs_type = parts[2];

        // Skip pseudo-filesystems
        if pseudo_fs.contains(&fs_type) {
            continue;
        }

        // Skip duplicates (same device mounted multiple times)
        if !seen_mounts.insert(device.to_string()) {
            continue;
        }

        // Call statvfs
        let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
        let c_path = std::ffi::CString::new(mount_point)?;
        let ret = unsafe { libc::statvfs(c_path.as_ptr(), &mut stat) };
        if ret != 0 {
            continue;
        }

        let block_size = stat.f_frsize as u64;
        let total = stat.f_blocks * block_size;
        let free = stat.f_bfree * block_size;
        let available = stat.f_bavail * block_size;
        let used = total.saturating_sub(free);
        let use_pct = if total > 0 {
            (used as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        table_rows.push(vec![
            device.to_string(),
            mount_point.to_string(),
            format_size(total),
            format_size(used),
            format_size(available),
            format!("{:.1}%", use_pct),
        ]);

        if mount_point == "/" {
            root_total = total;
            root_used = used;
            root_available = available;
        }
    }

    fields.push(Field {
        name: "filesystems".into(),
        value: FieldValue::Table(table_rows),
        unit: None,
        description: "Filesystem usage table: Device, MountPoint, Total, Used, Available, Use%".into(),
    });

    fields.push(Field {
        name: "total_disk".into(),
        value: FieldValue::Bytes(root_total),
        unit: Some("bytes".into()),
        description: "Total disk space on root filesystem".into(),
    });

    fields.push(Field {
        name: "used_disk".into(),
        value: FieldValue::Bytes(root_used),
        unit: Some("bytes".into()),
        description: "Used disk space on root filesystem".into(),
    });

    fields.push(Field {
        name: "available_disk".into(),
        value: FieldValue::Bytes(root_available),
        unit: Some("bytes".into()),
        description: "Available disk space on root filesystem".into(),
    });

    if root_total > 0 {
        let use_pct = (root_used as f64 / root_total as f64) * 100.0;
        fields.push(Field {
            name: "root_use_pct".into(),
            value: FieldValue::Float(use_pct),
            unit: Some("%".into()),
            description: "Root filesystem usage percentage".into(),
        });
    }

    Ok(ProcEntry {
        source: "statfs()".into(),
        fields,
    })
}

fn format_size(bytes: u64) -> String {
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

use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/meminfo")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
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
        "MemAvailable" => "Available memory for new processes without swapping".into(),
        "Buffers" => "Memory used by kernel buffers (block device I/O)".into(),
        "Cached" => "Page cache memory (file-backed pages)".into(),
        "SwapCached" => "Swap pages also present in RAM (avoids I/O on swap-in)".into(),
        "Active" => "Recently used memory (less likely to be reclaimed)".into(),
        "Inactive" => "Not recently used memory (more likely to be reclaimed)".into(),
        "Active(anon)" => "Recently used anonymous memory (heap, stack, etc.)".into(),
        "Inactive(anon)" => "Inactive anonymous memory eligible for swap-out".into(),
        "Active(file)" => "Recently used file-backed memory (page cache)".into(),
        "Inactive(file)" => "Inactive file-backed memory eligible for reclaim".into(),
        "Unevictable" => "Memory that cannot be reclaimed (mlock, ramfs, etc.)".into(),
        "Mlocked" => "Memory locked with mlock() system call".into(),
        "SwapTotal" => "Total swap space available".into(),
        "SwapFree" => "Unused swap space".into(),
        "Zswap" => "Memory consumed by compressed pages in zswap".into(),
        "Zswapped" => "Original uncompressed size of pages stored in zswap".into(),
        "Dirty" => "Memory waiting to be written to disk".into(),
        "Writeback" => "Memory actively being written to disk".into(),
        "AnonPages" => "Non-file-backed memory mapped into page tables".into(),
        "Mapped" => "Files mapped into memory with mmap()".into(),
        "Shmem" => "Shared memory (tmpfs, shmem, devtmpfs)".into(),
        "KReclaimable" => "Kernel memory that can be reclaimed (includes SReclaimable)".into(),
        "Slab" => "Kernel slab allocator memory (SReclaimable + SUnreclaim)".into(),
        "SReclaimable" => "Slab memory that can be reclaimed (dentry/inode caches)".into(),
        "SUnreclaim" => "Slab memory that cannot be reclaimed".into(),
        "KernelStack" => "Memory used by kernel thread stacks".into(),
        "PageTables" => "Memory used by page table entries".into(),
        "SecPageTables" => "Memory used by secondary page tables (KVM, IOMMU)".into(),
        "NFS_Unstable" => "NFS pages sent but not yet committed (always 0 on modern kernels)".into(),
        "Bounce" => "Bounce buffer memory for block device I/O".into(),
        "WritebackTmp" => "Temporary writeback memory used by FUSE".into(),
        "CommitLimit" => "Total memory available for allocation (based on overcommit ratio)".into(),
        "Committed_AS" => "Total memory currently allocated (may exceed physical RAM)".into(),
        "VmallocTotal" => "Total size of vmalloc address space".into(),
        "VmallocUsed" => "Amount of vmalloc area currently used".into(),
        "VmallocChunk" => "Largest contiguous free block in vmalloc area".into(),
        "Percpu" => "Memory allocated to per-CPU data structures".into(),
        "HardwareCorrupted" => "Memory detected as having hardware errors (ECC failures)".into(),
        "AnonHugePages" => "Anonymous memory backed by transparent huge pages".into(),
        "ShmemHugePages" => "Shared memory backed by huge pages".into(),
        "ShmemPmdMapped" => "Shared memory mapped with PMD-level huge pages".into(),
        "FileHugePages" => "File-backed memory backed by huge pages".into(),
        "FilePmdMapped" => "File-backed memory mapped with PMD-level huge pages".into(),
        "CmaTotal" => "Total Contiguous Memory Allocator (CMA) pages".into(),
        "CmaFree" => "Free Contiguous Memory Allocator (CMA) pages".into(),
        "HugePages_Total" => "Total number of pre-allocated huge pages".into(),
        "HugePages_Free" => "Number of unallocated huge pages".into(),
        "HugePages_Rsvd" => "Huge pages reserved but not yet allocated".into(),
        "HugePages_Surp" => "Huge pages in the pool above the configured value".into(),
        "Hugepagesize" => "Default size of each huge page".into(),
        "Hugetlb" => "Total memory used by all huge page sizes".into(),
        "DirectMap4k" => "Memory mapped with 4 kB pages in the direct map".into(),
        "DirectMap2M" => "Memory mapped with 2 MB pages in the direct map".into(),
        "DirectMap1G" => "Memory mapped with 1 GB pages in the direct map".into(),
        _ => format!("Memory info: {}", name),
    }
}

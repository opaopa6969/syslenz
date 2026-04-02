use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/vmstat")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
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
    // Exact matches first
    let desc = match name {
        // --- Page counts (nr_*) ---
        "nr_free_pages" => "Number of free pages",
        "nr_zone_inactive_anon" => "Inactive anonymous pages per zone",
        "nr_zone_active_anon" => "Active anonymous pages per zone",
        "nr_zone_inactive_file" => "Inactive file-backed pages per zone",
        "nr_zone_active_file" => "Active file-backed pages per zone",
        "nr_zone_unevictable" => "Unevictable pages per zone",
        "nr_zone_write_pending" => "Pages with pending writes per zone",
        "nr_mlock" => "Pages locked in memory (mlock)",
        "nr_bounce" => "Bounce buffer pages",
        "nr_zspages" => "Compressed pages (zswap/zram)",
        "nr_free_cma" => "Free CMA (Contiguous Memory Allocator) pages",
        "nr_unaccepted" => "Unaccepted memory pages (confidential VMs)",
        "nr_inactive_anon" => "Inactive anonymous pages (total)",
        "nr_active_anon" => "Active anonymous pages (total)",
        "nr_inactive_file" => "Inactive file-backed pages (total)",
        "nr_active_file" => "Active file-backed pages (total)",
        "nr_unevictable" => "Pages that cannot be evicted",
        "nr_slab_reclaimable" => "Reclaimable kernel slab pages",
        "nr_slab_unreclaimable" => "Unreclaimable kernel slab pages",
        "nr_isolated_anon" => "Isolated anonymous pages (migration)",
        "nr_isolated_file" => "Isolated file pages (migration)",
        "nr_anon_pages" => "Total anonymous pages",
        "nr_mapped" => "Pages mapped into page tables",
        "nr_file_pages" => "Total file-backed pages",
        "nr_dirty" => "Pages waiting to be written to disk",
        "nr_writeback" => "Pages being written to disk right now",
        "nr_writeback_temp" => "Temporary writeback pages",
        "nr_shmem" => "Shared memory pages (tmpfs, shmem)",
        "nr_shmem_hugepages" => "Shared memory huge pages",
        "nr_shmem_pmdmapped" => "Shared memory pages mapped by PMD",
        "nr_file_hugepages" => "File-backed huge pages",
        "nr_file_pmdmapped" => "File pages mapped by PMD",
        "nr_anon_transparent_hugepages" => "Anonymous transparent huge pages",
        "nr_vmscan_write" => "Pages written by page reclaim",
        "nr_vmscan_immediate_reclaim" => "Pages immediately reclaimed by vmscan",
        "nr_dirtied" => "Total pages dirtied since boot",
        "nr_written" => "Total pages written since boot",
        "nr_throttled_written" => "Pages written while throttled",
        "nr_kernel_misc_reclaimable" => "Reclaimable miscellaneous kernel pages",
        "nr_foll_pin_acquired" => "GUP pin-fast page references acquired",
        "nr_foll_pin_released" => "GUP pin-fast page references released",
        "nr_kernel_stack" => "Kernel stack pages",
        "nr_page_table_pages" => "Pages used for page tables",
        "nr_sec_page_table_pages" => "Pages used for secondary page tables",
        "nr_swapcached" => "Pages in swap cache",
        "nr_dirty_threshold" => "Dirty page threshold",
        "nr_dirty_background_threshold" => "Background dirty page threshold",
        "nr_unstable" => "NFS unstable pages (deprecated, always 0)",

        // --- Workingset tracking ---
        "workingset_nodes" => "Workingset shadow node count",
        "workingset_refault_anon" => "Anonymous page workingset refaults",
        "workingset_refault_file" => "File page workingset refaults",
        "workingset_activate_anon" => "Anonymous page workingset activations",
        "workingset_activate_file" => "File page workingset activations",
        "workingset_restore_anon" => "Anonymous page workingset restores",
        "workingset_restore_file" => "File page workingset restores",
        "workingset_nodereclaim" => "Workingset shadow nodes reclaimed",

        // --- NUMA stats ---
        "numa_hit" => "NUMA allocations on intended node",
        "numa_miss" => "NUMA allocations on non-intended node",
        "numa_foreign" => "NUMA allocations intended for this node but placed elsewhere",
        "numa_interleave" => "NUMA interleave policy allocations",
        "numa_local" => "NUMA allocations on the local node",
        "numa_other" => "NUMA allocations on a remote node",

        // --- Page I/O ---
        "pgpgin" => "Pages paged in from disk (KB)",
        "pgpgout" => "Pages paged out to disk (KB)",
        "pswpin" => "Pages swapped in",
        "pswpout" => "Pages swapped out",

        // --- Page fault stats ---
        "pgfault" => "Total page faults (minor + major)",
        "pgmajfault" => "Major page faults (required disk I/O)",

        // --- Page allocation ---
        "pgfree" => "Pages freed",
        "pgactivate" => "Pages moved to the active list",
        "pgdeactivate" => "Pages moved to the inactive list",
        "pglazyfree" => "Pages marked for lazy freeing",
        "pglazyfreed" => "Pages actually lazy-freed",
        "pgrefill" => "Pages scanned during refill",
        "pgreuse" => "Pages reused via page allocator fast path",

        // --- Page reclaim (kswapd, direct, khugepaged) ---
        "pgsteal_kswapd" => "Pages reclaimed by kswapd",
        "pgsteal_direct" => "Pages reclaimed by direct reclaim",
        "pgsteal_khugepaged" => "Pages reclaimed by khugepaged",
        "pgdemote_kswapd" => "Pages demoted by kswapd",
        "pgdemote_direct" => "Pages demoted by direct reclaim",
        "pgdemote_khugepaged" => "Pages demoted by khugepaged",
        "pgscan_kswapd" => "Pages scanned by kswapd",
        "pgscan_direct" => "Pages scanned by direct reclaim",
        "pgscan_khugepaged" => "Pages scanned by khugepaged",
        "pgscan_direct_throttle" => "Direct reclaim scans throttled",
        "pgscan_anon" => "Anonymous pages scanned",
        "pgscan_file" => "File pages scanned",
        "pgsteal_anon" => "Anonymous pages reclaimed",
        "pgsteal_file" => "File pages reclaimed",
        "zone_reclaim_failed" => "Zone reclaim failures",
        "pginodesteal" => "Pages reclaimed by inode freeing",
        "slabs_scanned" => "Slab pages scanned for reclaim",
        "kswapd_inodesteal" => "Inodes reclaimed by kswapd",
        "kswapd_low_wmark_hit_quickly" => "Times kswapd reached low watermark quickly",
        "kswapd_high_wmark_hit_quickly" => "Times kswapd reached high watermark quickly",
        "pageoutrun" => "Times kswapd was woken for page-out",
        "pgrotated" => "Pages rotated to tail of LRU",
        "drop_pagecache" => "Page cache drop events (echo to drop_caches)",
        "drop_slab" => "Slab cache drop events (echo to drop_caches)",

        // --- OOM ---
        "oom_kill" => "OOM killer invocations",

        // --- Migration ---
        "pgmigrate_success" => "Successful page migrations",
        "pgmigrate_fail" => "Failed page migrations",

        // --- THP (Transparent Huge Pages) ---
        "thp_migration_success" => "Successful THP migrations",
        "thp_migration_fail" => "Failed THP migrations",
        "thp_migration_split" => "THP splits during migration",
        "thp_fault_alloc" => "THP allocations on page fault",
        "thp_fault_fallback" => "THP allocation fallbacks on page fault",
        "thp_fault_fallback_charge" => "THP charge fallbacks on page fault",
        "thp_collapse_alloc" => "THP collapse allocations (khugepaged)",
        "thp_collapse_alloc_failed" => "Failed THP collapse allocations",
        "thp_file_alloc" => "THP file page allocations",
        "thp_file_fallback" => "THP file page allocation fallbacks",
        "thp_file_fallback_charge" => "THP file page charge fallbacks",
        "thp_file_mapped" => "THP file pages mapped",
        "thp_split_page" => "THP pages split into base pages",
        "thp_split_page_failed" => "Failed THP page splits",
        "thp_deferred_split_page" => "THP deferred split pages",
        "thp_split_pmd" => "THP PMD splits",
        "thp_scan_exceed_none_pte" => "THP scans exceeding none PTE threshold",
        "thp_scan_exceed_swap_pte" => "THP scans exceeding swap PTE threshold",
        "thp_scan_exceed_share_pte" => "THP scans exceeding shared PTE threshold",
        "thp_split_pud" => "THP PUD splits",
        "thp_zero_page_alloc" => "THP zero page allocations",
        "thp_zero_page_alloc_failed" => "Failed THP zero page allocations",
        "thp_swpout" => "THP pages swapped out whole",
        "thp_swpout_fallback" => "THP swap-out fallbacks (split required)",

        // --- Compaction ---
        "compact_migrate_scanned" => "Pages scanned for compaction migration",
        "compact_free_scanned" => "Pages scanned for compaction free pages",
        "compact_isolated" => "Pages isolated for compaction",
        "compact_stall" => "Compaction stalls (direct compaction)",
        "compact_fail" => "Compaction failures",
        "compact_success" => "Successful compactions",
        "compact_daemon_wake" => "Compaction daemon (kcompactd) wakeups",
        "compact_daemon_migrate_scanned" => "Pages scanned by compaction daemon for migration",
        "compact_daemon_free_scanned" => "Pages scanned by compaction daemon for free pages",

        // --- Huge TLB ---
        "htlb_buddy_alloc_success" => "Successful huge page buddy allocations",
        "htlb_buddy_alloc_fail" => "Failed huge page buddy allocations",

        // --- Unevictable pages ---
        "unevictable_pgs_culled" => "Unevictable pages culled from LRU",
        "unevictable_pgs_scanned" => "Unevictable pages scanned",
        "unevictable_pgs_rescued" => "Unevictable pages rescued back to LRU",
        "unevictable_pgs_mlocked" => "Pages made unevictable via mlock",
        "unevictable_pgs_munlocked" => "Pages made evictable via munlock",
        "unevictable_pgs_cleared" => "Unevictable pages cleared",
        "unevictable_pgs_stranded" => "Unevictable pages stranded on wrong LRU",

        // --- Balloon (virtio) ---
        "balloon_inflate" => "Balloon pages inflated (memory returned to host)",
        "balloon_deflate" => "Balloon pages deflated (memory reclaimed from host)",
        "balloon_migrate" => "Balloon pages migrated",

        // --- Swap read-ahead ---
        "swap_ra" => "Swap read-ahead pages",
        "swap_ra_hit" => "Swap read-ahead hits",

        // --- KSM (Kernel Same-page Merging) ---
        "ksm_swpin_copy" => "KSM pages copied on swap-in",
        "cow_ksm" => "Copy-on-write on KSM pages",

        // --- Direct map ---
        "direct_map_level2_splits" => "Direct map 2MB page splits",
        "direct_map_level3_splits" => "Direct map 1GB page splits",

        // Fallback: use prefix-based matching
        _ => return describe_vmstat_field_by_prefix(name),
    };
    desc.into()
}

fn describe_vmstat_field_by_prefix(name: &str) -> String {
    if name.starts_with("pgalloc_") {
        let zone = &name["pgalloc_".len()..];
        return format!("Page allocations from {} zone", zone);
    }
    if name.starts_with("allocstall_") {
        let zone = &name["allocstall_".len()..];
        return format!("Allocation stalls in {} zone", zone);
    }
    if name.starts_with("pgskip_") {
        let zone = &name["pgskip_".len()..];
        return format!("Pages skipped in {} zone during reclaim", zone);
    }
    if name.starts_with("nr_") {
        return format!(
            "Page count: {}",
            name.replace('_', " ").trim_start_matches("nr ")
        );
    }
    if name.starts_with("pgsteal_") {
        return format!("Pages reclaimed by {}", &name["pgsteal_".len()..]);
    }
    if name.starts_with("pgscan_") {
        return format!("Pages scanned by {}", &name["pgscan_".len()..]);
    }
    if name.starts_with("compact_") {
        return format!(
            "Memory compaction: {}",
            name.replace('_', " ").trim_start_matches("compact ")
        );
    }
    if name.starts_with("thp_") {
        return format!(
            "Transparent huge page: {}",
            name.replace('_', " ").trim_start_matches("thp ")
        );
    }
    if name.starts_with("numa_") {
        return format!(
            "NUMA allocation: {}",
            name.replace('_', " ").trim_start_matches("numa ")
        );
    }
    if name.starts_with("workingset_") {
        return format!(
            "Working set: {}",
            name.replace('_', " ").trim_start_matches("workingset ")
        );
    }
    if name.starts_with("unevictable_") {
        return format!(
            "Unevictable pages: {}",
            name.replace('_', " ").trim_start_matches("unevictable ")
        );
    }
    // Final fallback - still provide something
    name.replace('_', " ")
}

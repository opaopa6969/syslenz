use crate::i18n::Locale;

/// Metric category classification.
/// Extends the 6-variant `education::Category` to cover all platform metrics
/// with 8 flat variants (no hierarchy).
///
/// Design rationale (DGE 014): Category classifies *sources* for learning paths.
/// MetricKind classifies *individual fields*. For example, the `pressure` source
/// contains both Memory and Cpu metrics — source-level classification cannot
/// capture this. MetricKind can.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetricKind {
    /// RAM usage, caches, buffers, swap, memory pressure.
    /// Linux: meminfo, vmstat, swaps, buddyinfo, zoneinfo, slabinfo, pagetypeinfo, pressure(memory)
    /// macOS: hw.memsize, vm_stat
    /// Windows: Win32_OperatingSystem, perf_memory, PageFile
    Memory,

    /// CPU utilization, load averages, scheduling, interrupts, thermals.
    /// Linux: stat, loadavg, cpuinfo, schedstat, softirqs, interrupts, pressure(cpu), thermal
    /// macOS: top, vm.loadavg, smc/xcpm
    /// Windows: perf_cpu, \Processor, MSAcpi_ThermalZoneTemperature
    Cpu,

    /// Interfaces, connections, routing, DNS, firewalls.
    /// Linux: net/dev, net/tcp, net/udp, net/unix, net/arp, net/route, net/sockstat,
    ///        net/snmp, net/netstat, net/wireless
    /// macOS: netstat, lsof, net/config, dns
    /// Windows: Get-NetTCPConnection, udp_endpoints, dns_cache, firewall
    Network,

    /// Disk I/O, filesystems, mounts, partitions, volumes.
    /// Linux: diskstats, df, mounts, partitions, locks, pressure(io)
    /// macOS: df, diskutil, diskstats
    /// Windows: perf_disk, df, volumes
    Storage,

    /// Process listings, file descriptor / handle usage.
    /// Linux: processes, file-nr
    /// macOS: ps, open_files, kern.num_files
    /// Windows: Get-Process, handles
    Process,

    /// Uptime, kernel info, boot config, cgroups, modules.
    /// Linux: uptime, version, cmdline, cgroups, consoles, devices, filesystems,
    ///        iomem, ioports, misc, dma, timer_list, modules
    /// macOS: kern.boottime, system_profile, software_update
    /// Windows: uptime, version, hotfix, scheduled_tasks, eventlog
    System,

    /// Battery, power management.
    /// Linux: (laptop support planned via upower/sysfs)
    /// macOS: pmset, power_management
    /// Windows: Win32_Battery
    Power,

    /// Cryptography, firewalls, kernel modules/extensions.
    /// Linux: crypto, modules
    /// macOS: kextstat (kernel_extensions)
    /// Windows: firewall
    Security,
}

impl MetricKind {
    /// All variants in canonical order.
    pub fn all() -> &'static [MetricKind] {
        &[
            MetricKind::Memory,
            MetricKind::Cpu,
            MetricKind::Network,
            MetricKind::Storage,
            MetricKind::Process,
            MetricKind::System,
            MetricKind::Power,
            MetricKind::Security,
        ]
    }

    /// Short 3-character label for TUI display.
    pub fn icon(&self) -> &'static str {
        match self {
            MetricKind::Memory => "MEM",
            MetricKind::Cpu => "CPU",
            MetricKind::Network => "NET",
            MetricKind::Storage => "DSK",
            MetricKind::Process => "PRC",
            MetricKind::System => "SYS",
            MetricKind::Power => "PWR",
            MetricKind::Security => "SEC",
        }
    }

    /// Localized human-readable label.
    pub fn label(&self, locale: Locale) -> &'static str {
        match (self, locale) {
            (MetricKind::Memory, Locale::En) => "Memory",
            (MetricKind::Memory, Locale::Ja) => "メモリ",
            (MetricKind::Cpu, Locale::En) => "CPU / Load",
            (MetricKind::Cpu, Locale::Ja) => "CPU / 負荷",
            (MetricKind::Network, Locale::En) => "Network",
            (MetricKind::Network, Locale::Ja) => "ネットワーク",
            (MetricKind::Storage, Locale::En) => "Storage",
            (MetricKind::Storage, Locale::Ja) => "ストレージ",
            (MetricKind::Process, Locale::En) => "Process",
            (MetricKind::Process, Locale::Ja) => "プロセス",
            (MetricKind::System, Locale::En) => "System",
            (MetricKind::System, Locale::Ja) => "システム",
            (MetricKind::Power, Locale::En) => "Power",
            (MetricKind::Power, Locale::Ja) => "電源",
            (MetricKind::Security, Locale::En) => "Security",
            (MetricKind::Security, Locale::Ja) => "セキュリティ",
        }
    }

    /// Sources related to this metric kind (Linux source names).
    /// Analogous to `education::Category::related_sources` but for the
    /// expanded 8-category scheme.
    pub fn related_sources(&self) -> &'static [&'static str] {
        match self {
            MetricKind::Memory => &[
                "meminfo",
                "vmstat",
                "swaps",
                "buddyinfo",
                "zoneinfo",
                "slabinfo",
                "pagetypeinfo",
            ],
            MetricKind::Cpu => &[
                "stat",
                "loadavg",
                "cpuinfo",
                "schedstat",
                "softirqs",
                "interrupts",
                "thermal",
            ],
            MetricKind::Network => &[
                "net/dev",
                "net/tcp",
                "net/udp",
                "net/unix",
                "net/arp",
                "net/route",
                "net/sockstat",
                "net/snmp",
                "net/netstat",
                "net/wireless",
            ],
            MetricKind::Storage => &["diskstats", "df", "mounts", "partitions", "locks"],
            MetricKind::Process => &["processes", "file-nr"],
            MetricKind::System => &[
                "uptime",
                "version",
                "cmdline",
                "cgroups",
                "consoles",
                "devices",
                "filesystems",
                "iomem",
                "ioports",
                "misc",
                "dma",
                "timer_list",
                "modules",
            ],
            MetricKind::Power => &[
                // Linux: future upower/sysfs support
                // macOS: battery, power_management
                // Windows: battery
            ],
            MetricKind::Security => &["crypto"],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_returns_8_variants() {
        assert_eq!(MetricKind::all().len(), 8);
    }

    #[test]
    fn icons_are_3_chars() {
        for kind in MetricKind::all() {
            assert_eq!(kind.icon().len(), 3, "icon for {:?} is not 3 chars", kind);
        }
    }

    #[test]
    fn labels_are_nonempty() {
        for kind in MetricKind::all() {
            assert!(!kind.label(Locale::En).is_empty());
            assert!(!kind.label(Locale::Ja).is_empty());
        }
    }

    #[test]
    fn related_sources_non_overlapping_core() {
        // Memory and Cpu should not share sources (pressure is excluded from
        // related_sources — it belongs to the kind of the individual field).
        let mem = MetricKind::Memory.related_sources();
        let cpu = MetricKind::Cpu.related_sources();
        for s in mem {
            assert!(
                !cpu.contains(s),
                "source '{}' appears in both Memory and Cpu",
                s
            );
        }
    }
}

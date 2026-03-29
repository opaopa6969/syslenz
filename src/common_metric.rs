use crate::i18n::Locale;
use crate::metric_kind::MetricKind;
use crate::proc::{FieldValue, Snapshot};

/// How closely the cross-platform mapping matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MappingConfidence {
    /// Identical definition. Values are directly comparable across platforms.
    Exact,
    /// Same concept with minor definitional differences. Comparison is generally valid.
    Comparable,
    /// Approximation. Calculation methods differ across OSes. Compare with caution.
    Approximate,
}

/// Unit of measurement for a metric value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetricUnit {
    Bytes,
    Percent,
    Count,
    Seconds,
    Celsius,
    BytesPerSec,
    CountPerSec,
    None,
}

/// Platform identifier for cross-platform metric resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Linux,
    MacOS,
    Windows,
}

impl Platform {
    /// Detect the current compilation target platform.
    pub fn current() -> Self {
        #[cfg(target_os = "linux")]
        { Platform::Linux }
        #[cfg(target_os = "macos")]
        { Platform::MacOS }
        #[cfg(target_os = "windows")]
        { Platform::Windows }
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        { Platform::Linux } // fallback
    }
}

/// Cross-platform metric identifiers.
///
/// Each variant represents a concept that has equivalents on at least two platforms.
/// Use `resolve()` to get the platform-specific (source, field) pair, and
/// `confidence()` to understand how closely the mapping matches.
///
/// Design (DGE 014): Only metrics with Approximate or better confidence are included.
/// Metrics that cannot be meaningfully mapped (e.g., Load1 on Windows) return `None`
/// from `resolve()` for that platform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommonMetric {
    /// Total physical RAM.
    MemTotal,
    /// Memory available for new processes without swapping.
    MemAvailable,
    /// Memory currently in use (Total - Available).
    MemUsed,
    /// Swap / pagefile usage.
    SwapUsed,
    /// Overall CPU utilization percentage.
    CpuUsage,
    /// 1-minute load average (Linux/macOS only).
    Load1,
    /// 5-minute load average (Linux/macOS only).
    Load5,
    /// 15-minute load average (Linux/macOS only).
    Load15,
    /// Root filesystem usage percentage.
    DiskUsePct,
    /// CPU / system temperature.
    Temperature,
    /// Number of active TCP connections.
    TcpConnections,
    /// Total number of running processes.
    ProcessCount,
    /// File descriptor / handle usage.
    FdUsage,
    /// Time since last boot (seconds).
    Uptime,
    /// Battery charge percentage (macOS/Windows; Linux planned).
    Battery,
}

/// A resolved (source, field) pair for a specific platform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedMetric {
    pub source: &'static str,
    pub field: &'static str,
}

impl CommonMetric {
    /// All CommonMetric variants.
    pub fn all() -> &'static [CommonMetric] {
        &[
            CommonMetric::MemTotal,
            CommonMetric::MemAvailable,
            CommonMetric::MemUsed,
            CommonMetric::SwapUsed,
            CommonMetric::CpuUsage,
            CommonMetric::Load1,
            CommonMetric::Load5,
            CommonMetric::Load15,
            CommonMetric::DiskUsePct,
            CommonMetric::Temperature,
            CommonMetric::TcpConnections,
            CommonMetric::ProcessCount,
            CommonMetric::FdUsage,
            CommonMetric::Uptime,
            CommonMetric::Battery,
        ]
    }

    /// Which MetricKind this metric belongs to.
    pub fn metric_kind(&self) -> MetricKind {
        match self {
            CommonMetric::MemTotal
            | CommonMetric::MemAvailable
            | CommonMetric::MemUsed
            | CommonMetric::SwapUsed => MetricKind::Memory,

            CommonMetric::CpuUsage
            | CommonMetric::Load1
            | CommonMetric::Load5
            | CommonMetric::Load15
            | CommonMetric::Temperature => MetricKind::Cpu,

            CommonMetric::DiskUsePct => MetricKind::Storage,
            CommonMetric::TcpConnections => MetricKind::Network,

            CommonMetric::ProcessCount
            | CommonMetric::FdUsage => MetricKind::Process,

            CommonMetric::Uptime => MetricKind::System,
            CommonMetric::Battery => MetricKind::Power,
        }
    }

    /// Unit of measurement.
    pub fn unit(&self) -> MetricUnit {
        match self {
            CommonMetric::MemTotal
            | CommonMetric::MemAvailable
            | CommonMetric::MemUsed
            | CommonMetric::SwapUsed => MetricUnit::Bytes,

            CommonMetric::CpuUsage
            | CommonMetric::DiskUsePct
            | CommonMetric::FdUsage
            | CommonMetric::Battery => MetricUnit::Percent,

            CommonMetric::Load1
            | CommonMetric::Load5
            | CommonMetric::Load15 => MetricUnit::None, // dimensionless

            CommonMetric::Temperature => MetricUnit::Celsius,

            CommonMetric::TcpConnections
            | CommonMetric::ProcessCount => MetricUnit::Count,

            CommonMetric::Uptime => MetricUnit::Seconds,
        }
    }

    /// How closely the cross-platform mapping matches.
    pub fn confidence(&self) -> MappingConfidence {
        match self {
            CommonMetric::ProcessCount | CommonMetric::Uptime => MappingConfidence::Exact,

            CommonMetric::MemTotal
            | CommonMetric::MemUsed
            | CommonMetric::SwapUsed
            | CommonMetric::CpuUsage
            | CommonMetric::Load1
            | CommonMetric::Load5
            | CommonMetric::Load15
            | CommonMetric::DiskUsePct
            | CommonMetric::TcpConnections
            | CommonMetric::Battery => MappingConfidence::Comparable,

            CommonMetric::MemAvailable
            | CommonMetric::Temperature
            | CommonMetric::FdUsage => MappingConfidence::Approximate,
        }
    }

    /// Resolve to the platform-specific (source, field) pair.
    ///
    /// Returns `None` if this metric has no mapping on the given platform.
    /// For example, `Load1` has no Windows equivalent, and `Battery` has no
    /// Linux equivalent (yet).
    pub fn resolve(&self, platform: Platform) -> Option<ResolvedMetric> {
        match platform {
            Platform::Linux => self.resolve_linux(),
            Platform::MacOS => self.resolve_macos(),
            Platform::Windows => self.resolve_windows(),
        }
    }

    /// Resolve for the current compilation target.
    pub fn resolve_current(&self) -> Option<ResolvedMetric> {
        self.resolve(Platform::current())
    }

    fn resolve_linux(&self) -> Option<ResolvedMetric> {
        let r = match self {
            CommonMetric::MemTotal => ResolvedMetric { source: "meminfo", field: "MemTotal" },
            CommonMetric::MemAvailable => ResolvedMetric { source: "meminfo", field: "MemAvailable" },
            CommonMetric::MemUsed => ResolvedMetric { source: "meminfo", field: "MemUsed" },
            CommonMetric::SwapUsed => ResolvedMetric { source: "meminfo", field: "SwapUsed" },
            CommonMetric::CpuUsage => ResolvedMetric { source: "stat", field: "cpu_usage_pct" },
            CommonMetric::Load1 => ResolvedMetric { source: "loadavg", field: "load_1min" },
            CommonMetric::Load5 => ResolvedMetric { source: "loadavg", field: "load_5min" },
            CommonMetric::Load15 => ResolvedMetric { source: "loadavg", field: "load_15min" },
            CommonMetric::DiskUsePct => ResolvedMetric { source: "df", field: "root_use_pct" },
            CommonMetric::Temperature => ResolvedMetric { source: "thermal", field: "max_temp" },
            CommonMetric::TcpConnections => ResolvedMetric { source: "net/tcp", field: "connection_count" },
            CommonMetric::ProcessCount => ResolvedMetric { source: "processes", field: "process_count" },
            CommonMetric::FdUsage => ResolvedMetric { source: "file-nr", field: "fd_usage_pct" },
            CommonMetric::Uptime => ResolvedMetric { source: "uptime", field: "uptime" },
            CommonMetric::Battery => return None, // Linux laptop support planned
        };
        Some(r)
    }

    fn resolve_macos(&self) -> Option<ResolvedMetric> {
        let r = match self {
            CommonMetric::MemTotal => ResolvedMetric { source: "meminfo", field: "MemTotal" },
            CommonMetric::MemAvailable => ResolvedMetric { source: "meminfo", field: "MemAvailable" },
            CommonMetric::MemUsed => ResolvedMetric { source: "meminfo", field: "MemUsed" },
            CommonMetric::SwapUsed => ResolvedMetric { source: "meminfo", field: "SwapUsed" },
            CommonMetric::CpuUsage => ResolvedMetric { source: "top_summary", field: "cpu_usage_pct" },
            CommonMetric::Load1 => ResolvedMetric { source: "loadavg", field: "load_1min" },
            CommonMetric::Load5 => ResolvedMetric { source: "loadavg", field: "load_5min" },
            CommonMetric::Load15 => ResolvedMetric { source: "loadavg", field: "load_15min" },
            CommonMetric::DiskUsePct => ResolvedMetric { source: "df", field: "root_use_pct" },
            CommonMetric::Temperature => ResolvedMetric { source: "thermal", field: "cpu_temp" },
            CommonMetric::TcpConnections => ResolvedMetric { source: "net/connections", field: "tcp_count" },
            CommonMetric::ProcessCount => ResolvedMetric { source: "processes", field: "process_count" },
            CommonMetric::FdUsage => ResolvedMetric { source: "file-nr", field: "fd_usage_pct" },
            CommonMetric::Uptime => ResolvedMetric { source: "uptime", field: "uptime" },
            CommonMetric::Battery => ResolvedMetric { source: "battery", field: "battery_percent" },
        };
        Some(r)
    }

    fn resolve_windows(&self) -> Option<ResolvedMetric> {
        let r = match self {
            CommonMetric::MemTotal => ResolvedMetric { source: "meminfo", field: "MemTotal" },
            CommonMetric::MemAvailable => ResolvedMetric { source: "meminfo", field: "MemFree" },
            CommonMetric::MemUsed => ResolvedMetric { source: "meminfo", field: "MemUsed" },
            CommonMetric::SwapUsed => ResolvedMetric { source: "meminfo", field: "SwapUsed" },
            CommonMetric::CpuUsage => ResolvedMetric { source: "perf_cpu", field: "ProcessorTimePct" },
            CommonMetric::Load1 => return None, // No true load average on Windows
            CommonMetric::Load5 => return None,
            CommonMetric::Load15 => return None,
            CommonMetric::DiskUsePct => ResolvedMetric { source: "df", field: "root_use_pct" },
            CommonMetric::Temperature => ResolvedMetric { source: "thermal", field: "temperature" },
            CommonMetric::TcpConnections => ResolvedMetric { source: "tcp_connections", field: "tcp_count" },
            CommonMetric::ProcessCount => ResolvedMetric { source: "processes", field: "process_count" },
            CommonMetric::FdUsage => ResolvedMetric { source: "handles", field: "handle_count" },
            CommonMetric::Uptime => ResolvedMetric { source: "uptime", field: "uptime" },
            CommonMetric::Battery => ResolvedMetric { source: "battery", field: "battery_percent" },
        };
        Some(r)
    }

    /// Localized description of this metric.
    pub fn description(&self, locale: Locale) -> &'static str {
        match (self, locale) {
            (CommonMetric::MemTotal, Locale::En) => "Total physical RAM",
            (CommonMetric::MemTotal, Locale::Ja) => "物理RAMの合計",
            (CommonMetric::MemAvailable, Locale::En) => "Memory available without swapping",
            (CommonMetric::MemAvailable, Locale::Ja) => "スワップなしで利用可能なメモリ",
            (CommonMetric::MemUsed, Locale::En) => "Memory currently in use",
            (CommonMetric::MemUsed, Locale::Ja) => "使用中のメモリ",
            (CommonMetric::SwapUsed, Locale::En) => "Swap / pagefile in use",
            (CommonMetric::SwapUsed, Locale::Ja) => "使用中のスワップ / ページファイル",
            (CommonMetric::CpuUsage, Locale::En) => "Overall CPU utilization",
            (CommonMetric::CpuUsage, Locale::Ja) => "CPU使用率",
            (CommonMetric::Load1, Locale::En) => "1-minute load average",
            (CommonMetric::Load1, Locale::Ja) => "1分間のロードアベレージ",
            (CommonMetric::Load5, Locale::En) => "5-minute load average",
            (CommonMetric::Load5, Locale::Ja) => "5分間のロードアベレージ",
            (CommonMetric::Load15, Locale::En) => "15-minute load average",
            (CommonMetric::Load15, Locale::Ja) => "15分間のロードアベレージ",
            (CommonMetric::DiskUsePct, Locale::En) => "Root filesystem usage",
            (CommonMetric::DiskUsePct, Locale::Ja) => "ルートファイルシステム使用率",
            (CommonMetric::Temperature, Locale::En) => "CPU / system temperature",
            (CommonMetric::Temperature, Locale::Ja) => "CPU / システム温度",
            (CommonMetric::TcpConnections, Locale::En) => "Active TCP connections",
            (CommonMetric::TcpConnections, Locale::Ja) => "アクティブなTCP接続数",
            (CommonMetric::ProcessCount, Locale::En) => "Total running processes",
            (CommonMetric::ProcessCount, Locale::Ja) => "実行中のプロセス数",
            (CommonMetric::FdUsage, Locale::En) => "File descriptor / handle usage",
            (CommonMetric::FdUsage, Locale::Ja) => "ファイルディスクリプタ / ハンドル使用率",
            (CommonMetric::Uptime, Locale::En) => "Time since last boot",
            (CommonMetric::Uptime, Locale::Ja) => "最後の起動からの経過時間",
            (CommonMetric::Battery, Locale::En) => "Battery charge level",
            (CommonMetric::Battery, Locale::Ja) => "バッテリー充電レベル",
        }
    }

    /// Extract this metric's value from a Snapshot using the current platform mapping.
    pub fn extract<'a>(&self, snapshot: &'a Snapshot) -> Option<&'a FieldValue> {
        let resolved = self.resolve_current()?;
        snapshot
            .entries
            .get(resolved.source)
            .and_then(|entry| entry.fields.iter().find(|f| f.name == resolved.field))
            .map(|f| &f.value)
    }

    /// Look up a CommonMetric by name (case-insensitive).
    pub fn from_name(name: &str) -> Option<CommonMetric> {
        for m in Self::all() {
            if format!("{:?}", m).eq_ignore_ascii_case(name) {
                return Some(*m);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_returns_15_variants() {
        assert_eq!(CommonMetric::all().len(), 15);
    }

    #[test]
    fn every_variant_has_a_kind() {
        for m in CommonMetric::all() {
            let _ = m.metric_kind(); // should not panic
        }
    }

    #[test]
    fn every_variant_has_descriptions() {
        for m in CommonMetric::all() {
            assert!(!m.description(Locale::En).is_empty(), "{:?} missing EN desc", m);
            assert!(!m.description(Locale::Ja).is_empty(), "{:?} missing JA desc", m);
        }
    }

    #[test]
    fn linux_resolves_all_except_battery() {
        for m in CommonMetric::all() {
            let resolved = m.resolve(Platform::Linux);
            if *m == CommonMetric::Battery {
                assert!(resolved.is_none(), "Battery should not resolve on Linux");
            } else {
                assert!(resolved.is_some(), "{:?} should resolve on Linux", m);
            }
        }
    }

    #[test]
    fn windows_has_no_load_averages() {
        assert!(CommonMetric::Load1.resolve(Platform::Windows).is_none());
        assert!(CommonMetric::Load5.resolve(Platform::Windows).is_none());
        assert!(CommonMetric::Load15.resolve(Platform::Windows).is_none());
    }

    #[test]
    fn macos_resolves_all() {
        for m in CommonMetric::all() {
            assert!(
                m.resolve(Platform::MacOS).is_some(),
                "{:?} should resolve on macOS",
                m,
            );
        }
    }

    #[test]
    fn resolve_returns_correct_source_field() {
        let r = CommonMetric::MemTotal.resolve(Platform::Linux).unwrap();
        assert_eq!(r.source, "meminfo");
        assert_eq!(r.field, "MemTotal");

        let r = CommonMetric::CpuUsage.resolve(Platform::MacOS).unwrap();
        assert_eq!(r.source, "top_summary");
        assert_eq!(r.field, "cpu_usage_pct");

        let r = CommonMetric::CpuUsage.resolve(Platform::Windows).unwrap();
        assert_eq!(r.source, "perf_cpu");
        assert_eq!(r.field, "ProcessorTimePct");
    }

    #[test]
    fn from_name_case_insensitive() {
        assert_eq!(CommonMetric::from_name("memtotal"), Some(CommonMetric::MemTotal));
        assert_eq!(CommonMetric::from_name("UPTIME"), Some(CommonMetric::Uptime));
        assert_eq!(CommonMetric::from_name("nonexistent"), None);
    }

    #[test]
    fn confidence_exact_for_uptime_and_process_count() {
        assert_eq!(CommonMetric::Uptime.confidence(), MappingConfidence::Exact);
        assert_eq!(CommonMetric::ProcessCount.confidence(), MappingConfidence::Exact);
    }

    #[test]
    fn confidence_approximate_for_mem_available() {
        assert_eq!(CommonMetric::MemAvailable.confidence(), MappingConfidence::Approximate);
    }
}

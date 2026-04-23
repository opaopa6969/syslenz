//! OpenTelemetry metrics export.
//!
//! Exports all numeric fields from a [`Snapshot`] as OTEL metrics
//! to an OTLP-compatible endpoint (default: `http://localhost:4317`).
//!
//! Features:
//! - Resource attributes (hostname, OS, kernel version)
//! - Metric descriptions from field metadata (with i18n support)
//! - Gauge vs counter semantics based on field characteristics
//! - Metric units from field unit metadata
//!
//! BL-073: Supports `--otel-level core|full` to control which metrics are exported.

#[cfg(feature = "otel")]
use crate::i18n::{self, Locale};
#[cfg(feature = "otel")]
use crate::proc::{FieldValue, Snapshot};

#[cfg(feature = "otel")]
use opentelemetry::KeyValue;
#[cfg(feature = "otel")]
use opentelemetry::metrics::MeterProvider;
#[cfg(feature = "otel")]
use opentelemetry_otlp::WithExportConfig;
#[cfg(feature = "otel")]
use opentelemetry_sdk::Resource;
#[cfg(feature = "otel")]
use opentelemetry_sdk::metrics::SdkMeterProvider;
#[cfg(feature = "otel")]
use std::time::Duration;

/// Which metrics to export via OTEL (BL-073).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OtelLevel {
    /// Export only the ~10 most important metrics (meminfo, loadavg, stat, net/dev, uptime, df).
    Core,
    /// Export all numeric metrics from every /proc source.
    Full,
}

impl OtelLevel {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "core" => OtelLevel::Core,
            _ => OtelLevel::Full,
        }
    }
}

/// The core sources exported when `--otel-level core` is set.
#[cfg(feature = "otel")]
const CORE_SOURCES: &[&str] = &[
    "meminfo", "loadavg", "stat", "net/dev", "uptime", "df", "vmstat", "pressure", "file-nr",
    "version",
];

/// Field name patterns that represent monotonically increasing counters.
///
/// These are cumulative values read from /proc. Because syslenz reads
/// absolute values (not deltas), we still export them as gauges in OTEL
/// (Counter::add() expects increments). However, we tag them with a
/// `syslenz.metric_type=counter` attribute so downstream systems can
/// apply rate() or irate() appropriately.
#[cfg(feature = "otel")]
const COUNTER_PATTERNS: &[&str] = &[
    "context_switches",
    "processes_total",
    "interrupts_total",
    "forks",
    "rx_bytes",
    "tx_bytes",
    "rx_packets",
    "tx_packets",
    "rx_errors",
    "tx_errors",
    "rx_dropped",
    "tx_dropped",
    "pgfault",
    "pgmajfault",
    "pswpin",
    "pswpout",
    "pgpgin",
    "pgpgout",
    "nr_page_table_pages",
    "cpu_user",
    "cpu_nice",
    "cpu_system",
    "cpu_idle",
    "cpu_iowait",
    "cpu_irq",
    "cpu_softirq",
    "cpu_steal",
    "reads_completed",
    "writes_completed",
    "sectors_read",
    "sectors_written",
];

/// Build OTEL resource attributes (hostname, OS, kernel version, service info).
#[cfg(feature = "otel")]
fn build_resource() -> Resource {
    let hostname = std::fs::read_to_string("/proc/sys/kernel/hostname")
        .unwrap_or_else(|_| "unknown".into())
        .trim()
        .to_string();

    let os_type = std::fs::read_to_string("/proc/sys/kernel/ostype")
        .unwrap_or_else(|_| "Linux".into())
        .trim()
        .to_string();

    let kernel_version = std::fs::read_to_string("/proc/sys/kernel/osrelease")
        .unwrap_or_else(|_| "unknown".into())
        .trim()
        .to_string();

    let os_description = std::fs::read_to_string("/proc/version")
        .unwrap_or_else(|_| String::new())
        .trim()
        .to_string();

    Resource::builder()
        .with_attributes([
            KeyValue::new("service.name", "syslenz"),
            KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
            KeyValue::new("host.name", hostname),
            KeyValue::new("os.type", os_type),
            KeyValue::new("os.version", kernel_version),
            KeyValue::new("os.description", os_description),
        ])
        .build()
}

/// Determine whether a field represents a counter (monotonically increasing value).
#[cfg(feature = "otel")]
fn is_counter_field(field_name: &str) -> bool {
    COUNTER_PATTERNS.iter().any(|p| field_name == *p)
        || field_name.ends_with("_total")
        || field_name.ends_with("_errors")
        || field_name.ends_with("_retransmits")
}

/// Build a metric description with i18n support.
///
/// Tries the i18n system first for a richer description in the chosen locale,
/// then falls back to the field's built-in description.
#[cfg(feature = "otel")]
fn metric_description(locale: Locale, source: &str, field_name: &str, fallback: &str) -> String {
    // Try the i18n system for a locale-aware description (returns the short form)
    if let Some((short, _, _)) = i18n::field_description(locale, source, field_name) {
        short.to_string()
    } else {
        fallback.to_string()
    }
}

/// Run the OpenTelemetry export loop with a specific metrics level (BL-073).
#[cfg(feature = "otel")]
pub fn run_otel_export_with_level(
    endpoint: &str,
    interval_secs: u64,
    level: OtelLevel,
    locale: Locale,
) -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let endpoint = endpoint.to_owned();
    rt.block_on(async {
        let exporter = opentelemetry_otlp::MetricExporter::builder()
            .with_tonic()
            .with_endpoint(&endpoint)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build OTLP exporter: {}", e))?;

        let reader = opentelemetry_sdk::metrics::PeriodicReader::builder(exporter)
            .with_interval(Duration::from_secs(interval_secs))
            .build();

        let resource = build_resource();

        let provider = SdkMeterProvider::builder()
            .with_reader(reader)
            .with_resource(resource)
            .build();

        let meter = provider.meter("syslenz");

        let level_label = if level == OtelLevel::Core {
            "core"
        } else {
            "full"
        };
        let locale_label = locale.name();
        eprintln!(
            "syslenz: OTEL export to {} (interval: {}s, level: {}, locale: {})",
            endpoint, interval_secs, level_label, locale_label
        );
        eprintln!("Press Ctrl+C to stop.");

        loop {
            match Snapshot::capture() {
                Ok(snapshot) => {
                    export_snapshot_metrics(&meter, &snapshot, level, locale);
                }
                Err(e) => {
                    eprintln!("Capture error: {}", e);
                }
            }
            tokio::time::sleep(Duration::from_secs(interval_secs)).await;
        }
    })
}

/// Export all numeric fields from a snapshot as OTEL metrics.
///
/// Each field is exported as a gauge with:
/// - A description from the i18n system (or field metadata fallback)
/// - A unit from the field metadata (bytes, seconds, percent, etc.)
/// - Attributes indicating the metric semantic type (gauge vs counter)
#[cfg(feature = "otel")]
fn export_snapshot_metrics(
    meter: &opentelemetry::metrics::Meter,
    snapshot: &Snapshot,
    level: OtelLevel,
    locale: Locale,
) {
    for (source_key, entry) in &snapshot.entries {
        // BL-073: Skip non-core sources when level is Core
        if level == OtelLevel::Core && !CORE_SOURCES.contains(&source_key.as_str()) {
            continue;
        }

        for field in &entry.fields {
            let metric_name = format!("syslenz.{}.{}", source_key.replace('/', "."), field.name);

            let value = match &field.value {
                FieldValue::Bytes(v) => Some(*v as f64),
                FieldValue::Integer(v) => Some(*v as f64),
                FieldValue::Float(v) => Some(*v),
                FieldValue::Duration(v) => Some(*v),
                _ => None,
            };

            if let Some(v) = value {
                let description =
                    metric_description(locale, source_key, &field.name, &field.description);

                // Build gauge with description and optional unit
                let mut builder = meter.f64_gauge(metric_name).with_description(description);

                if let Some(ref unit) = field.unit {
                    builder = builder.with_unit(unit.clone());
                } else {
                    // Infer unit from FieldValue type
                    let inferred_unit = match &field.value {
                        FieldValue::Bytes(_) => Some("By"),
                        FieldValue::Duration(_) => Some("s"),
                        _ => None,
                    };
                    if let Some(u) = inferred_unit {
                        builder = builder.with_unit(u);
                    }
                }

                let gauge = builder.build();

                // Tag with source and semantic type attributes
                let semantic_type = if is_counter_field(&field.name) {
                    "counter"
                } else {
                    "gauge"
                };

                let attributes = [
                    KeyValue::new("syslenz.source", source_key.clone()),
                    KeyValue::new("syslenz.metric_type", semantic_type),
                ];
                gauge.record(v, &attributes);
            }
        }
    }
}

/// Stub when otel feature is not enabled.
#[cfg(not(feature = "otel"))]
pub fn run_otel_export_with_level(
    _endpoint: &str,
    _interval_secs: u64,
    _level: OtelLevel,
    _locale: crate::i18n::Locale,
) -> anyhow::Result<()> {
    anyhow::bail!(
        "OpenTelemetry support is not compiled in.\n\
         Rebuild with: cargo build --features otel"
    )
}

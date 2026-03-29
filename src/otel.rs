//! OpenTelemetry metrics export.
//!
//! Exports all numeric fields from a [`Snapshot`] as OTEL gauge metrics
//! to an OTLP-compatible endpoint (default: `http://localhost:4317`).
//!
//! BL-073: Supports `--otel-level core|full` to control which metrics are exported.

#[cfg(feature = "otel")]
use crate::proc::{FieldValue, Snapshot};

#[cfg(feature = "otel")]
use opentelemetry::metrics::MeterProvider;
#[cfg(feature = "otel")]
use opentelemetry_otlp::WithExportConfig;
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
    "meminfo", "loadavg", "stat", "net/dev", "uptime", "df",
    "vmstat", "pressure", "file-nr", "version",
];

/// Run the OpenTelemetry export loop (headless, no TUI).
///
/// Captures a snapshot every `interval_secs` and pushes all numeric fields
/// as gauge metrics to the given OTLP endpoint.
#[cfg(feature = "otel")]
pub fn run_otel_export(endpoint: &str, interval_secs: u64) -> anyhow::Result<()> {
    run_otel_export_with_level(endpoint, interval_secs, OtelLevel::Full)
}

/// Run the OpenTelemetry export loop with a specific metrics level (BL-073).
#[cfg(feature = "otel")]
pub fn run_otel_export_with_level(endpoint: &str, interval_secs: u64, level: OtelLevel) -> anyhow::Result<()> {
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

        let provider = SdkMeterProvider::builder()
            .with_reader(reader)
            .build();

        let meter = provider.meter("syslenz");

        let level_label = if level == OtelLevel::Core { "core" } else { "full" };
        eprintln!("syslenz: OTEL export to {} (interval: {}s, level: {})", endpoint, interval_secs, level_label);
        eprintln!("Press Ctrl+C to stop.");

        loop {
            match Snapshot::capture() {
                Ok(snapshot) => {
                    export_snapshot_metrics(&meter, &snapshot, level);
                }
                Err(e) => {
                    eprintln!("Capture error: {}", e);
                }
            }
            tokio::time::sleep(Duration::from_secs(interval_secs)).await;
        }
    })
}

#[cfg(feature = "otel")]
fn export_snapshot_metrics(meter: &opentelemetry::metrics::Meter, snapshot: &Snapshot, level: OtelLevel) {
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
                let gauge = meter.f64_gauge(metric_name).build();
                gauge.record(v, &[]);
            }
        }
    }
}

/// Stub when otel feature is not enabled.
#[cfg(not(feature = "otel"))]
pub fn run_otel_export_with_level(_endpoint: &str, _interval_secs: u64, _level: OtelLevel) -> anyhow::Result<()> {
    anyhow::bail!(
        "OpenTelemetry support is not compiled in.\n\
         Rebuild with: cargo build --features otel"
    )
}

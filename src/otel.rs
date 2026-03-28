//! OpenTelemetry metrics export.
//!
//! Exports all numeric fields from a [`Snapshot`] as OTEL gauge metrics
//! to an OTLP-compatible endpoint (default: `http://localhost:4317`).

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

/// Run the OpenTelemetry export loop (headless, no TUI).
///
/// Captures a snapshot every `interval_secs` and pushes all numeric fields
/// as gauge metrics to the given OTLP endpoint.
#[cfg(feature = "otel")]
pub fn run_otel_export(endpoint: &str, interval_secs: u64) -> anyhow::Result<()> {
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

        eprintln!("syslenz: OTEL export to {} (interval: {}s)", endpoint, interval_secs);
        eprintln!("Press Ctrl+C to stop.");

        loop {
            match Snapshot::capture() {
                Ok(snapshot) => {
                    export_snapshot_metrics(&meter, &snapshot);
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
fn export_snapshot_metrics(meter: &opentelemetry::metrics::Meter, snapshot: &Snapshot) {
    for (source_key, entry) in &snapshot.entries {
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
pub fn run_otel_export(_endpoint: &str, _interval_secs: u64) -> anyhow::Result<()> {
    anyhow::bail!(
        "OpenTelemetry support is not compiled in.\n\
         Rebuild with: cargo build --features otel"
    )
}

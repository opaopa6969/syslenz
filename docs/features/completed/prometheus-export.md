# Prometheus Export

- **Status**: completed
- **Version**: v1.3.0
- **Source module**: `src/export/prometheus.rs`
- **User docs**: [en](../../en/otel.md#prometheus-export-v130) | [ja](../../ja/otel.md)

## Summary

Built-in Prometheus metrics endpoint via the `--prometheus` flag. Starts a lightweight
HTTP server that exposes all numeric syslenz metrics at `/metrics` in standard
Prometheus exposition format. Unlike the `--otel` mode, this does not require an
external OTLP collector and can run alongside the interactive TUI.

## Key capabilities

- `--prometheus [port]` flag starts HTTP server (default port 9101)
- `/metrics` endpoint in standard Prometheus exposition format
- All numeric fields exported as gauges with `syslenz_<source>_<field>` naming
- Runs alongside TUI (non-blocking background server)
- No external collector required -- Prometheus scrapes syslenz directly
- Compatible with Grafana dashboards via Prometheus data source
- Configurable via `[prometheus]` section in config.toml
- Requires `otel` compile-time feature flag

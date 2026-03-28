---
version: v1.0.0
lang: en
---

# OpenTelemetry Integration

[🇯🇵 日本語版](../ja/otel.md)

[<- Prev: Keybindings](keybindings.md) | [Index](index.md) | [Next: Data Sources ->](sources.md)


## Table of Contents

- [Overview](#overview)
- [Setup](#setup)
- [Metrics Exported](#metrics-exported)
- [Metric Naming Convention](#metric-naming-convention)
- [Docker Compose with Prometheus and Grafana](#docker-compose-with-prometheus-and-grafana)
- [Comparison with node_exporter](#comparison-with-node_exporter)
- [Configuration](#configuration)

## Overview

syslenz can export all numeric system metrics to any OpenTelemetry Protocol (OTLP)-compatible backend. This runs as a headless daemon (no TUI) that captures snapshots at a configurable interval and pushes gauge metrics via gRPC.

The OTEL feature requires compile-time opt-in:

```bash
cargo build --release --features otel
```

## Setup

### Basic usage

```bash
# Export to default endpoint (http://localhost:4317) every 5 seconds

[🇯🇵 日本語版](../ja/otel.md)
syslenz --otel

# Custom endpoint

[🇯🇵 日本語版](../ja/otel.md)
syslenz --otel http://otel-collector.example.com:4317

# Custom interval (10 seconds)

[🇯🇵 日本語版](../ja/otel.md)
syslenz --otel http://localhost:4317 --interval 10
```

The process runs in the foreground and prints:

```
syslenz: OTEL export to http://localhost:4317 (interval: 5s)
Press Ctrl+C to stop.
```

### If OTEL feature is not compiled in

```
OpenTelemetry support is not compiled in.
Rebuild with: cargo build --features otel
```

## Metrics Exported

syslenz exports **every numeric field** from every data source as an OTLP gauge metric. This includes:

- All `Bytes` fields (exported as `f64`)
- All `Integer` fields (exported as `f64`)
- All `Float` fields
- All `Duration` fields (in seconds, as `f64`)

Non-numeric fields (`Text`, `Table`) are skipped.

On a typical system with 55+ sources, this produces several hundred metrics covering:

- Memory (MemTotal, MemAvailable, MemFree, Cached, Buffers, SwapTotal, SwapFree, etc.)
- CPU (cpu_user, cpu_system, cpu_idle, cpu_iowait, cpu_steal, context_switches, etc.)
- Load (load1, load5, load15)
- Network (rx_bytes, tx_bytes, rx_packets, tx_packets per interface)
- Disk (reads_completed, writes_completed, io_time_ms, etc.)
- Pressure (cpu_some_avg10, memory_some_avg10, io_some_avg10, etc.)
- Thermal (max_temp, per-zone temperatures)
- File descriptors (allocated_fds, max_fds, fd_usage_pct)
- And more from all 55+ sources

## Metric Naming Convention

Metrics follow the pattern:

```
syslenz.<source>.<field_name>
```

Slashes in source names are replaced with dots. Examples:

| Source | Field | Metric Name |
|--------|-------|-------------|
| `meminfo` | `MemTotal` | `syslenz.meminfo.MemTotal` |
| `loadavg` | `load1` | `syslenz.loadavg.load1` |
| `net/dev` | `rx_bytes` | `syslenz.net.dev.rx_bytes` |
| `stat` | `cpu_user` | `syslenz.stat.cpu_user` |
| `pressure` | `cpu_some_avg10` | `syslenz.pressure.cpu_some_avg10` |
| `df` | `root_use_pct` | `syslenz.df.root_use_pct` |
| `thermal` | `max_temp` | `syslenz.thermal.max_temp` |

All metrics are exported as OTLP gauges (not counters or histograms) because snapshot values represent point-in-time state.

## Docker Compose with Prometheus and Grafana

A complete monitoring stack using syslenz as the metrics source:

```yaml
version: "3.8"

services:
  # syslenz OTEL exporter
  syslenz:
    build:
      context: .
      args:
        FEATURES: otel
    command: ["syslenz", "--otel", "http://otel-collector:4317", "--interval", "5"]
    pid: host
    privileged: true
    depends_on:
      - otel-collector

  # OpenTelemetry Collector
  otel-collector:
    image: otel/opentelemetry-collector-contrib:latest
    volumes:
      - ./otel-config.yaml:/etc/otelcol-contrib/config.yaml
    ports:
      - "4317:4317"   # OTLP gRPC
      - "8889:8889"   # Prometheus exporter

  # Prometheus
  prometheus:
    image: prom/prometheus:latest
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
    ports:
      - "9090:9090"
    depends_on:
      - otel-collector

  # Grafana
  grafana:
    image: grafana/grafana:latest
    ports:
      - "3001:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    depends_on:
      - prometheus
```

### otel-config.yaml

```yaml
receivers:
  otlp:
    protocols:
      grpc:
        endpoint: 0.0.0.0:4317

exporters:
  prometheus:
    endpoint: 0.0.0.0:8889

service:
  pipelines:
    metrics:
      receivers: [otlp]
      exporters: [prometheus]
```

### prometheus.yml

```yaml
global:
  scrape_interval: 10s

scrape_configs:
  - job_name: 'syslenz'
    static_configs:
      - targets: ['otel-collector:8889']
```

### Grafana Dashboard

After starting the stack:

1. Open Grafana at `http://localhost:3001` (admin/admin)
2. Add Prometheus data source: `http://prometheus:9090`
3. Create a dashboard with queries like:
   - `syslenz_meminfo_MemAvailable` -- memory available over time
   - `syslenz_loadavg_load1` -- 1-minute load average
   - `syslenz_stat_cpu_user` -- CPU user time
   - `syslenz_pressure_cpu_some_avg10` -- CPU pressure

## Comparison with node_exporter

| Feature | syslenz OTEL | node_exporter |
|---------|-------------|---------------|
| Protocol | OTLP (push) | Prometheus (pull) |
| Metrics | All syslenz sources (55+) | ~100 collectors |
| Types | All gauges | Gauges + counters |
| Setup | Single binary | Single binary |
| Custom sources | Plugin system | Custom collectors |
| Structured data | Schema-driven types | Raw floats |
| TUI | Yes (same binary) | No |
| Educational content | Built-in | No |

**When to use syslenz OTEL:**
- You already use syslenz for interactive monitoring and want the same data in dashboards
- You use an OTLP-native backend (e.g., Datadog, Honeycomb, Grafana Cloud)
- You want the plugin system for custom metrics

**When to use node_exporter:**
- You want production-hardened metrics with well-defined types (counters vs gauges)
- You need specific collectors (systemd, textfile, NTP)
- You are in a pure Prometheus ecosystem

## Configuration

In `~/.config/syslenz/config.toml`:

```toml
[otel]
endpoint = "http://otel-collector:4317"
interval_secs = 10
```

CLI flags override config values. See the [Configuration reference](config.md) for details.

---

[<- Prev: Keybindings](keybindings.md) | [Index](index.md) | [Next: Data Sources ->](sources.md)

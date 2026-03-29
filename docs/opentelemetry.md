# OpenTelemetry and Prometheus Integration

syslenz can export all numeric system metrics to external monitoring backends via two methods:

1. **OpenTelemetry (OTLP)** -- push-based, requires the `otel` feature flag
2. **Prometheus** -- pull-based, built-in HTTP `/metrics` endpoint (no feature flag needed)

## Table of Contents

- [OpenTelemetry Export](#opentelemetry-export)
  - [Building with the otel feature](#building-with-the-otel-feature)
  - [Usage](#usage)
  - [What gets exported](#what-gets-exported)
  - [Metric naming convention](#metric-naming-convention)
- [Prometheus Export](#prometheus-export)
  - [Usage](#prometheus-usage)
  - [Scrape configuration](#scrape-configuration)
- [Configuration](#configuration)
- [Docker Compose Quick Start](#docker-compose-quick-start)
- [Comparison: --otel vs --prometheus vs node_exporter](#comparison)

---

## OpenTelemetry Export

### Building with the otel feature

The OTEL export requires compile-time opt-in because it adds gRPC dependencies (tonic, opentelemetry_sdk, tokio):

```bash
cargo build --release --features otel
```

Without the feature flag, running `syslenz --otel` prints an error:

```
OpenTelemetry support is not compiled in.
Rebuild with: cargo build --features otel
```

### Usage

```bash
# Export to default endpoint (http://localhost:4317) every 5 seconds
syslenz --otel

# Custom endpoint
syslenz --otel http://otel-collector.example.com:4317

# Custom interval (10 seconds)
syslenz --otel http://localhost:4317 --interval 10

# Japanese metric descriptions
syslenz --otel --lang ja

# Core metrics only with Japanese descriptions
syslenz --otel --otel-level core --lang ja
```

The `--otel` flag runs syslenz in headless mode (no TUI). It captures a full system snapshot at each interval and pushes all numeric fields as OTLP gauge metrics via gRPC.

Output on startup:

```
syslenz: OTEL export to http://localhost:4317 (interval: 5s, level: full, locale: EN)
Press Ctrl+C to stop.
```

### What gets exported

Every numeric field from every data source is exported as an OTLP gauge with rich metadata:

| Field Type | Exported As | Unit | Example |
|------------|-------------|------|---------|
| `Bytes` | `f64` | `By` | MemTotal, MemAvailable, rx_bytes |
| `Integer` | `f64` | (from field) | context_switches, processes_total |
| `Float` | `f64` | (from field) | load1, cpu_user_pct |
| `Duration` | `f64` | `s` | uptime |

Non-numeric fields (`Text`, `Table`) are skipped.

On a typical Linux system with 50+ sources, this produces several hundred metrics covering memory, CPU, load, network interfaces, disk I/O, pressure stall info, thermal zones, file descriptors, and more.

### Resource attributes

Each metric export includes OTEL resource attributes for host identification:

| Attribute | Source | Example |
|-----------|--------|---------|
| `service.name` | hardcoded | `syslenz` |
| `service.version` | Cargo.toml | `1.3.0` |
| `host.name` | `/proc/sys/kernel/hostname` | `web-server-01` |
| `os.type` | `/proc/sys/kernel/ostype` | `Linux` |
| `os.version` | `/proc/sys/kernel/osrelease` | `6.1.0-25-amd64` |
| `os.description` | `/proc/version` | `Linux version 6.1.0-25-amd64 ...` |

### Metric descriptions (i18n)

Each metric includes a human-readable description from syslenz's i18n system. Use `--lang ja` to export with Japanese descriptions:

```
# English (default)
syslenz.meminfo.MemAvailable → "Available memory for new processes"

# Japanese (--lang ja)
syslenz.meminfo.MemAvailable → "新しいプロセスに使えるメモリ"
```

### Metric attributes

Each metric data point includes attributes:

| Attribute | Description | Example |
|-----------|-------------|---------|
| `syslenz.source` | The /proc source key | `meminfo`, `net/dev` |
| `syslenz.metric_type` | Semantic type hint | `gauge` or `counter` |

The `syslenz.metric_type` attribute indicates whether the value is a point-in-time measurement (`gauge`) or a monotonically increasing cumulative value (`counter`). Counter-type metrics (like `context_switches`, `rx_bytes`, `cpu_user`) should have `rate()` or `irate()` applied in dashboards.

### Metric naming convention

```
syslenz.<source>.<field_name>
```

Slashes in source names are replaced with dots:

| Source | Field | OTLP Metric Name |
|--------|-------|-------------------|
| `meminfo` | `MemTotal` | `syslenz.meminfo.MemTotal` |
| `loadavg` | `load1` | `syslenz.loadavg.load1` |
| `net/dev` | `rx_bytes` | `syslenz.net.dev.rx_bytes` |
| `stat` | `cpu_user` | `syslenz.stat.cpu_user` |
| `pressure` | `cpu_some_avg10` | `syslenz.pressure.cpu_some_avg10` |
| `df` | `root_use_pct` | `syslenz.df.root_use_pct` |
| `thermal` | `max_temp` | `syslenz.thermal.max_temp` |
| `gpu` | `gpu_utilization` | `syslenz.gpu.gpu_utilization` |

All metrics use the OTEL gauge instrument because syslenz reads absolute values from /proc (not deltas). For counter-type values, use the `syslenz.metric_type` attribute to identify which metrics need `rate()` in your dashboards.

---

## Prometheus Export

### Prometheus Usage

No feature flag required. The Prometheus exporter is always available:

```bash
# Start on default port (9101)
syslenz --prometheus

# Custom bind address
syslenz --prometheus 0.0.0.0:9102
```

This starts an HTTP server alongside syslenz. The single endpoint is:

```
GET http://localhost:9101/metrics
```

Output format follows the Prometheus exposition standard:

```
# HELP syslenz_meminfo_MemAvailable Memory available to applications in bytes
# TYPE syslenz_meminfo_MemAvailable gauge
syslenz_meminfo_MemAvailable 8.589934592e+09
# HELP syslenz_loadavg_load1 1-minute load average
# TYPE syslenz_loadavg_load1 gauge
syslenz_loadavg_load1 2.41
```

Prometheus metric names use underscores: `syslenz_<source>_<field>`.

### Scrape configuration

Add to your `prometheus.yml`:

```yaml
scrape_configs:
  - job_name: 'syslenz'
    scrape_interval: 5s
    static_configs:
      - targets: ['localhost:9101']
```

---

## Configuration

In `~/.config/syslenz/config.toml`:

```toml
[otel]
endpoint = "http://otel-collector:4317"   # OTLP gRPC endpoint
interval_secs = 10                         # Capture interval in seconds
```

CLI flags override config values:
- `--otel <endpoint>` overrides `otel.endpoint`
- `--interval <secs>` overrides `otel.interval_secs`

---

## Docker Compose Quick Start

A complete monitoring stack: syslenz -> OTel Collector -> Prometheus -> Grafana.

### docker-compose.yml

```yaml
version: "3.8"

services:
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

  otel-collector:
    image: otel/opentelemetry-collector-contrib:latest
    volumes:
      - ./otel-config.yaml:/etc/otelcol-contrib/config.yaml
    ports:
      - "4317:4317"
      - "8889:8889"

  prometheus:
    image: prom/prometheus:latest
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
    ports:
      - "9090:9090"
    depends_on:
      - otel-collector

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

### Simpler alternative: Prometheus-only (no OTel Collector)

```yaml
version: "3.8"

services:
  syslenz:
    build: .
    command: ["syslenz", "--prometheus", "0.0.0.0:9101"]
    pid: host
    privileged: true
    ports:
      - "9101:9101"

  prometheus:
    image: prom/prometheus:latest
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
    ports:
      - "9090:9090"

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3001:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
```

With `prometheus.yml` pointing to `syslenz:9101`.

### Grafana dashboard queries

After starting the stack, open Grafana at `http://localhost:3001` (admin/admin), add Prometheus as data source, then use queries like:

```promql
# Memory usage ratio
syslenz_meminfo_MemAvailable / syslenz_meminfo_MemTotal

# Load per CPU
syslenz_loadavg_load1 / syslenz_stat_cpu_count

# Context switch rate
rate(syslenz_stat_context_switches[5m])

# GPU utilization (v1.3.0)
syslenz_gpu_gpu_utilization

# Failed systemd units (v1.3.0)
syslenz_systemd_failed_failed_count
```

---

## Comparison

| Feature | `syslenz --otel` | `syslenz --prometheus` | `node_exporter` |
|---------|-------------------|------------------------|-----------------|
| Protocol | Push (OTLP gRPC) | Pull (HTTP scrape) | Pull (HTTP scrape) |
| Feature flag | `otel` required | None | N/A |
| Requires collector | Yes | No | No |
| Runs alongside TUI | No (headless) | Yes | N/A |
| Metric count | 300+ | 300+ | ~100 collectors |
| Custom sources | Plugin system | Plugin system | Custom collectors |
| Educational content | Built-in | Built-in | No |
| Unique metrics | pressure, buddyinfo, slabinfo, net/tcp states, cgroups | Same | Partial |

**Choose `--otel`** when you use an OTLP-native backend (Datadog, Honeycomb, Grafana Cloud OTLP) or need push-based export (no inbound access).

**Choose `--prometheus`** when you already have Prometheus and want the simplest setup with no extra infrastructure.

---

See also: [English docs](en/otel.md) | [Japanese docs](ja/otel.md)

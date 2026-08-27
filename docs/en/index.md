---
version: v1.4.0
lang: en
---

# syslenz Documentation

[🇯🇵 日本語版](../ja/index.md)


> **Wireshark for /proc** -- a structured, schema-driven system information viewer

syslenz parses every `/proc`, `/sys`, and network configuration file on Linux into typed, structured data and presents it in a fast TUI with diffing, time-series graphs, auto-diagnostics, educational guides, and JSON export/import. Whether you are debugging a production incident at 3 AM or learning how Linux works, syslenz gives you immediate, structured insight into your system.

![demo](../assets/demo.gif)

## Table of Contents

- [Quick Install](#quick-install)
- [Feature Highlights](#feature-highlights)
- [Document Index](#document-index)

## Quick Install

**From source (Cargo):**

```bash
cargo install --path .
```

**With optional features:**

```bash
# Web UI support

[🇯🇵 日本語版](../ja/index.md)
cargo install --path . --features web

# OpenTelemetry export

[🇯🇵 日本語版](../ja/index.md)
cargo install --path . --features otel

# All features

[🇯🇵 日本語版](../ja/index.md)
cargo install --path . --features "web,otel,x11widget"
```

**Docker:**

```bash
docker run --rm -it --pid=host --privileged opaopa6969/syslenz
```

**Binary download:**

Download the latest release from the [GitHub Releases](https://github.com/opaopa6969/syslenz/releases) page and place it on your `PATH`.

## Feature Highlights

- **60+ data sources** -- `/proc`, `/sys`, GPU, systemd, network config, thermal, disk, conntrack, DNS, and more
- **Schema-driven parsing** -- every field has a type (Bytes, Integer, Float, Duration, Text, Table), optional unit, and human-readable description
- **Dashboard view** -- at-a-glance system health with load, memory, CPU, network, and disk
- **Auto-diagnostics** -- automatic detection of memory pressure, CPU overload, swap exhaustion, zombie processes, socket leaks, disk full, thermal throttling, FD exhaustion, DNS misconfiguration, and conntrack overflow
- **Educational category guides** -- learn Linux internals through structured stories about Memory, CPU, Network, Storage, and Processes
- **Snapshot diffing** -- see what changed between refreshes, highlighted in color
- **Time-series graphs** -- sparkline visualization of any numeric field over time (60-snapshot ring buffer)
- **JSON export/import** -- capture system state, share it, replay it later
- **Remote monitoring** -- SSH, Docker exec, and TCP server/client modes
- **Web UI** -- browser-based dashboard with SSE real-time streaming
- **Plugin system** -- extend syslenz with custom data sources via executable plugins
- **OpenTelemetry export** -- push all numeric metrics to any OTLP-compatible backend
- **Bilingual** -- full English and Japanese support (switch with `L` key or `--lang`)
- **Keyboard-driven TUI** -- sidebar navigation, drill-in, search, copy to clipboard
- **Diagnostics jump** -- navigate from diagnostic findings directly to the related metric
- **Learning breadcrumbs** -- "next step" hints at EXTRA help level (18 fields)
- **"Did you know?" Tips** -- random learning tips on the Welcome screen
- **Tutorial mode** -- `--tutorial` launches an 8-step guided walkthrough with live data
- **3 SDKs** -- Java (syslenz4j), Python (syslenz4py), Node.js (syslenz4node) for programmatic access
- **Grafana integration** -- `docker compose --profile grafana` for auto-provisioned dashboards

## What's New in v1.4.0

- **Diagnostics jump** -- navigate directly from a diagnostic finding to the related metric source (view_history stack + picker UI)
- **"Did you know?" Tips** -- random educational tips displayed dynamically on the Welcome view
- **Learning breadcrumbs** -- at EXTRA help level, 18 fields (EN/JA) show "next step" hints guiding deeper exploration
- **Interactive tutorial mode** -- `--tutorial` launches an 8-step guided walkthrough using live system data
- **SEE ALSO cross-links** -- 31 fields with 105 cross-references to related metrics
- **Python SDK (syslenz4py)** -- connect to syslenz from Python (`sdk/python/`)
- **Node.js SDK (syslenz4node)** -- connect to syslenz from Node.js (`sdk/node/`)
- **OTEL bridge improvements** -- resource attributes, i18n descriptions, counter detection
- **Provider contribution guide** -- step-by-step guide for creating new providers (JA/EN, with template)
- **Grafana dashboard provisioning** -- `docker compose --profile grafana` sets up Prometheus + Grafana with pre-built dashboards
- **MetricKind / CommonMetric enums** -- typed metric classification (8 variants) and cross-platform metrics (15 metrics)
- **Diagnostics expanded to 27 check functions** (40+ patterns) -- diagnostic deep-dive with related_metrics linking

## What's New in v1.3.0

- **GPU monitoring** -- real-time GPU utilization, memory, temperature, power draw, and per-process GPU usage for NVIDIA (via NVML) and AMD (via sysfs) GPUs; GPU metrics appear in the Dashboard, Classic mode, and are exported via OTLP/Prometheus
- **systemd integration** -- new data sources for systemd unit status, failed services, and timers; failed service count feeds into the diagnostics engine
- **Prometheus export** -- new `--prometheus` flag starts a built-in HTTP server exposing a `/metrics` endpoint in standard Prometheus exposition format, eliminating the need for an OTLP collector; works alongside the TUI
- **Provider ecosystem** -- curated executable plugins for MySQL, PostgreSQL, Redis, and nginx; install them from `providers/` into the plugins directory
- **14 new diagnostic patterns** -- memory leak detection, swap activity monitoring, OOM kill tracking, network error alerting, recent reboot detection, load trend analysis, high-memory process flagging, orphaned TCP socket detection, IP forwarding check, kernel taint inspection, inode pressure monitoring, context switch rate analysis, conntrack growth rate tracking, and TCP listen port auditing
- **60+ data sources** -- expanded from 55+ with the addition of GPU, systemd, and inode sources

## What's New in v1.1.0

- **Time-travel diff** -- use `[` and `]` keys in Diff view to compare with older or newer snapshots, with a T-N indicator showing which snapshot pair you are viewing
- **Alert system** -- define custom `[[alert]]` rules in config.toml to monitor field thresholds; alerts appear in the status bar, color the sidebar, and mark individual fields
- **AA bar graphs in Dashboard** -- RAM, Swap, and CPU sections now display ASCII-art bar graphs (`████░░░░`) for instant visual resource utilization
- **Sparkline graphs in Dashboard** -- load average and memory history are visualized with sparkline characters (`▁▂▃▄▅▆▇█`) directly in the dashboard
- **Auto-sparkline in Detail view** -- numeric fields in Classic mode automatically display a sparkline graph below the value, showing recent history at a glance
- **Visible search bar** -- pressing `/` now shows a search input with a visible cursor in the status bar, making it clear that search mode is active
- **[Enter to expand] indicator** -- table-type fields in the detail view now display an `[Enter to expand]` hint, making discoverability easier for new users
- **ViewData unified UI** -- the TUI and Web UI now render from the same `ViewData` structure, ensuring identical data presentation across both interfaces
- **Web UI improvements** -- auto-graph rendering for numeric fields, PgUp/PgDn keyboard navigation, and Category Guide served directly from the server
- **i18n expansion** -- internationalization coverage has grown to 390 out of 600 fields with translated descriptions

## Document Index

| Document | Description |
|----------|-------------|
| [Getting Started](getting-started.md) | Installation, first run, basic navigation, CLI flags |
| [Dashboard](dashboard.md) | Dashboard view: metrics, sections, navigation |
| [Classic Mode](classic-mode.md) | Sidebar + detail view, search, diff, graph |
| [Diagnostics](diagnostics.md) | Auto-diagnostics engine, severity levels, all checks |
| [Education](education.md) | Category guides, help levels, learning Linux internals |
| [Remote Monitoring](remote.md) | SSH, Docker, TCP server/client modes |
| [Web UI](web-ui.md) | Browser-based dashboard setup and usage |
| [Plugins](plugins.md) | Plugin system, writing custom plugins |
| [Configuration](config.md) | config.toml reference, all options |
| [Keybindings](keybindings.md) | Complete keybinding reference by view |
| [OpenTelemetry](otel.md) | OTLP metrics export, Prometheus/Grafana setup |
| [Data Sources](sources.md) | All 55+ data sources: what they read and key fields |
| [Provider Contribution Guide](provider-contribution-guide.md) | How to create new providers, template, testing |

---

[Index](index.md) | [Next: Getting Started ->](getting-started.md)

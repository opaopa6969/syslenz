---
version: v1.0.0
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
cargo install --path . --features web

# OpenTelemetry export
cargo install --path . --features otel

# All features
cargo install --path . --features "web,otel,x11widget"
```

**Docker:**

```bash
docker run --rm -it --pid=host --privileged syslenz/syslenz
```

**Binary download:**

Download the latest release from the [GitHub Releases](https://github.com/opaopa6969/syslenz/releases) page and place it on your `PATH`.

## Feature Highlights

- **55+ data sources** -- `/proc`, `/sys`, network config, thermal, disk, conntrack, DNS, and more
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

---

[Index](index.md) | [Next: Getting Started ->](getting-started.md)

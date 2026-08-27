---
version: v1.4.0
lang: en
---

# Getting Started

[日本語版](../ja/getting-started.md)

[← Index](index.md) | [Next: Dashboard →](dashboard.md)

---

## Table of Contents

- [Requirements](#requirements)
- [Installation](#installation)
  - [One-liner installer](#one-liner-installer)
  - [cargo install (crates.io)](#cargo-install-cratesio)
  - [From source](#from-source)
  - [Optional features](#optional-features)
  - [Docker](#docker)
  - [Pre-built binary](#pre-built-binary)
- [First Run](#first-run)
- [Basic Navigation](#basic-navigation)
- [Switching Views](#switching-views)
- [Getting Help](#getting-help)
- [Serving metrics (--serve)](#serving-metrics---serve)
- [Web UI (--web)](#web-ui---web)
- [CLI Flags Reference](#cli-flags-reference)

---

## Requirements

- **Linux** (primary): any kernel 3.10+ with `/proc` and `/sys`
- **macOS** / **Windows**: supported via platform adapters (24 sources each)
- **Rust**: edition 2024, rustc 1.85+ (for building from source)

---

## Installation

### One-liner installer

```bash
curl -sSf https://raw.githubusercontent.com/opaopa6969/syslenz/main/scripts/install.sh | sh
```

Downloads the latest pre-built binary for your platform and places it in `/usr/local/bin/`.

### cargo install (crates.io)

```bash
cargo install syslenz
```

Installs the default feature set (`web` enabled). Requires Rust 1.85+.

### From source

```bash
git clone https://github.com/opaopa6969/syslenz.git
cd syslenz
cargo build --release
sudo cp target/release/syslenz /usr/local/bin/
```

Or install directly from the cloned directory:

```bash
cargo install --path .
```

### Optional features

syslenz has three optional compile-time features:

```bash
# Web UI (HTTP server, Settings GUI — included in default build)
cargo build --release --features web

# OpenTelemetry metrics export
cargo build --release --features otel

# X11 floating widget
cargo build --release --features x11widget

# All features
cargo build --release --features "web,otel,x11widget"
```

### Docker

```bash
# Run interactively (TUI inside container)
docker run --rm -it --pid=host --privileged opaopa6969/syslenz

# Export a snapshot to stdout
docker run --rm --pid=host --privileged opaopa6969/syslenz --export /dev/stdout > snapshot.json

# TCP server mode — listen on port 9100 (no auth; use on trusted networks only)
docker run --rm -p 9100:9100 --pid=host opaopa6969/syslenz --serve

# Web UI
docker compose up -d
# Open http://localhost:3000

# Grafana + Prometheus + syslenz
docker compose --profile grafana up -d
# Open http://localhost:3001 (Grafana), http://localhost:9090 (Prometheus)
```

The `--pid=host` and `--privileged` flags let the container read the host's `/proc` filesystem.

### Pre-built binary

Pre-built binaries for Linux (x86_64, aarch64, musl), macOS (x86_64, aarch64), and Windows are available on the [GitHub Releases](https://github.com/opaopa6969/syslenz/releases) page.

```bash
# Linux x86_64
curl -L https://github.com/opaopa6969/syslenz/releases/latest/download/syslenz-linux-x86_64.tar.gz | tar xz
sudo mv syslenz /usr/local/bin/

# Verify checksum
sha256sum -c syslenz-linux-x86_64.tar.gz.sha256
```

---

## First Run

```bash
syslenz
```

You will see the **Dashboard** view — a full-width overview of your system with:

- Load averages (1, 5, 15 min) with sparkline history
- Memory usage (total, available, cached) with bar graph
- CPU utilization breakdown
- Network interface traffic
- Disk usage
- Active process summary

The dashboard auto-refreshes every second by default.

### Import mode (read-only replay)

```bash
syslenz --import snapshot.json
```

Opens in Classic mode with auto-refresh disabled. Use this to inspect a snapshot captured on another machine or at a past point in time.

---

## Basic Navigation

syslenz is fully keyboard-driven. These keys work in all views:

| Key | Action |
|-----|--------|
| `j` / `k` or Arrow Down/Up | Move selection down / up |
| `Enter` or Arrow Right | Drill into selected item |
| `Backspace` or Arrow Left | Go back |
| `Tab` | Toggle focus between sidebar and content |
| `PageUp` / `PageDown` | Scroll by page |
| `q` or `Esc` | Quit |

### In the Dashboard

- Use `j`/`k` to select a section (Load, Memory, CPU, Network, etc.)
- Press `Enter` to drill into that section's detailed view in Classic mode
- Press `Backspace` to return to the Dashboard

### In Classic mode

- The **sidebar** lists all data sources (`meminfo`, `loadavg`, `net/tcp`, …)
- Use `j`/`k` to navigate sources, `Enter` to view fields
- Press `Tab` to switch focus between sidebar and detail panel
- In the detail panel, use `j`/`k` to scroll through fields

---

## Switching Views

| Key | View | Description |
|-----|------|-------------|
| `D` | Dashboard | System health overview (default at startup) |
| `O` | Classic | Sidebar + detail panel |
| `W` | Welcome | Keybinding reference and tips |
| `X` | Diagnostics | Auto-detected issues with suggested actions |
| `C` | Category Guide | Educational Linux internals guide |

Press `d` from any view to enter the **Diff** view (compare current snapshot with a previous one).

---

## Getting Help

syslenz has a built-in multi-level help system:

| Key | Action |
|-----|--------|
| `?` | Cycle help level: OFF → NORMAL → DETAILED → EXTRA → OFF |
| `L` | Switch language (English ↔ Japanese) |

When help is active, a panel appears at the bottom of the screen with contextual information about the current field. The EXTRA level shows SEE ALSO cross-references and learning breadcrumbs.

---

## Serving metrics (--serve)

`--serve` starts a lightweight TCP server that responds to `SNAPSHOT` requests with JSON:

```bash
# Start server on all interfaces (default port 9100)
syslenz --serve

# Restrict to loopback — recommended for shared hosts
syslenz --serve 127.0.0.1:9100

# Connect from another terminal or remote machine
syslenz --connect localhost:9100
```

> **Security**: `--serve` has no authentication. On shared or internet-facing hosts, bind to `127.0.0.1` or use a firewall rule to restrict access. SDKs (`syslenz4j`, `syslenz4py`, `syslenz4node`) connect to this endpoint.

---

## Web UI (--web)

`--web` starts an HTTP server with a browser dashboard (requires `web` feature, which is on by default):

```bash
# Start on port 3000
syslenz --web 3000

# Open in browser
# http://localhost:3000          — live dashboard
# http://localhost:3000/settings — Settings GUI (alert rule editor)
```

> **Security**: The web server has no authentication in the current release. Use only on localhost or behind a reverse proxy with TLS and auth when network-accessible.
>
> **Planned but not yet implemented**: Fleet View (`/fleet`) and authentication (Basic Auth / Token).

### Settings GUI

Open `http://localhost:3000/settings` to edit alert rules in the browser. Changes are saved to `~/.config/syslenz/config.toml` and take effect immediately without restarting syslenz.

---

## CLI Flags Reference

| Flag | Arguments | Description |
|------|-----------|-------------|
| `--export` | `<file.json>` | Export a snapshot to JSON and exit |
| `--import` | `<file.json>` | Open TUI with imported snapshot(s) |
| `--export-series` | `<dir>` | Export time-series snapshots to directory |
| `--interval` | `<seconds>` | Interval for `--export-series` and `--otel` |
| `--count` | `<n>` | Number of snapshots for `--export-series` |
| `--ssh` | `<user@host>` | Monitor a remote host via SSH (repeatable) |
| `--docker` | `<container>` | Monitor a Docker container via exec |
| `--connect` | `<host:port>` | Connect to a syslenz TCP server |
| `--serve` | `[bind_addr]` | Start TCP server (default: `0.0.0.0:9100`) |
| `--web` | `[addr:port]` | Start Web UI (default: `0.0.0.0:3000`, requires `web`) |
| `--otel` | `[endpoint]` | OTLP export (default: `http://localhost:4317`, requires `otel`) |
| `--prometheus` | `[port]` | Prometheus `/metrics` endpoint (default: 9101, requires `otel`) |
| `--widget` | — | X11 floating widget (requires `x11widget`) |
| `--lang` | `<en\|ja>` | Set UI language (overrides config) |
| `--classic` | — | Start in Classic mode instead of Dashboard |
| `--tutorial` | — | Launch the interactive 8-step tutorial |

### Examples

```bash
# Basic interactive use
syslenz

# Japanese interface
syslenz --lang ja

# Export a snapshot
syslenz --export snapshot.json

# Capture 60 snapshots, 1 per second
syslenz --export-series ./data --interval 1 --count 60

# Monitor a remote server
syslenz --ssh admin@192.168.1.100

# Monitor two hosts simultaneously (F1/F2 to switch tabs)
syslenz --ssh admin@host1 --ssh admin@host2

# Monitor a Docker container
syslenz --docker my-app-container

# Start a TCP server on loopback only (safe on shared hosts)
syslenz --serve 127.0.0.1:9100

# Connect to a remote TCP server
syslenz --connect 192.168.1.100:9100

# Start the web UI on port 8080
syslenz --web 8080

# Export metrics to an OTLP collector
syslenz --otel http://otel-collector:4317 --interval 10

# Prometheus metrics endpoint
syslenz --prometheus

# Prometheus on a custom port (installed providers are discovered automatically)
syslenz --prometheus 9102

# Launch tutorial mode
syslenz --tutorial
```

---

[← Index](index.md) | [Next: Dashboard →](dashboard.md)

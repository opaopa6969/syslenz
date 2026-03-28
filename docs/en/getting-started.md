---
version: v1.1.0
lang: en
---

# Getting Started

[🇯🇵 日本語版](../ja/getting-started.md)

[<- Prev: Index](index.md) | [Index](index.md) | [Next: Dashboard ->](dashboard.md)


## Table of Contents

- [Installation](#installation)
- [First Run](#first-run)
- [Basic Navigation](#basic-navigation)
- [Switching Views](#switching-views)
- [Getting Help](#getting-help)
- [CLI Flags Reference](#cli-flags-reference)

## Installation

### From Source (Cargo)

Requires Rust 2024 edition (1.85+).

```bash
git clone https://github.com/opaopa6969/syslenz.git
cd syslenz
cargo build --release
sudo cp target/release/syslenz /usr/local/bin/
```

Or install directly:

```bash
cargo install --path .
```

### Optional Features

syslenz has three optional compile-time features:

```bash
# Web UI (adds ~3 MB to binary, requires tokio + axum)

[🇯🇵 日本語版](../ja/getting-started.md)
cargo build --release --features web

# OpenTelemetry metrics export (requires tokio + OTLP crate)

[🇯🇵 日本語版](../ja/getting-started.md)
cargo build --release --features otel

# X11 floating widget

[🇯🇵 日本語版](../ja/getting-started.md)
cargo build --release --features x11widget

# Everything

[🇯🇵 日本語版](../ja/getting-started.md)
cargo build --release --features "web,otel,x11widget"
```

### Docker

```bash
# Run interactively

[🇯🇵 日本語版](../ja/getting-started.md)
docker run --rm -it --pid=host --privileged syslenz/syslenz

# Export a snapshot

[🇯🇵 日本語版](../ja/getting-started.md)
docker run --rm --pid=host --privileged syslenz/syslenz --export /dev/stdout > snapshot.json
```

The `--pid=host` and `--privileged` flags are needed so the container can read the host's `/proc` filesystem.

The repository includes a `Dockerfile` for building a minimal container image and a `docker-compose.yml` for quick setup:

```bash
# Build and run with Docker Compose (TCP server mode)

[🇯🇵 日本語版](../ja/getting-started.md)
docker compose up -d
syslenz --connect localhost:9100

# Web UI profile

[🇯🇵 日本語版](../ja/getting-started.md)
docker compose --profile web up -d
# Open http://localhost:3000

[🇯🇵 日本語版](../ja/getting-started.md)
```

A convenience script `run-web.sh` is also provided to build and launch the Web UI in one step:

```bash
./run-web.sh          # port 3000, English
./run-web.sh 8080     # port 8080, English
./run-web.sh 3000 ja  # port 3000, Japanese
```

See the [Remote Monitoring](remote.md) page for detailed Docker Compose configuration and the [Web UI](web-ui.md) page for browser-based access.

### Binary Download

Pre-built binaries for `x86_64-unknown-linux-gnu` are available on the [GitHub Releases](https://github.com/opaopa6969/syslenz/releases) page.

```bash
curl -L https://github.com/opaopa6969/syslenz/releases/latest/download/syslenz-linux-amd64 -o syslenz
chmod +x syslenz
sudo mv syslenz /usr/local/bin/
```

## First Run

Simply run:

```bash
syslenz
```

You will see the **Dashboard** view -- a full-screen overview of your system's health. The dashboard shows:

- System load averages (1, 5, 15 min)
- Memory usage (total, available, cached)
- CPU utilization breakdown
- Network interface traffic
- Disk usage
- Active process summary

The dashboard auto-refreshes every second by default.

### Import Mode (Read-Only)

You can also view a previously captured snapshot:

```bash
syslenz --import snapshot.json
```

This opens in Classic (Overview) mode with auto-refresh disabled, allowing you to inspect a snapshot from another machine or a past point in time.

## Basic Navigation

syslenz is fully keyboard-driven. The core navigation keys work in all views:

| Key | Action |
|-----|--------|
| `j` / `k` or Arrow Down/Up | Move selection down / up |
| `Enter` or Arrow Right | Drill into selected item |
| `Backspace` or Arrow Left | Go back to previous view |
| `Tab` | Toggle focus between sidebar and content |
| `PageUp` / `PageDown` | Scroll by page |
| `q` or `Esc` | Quit syslenz |

### In the Dashboard

- Use `j`/`k` to select a dashboard section (Load, Memory, CPU, Network, etc.)
- Press `Enter` to drill into that section's detailed view in Classic mode

### In Classic Mode

- The **sidebar** lists all data sources (e.g., `meminfo`, `loadavg`, `net/tcp`)
- Use `j`/`k` to navigate sources, `Enter` to view details
- Press `Tab` to switch focus between the sidebar and the detail panel
- In the detail panel, use `j`/`k` to scroll through fields

## Switching Views

syslenz has several views accessible via single-key shortcuts:

| Key | View | Description |
|-----|------|-------------|
| `D` | Dashboard | System health overview (default) |
| `O` | Overview (Classic) | Sidebar + detail panel |
| `W` | Welcome | Welcome screen with quick-start info |
| `X` | Diagnostics | Auto-diagnostics findings |
| `C` | Category Guide | Educational Linux internals guide |

You can switch between views at any time. The view you came from is remembered so `Backspace` takes you back.

## Getting Help

syslenz has a built-in multi-level help system:

| Key | Action |
|-----|--------|
| `?` | Cycle help level: OFF -> NORMAL -> DETAILED -> EXTRA -> OFF |
| `L` | Switch language (English <-> Japanese) |

When help is active, a panel appears at the bottom of the screen showing contextual information about the current view and selected item. Higher help levels provide more detail, including field descriptions and usage tips.

## CLI Flags Reference

| Flag | Arguments | Description |
|------|-----------|-------------|
| `--export` | `<file.json>` | Export a snapshot to JSON and exit |
| `--import` | `<file.json>` | Open TUI with imported snapshot(s) |
| `--export-series` | `<dir>` | Export time-series snapshots to directory |
| `--interval` | `<seconds>` | Interval for `--export-series` and `--otel` |
| `--count` | `<n>` | Number of snapshots for `--export-series` |
| `--ssh` | `<user@host>` | Monitor a remote host via SSH |
| `--docker` | `<container>` | Monitor a Docker container via exec |
| `--connect` | `<host:port>` | Connect to a syslenz TCP server |
| `--serve` | `[bind_addr]` | Start TCP server (default: `0.0.0.0:9100`) |
| `--web` | `[port]` | Start Web UI (default port: `3000`, requires `web` feature) |
| `--otel` | `[endpoint]` | Export metrics via OTLP (default: `http://localhost:4317`, requires `otel` feature) |
| `--widget` | | Start X11 floating widget (requires `x11widget` feature) |
| `--lang` | `<en\|ja>` | Set language (overrides config) |
| `--classic` | | Start in Classic mode instead of Dashboard |

### Examples

```bash
# Basic interactive use

[🇯🇵 日本語版](../ja/getting-started.md)
syslenz

# Export a snapshot

[🇯🇵 日本語版](../ja/getting-started.md)
syslenz --export snapshot.json

# Capture 60 snapshots, 1 per second

[🇯🇵 日本語版](../ja/getting-started.md)
syslenz --export-series ./data --interval 1 --count 60

# Monitor a remote server

[🇯🇵 日本語版](../ja/getting-started.md)
syslenz --ssh admin@192.168.1.100

# Monitor a Docker container

[🇯🇵 日本語版](../ja/getting-started.md)
syslenz --docker my-app-container

# Start a TCP server inside a container

[🇯🇵 日本語版](../ja/getting-started.md)
syslenz --serve 0.0.0.0:9100

# Connect to a remote TCP server

[🇯🇵 日本語版](../ja/getting-started.md)
syslenz --connect 192.168.1.100:9100

# Start the web UI on port 8080

[🇯🇵 日本語版](../ja/getting-started.md)
syslenz --web 8080

# Export metrics to an OTLP collector

[🇯🇵 日本語版](../ja/getting-started.md)
syslenz --otel http://otel-collector:4317 --interval 10

# Japanese interface

[🇯🇵 日本語版](../ja/getting-started.md)
syslenz --lang ja
```

---

[<- Prev: Index](index.md) | [Index](index.md) | [Next: Dashboard ->](dashboard.md)

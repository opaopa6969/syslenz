[日本語版はこちら / Japanese](README-ja.md)

# syslenz

[![CI](https://github.com/opaopa6969/syslenz/actions/workflows/ci.yml/badge.svg)](https://github.com/opaopa6969/syslenz/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/syslenz.svg)](https://crates.io/crates/syslenz)
[![Docker Pulls](https://img.shields.io/docker/pulls/opaopa6969/syslenz.svg)](https://hub.docker.com/r/opaopa6969/syslenz)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> **Wireshark for /proc** — every Linux metric as structured, typed data.

Zero config. One binary. Your data never leaves your machine.

---

## Table of Contents

- [Why syslenz](#why-syslenz)
- [Quick Start](#quick-start)
- [Install](#install)
- [TUI Views and Keybindings](#tui-views-and-keybindings)
- [HTTP Server and API](#http-server-and-api)
- [Settings GUI](#settings-gui)
- [Configuration](#configuration)
- [Data Sources (50+)](#data-sources-50)
- [Auto-Diagnostics](#auto-diagnostics)
- [Education Features](#education-features)
- [SDKs](#sdks)
- [Grafana Integration](#grafana-integration)
- [Architecture](#architecture)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [Roadmap](#roadmap)
- [License](#license)

---

## Why syslenz

| Axis | syslenz | SaaS Monitoring | htop/top |
|------|---------|-----------------|----------|
| **Time to Value** | 21 seconds | 7+ minutes | Instant but shallow |
| **Data Sovereignty** | 100% local | Cloud (vendor lock-in) | Local |
| **Education** | 4-level help, category guides, tutorial mode, article overlay | Docs site (separate) | None |
| **Embeddability** | SDK (Java, Python, Node.js), OTEL, Prometheus, JSON | API (paid tier) | None |
| **Cost** | Free / MIT | $15–35/host/month | Free |

### For the SRE

SSH in, run `syslenz`, see everything. No agents, no config, no setup. Export JSON, diff between hosts, pipe to `jq`, feed to CI. 50+ data sources out of the box.

### For the learner

Every field has a human-readable description at 4 detail levels. Press `?` to cycle through them. The Category Guide connects sources into narratives like "Where did all my RAM go?" and "The Life of a Packet."

### For the educator

Every metric links to a dedicated Article Overlay (TUI `A` / Web `A`) with SEE ALSO navigation, source guides, and concept stories. Dashboard controls (`s` for axis mode, `RT` badge/`r` key for real-time stream) keep the learning view stable.

### For the security auditor

Capture full system state as JSON, compare across hosts, track changes over time. Kernel modules, open connections, cgroup policies, mounted filesystems — all in one export. See [Audit Examples](docs/audit-examples.md).

---

## Quick Start

> [!CAUTION]
> `curl | sh` runs an arbitrary script from the internet. Review
> [`scripts/install.sh`](scripts/install.sh) before piping it to a shell, and pin
> the URL to a signed tag if you need integrity guarantees. crates.io and
> source installs below avoid this risk.

```bash
# One-liner install
curl -sSf https://raw.githubusercontent.com/opaopa6969/syslenz/main/scripts/install.sh | sh

# Launch the TUI
syslenz

# Navigate: j/k to move, Enter to drill in, Backspace to go back
# Press ? to cycle help levels (OFF → NORMAL → DETAILED → EXTRA)
# Press D for Dashboard, X for Diagnostics, C for Category Guide
# Press q to quit
```

---

## Install

> [!CAUTION]
> `curl | sh` executes an arbitrary script from the internet (MITM /
> repository-tampering risk). Review
> [`scripts/install.sh`](scripts/install.sh) first, or prefer the crates.io /
> source builds below, which do not pipe a remote script to a shell.

```bash
# One-liner install script
curl -sSf https://raw.githubusercontent.com/opaopa6969/syslenz/main/scripts/install.sh | sh

# From crates.io (Rust 1.85+ / edition 2024 required)
cargo install syslenz

# From source
git clone https://github.com/opaopa6969/syslenz.git
cd syslenz
cargo install --path .

# With optional features
cargo install --path . --features "otel,web"

# Docker — instant Web UI (no build, linux/amd64 + arm64)
docker run --rm -p 3000:3000 --pid=host opaopa6969/syslenz
# Open http://localhost:3000

# Docker — TCP server mode (no auth; use only on trusted networks)
docker run --rm -p 9100:9100 --pid=host opaopa6969/syslenz --serve
syslenz --connect localhost:9100

# Docker Compose — Web UI
docker compose up -d
# Open http://localhost:3000

# Docker Compose — Grafana + Prometheus + syslenz
docker compose --profile grafana up -d
# Open http://localhost:3001 (Grafana), http://localhost:9090 (Prometheus)

# Pre-built binary
# See https://github.com/opaopa6969/syslenz/releases
```

> **Security note**: `--serve` starts an unauthenticated TCP server. Bind to `127.0.0.1:9100` or use firewall rules when running on a shared or internet-facing host:
> ```bash
> syslenz --serve 127.0.0.1:9100   # loopback only
> ```

### Docker Hub

```bash
# Instant Web UI — no build, no install
docker run --rm -p 3000:3000 --pid=host opaopa6969/syslenz
```

| Tag | Description |
|-----|-------------|
| `latest` | Latest stable release |
| `v1.7.0` | Specific version |
| `1.7` | Latest patch for 1.7.x |

Platforms: `linux/amd64`, `linux/arm64`

---

## TUI Views and Keybindings

### View shortcuts

| Key | View | Description |
|-----|------|-------------|
| `D` | Dashboard | System overview: load, memory, CPU, network |
| `O` | Classic | Sidebar + detail panel (traditional mode) |
| `W` | Welcome | Keybinding reference and onboarding |
| `X` | Diagnostics | Auto-detected issues with suggested actions |
| `C` | Category Guide | Educational content by topic |
| `A` | Article Overlay | Full article for the selected metric |

### Navigation

| Key | Action |
|-----|--------|
| `j` / `k` | Navigate sources / fields |
| `Enter` / `Backspace` | Drill in / go back |
| `Tab` | Toggle sidebar / content focus |
| `/` | Search sources |
| `d` | Diff view |
| `g` | Graph (sparkline) |
| `s` | Toggle y-axis mode in graph |
| `?` | Cycle help level (OFF / NORMAL / DETAILED / EXTRA) |
| `L` | Toggle language (EN / JA) |
| `c` | Copy to clipboard |
| `e` | Export snapshot to JSON |
| `a` | Toggle auto-refresh |
| `r` | Manual refresh |
| `q` | Quit |

### Multi-host tabs

When monitoring multiple hosts (`--ssh`, `--docker`, `--connect`), use `F1`–`F9` to switch between host tabs.

---

## HTTP Server and API

When started with `--web`, syslenz exposes an HTTP server (default port 3000, requires `web` feature).

### Endpoints

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/` | Web UI dashboard |
| `GET` | `/api/snapshot` | Current snapshot as JSON |
| `GET` | `/api/history` | Snapshot history as JSON array |
| `GET` | `/api/sources` | Available data sources |
| `GET` | `/api/stream` | Server-Sent Events stream (live updates) |
| `GET` | `/api/view` | Rendered view data (TUI-equivalent) |
| `GET` | `/api/field-help` | Field description at given help level |
| `GET` | `/settings` | Settings GUI (browser) |
| `GET` | `/api/v1/settings` | Current config as JSON (API v1) |
| `POST` | `/api/v1/settings/alerts` | Write alert rules to config file (API v1) |

### API v1

Versioned endpoints carry the `X-Syslenz-API-Version: 1` response header. The `/api/v1/*` prefix is stable and will not change without a major version bump.

```bash
# Fetch current config
curl http://localhost:3000/api/v1/settings

# Push alert rules
curl -X POST http://localhost:3000/api/v1/settings/alerts \
  -H 'Content-Type: application/json' \
  -d '[{"source":"meminfo","field":"MemAvailable","condition":"< 500000000","severity":"critical","message":"Low memory"}]'
```

> **Security note**: The HTTP server has no authentication in the current release. Bind to loopback or place behind a reverse proxy with TLS and auth when exposing outside localhost.
>
> **Fleet View** (`/fleet`) and **authentication** (Basic Auth / Token) are planned for a future release and are **not yet implemented**.

---

## Settings GUI

Open `http://localhost:3000/settings` while `syslenz --web` is running.

The Settings GUI lets you:

- View the current configuration (loaded from `~/.config/syslenz/config.toml`)
- Add, edit, and delete alert rules
- Save changes back to the config file without restarting syslenz

---

## Configuration

```toml
# ~/.config/syslenz/config.toml

[general]
lang = "en"                 # "en" or "ja"
interval_ms = 1000          # auto-refresh interval
default_view = "dashboard"  # "dashboard" or "classic"

[web]
port = 3000

[otel]
endpoint = "http://localhost:4317"
interval_secs = 5

[ssh]
hosts = ["user@server1", "user@server2"]

[[alert]]
source = "meminfo"
field = "MemAvailable"
condition = "< 500000000"
severity = "critical"
message = "Memory critically low"
```

CLI flags override config values. See [`docs/en/config.md`](docs/en/config.md) for the full reference.

### Key CLI flags

| Flag | Description |
|------|-------------|
| `--classic` | Start in Classic sidebar mode |
| `--lang ja` | Japanese UI |
| `--ssh user@host` | Remote monitoring via SSH (repeatable) |
| `--docker container` | Docker container monitoring |
| `--serve [addr]` | TCP server mode (default: `0.0.0.0:9100`) |
| `--connect host:port` | Connect to a TCP server |
| `--web [port]` | Web UI (default: 3000, requires `web` feature) |
| `--export file.json` | Export snapshot as JSON |
| `--import file.json` | Replay mode from snapshot |
| `--prometheus [port]` | Prometheus `/metrics` endpoint |
| `--otel [endpoint]` | OpenTelemetry export (requires `otel` feature) |
| `--tutorial` | Interactive 8-step guided walkthrough |
| `--widget` | X11 floating widget (requires `x11widget` feature) |

---

## Data Sources (50+)

<details>
<summary><strong>/proc (43 sources — Linux)</strong></summary>

| Category | Sources |
|----------|---------|
| System | uptime, loadavg, version, cmdline, modules, filesystems, devices, consoles, misc, dma |
| Memory | meminfo, vmstat, zoneinfo, buddyinfo, slabinfo, pagetypeinfo, swaps |
| CPU | cpuinfo, stat, interrupts, softirqs, schedstat, timer_list, pressure |
| Storage | mounts, partitions, diskstats, locks |
| Network | net/dev, net/tcp, net/udp, net/unix, net/arp, net/route, net/sockstat, net/snmp, net/netstat, net/wireless |
| Security | crypto, cgroups, iomem, ioports |
| Processes | processes (all PIDs with name, state, RSS, threads, UID) |

</details>

<details>
<summary><strong>/sys (3 sources)</strong></summary>

| Source | Description |
|--------|-------------|
| df | Filesystem disk space usage (via statfs) |
| thermal | CPU/GPU temperature from thermal zones |
| file-nr | System-wide file descriptor usage |

</details>

<details>
<summary><strong>Network deep-dive (5 sources)</strong></summary>

| Source | Description |
|--------|-------------|
| ip/route | Full routing table with metrics and default gateway |
| ip/neighbor | ARP/NDP cache with reachability state |
| ss | Socket statistics (TCP established, TIME_WAIT, orphaned) |
| dns | DNS configuration + resolution speed test |
| conntrack | Connection tracking table usage |

</details>

<details>
<summary><strong>Plugins (unlimited)</strong></summary>

Drop any executable in `~/.config/syslenz/plugins/`. It outputs JSON to stdout; syslenz picks it up automatically.

Bundled examples: **JVM** (jstat/jcmd), **Docker** (container stats).

See `plugins/examples/` and [`docs/en/plugins.md`](docs/en/plugins.md) for details.

</details>

<details>
<summary><strong>Cross-platform</strong></summary>

| Platform | Sources | Method |
|----------|---------|--------|
| Linux | 51+ | /proc + /sys + commands |
| macOS | 24 | sysctl + vm_stat + system commands |
| Windows | 24 | PowerShell + WMI |

</details>

---

## Auto-Diagnostics

Press `X` in the TUI to run 27 check functions covering 40+ patterns:

- Memory pressure, swap exhaustion, OOM kills, memory leak detection
- CPU overload, load spikes, pressure stalls, context switch rate
- Disk usage, inode pressure, temperature warnings
- Network: SYN flood, CLOSE_WAIT leak, TIME_WAIT excess, orphaned TCP, retransmissions, UDP errors
- Zombie processes, D-state stuck processes, high-memory process flagging
- File descriptor exhaustion, DNS misconfiguration, conntrack overflow
- IP forwarding detection, kernel taint inspection, recent reboot notification

Use **Diagnostics Jump** to navigate directly from a finding to the related metric source.

---

## Education Features

- **4-level contextual help** — press `?` to cycle OFF / NORMAL / DETAILED / EXTRA
- **Article Overlay** (`A`) — full articles for every metric (691 metrics × EN + JA)
- **Category Guide** (`C`) — structured learning paths: Memory, CPU, Network, Storage, Process, Hardware
- **Learning Breadcrumbs** — at EXTRA level, 18 fields show "next step" hints
- **"Did you know?" Tips** — random tips on the Welcome screen (`W`)
- **Tutorial Mode** (`--tutorial`) — guided 8-step walkthrough using live data
- **SEE ALSO cross-links** — 31 fields with 105 cross-references to related metrics
- **Diagnostics Jump** — jump from any finding directly to the related source

---

## SDKs

| SDK | Language | Package |
|-----|----------|---------|
| [syslenz4j](https://github.com/opaopa6969/syslenz4j) | Java 17+ | Maven Central: `org.unlaxer.infra:syslenz4j` |
| syslenz4py | Python 3.8+ | `sdk/python/` (PyPI planned) |
| syslenz4node | Node.js 18+ | `sdk/node/` (npm planned) |

All SDKs connect to the TCP server (`--serve`) and provide typed metric access with `MetricKind` (8 variants) and `CommonMetric` (15 cross-platform metrics).

---

## Grafana Integration

```bash
docker compose --profile grafana up -d
# Grafana → http://localhost:3001
# Prometheus → http://localhost:9090
```

Pre-provisioned dashboards included. syslenz exports via Prometheus (`--prometheus`) or OpenTelemetry (`--otel`); the Grafana profile sets up scraping and provisioning automatically.

---

## Architecture

```
CLI args / config.toml
        |
        v
    +--------+       +----------+       +-----------+
    | main() | ----> | Snapshot  | ----> | TUI / Web |
    +--------+       | .capture()| <---- | render()  |
        |            +----------+       +-----------+
        |                 |
        v                 v
   +---------+     +------------+
   | Remote  |     | Parsers    |
   | (SSH /  |     | /proc (43) |
   |  Docker |     | /sys  (3)  |
   |  TCP)   |     | net   (5)  |
   +---------+     | plugins    |
                   +------------+
                         |
                         v
                 +---------------+
                 | Export        |
                 | JSON / OTEL   |
                 | Prometheus    |
                 +---------------+
```

See [`docs/en/architecture.md`](docs/en/architecture.md) for the full design document.

---

## Documentation

| Document | English | Japanese |
|----------|---------|----------|
| Getting Started | [`docs/en/getting-started.md`](docs/en/getting-started.md) | [`docs/ja/getting-started.md`](docs/ja/getting-started.md) |
| Architecture | [`docs/en/architecture.md`](docs/en/architecture.md) | [`docs/ja/architecture.md`](docs/ja/architecture.md) |
| Configuration | [`docs/en/config.md`](docs/en/config.md) | [`docs/ja/config.md`](docs/ja/config.md) |
| Dashboard | [`docs/en/dashboard.md`](docs/en/dashboard.md) | [`docs/ja/dashboard.md`](docs/ja/dashboard.md) |
| Diagnostics | [`docs/en/diagnostics.md`](docs/en/diagnostics.md) | [`docs/ja/diagnostics.md`](docs/ja/diagnostics.md) |
| Remote Monitoring | [`docs/en/remote.md`](docs/en/remote.md) | [`docs/ja/remote.md`](docs/ja/remote.md) |
| Plugins | [`docs/en/plugins.md`](docs/en/plugins.md) | [`docs/ja/plugins.md`](docs/ja/plugins.md) |
| OpenTelemetry / Prometheus | [`docs/opentelemetry.md`](docs/opentelemetry.md) | — |
| Audit Examples | [`docs/audit-examples.md`](docs/audit-examples.md) | — |
| Phase 3 Decisions | [`docs/decisions/phase3-api-settings-fleet.md`](docs/decisions/phase3-api-settings-fleet.md) | — |

---

## Verifying Downloads

```bash
sha256sum -c syslenz-linux-x86_64.tar.gz.sha256
sha256sum -c checksums.txt
shasum -a 256 -c syslenz-macos-aarch64.tar.gz.sha256   # macOS
```

---

## Contributing

```bash
git clone https://github.com/opaopa6969/syslenz.git
cd syslenz
cargo build
cargo test
cargo run
```

Run checks before submitting a PR:

```bash
cargo fmt --check && cargo clippy && cargo test
```

### Feature flags

| Feature | Description | Build command |
|---------|-------------|---------------|
| `otel` | OpenTelemetry OTLP export | `cargo build --features otel` |
| `web` | Web UI + HTTP API (axum) | `cargo build --features web` |
| `x11widget` | X11 floating widget | `cargo build --features x11widget` |

### Releasing

1. Update `version` in `Cargo.toml`
2. Add a CHANGELOG entry under `## [x.y.z] - YYYY-MM-DD`
3. Commit, tag, push — the release workflow builds binaries for Linux (x86_64, aarch64, musl), macOS, and Windows, generates SHA256 checksums, pushes to Docker Hub, and publishes to crates.io.

---

## Roadmap

| Feature | Status |
|---------|--------|
| TUI Dashboard, Classic, Diagnostics, Category Guide | Shipped (v1.0) |
| Alert system, time-travel diff, multi-host | Shipped (v1.1–v1.2) |
| Prometheus export, GPU metrics, cross-platform | Shipped (v1.3) |
| Education: tutorial, breadcrumbs, tips, SDK | Shipped (v1.4) |
| HTTP API v1, Settings GUI (`/settings`) | Shipped (v1.4) |
| Article Overlay (691 metrics × EN + JA) | Shipped (v1.5–v1.6) |
| Docker Hub multi-platform image | Shipped (v1.7) |
| Fleet View (`/fleet`) — multi-host web dashboard | **Planned** (not yet implemented) |
| Web authentication (Basic Auth / Token) | **Planned** (not yet implemented) |

---

## License

MIT

---

v1.7.0 | [Changelog](CHANGELOG.md) | [GitHub](https://github.com/opaopa6969/syslenz)

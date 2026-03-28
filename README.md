# syslenz

> Wireshark for Linux — explore /proc, /sys, and network as structured, typed data.

![demo](docs/assets/demo.gif)

**Zero config. One binary. Full JSON export.**

Explore every system metric as structured, typed data — memory, CPU, network sockets, kernel modules, disk space, temperature, routing tables, and 40+ more sources. With built-in auto-diagnostics and educational content that teaches you how computers work.

## Why syslenz?

| | |
|---|---|
| **Instant deep-dive** | SSH in, run `syslenz`, see everything. No agents, no config, no setup. |
| **Structured export** | Every field is typed (Bytes, Integer, Float, Duration) with full JSON export. Pipe to `jq`, diff between hosts, feed to CI. |
| **Learn Linux internals** | Every field has a human-readable description at 4 detail levels. Browse the system like a textbook. |
| **Auto-diagnostics** | Detects memory pressure, CPU overload, disk full, connection leaks, and 15+ patterns automatically. |
| **Plugin ecosystem** | Drop a script in `~/.config/syslenz/plugins/` — instant JVM, database, or custom metrics. |

## Install

```bash
# From source
cargo install --path .

# Docker
docker run --rm -p 9100:9100 --pid=host syslenz --serve
syslenz --connect localhost:9100

# Binary releases (GitHub Releases)
# See https://github.com/opaopa6969/syslenz/releases
```

## Data Sources (50+)

### /proc (43 sources — Linux)

| Category | Sources |
|----------|---------|
| System | uptime, loadavg, version, cmdline, modules, filesystems, devices, consoles, misc, dma |
| Memory | meminfo, vmstat, zoneinfo, buddyinfo, slabinfo, pagetypeinfo, swaps |
| CPU | cpuinfo, stat, interrupts, softirqs, schedstat, timer_list, pressure |
| Storage | mounts, partitions, diskstats, locks |
| Network | net/dev, net/tcp, net/udp, net/unix, net/arp, net/route, net/sockstat, net/snmp, net/netstat, net/wireless |
| Security | crypto, cgroups, iomem, ioports |
| Processes | processes (all PIDs with name, state, RSS, threads, UID) |

### /sys (3 sources)

| Source | Description |
|--------|-------------|
| df | Filesystem disk space usage (via statfs) |
| thermal | CPU/GPU temperature from thermal zones |
| file-nr | System-wide file descriptor usage |

### Network Deep-Dive (5 sources)

| Source | Description |
|--------|-------------|
| ip/route | Full routing table with metrics and default gateway |
| ip/neighbor | ARP/NDP cache with reachability state |
| ss | Socket statistics (TCP established, TIME_WAIT, orphaned) |
| dns | DNS configuration + resolution speed test |
| conntrack | Connection tracking table usage |

### Plugins (unlimited)

Drop any executable in `~/.config/syslenz/plugins/`. It outputs JSON to stdout, syslenz picks it up.

Example plugins included: **JVM** (jstat), **Docker** (container stats).

### Cross-Platform

| Platform | Sources | Method |
|----------|---------|--------|
| Linux | 51+ | /proc + /sys + commands |
| macOS | 14 | sysctl + vm_stat + system commands |
| Windows | 13 | PowerShell + WMI |

## Usage

```bash
syslenz                                    # TUI (Dashboard default)
syslenz --classic                          # TUI (Classic sidebar mode)
syslenz --lang ja                          # Japanese UI
syslenz --ssh user@host                    # Remote monitoring via SSH
syslenz --docker my-container              # Docker container monitoring
syslenz --serve 0.0.0.0:9100              # TCP server mode
syslenz --connect host:9100                # Connect to TCP server
syslenz --web 3000                         # Web UI (http://localhost:3000)
syslenz --web 3000 --lang ja               # Japanese Web UI
syslenz --export snapshot.json             # Export snapshot as JSON
syslenz --import snapshot.json             # Replay mode
syslenz --otel http://localhost:4317       # OpenTelemetry export
syslenz --widget                           # X11 floating widget
```

## TUI Views

| Key | View | Description |
|-----|------|-------------|
| `D` | Dashboard | System overview: load, memory, CPU, network |
| `O` | Classic | Sidebar + detail (traditional mode) |
| `W` | Welcome | Keybinding reference |
| `X` | Diagnostics | Auto-detected issues with suggestions |
| `C` | Category Guide | Educational content by topic |

## Keybindings

| Key | Action |
|-----|--------|
| `j`/`k` `Up`/`Down` | Navigate sources / fields / sections |
| `Enter` `Right` | Drill in (detail or table view) |
| `Backspace` `Left` | Go back |
| `Tab` | Toggle sidebar / content focus |
| `/` | Search sources |
| `d` | Diff view (changes since last refresh) |
| `g` | Graph (sparkline of selected numeric field) |
| `?` | Cycle help level (OFF → NORMAL → DETAILED → EXTRA) |
| `L` | Toggle language (EN/JA) |
| `c` | Copy to clipboard |
| `e` | Export current snapshot to JSON |
| `a` | Toggle auto-refresh |
| `r` | Manual refresh |
| `PgUp`/`PgDn` | Page scroll (in Category Guide) |
| `q` | Quit |

## Educational Features

### 4-Level Help (`?` key cycles)

| Level | Content |
|-------|---------|
| OFF | No help panel |
| NORMAL | One-line field summary |
| DETAILED | 2-3 sentence explanation |
| EXTRA | Full explanation + diagnostic tips + common issues |

All content available in English and Japanese.

### Category Guide (`C` key)

Cross-source educational narratives:
- **Memory**: "Where did all my RAM go?" — meminfo → vmstat → swaps → pressure chain
- **CPU/Load**: "Why is my server slow?" — loadavg → stat → pressure → schedstat
- **Network**: "The Life of a Packet" — net/dev → net/tcp → net/sockstat → routing

### Auto-Diagnostics (`X` key)

Automatically detects 15+ patterns:
- Memory: MemAvailable < 10% (CRIT), < 20% (WARN)
- CPU: load > 2x CPU count (CRIT), > 1x (WARN)
- Swap: exhausted (CRIT), no swap configured (INFO)
- PSI: CPU/Memory/IO pressure > 50% (CRIT), > 25% (WARN)
- Processes: zombies (WARN), D-state stuck (WARN)
- Network: SYN_SENT flood (WARN), CLOSE_WAIT leak (WARN), TIME_WAIT excess (INFO)
- Disk: usage > 90% (CRIT), > 80% (WARN)
- Temperature: > 90C (CRIT), > 75C (WARN)
- File descriptors: usage > 80% (WARN)
- DNS: no nameservers (WARN)
- Conntrack: usage > 80% (WARN)

## Web UI

```bash
./run-web.sh              # or: cargo run --features web -- --web 3000
```

- Same keyboard shortcuts as TUI
- Chart.js graphs: load trend, memory donut, CPU bars
- Real-time SSE updates
- Dark theme (Tokyo Night)

## Plugin System

```bash
# Create a plugin (any language)
cat > ~/.config/syslenz/plugins/hello << 'EOF'
#!/bin/bash
echo '{"source":"my-app","fields":[{"name":"status","value":{"Text":"running"},"unit":null,"description":"App status"}]}'
EOF
chmod +x ~/.config/syslenz/plugins/hello

# syslenz automatically picks it up as "plugin/hello"
```

See `plugins/examples/` for JVM and Docker plugin examples.

## Configuration

```toml
# ~/.config/syslenz/config.toml

[general]
lang = "en"              # "en" or "ja"
interval_ms = 1000       # Auto-refresh interval
default_view = "dashboard"  # "dashboard" or "classic"

[web]
port = 3000

[ssh]
hosts = ["user@server1", "user@server2"]

[[alert]]
source = "meminfo"
field = "MemAvailable"
condition = "< 500000000"
severity = "critical"
message = "Memory critically low"
```

## Docker

```bash
# Quick start
docker compose up -d
syslenz --connect localhost:9100

# With Web UI
docker compose --profile web up -d
# Open http://localhost:3000
```

## Documentation

Full documentation in English and Japanese:
- [English docs](docs/en/index.md)
- [Japanese docs (日本語)](docs/ja/index.md)

## License

MIT

---

v1.0.0 | [Changelog](CHANGELOG.md) | [GitHub](https://github.com/opaopa6969/syslenz)

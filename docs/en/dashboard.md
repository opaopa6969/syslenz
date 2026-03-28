---
version: v1.0.0
lang: en
---

# Dashboard View

[<- Prev: Getting Started](getting-started.md) | [Index](index.md) | [Next: Classic Mode ->](classic-mode.md)

[🇯🇵 日本語版](../ja/dashboard.md)

## Table of Contents

- [Overview](#overview)
- [Dashboard Sections](#dashboard-sections)
- [Section Navigation](#section-navigation)
- [Understanding Each Metric](#understanding-each-metric)
- [Drilling Into Details](#drilling-into-details)
- [Customization](#customization)

## Overview

The Dashboard is syslenz's default view. It provides a single-screen summary of your system's health, designed so you can assess the state of a machine in under 5 seconds. The dashboard auto-refreshes every second (configurable).

Press `D` from any view to return to the Dashboard.

## Dashboard Sections

The dashboard is divided into sections, each displaying a key subsystem:

| Section | Data Sources | What It Shows |
|---------|-------------|---------------|
| **Load** | `loadavg`, `stat` | 1/5/15 min load averages, CPU count, load-to-core ratio |
| **Memory** | `meminfo` | MemTotal, MemAvailable, MemFree, Cached, Buffers, usage percentage |
| **CPU** | `stat` | Per-state breakdown: user, system, idle, iowait, steal |
| **Swap** | `meminfo`, `swaps` | SwapTotal, SwapFree, SwapUsed, usage percentage |
| **Network** | `net/dev` | Per-interface RX/TX bytes, packets, errors, drops |
| **Disk** | `df`, `diskstats` | Root filesystem usage, disk I/O stats |
| **Processes** | `processes`, `stat` | Total count, running, sleeping, zombie, D-state |
| **Uptime** | `uptime` | System uptime in human-readable format |

## Section Navigation

In the Dashboard view:

| Key | Action |
|-----|--------|
| `j` / `k` or Arrow Down/Up | Select next / previous section |
| `Enter` | Drill into the selected section (switches to Classic mode) |
| `Tab` | No effect (no sidebar in Dashboard) |
| `D` | Return to Dashboard from any view |

The currently selected section is highlighted. Pressing `Enter` switches to Classic mode with the corresponding data source pre-selected.

## Understanding Each Metric

### Load Averages

Load average represents the average number of tasks in the run queue plus tasks in uninterruptible I/O wait. The three values represent 1-minute, 5-minute, and 15-minute averages.

**How to interpret:**
- Compare load to your CPU core count (shown in the dashboard)
- Load < cores: system has spare capacity
- Load = cores: fully utilized but not overloaded
- Load > cores: tasks are queuing, response times degrade
- Load > 2x cores: significant overload

### Memory

The dashboard shows memory in both absolute values and percentages.

**Key fields:**
- **MemTotal**: Total physical RAM installed
- **MemAvailable**: Memory available for applications without swapping (the most important number)
- **Cached**: RAM used as disk cache (reclaimable)
- **Buffers**: RAM used for filesystem metadata (reclaimable)

**How to interpret:**
- MemAvailable > 20% of MemTotal: healthy
- MemAvailable 10-20%: monitor closely
- MemAvailable < 10%: critical, OOM Killer risk

### CPU Utilization

Shown as a percentage breakdown of CPU time:

- **user**: Time running application code
- **system**: Time in kernel code
- **idle**: Nothing to do
- **iowait**: CPU idle, waiting for I/O to complete
- **steal**: Time stolen by the hypervisor (VMs only)

**How to interpret:**
- High user: application is CPU-bound
- High iowait: storage bottleneck (not a CPU problem)
- High steal: host is overcommitted, consider migrating the VM

### Network

Per-interface traffic statistics:

- **RX bytes / TX bytes**: Data received / transmitted
- **RX packets / TX packets**: Packets received / transmitted
- **RX errors / TX errors**: Receive / transmit errors (should be 0)
- **RX drops / TX drops**: Dropped packets (should be 0)

Non-zero errors or drops indicate hardware issues, driver bugs, or network congestion.

### Disk

Root filesystem usage and I/O statistics:

- **Usage %**: How full the root filesystem is
- **Reads/Writes**: I/O operations completed

Usage above 80% warrants attention; above 90% is critical.

## Drilling Into Details

When you select a section and press `Enter`, syslenz switches to Classic mode with the relevant data source selected. For example:

- Selecting **Memory** and pressing Enter opens Classic mode with `meminfo` selected
- Selecting **Network** opens `net/dev`
- Selecting **Load** opens `loadavg`

From Classic mode you can access the full detail view, diff view, and graph for any field. Press `Backspace` to return to the Dashboard.

## Customization

### Refresh Interval

The refresh interval can be configured in `~/.config/syslenz/config.toml`:

```toml
[general]
interval_ms = 1000  # milliseconds (default: 1000)
```

Or toggle auto-refresh at runtime with `a`, and manually refresh with `r`.

### Default View

To start in Classic mode instead of Dashboard:

```toml
[general]
default_view = "classic"
```

Or use the `--classic` CLI flag.

### Language

```toml
[general]
lang = "ja"  # "en" or "ja"
```

---

[<- Prev: Getting Started](getting-started.md) | [Index](index.md) | [Next: Classic Mode ->](classic-mode.md)

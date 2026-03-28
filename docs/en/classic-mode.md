---
version: v1.1.0
lang: en
---

# Classic Mode

[🇯🇵 日本語版](../ja/classic-mode.md)

[<- Prev: Dashboard](dashboard.md) | [Index](index.md) | [Next: Diagnostics ->](diagnostics.md)


## Table of Contents

- [Overview](#overview)
- [Sidebar Navigation](#sidebar-navigation)
- [Detail View](#detail-view)
  - [\[Enter to expand\] Indicator (v1.1.0)](#enter-to-expand-indicator-v110)
  - [Auto-Sparkline (v1.1.0)](#auto-sparkline-v110)
- [Table Drill-In](#table-drill-in)
- [Search](#search)
  - [Visible Search Bar (v1.1.0)](#visible-search-bar-v110)
- [Diff View](#diff-view)
  - [Time-Travel Diff (v1.1.0)](#time-travel-diff-v110)
- [Graph View](#graph-view)
- [Export and Copy](#export-and-copy)

## Overview

Classic mode is the "Wireshark-style" interface: a sidebar listing all data sources on the left, and a detail panel on the right showing the fields of the selected source. This is the most powerful view for deep system inspection.

Press `O` from any view to enter Classic mode.

## Sidebar Navigation

The sidebar lists all available data sources in alphabetical order (using a `BTreeMap` for stable sorting). Sources include everything from `/proc` parsers (e.g., `meminfo`, `cpuinfo`, `net/tcp`), `/sys` sources (e.g., `df`, `thermal`), network tools (e.g., `dns`, `conntrack`), and any loaded plugins (prefixed with `plugin/`).

| Key | Action |
|-----|--------|
| `j` / `k` or Arrow Down/Up | Navigate source list |
| `Enter` or Arrow Right | Select source and move focus to detail panel |
| `Tab` | Toggle focus between sidebar and content |
| `/` | Start search |
| `PageUp` / `PageDown` | Scroll by page |

When focus is on the sidebar, the currently highlighted source's fields are shown in the detail panel. The total number of sources is displayed at the bottom of the sidebar.

## Detail View

When you select a source and switch focus to the content panel, the detail view shows every field parsed from that source:

| Column | Description |
|--------|-------------|
| **Name** | Field name (e.g., `MemTotal`, `load1`, `rx_bytes`) |
| **Value** | Current value, formatted by type (bytes with units, floats with 2 decimals, durations in human-readable form) |
| **Unit** | Optional unit (e.g., `kB`, `seconds`) |
| **Description** | Human-readable explanation of the field |

| Key | Action |
|-----|--------|
| `j` / `k` or Arrow Down/Up | Scroll through fields |
| `Enter` | Drill into table fields (if the field is a Table type) |
| `Backspace` or Arrow Left | Return to sidebar |
| `d` | Switch to Diff view |
| `g` | Switch to Graph view for the selected field |
| `c` | Copy the selected field value to clipboard |
| `e` | Export current snapshot to JSON file |

### Field Types

syslenz parses every value into one of six typed variants:

| Type | Display Format | Example |
|------|---------------|---------|
| **Bytes** | Human-readable (KiB, MiB, GiB) | `15.7 GiB` |
| **Integer** | Plain number | `42` |
| **Float** | Two decimal places | `3.14` |
| **Text** | String | `Linux 6.1.0` |
| **Duration** | Days, hours, minutes, seconds | `3d 14h 22m` |
| **Table** | Row count indicator | `[256 rows]` |

### [Enter to expand] Indicator (v1.1.0)

As of v1.1.0, fields with a Table type display an `[Enter to expand]` indicator next to the row count. This makes it immediately obvious to new users that they can drill into table data, improving discoverability without requiring knowledge of keybindings.

### Auto-Sparkline (v1.1.0)

As of v1.1.0, numeric fields (Bytes, Integer, Float) in the Detail view automatically display a small sparkline graph (`▁▂▃▅▇`) below the current value. The sparkline shows the recent history of that field from the ring buffer, giving you an instant visual trend without needing to open the full Graph view. This works for any numeric field and updates in real time when auto-refresh is enabled.

## Table Drill-In

When a field has a Table type (e.g., `net/tcp` connections, `processes` list, `mounts` entries), press `Enter` to open the table view. This shows the data in a scrollable table with columns.

| Key | Action |
|-----|--------|
| `j` / `k` or Arrow Down/Up | Scroll through rows |
| `PageUp` / `PageDown` | Scroll by page |
| `Backspace` | Return to detail view |

Tables can be very large (thousands of TCP connections, hundreds of processes). Use `PageUp`/`PageDown` for fast scrolling.

## Search

Press `/` to enter search mode. A search input appears at the bottom of the screen.

| Key | Action |
|-----|--------|
| Type characters | Filter source list in real-time |
| `Enter` | Apply search and return to navigation |
| `Esc` | Cancel search |
| `Backspace` | Delete last character |

Search filters the sidebar source list. For example, typing `net` shows only sources containing "net" (e.g., `net/dev`, `net/tcp`, `net/udp`, `net/route`, `net/snmp`, `net/netstat`, `net/sockstat`, `net/wireless`, `net/arp`, `net/unix`).

After applying a search, the sidebar shows only matching sources. Press `Esc` or clear the search to restore the full list.

### Visible Search Bar (v1.1.0)

As of v1.1.0, pressing `/` displays a visible search input with a blinking cursor in the status bar at the bottom of the screen. This makes it immediately clear that search mode is active and shows your typed query in real time as you filter the source list.

## Diff View

Press `d` to enter Diff view. This compares the current snapshot with the previous one and highlights changes:

- **Added** fields (new values that did not exist before)
- **Removed** fields (values that disappeared)
- **Changed** fields with old and new values displayed side by side

Diff view is essential for understanding system dynamics: which counters are incrementing, which memory values are shifting, which network connections appeared or disappeared.

| Key | Action |
|-----|--------|
| `j` / `k` | Scroll through diff entries |
| `PageUp` / `PageDown` | Scroll by page |
| `[` | Compare with an older snapshot (time-travel back) |
| `]` | Compare with a newer snapshot (time-travel forward) |
| `Backspace` | Return to previous view |

### Time-Travel Diff (v1.1.0)

As of v1.1.0, Diff view supports **time-travel**: press `[` to step back to an older snapshot pair or `]` to step forward to a newer one. A **T-N indicator** is displayed in the status bar (e.g., `T-3` means you are comparing the snapshot from 3 refreshes ago with the one before it). This lets you review historical changes without leaving the TUI, which is invaluable for post-incident analysis when you want to pinpoint exactly when a value changed.

When you reach the oldest available snapshot pair, `[` has no further effect; likewise `]` stops at the most recent pair.

**Tip:** Enable auto-refresh (`a`) and switch to diff view to watch your system change in real time. This is particularly useful for monitoring `vmstat` counters, `net/dev` traffic, and `diskstats` I/O.

## Graph View

Press `g` while a numeric field is selected to open the Graph view. This shows a sparkline time-series chart of that field's values over the last 60 snapshots (configurable via `history_size` in config).

The graph updates in real time when auto-refresh is enabled. It works with any numeric field type: Bytes, Integer, Float, or Duration.

| Key | Action |
|-----|--------|
| `Backspace` | Return to previous view |

**Use cases:**
- Memory trend: select `MemAvailable` in `meminfo` and press `g` to watch memory availability over time
- CPU utilization: select `cpu_user` in `stat` and graph it
- Network throughput: select `rx_bytes` on an interface in `net/dev`
- Load tracking: select `load1` in `loadavg`

## Export and Copy

### Export to JSON

Press `e` to export the current snapshot to a JSON file. The file is saved in the current directory with a timestamp-based name:

```
syslenz_snapshot_1711612800.json
```

### Copy to Clipboard

Press `c` to copy the currently selected field's value to the system clipboard. syslenz tries the following clipboard commands in order:

1. `xclip -selection clipboard` (X11)
2. `xsel --clipboard --input` (X11)
3. `wl-copy` (Wayland)
4. `pbcopy` (macOS)

A status message confirms the copy, showing a truncated preview of the copied text.

---

[<- Prev: Dashboard](dashboard.md) | [Index](index.md) | [Next: Diagnostics ->](diagnostics.md)

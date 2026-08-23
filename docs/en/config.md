---
version: v1.1.0
lang: en
---

# Configuration Reference

[🇯🇵 日本語版](../ja/config.md)

[<- Prev: Plugins](plugins.md) | [Index](index.md) | [Next: Keybindings ->](keybindings.md)


## Table of Contents

- [File Location](#file-location)
- [Priority Order](#priority-order)
- [Configuration Sections](#configuration-sections)
  - [general](#general)
  - [otel](#otel)
  - [web](#web)
  - [ssh](#ssh)
  - [alert](#alert-v110)
- [Complete Example](#complete-example)
- [Minimal Example](#minimal-example)

## File Location

syslenz reads its configuration from:

```
~/.config/syslenz/config.toml
```

Or, if `$XDG_CONFIG_HOME` is set:

```
$XDG_CONFIG_HOME/syslenz/config.toml
```

If the file does not exist, syslenz silently uses defaults. If the file exists but contains errors, a warning is printed to stderr and defaults are used.

## Priority Order

Settings are resolved in this order (highest priority first):

1. **CLI flags** (e.g., `--lang ja` overrides config)
2. **Environment variables** (e.g., `$XDG_CONFIG_HOME`)
3. **config.toml**
4. **Built-in defaults**

## Configuration Sections

### `[general]`

Core settings that affect the TUI behavior.

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `lang` | String | `"en"` | Interface language. Valid values: `"en"`, `"ja"` |
| `interval_ms` | Integer | `1000` | Auto-refresh interval in milliseconds |
| `default_view` | String | `"dashboard"` | Starting view. Valid values: `"dashboard"`, `"classic"` |
| `history_size` | Integer | `60` | Number of snapshots kept in the ring buffer for graphs and diffs |

```toml
[general]
lang = "en"
interval_ms = 1000
default_view = "dashboard"
history_size = 60
```

**Notes:**
- `interval_ms` affects both auto-refresh in the TUI and the polling interval for remote connections (SSH, Docker, TCP)
- `history_size` controls how many data points appear in graph view. Larger values use more memory but show longer trends
- `default_view = "classic"` is equivalent to the `--classic` CLI flag

### `[otel]`

OpenTelemetry export settings (used by `--otel` mode).

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `endpoint` | String | `"http://localhost:4317"` | OTLP gRPC endpoint URL |
| `interval_secs` | Integer | `5` | Metric push interval in seconds |

```toml
[otel]
endpoint = "http://localhost:4317"
interval_secs = 5
```

**Notes:**
- The endpoint must be an OTLP-compatible gRPC endpoint
- CLI flag `--otel [endpoint]` overrides `otel.endpoint`
- CLI flag `--interval <secs>` overrides `otel.interval_secs`

### `[web]`

Web UI settings (used by `--web` mode).

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `port` | Integer | `3000` | HTTP port for the web server |
| `capture_interval_secs` | Integer | `1` | Snapshot capture interval in seconds |
| `max_history_count` | Integer | `60` | Maximum number of in-memory history snapshots |
| `max_history_bytes` | Integer | `67108864` | Maximum total bytes of in-memory history (approx, 0 disables) |
| `truncate_large_tables` | Boolean | `true` | Truncate large tables in history to save memory |
| `truncate_table_rows` | Integer | `20` | Row threshold for table truncation in history |

```toml
[web]
port = 3000
capture_interval_secs = 1
max_history_count = 60
max_history_bytes = 67108864  # 64 MB
truncate_large_tables = true
truncate_table_rows = 20
```

**Memory usage:**

`--web` mode runs a background task that periodically captures the entire
`/proc` tree and keeps history in memory. The following guards prevent RSS
from growing unbounded over long uptimes:

- **Count limit** (`max_history_count`): caps the number of snapshots kept
- **Byte limit** (`max_history_bytes`): drops oldest snapshots when total
  size exceeds the limit
- **Table truncation** (`truncate_large_tables`): replaces large tables
  (e.g. process lists) in historical snapshots with the first
  `truncate_table_rows` rows plus a `[truncated: N rows]` marker.
  The latest snapshot (`/api/snapshot`) is always full-size
- **malloc_trim**: calls `malloc_trim(0)` every 60 seconds to return
  fragmented glibc malloc memory to the kernel
- **MALLOC_ARENA_MAX**: sets `mallopt(M_ARENA_MAX, 2)` at startup to
  limit glibc malloc arena count

Check current memory settings and history count via `/healthz`:

```bash
curl http://localhost:3000/healthz
# {"status":"ok","history_len":42,"max_history_count":60,...}
```

**Notes:**
- The web server always binds to `0.0.0.0` (all interfaces)
- CLI flag `--web [port]` overrides `web.port`

### `[ssh]`

SSH-related settings.

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `hosts` | Array of Strings | `[]` | Pre-configured remote hosts (for future multi-host view) |

```toml
[ssh]
hosts = [
    "admin@web-server-1",
    "admin@web-server-2",
    "root@db-server",
]
```

**Notes:**
- This field is reserved for future multi-host monitoring support. Currently, use `--ssh` CLI flag for single-host monitoring.

### `[[alert]]` (v1.1.0)

As of v1.1.0, you can define custom alert rules using TOML array-of-tables syntax. Each `[[alert]]` entry specifies a data source, a field name, a comparison operator, and a threshold value. When the condition is met, syslenz highlights the alert in multiple places:

- **Status bar** -- an alert summary (e.g., `ALERT: 2 active`) appears in the status bar
- **Sidebar coloring** -- sources with active alerts are colored to stand out
- **Field markers** -- individual fields that triggered an alert are marked in the detail view

| Key | Type | Required | Description |
|-----|------|----------|-------------|
| `source` | String | Yes | Data source name (e.g., `"meminfo"`, `"loadavg"`) |
| `field` | String | Yes | Field name within the source (e.g., `"MemAvailable"`, `"load1"`) |
| `condition` | String | Yes | Comparison expression, e.g. `"< 500000000"` or `">= 8.0"` |
| `severity` | String | Yes | Alert severity: `"info"`, `"warning"`, or `"critical"` |
| `message` | String | Yes | Human-readable alert message |
| `action` | String | No | External command for newly firing alerts; supports `{message}`, `{source}`, `{field}`, `{value}`, and `{severity}` |
| `notify` | Array of Strings | No | Notification endpoints such as `slack:URL` or `webhook:URL` |

```toml
# Alert when available memory drops below 500 MB (in kB)
[[alert]]
source = "meminfo"
field = "MemAvailable"
condition = "< 512000"
severity = "critical"
message = "Low memory"

# Alert when 1-minute load exceeds 8
[[alert]]
source = "loadavg"
field = "load1"
condition = "> 8.0"
severity = "warning"
message = "High load"

# Alert when root disk usage exceeds 90%
[[alert]]
source = "df"
field = "use_percent"
condition = "> 90.0"
severity = "warning"
message = "Disk almost full"
```

**Notes:**
- Alert rules are evaluated on every snapshot refresh
- Multiple alerts can fire simultaneously; all active alerts are shown in the status bar
- Alerts integrate with the existing diagnostics engine but are user-configurable, allowing you to set thresholds appropriate for your environment
- Field values are compared as floating-point numbers; text fields cannot be used with alerts

## Complete Example

```toml
# syslenz configuration

# Location: ~/.config/syslenz/config.toml


[general]
# Interface language: "en" or "ja"

lang = "en"

# Auto-refresh interval (milliseconds)

interval_ms = 1000

# Starting view: "dashboard" or "classic"

default_view = "dashboard"

# Number of snapshots to keep for graphs

history_size = 60

[otel]
# OTLP gRPC endpoint

endpoint = "http://otel-collector.local:4317"

# Metric push interval (seconds)

interval_secs = 10

[web]
# Web UI port

port = 8080

[ssh]
# Pre-configured remote hosts

hosts = [
    "admin@prod-web-01",
    "admin@prod-web-02",
    "root@prod-db-01",
]
```

## Minimal Example

For most users, no config file is needed. If you only want to change the language:

```toml
[general]
lang = "ja"
```

---

[<- Prev: Plugins](plugins.md) | [Index](index.md) | [Next: Keybindings ->](keybindings.md)

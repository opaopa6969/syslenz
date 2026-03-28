# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/).

## [1.2.0] - 2026-03-29

### Added
- Multi-host monitoring: multiple --ssh/--docker/--connect flags, F1-F9 tab switching
- Education: Storage, Process, Hardware categories with full EN/JA content
- Learning paths: Beginner, Performance Diagnosis, Server Health stubs
- JVM plugin (plugins/jvm/syslenz-jvm): comprehensive JVM monitoring via jstat/jcmd
- syslenz4j: separate Java library (github.com/opaopa6969/syslenz4j)
- i18n: 584 entries, 86% field coverage (521/600)
- Document restructure: DGE-toolkit pattern (docs/features, design-materials)

## [1.1.0] - 2026-03-29

### Added
- Time-travel diff: `[` and `]` keys to compare with any past snapshot (T-N indicator)
- Alert system: configurable `[[alert]]` rules in config.toml with TUI display (status bar, sidebar, field markers)
- ViewData unified UI layer: TUI Dashboard/Detail/Diagnostics now render from shared ViewData structs
- Web UI ViewData integration: fetches `/api/view` for consistent display with TUI
- Auto-graph in Web UI: numeric fields show Chart.js chart automatically on selection
- TUI Dashboard bar graphs: RAM/Swap/CPU with ████░░░░ visualization
- TUI Dashboard sparkline graphs: load + memory history (▁▂▃▄▅▆▇█)
- Auto-sparkline in Detail view: numeric fields show graph below field table automatically
- Visible search bar: `/` shows search input with cursor in status bar
- AA line charts in TUI using block characters
- PgUp/PgDn in Web UI for all views
- `[Enter to expand]` indicator for table fields (TUI + Web)
- i18n expansion: 390/600 fields covered in EN/JA 3-level descriptions
- All parser fields now have non-empty English descriptions (600/600)
- vmstat: 165 fields with full descriptions
- meminfo, net_snmp, net_netstat, pressure: comprehensive descriptions added
- 48 Playwright Web UI automated tests with video recording
- alert.rs with condition parser and debounce support

### Changed
- Dashboard is now default view (Classic mode via `O` key)
- Graph view shows as split panel below detail (not full screen replacement)
- Search from Dashboard auto-switches to Classic mode
- Tab from fullwidth views switches to Classic mode

### Fixed
- Load average display (load_1min field name mismatch)
- Dashboard network section excessive whitespace
- Web UI missing keybindings: C (Category Guide), W (Welcome), e (Export)
- Help EXTRA panel height now dynamic based on content

## [1.0.0] - 2026-03-28

### Added
- TUI dashboard with Overview, Detail, Diff, Table, and Graph views
- Welcome screen with guided onboarding
- Classic mode for traditional terminal display
- 43 /proc source parsers (meminfo, uptime, loadavg, cpuinfo, stat, mounts, partitions, diskstats, processes, swaps, buddyinfo, cgroups, cmdline, consoles, crypto, devices, filesystems, interrupts, iomem, ioports, locks, modules, vmstat, zoneinfo, softirqs, misc, pressure, net/dev, net/tcp, net/udp, net/unix, net/arp, net/route, net/sockstat, net/snmp, net/netstat, net/wireless, slabinfo, pagetypeinfo, schedstat, dma, timer_list, version)
- 3 /sys sources (df, thermal, file-nr)
- Network deep-dive sources (ip/route, ip/neighbor, ss, dns, conntrack)
- SSH, Docker, and TCP remote monitoring
- Web UI with Chart.js visualization (feature: web)
- X11 floating widget (feature: x11widget)
- OpenTelemetry metric export (feature: otel)
- JSON export/import for snapshots and time series
- Snapshot diff engine with type-aware comparison thresholds
- Diagnostics auto-analysis engine
- Category Guide education system
- Plugin system for custom metric sources
- i18n support for English and Japanese with 4-level contextual help
- Copy to clipboard support
- Config file support (TOML)
- Cross-platform support for macOS and Windows
- Unit tests for format_bytes, format_duration, diff_snapshots, systemtime_iso8601, export/import round-trip, and i18n completeness
- Integration smoke tests for all 43+ parsers (Linux)
- GitHub Actions CI pipeline with fmt, clippy, build, and test

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/).

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

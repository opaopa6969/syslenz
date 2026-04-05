# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]

## [1.7.0] - 2026-04-06 "Docker Hub + Web Parity"

### Added
- **Docker Hub** — `docker run --rm -p 3000:3000 --pid=host opaopa6969/syslenz` (no build needed)
  - Multi-platform: `linux/amd64` + `linux/arm64`, default CMD `--web 3000`
  - Tags: `latest`, `vX.Y.Z`, `X.Y` floating minor
- **release.yml: docker job** — auto-push to Docker Hub on every version tag
- **Web UI: sidebar tree/flat toggle** — `t` key groups sources by path prefix
- **Web UI: TableView RT stability** — view persists across auto-refresh cycles
- **Web UI: j/k navigation in TableView** — keyboard row navigation with highlight
- **Web UI: ProcessDetail view** — Enter on processes row → full `/proc/[pid]` detail
- **Web UI: graph y-axis fixed** — all-time min/max prevents pikon rescaling
- **Web UI: time window selection** — `[`/`]` keys + buttons (30s/1m/2m/5m/15m/1h)
- **JA process detail help** — all ~60 status fields, 7 io fields, inline descriptions in Japanese

### Fixed
- TUI graph pikon: min/max now computed from ALL snapshots, not just visible window
- TUI TableView RT cancel: `table_view_source` pins source by name, bypasses index drift
- Dockerfile: multi-platform musl build, default CMD `--web 3000`

## [1.6.0] - 2026-04-02 "Full Article Coverage"

### Added
- Article overlay coverage for all 691 metrics (EN + JA) — every field now has a dedicated article
- Group articles with full quality content (volta style: real episodes, ASCII diagrams, tuning steps):
  - vmstat families: thp, compact, allocstall, numa, workingset, kswapd, pgscan/pgsteal, pgalloc, swap, balloon, nr_zone, nr_active_inactive, unevictable_pgs
  - net/netstat: TcpExt (130+ fields), IpExt
  - net/snmp: Ip, Icmp, Tcp, Udp, UdpLite
  - meminfo: HugePages, Active_Inactive, Slab
- Sourceguide articles for all 45 data sources
- 29 concept articles: bottleneck-triage, memory-pressure, latency-analysis, pressure-stall, resource-model, and more
- 717 EN + 717 JA markdown article files total

### Fixed
- Duplicate `jump_to_metric` in `app.rs` (merged features, removed duplicate)
- Duplicate `Router` import in `web.rs`
- Missing `use std::fs` in `main.rs`

## [1.5.0] - 2026-04-02 "Fleet, Articles, Live Controls"

### Added
- All features from v1.4.0 and v1.3.1 merged into a single release.

## [1.4.0] - 2026-03-29 "Teach, Connect, Scale"

### Added
- **Diagnostics jump** -- navigate directly from a diagnostic finding to the related metric source (view_history stack + picker UI)
- **"Did you know?" Tips** -- random educational tips displayed dynamically on the Welcome view
- **Learning breadcrumbs** -- at EXTRA help level, 18 fields (EN/JA) show "next step" hints guiding deeper exploration
- **Interactive tutorial mode** -- `--tutorial` flag launches an 8-step guided walkthrough using live system data
- **SEE ALSO cross-links** -- 31 fields with 105 cross-references to related metrics
- **Contextual hints** -- 10 fields display context-aware guidance
- **syslenz4py** (Python SDK) -- connect to syslenz from Python (`sdk/python/`)
- **syslenz4node** (Node.js SDK) -- connect to syslenz from Node.js (`sdk/node/`)
- **OTEL bridge improvements** -- resource attributes, i18n descriptions, counter detection
- **Provider contribution guide** -- step-by-step guide for creating new providers (JA/EN, with template)
- **Grafana dashboard provisioning** -- `docker compose --profile grafana` sets up Prometheus + Grafana with pre-built dashboards
- **MetricKind enum** -- 8 variants for typed metric classification
- **CommonMetric enum** -- 15 cross-platform metrics for SDK interoperability
- **Diagnostics expanded to 27 check functions** (40+ patterns) -- added diagnostic deep-dive, related_metrics linking
- **DGE 017** -- education-first design philosophy
- **DGE 018** -- education feature expansion (diagnostics jump, tips, breadcrumbs, tutorial)
- **DGE 019** -- SDK + Grafana + MetricKind design session

## [1.3.1] - 2026-04-02 "Article Overlay + Live Controls"

### Added
- Article-annotated resolver + markdown-per-article resources (680 ids) with CLI export/import helpers and Web/TUI overlays.
- Article overlay navigation (SEE ALSO links, overlay modal + drawer) for both TUI `A` key and Web `A` button, with `sourceguide.*` fallback coverage.
- Web dashboard controls: `AXIS` badge/key toggles Chart.js axes, `RT` badge/key pauses/resumes SSE streams, and Web docs cover the new flow.
- TUI dashboard history axis mode (`S` key) plus `c`/`A`/`?` key guides updated accordingly.

### Changed
- Version bump to 1.3.1 to capture the new education overlay + dashboard UX updates.

## [1.3.0] - 2026-03-29 "See More, Learn More"

### Added
- GPU metrics (nvidia-smi): temperature, utilization, memory, power, fan speed
- systemd service status: running/failed counts, system state, failed service list
- Prometheus export: `--prometheus` flag, HTTP /metrics endpoint, Prometheus text format
- serve.rs METRICS command for Prometheus scraping via TCP
- Diagnostics expanded to 25+ patterns: memory leak, swap activity, OOM kills, network errors, recent reboot, load trend, high-memory process, orphaned TCP, IP forwarding, kernel taint
- Provider scaffold template with helper functions
- MySQL provider (syslenz-provider-mysql)
- PostgreSQL provider (syslenz-provider-postgres)
- Redis provider (syslenz-provider-redis)
- nginx provider (syslenz-provider-nginx)
- DGE 015: competitive gap analysis (vs htop/netdata/Datadog/Prometheus)
- DGE 016: product lineup strategy (SDK/Provider roadmap)
- macOS: +10 parsers (24 total) — network connections, launchd, diskutil, system_profiler, dns, software_update, power, kexts
- Windows: +11 parsers (24 total) — TCP/UDP connections, perf counters, handles, hotfix, scheduled tasks, volumes, dns cache, firewall
- DGE 014: cross-platform metric enum design
- 5 Prometheus format unit tests

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

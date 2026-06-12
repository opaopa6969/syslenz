# Changelog

All notable changes to syslenz are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Versions are published to [crates.io](https://crates.io/crates/syslenz) and [Docker Hub](https://hub.docker.com/r/opaopa6969/syslenz).

---

## [Unreleased]

### Fixed
- **TUI corruption from untrusted strings** — process names/cmdlines from /proc, plugin output, and remote snapshots can carry ANSI escape sequences or control bytes that ratatui writes to the terminal verbatim, garbling the display. All snapshots are now sanitized at the ingestion boundaries (local capture, ssh/docker/tcp parse): ANSI sequences are stripped, remaining control characters become spaces.

### Added
- **Agent-evaluated alerts** — snapshots may carry a top-level `alerts` array (emitted e.g. by the syslenz4j Watch API). They are merged into the active alert display (status bar counts, sidebar severity badges), attributed to the entry holding the matching metric. Display-only: actions/notifications/history remain the agent's responsibility. Older snapshots without the key parse unchanged.

### Planned
- **Fleet View** (`/fleet`) — multi-host web dashboard with status matrix and auto-refresh
- **Web authentication** — Basic Auth and Token Auth via `[web]` config section

---

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

---

## [1.6.0] - 2026-04-02 "Full Article Coverage"

### Added
- **Article overlay coverage** for all 691 metrics (EN + JA) — every field now has a dedicated article
- Group articles with full quality content (real episodes, ASCII diagrams, tuning steps):
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

---

## [1.5.0] - 2026-04-02 "Fleet, Articles, Live Controls"

### Added
- All features from v1.4.0 and v1.3.1 merged into a single release

---

## [1.4.0] - 2026-03-29 "Teach, Connect, Scale"

### Added
- **Diagnostics jump** — navigate directly from a diagnostic finding to the related metric source (view_history stack + picker UI)
- **"Did you know?" Tips** — random educational tips displayed dynamically on the Welcome view
- **Learning breadcrumbs** — at EXTRA help level, 18 fields (EN/JA) show "next step" hints guiding deeper exploration
- **Interactive tutorial mode** — `--tutorial` flag launches an 8-step guided walkthrough using live system data
- **SEE ALSO cross-links** — 31 fields with 105 cross-references to related metrics
- **Contextual hints** — 10 fields display context-aware guidance
- **HTTP API v1** — versioned prefix `/api/v1/*` with `X-Syslenz-API-Version: 1` response header
- **Settings GUI** (`/settings`) — browser-based alert rule editor
- **`GET /api/v1/settings`** — returns current config as JSON
- **`POST /api/v1/settings/alerts`** — writes alert rules to config file without restart
- **syslenz4py** (Python SDK) — connect to syslenz from Python (`sdk/python/`)
- **syslenz4node** (Node.js SDK) — connect to syslenz from Node.js (`sdk/node/`)
- **OTEL bridge improvements** — resource attributes, i18n descriptions, counter detection
- **Provider contribution guide** — step-by-step guide for creating new providers (JA/EN, with template)
- **Grafana dashboard provisioning** — `docker compose --profile grafana` sets up Prometheus + Grafana with pre-built dashboards
- **MetricKind enum** — 8 variants for typed metric classification
- **CommonMetric enum** — 15 cross-platform metrics for SDK interoperability
- **Diagnostics expanded to 27 check functions** (40+ patterns) — diagnostic deep-dive, related_metrics linking
- **DGE 017** — education-first design philosophy
- **DGE 018** — education feature expansion (diagnostics jump, tips, breadcrumbs, tutorial)
- **DGE 019** — SDK + Grafana + MetricKind design session
- **DGE 020** — Phase 3 spec: Fleet View, API v1, Settings GUI

### Note
Fleet View (`/fleet`) and authentication (Basic Auth / Token) were designed in Phase 3 (DGE 020) but are **not yet implemented** in this release. See [`docs/decisions/phase3-api-settings-fleet.md`](docs/decisions/phase3-api-settings-fleet.md).

---

## [1.3.1] - 2026-04-02 "Article Overlay + Live Controls"

### Added
- Article-annotated resolver + markdown-per-article resources (680 ids) with CLI export/import helpers and Web/TUI overlays
- Article overlay navigation (SEE ALSO links, overlay modal + drawer) for TUI `A` key and Web `A` button
- Web dashboard controls: `AXIS` badge/key toggles Chart.js axes, `RT` badge/key pauses/resumes SSE streams
- TUI dashboard history axis mode (`S` key)

### Changed
- Version bump to 1.3.1 to capture the new education overlay + dashboard UX updates

---

## [1.3.0] - 2026-03-29 "See More, Learn More"

### Added
- **GPU metrics** (nvidia-smi): temperature, utilization, memory, power, fan speed
- **systemd service status**: running/failed counts, system state, failed service list
- **Prometheus export**: `--prometheus` flag, HTTP `/metrics` endpoint, Prometheus text format
- **`serve.rs` METRICS command** for Prometheus scraping via TCP
- **Diagnostics expanded** to 25+ patterns: memory leak, swap activity, OOM kills, network errors, recent reboot, load trend, high-memory process, orphaned TCP, IP forwarding, kernel taint
- **Provider scaffold template** with helper functions
- **MySQL provider** (syslenz-provider-mysql)
- **PostgreSQL provider** (syslenz-provider-postgres)
- **Redis provider** (syslenz-provider-redis)
- **nginx provider** (syslenz-provider-nginx)
- **DGE 015** — competitive gap analysis (vs htop/netdata/Datadog/Prometheus)
- **DGE 016** — product lineup strategy (SDK/Provider roadmap)
- **macOS**: +10 parsers (24 total) — network connections, launchd, diskutil, system_profiler, dns, software_update, power, kexts
- **Windows**: +11 parsers (24 total) — TCP/UDP connections, perf counters, handles, hotfix, scheduled tasks, volumes, dns cache, firewall
- **DGE 014** — cross-platform metric enum design
- 5 Prometheus format unit tests

---

## [1.2.0] - 2026-03-29 "Many Hosts"

### Added
- **Multi-host monitoring** — multiple `--ssh`/`--docker`/`--connect` flags, F1–F9 tab switching
- **Education**: Storage, Process, Hardware categories with full EN/JA content
- **Learning paths**: Beginner, Performance Diagnosis, Server Health stubs
- **JVM plugin** (`plugins/jvm/syslenz-jvm`) — comprehensive JVM monitoring via jstat/jcmd
- **syslenz4j** — separate Java library (github.com/opaopa6969/syslenz4j)
- **i18n**: 584 entries, 86% field coverage (521/600)
- **Document restructure**: DGE-toolkit pattern (`docs/features`, `design-materials`)

---

## [1.1.0] - 2026-03-29 "Time Travel and Alerts"

### Added
- **Time-travel diff** — `[` and `]` keys to compare with any past snapshot (T-N indicator in status bar)
- **Alert system** — configurable `[[alert]]` rules in `config.toml` with TUI display: status bar counter, sidebar source coloring, field background color
- **ViewData unified UI layer** — TUI Dashboard/Detail/Diagnostics render from shared `ViewData` structs
- **Web UI ViewData integration** — fetches `/api/view` for consistent display with TUI
- **Auto-graph in Web UI** — numeric fields show Chart.js chart automatically on selection
- **TUI Dashboard bar graphs** — RAM/Swap/CPU with `████░░░░` visualization
- **TUI Dashboard sparkline graphs** — load + memory history (`▁▂▃▄▅▆▇█`)
- **Auto-sparkline in Detail view** — numeric fields show graph below field table automatically
- **Visible search bar** — `/` shows search input with cursor in status bar
- **AA line charts** in TUI using block characters
- **PgUp/PgDn** in Web UI for all views
- **`[Enter to expand]` indicator** for table fields (TUI + Web)
- **i18n expansion**: 390/600 fields covered in EN/JA 3-level descriptions
- All parser fields now have non-empty English descriptions (600/600)
- `vmstat`: 165 fields with full descriptions
- `meminfo`, `net_snmp`, `net_netstat`, `pressure`: comprehensive descriptions added
- 48 Playwright Web UI automated tests with video recording
- `alert.rs` with condition parser and debounce support

### Changed
- Dashboard is now the default view (Classic mode via `O` key)
- Graph view shows as split panel below detail (not full-screen replacement)
- Search from Dashboard auto-switches to Classic mode
- Tab from fullwidth views switches to Classic mode

### Fixed
- Load average display (load_1min field name mismatch)
- Dashboard network section excessive whitespace
- Web UI missing keybindings: `C` (Category Guide), `W` (Welcome), `e` (Export)
- Help EXTRA panel height now dynamic based on content

---

## [1.0.0] - 2026-03-28 "Phase 1 + 2 Complete"

### Added
- **TUI** — Dashboard, Classic (Overview), Diff, Table, Graph, Welcome, Diagnostics, Category Guide views
- **ratatui** 0.29 for terminal rendering; **crossterm** 0.28 for input
- **43 /proc parsers** (Linux): meminfo, uptime, loadavg, cpuinfo, stat, mounts, partitions, diskstats, processes, swaps, buddyinfo, cgroups, cmdline, consoles, crypto, devices, filesystems, interrupts, iomem, ioports, locks, modules, vmstat, zoneinfo, softirqs, misc, pressure, net/dev, net/tcp, net/udp, net/unix, net/arp, net/route, net/sockstat, net/snmp, net/netstat, net/wireless, slabinfo, pagetypeinfo, schedstat, dma, timer_list, version
- **3 /sys sources**: df, thermal, file-nr
- **Network deep-dive**: ip/route, ip/neighbor, ss, dns, conntrack
- **SSH, Docker, TCP** remote monitoring (`--ssh`, `--docker`, `--serve`/`--connect`)
- **Web UI** with Chart.js visualization (`web` feature: axum + tokio)
- **X11 floating widget** (`x11widget` feature)
- **OpenTelemetry** metric export (`otel` feature)
- **JSON export/import** for snapshots and time series
- **Snapshot diff engine** with type-aware comparison thresholds
- **Diagnostics** auto-analysis engine (initial implementation)
- **Category Guide** education system (Memory, CPU, Network)
- **Plugin system** — drop executables in `~/.config/syslenz/plugins/`
- **i18n** — English and Japanese with 4-level contextual help
- **Config file** support (`~/.config/syslenz/config.toml`, TOML)
- Cross-platform support: macOS (14 parsers), Windows (13 parsers)
- Copy to clipboard support
- Unit tests: format_bytes, format_duration, diff_snapshots, systemtime_iso8601, export/import round-trip, i18n completeness
- Integration smoke tests for all 43+ parsers (Linux)
- GitHub Actions CI: fmt, clippy, build, test

[Unreleased]: https://github.com/opaopa6969/syslenz/compare/v1.7.0...HEAD
[1.7.0]: https://github.com/opaopa6969/syslenz/compare/v1.6.0...v1.7.0
[1.6.0]: https://github.com/opaopa6969/syslenz/compare/v1.5.0...v1.6.0
[1.5.0]: https://github.com/opaopa6969/syslenz/compare/v1.4.0...v1.5.0
[1.4.0]: https://github.com/opaopa6969/syslenz/compare/v1.3.1...v1.4.0
[1.3.1]: https://github.com/opaopa6969/syslenz/compare/v1.3.0...v1.3.1
[1.3.0]: https://github.com/opaopa6969/syslenz/compare/v1.2.0...v1.3.0
[1.2.0]: https://github.com/opaopa6969/syslenz/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/opaopa6969/syslenz/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/opaopa6969/syslenz/releases/tag/v1.0.0

# Phase 3 Design Decisions: API v1, Settings GUI, Fleet View

**Date**: 2026-03-31  
**Status**: Partially implemented  
**Session**: DGE 020 (G20-12 through G20-15)

---

## Context

After Phase 1 (core TUI) and Phase 2 (Web UI, alerts, time-travel diff, multi-host), Phase 3 addressed the gap between syslenz as a single-host CLI tool and a team-usable monitoring platform.

Four work items were designed (DGE 020):

| ID | Feature | Status |
|----|---------|--------|
| G20-12 | Fleet View (`/fleet`) | Designed, **not implemented** |
| G20-13 | HTTP API v1 versioning | **Implemented** (v1.4.0) |
| G20-14 | Web authentication | Designed, **not implemented** |
| G20-15 | Settings GUI (`/settings`) | **Implemented** (v1.4.0) |

---

## Decisions

### G20-13: API v1 prefix and stability contract

**Decision**: Prefix all stable Web UI API routes with `/api/v1/`. Legacy routes (`/api/snapshot`, `/api/view`, etc.) remain unchanged and are not versioned in this release.

**Rationale**: Versioning only the settings and config-write endpoints in v1.4 provides a stable surface for scripts and SDKs that write alert rules without committing to version all read endpoints prematurely.

**Implementation**: All `/api/v1/*` responses include `X-Syslenz-API-Version: 1`.

**Endpoints shipped**:
- `GET /api/v1/settings` — read current config as JSON
- `POST /api/v1/settings/alerts` — write alert rules to config file

---

### G20-15: Settings GUI as server-side HTML

**Decision**: Implement `/settings` as a self-contained HTML page with inline JavaScript. No React, no external CDN.

**Rationale**: syslenz is a single binary. Adding a JS build step and CDN dependencies contradicts the zero-configuration design principle. A self-contained page keeps the binary self-sufficient.

**Trade-off**: The HTML is embedded as a Rust string in `web.rs`. This is harder to iterate on than a separate file but avoids file-serving complexity and keeps the binary count at one.

---

### G20-12: Fleet View deferred

**Decision**: Do not implement Fleet View in v1.4. Document it clearly as planned-but-unimplemented.

**Rationale**: Fleet View requires a backend that aggregates snapshots from multiple `--serve` instances. This is a non-trivial polling or push architecture. Shipping a skeleton page would mislead users. The correct approach is to implement the aggregation layer first, then the UI.

**What will be needed**:
- A `FleetConfig` with a list of `{name, addr}` entries
- A background task per remote that polls `SNAPSHOT` over TCP
- A `/fleet` route that renders the aggregated state matrix
- Per-host health indicators (last seen, alert count, top metrics)

---

### G20-14: Authentication deferred

**Decision**: Do not implement authentication in v1.4. Document the security posture clearly.

**Rationale**: Authentication requires a threat model review, a secure token storage design, and testing. Shipping incomplete auth is worse than shipping no auth with clear documentation. The current guidance (bind to loopback, use firewall) is accurate and sufficient for the primary use case (single-user, local host).

**What will be needed**:
- `[web] auth_type = "basic" | "token" | "none"` in `config.toml`
- `[web] username`, `[web] password_hash` for Basic Auth
- `[web] token` for token auth (Bearer header)
- Axum middleware layer applied to all routes
- HTTPS guidance (TLS termination at reverse proxy)

---

## Implications for users

- Do not expose `--serve` or `--web` on untrusted networks in the current release.
- The `/api/v1/` prefix is stable: scripts using `GET /api/v1/settings` will not break.
- Fleet View and auth timelines are not committed. Watch the [CHANGELOG](../../CHANGELOG.md) for updates.

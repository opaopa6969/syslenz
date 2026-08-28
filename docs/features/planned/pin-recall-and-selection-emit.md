# Pin/Recall and Selection Emit

- **Status**: Phase 1 implemented (issue #7)
- **Origin**: design discussion 2026-06-12 — "syslenz is a viewer; should it save/recall selected items as logs, or hand selections to a separate process?"

## Summary

Let users **save what they were looking at** (pinned source/field selections) and
**recall** that investigation context on the next launch — while keeping all
heavyweight data logging and processing **outside** the viewer, in separate
processes that consume what syslenz emits.

The core design decision is to separate two things that look similar but have
very different costs:

| | What is saved | Where it lives | Why |
|---|---|---|---|
| **Context** | *Which* items were selected (pins) | Inside the viewer (state file) | It is view state; tiny; restoring it is what "recall" actually means to a user mid-investigation |
| **Data** | The *values* of those items over time | Outside (append-only emit, consumed by other processes) | Owning storage turns the viewer into a database (rotation, schema, retention, search) |

This follows precedent already in the codebase: the TUI keeps 60 in-memory
snapshots for time-travel diff, `history.rs` records/imports snapshots,
alert history is appended as JSONL, and alert rules can fire external
commands (`action`) and webhooks (`notify`). syslenz already *records and
emits*; it has never *owned* a queryable datastore — and should not start.

## Design principles

1. **Recall context, not data.** Restoring "I was watching `meminfo.MemAvailable`
   and `jvm.heap_used`" is cheap and high-value. Replaying historical values is
   a different product.
2. **Emit, don't own.** Anything that accumulates goes to an append-only file or
   an external command. Rotation/retention/analysis belong to logrotate, jq,
   Prometheus, or any downstream process.
3. **Reuse existing seams.** Selection emit reuses the alert `action` template
   machinery; value logging is a constrained variant of `--export-series`;
   persistence follows the config-file conventions in `config.rs`.
4. **Zero new dependencies.** State files are TOML (already a dependency) or
   JSONL (already hand-emitted elsewhere).

## Feature 1 — Pins: save and recall selections

**UX**

- A keybinding (proposal: `p`; verify against the current keymap before
  implementation) toggles a pin on the focused source or `source.field`.
- Pinned items get a marker in the sidebar/detail view (e.g. `📌` or `*`,
  consistent with existing badge styling).
- A pinned-only filter view (proposal: `P`) shows just the pinned items across
  sources — the "my investigation" screen.
- Pins survive restart: on startup, pins are loaded and the marker/filter state
  is restored. If a pinned source/field is absent in the current snapshot it is
  shown greyed-out rather than dropped (the process you were watching may
  simply not be running *yet*).

**Storage**

`~/.config/syslenz/pins.toml` (same directory conventions as existing config):

```toml
# Written by syslenz; safe to edit by hand.
[[pin]]
source = "meminfo"
field  = "MemAvailable"   # omit `field` to pin a whole source
host   = ""                # multi-host: empty = local / primary

[[pin]]
source = "jvm"
field  = "heap_used"
host   = "tcp:127.0.0.1:9100"
```

- Load errors are non-fatal (warn to stderr, start with no pins) — same policy
  as `config.rs`.
- Names are written post-sanitization (snapshots are sanitized at ingestion
  since the control-character fix, so pin keys are always terminal-safe).

**Non-goals for this feature**: pin ordering/grouping UIs, per-pin notes,
syncing pins between machines.

## Feature 2 — `--log`: append-only value emit for pinned items

**UX**

```
syslenz --log /var/log/syslenz/pins.jsonl            # log pins every snapshot
syslenz --log pins.jsonl --interval 5                # custom cadence
syslenz --connect host:9100 --log pins.jsonl         # works in remote mode too
```

One JSON object per line, one line per pinned item per snapshot:

```json
{"ts":"2026-06-12T10:00:00Z","host":"","source":"meminfo","field":"MemAvailable","value":{"Bytes":8217034752}}
```

**Behavior**

- Append-only; flush per snapshot batch. No rotation, no size caps, no
  compaction — that is logrotate's job (document this explicitly).
- If no pins exist, `--log` logs nothing and prints a one-line hint.
- Works headless (no TUI) when combined with a future `--headless`/existing
  serve-style mode, but the initial scope is "TUI running, file fills up".
- Downstream examples to include in docs: `jq` one-liners, feeding gnuplot,
  `tail -f | grep`.

**Why not extend `--export-series`?** It exports *whole snapshots* at a fixed
cadence for a fixed count — a capture tool. `--log` is open-ended and
pin-scoped; conflating them would complicate both. They share the field
serialization helpers instead.

## Feature 3 — Selection action: hand the selected item to another process

**UX**

A keybinding (proposal: `!`) on the focused item runs a user-configured command
with the same placeholder template syntax as alert `action`:

```toml
# config.toml
[selection]
action = "tmux split-window 'watch -n1 \"grep {field} /proc/{source}\"'"
# placeholders: {host} {source} {field} {value} {unit}
```

- Reuses the detached-spawn executor from `alert::execute_actions` (BL-071) —
  the TUI never blocks.
- Multiple actions: `[[selection.actions]]` with a `name` each; if more than
  one is configured, the keypress opens a small picker.
- This is the composition escape hatch: anything "not simple" (databases,
  notebooks, custom analyzers) lives behind this boundary as a separate
  process.

## Explicit non-goals (the "stay a viewer" line)

- No embedded time-series database, no query language, no retention policies.
- No replay-from-log in the TUI (time-travel stays bounded to the in-memory
  60-snapshot window; `history.rs` import remains the offline path).
- No background daemon mode bundled into this work (revisit separately if
  `--log` without TUI proves wanted).

## Integration points

| Area | Existing code | Reuse |
|------|--------------|-------|
| Config & state files | `src/config.rs` | directory resolution, lenient load policy |
| Action execution | `src/alert.rs` (`execute_actions`) | placeholder expansion, detached spawn |
| Field serialization | `src/export.rs` / `src/history.rs` | JSONL value encoding |
| Sidebar/detail badges | `src/ui/render.rs` (alert severity badges) | marker rendering pattern |
| Snapshot cadence | `src/ui/app.rs` (`refresh`) | hook point for `--log` append |

## Phasing

1. **Pins + recall** (Feature 1) — pure viewer state, zero protocol impact,
   highest value/cost ratio. Ships alone.
2. **`--log`** (Feature 2) — depends on pins existing.
3. **Selection action** (Feature 3) — independent of 2; can ship in parallel
   after 1.

Each phase is one issue; the issues link back to this document.

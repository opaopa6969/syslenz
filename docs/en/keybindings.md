---
version: v1.0.0
lang: en
---

# Keybinding Reference

[<- Prev: Configuration](config.md) | [Index](index.md) | [Next: OpenTelemetry ->](otel.md)

[🇯🇵 日本語版](../ja/keybindings.md)

## Table of Contents

- [Global Keys](#global-keys)
- [Dashboard Keys](#dashboard-keys)
- [Classic Mode Keys](#classic-mode-keys)
- [Detail View Keys](#detail-view-keys)
- [Table View Keys](#table-view-keys)
- [Diff View Keys](#diff-view-keys)
- [Graph View Keys](#graph-view-keys)
- [Diagnostics Keys](#diagnostics-keys)
- [Category Guide Keys](#category-guide-keys)
- [Search Mode Keys](#search-mode-keys)

## Global Keys

These keys work in all views.

| Key | Action |
|-----|--------|
| `q` | Quit syslenz |
| `Esc` | Quit syslenz (or cancel search) |
| `D` | Switch to Dashboard view |
| `O` | Switch to Classic (Overview) mode |
| `W` | Switch to Welcome view |
| `X` | Switch to Diagnostics view |
| `C` | Switch to Category Guide view |
| `?` | Cycle help level: OFF -> NORMAL -> DETAILED -> EXTRA |
| `L` | Switch language (English <-> Japanese) |
| `a` | Toggle auto-refresh on/off |
| `r` | Manual refresh (capture new snapshot) |
| `e` | Export current snapshot to JSON file |
| `c` | Copy selected value to clipboard |

## Dashboard Keys

| Key | Action |
|-----|--------|
| `j` / `k` | Select next / previous section |
| Arrow Down / Up | Select next / previous section |
| `Enter` | Drill into selected section (opens Classic mode) |

## Classic Mode Keys

### Sidebar Focus

| Key | Action |
|-----|--------|
| `j` / `k` | Navigate source list down / up |
| Arrow Down / Up | Navigate source list down / up |
| `Enter` / Arrow Right | Select source and move to detail panel |
| `Tab` | Switch focus to content panel |
| `/` | Start search |
| `PageUp` / `PageDown` | Scroll source list by page |

### Content Focus

| Key | Action |
|-----|--------|
| `j` / `k` | Scroll fields down / up |
| Arrow Down / Up | Scroll fields down / up |
| `Enter` / Arrow Right | Drill into table field |
| `Backspace` / Arrow Left | Return to sidebar |
| `Tab` | Switch focus to sidebar |
| `d` | Open Diff view |
| `g` | Open Graph view for selected field |
| `PageUp` / `PageDown` | Scroll fields by page |

## Detail View Keys

| Key | Action |
|-----|--------|
| `j` / `k` | Scroll through fields |
| Arrow Down / Up | Scroll through fields |
| `Enter` | Drill into table fields |
| `Backspace` | Go back |
| `d` | Switch to Diff view |
| `g` | Open Graph for selected numeric field |
| `c` | Copy selected field value |

## Table View Keys

| Key | Action |
|-----|--------|
| `j` / `k` | Scroll rows down / up |
| Arrow Down / Up | Scroll rows down / up |
| `PageUp` / `PageDown` | Scroll rows by page |
| `Backspace` | Return to detail view |

## Diff View Keys

| Key | Action |
|-----|--------|
| `j` / `k` | Scroll diff entries down / up |
| Arrow Down / Up | Scroll diff entries down / up |
| `PageUp` / `PageDown` | Scroll by page |
| `Backspace` | Return to previous view |

## Graph View Keys

| Key | Action |
|-----|--------|
| `Backspace` | Return to previous view |

The graph auto-updates when auto-refresh is enabled.

## Diagnostics Keys

| Key | Action |
|-----|--------|
| `j` / `k` | Scroll findings down / up |
| Arrow Down / Up | Scroll findings down / up |
| `PageUp` / `PageDown` | Scroll by page |
| `c` | Copy diagnostics to clipboard |
| `Backspace` | Return to previous view |

## Category Guide Keys

| Key | Action |
|-----|--------|
| `h` / `l` | Switch to previous / next category |
| Arrow Left / Right | Switch to previous / next category |
| `j` / `k` | Scroll content down / up |
| Arrow Down / Up | Scroll content down / up |
| `PageUp` / `PageDown` | Scroll content by page |
| `Backspace` | Return to previous view |

## Search Mode Keys

Activated by pressing `/` in Classic mode.

| Key | Action |
|-----|--------|
| Type characters | Filter source list |
| `Enter` | Apply search filter |
| `Esc` | Cancel search |
| `Backspace` | Delete last character |

---

[<- Prev: Configuration](config.md) | [Index](index.md) | [Next: OpenTelemetry ->](otel.md)

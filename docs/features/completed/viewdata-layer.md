# ViewData Layer

- **Status**: completed
- **Source module**: `src/ui/view_data.rs`
- **DGE sessions**: 013-unified-ui-architecture

## Summary

The ViewData layer is an abstraction that decouples data preparation from rendering.
It transforms raw snapshot data into a structured, render-ready format that both the
TUI and web UI consume. This unified architecture ensures consistent behavior across
display backends.

## Key capabilities

- Unified data preparation for TUI and web UI
- Structured render-ready output format
- Decoupled from specific rendering backend
- Supports all view modes (Dashboard, Classic, Detail, Diff, Graph)

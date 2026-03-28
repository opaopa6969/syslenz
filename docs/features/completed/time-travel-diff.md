# Time-Travel Diff

- **Status**: completed
- **Source module**: `src/ui/app.rs`, `src/ui/render.rs`
- **User docs**: [en](../../en/classic-mode.md) (diff section) | [ja](../../ja/classic-mode.md)
- **DGE sessions**: 005-g5-g6-g7-timeseries-alerts-multihost

## Summary

Time-travel diff allows users to compare the current system snapshot against any
previous snapshot in the history buffer. Changed fields are highlighted with color-coded
diffs showing increases and decreases. This enables quick identification of what changed
between two points in time.

## Key capabilities

- Snapshot history buffer with configurable depth
- Side-by-side diff of any two snapshots
- Color-coded increase/decrease indicators
- Navigate through history with keyboard shortcuts

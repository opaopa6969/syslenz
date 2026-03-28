# Diagnostics

- **Status**: completed
- **Source module**: `src/diagnostics.rs`
- **User docs**: [en](../../en/diagnostics.md) | [ja](../../ja/diagnostics.md)
- **DGE sessions**: 009-education-diagnostics

## Summary

The diagnostics system analyzes parsed `/proc` and `/sys` data to surface actionable
health insights. It detects anomalies such as high memory pressure, swap thrashing,
excessive context switches, and disk saturation, then presents color-coded diagnostic
messages in the TUI and web UI.

## Key capabilities

- Rule-based diagnostic engine over parsed system metrics
- Severity levels (info, warning, critical)
- Automatic threshold detection with configurable sensitivity
- Integration with both TUI and web UI rendering

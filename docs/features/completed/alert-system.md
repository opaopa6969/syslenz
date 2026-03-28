# Alert System

- **Status**: completed
- **Source module**: `src/alert.rs`
- **User docs**: [en](../../en/config.md) (alert section) | [ja](../../ja/config.md)
- **DGE sessions**: 005-g5-g6-g7-timeseries-alerts-multihost

## Summary

The alert system monitors system metrics against user-defined thresholds and triggers
notifications when conditions are met. Alerts are defined in the TOML configuration
file using `[[alert]]` sections with field path, operator, and threshold value.

## Key capabilities

- Threshold-based alerting on any parsed metric field
- Configurable via `[[alert]]` TOML sections
- Visual alert indicators in TUI status bar
- Support for >, <, >=, <=, == operators

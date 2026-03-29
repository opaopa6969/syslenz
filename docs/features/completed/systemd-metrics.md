# systemd Metrics

- **Status**: completed
- **Version**: v1.3.0
- **Source module**: `src/sources/systemd.rs`
- **User docs**: [en](../../en/sources.md#systemd-v130) | [ja](../../ja/sources.md)

## Summary

systemd integration providing visibility into service unit status, failed services,
and timer schedules. Data is collected by invoking `systemctl` with machine-parseable
output flags. The failed service count integrates with the diagnostics engine to
generate warnings when services are in a failed state.

## Key capabilities

- `systemd/units` source: lists all service units with load, active, and sub states
- `systemd/failed` source: lists failed units with a `failed_count` summary field
- `systemd/timers` source: lists all timers with next/last run times
- Diagnostics integration: warns when failed_count > 0
- Compatible with systemd v230+ (covers all modern distributions)
- Graceful fallback: sources are omitted on non-systemd systems

# Remote Monitoring

- **Status**: completed
- **Source module**: `src/remote.rs`, `src/serve.rs`
- **User docs**: [en](../../en/remote.md) | [ja](../../ja/remote.md)
- **DGE sessions**: 005-g5-g6-g7-timeseries-alerts-multihost

## Summary

Remote monitoring allows syslenz to connect to a remote host via SSH and display its
system metrics in real-time. The remote host runs `syslenz --serve` to expose a JSON
snapshot endpoint, and the local client polls and renders the data in the standard TUI.

## Key capabilities

- SSH-based remote data collection
- `--serve` mode for headless JSON snapshot export
- Real-time refresh of remote metrics
- Same TUI experience for local and remote hosts

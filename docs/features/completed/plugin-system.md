# Plugin System

- **Status**: completed
- **Source module**: `src/plugin/mod.rs`
- **User docs**: [en](../../en/plugins.md) | [ja](../../ja/plugins.md)
- **DGE sessions**: 012-plugin-architecture-vision

## Summary

The plugin system allows extending syslenz with custom data sources beyond the built-in
`/proc` and `/sys` parsers. Plugins are external executables that output JSON in the
syslenz snapshot format. They are discovered and loaded via configuration.

## Key capabilities

- External executable plugin interface (JSON stdout protocol)
- Configuration-driven plugin discovery
- Plugin output integrated into standard TUI views
- Sandboxed execution with timeout handling

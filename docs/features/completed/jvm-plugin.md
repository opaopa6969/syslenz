# JVM Plugin

- **Status**: completed
- **Source module**: `src/plugin/mod.rs` + `providers/jvm/`
- **DGE sessions**: 012-plugin-architecture-vision

## Summary

A reference plugin implementation for monitoring JVM (Java Virtual Machine) metrics,
shipped in v1.2.0. The plugin demonstrates the plugin system's capabilities by
collecting GC stats, heap usage, thread counts, and class loading metrics from a
running JVM via `jstat` / `jcmd` (shell-based plugin under `providers/jvm/`,
invoked through `src/plugin/mod.rs`).

## Key capabilities

- JVM heap and GC metrics collection (`jstat`)
- Thread pool monitoring (`jcmd`)
- Class loading statistics
- Integration with syslenz alert system for JVM-specific thresholds

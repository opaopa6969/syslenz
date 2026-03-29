# Provider Ecosystem

- **Status**: completed
- **Version**: v1.3.0
- **Source module**: `src/providers/`
- **User docs**: [en](../../en/plugins.md#provider-ecosystem-v130) | [ja](../../ja/plugins.md)

## Summary

A curated provider system for collecting metrics from popular services. Providers
follow the same ProcEntry protocol as plugins but ship with syslenz and are configured
via `[[provider]]` entries in config.toml or the `--provider` CLI flag. Initial
providers cover MySQL, PostgreSQL, Redis, and nginx.

## Key capabilities

- `Provider` trait for implementing Rust-native providers
- Script-based providers supported via `~/.config/syslenz/providers/` directory
- Configuration via `[[provider]]` in config.toml with per-provider `[provider.config]`
- CLI activation via `--provider <name>` (repeatable)
- Built-in providers:
  - **MySQL**: connections, query rate, slow queries, InnoDB buffer pool, replication lag
  - **PostgreSQL**: connections, transaction rate, cache hit ratio, dead tuples, DB size
  - **Redis**: clients, memory, hit rate, ops/sec, evictions, blocked clients
  - **nginx**: active connections, accepts, handled, requests, reading/writing/waiting
- Providers coexist with user plugins; both appear in sidebar and all views
- Credentials read from environment variables for security (password_env pattern)

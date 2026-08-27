# Provider Ecosystem

- **Status**: completed
- **Version**: v1.3.0
- **Runtime**: `src/plugin/mod.rs`
- **Provider scripts**: `providers/`
- **User docs**: [en](../../en/plugins.md#provider-ecosystem-v130) | [ja](../../ja/plugins.md#provider-v130)

## Summary

syslenz ships curated executable providers for common services. A provider is a
shell-based plugin: install its executable in `~/.config/syslenz/plugins/`, set
the documented environment variables, and start syslenz. The regular plugin
loader discovers and executes it automatically.

Providers use the same `ProcEntry` JSON protocol, five-second timeout, automatic
discovery, and `plugin/<filename>` sidebar key as other plugins.

## Shipped providers

- **MySQL**: `providers/mysql/syslenz-provider-mysql`
- **PostgreSQL**: `providers/postgres/syslenz-provider-postgres`
- **Redis**: `providers/redis/syslenz-provider-redis`
- **nginx**: `providers/nginx/syslenz-provider-nginx`
- **Template**: `providers/template/syslenz-provider-template`

Each provider directory contains a README with dependencies, environment
variables, installation, and direct-execution test instructions.

# syslenz PostgreSQL Provider

Monitors a PostgreSQL server using `pg_stat_database`, `pg_stat_activity`, and `pg_database_size`.

## What it monitors

- **Connections**: backends, active/idle/idle-in-transaction counts
- **Transactions**: commits, rollbacks
- **Buffer cache**: blocks read from disk, cache hits, hit ratio
- **Tuple operations**: returned, fetched, inserted, updated, deleted
- **Conflicts and deadlocks**
- **Database size**

## Requirements

- `psql` CLI client installed and on `$PATH`
- Network access to the PostgreSQL server
- A PostgreSQL user with access to `pg_stat_database` and `pg_stat_activity`

## Configuration

Uses standard PostgreSQL environment variables:

| Variable     | Default     | Description              |
|--------------|-------------|--------------------------|
| `PGHOST`     | `localhost` | PostgreSQL hostname      |
| `PGPORT`     | `5432`      | PostgreSQL port          |
| `PGUSER`     | `postgres`  | PostgreSQL username      |
| `PGDATABASE` | `postgres`  | Database to monitor      |
| `PGPASSWORD` | (none)      | PostgreSQL password      |

You can also use a `.pgpass` file or `pg_service.conf` as usual.

## Installation

```bash
mkdir -p ~/.config/syslenz/plugins
cp syslenz-provider-postgres ~/.config/syslenz/plugins/
chmod +x ~/.config/syslenz/plugins/syslenz-provider-postgres

export PGUSER=monitor
export PGPASSWORD=secret
export PGDATABASE=myapp

syslenz
```

## Testing

```bash
./syslenz-provider-postgres | jq .
```

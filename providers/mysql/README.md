# syslenz MySQL Provider

Monitors a MySQL server by querying `SHOW GLOBAL STATUS`.

## What it monitors

- **Connections**: total attempts, active threads, connected threads
- **Query performance**: total queries, slow queries
- **InnoDB buffer pool**: read requests, cache misses, hit rate
- **InnoDB row operations**: rows read, inserted, updated, deleted
- **InnoDB locking**: row lock waits
- **Network I/O**: bytes received and sent
- **Uptime**: server uptime

## Requirements

- `mysql` CLI client installed and on `$PATH`
- Network access to the MySQL server
- A MySQL user with permission to run `SHOW GLOBAL STATUS`

## Configuration

Set these environment variables before running syslenz:

| Variable     | Default     | Description              |
|--------------|-------------|--------------------------|
| `MYSQL_USER` | `root`      | MySQL username           |
| `MYSQL_PASS` | (none)      | MySQL password           |
| `MYSQL_HOST` | `localhost` | MySQL server hostname    |
| `MYSQL_PORT` | `3306`      | MySQL server port        |

## Installation

```bash
# Copy the provider into the syslenz plugins directory
mkdir -p ~/.config/syslenz/plugins
cp syslenz-provider-mysql ~/.config/syslenz/plugins/
chmod +x ~/.config/syslenz/plugins/syslenz-provider-mysql

# Set credentials
export MYSQL_USER=monitor
export MYSQL_PASS=secret

# Run syslenz -- MySQL metrics appear under plugin/syslenz-provider-mysql
syslenz
```

## Testing

```bash
export MYSQL_USER=root
export MYSQL_PASS=yourpassword
./syslenz-provider-mysql | jq .
```

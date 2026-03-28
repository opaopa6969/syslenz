# syslenz Redis Provider

Monitors a Redis server using `redis-cli INFO`.

## What it monitors

- **Clients**: connected clients, blocked clients
- **Memory**: used memory, peak memory, RSS
- **Throughput**: total commands processed, instantaneous ops/sec
- **Cache effectiveness**: keyspace hits, misses, hit rate
- **Replication**: connected replicas
- **Uptime**

## Requirements

- `redis-cli` installed and on `$PATH`
- Network access to the Redis server

## Configuration

| Variable     | Default     | Description                                     |
|--------------|-------------|-------------------------------------------------|
| `REDIS_HOST` | `localhost` | Redis hostname                                  |
| `REDIS_PORT` | `6379`      | Redis port                                      |
| `REDIS_PASS` | (none)      | Redis AUTH password                              |
| `REDIS_URL`  | (none)      | Full Redis URL (overrides host/port/pass)       |

## Installation

```bash
mkdir -p ~/.config/syslenz/plugins
cp syslenz-provider-redis ~/.config/syslenz/plugins/
chmod +x ~/.config/syslenz/plugins/syslenz-provider-redis

# If auth is needed:
export REDIS_PASS=secret

syslenz
```

## Testing

```bash
./syslenz-provider-redis | jq .
```

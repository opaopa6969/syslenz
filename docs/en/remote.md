---
version: v1.1.0
lang: en
---

# Remote Monitoring

[🇯🇵 日本語版](../ja/remote.md)

[<- Prev: Education](education.md) | [Index](index.md) | [Next: Web UI ->](web-ui.md)


## Table of Contents

- [Overview](#overview)
- [SSH Mode](#ssh-mode)
- [Docker Mode](#docker-mode)
- [TCP Server/Client Mode](#tcp-serverclient-mode)
- [Docker Compose Setup](#docker-compose-setup)
- [Troubleshooting](#troubleshooting)

## Overview

syslenz supports three remote monitoring methods:

| Method | Flag | Use Case |
|--------|------|----------|
| SSH | `--ssh user@host` | Monitor remote servers with SSH access |
| Docker | `--docker container` | Monitor containers via `docker exec` |
| TCP | `--serve` / `--connect` | Lightweight agent for containers without SSH |

All three methods work by running syslenz on the remote target and streaming JSON snapshots back to the local TUI. The local TUI displays the remote system's data with full interactivity (Dashboard, Classic, Diagnostics, etc.).

## SSH Mode

```bash
syslenz --ssh user@host
```

**How it works:**
1. syslenz spawns `ssh -T -o BatchMode=yes -o ConnectTimeout=10 user@host syslenz --export -`
2. The remote syslenz captures a snapshot and writes JSON to stdout
3. The local syslenz deserializes the JSON and displays it in the TUI
4. This repeats every second (configurable via `interval_ms` in config)

**Requirements:**
- syslenz must be installed on the remote host and on the `PATH`
- SSH key-based authentication must be configured (BatchMode disables password prompts)
- The remote host must be reachable

**Features:**
- Inherits your local SSH agent and config (`~/.ssh/config`)
- Resilient to transient failures: up to 5 consecutive SSH failures are silently skipped before giving up
- The TUI title bar shows the remote host name

**Example with jump host:**

Configure your `~/.ssh/config`:

```
Host prod-server
    HostName 10.0.1.50
    User admin
    ProxyJump bastion.example.com
```

Then:

```bash
syslenz --ssh prod-server
```

## Docker Mode

```bash
syslenz --docker container_name
```

**How it works:**
1. syslenz spawns `docker exec container_name syslenz --export -`
2. The container's syslenz captures a snapshot from the container's `/proc`
3. JSON is streamed back to the local TUI

**Requirements:**
- syslenz must be installed inside the container
- `docker` must be on the local `PATH`
- The container must be running

**Note:** The snapshot reflects the container's view of `/proc`, which may differ from the host. Containers in the host PID namespace (`--pid=host`) see host-level data.

**Example: Adding syslenz to a Dockerfile:**

```dockerfile
FROM ubuntu:24.04
# ... your app setup ...

[🇯🇵 日本語版](../ja/remote.md)
COPY --from=syslenz/syslenz:latest /usr/local/bin/syslenz /usr/local/bin/syslenz
```

## TCP Server/Client Mode

For environments where SSH is not available (e.g., minimal containers, Kubernetes pods), syslenz includes a lightweight TCP protocol.

### Server Side

```bash
syslenz --serve [bind_addr]
```

Default bind address: `0.0.0.0:9100`

The server listens for TCP connections. When a client sends `SNAPSHOT\n`, the server captures a snapshot, serializes it as JSON, and sends it back. One request per connection (simple protocol).

The server handles one connection at a time in the same thread, making it extremely lightweight (no runtime dependencies, no async).

### Client Side

```bash
syslenz --connect host:port
```

The client connects to the TCP server every second, sends `SNAPSHOT\n`, reads the JSON response, and displays it in the local TUI.

**Example:**

On the remote machine or container:

```bash
syslenz --serve 0.0.0.0:9100
```

On your local machine:

```bash
syslenz --connect 192.168.1.100:9100
```

## Docker Compose Setup

A typical setup for monitoring application containers:

```yaml
version: "3.8"

services:
  app:
    image: myapp:latest
    # ... your app config ...

  syslenz-agent:
    image: syslenz/syslenz:latest
    command: ["syslenz", "--serve", "0.0.0.0:9100"]
    pid: "host"
    privileged: true
    ports:
      - "9100:9100"
```

Then from your workstation:

```bash
syslenz --connect your-docker-host:9100
```

### Monitoring Multiple Hosts

You can run multiple syslenz instances with different remote targets. While syslenz does not have a multi-host view, you can use multiple terminal panes:

```bash
# Terminal 1

[🇯🇵 日本語版](../ja/remote.md)
syslenz --ssh web-server-1

# Terminal 2

[🇯🇵 日本語版](../ja/remote.md)
syslenz --ssh web-server-2

# Terminal 3

[🇯🇵 日本語版](../ja/remote.md)
syslenz --connect db-server:9100
```

## Troubleshooting

### SSH: "syslenz not found"

The remote host does not have syslenz installed or it is not on the PATH.

**Fix:** Install syslenz on the remote host:

```bash
ssh user@host 'curl -L https://github.com/opaopa6969/syslenz/releases/latest/download/syslenz-linux-amd64 -o /usr/local/bin/syslenz && chmod +x /usr/local/bin/syslenz'
```

### SSH: "Permission denied"

SSH key authentication is not configured.

**Fix:** Set up SSH key auth:

```bash
ssh-keygen -t ed25519  # if you do not have a key
ssh-copy-id user@host
```

### SSH: Connection timeout

The remote host is unreachable or a firewall is blocking SSH.

**Fix:** Test connectivity manually:

```bash
ssh -o ConnectTimeout=10 user@host echo ok
```

### Docker: "No such container"

The container name is wrong or the container is not running.

**Fix:** Check running containers:

```bash
docker ps --format '{{.Names}}'
```

### Docker: "syslenz not found" in container

syslenz is not installed in the container image.

**Fix:** Add syslenz to your Dockerfile or install it at runtime:

```bash
docker exec container_name sh -c 'curl -L <release-url> -o /usr/local/bin/syslenz && chmod +x /usr/local/bin/syslenz'
```

### TCP: "Connection refused"

The syslenz server is not running, the port is wrong, or a firewall is blocking the connection.

**Fix:** Verify the server is listening:

```bash
ss -tlnp | grep 9100
```

### Remote stream stops after 5 failures

All three remote methods have a retry limit of 5 consecutive failures. After 5 failures, the stream stops and the TUI freezes on the last received snapshot.

**Fix:** Check network connectivity, restart the remote syslenz process, or restart the local syslenz.

---

[<- Prev: Education](education.md) | [Index](index.md) | [Next: Web UI ->](web-ui.md)

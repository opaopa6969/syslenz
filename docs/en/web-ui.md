---
version: v1.0.0
lang: en
---

# Web UI

[🇯🇵 日本語版](../ja/web-ui.md)

[<- Prev: Remote Monitoring](remote.md) | [Index](index.md) | [Next: Plugins ->](plugins.md)


## Table of Contents

- [Overview](#overview)
- [Starting the Web UI](#starting-the-web-ui)
- [Web Interface](#web-interface)
- [Real-Time Streaming](#real-time-streaming)
- [API Endpoints](#api-endpoints)
- [Language Support](#language-support)
- [Deployment Notes](#deployment-notes)

## Overview

syslenz includes an optional browser-based Web UI powered by Axum. It provides a live-updating dashboard accessible from any device with a web browser. The Web UI requires the `web` feature to be enabled at compile time.

## Starting the Web UI

### Quick Start with run-web.sh

The easiest way to start the Web UI is the `run-web.sh` convenience script in the repository root. It builds with the `web` feature if needed and starts the server:

```bash
./run-web.sh          # port 3000, English
./run-web.sh 8080     # port 8080, English
./run-web.sh 3000 ja  # port 3000, Japanese
```

### Compile with the web feature

```bash
cargo build --release --features web
```

### Start the server

```bash
# Default port (3000)

[🇯🇵 日本語版](../ja/web-ui.md)
syslenz --web

# Custom port

[🇯🇵 日本語版](../ja/web-ui.md)
syslenz --web 8080

# With Japanese locale

[🇯🇵 日本語版](../ja/web-ui.md)
syslenz --web 8080 --lang ja
```

The server binds to `0.0.0.0` and prints the URL:

```
syslenz web UI: http://localhost:3000
```

Open this URL in a browser.

### If web feature is not compiled in

If you run `syslenz --web` without the feature, you will see:

```
Web UI support is not compiled in. Rebuild with: cargo build --features web
```

## Web Interface

The Web UI serves a single-page application with a dashboard similar to the TUI:

- **System load** with 1/5/15 minute averages
- **Memory usage** with total, available, and cached breakdown
- **CPU utilization** percentage breakdown
- **Network interface traffic** per-interface RX/TX
- **Disk usage** for the root filesystem
- **Process summary** with state counts

The page auto-updates every second via Server-Sent Events (SSE) -- no manual refresh needed.

## Real-Time Streaming

The Web UI uses SSE (Server-Sent Events) for real-time updates:

1. The server captures a snapshot every second in a background task
2. Each snapshot is serialized to JSON and broadcast to all connected SSE clients
3. The browser receives events and updates the page without full reloads

A keep-alive ping is sent every 10 seconds to maintain the connection.

The server also maintains a history ring buffer of 60 snapshots for charting purposes.

## API Endpoints

The Web UI exposes a REST API that can be used independently of the browser interface:

### `GET /`

Returns the HTML dashboard page.

### `GET /api/snapshot`

Returns the current snapshot as JSON.

```bash
curl http://localhost:3000/api/snapshot | jq .
```

**Response:** Full `Snapshot` object with all entries and fields.

### `GET /api/history`

Returns the history buffer (up to 60 snapshots) as a JSON array.

```bash
curl http://localhost:3000/api/history | jq 'length'
```

### `GET /api/sources`

Returns a list of all data source names as a JSON array.

```bash
curl http://localhost:3000/api/sources | jq .
```

**Response example:**

```json
["buddyinfo", "cgroups", "cmdline", "conntrack", "cpuinfo", "df", ...]
```

### `GET /api/stream`

SSE endpoint for real-time snapshot streaming. Each event contains a full snapshot as JSON in the `data` field.

```bash
curl -N http://localhost:3000/api/stream
```

### `GET /api/view`

Returns a structured view representation. Accepts query parameters:

| Parameter | Values | Default |
|-----------|--------|---------|
| `view` | `dashboard`, `welcome`, `detail`, `diff`, `table`, `graph`, `diagnostics`, `category` | `dashboard` |
| `locale` | `en`, `ja` | Server's locale |

```bash
curl 'http://localhost:3000/api/view?view=diagnostics&locale=en'
```

## Language Support

The Web UI respects the `--lang` flag:

```bash
syslenz --web --lang ja
```

Individual API requests can also override the language via the `locale` query parameter on the `/api/view` endpoint.

## Deployment Notes

### Running behind a reverse proxy

The Web UI works behind nginx or similar reverse proxies. Ensure SSE connections are not buffered:

```nginx
location /api/stream {
    proxy_pass http://localhost:3000;
    proxy_http_version 1.1;
    proxy_set_header Connection '';
    proxy_buffering off;
    proxy_cache off;
}

location / {
    proxy_pass http://localhost:3000;
}
```

### Running in Docker

```dockerfile
FROM rust:1.85 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --features web

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/syslenz /usr/local/bin/
EXPOSE 3000
CMD ["syslenz", "--web"]
```

```bash
docker build -t syslenz-web .
docker run --rm --pid=host --privileged -p 3000:3000 syslenz-web
```

### Security considerations

- The Web UI has no authentication. Use a reverse proxy with auth if exposing to the internet.
- The API exposes detailed system information. Restrict access to trusted networks.
- CORS is enabled via `tower-http` for API consumers.

---

[<- Prev: Remote Monitoring](remote.md) | [Index](index.md) | [Next: Plugins ->](plugins.md)

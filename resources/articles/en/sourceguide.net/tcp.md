# TCP Connection Details

[日本語版](../ja/sourceguide.net/tcp.md)

---

## What is it?

/proc/net/tcp provides per-connection TCP socket details: local/remote address:port, state, send/receive queue sizes, and socket memory. Raw but complete view of all TCP connections.

---

## Quick start

```sh
cat /proc/net/tcp
# or use syslenz to browse with descriptions
```

---

## See also

- `ss.tcp_established`
- `net/sockstat.TCP_inuse`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

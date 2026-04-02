# Network Interface Statistics

[日本語版](../ja/sourceguide.net/dev.md)

---

## What is it?

/proc/net/dev provides per-interface packet and byte counters: received/transmitted packets and bytes, plus error and drop counters. The primary source for network throughput monitoring.

---

## Quick start

```sh
cat /proc/net/dev
# or use syslenz to browse with descriptions
```

---

## See also

- `net/snmp.Ip`
- `ss.tcp_established`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

# Socket Statistics

[日本語版](../ja/sourceguide.net/sockstat.md)

---

## What is it?

/proc/net/sockstat shows current socket counts by protocol: TCP sockets (inuse, orphan, timewait), UDP, raw, and fragment queues. Quick overview of socket resource usage.

---

## Quick start

```sh
cat /proc/net/sockstat
# or use syslenz to browse with descriptions
```

---

## See also

- `ss.tcp_established`
- `net/snmp.Tcp`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

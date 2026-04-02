# Extended Network Statistics

[日本語版](../ja/sourceguide.net/netstat.md)

---

## What is it?

/proc/net/netstat contains extended protocol statistics: TcpExt (130+ advanced TCP counters) and IpExt (extended IP counters). Used for deep TCP/IP debugging.

---

## Quick start

```sh
cat /proc/net/netstat
# or use syslenz to browse with descriptions
```

---

## See also

- `net/snmp.Tcp`
- `net/netstat.TcpExt`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

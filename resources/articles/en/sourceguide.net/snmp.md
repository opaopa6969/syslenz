# Protocol MIB Statistics

[日本語版](../ja/sourceguide.net/snmp.md)

---

## What is it?

/proc/net/snmp contains RFC-standard SNMP MIB statistics for IP, ICMP, TCP, UDP, and UDP-Lite protocols. These are the standard protocol health counters.

---

## Quick start

```sh
cat /proc/net/snmp
# or use syslenz to browse with descriptions
```

---

## See also

- `net/netstat.TcpExt`
- `ss.tcp_established`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

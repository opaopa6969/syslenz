# IP Statistics — /proc/net/snmp

[日本語版](../../ja/net/snmp.Ip.md)

---

## What is it?

Counters for IP packet processing: forwarding, fragmentation, reassembly, and errors from `/proc/net/snmp`.

These counters come from `/proc/net/snmp`, which tracks RFC-defined MIB (Management Information Base) statistics for each protocol.

---

## Key metrics to watch

```sh
cat /proc/net/snmp
```

---

## See also

- `sourceguide.net/snmp` — full /proc/net/snmp source overview
- `net/netstat.TcpExt` — extended TCP statistics (TcpExt/IpExt)

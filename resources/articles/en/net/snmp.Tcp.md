# TCP Statistics — /proc/net/snmp

[日本語版](../../ja/net/snmp.Tcp.md)

---

## What is it?

Core TCP protocol counters: connections opened/closed, segments sent/received, retransmissions, and errors from `/proc/net/snmp`.

These counters come from `/proc/net/snmp`, which tracks RFC-defined MIB (Management Information Base) statistics for each protocol.

---

## Key metrics to watch

**Key health indicators:**
- `Tcp_RetransSegs / Tcp_OutSegs` = retransmit rate (healthy: < 1%)
- `Tcp_AttemptFails` rising = connections failing (SYN timeouts, refused)
- `Tcp_EstabResets` rising = connections being forcefully closed (RST floods, app crashes)
- `Tcp_CurrEstab` = current connection load

```sh
grep ^Tcp /proc/net/snmp | awk 'NR==1{split($0,h)} NR==2{for(i=2;i<=NF;i++) print h[i], $i}'
```

---

## See also

- `sourceguide.net/snmp` — full /proc/net/snmp source overview
- `net/netstat.TcpExt` — extended TCP statistics (TcpExt/IpExt)

# UDP-Lite Statistics — /proc/net/snmp

[日本語版](../../ja/net/snmp.UdpLite.md)

---

## What is it?

UDP-Lite protocol counters (RFC 3828 — partial checksums for multimedia). Same structure as UDP counters.

These counters come from `/proc/net/snmp`, which tracks RFC-defined MIB (Management Information Base) statistics for each protocol.

---

## Key metrics to watch

**Key health indicators:**
- `Udp_NoPorts` rising → packets arriving for closed ports (port misconfiguration or scan)
- `Udp_RcvbufErrors` rising → receive buffer overflow — application too slow or buffer too small
- `Udp_InErrors` → delivery failures (non-port-related)

```sh
grep ^Udp  /proc/net/snmp | awk 'NR==1{split($0,h)} NR==2{for(i=2;i<=NF;i++) print h[i], $i}'
```

---

## See also

- `sourceguide.net/snmp` — full /proc/net/snmp source overview
- `net/netstat.TcpExt` — extended TCP statistics (TcpExt/IpExt)

# ICMP Statistics — /proc/net/snmp

[日本語版](../../ja/net/snmp.Icmp.md)

---

## What is it?

Counters for ICMP message processing: pings, errors, redirects, and time-exceeded messages from `/proc/net/snmp`.

These counters come from `/proc/net/snmp`, which tracks RFC-defined MIB (Management Information Base) statistics for each protocol.

---

## Key metrics to watch

**Normal systems:** `Icmp_InEchos` / `Icmp_OutEchoReps` should be roughly balanced (ping traffic).

**Warning signs:**
- `Icmp_InDestUnreachs` rising → routing problems or services unreachable
- `Icmp_InErrors` rising → malformed ICMP packets (possible attack)
- `Icmp_InRedirects` high → router is sending redirects (routing misconfiguration)

```sh
grep ^Icmp /proc/net/snmp | awk 'NR==1{split($0,h)} NR==2{for(i=2;i<=NF;i++) print h[i], $i}'
```

---

## See also

- `sourceguide.net/snmp` — full /proc/net/snmp source overview
- `net/netstat.TcpExt` — extended TCP statistics (TcpExt/IpExt)

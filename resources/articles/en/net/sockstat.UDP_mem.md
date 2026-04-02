# UDP_mem

[日本語版](../../ja/net/sockstat.UDP_mem.md)

---

`UDP_mem` — UDP socket memory in pages.

**Source:** `/proc/net/sockstat`

**Quick reference for TCP socket states:**
- `TCP_inuse` = active sockets (ESTABLISHED, LISTEN, SYN_SENT, etc.)
- `TCP_orphan` = sockets with no owner — high values mean connection leak
- `TCP_tw` = TIME-WAIT — normal after HTTP traffic; high = port exhaustion risk

---

## See also

- `ss.tcp_established` — live connection counts
- `net/snmp.Tcp` — TCP connection open/close/reset counters
- `sourceguide.net/sockstat` — full source overview

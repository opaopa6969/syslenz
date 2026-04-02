# Routing Table

[日本語版](../ja/sourceguide.net/route.md)

---

## What is it?

The kernel IP routing table from /proc/net/route: destination networks, gateways, interface assignments, and metrics. Essential for diagnosing routing problems.

---

## Quick start

```sh
cat /proc/net/route
# or use syslenz to browse with descriptions
```

---

## See also

- `net/arp.entries`
- `net/snmp.Ip`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

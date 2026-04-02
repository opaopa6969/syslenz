# Connection Tracking

[日本語版](../ja/sourceguide.conntrack.md)

---

## What is it?

Connection tracking (/proc/net/nf_conntrack) tracks all active network connections that pass through the Linux netfilter layer. Used by NAT, firewalls, and stateful packet filtering.

---

## Quick start

```sh
cat /proc/conntrack
# or use syslenz to browse with descriptions
```

---

## See also

- `ss.tcp_established`
- `net/snmp.Tcp`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

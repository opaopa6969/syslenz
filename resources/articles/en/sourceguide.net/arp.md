# ARP Table

[日本語版](../ja/sourceguide.net/arp.md)

---

## What is it?

/proc/net/arp shows the ARP (Address Resolution Protocol) cache: IP-to-MAC address mappings on the local network. Useful for diagnosing network connectivity issues.

---

## Quick start

```sh
cat /proc/net/arp
# or use syslenz to browse with descriptions
```

---

## See also

- `net/route.routes`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

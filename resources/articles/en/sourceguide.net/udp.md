# UDP Socket Details

[日本語版](../ja/sourceguide.net/udp.md)

---

## What is it?

/proc/net/udp shows all open UDP sockets with local address:port, remote address, queue sizes, and inode. Similar to /proc/net/tcp but for UDP.

---

## Quick start

```sh
cat /proc/net/udp
# or use syslenz to browse with descriptions
```

---

## See also

- `net/snmp.Udp`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

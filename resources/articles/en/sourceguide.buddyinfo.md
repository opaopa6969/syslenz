# Buddy Allocator Info

[日本語版](../ja/sourceguide.buddyinfo.md)

---

## What is it?

/proc/buddyinfo shows how many free pages are available at each order (power-of-2 size) in each memory zone. It's the kernel's view of physical memory fragmentation.

---

## Quick start

```sh
cat /proc/buddyinfo
# or use syslenz to browse with descriptions
```

---

## See also

- `vmstat.compact`
- `vmstat.thp`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

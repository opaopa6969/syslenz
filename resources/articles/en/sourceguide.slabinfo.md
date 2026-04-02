# Kernel Slab Allocator

[日本語版](../ja/sourceguide.slabinfo.md)

---

## What is it?

/proc/slabinfo shows per-cache slab allocator statistics: object counts, sizes, and allocation rates. The most detailed view of kernel object memory usage.

---

## Quick start

```sh
cat /proc/slabinfo
# or use syslenz to browse with descriptions
```

---

## See also

- `meminfo.Slab`
- `meminfo.SReclaimable`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

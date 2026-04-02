# Page Allocation by Zone (pgalloc/pgskip) — vmstat

[日本語版](../ja/vmstat.pgalloc.md)

---

## What is it?

When the kernel allocates a page, it picks from one of several **memory zones**. Each zone serves different purposes and constraints:

| Zone | Physical address range | Purpose |
|------|------------------------|---------|
| `dma32` | < 4 GB | Legacy DMA devices |
| `normal` | > 4 GB (64-bit) | Main system RAM |
| `movable` | Configurable | Hot-pluggable memory |
| `device` | Device memory | GPU/persistent memory |

**`pgalloc_*`** counts successful allocations from each zone.  
**`pgskip_*`** counts times a zone was skipped during reclaim because it had nothing to reclaim.

---

## Why does it matter?

Normal operation: almost all allocations come from `pgalloc_normal`. `pgalloc_dma32` should be low on modern systems.

**High `pgskip_normal`** during a memory pressure event means the normal zone has nothing left to reclaim — the kernel is cycling through zones fruitlessly.

**`pgfree`** (also in this family conceptually) should roughly track with total pgalloc — the kernel frees about as much as it allocates in steady state.

```sh
# Watch allocation patterns
watch -n 2 'grep "pgalloc\|pgskip\|pgfree" /proc/vmstat'
```

---

## See also

- `vmstat.pgscan_pgsteal` — reclaim pipeline
- `vmstat.allocstall` — stalls when zones can't satisfy allocation
- `buddyinfo.zones` — per-zone free page fragmentation
- `sourceguide.vmstat` — full vmstat source overview

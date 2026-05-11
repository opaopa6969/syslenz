# Reclaim vs. Cache

[日本語版](../ja/concept.reclaim-vs-cache.md)

---

Linux uses free memory as file cache — this is normal and desirable. But there's a crucial distinction between **cache** and **working set**.

**Page cache** = files read from disk, kept in RAM for reuse. Free to evict.

**Working set** = pages actively needed by running processes. Evicting these causes refaults (disk reads).

RAM usage breakdown:

| Region | Reclaim policy |
|---|---|
| AnonPages (app heap/stack) | keep |
| File cache: Active(file) | keep |
| File cache: Inactive(file) | evict |
| MemFree | (free pool) |

**When reclaim is healthy:**
- `Inactive(file)` shrinks
- `workingset_refault_file` stays near 0
- Page cache just getting smaller — no penalty

**When reclaim is hurting:**
- `workingset_refault_file` rising (evicting working set)
- `vmstat.pgscan_direct` high (foreground reclaim)
- Each eviction causes a disk read on re-access

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

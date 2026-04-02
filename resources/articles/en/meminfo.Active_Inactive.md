# Active and Inactive Memory — meminfo

[日本語版](../ja/meminfo.Active_Inactive.md)

---

## What is it?

Linux's page reclaim algorithm sorts pages into **active** (recently used) and **inactive** (not recently used) lists. Pages on the inactive list are first candidates for eviction when memory gets tight.

| Metric | What it is |
|--------|-----------|
| `Active` | Total active memory (anon + file) |
| `Inactive` | Total inactive memory (anon + file) |
| `Active(anon)` | Recently used heap/stack/mmap memory |
| `Inactive(anon)` | Inactive anonymous memory — swap candidates |
| `Active(file)` | Recently used file cache |
| `Inactive(file)` | File cache eligible for eviction |

---

## Why does it matter?

**`Inactive(anon)` growing** = the kernel has anonymous pages it's considering swapping. Watch for `pswpout` starting to rise alongside this.

**`Inactive(file)` shrinking** = page cache is being depleted under pressure. Subsequent file reads will miss cache and hit disk.

**Healthy pattern on a stable system:**
```
Active(file):   large (working file cache)
Inactive(file): large (cold cache, available to evict)
Active(anon):   moderate (active app memory)
Inactive(anon): near 0 (no memory to swap)
```

Under memory pressure, `Inactive(file)` shrinks first (cache eviction), then `Inactive(anon)` grows (swap candidates accumulate).

---

## See also

- `meminfo.MemAvailable` — overall memory availability
- `vmstat.nr_active_inactive` — same data in vmstat format
- `vmstat.pgscan_pgsteal` — what happens when these lists are processed
- `sourceguide.meminfo` — full meminfo source overview

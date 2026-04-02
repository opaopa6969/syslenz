# Active and Inactive Page Lists — vmstat

[日本語版](../ja/vmstat.nr_active_inactive.md)

---

## What is it?

Linux maintains two LRU (Least Recently Used) lists for page cache management:

- **Active list**: pages recently accessed — protected from reclaim
- **Inactive list**: pages not recently accessed — candidates for eviction

Each list is split by page type:

| Metric | What it counts |
|--------|---------------|
| `nr_active_anon` | Active anonymous pages (heap, stack, mmap) |
| `nr_inactive_anon` | Inactive anonymous pages (swap candidates) |
| `nr_active_file` | Active file-backed pages (page cache, mmap files) |
| `nr_inactive_file` | Inactive file-backed pages (eviction candidates) |

---

## Why does it matter?

**The active/inactive ratio tells you about memory pressure:**

```
  Healthy (plenty of RAM):
  active_file: large     inactive_file: large
  active_anon: moderate  inactive_anon: near 0

  Memory pressure:
  active_file: shrinking  inactive_file: shrinking (cache being evicted)
  inactive_anon: growing  (swap candidates building up)
```

**High `nr_inactive_anon`** means the kernel has pages that it's considering swapping out. If `pswpout` starts rising alongside this, swap is happening.

**Shrinking `nr_inactive_file`** means the file cache is being depleted — subsequent file reads will hit disk.

The kernel automatically promotes pages from inactive to active on second access, and demotes active pages to inactive when memory is needed.

---

## See also

- `meminfo.Active` / `meminfo.Inactive` — same concept in meminfo
- `vmstat.kswapd` — reclaimer that processes these lists
- `vmstat.swap` — what happens to inactive anonymous pages
- `sourceguide.vmstat` — full vmstat source overview

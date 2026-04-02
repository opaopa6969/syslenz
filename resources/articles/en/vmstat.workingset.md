# Working Set — vmstat

[日本語版](../ja/vmstat.workingset.md)

---

## What is it?

The **working set** is the set of pages a process actually uses right now. Linux tracks this to make smarter eviction decisions. When memory gets tight, the kernel evicts pages from the page cache. If an evicted page is accessed again, that's a **refault** — the kernel had to re-read it from disk.

The working set machinery uses "shadow entries" — tiny metadata left where a page was evicted — to track whether re-accessed pages were evicted recently or long ago.

```
  Page is evicted from cache:
  [Page Data] removed, [Shadow Entry] left behind

  Page is re-accessed:
  Shadow found → "refault" counted → page was in working set
  No shadow   → "cold fault" → page wasn't being used
```

---

## Metrics

| Metric | What it counts |
|--------|---------------|
| `workingset_refault_anon` | Anonymous page refaults (was evicted, now needed) |
| `workingset_refault_file` | File page refaults |
| `workingset_activate_anon` | Anon pages promoted to active list after refault |
| `workingset_activate_file` | File pages promoted to active list after refault |
| `workingset_restore_anon` | Anon pages whose shadow was kept (working set) |
| `workingset_restore_file` | File pages whose shadow was kept |
| `workingset_nodereclaim` | Shadow node reclaim events |
| `workingset_nodes` | Current shadow node count |

---

## Why does it matter?

**High `workingset_refault_file` = your page cache is too small.**

If files keep getting evicted and then re-read, you're doing unnecessary disk I/O. The system is thrashing its file cache.

**High `workingset_refault_anon` = swap thrash.**

Anonymous page refaults mean swap pages are being re-read from disk. This is costly — each refault is a disk read.

```sh
# Watch refault rate
watch -n 5 'grep workingset /proc/vmstat'

# High refault_file → add RAM, or identify which process is evicting cache
vmstat -s | grep -i "page"
```

---

## See also

- `meminfo.Cached` — file page cache size
- `meminfo.SwapFree` — swap headroom
- `vmstat.pgmajfault` — major page faults
- `sourceguide.vmstat` — full vmstat source overview

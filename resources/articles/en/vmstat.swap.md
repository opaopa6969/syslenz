# Swap I/O — vmstat

[日本語版](../ja/vmstat.swap.md)

---

## What is it?

When physical RAM runs low, the kernel moves anonymous pages (heap, stack) to swap space on disk. These counters track that activity:

| Metric | What it counts |
|--------|---------------|
| `pswpin` | Pages swapped **in** (disk → RAM) |
| `pswpout` | Pages swapped **out** (RAM → disk) |
| `swap_ra` | Swap read-ahead pages (prefetched) |
| `swap_ra_hit` | Swap read-ahead hits (prefetch was useful) |

---

## Why does it matter?

**`pswpout` rising** = RAM is full and the kernel is actively evicting memory to disk. Each swap-out is a disk write.

**`pswpin` rising** = previously swapped pages are being brought back. This means processes are accessing memory that was evicted — a disk read per page. This is the **swap I/O latency** your applications feel.

**The ratio matters:**
- `pswpout` with no `pswpin`: memory is being evacuated (possible preemptive swapping)
- Both rising: active swap thrash — processes are competing for RAM, each page-in causes another page-out

**Swap read-ahead efficiency:**
```
swap_ra_hit / swap_ra = read-ahead hit rate
```
Low hit rate means the kernel is prefetching swap pages that won't be needed — wasted I/O.

```sh
# Watch swap activity live
vmstat 1 | awk '{print $7, $8}'  # si=swap-in, so=swap-out

# Or directly
watch -n 2 'grep "pswp\|swap_ra" /proc/vmstat'
```

---

## A real episode

A Redis instance was set up with 8 GB RAM on a host with 16 GB total. Over weeks, the dataset grew to 9 GB. At 3 AM, `pswpout` started climbing — Redis was getting swapped. Redis is latency-critical: a swap-in mid-command caused the client timeout to fire, triggering connection retries, which triggered more memory allocation, which triggered more swapping.

The latency spiral took down 30% of the application servers before the on-call team noticed `vmstat -s` showed swap active.

Fix: add swap monitoring (`pswpout > 0` alert), and set `maxmemory` in Redis config.

---

## See also

- `meminfo.SwapFree` — remaining swap headroom
- `meminfo.SwapCached` — swap pages also in RAM
- `pressure.memory_some_avg10` — memory stall signal
- `sourceguide.vmstat` — full vmstat source overview

# SwapFree

[日本語版](../ja/meminfo.SwapFree.md)

---

## What is it?

`SwapFree` is the unused space remaining in your swap partition or swap file. Swap is disk space reserved to hold anonymous memory pages that the kernel has evicted from RAM.

Think of swap as a safety net at the bottom of a tightrope — it stops you from crashing completely, but you never want to fall onto it. Swap is not extra RAM. Even on an NVMe SSD, swap is **10 to 100 times slower** than RAM.

```
  RAM: nanosecond access
  ┌──────────────────────────────────────┐
  │ App A  │ App B  │ Cache  │ Free      │
  └──────────────────────────────────────┘
                ↓ kernel evicts App B to swap
  Swap (disk): microsecond to millisecond access
  ┌────────────────┐
  │ App B (paged)  │ SwapTotal - SwapFree
  └────────────────┘
```

`SwapFree = SwapTotal - (swap currently in use)`

---

## Why does it matter?

**SwapFree declining over time means your system is accumulating swap usage.** If it never comes back up, something is leaking memory. If it bounces — swap fills and drains — the system is under pressure but recovering.

**The real danger is not SwapFree = 0; it's active swap I/O.** A system with 2 GB in swap but no active swapping is stable. A system with 500 MB in swap and `vmstat` showing `so=50` (50 pages swapped out per second) is under active stress — your applications are running in slow motion.

**SwapFree = SwapTotal (no swap used) is ideal** but doesn't mean you're safe — it might mean swap hasn't been needed *yet*. Watch `MemAvailable` for the leading indicator.

---

## How to read it

```sh
# Swap status
grep -E "SwapTotal|SwapFree|SwapCached" /proc/meminfo

# Swap in use as a percentage
awk '/SwapTotal/{t=$2} /SwapFree/{f=$2} END{
  if(t>0) printf "Swap used: %.1f%%\n", (t-f)/t*100
  else print "No swap configured"
}' /proc/meminfo

# CRITICAL: check for active swap I/O, not just usage
vmstat 1 10
# si = pages swapped in from disk per second
# so = pages swapped out to disk per second
# Non-zero so means the system is paging RIGHT NOW
```

| Situation | What to do |
|-----------|------------|
| SwapFree = SwapTotal, so=0 | Healthy — swap unused |
| SwapFree < SwapTotal, so=0 | Swap used but not active — monitor trend |
| SwapFree declining steadily | Memory leak — find with AnonPages |
| so > 0 continuously | Active pressure — investigate now |
| SwapFree = 0, so > 0 | Critical — OOM likely |

---

## A real episode

A Redis instance on a 16 GB server had been running for six months without issues. The operations team noticed query latencies creeping up — from 0.5ms to 3ms over two weeks. Nothing obvious in CPU or network graphs.

Someone ran `vmstat 1` and saw `si=12` — 12 pages being swapped back in every second, steadily. `SwapFree` showed 1.4 GB used out of 4 GB total. Not alarming on its own. But `so` and `si` were non-zero around the clock.

The cause: Redis was configured with `maxmemory` set to 12 GB, but the OS was also running several background jobs that accumulated 4 GB of anonymous pages over weeks. The kernel was slowly evicting older Redis pages to swap. When those pages were accessed, they had to be swapped back in — causing the latency spikes.

Fix: reduce the background job memory footprint, increase Redis `maxmemory-policy` to evict more aggressively within Redis rather than letting the kernel manage it.

The lesson: swap usage can be silent for a long time. The real signal is `vmstat`'s `si`/`so`, not just `SwapFree`.

---

## What to do when SwapFree is declining

**Step 1: Check if swap is actively being used (not just allocated).**
```sh
vmstat 1 10
# Look at si and so columns
```

**Step 2: Find the memory hog.**
```sh
# Check AnonPages trend — is it growing?
grep AnonPages /proc/meminfo

# Which processes have the most RSS?
ps aux --sort=-%mem | head -15
```

**Step 3: Check how long swap has been filling.**
If you have historical monitoring, check the slope. Steady slow decline = probable leak. Fast recent change = load spike.

**Step 4: If active swapping (so > 0):**
- Short term: identify and restart the leaking process, or free other memory
- Medium term: add RAM or reduce per-process memory limits
- Do NOT rely on swap as a permanent solution

**Step 5: Consider swappiness tuning** (only if you understand the tradeoff):
```sh
# Check current setting (default: 60)
cat /proc/sys/vm/swappiness

# Lower value = kernel prefers keeping anon pages in RAM over swapping
# 10 is common for latency-sensitive workloads
sysctl vm.swappiness=10
```

---

## Common mistakes

**Treating swap as "extra RAM."** Swap is a fallback for emergencies. Sizing swap generously does not mean you can run more applications — it just means the OOM killer arrives later, after performance has already degraded significantly.

**Ignoring active swap I/O because SwapFree looks OK.** 1 GB of swap used with `so=100` is far worse than 3 GB used with `so=0`.

**Setting swappiness=0.** This tells the kernel to avoid swap entirely, which can cause the OOM killer to fire sooner than necessary. `swappiness=10` is a better tradeoff.

**Not having swap at all on production systems.** Without swap, the OOM killer fires the moment memory runs out. With swap, there's a degraded-but-functional grace period to notice and respond.

---

## See also

- `meminfo.SwapTotal` — total swap space configured
- `meminfo.MemAvailable` — leading indicator before swap starts filling
- `meminfo.AnonPages` — what fills up swap (anonymous pages)
- `vmstat.si` / `vmstat.so` — actual swap I/O rate (the real latency signal)
- `pressure/memory_some_avg10` — kernel stall signal that precedes OOM

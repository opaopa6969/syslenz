# kswapd — vmstat

[日本語版](../ja/vmstat.kswapd.md)

---

## What is it?

**kswapd** is the kernel's background memory reclaim daemon. While `allocstall` represents emergency synchronous reclaim (bad), kswapd runs proactively in the background (good — usually).

kswapd wakes up when free memory drops below the **low watermark** and works to get memory above the **high watermark**. It reclaims pages from the page cache and, if necessary, swaps anonymous pages.

```
  Memory pressure model:
  
  High watermark ──── kswapd stops ────────────────────
  Low watermark  ──── kswapd starts ──────────────────
  Min watermark  ──── direct reclaim starts ──────────
  OOM            ──── OOM killer fires ───────────────
```

---

## Metrics

| Metric | What it counts |
|--------|---------------|
| `kswapd_inodesteal` | Inodes freed by kswapd to reclaim memory |
| `kswapd_low_wmark_hit_quickly` | kswapd reached low watermark quickly (easy reclaim) |
| `kswapd_high_wmark_hit_quickly` | kswapd reached high watermark quickly (fast recovery) |

**`pageoutrun`** — how many times kswapd was woken up for page-out — is also in this group conceptually.

---

## Why does it matter?

**kswapd_inodesteal rising** means kswapd is freeing inode caches aggressively. This causes subsequent file operations to be slower (cold directory cache). It signals sustained memory pressure.

**Neither low_wmark nor high_wmark hit quickly** means kswapd is struggling — it's taking many iterations to free enough memory. Memory is under heavy pressure.

**Watch this combination:**
```sh
grep 'kswapd\|pageoutrun\|allocstall' /proc/vmstat
```
kswapd running frequently + allocstall also rising = memory is critically tight.

---

## See also

- `vmstat.allocstall` — direct reclaim stalls (worse than kswapd)
- `vmstat.pgscan_pgsteal` — the reclaim pipeline
- `pressure.memory_some_avg10` — stall signal
- `sourceguide.vmstat` — full vmstat source overview

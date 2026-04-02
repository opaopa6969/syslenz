# Page Scan and Steal — vmstat

[日本語版](../ja/vmstat.pgscan_pgsteal.md)

---

## What is it?

When the kernel reclaims memory, it runs in two phases:
1. **Scan**: walk the LRU lists looking for candidate pages to evict
2. **Steal**: actually evict (reclaim) those pages

The ratio of steal to scan is **reclaim efficiency**. If the kernel scans 1000 pages to reclaim 10, something is wrong.

---

## Metrics

**By reclaimer:**

| Metric | Reclaimer |
|--------|-----------|
| `pgscan_kswapd` / `pgsteal_kswapd` | Background kswapd daemon |
| `pgscan_direct` / `pgsteal_direct` | Direct reclaim (foreground, stalls process) |
| `pgscan_khugepaged` / `pgsteal_khugepaged` | khugepaged (THP collapse) |

**By page type:**

| Metric | Type |
|--------|------|
| `pgscan_anon` / `pgsteal_anon` | Anonymous pages (heap, stack) |
| `pgscan_file` / `pgsteal_file` | File-backed pages (page cache) |

**`pgscan_direct_throttle`** — direct reclaim was throttled (too much pressure).

**`pgskip_*`** — pages skipped during reclaim scan per zone.

---

## Why does it matter?

**Efficiency ratio:**
```
efficiency = pgsteal_* / pgscan_*
```
Healthy: 50–90%. Below 20% means the kernel is spending a lot of effort scanning pages it can't reclaim (e.g., all pages are mlock'd, or dirty pages that need writeback first).

**Direct vs kswapd ratio:**
- Mostly `pgscan_kswapd`: background reclaim keeping up — normal
- High `pgscan_direct`: applications are triggering reclaim themselves — latency impact

**Anon vs file:**
- High `pgscan_anon` relative to file: the kernel is considering swapping out application memory
- High `pgsteal_anon`: swap is actually happening

---

## See also

- `vmstat.kswapd` — background reclaim daemon
- `vmstat.allocstall` — direct reclaim stalls
- `vmstat.workingset` — what gets refaulted after eviction
- `sourceguide.vmstat` — full vmstat source overview

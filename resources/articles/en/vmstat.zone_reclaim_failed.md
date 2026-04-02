# zone_reclaim_failed

[日本語版](../ja/vmstat.zone_reclaim_failed.md)

---

Part of the **[vmstat.pgscan_pgsteal](../vmstat.pgscan_pgsteal.md)** family — see the group article for full context, tuning, and real episodes.

`zone_reclaim_failed` — Zone reclaim failures (NUMA zone reclaim could not free enough).

**Source:** `/proc/vmstat`  
**Unit:** cumulative count (monotonically increasing since boot)

---

## See also

- `vmstat.pgscan_pgsteal` — group article with full context
- `sourceguide.vmstat` — full vmstat source overview
- `pressure.memory_some_avg10` — kernel memory stall signal

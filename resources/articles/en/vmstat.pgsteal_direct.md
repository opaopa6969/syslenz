# pgsteal_direct

[日本語版](../ja/vmstat.pgsteal_direct.md)

---

Part of the **[vmstat.pgscan_pgsteal](../vmstat.pgscan_pgsteal.md)** family — see the group article for full context, tuning, and real episodes.

`pgsteal_direct` — Pages reclaimed by direct reclaim (foreground, stalls process).

**Source:** `/proc/vmstat`  
**Unit:** cumulative count (monotonically increasing since boot)

---

## See also

- `vmstat.pgscan_pgsteal` — group article with full context
- `sourceguide.vmstat` — full vmstat source overview
- `pressure.memory_some_avg10` — kernel memory stall signal

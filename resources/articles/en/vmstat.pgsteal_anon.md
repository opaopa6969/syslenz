# pgsteal_anon

[日本語版](../ja/vmstat.pgsteal_anon.md)

---

Part of the **[vmstat.pgscan_pgsteal](../vmstat.pgscan_pgsteal.md)** family — see the group article for full context, tuning, and real episodes.

`pgsteal_anon` — Anonymous pages actually reclaimed (swapped or freed).

**Source:** `/proc/vmstat`  
**Unit:** cumulative count (monotonically increasing since boot)

---

## See also

- `vmstat.pgscan_pgsteal` — group article with full context
- `sourceguide.vmstat` — full vmstat source overview
- `pressure.memory_some_avg10` — kernel memory stall signal

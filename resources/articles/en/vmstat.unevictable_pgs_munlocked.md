# unevictable_pgs_munlocked

[日本語版](../ja/vmstat.unevictable_pgs_munlocked.md)

---

Part of the **[vmstat.unevictable_pgs](../vmstat.unevictable_pgs.md)** family — see the group article for full context, tuning, and real episodes.

`unevictable_pgs_munlocked` — Pages released from mlock() back to evictable LRU.

**Source:** `/proc/vmstat`  
**Unit:** cumulative count (monotonically increasing since boot)

---

## See also

- `vmstat.unevictable_pgs` — group article with full context
- `sourceguide.vmstat` — full vmstat source overview
- `pressure.memory_some_avg10` — kernel memory stall signal

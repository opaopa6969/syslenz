# How to Read Linux Metrics

[日本語版](../ja/concept.reading-metrics.md)

---

Several principles make metric reading more effective.

**Counters vs. gauges:**
- **Counters**: monotonically increasing since boot (`vmstat.pgfault`, `stat.context_switches`)
  - To get rate: subtract previous reading, divide by time
  - syslenz shows them as cumulative counts
- **Gauges**: current instantaneous value (`meminfo.MemAvailable`, `loadavg.load_1min`)

**Rate of change matters more than absolute values:**
- `vmstat.nr_dirty = 50000` might be fine
- `vmstat.nr_dirty` climbing 1000/second for 10 minutes is not fine

**Ratios reveal efficiency:**
- `pgsteal / pgscan` = reclaim efficiency
- `numa_hit / (numa_hit + numa_miss)` = NUMA efficiency
- `thp_fault_alloc / (thp_fault_alloc + thp_fault_fallback)` = THP success rate

**Context matters:**
A web server and a database have different normal ranges. Know your baseline.

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

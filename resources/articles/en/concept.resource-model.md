# Linux Resource Model

[日本語版](../ja/concept.resource-model.md)

---

A Linux server has four primary resources. Every performance problem traces back to one or more of them.

| Resource | Substructure | Concerns |
|---|---|---|
| CPU | Runqueue | Context switching, Scheduling |
| Memory | LRU lists | Swap/reclaim, OOM |
| I/O | I/O queue | Disk queue, IOPS limit |
| Network | Socket buffers | TCP stack, Bandwidth |

**Monitoring each:**
- CPU: `loadavg`, `stat.cpu_user`, `pressure.cpu_some_avg10`
- Memory: `meminfo.MemAvailable`, `vmstat.pswpout`, `pressure.memory_some_avg10`
- I/O: `diskstats`, `vmstat.nr_dirty`, `pressure.io_some_avg10`
- Network: `net/dev`, `net/snmp.Tcp_RetransSegs`, `net/sockstat`

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

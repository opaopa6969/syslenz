# Linux Resource Model

[日本語版](../ja/concept.resource-model.md)

---

A Linux server has four primary resources. Every performance problem traces back to one or more of them.

```
  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐
  │   CPU   │  │ Memory  │  │   I/O   │  │Network  │
  └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘
       │            │             │              │
  Runqueue      LRU lists    I/O queue     Socket buffers
  Context sw.   Swap/reclaim  Disk queue    TCP stack
  Scheduling    OOM           IOPS limit    Bandwidth
```

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

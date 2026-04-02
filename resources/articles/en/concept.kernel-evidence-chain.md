# Kernel Evidence Chain

[日本語版](../ja/concept.kernel-evidence-chain.md)

---

Diagnosing a Linux system issue is like forensics: you follow the evidence chain from symptom to cause.

**Memory pressure evidence chain:**
```
User report: "app is slow"
  → check: pressure.memory_some_avg10 > 0 (stalls happening)
  → check: vmstat.pswpin rising (swap reads)
  → check: meminfo.MemAvailable < 10% (critically low)
  → check: processes with high RSS (who is using it)
  → root cause: memory leak in service X
```

**I/O latency evidence chain:**
```
User report: "disk writes are slow"
  → check: pressure.io_some_avg10 high
  → check: vmstat.nr_dirty high + nr_writeback low
  → check: dmesg for I/O errors
  → check: diskstats.io_queue_depth_distribution
  → root cause: failing disk / wrong I/O scheduler
```

The key skill: know which metric confirms or refutes each hypothesis.

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

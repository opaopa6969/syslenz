# Capacity vs. Utilization

[日本語版](../ja/concept.capacity-vs-utilization.md)

---

A system at 70% utilization behaves very differently from one at 95% utilization. **Queueing theory** explains why.

As utilization approaches 100%, average queue length grows **exponentially** — not linearly.

```
Utilization: 50% → avg queue length: 1x
Utilization: 80% → avg queue length: 4x  
Utilization: 90% → avg queue length: 9x
Utilization: 95% → avg queue length: 19x
Utilization: 99% → avg queue length: 99x
```

**Practical thresholds:**
- CPU: alert at 80% sustained, page at 90%
- Disk: alert at 70% I/O utilization (especially HDD)
- Network: alert at 60-70% of link capacity

**Why plan for headroom?**
Bursty workloads (cron jobs, batch processing, traffic spikes) need capacity headroom to absorb spikes without spiking latency. Sizing for 95% average utilization means every burst causes a queue avalanche.

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

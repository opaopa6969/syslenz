# Queueing vs. Saturation

[日本語版](../ja/concept.queueing-vs-saturation.md)

---

**Queueing** and **saturation** are related but distinct concepts.

**Saturation** = a resource is at 100% utilization — it cannot accept more work.

**Queueing** = work is waiting because the resource is busy.

A resource can have a queue without being saturated (if requests arrive in bursts). A saturated resource always has a queue.

```
  Disk request rate:    ████████████░░░░  (75% utilized)
  Disk queue depth:     ░░░░░░█████░░░░░  (burst caused queue)
  
  vs.
  
  Disk request rate:    ████████████████  (100% saturated)
  Disk queue depth:     ████████████████  (always queuing)
```

**Why it matters:**
- `diskstats.io_queue_depth_distribution` high during bursts = queueing, possibly not saturated
- `diskstats.io_queue_depth_distribution` consistently high = saturation

**Diagnose with PSI:**
`io_full_avg10` > 0 = saturation (all tasks blocked)
`io_some_avg10` > 0, `io_full_avg10` = 0 = queueing (some tasks affected)

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

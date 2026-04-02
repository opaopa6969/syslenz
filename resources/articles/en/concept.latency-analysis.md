# Latency Analysis

[日本語版](../ja/concept.latency-analysis.md)

---

Latency problems are among the hardest to diagnose because they're often intermittent and multi-causal.

**Sources of latency:**
- **Compute**: too much work → high CPU utilization
- **Queueing**: resource busy → waiting in queue
- **Blocking**: waiting for lock, I/O, network
- **Garbage collection**: JVM/language runtime pauses

**The tail latency problem:**
Average latency can look fine while p99 is terrible. A single slow outlier at p99 means 1 in 100 requests is slow. For a service with 10 dependencies, p99 of the combined call can be much worse.

**Where to look:**
```sh
# CPU queueing
cat /proc/loadavg  # load >> CPUs means queueing

# I/O wait
grep cpu /proc/stat  # column 5 = iowait

# Memory stalls
cat /proc/pressure/memory

# Network latency
ss -tip  # retransmit counts per connection
```

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

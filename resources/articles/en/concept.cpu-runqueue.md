# CPU Run Queue

[日本語版](../ja/concept.cpu-runqueue.md)

---

The **run queue** is the list of processes ready to run but waiting for CPU time. The load average is essentially a smoothed run queue length.

```
  Run Queue:    [process A] [process B] [process C] ...
                     ↓
  CPU:          ← currently running
```

**Key relationship:**
- `loadavg.load_1min` ≈ average run queue length over 1 minute
- `loadavg.running_threads` = instantaneous run queue
- `stat.procs_running` = same as running_threads

**Interpreting run queue:**
- Run queue < # CPUs: healthy (CPUs not fully utilized)
- Run queue ≈ # CPUs: balanced (CPUs busy but not overloaded)
- Run queue >> # CPUs: overloaded (processes waiting for CPU)

**Example:** 4 CPUs, load 6.0 = run queue of 6 = 2 processes always waiting, average wait time of 50% extra per CPU cycle.

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

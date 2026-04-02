# Process Contention

[日本語版](../ja/concept.process-contention.md)

---

Multiple processes competing for the same resource causes contention and unpredictable latency.

**CPU contention:**
- More runnable processes than CPUs → `loadavg > # CPUs`
- Context switches increase → `stat.context_switches` high
- PSI: `pressure.cpu_some_avg10 > 0`

**Memory contention:**
- Multiple processes with large heaps filling RAM
- One process's allocation stalls another's
- PSI: `pressure.memory_some_avg10 > 0`

**I/O contention:**
- Multiple processes reading/writing the same disk
- Queue depth grows → `diskstats.io_queue_depth_distribution`
- PSI: `pressure.io_some_avg10 > 0`

**Lock contention (application level):**
- Shows up as high CPU but low throughput
- Many threads in D state: `stat.procs_blocked`
- Java: JVM thread dump shows blocking on monitor

**Diagnosis principle:** Look for PSI `some` > `full`. Large gap means one process is causing problems for others.

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

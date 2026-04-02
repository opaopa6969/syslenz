# pgmajfault

What is this?
Page faults that needed disk I/O to resolve.

Why it matters
Major faults are much more expensive than minor faults. A steady rise usually means the system is fetching data from disk under pressure.

How to read
- Near-zero on steady workloads is ideal.
- Bursts during startup or cold cache warm-up are normal.
- Continuous growth points to memory pressure or working sets that do not fit.

Next check
Look at diskstats.active_devices and pressure.io_some_avg10.
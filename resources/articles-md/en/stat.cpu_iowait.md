# cpu_iowait

What is this?
Time the CPU spent idle while waiting for storage operations to complete.

Why it matters
iowait is not real CPU work. It is a symptom of blocked storage or filesystem work.

How to read
- Small values can be normal.
- Persistent growth means the system is waiting on disk.
- Pair it with pressure.io_some_avg10 to see whether tasks are stalling.

Next check
Compare with diskstats.active_devices and pressure.io_some_avg10.
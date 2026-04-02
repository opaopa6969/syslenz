# io_full_avg10

What is this?
The fraction of the last 10 seconds where all non-idle tasks were stalled on I/O.

Why it matters
This is a strong sign that storage is the bottleneck, not just a noisy background signal.

How to read
- Any non-zero value is bad.
- Sustained elevation means the host is effectively I/O bound.
- Pair it with writeback and diskstats to separate device saturation from workload bursts.

Next check
Look at nr_writeback and diskstats.active_devices.
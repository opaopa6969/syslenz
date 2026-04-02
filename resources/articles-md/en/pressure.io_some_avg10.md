# io_some_avg10

What is this?
The fraction of the last 10 seconds where at least one task was stalled waiting for I/O.

Why it matters
This is the cleanest signal that storage latency is affecting tasks.

How to read
- Rising dirty pages plus this metric usually means writeback is behind.
- If it stays above zero, tasks are waiting on storage.
- Spikes can come from log bursts or backup windows.

Next check
Compare with diskstats.active_devices and cpu_iowait.
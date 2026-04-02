# cpu_some_avg10

What is this?
The fraction of the last 10 seconds where at least one task was stalled waiting for CPU time.

Why it matters
This is a demand-side metric. It tells you whether users are waiting, not just whether CPUs are busy.

How to read
- Any sustained non-zero value means contention exists.
- Rising loadavg with zero PSI is queueing without user-visible pain yet.
- Rising PSI means the pain is real.

Next check
Compare with loadavg.load_1min and stat.procs_running.
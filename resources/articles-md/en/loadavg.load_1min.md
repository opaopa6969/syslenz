# load_1min

What is this?
The average number of runnable or uninterruptible tasks over the last minute.

Why it matters
Load is demand, not utilization. It tells you whether the scheduler queue is getting crowded.

How to read
- Compare it with CPU count.
- A load near CPU count is often acceptable.
- A load above CPU count plus rising PSI means real contention.

Next check
Look at procs_running and pressure.cpu_some_avg10.
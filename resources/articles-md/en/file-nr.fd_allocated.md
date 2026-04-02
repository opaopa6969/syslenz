# fd_allocated

What is this?
File handles currently allocated by the kernel.

Why it matters
This is the system-wide file descriptor pool. When it gets close to the limit, opens and sockets start failing.

How to read
- Growing over time can indicate a leak.
- Some reuse is normal, so do not panic over short spikes.
- Compare with fd_usage_pct to understand how close you are to exhaustion.

Next check
Look at fd_usage_pct and process_count.
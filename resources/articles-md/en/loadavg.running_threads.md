# running_threads

What is this?
The instantaneous runnable thread count from /proc/loadavg.

Why it matters
This is the live queue depth, not a smoothed average. It moves faster than load_1min.

How to read
- Use it to catch spikes that the average hides.
- If it stays above CPU count, the queue is backing up.
- Pair it with context_switches for scheduler churn.

Next check
Compare with procs_running and process_count.
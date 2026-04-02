# memory_full_avg10

What is this?
The fraction of the last 10 seconds where all non-idle tasks were stalled on memory.

Why it matters
This is severe memory pressure. It means the whole system is being held up by memory reclaim or swap.

How to read
- Any non-zero value is serious.
- If it persists, the machine is effectively degraded.
- Correlate it with pswpout and pgmajfault.

Next check
Look at MemAvailable and vmstat.pswpout.
# context_switches

What is this?
Total voluntary and involuntary context switches since boot.

Why it matters
This is a scheduler churn signal. It helps explain why the CPU may feel busy even when raw utilization is moderate.

How to read
- Steady growth is normal on multi-process systems.
- Sudden jumps can point to lock contention or too many short-lived threads.
- Use it with process_count to see whether the system is thrashing.

Next check
Compare with process_count and loadavg.load_1min.
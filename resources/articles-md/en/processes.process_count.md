# process_count

What is this?
The total number of processes currently visible under /proc.

Why it matters
A growing process count can signal a fork storm, container churn, or a process leak.

How to read
- A sudden jump is worth checking immediately.
- A slow climb over time can be a leak.
- Pair it with context_switches if the host feels busy.

Next check
Compare with file descriptor usage and procs_running.
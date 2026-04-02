# procs_running

What is this?
Processes or threads that are runnable right now.

Why it matters
This is a direct view of scheduler demand. If it stays above available CPU capacity, latency climbs.

How to read
- Small spikes are fine.
- Sustained elevation plus high loadavg means CPU demand is real.
- If pressure.cpu_some_avg10 rises too, tasks are actually waiting.

Next check
Compare with loadavg.running_threads and pressure.cpu_some_avg10.
# max_temp

What is this?
The hottest thermal zone currently reported by the system.

Why it matters
Thermal limits can silently reduce CPU speed before you notice a clear failure.

How to read
- Below 50°C is usually comfortable.
- Around 75°C is warm enough to watch.
- Above 90°C usually means throttling risk or active throttling.

Next check
Compare with cpu_user and cpu_iowait to see whether heat is workload-driven.
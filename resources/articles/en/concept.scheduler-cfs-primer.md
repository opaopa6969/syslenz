# CFS Scheduler Primer

[日本語版](../ja/concept.scheduler-cfs-primer.md)

---

The **Completely Fair Scheduler (CFS)** is Linux's default CPU scheduler. It aims to give each process a proportional share of CPU time based on priority (nice value).

**Key concepts:**
- **vruntime**: virtual runtime — how much CPU time a task has used (weighted by priority)
- **min_vruntime**: the minimum vruntime of any runnable task
- CFS always picks the task with the lowest vruntime (most "behind" on CPU time)

**Nice values:**
- Range: -20 (highest priority) to 19 (lowest)
- Each step = ~10% more/less CPU weight
- Default: 0 (all equal weight)

```sh
# See process priorities
ps -eo pid,ni,comm --sort=ni | head -20

# Set priority
nice -n 10 ./low-priority-job
renice -n -5 -p <pid>   # increase priority (need root for negative)
```

**CFS vs. real-time:**
Real-time tasks (SCHED_FIFO, SCHED_RR) bypass CFS. A runaway RT task can starve CFS tasks. Check: `grep ^cpu /proc/sched_debug | head -5`

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

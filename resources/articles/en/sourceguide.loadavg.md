# sourceguide: loadavg

[日本語版](../ja/sourceguide.loadavg.md)

---

## What is this source?

`/proc/loadavg` exposes the system's load averages — a single line with five fields that the kernel updates every 5 seconds.

```
$ cat /proc/loadavg
2.34 1.87 1.52 3/412 29518
 │    │    │   │  │    │
 │    │    │   │  │    └─ last PID created
 │    │    │   │  └─ total threads in system
 │    │    │   └─ runnable/running threads right now
 │    │    └─ 15-minute load average
 │    └─ 5-minute load average
 └─ 1-minute load average
```

Load average is an **exponentially weighted moving average** of the number of threads in the run queue (running or waiting to run) plus threads in uninterruptible sleep. The kernel samples the scheduler's counters and updates the averages using a decay formula — recent samples count more than older ones.

**The Linux difference:** BSD and Solaris count only runnable threads. Linux also counts threads in `D` state (uninterruptible wait — typically waiting for I/O, kernel locks, or NFS). This means a high load average on Linux can indicate I/O saturation, not just CPU saturation.

```
  Load = (threads running) + (threads waiting to run) + (threads in D state)
                                                         ↑
                                           Linux-specific: I/O waiters included
```

---

## What questions does it answer?

- Is the system more or less busy than its baseline? (compare to CPU count)
- Is pressure building up (1-min > 5-min > 15-min) or recovering (15-min > 5-min > 1-min)?
- Is a sudden spike CPU-driven or I/O-driven? (correlate with `iowait` in `/proc/stat`)
- How many runnable threads exist right now? (the `3/412` field)

---

## Key fields to watch

| Field | What it means |
|---|---|
| 1-minute avg | Immediate pressure. Changes fastest. Noisy on bursty workloads. |
| 5-minute avg | Medium-term trend. Most useful for incident correlation. |
| 15-minute avg | Background trend. Useful as a stable baseline for comparison. |
| Running/total | `3/412` means 3 threads runnable or running, 412 total. Runnable > CPU count = contention. |

**The rule of thumb:** load ÷ number of CPUs. A 4-CPU system with load 4.0 is fully loaded. Load 8.0 on the same system means threads are waiting. Load 1.0 means one CPU is busy on average.

But load average alone cannot tell you *why*. A load of 8.0 could be 8 CPU-hungry threads, or 8 threads blocked on a slow NFS mount.

---

## How to read it directly

```sh
cat /proc/loadavg

# More context with uptime
uptime

# Number of CPUs for comparison
nproc
```

To distinguish CPU load from I/O load, check `/proc/stat` for `iowait`:

```sh
# High iowait% = threads blocked on I/O are inflating load average
grep 'cpu ' /proc/stat
# columns: user nice system idle iowait irq softirq steal guest
```

---

## A real episode

An alert fired at 03:45: load average hit 18.2 on a 16-CPU host. The on-call engineer checked CPU — all cores were under 30% utilization. No obvious compute problem.

The 1-minute average had spiked while the 15-minute average was still around 2.0 — clearly a sudden event, not a slow buildup. `/proc/loadavg`'s runnable field showed `4/890`: only 4 runnable threads despite 890 total. The other 886 were sleeping, but why were so many in D state?

`/proc/stat` showed `iowait` at 68%. `dmesg` revealed EXT4 errors on one disk. A failing drive was causing kernel I/O paths to time out, creating a cascade of threads blocked in uninterruptible wait. The load average had faithfully reported the I/O disaster that CPU metrics completely missed.

---

## See also

- `sourceguide.stat` — CPU time breakdown including iowait and steal; needed to interpret load cause
- `sourceguide.pressure` — PSI gives per-resource stall time, replacing load average as a pressure signal
- `sourceguide.schedstat` — per-CPU runqueue wait times for deeper scheduler analysis
- `sourceguide.processes` — live thread state breakdown to see how many threads are in D state

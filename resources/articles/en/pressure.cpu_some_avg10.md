# cpu_some_avg10

[日本語版](../ja/pressure.cpu_some_avg10.md)

---

## What is it?

`cpu_some_avg10` is part of Linux's **PSI (Pressure Stall Information)**, introduced in kernel 4.20. It measures the percentage of time in the last 10 seconds during which at least one task was waiting for CPU — unable to run because all cores were busy.

The scale is 0–100%. A value of 5.0 means that for 5% of the last 10 seconds, at least one process was ready to run but couldn't get a CPU.

```
  PSI measures real waiting time, not utilization:

  CPU utilization:  ████████████████████░░░░  80% busy
                    ↑ traditional metric — doesn't tell you if tasks are waiting

  cpu_some_avg10:   ████░░░░░░░░░░░░░░░░░░░░  15% of time, someone was waiting
                    ↑ PSI — directly measures "did processes get delayed?"
```

**Why PSI beats load average for CPU pressure:** Load average counts processes in the run queue but doesn't tell you the actual delay they experience. PSI measures the time processes spent stalled. A system with load=2.0 and cpu_some_avg10=40% is starving its processes. A system with load=6.0 and cpu_some_avg10=2% is busy but not hurting anyone.

The `avg10` suffix means this is the 10-second exponential moving average. There are also `avg60` (1-minute) and `avg300` (5-minute) variants.

---

## Why does it matter?

Load average can lie about CPU pressure — especially when I/O-blocked processes inflate it. PSI cannot lie: it is a direct measurement of scheduler delay.

**Where PSI shines:**
- A system at 70% CPU utilization with cpu_some_avg10=0.5%: processes are well-scheduled, no one is starving.
- A system at 50% CPU utilization with cpu_some_avg10=30%: something is serialized — lock contention, pinned to a single CPU, or noisy-neighbor cgroup interference.

**SLO budgeting:** PSI is the metric behind Linux cgroup CPU throttling decisions and Facebook's `oomd`. If you are building latency SLOs, `cpu_some_avg10` tells you directly what fraction of time your processes were delayed — which maps cleanly to p99 latency.

**Noisy-neighbor detection:** In containerized environments, `cpu_some_avg10` at the cgroup level reveals which container is experiencing CPU starvation, even if system-wide CPU% looks fine.

---

## How to read it

```sh
cat /proc/pressure/cpu
# some avg10=2.45 avg60=1.83 avg300=1.20 total=12345678
# full avg10=0.00 avg60=0.00 avg300=0.00 total=0
```

**`some` vs `full`:**
- `some`: at least one task was waiting for CPU (one or more processes delayed)
- `full`: ALL non-idle tasks were waiting for CPU simultaneously (system-wide stall — very serious)

`full` is almost always 0 for CPU (it's common for I/O, not for CPU). Focus on `some` for CPU pressure.

**Practical thresholds:**

| cpu_some_avg10 | Interpretation |
|---|---|
| 0 – 5% | Healthy — minimal CPU contention |
| 5 – 20% | Moderate pressure — monitor for sustained levels |
| 20 – 40% | High — latency impact likely, investigate runqueue |
| > 40% | Severe — processes are starving, immediate attention needed |

These are guidelines. A latency-sensitive real-time service might care about > 5%. A batch job might tolerate > 40%.

---

## A real episode

A Go microservice was showing p99 latency of 180ms against an SLO of 50ms. CPU utilization was 45% — plenty of headroom by conventional wisdom. Load average was 3.2 on an 8-core machine. Dashboards looked yellow, not red.

An engineer ran:
```sh
cat /proc/pressure/cpu
# some avg10=38.2 avg60=35.1 avg300=29.8 total=...
```

CPU `some` pressure at 38%. More than a third of time, processes were waiting for a CPU slot. The culprit: the service was running in a Kubernetes pod with a CPU limit of 2000m (2 cores). The application was spawning goroutines that were fighting over 2 virtual CPUs even though the node had 8 physical cores.

Increasing the CPU limit to 4000m dropped `cpu_some_avg10` to 4.1% and p99 latency to 48ms — under SLO.

**Lesson:** CPU pressure (`cpu_some_avg10`) reveals contention even when utilization looks safe. Check cgroup CPU limits before assuming you have plenty of CPU.

---

## What to do when it's high

**Step 1: Confirm it's sustained, not a spike.**
```sh
# Watch all three averages
watch -n 5 'cat /proc/pressure/cpu'
# If avg10 is high but avg300 is low, it's a recent burst
```

**Step 2: Check if a CPU limit is the constraint.**
```sh
# In a container/cgroup environment
cat /sys/fs/cgroup/cpu/cpu.cfs_quota_us
cat /sys/fs/cgroup/cpu/cpu.cfs_period_us
# quota/period < number of cores = throttling possible
```

**Step 3: Find which processes are consuming CPU.**
```sh
top -H        # thread-level, sorted by CPU
pidstat 1 5   # per-process CPU usage over time
```

**Step 4: Check for CPU pinning or NUMA issues.**
```sh
taskset -p <pid>   # which CPUs is this process allowed to use?
numastat           # NUMA memory access patterns
```

**Step 5: Look at scheduler statistics.**
```sh
# Check runqueue length per CPU
cat /proc/schedstat
sar -q 1 5   # if sysstat installed
```

---

## Common mistakes

**Ignoring PSI because load average looks OK.** Load average is blind to cgroup CPU limits and CPU pinning. PSI is not. If you are running containers, load average will look fine while individual pods starve.

**Confusing `cpu_some_avg10` with CPU utilization.** 38% PSI does not mean 38% CPU utilization. It means 38% of time, at least one process was delayed — even if overall CPU utilization is low.

**Treating any nonzero value as a problem.** On a busy multi-tenant system, `cpu_some_avg10` of 2–5% is normal and harmless. Worry when it climbs above 20% and correlates with user-visible latency.

**Ignoring `avg60` and `avg300`.** A high `avg10` with low `avg300` is a recent burst. A high `avg300` is a chronic problem. Both matter, but differently.

---

## See also

- `pressure.cpu_full_avg10` — all tasks stalled simultaneously (extreme case)
- `pressure.cpu_some_avg60` — 1-minute smoothed view of CPU pressure
- `loadavg.load_1min` — traditional alternative (less precise for cgroup contexts)
- `stat.cpu_iowait` — separates I/O wait from actual CPU pressure
- `schedstat.runqueue_distribution` — per-CPU queue depth over time

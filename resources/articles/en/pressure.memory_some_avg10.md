# memory_some_avg10

[日本語版](../ja/pressure.memory_some_avg10.md)

---

## What is it?

`memory_some_avg10` measures the percentage of time in the last 10 seconds during which at least one task was stalled waiting for memory — unable to proceed because the kernel was reclaiming pages, waiting for swap I/O, or handling a memory allocation that couldn't complete immediately.

It is part of Linux PSI (Pressure Stall Information), available since kernel 4.20, and is read from `/proc/pressure/memory`.

```
  What memory stalls look like:

  Process wants to allocate memory
         |
         v
  Kernel: "I need to reclaim pages first"
         |
         v
  Process BLOCKS — it is counted in memory_some_avg10
         |
         v
  Kernel frees pages (evicts cache, reads from swap)
         |
         v
  Process continues

  memory_some_avg10 = % of time spent in the "blocked" step above
```

**Why this matters more than MemAvailable:** `MemAvailable` tells you how much free memory exists. `memory_some_avg10` tells you whether processes are actually experiencing delays because of memory pressure. A system can have 2GB MemAvailable and still show `memory_some_avg10=25%` if the kernel is constantly reclaiming cache pages to serve a working set that barely fits.

---

## Why does it matter?

Memory pressure is one of the hardest problems to diagnose because it often shows up as mysterious latency without any obvious memory exhaustion. `memory_some_avg10` bridges that gap.

**Scenario A — silent killer:** A service has 60% memory utilization, MemAvailable shows 4GB free, but p99 latency spikes every 30 seconds. Nobody suspects memory. `memory_some_avg10` shows 18%: the working set is just barely fitting in RAM, and the kernel is frantically reclaiming LRU pages every time a new allocation arrives.

**Scenario B — swap blindspot:** A machine has swap enabled. MemAvailable is 512MB. The machine is not swapping visibly. But `memory_some_avg10` is 45%, and `memory_full_avg10` is 12% (all tasks stalled). The system is in deep memory trouble that neither MemAvailable nor swap usage makes obvious.

**Cgroup-level visibility:** In containerized environments, `/sys/fs/cgroup/memory.pressure` shows PSI for individual containers. A pod can be experiencing memory stalls even when the node appears healthy.

---

## How to read it

```sh
cat /proc/pressure/memory
# some avg10=3.21 avg60=2.87 avg300=1.54 total=34567890
# full avg10=0.12 avg60=0.08 avg300=0.03 total=789012
```

**`some` vs `full`:**
- `some` (memory_some_avg10): At least one task was stalled for memory. Most of the time, other tasks could still run. Indicates memory pressure is present.
- `full` (memory_full_avg10): ALL non-idle tasks were stalled simultaneously. CPU was idle while every process waited for memory. This is severe — system is nearly unusable.

A `full` value above 1–2% is a serious alarm. `some` up to 10% can be tolerated on some workloads.

**Practical thresholds:**

| memory_some_avg10 | Interpretation |
|---|---|
| 0 – 2% | Healthy |
| 2 – 10% | Light pressure — worth monitoring |
| 10 – 25% | Moderate — latency impact likely, check MemAvailable and swap |
| > 25% | High — significant stall time, investigate reclaim |
| > 50% | Critical — system is struggling, OOM risk |

---

## A real episode

A machine learning inference service was hit by periodic latency spikes — every 2–3 minutes, p99 latency would jump from 80ms to 600ms for about 10 seconds. The memory utilization alerts were set at 90% usage, and the machine was at 72%. Nothing triggered.

An engineer set up a quick monitoring loop:
```sh
while true; do
  date
  cat /proc/pressure/memory
  sleep 5
done
```

The output showed `memory_some_avg10` oscillating between 0.5% (normal) and 28–35% (spike windows). The spikes aligned perfectly with the latency events.

Further investigation: the inference model had a hot access pattern that cycled through 3.2GB of model weights. The machine had 8GB RAM but was running multiple other services. The effective available RAM for this service was ~2.8GB. Every 2 minutes, the kernel evicted model weights from page cache to serve another service's allocations — and when the model accessed those weights again, it triggered major page faults and memory reclaim.

Fix: pinned the model weights into memory with `mlock()`, reserved memory limit on the competing services. Latency spikes disappeared.

**Lesson:** `memory_some_avg10` can pinpoint memory-driven latency that MemAvailable and utilization percentages completely miss.

---

## What to do when it's high

**Step 1: Check `full` as well as `some`.**
```sh
cat /proc/pressure/memory
```
If `full_avg10` > 2%, treat as critical. If only `some` is high, there is still headroom.

**Step 2: Check MemAvailable and swap.**
```sh
free -h
vmstat 1 5   # si/so columns: swap in/swap out per second
```
High `memory_some_avg10` with low swap activity means reclaim is happening in page cache (evicting file-backed pages). High swap I/O means the system is going beyond cache into swap.

**Step 3: Find who is consuming memory.**
```sh
ps aux --sort=-%mem | head -20
# Or for cgroups:
cat /sys/fs/cgroup/*/memory.current 2>/dev/null | sort -rn | head -10
```

**Step 4: Check for page reclaim activity.**
```sh
vmstat 1 5
# Look at 'si'/'so' (swap) and 'bi'/'bo' (block I/O)
# High 'bi' with no obvious I/O workload = page reclaim reading from swap or disk
```

**Step 5: Consider adding memory or adjusting limits.**
If reclaim is constant and unavoidable, the working set doesn't fit. Add RAM, reduce memory limits on other services, or reduce the working set.

---

## Common mistakes

**Setting memory alerts only on MemFree or utilization percentage.** These metrics tell you how much memory is used, not whether processes are being hurt by it. A busy kernel can have processes stalling at 70% utilization.

**Ignoring `memory_full_avg10`.** The `some` metric is a warning. The `full` metric is an emergency. Not checking `full` means you might miss the system tipping into near-unusable state.

**Attributing latency spikes to network or application bugs without checking PSI.** Memory-driven latency looks identical to network latency from outside the system. Check `memory_some_avg10` before opening a packet capture.

**Thinking swap == memory pressure.** A system with swap disabled can have severe memory pressure (all reclaim from page cache). A system actively swapping might show lower `memory_some_avg10` than one thrashing page cache. PSI measures the actual stall, not the mechanism.

---

## See also

- `memory_full_avg10` — when ALL tasks are stalled (critical threshold)
- `memory_some_avg60` — 1-minute view for trend confirmation
- `meminfo.MemAvailable` — available memory estimate (complement to PSI)
- `vmstat.pgmajfault` — major page faults, often correlated with memory stalls
- `pressure.io_some_avg10` — I/O pressure, often co-occurs with memory pressure during swap

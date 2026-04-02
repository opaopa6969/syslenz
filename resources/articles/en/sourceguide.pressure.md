# sourceguide: pressure

[日本語版](../ja/sourceguide.pressure.md)

---

## What is this source?

`/proc/pressure/` is a directory containing three files — `cpu`, `memory`, and `io` — each exposing **PSI (Pressure Stall Information)** metrics. Available since Linux 4.20.

PSI answers a question that load average cannot: **how much time were processes unable to make progress because a resource was unavailable?** Instead of counting threads in a queue, it measures wall-clock time lost to resource stalls, expressed as a percentage over a sliding window.

```
  /proc/pressure/memory:
  some avg10=0.34 avg60=0.12 avg300=0.05 total=183429
  full avg10=0.00 avg60=0.00 avg300=0.00 total=0

       │     │         │         │          │
       │     │         │         │          └─ microseconds total since boot
       │     │         │         └─ 5-minute sliding window (%)
       │     │         └─ 1-minute sliding window (%)
       │     └─ 10-second sliding window (%)
       └─ "some" line: at least one task was stalled
          "full" line: ALL runnable tasks were stalled simultaneously
```

**Some vs Full:**
- `some`: at least one process was waiting for this resource. The system was still making progress, but not everyone could.
- `full`: every runnable process was blocked on this resource simultaneously. No forward progress anywhere. This is severe.

---

## What questions does it answer?

- Is the system actually stalling on CPU, memory, or I/O — not just "busy"? (`some` lines)
- Has the system reached a point where *no work is getting done* on a resource? (`full` lines)
- Is pressure improving or worsening over the last 10 seconds vs last 5 minutes? (compare avg10 to avg300)
- Is load average high because of CPU contention or I/O blocking? (compare `cpu` and `io` PSI)

---

## Key fields to watch

| File | Metric | Alert signal |
|---|---|---|
| `cpu` | `some avg10` | CPU scheduling delay. Consistent >10% means CPU oversubscription. |
| `memory` | `some avg10` | Processes waiting for memory. Even 1–2% sustained is a warning. |
| `memory` | `full avg10` | Every task blocked on memory. Any nonzero value is serious. |
| `io` | `some avg10` | I/O wait. High on write-heavy workloads; correlate with diskstats. |
| `io` | `full avg10` | Complete I/O stall. Nonzero means the storage layer is the bottleneck. |

**PSI is better than load average for alerting.** Load average conflates CPU and I/O pressure into one number. PSI separates them by resource and measures actual stall time rather than queue length.

---

## How to read it directly

```sh
cat /proc/pressure/cpu
cat /proc/pressure/memory
cat /proc/pressure/io
```

Watch all three together:

```sh
watch -n 2 'echo "=== CPU ===" && cat /proc/pressure/cpu && echo "=== Memory ===" && cat /proc/pressure/memory && echo "=== I/O ===" && cat /proc/pressure/io'
```

PSI thresholds can trigger kernel events via a file descriptor notification mechanism — useful for cgroup-level alerting:

```sh
# Alert when memory pressure some avg10 exceeds 5% for 500ms
fd=$(open /proc/pressure/memory rw)
echo "some 5000 500000" > /proc/pressure/memory
```

---

## A real episode

A Postgres database host had load average of 4.2 on a 4-CPU machine — textbook "fully loaded." The on-call team was about to add more CPU capacity. Before provisioning, someone checked PSI:

```
cpu:    some avg10=1.2  avg60=0.8   (negligible CPU scheduling delay)
memory: some avg10=18.4 avg60=14.1  (significant memory stall!)
io:     some avg10=22.1 avg60=19.8  (I/O also stalling)
```

The load average of 4.2 was almost entirely I/O and memory stall, not CPU saturation. The `D` state threads inflating load average were blocked on memory reclaim and disk writes. Adding CPUs would have done nothing.

The actual fix: the Postgres `shared_buffers` setting was too small, causing constant page eviction and re-read from disk. Increasing it from 256 MB to 4 GB cut `memory some` from 18% to 0.3% and `io some` from 22% to 4%.

---

## See also

- `sourceguide.meminfo` — memory state counters that explain why memory pressure is occurring
- `sourceguide.vmstat` — page reclaim and swap activity that drives memory PSI
- `sourceguide.loadavg` — the older signal that PSI is designed to improve upon
- `sourceguide.diskstats` — per-device I/O metrics that correlate with io PSI

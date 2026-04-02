# io_some_avg10

[日本語版](../ja/pressure.io_some_avg10.md)

---

## What is it?

`io_some_avg10` measures the percentage of time in the last 10 seconds during which at least one task was stalled waiting for I/O — blocked on a read or write that hadn't completed yet. It is part of Linux PSI (Pressure Stall Information), available since kernel 4.20, and is read from `/proc/pressure/io`.

Unlike CPU pressure, I/O pressure has a meaningful `full` metric: when `io_full_avg10` is nonzero, every task on the system was waiting for I/O simultaneously — which means the CPU was completely idle while the storage device was the only thing working.

```
  io_some vs io_full:

  Task A: ████░░░░████████░░░░████  (reading from disk)
  Task B: ████████░░░░░░░░████████  (also reading from disk)
  Task C: ██████████████████░░░░░░  (running on CPU)

  some: any window where A or B is blocked  → most of the time
  full: window where A, B, AND C are ALL blocked simultaneously → io_full

  io_some_avg10 catches: "someone is waiting for I/O"
  io_full_avg10 catches: "everyone is waiting, nothing is making progress"
```

---

## Why does it matter?

I/O pressure is the hidden tax on almost every modern system. Storage is hundreds to thousands of times slower than RAM. When processes start serializing on disk I/O, latency compounds quickly — especially if writes are synchronous or if multiple processes share the same I/O queue.

**What `io_some_avg10` detects that other metrics miss:**

- `iostat %util`: Shows how busy the device is, but 100% utilization on an NVMe doesn't mean processes are stalling — the queue depth might still be serving requests fast.
- `iowait` in `vmstat`: Shows CPU idle time during I/O, but is per-CPU and misleading in multi-core systems.
- `io_some_avg10`: Directly measures "did processes actually wait?" Not a proxy. The answer is yes or no, measured in time.

**The full/some split reveals severity:** `io_some_avg10 = 20%` with `io_full_avg10 = 0%` means I/O pressure exists but the system is partially making progress. `io_some_avg10 = 20%` with `io_full_avg10 = 15%` means the system is frequently in a state where nothing is making progress — that's a storage bottleneck worth acting on immediately.

---

## How to read it

```sh
cat /proc/pressure/io
# some avg10=8.42 avg60=6.71 avg300=3.14 total=123456789
# full avg10=1.23 avg60=0.98 avg300=0.41 total=23456789
```

**Practical thresholds:**

| io_some_avg10 | io_full_avg10 | Interpretation |
|---|---|---|
| 0 – 5% | 0% | Normal I/O activity |
| 5 – 20% | 0% | Moderate I/O pressure — monitor |
| 20 – 50% | 0 – 2% | High pressure — investigate device and workload |
| > 50% | 0 – 5% | Severe — I/O is a bottleneck |
| Any | > 5% | Critical — storage is blocking entire system progress |

---

## A real episode

A PostgreSQL server was experiencing intermittent query timeouts — not consistent, not correlated with query complexity, just random 10–30 second timeouts that appeared a few times per hour. The database team checked connection pool exhaustion, lock waits, query plans. Nothing obvious.

An ops engineer checked PSI:
```sh
watch -n 2 'cat /proc/pressure/io'
```

```
some avg10=52.3 avg60=41.8 avg300=22.6 total=...
full avg10=18.7 avg60=14.2 avg300=7.1  total=...
```

`io_full_avg10` at 18.7% — nearly one in five seconds, the entire system was idle waiting for I/O. That's why queries timed out: even simple lookups stalled when the storage queue was saturated.

```sh
iostat -x 1 5
```

The disk showed `%util=99%`, `await=280ms` (average wait time per I/O operation). The PostgreSQL data directory was on the same disk as the system logs. A logging-heavy application was writing gigabytes of logs per hour, saturating the HDD.

Fix: Moved logs to a separate disk. `io_full_avg10` dropped to 0.2%, query timeouts disappeared.

**Lesson:** High `io_full_avg10` is one of the clearest signals that your storage is your bottleneck. Don't wait for application-level timeouts to tell you what PSI already knows.

---

## What to do when it's high

**Step 1: Check `full` alongside `some`.**
```sh
cat /proc/pressure/io
```
If `full_avg10` > 5%, prioritize immediately — the storage device is likely saturated.

**Step 2: Identify the saturated device.**
```sh
iostat -x 1 5
# Look for: %util near 100%, await > 20ms (HDD) or > 1ms (NVMe)
# The device with highest await and util is the bottleneck
```

**Step 3: Find which processes are causing I/O.**
```sh
iotop -b -n 3 -d 5  # top I/O consumers over 15 seconds
pidstat -d 1 5      # per-process I/O statistics
```

**Step 4: Check for synchronous writes.**
Synchronous writes (`O_SYNC`, `fsync()`, `fdatasync()`) are a common cause of I/O stalls. Check if the application forces sync:
```sh
# Track fsync calls per process
strace -e trace=fsync,fdatasync -p <pid> 2>&1 | head -20
```

**Step 5: Consider the write path.**
```sh
vmstat 1 5
# Look at bo (blocks written per second) vs bi (blocks read)
# If bo is consistently high, check vm.dirty_ratio thresholds
cat /proc/vmstat | grep -E 'nr_dirty|nr_writeback'
```

---

## Common mistakes

**Using `%iowait` from top/vmstat as a substitute for PSI.** `%iowait` is a per-CPU metric that is misleading on multi-core systems. On a 16-core machine, one CPU at 100% iowait only shows as 6.25% in the average. `io_some_avg10` has no such dilution.

**Ignoring `io_full_avg10`.** Many engineers only look at `io_some_avg10`. The `full` metric is where the real damage shows. A full stall means the storage device controls all system throughput — the application cannot overlap CPU work with I/O at all.

**Blaming the application when storage is the issue.** When `io_some_avg10` is high, application-level timeouts are a symptom, not the cause. Fix the storage bottleneck first; application tuning won't help if the device is saturated.

**Treating NVMe the same as HDD for thresholds.** NVMe can sustain much higher `io_some_avg10` before latency becomes unacceptable because its queue depth is larger. A spinning HDD at 20% io_some is much more dangerous than an NVMe at 20%.

**Not checking at the cgroup level.** In containers, `/sys/fs/cgroup/io.pressure` shows per-container I/O pressure. A container might show high `io_some_avg10` while the system-level value looks fine, if I/O limits are set on the cgroup.

---

## See also

- `pressure.io_full_avg10` — all tasks stalled simultaneously on I/O (critical)
- `pressure.io_some_avg60` — 1-minute view for trend confirmation
- `diskstats.read_wait_distribution` — per-device I/O latency distribution
- `vmstat.nr_dirty` — dirty page accumulation that causes writeback I/O storms
- `pressure.memory_some_avg10` — memory pressure, often co-occurs during swap-induced I/O

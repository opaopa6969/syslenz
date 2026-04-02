# load_1min

[日本語版](../ja/loadavg.load_1min.md)

---

## What is it?

`load_1min` is the 1-minute exponential moving average of the number of processes in the kernel's run queue — either actively running on a CPU or blocked waiting for one. It is read from the first field of `/proc/loadavg`.

Think of it as a headcount of everyone trying to use the CPU right now. On a 4-core machine, a load of 4.0 means every core is fully occupied. A load of 8.0 means twice as many processes are competing as there are cores to serve them.

```
  Load average = running + waiting-for-CPU + waiting-for-I/O (Linux!)
                 ^^^^^^^^   ^^^^^^^^^^^^^^^   ^^^^^^^^^^^^^^^^^^
                 on CPU      in runqueue       in D state (disk/net)

  4-core machine:
    load 2.0  →  50% capacity used       (comfortable)
    load 4.0  →  100% capacity used      (fully loaded)
    load 8.0  →  200% capacity used      (overloaded — queue building)
```

**Linux-specific:** Unlike traditional Unix, Linux counts processes blocked in uninterruptible I/O wait (D state) toward the load average. This is crucial — a load of 40 with idle CPUs usually means disk or NFS is the real culprit, not compute.

---

## Why does it matter?

Load average is the most visible system health signal. It is what `uptime` shows first, what `top` puts in the header, and what most monitoring alerts fire on.

But it lies — or rather, it tells only part of the story. A sustained high load means the system cannot keep up with demand. A sudden spike followed by rapid recovery is usually harmless. The 1-minute value tells you what is happening right now, but without knowing your CPU count and whether the load is CPU-bound or I/O-bound, you cannot act on it.

**Real failure scenario:** A web server shows load=20 on an 8-core box. On-call engineer panics and starts killing processes. The processes were actually blocked on a slow NFS mount — killing them restarts the services and floods the mount harder. Load goes to 60. The real fix was to remount with `soft,timeo=` options.

---

## How to read it

```sh
uptime
# 10:42:33 up 3 days,  2:17,  2 users,  load average: 3.45, 2.91, 2.58

cat /proc/loadavg
# 3.45 2.91 2.58 4/312 18294
# fields: 1min  5min  15min  running/total  last_pid

nproc   # or: grep -c ^processor /proc/cpuinfo
# 8
```

| load / nproc | Meaning |
|---|---|
| < 0.7 | Comfortable — headroom available |
| 0.7 – 1.0 | Approaching saturation on single-core, fine on multi-core |
| ~1.0 × nproc | Fully loaded, no headroom |
| > 1.5 × nproc | Overloaded — requests queuing |
| > 5 × nproc | Serious problem — check for I/O stall first |

**Pair with CPU iowait before concluding:**
```sh
# Check if load is CPU-driven or I/O-driven
vmstat 1 5
# Look at 'wa' column (I/O wait %) and 'r' column (runqueue length)
# High wa + high load = I/O problem
# High r + high load = CPU problem
```

---

## A real episode

A media transcoding service on a 16-core machine hit load=40 during a batch job window. Alerts fired, Slack lit up, and the on-call team prepared to scale out. Someone ran `vmstat 1` and saw:

```
procs -----------memory---------- ---swap-- -----io---- -system-- ------cpu-----
 r  b   swpd   free   buff  cache   si   so    bi    bo   in   cs us sy id wa st
 2 38   ....   ....   ....  .....    0    0  8420   140  ...  ... 3  1 91  5  0
```

The `r` column showed only 2 runnable processes. The `b` column showed 38 blocked. CPUs were 91% idle. The batch job was spawning 40 ffmpeg processes, each waiting for read from a slow spinning disk RAID. Adding a faster NVMe scratch volume dropped load from 40 to 3 in under two minutes.

**Lesson:** When load is high but CPUs are idle, stop looking at the CPU and start looking at storage.

---

## What to do when it's high

**Step 1: Get your CPU count.**
```sh
nproc
```
Scale everything against this number. Load of 10 on a 2-core box is a disaster. Load of 10 on a 32-core box is unremarkable.

**Step 2: Is it CPU or I/O?**
```sh
vmstat 1 3
# 'wa' > 20% with high load → I/O bottleneck
# 'r'  > nproc with high load → CPU bottleneck
```

**Step 3: Find the blocked processes (I/O path).**
```sh
ps aux --sort=-pcpu | head -20
# Look for processes in D state (uninterruptible sleep)
ps aux | awk '$8=="D"' | head -20
```

**Step 4: If I/O — check which device.**
```sh
iostat -x 1 5
# Look for %util near 100% and high await values
```

**Step 5: If CPU — find what's consuming it.**
```sh
top -H   # thread-level view
perf top  # if available
```

---

## Common mistakes

**Panicking at a high number without checking nproc.** Load average is meaningless without knowing core count. Always divide by `nproc` before reacting.

**Assuming high load means CPU is the bottleneck.** In Linux, I/O-blocked processes count toward load. Always check `vmstat`'s `wa` and `b` columns first.

**Treating load_1min as a stable signal.** The 1-minute value fluctuates. A single reading of 8.0 might be the tail of a 10-second burst. Look at load_5min and load_15min for trend.

**Killing processes to reduce load.** If processes are blocked on I/O, killing them often causes service restarts that increase load. Fix the I/O source first.

**Setting alerts on absolute load values.** An alert threshold of `load > 10` is meaningless across machines with different core counts. Alert on `load / nproc > 1.5` instead.

---

## See also

- `loadavg.load_5min` — 5-minute average for trend direction
- `loadavg.load_15min` — 15-minute average for sustained load
- `loadavg.running_threads` — current runnable count (instantaneous)
- `stat.cpu_iowait` — confirms whether load is I/O-driven
- `pressure.cpu_some_avg10` — more precise CPU contention signal (PSI)
- `pressure.io_some_avg10` — confirms I/O stall contribution to load

# pgmajfault

[日本語版](../ja/vmstat.pgmajfault.md)

---

## What is it?

`pgmajfault` counts **major page faults** — events where a process accesses a virtual memory address and the required page is **not in RAM**. The kernel must fetch the page from disk (either from swap space or from a file) before the process can continue.

Major faults are fundamentally different from minor faults. A minor fault takes microseconds (just a page table update). A major fault takes milliseconds — it blocks the process until the disk read completes.

```
  Minor fault (fast):
  Process → address not mapped → page IS in RAM → map it → continue
                                                    ~1 µs

  Major fault (slow):
  Process → address not mapped → page NOT in RAM → read from disk
                                                     → map it → continue
                                                     ~5–50 ms (HDD)
                                                     ~0.1–1 ms (SSD)
```

The most common source of major faults on production servers:
1. **Swap reads** — memory was paged out under pressure, now being read back
2. **Cold file reads** — a file that hasn't been accessed recently, not in page cache
3. **After OOM activity** — surviving processes whose pages were evicted

---

## Why does it matter?

Even a small number of major faults can cause visible latency spikes. A process handling requests that suddenly takes 10ms+ for page reads looks like a slow application, not a memory problem.

**The key misread:** "The application is slow" when the real cause is "the application's memory was swapped out last night, and now every access is a disk read."

Real-world consequence: a rate of 100 major faults/second on an HDD system means at least 500ms of blocking per second just in page reads — spread unevenly across processes, making latency spiky and hard to diagnose.

```
  Major fault impact on latency:

  Normal request: [ app logic 2ms ]
  During major faults: [ wait for page 30ms ][ app logic 2ms ]

  From the outside: "why is this request sometimes 15x slower?"
```

---

## How to read it

```sh
# System-wide rate (column 11 in vmstat output)
vmstat 1 5
# Header: r b swpd free buff cache si so bi bo in cs us sy id wa st
# si (swap in) and bi (blocks in) also show if pages are coming from swap/disk

# Per-process rate
pidstat -f 1 5
# Watch: majflt/s column

# Historical counts for a process
cat /proc/<PID>/stat | awk '{print "minor:", $10, "major:", $12}'
```

| `pgmajfault` rate (per second) | Interpretation |
|-------------------------------|----------------|
| 0–2/s | Normal background activity |
| 2–20/s | Occasional; monitor with `vmstat si` |
| 20–100/s | Noticeable latency impact; investigate memory |
| 100+/s | Severe; system thrashing or heavy cold reads |

**Correlate with swap:**
```sh
# Is swap being read? (si = swap in pages/second)
vmstat 1 | awk '{print "swap_in:", $7, "major_faults:", $11}'
```

If `vmstat si` (swap-in) is high and `pgmajfault` is high → the system is reading from swap. This is almost always the critical path to fix.

---

## A real episode

A Java application at an e-commerce company processed morning traffic fine but was consistently slow for the first 10–15 minutes after 08:00. Engineers assumed "JVM warmup" and ignored it for months.

A new engineer looked at `pgmajfault` at 08:05:
```sh
pidstat -f 1 | grep java
# PID  minflt/s  majflt/s
# 2381   8,200    1,847
```

1,847 major faults per second. The JVM was reading pages from swap as fast as it could.

Investigation: a nightly OOM event had been silently killing a low-priority background process and causing the JVM's heap pages to be partially swapped out around 04:00. By morning, the heap was in swap. The JVM's first 15 minutes of traffic was largely disk reads.

Fixes applied:
1. Added swap memory to reduce OOM pressure
2. Added `mlockall()` JVM option to prevent heap pages from being swapped
3. Fixed the root cause: the background process had a memory leak

After the fix, morning major faults were 0.3/s. The "JVM warmup" latency disappeared entirely.

**Lesson:** If your application is consistently slow after a quiet period (nights, weekends), check `pgmajfault`. "Warmup" is often just "memory was swapped out."

---

## What to do when it's high

**Step 1: Determine if swap is involved.**
```sh
vmstat 1 5
# si (column 7) = swap pages read per second
# If si > 0 and rising, this is a swap-thrashing issue
```

**Step 2: Check swap usage.**
```sh
free -h
swapon -s
cat /proc/swaps
```

**Step 3: Identify which process is causing faults.**
```sh
pidstat -f 1 5
# Sort by majflt/s to find the culprit
```

**Step 4: Address the root cause.**

```sh
# Option A: Process is using too much memory — find and fix the leak
# Check RSS (resident set size) trend
ps -o pid,rss,vsz,comm -p <PID>

# Option B: OOM is evicting pages — add swap or reduce memory pressure
vmstat 1 | grep -E "^[0-9]" | awk '{print "free:", $4, "swap_used:", $3}'

# Option C: Lock critical process pages in RAM
# (in application code)
# mlockall(MCL_CURRENT | MCL_FUTURE)  # prevents swapping

# Option D: Prioritize process to avoid OOM eviction
echo -100 > /proc/<PID>/oom_score_adj
```

**Step 5: For file-based major faults (cold reads, not swap):**
- Increase available RAM to grow the page cache
- Warm up the cache deliberately on startup
- Use `posix_fadvise(FADV_WILLNEED)` to prefetch files the application will need

---

## Common mistakes

**Blaming "JVM warmup" or "application startup."** If a Java/Python/Ruby application is slow for 5–20 minutes after low traffic periods, check `pgmajfault` before assuming it's a runtime issue. It's often swap.

**Seeing `pgmajfault = 0` and assuming no memory pressure.** If pages are being read from the page cache (cold but not swapped), they show up as major faults only when they need to be brought in. After a cache is warm, major faults drop. `pgmajfault` dropping to 0 after startup doesn't mean there was no problem at startup.

**Fixing major faults without understanding whether it's swap or cold-read.** These require different solutions. Swap: reduce memory pressure or add swap. Cold reads: warm the cache or add RAM.

**Not checking OOM history.** If you see major faults from swap, check `dmesg` or `journalctl` for OOM kill events. The process being slow is often not the process that was killed.

---

## See also

- `vmstat.pgfault` — total page faults including the cheaper minor faults
- `vmstat.pswpin` — pages swapped in from disk; direct measure of swap reads
- `vmstat.pswpout` — pages swapped out; if high, swap pressure is building
- `pressure/memory_some_avg10` — PSI memory pressure; confirms tasks are stalling
- `stat.cpu_iowait` — if high alongside pgmajfault, disk is the bottleneck

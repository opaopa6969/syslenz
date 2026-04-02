# pgfault

[日本語版](../ja/vmstat.pgfault.md)

---

## What is it?

`pgfault` counts the total number of **page faults** — events where a process accesses a virtual memory address that doesn't have a physical memory page mapped to it yet. The kernel intercepts this, maps the page, and lets the process continue.

Almost all page faults are **minor faults**: the page already exists in RAM (perhaps in the page cache or shared memory), and the kernel just needs to update the process's page table to point to it. This is fast — microseconds.

The count in `/proc/vmstat` is cumulative since boot. What matters is the **rate** of change, not the absolute value.

```
  Process accesses address 0xABCD1234
           │
           v
  ┌──────────────────┐
  │  Page table: ?   │  <- no mapping yet → PAGE FAULT
  └──────────────────┘
           │
           ├─ Page in RAM (page cache, shared)?  → MINOR fault: just map it
           │
           └─ Page NOT in RAM?                   → MAJOR fault: read from disk
```

Think of minor faults like finding a book already on your desk — you just need to pick it up. Major faults are like having to go to the library to get the book first.

---

## Why does it matter?

A moderate rate of `pgfault` (minor faults) is completely normal. Every time a process allocates memory and first touches a page, or first accesses a shared library, or inherits a copy-on-write page after a fork, it generates a minor fault. Tens of thousands per second is not unusual on a busy system.

**When to pay attention:**

- **Rate spikes after a deployment** — new code may have different memory access patterns, causing more frequent minor faults. Usually not a problem, but worth noting.
- **Sustained high rate with high CPU system time** — minor fault handling runs in kernel context; very high fault rates can add up to meaningful kernel CPU overhead.
- **pgfault rising without corresponding work increase** — may indicate memory fragmentation, inefficient memory allocation, or a leak causing the process to constantly allocate and map new pages.

The sister metric `pgmajfault` (major faults) is where serious performance problems usually live. Always check both together.

---

## How to read it

```sh
# Rate of page faults per second (system-wide)
vmstat 1 | awk '{print $10, $11}'   # columns: minor, major faults

# Per-process page fault rate
pidstat -f 1 5
# or
/usr/bin/time -v <command>   # shows major+minor faults after completion
```

| `pgfault` rate (per second) | Interpretation |
|-----------------------------|----------------|
| 0–10,000/s | Normal; light workload |
| 10,000–100,000/s | Normal under active workload |
| 100,000–500,000/s | Heavy; check for memory pressure |
| 500,000+/s | Very high; investigate process memory patterns |

**Compare with `pgmajfault`:**
- If `pgfault` is 50,000/s and `pgmajfault` is 5/s → normal minor fault activity
- If `pgfault` is 50,000/s and `pgmajfault` is 2,000/s → significant disk I/O from page faults

---

## A real episode

A search service was running a nightly re-indexing job. Engineers noticed `cpu_system` spiked to 40% during the job — much higher than the 5% seen during normal queries. Application CPU (`cpu_user`) was normal. What was the kernel doing?

`pidstat -f 1` during the job showed:
```
PID  minflt/s  majflt/s
1842  87,000      1.2
```

87,000 minor faults per second from the indexer. The job allocated large buffers repeatedly, used them briefly, freed them, and reallocated — causing a constant cycle of new page mappings. The fix was to pre-allocate a pool at startup and reuse it, reducing minor faults to 3,000/s and dropping `cpu_system` to 8%.

**Lesson:** Minor faults are cheap individually but expensive at scale. A process generating 80,000 minor faults per second is making 80,000 kernel trap-and-return cycles per second. On a latency-sensitive system, this adds up.

---

## What to do when the rate is high

**Step 1: Identify which process is generating faults.**
```sh
pidstat -f 1 5
# minflt/s = minor fault rate, majflt/s = major fault rate
```

**Step 2: Is it causing a real problem?**
Check `cpu_system` — if kernel CPU is elevated, fault handling overhead may be the cause. Check application latency.

**Step 3: If major faults are also high, that's the priority.**
See `vmstat.pgmajfault` — major faults require disk I/O and are orders of magnitude more expensive.

**Step 4: Reduce minor fault rate.**
```sh
# Use huge pages to reduce page table overhead (if app supports it)
echo madvise > /sys/kernel/mm/transparent_hugepage/enabled

# Check if process is using mmap heavily and repeatedly
strace -e mmap,mprotect -p <PID> 2>&1 | head -30
```

---

## Common mistakes

**Confusing cumulative values with rates.** `/proc/vmstat` shows totals since boot. A high number after weeks of uptime is expected. Always look at the **rate of change** (use `vmstat 1` or `pidstat`).

**Worrying about minor faults before checking major faults.** Major faults involve disk I/O and are ~1000x more expensive than minor faults. If `pgmajfault` is rising, that's the urgent problem.

**Not correlating with cpu_system.** Page fault handling runs in kernel space. Very high minor fault rates will show up as elevated `cpu_system`. If `cpu_system` is high and you don't see obvious syscall activity, check fault rates.

---

## See also

- `vmstat.pgmajfault` — major page faults; requires reading from disk; the expensive version
- `vmstat.pswpin` — pages read from swap; a primary source of major faults
- `stat.cpu_system` — kernel CPU time; elevated by high fault rates
- `pressure/memory_some_avg10` — whether memory pressure is causing task delays

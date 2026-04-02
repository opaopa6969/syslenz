# cpu_iowait

[日本語版](../ja/stat.cpu_iowait.md)

---

## What is it?

`cpu_iowait` is the percentage of time the CPU was **idle and waiting for I/O to complete**. This is one of the most misread metrics in Linux performance monitoring.

The critical distinction: iowait is not CPU time. It is *idle* time — time when the CPU had nothing to run, specifically because all runnable tasks were blocked waiting for disk or network I/O. The CPU is doing nothing productive, but it can't run anything else either because all pending work is stuck waiting.

```
  What iowait actually is:

  Process A: [running][running][ BLOCKED on disk read ][running]
  Process B: [running][   BLOCKED on disk read         ][run]
  CPU:       [busy   ][busy   ][ idle — iowait         ][busy]

  The CPU is free, but there's nothing runnable.
  This idle time is counted as iowait.
```

Think of it like an assembly line worker waiting for parts to arrive. The worker (CPU) isn't doing anything — but it's not because they're lazy. The bottleneck is the parts supplier (storage).

---

## Why does it matter?

**iowait high = storage is slow, not CPU.** This is the core insight. When iowait is high, adding faster CPUs or more CPU cores does nothing. The system is limited by how fast data can move between storage and RAM.

Common scenarios where iowait spikes:
- A database query hits an uncached table — must read cold data from disk
- Log rotation or backup jobs writing large files
- Application reading config files on every request (should be cached)
- RAID rebuild or filesystem check running in the background
- Swap is active and pages are being read back from swap space

**Why does iowait sometimes look like "CPU is busy"?** Because tools like `top` count iowait toward "CPU time" in their summary. If `top` shows 90% CPU, that might be 10% user + 80% iowait — the CPU is mostly sitting idle waiting for disk. This is completely different from 80% user + 10% iowait.

---

## How to read it

```sh
# See iowait in context
mpstat -P ALL 1 5

# Identify which disks are busy
iostat -x 1

# Which processes are doing I/O?
iotop -a
```

| `cpu_iowait` value | Interpretation |
|-------------------|----------------|
| 0–5% | Negligible; I/O is fast or light |
| 5–20% | Moderate; worth watching with iostat |
| 20–40% | High; storage may be bottleneck |
| 40%+ | Severe; system is likely I/O-bound |

**The iowait + iostat cross-check:**

```sh
# Run these together to confirm I/O is the cause
iostat -x 1 | grep -v '^$'
# Look for: %util approaching 100, await > 10ms on HDDs, > 1ms on SSDs
```

| Situation | iowait | iostat %util | What it means |
|-----------|--------|--------------|---------------|
| Normal | Low | Low | Healthy |
| Storage bottleneck | High | ~100% | Disk saturated |
| I/O burst (SSD) | Spike then drops | Spike then drops | Transient; monitor |
| Mystery high iowait | High | Low | Check NFS, network storage |

---

## A real episode

An e-commerce platform had been running smoothly for six months. Then the team received alerts: "CPU usage at 92%." Engineers started profiling application code, tuning JVM settings, and discussing whether to upgrade the instance type.

Three hours into the investigation, someone ran `mpstat -P ALL 1` and noticed: `%usr` was 11%, `%sys` was 1%, but `%iowait` was 80%. The total showed 92%, which is why monitoring had fired the "CPU" alert.

A quick `iostat -x 1` revealed one disk at 100% utilization. The culprit: a scheduled antivirus scan that had started at 02:00 and was grinding through the entire `/var` directory, which held the application's session files.

Fix: reschedule the scan to 05:00 (after traffic drops) and exclude session file directories. `cpu_iowait` dropped to 2%. No code changes. No hardware upgrades.

**Lesson:** When someone says "CPU is at 90%," always ask: "How much of that is iowait?" The answer changes everything.

---

## What to do when it's high

**Step 1: Confirm I/O is the actual cause.**
```sh
iostat -x 1 5
# Look at: %util (saturation), await (latency ms), r/s and w/s (throughput)
```

**Step 2: Identify which device and which process.**
```sh
# Which device is saturated?
iostat -x 1 | sort -k 14 -rn | head -5

# Which process is doing the I/O?
iotop -aoP
```

**Step 3: Categorize the I/O.**
- Is it read or write? (`r/s` vs `w/s` in iostat)
- Is it sequential or random? (check `rrqm/s` and `wrqm/s` merge rates)
- Is it from application code, a background job, or the OS itself?

**Step 4: Take action based on the cause.**

```sh
# If a specific process is the culprit, limit its I/O priority
ionice -c 3 -p <PID>   # idle priority: yield to others

# Check if swap is contributing (pswpin/pswpout in vmstat)
vmstat 1 5

# Check for stuck I/O at the kernel level
cat /proc/diskstats
dmesg | grep -i "error\|timeout\|reset" | tail -20
```

**Step 5: Hardware considerations.**
- HDDs: await > 20ms is a problem; consider SSD migration
- SSDs: if %util is consistently > 80%, you need more IOPS (add drives, use NVMe)
- Network storage (NFS, iSCSI): check network latency, not disk health

---

## Common mistakes

**Calling it a "CPU problem."** iowait is CPU idle time. The CPU is fine. The storage is not. Don't tune CPU settings when iowait is the issue.

**Ignoring it because "the CPU is actually idle."** True — but your application is stalled waiting for that I/O. Your users are experiencing slowness even though the CPU is free.

**Not checking network storage.** High iowait with low local disk utilization often means NFS or iSCSI is slow. `iostat` won't show NFS — you need `nfsstat` or `mountstats`.

**Panic-killing I/O-heavy processes.** If a backup job is causing iowait, killing it mid-run may leave data in an inconsistent state. Use `ionice` to throttle it instead.

---

## See also

- `stat.cpu_user` — user-space CPU time; high user + high iowait is the classic false-alarm pattern
- `vmstat.pgpgin` / `vmstat.pgpgout` — page-level I/O counters; rising means swap or file I/O
- `vmstat.pswpin` / `vmstat.pswpout` — swap I/O; if these are rising, iowait may be swap-driven
- `diskstats` — per-device I/O statistics for pinpointing the saturated device
- `pressure/io_some_avg10` — kernel PSI; confirms whether I/O pressure is actually stalling tasks

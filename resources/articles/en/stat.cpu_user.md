# cpu_user

[日本語版](../ja/stat.cpu_user.md)

---

## What is it?

`cpu_user` is the percentage of CPU time spent executing code in **user space** — your applications, databases, language runtimes, and any other non-kernel code. The Linux kernel tracks this per-CPU in `/proc/stat` as a tick counter, and syslenz exposes it as a percentage over the sampling interval.

Think of it like a restaurant kitchen. The CPU is the chef. `cpu_user` measures how much time the chef spends actually cooking food (your app). `cpu_system` measures time spent doing admin work like restocking supplies (kernel calls). `cpu_iowait` is the chef standing idle waiting for a delivery.

```
  CPU time budget (100%)
  ┌──────────────────────────────┐
  │ user   ████████████ 60%      │  <- your app runs here
  │ system ████ 20%              │  <- kernel runs on behalf of your app
  │ iowait ██ 10%                │  <- CPU idle, waiting for I/O
  │ idle   ██ 10%                │  <- nothing to do
  └──────────────────────────────┘
```

---

## Why does it matter?

High `cpu_user` means your application is doing real computation. This is often **healthy** — it means the CPU is productive. However, it can also signal runaway processes, inefficient algorithms, or a workload that has grown beyond what the hardware can sustain.

The key insight: **high `cpu_user` alone is not a problem.** The problem arises when:
- `cpu_user` is high AND latency is rising (CPU is the bottleneck)
- `cpu_user` is high AND `cpu_iowait` is also high (the CPU looks busy, but it's mostly waiting — real issue is I/O)
- One process is consuming unexpectedly large share of CPU (runaway)

Scenario: a batch job that normally takes 5 minutes starts taking 40. `cpu_user` is pegged at 95%. Is the CPU too slow? Or did someone push an inefficient SQL query that now does a full table scan? The metric points you to user space; the investigation starts there.

---

## How to read it

```sh
# Real-time CPU breakdown, 1-second samples
mpstat -P ALL 1

# Which processes are consuming user CPU?
top -b -n 1 | head -20

# Or with more detail
pidstat -u 1 5
```

| `cpu_user` value | Interpretation |
|-----------------|----------------|
| 0–30% | Comfortable headroom |
| 30–70% | Normal load for busy systems |
| 70–90% | Watch for latency impact; check trends |
| 90–100% | CPU-bound; scale or optimize |

**Pair with:**
- `cpu_system`: if system is also high, the app makes lots of syscalls — check file I/O, network
- `cpu_iowait`: if iowait is high and user is high, user is misleading — the real bottleneck is storage
- `stat.procs_running`: if runqueue is growing, CPU is genuinely saturated

---

## A real episode

A media transcoding service ran fine at 60% `cpu_user` for months. After a library upgrade, users started reporting that jobs took 3x longer. Monitoring showed `cpu_user` had jumped to 98%. The team's first instinct was "we need more servers."

But a closer look with `pidstat` showed a single transcoding worker process using 95% of one CPU, while 15 other workers sat near-idle. The upgraded library had silently changed from multi-threaded to single-threaded mode. The work was piling into one thread.

Fix: one config line to re-enable threading. `cpu_user` dropped from 98% to 65%, spread across all cores. No new hardware needed.

**Lesson:** `cpu_user` tells you *that* the CPU is working hard. It doesn't tell you *where* or *whether the work is efficient*. Always dig into which processes and threads are running.

---

## What to do when it's high

**Step 1: Identify who is using the CPU.**
```sh
# Top CPU consumers by process
ps aux --sort=-%cpu | head -15

# Per-thread breakdown
pidstat -u -t 1 5
```

**Step 2: Check if it's actually causing a problem.**
Is latency rising? Are requests timing out? High `cpu_user` on a batch workload with no user-facing impact is fine. On an API server, 95% `cpu_user` may mean requests are queuing.

**Step 3: Rule out iowait masking.**
```sh
# If iowait is also elevated, the issue is storage, not compute
cat /proc/stat | awk 'NR==1{print "user="$2, "system="$4, "iowait="$6, "idle="$5}'
```

**Step 4: Profile the hot process.**
```sh
# Sample what code is executing (requires perf)
perf top -p <PID>

# Or use strace to see what syscalls dominate
strace -c -p <PID>
```

**Step 5: If CPU is genuinely saturated**, consider:
- Horizontal scaling (more instances)
- Algorithmic optimization (profile first)
- Reducing work per request (caching, batching)
- CPU affinity tuning for NUMA systems

---

## Common mistakes

**Treating 90% cpu_user as automatically bad.** A video encoder at 90% is healthy — that's what encoders are supposed to do. A web API server at 90% under normal traffic is a warning sign.

**Not pairing with iowait.** If `cpu_user + cpu_iowait = 95%`, the CPU looks busy but is mostly idle waiting for disk. Adding more compute won't help.

**Reacting to a single spike.** Look at sustained trends. A 5-second spike to 100% during a garbage collection cycle is normal. A sustained 90%+ over 10 minutes under normal traffic is not.

**Missing the per-core view.** System-wide `cpu_user` averages across all cores. One core at 100% with 15 idle cores shows up as only 6% in the aggregate. Always check per-CPU data.

---

## See also

- `stat.cpu_iowait` — time the CPU is idle waiting for I/O; the most common misread alongside cpu_user
- `stat.cpu_system` — kernel time; high system often means lots of syscalls from user code
- `stat.procs_running` — runqueue length; grows when CPU is the bottleneck
- `schedstat.cpu_stats` — per-CPU scheduler statistics for deeper analysis

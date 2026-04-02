# MemAvailable

[日本語版](../ja/meminfo.MemAvailable.md)

---

## What is it?

`MemAvailable` is the kernel's estimate of how much memory is available to start a new application without using swap. Added in kernel 3.14, it answers the question that `MemFree` cannot: "how much memory can I actually use right now?"

The key insight is that Linux treats file cache as disposable. When a process needs memory, the kernel can reclaim cache pages on the fly — so cache isn't "used" in any meaningful sense. `MemAvailable` = free memory + reclaimable cache + reclaimable slab.

```
  Total RAM: 16 GB
  ┌──────────────────────────────────────────┐
  │ App memory (AnonPages)    4 GB  ← locked │
  │ File cache (Cached)       8 GB  ← can reclaim
  │ Kernel / slab             1 GB  ← partially reclaimable
  │ MemFree                   3 GB  ← truly idle
  └──────────────────────────────────────────┘
                              ↑
  MemAvailable ≈ 3 + 8 + 0.5 ≈ 11 GB
```

---

## Why does it matter?

**MemAvailable is the single most useful memory health indicator** for day-to-day monitoring. When it drops:

- **Below ~20% of MemTotal**: swap pressure begins. The kernel starts moving anonymous pages to swap to make room.
- **Below ~5%**: the OOM killer becomes likely. It will pick a process and kill it — often one you didn't expect.
- **At 0**: the kernel is in crisis. New allocations fail; the system becomes unresponsive.

The drop is often gradual and silent. An application leaking memory 10 MB/hour won't trigger alerts for hours — until it's too late.

---

## How to read it

```sh
# Quick check — are you healthy?
free -h
# Look at "available" column, not "free"

# Watch it over time
watch -n 5 'grep -E "MemAvailable|MemTotal|SwapFree" /proc/meminfo'

# As a percentage
awk '/MemAvailable/{a=$2} /MemTotal/{t=$2} END{printf "%.1f%%\n", a/t*100}' /proc/meminfo
```

| MemAvailable / MemTotal | Status |
|-------------------------|--------|
| > 40% | Healthy |
| 20–40% | Watch for trends |
| 10–20% | Elevated pressure — investigate |
| < 10% | Danger — act now |
| < 5% | OOM imminent |

**Alert threshold recommendation**: alert at < 15%, page at < 5%.

---

## A real episode

It was 3 AM on a Tuesday. A Node.js app was running a session cache in memory — each user login stored a few KB of JSON. The load had grown slowly over months. Nobody noticed the session expiry was broken: sessions were never evicted.

At 2:47 AM, `MemAvailable` hit 4%. The kernel started swapping anonymous pages. Response times spiked from 50ms to 4 seconds. At 3:02 AM, the OOM killer fired and took down the database sidecar — not the Node.js app — because the sidecar had the highest RSS at that moment.

The on-call engineer woke to "database connection refused" alerts, not "memory low" alerts. They spent 45 minutes debugging the wrong thing.

The lesson: monitor `MemAvailable` as a percentage of `MemTotal`, set an alert threshold before it gets critical, and don't wait for the OOM killer to tell you there's a problem.

---

## What to do when it's low

**Step 1: Quantify the pressure.**
```sh
awk '/MemAvailable/{a=$2} /MemTotal/{t=$2} END{printf "%.1f%%\n", a/t*100}' /proc/meminfo
```

**Step 2: Is swap being used?**
```sh
grep SwapFree /proc/meminfo
# Also check actual swap activity:
vmstat 1 5 | awk '{print $7, $8}'  # si (swap-in) and so (swap-out)
```
If `so` is non-zero, your system is actively swapping — that's latency you're paying right now.

**Step 3: Who is consuming the memory?**
```sh
# Top memory consumers by RSS
ps aux --sort=-%mem | head -20

# Check if AnonPages is high (real app memory, not cache)
grep AnonPages /proc/meminfo
```

If `AnonPages` is large and growing, an application is leaking or not releasing memory. If `Cached` is large, reclaim may resolve it automatically — but verify with step 2.

**Step 4: Force cache drop (non-destructive, for testing only).**
```sh
sync && echo 3 > /proc/sys/vm/drop_caches
# This drops page cache + dentries + inodes. MemAvailable will spike briefly.
# Do NOT do this in production routinely — it hurts performance.
```

**Step 5: If a leak is confirmed, restart the leaking service** or add memory to the host. Kernel tuning does not fix a leak.

---

## Common mistakes

**Watching `MemFree` instead of `MemAvailable`.** `MemFree` is almost always low on a busy Linux box. `MemAvailable` is what matters.

**Panicking when cache is large.** `Cached` consuming 60% of RAM is healthy — it means your filesystem cache is working hard. Check `MemAvailable`, not `Cached`.

**Setting alerts on absolute values.** A "MemAvailable < 1 GB" alert is useless on a 4 GB host but excessive on a 512 GB host. Always use percentages.

**Ignoring the trend.** A single low reading may be a burst. `MemAvailable` falling steadily over hours is a leak. Use a time-series view.

**Restarting the wrong process.** The OOM killer picks the "highest-cost" victim, not necessarily the cause. Find the actual leaker with `ps` or `/proc/<pid>/status`.

---

## See also

- `meminfo.MemFree` — truly free pages (usually not what you want to monitor)
- `meminfo.Cached` — file cache that makes up most of MemAvailable
- `meminfo.AnonPages` — application heap/stack; rising here means real consumption
- `meminfo.SwapFree` — swap headroom; if this drops alongside MemAvailable, you're in trouble
- `pressure/memory_some_avg10` — kernel-level memory stall signal

# load_5min

[日本語版](../ja/loadavg.load_5min.md)

---

## What is it?

`load_5min` is the 5-minute exponential moving average of the Linux load average — the same count of running and I/O-blocked processes as `load_1min`, but smoothed over a longer window. It is the second field in `/proc/loadavg`.

If `load_1min` is a glance at the speedometer right now, `load_5min` is the average speed over the last stretch of highway. Short traffic jams don't show up much; sustained congestion does.

```
  load_1min  ████████████░░░░░░  (reactive — spikes visible)
  load_5min  ██████████░░░░░░░░  (smoother — trend visible)
  load_15min █████████░░░░░░░░░  (slowest — baseline visible)
              0         time →
```

The 5-minute value is the most operationally useful of the three. It responds fast enough to catch real problems, but is stable enough to filter out harmless one-minute spikes.

---

## Why does it matter?

The real power of `load_5min` emerges when you read all three values together. The relationship between them tells you whether a situation is improving or getting worse — without waiting another 15 minutes to find out.

**The trend rule:**
- `load_1min > load_15min`: load is **increasing** — the spike started recently
- `load_1min ≈ load_15min`: load is **stable** — whatever is happening has been happening a while
- `load_1min < load_15min`: load is **decreasing** — the worst is likely past

This is why operators look at all three in the same glance. The numbers themselves matter less than their relationship.

**Scenario:** An alert fires at 02:00 with load=12 (8-core machine). You check:
```
load average: 12.3, 4.1, 1.8
```
This says load just exploded in the last few minutes. Check what started at 02:00 — a cron job, a deployment, a traffic spike.

Compare with:
```
load average: 12.1, 11.9, 12.2
```
This says load has been at 12 for the last 15 minutes. The alert was probably delayed. Look for something that started 20+ minutes ago.

---

## How to read it

```sh
uptime
# load average: 5.20, 3.87, 2.41
#               1min  5min  15min
```

**Reading the trend at a glance:**

| 1min | 5min | 15min | Interpretation |
|------|------|-------|----------------|
| 8.0  | 4.0  | 2.0   | Rapidly worsening — act now |
| 4.0  | 4.0  | 4.0   | Stable high — been this way for a while |
| 2.0  | 5.0  | 8.0   | Recovering — may just need time |
| 0.5  | 3.0  | 6.0   | Recovering fast — incident likely over |

**Check whether the load is real:**
```sh
# Watch load trend over 30 seconds
watch -n 5 uptime

# Confirm with vmstat to see if CPU or I/O is the cause
vmstat 1 5
```

---

## A real episode

A database server started alerting at load=24 (12 cores). The on-call engineer arrived 8 minutes after the alert and saw:

```
load average: 24.1, 22.8, 8.3
```

The 5-minute value (22.8) being close to the 1-minute value (24.1) told her the situation was not new — it had been building for at least 5 minutes before she arrived. The 15-minute value (8.3) told her the problem started roughly 10–15 minutes ago. She checked recent deployments and found a migration job had started 12 minutes earlier that was doing full-table scans without indexes. She killed the job, and 5 minutes later:

```
load average: 4.2, 18.1, 12.5
```

The drop in load_1min confirmed the fix was working. Load_5min would take a few more minutes to catch up. She knew from the numbers that recovery was in progress without having to wait.

**Lesson:** The three values together form a timeline. When 1min drops but 5min is still high, recovery has started but isn't complete.

---

## What to do when load_5min is sustained high

"Sustained" means load_5min stays elevated for more than 5 minutes. This rules out harmless minute-scale spikes.

**Step 1: Compare all three values.**
```sh
uptime
```
Determine if load is still rising (1min > 5min), stable, or recovering.

**Step 2: Confirm the load type.**
```sh
vmstat 1 3
# wa column: high = I/O bound
# r  column: high = CPU bound, b column: high = blocked on I/O
```

**Step 3: Find the cause using the timeline.**
The difference between `load_1min` and `load_15min` gives a rough idea of when the problem started. If they are equal, it has been going on for 15+ minutes.

```sh
# Check what changed recently
journalctl --since "15 minutes ago" | tail -50
last -n 10  # recent logins
```

**Step 4: If the trend is worsening, isolate before tuning.**
Avoid tuning under a rising load. Isolate the process or job causing the load first.

---

## Common mistakes

**Reacting to load_1min without checking load_5min.** A 1-minute spike is often harmless. Wait for load_5min to confirm before escalating.

**Ignoring that load_5min lags behind.** When you fix a problem, load_5min takes several minutes to drop. Don't assume the fix failed just because load_5min is still high immediately after.

**Reading the numbers without the trend direction.** A load of 6 on an 8-core machine means nothing without knowing if it is going up or down. The three-value spread is the signal.

**Alerting on load_5min with the same threshold across machines.** Always normalize by core count: `load_5min / nproc > 1.5` is a more portable threshold.

---

## See also

- `loadavg.load_1min` — instantaneous view, more reactive
- `loadavg.load_15min` — the slowest-changing baseline value
- `stat.cpu_iowait` — confirms the I/O vs CPU nature of load
- `pressure.cpu_some_avg10` — PSI-based CPU pressure (more precise than load average)
- `stat.procs_blocked` — direct count of blocked processes

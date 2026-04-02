# load_15min

[日本語版](../ja/loadavg.load_15min.md)

---

## What is it?

`load_15min` is the 15-minute exponential moving average of the Linux load average — the slowest-moving of the three values in `/proc/loadavg`. It is the third field.

If `load_1min` is the current heartbeat and `load_5min` is the recent trend, `load_15min` is the baseline. It tells you what normal looks like for this system, right now.

```
  A sudden spike and recovery:          A sustained overload:
  ▲                                     ▲
  │      /\                             │           ___________
  │     /  \    load_1min              │          /
  │    /    \__  load_5min             │         /
  │___/______\___ load_15min           │________/   load_15min slowly rises
  └───────────────────────▶ time       └─────────────────────────▶ time
```

Because of the long smoothing window, `load_15min` changes slowly. A sudden problem that lasts only 3 minutes barely moves it. A problem sustained for 20 minutes will push it noticeably.

---

## Why does it matter?

`load_15min` is the number that tells you whether you are in an incident or just watching noise.

**As a baseline:** When a system is healthy, `load_15min` reflects its typical workload. If it sits at 2.0 on a healthy day, then 2.0 is your baseline. A spike to 6.0 in `load_1min` is notable. A `load_15min` of 6.0 means it has been at that level for most of the last 15 minutes — that is an incident.

**As a recovery signal:** After you fix a problem, `load_1min` drops first. `load_5min` follows. `load_15min` lags behind the most. If `load_1min` is back to normal but `load_15min` is still elevated, the incident likely just ended — you are watching the 15-minute average catch up.

**As a drift detector:** If `load_15min` creeps up slowly over days or weeks — from 1.5 to 2.5 to 3.5 — without any incident, the system is accumulating background load. This is often a sign of a slow memory leak, runaway cron jobs, or a capacity problem that will eventually surface as an incident.

---

## How to read it

```sh
uptime
# load average: 1.20, 2.85, 5.40
#               ↑     ↑     ↑
#               1min  5min  15min
```

This output tells a specific story: load has been at 5.4 for a while but is now dropping fast. Load_1min at 1.2 means the heavy period recently ended. Recovery is well underway.

**The direction matrix:**

| Condition | Meaning |
|-----------|---------|
| 1min > 5min > 15min | Accelerating — load is rising, has been for a while |
| 1min < 5min < 15min | Decelerating — load is dropping, recovering |
| 1min ≈ 5min ≈ 15min | Stable — whatever level it's at, it's been there |
| 1min >> 15min | Sudden spike — watch for fast recovery or continued rise |
| 1min << 15min | Sudden drop — incident recently resolved |

**Is the 15min value unusual for this machine?**
```sh
# Watch how load_15min behaves during normal hours
# Compare morning vs afternoon vs overnight
sar -q 1 0  # if sysstat is installed — shows load over time
```

---

## A real episode

A payment processing server had been running at `load_15min` ≈ 1.5 for months. One Monday morning, an engineer noticed it had drifted to 3.8 over the previous two weeks — no incidents, no alerts, just a slow climb.

Digging in, she found a batch reporting job added six weeks earlier that was re-scanning an increasingly large table every hour. The table had grown 10x since the job was added. The job took 4 minutes at first; now it took 22 minutes per run, overlapping with the next run and accumulating.

Nothing had "broken" — but the 15-minute average showed the drift that the 1-minute value hid in noise. She added an index, and `load_15min` dropped to 1.6 over the next few days as the backlog cleared.

**Lesson:** Slow drift in `load_15min` over days or weeks is often more diagnostic than any single spike.

---

## What to do when load_15min is high

High `load_15min` means the overload is not a spike — it has been sustained.

**Step 1: Determine the trend direction.**
```sh
uptime
```
If 1min > 15min: still getting worse. If 1min < 15min: peak has passed.

**Step 2: Find out how long the load has been elevated.**
If `load_15min` is significantly above your normal baseline, the problem has been going on for 15+ minutes. Look further back:
```sh
journalctl --since "30 minutes ago" | grep -i "error\|warn\|fail"
```

**Step 3: Check the load type.**
```sh
vmstat 1 3
# I/O-bound: high 'wa', high 'b', low 'r'
# CPU-bound: low 'wa', high 'r'
```

**Step 4: After fixing, watch the slow recovery.**
`load_15min` will take 10–15 minutes to reflect the fix. Be patient. Confirm recovery with `load_1min` first, then watch `load_15min` trend downward.

---

## Common mistakes

**Using `load_15min` to detect fast incidents.** It is too slow for that. Use `load_1min` for alerting on sudden spikes. Use `load_15min` to confirm sustained problems or detect slow drift.

**Declaring recovery when `load_15min` is still high.** The 15-minute average is a lagging indicator. If you resolved a problem 5 minutes ago, `load_15min` will still show the elevated value for another 10 minutes. Confirm recovery with `load_1min` instead.

**Ignoring slow drift over days.** Most monitoring focuses on threshold crossing. Slow `load_15min` drift that stays below threshold is invisible to most alerting systems. Consider a trend-based alert or weekly review.

---

## See also

- `loadavg.load_1min` — reactive, current value
- `loadavg.load_5min` — medium-term trend, most operationally useful
- `stat.cpu_iowait` — confirms I/O vs CPU nature of load
- `pressure.cpu_some_avg10` — more precise CPU contention signal
- `vmstat.nr_dirty` — can reveal a slow writeback backlog contributing to load drift

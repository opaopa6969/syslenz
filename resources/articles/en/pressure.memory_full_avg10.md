# memory_full_avg10

[日本語版](../ja/pressure.memory_full_avg10.md)

---

Part of the **[PSI (Pressure Stall Information)](pressure.memory_some_avg10.md)** metrics — see the `avg10` article for full context.

`memory_full_avg10` — % of time ALL tasks stalled on memory (10s avg) — severe memory pressure.

**Source:** `/proc/pressure/{cpu,memory,io}`  
**Unit:** percentage (0–100) or microseconds (for `_total`)

**Quick guide to PSI variants:**
- `some_*` = ANY task stalled (one or more affected)
- `full_*` = ALL tasks stalled (system completely blocked)
- `avg10` = 10-second exponential moving average
- `avg60` = 60-second average (smoother trend)
- `avg300` = 5-minute average (long-term trend)
- `total` = cumulative microseconds since boot

---

## See also

- `pressure.memory_some_avg10` — primary article with context and thresholds
- `sourceguide.pressure` — full PSI source overview

# io_full_avg300

[日本語版](../ja/pressure.io_full_avg300.md)

---

Part of the **[PSI (Pressure Stall Information)](pressure.io_some_avg10.md)** metrics — see the `avg10` article for full context.

`io_full_avg300` — % of time ALL tasks stalled on I/O (5min average).

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

- `pressure.io_some_avg10` — primary article with context and thresholds
- `sourceguide.pressure` — full PSI source overview

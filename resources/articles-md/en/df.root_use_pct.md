# root_use_pct

What is this?
Percentage of the root filesystem that is in use.

Why it matters
This is the easiest disk-capacity alarm to understand. Once the root filesystem fills, many services fail in ugly ways.

How to read
- Watch the trend, not just the absolute number.
- 80% can already be uncomfortable on busy hosts.
- 90%+ is usually a cleanup or expansion task.

Next check
Compare with available_disk and diskstats.active_devices.
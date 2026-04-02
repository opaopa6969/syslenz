# active_devices

What is this?
The number of block devices that have seen real I/O activity since boot.

Why it matters
It is a quick way to tell whether storage load is concentrated on a few devices or spread across many.

How to read
- Low count with high I/O pressure points to a few hot disks.
- High count with low pressure can be background noise.
- Use the device table for the real breakdown.

Next check
Compare with pressure.io_some_avg10 and root disk usage.
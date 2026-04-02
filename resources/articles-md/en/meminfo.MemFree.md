# MemFree

What is this?
RAM that is not allocated to anything right now.

Why it matters
Low MemFree is normal on Linux because the kernel uses RAM for cache. Do not treat MemFree as a shortage signal by itself.

How to read
- If MemAvailable is healthy, low MemFree is usually fine.
- If both MemFree and MemAvailable fall, pressure is real.
- If swap activity rises too, reclaim is no longer enough.

Next check
Look at Cached, MemAvailable, and swap activity together.
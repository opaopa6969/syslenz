# Cached

What is this?
File-backed memory kept in RAM so reads do not have to hit disk again.

Why it matters
Cached RAM is not wasted RAM. It is usually the first reclaimable pool when memory pressure rises.

How to read
- Large Cached on a file server is often good.
- Falling Cached plus rising pswpout means the kernel is running out of cheap reclaim.
- Dirty pages mixed into the picture mean writeback may be lagging.

Next check
Compare with nr_dirty, nr_writeback, and MemAvailable.
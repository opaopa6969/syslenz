# Storage Latency

What is this?
The time it takes for a block device and its filesystem path to complete reads, writes, and flushes.

Why it matters
Storage often looks idle by bandwidth but still hurts latency when the queue is deep or the device is busy with small IO.

How to use
- Separate sequential throughput from latency
- Watch average, tail, and queue depth together
- Check fsync, flush, and write amplification behavior

Common mistakes
- Thinking bandwidth alone explains disk performance
- Ignoring small random IO
- Missing that the filesystem layer can amplify device delays

Diagnostic flow
1. Confirm whether the symptom is read, write, or flush heavy.
2. Check queue depth and tail latency.
3. Check pressure and retries.
4. Decide whether the bottleneck is filesystem, driver, or device media.

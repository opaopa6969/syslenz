# nr_dirty

[日本語版](../ja/vmstat.nr_dirty.md)

---

## What is it?

`nr_dirty` counts the number of memory pages that have been written to but not yet flushed to disk. The kernel buffers writes in RAM first — your app writes to a file, the kernel marks that page "dirty," and a background kernel thread (`pdflush` / `kworker`) eventually flushes it to storage.

Think of it as unsaved work. Every dirty page is data that exists only in RAM. If the system crashes right now, that data is gone.

```
  App writes to file
       |
       v
  Page Cache (RAM)     <-- nr_dirty counts these
  [dirty] [dirty] [dirty] [clean] [clean]
       |
       | pdflush / kworker flushes in background
       v
  Disk (persistent)    <-- nr_writeback counts in-progress flushes
```

---

## Why does it matter?

**Write pressure.** When `nr_dirty` climbs and stays high, the kernel is accumulating writes faster than it can flush them. Eventually the kernel throttles your application — literally stalls it — until the backlog drains. This shows up as unpredictable write latency spikes.

**Data durability risk.** High dirty page counts mean more data loss exposure on a crash or power failure. Most apps don't care (they use `fsync()`), but databases and message queues care a lot.

**Flush storms.** When `nr_dirty` hits the kernel's `dirty_background_ratio` threshold, background flushing kicks in. When it hits `dirty_ratio`, foreground flushing blocks your app. If dirty accumulates slowly and then drains in one big burst, you get periodic latency spikes with no obvious cause.

```
  nr_dirty over time:

  ▲
  │         /\        /\
  │        /  \      /  \      <-- flush storms: burst then drain
  │       /    \    /    \
  │______/      \__/      \___
  └──────────────────────────▶ time

  vs. healthy:
  ▲
  │ ~~~~~~~~~~~~~~~~~~~        <-- steady low value, continuous flushing
  └──────────────────────────▶ time
```

---

## How to read it

**Absolute value matters less than trend.** A server doing heavy write I/O might normally sit at 50,000 dirty pages — that's fine. The same value on a mostly-read workload is a warning sign.

**Always pair with `nr_writeback`.** These two tell the story together:

| Situation | nr_dirty | nr_writeback | What it means |
|-----------|----------|--------------|---------------|
| Normal write load | Low–medium | Low | Flush keeping up |
| Write burst | Rising | Rising | Flush active, watch for throttle |
| Throttle risk | High, stable | Low | Flush stopped or blocked |
| After flush storm | Dropping | Dropping | Recovering |
| Storage bottleneck | High | 0 | Kernel can't write — check disk |

**Kernel thresholds to know:**

```
  /proc/sys/vm/dirty_background_ratio   (default: 10%)
  → Background flush starts when dirty exceeds this % of RAM

  /proc/sys/vm/dirty_ratio              (default: 20%)
  → App writes BLOCK until dirty drops below this

  /proc/sys/vm/dirty_expire_centisecs   (default: 3000 = 30s)
  → Pages older than this are flushed regardless of ratio
```

Check these on your system:
```sh
sysctl vm.dirty_background_ratio vm.dirty_ratio vm.dirty_expire_centisecs
```

---

## What to do when it's high

**Step 1: Is it actually causing a problem?**
Check `pressure/io_some_avg10`. If I/O pressure is also elevated, your app is being throttled. If pressure is low, the high dirty count may be normal write buffering.

**Step 2: Check if flushing is running.**
```sh
cat /proc/vmstat | grep -E 'nr_dirty|nr_writeback'
# Run twice, 5 seconds apart — are the numbers moving?
```
If `nr_dirty` is high and `nr_writeback` is 0, something is blocking flushes. Check disk health and I/O errors:
```sh
dmesg | tail -20
iostat -x 1
```

**Step 3: Check your writeback bandwidth.**
`nr_written` in `/proc/vmstat` counts total pages written to disk. A rising `nr_dirty` with a flat `nr_written` means flushes aren't completing.

**Step 4: Tune if needed.**
If the flush storms are hurting latency, reduce dirty ratios to flatten the burst:
```sh
# More aggressive background flushing, less bursty
sysctl vm.dirty_background_ratio=5
sysctl vm.dirty_ratio=10
```
Test before applying permanently — tighter ratios increase I/O throughput requirements.

---

## Common mistakes

**Treating a high number as automatically bad.** A database doing bulk writes will have high `nr_dirty`. That's expected. Look at whether it's causing throttle (`dirty_ratio` crossed) or latency.

**Ignoring `nr_writeback`.** `nr_dirty` alone doesn't tell you if flushing is healthy. A dirty count of 100,000 with active writeback is very different from 100,000 with zero writeback.

**Tuning `dirty_ratio` without measuring throughput.** Lower dirty ratios mean more frequent, smaller flushes. This increases I/O operations per second required from your storage. On SSDs this is fine. On HDDs, too-frequent flushes cause seek overhead.

---

## See also

- `vmstat.nr_writeback` — pages currently being written to disk (the flush pipeline)
- `vmstat.nr_dirty_threshold` — the computed kernel limit before app stall
- `pressure/io_some_avg10` — whether I/O pressure is actually being felt
- `diskstats` — per-device write throughput and queue depth

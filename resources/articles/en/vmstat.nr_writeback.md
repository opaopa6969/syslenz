# nr_writeback

[日本語版](../ja/vmstat.nr_writeback.md)

---

## What is it?

`nr_writeback` counts the number of memory pages **currently being written to disk** — in-flight writes. These are pages that have already been marked dirty and the kernel has started flushing them, but the write hasn't completed yet.

If `nr_dirty` is the queue of work waiting to be done, `nr_writeback` is the work actively in progress.

```
  Write pipeline:

  App writes data
       │
       v
  Page Cache (RAM)
  [dirty] [dirty] [dirty]   <- nr_dirty: waiting to flush
       │
       │ kernel triggers flush (pdflush / kworker)
       v
  [writeback] [writeback]   <- nr_writeback: being written now
       │
       v
  Disk (persistent storage)  <- write complete, page becomes clean
```

Think of it as packages at the loading dock: `nr_dirty` is boxes stacked up waiting to be loaded; `nr_writeback` is boxes being loaded onto the truck right now.

---

## Why does it matter?

`nr_writeback` by itself is not alarming — some pages in-flight is completely normal during any write workload. The signal comes from reading it **together with `nr_dirty`** and watching their relationship over time.

**Pattern 1: Both high and moving.** Flushing is active and keeping up (roughly). This is normal under heavy write load. Watch whether `nr_dirty` is growing faster than `nr_writeback` can drain it.

**Pattern 2: `nr_dirty` high, `nr_writeback` near zero.** This is the danger sign. Dirty pages are accumulating but nothing is being written to disk. Possible causes: storage device saturated, I/O errors, NFS mount unresponsive, or filesystem in error state. The kernel may be throttling applications (write stall).

**Pattern 3: `nr_writeback` sustained high.** The flush pipeline is always full — storage can't absorb writes fast enough. Over time, this backs up into `nr_dirty`, eventually triggering write throttling where applications block in `write()`.

```
  nr_writeback sustained high → storage bottleneck:

  ▲ pages
  │         nr_dirty  ___________
  │                  /           \
  │  nr_writeback ~~~~~~~~~~~~~~~~~~~  <- always at max
  │
  └────────────────────────────────▶ time
    ("truck is always full; more boxes keep arriving")
```

---

## How to read it

```sh
# Watch both fields in real time
watch -n 1 'grep -E "nr_dirty|nr_writeback" /proc/vmstat'

# Or with vmstat
vmstat -m 1   # shows memory stats including writeback

# Check write throughput (is flushing making progress?)
iostat -x 1 | grep -v '^$'
```

| `nr_writeback` | `nr_dirty` | Interpretation |
|----------------|------------|----------------|
| Low | Low | No significant write activity |
| Low–medium | Medium–high | Write backlog building; watch trend |
| High (moving) | Declining | Flush is running; healthy drain |
| High (stable) | Rising | Storage can't keep up; watch for stall |
| Near zero | High | Flush stopped — check disk/errors |

**Units:** Each page is 4 KB on most systems. 10,000 pages = 40 MB in-flight.

```sh
# Convert pages to MB
echo $(( $(grep nr_writeback /proc/vmstat | awk '{print $2}') * 4 / 1024 )) MB
```

---

## A real episode

A logging infrastructure team noticed their Elasticsearch nodes had degraded write performance — indexing latency had doubled over 48 hours. CPU looked fine. Memory looked fine. Network looked fine.

Someone checked `nr_writeback`:
```
nr_dirty      12450
nr_writeback    0
```

Dirty pages were accumulating — over 12,000 — but nothing was being written to disk (`nr_writeback` was zero). `iostat` showed the data disk at 0% utilization. The disk looked idle.

`dmesg` told the story:
```
EXT4-fs error (device sdb1): ext4_find_entry:1455: inode #2: comm kworker: reading directory lblock 0
```

The filesystem had hit an error state and had silently stopped all writes. The kernel was accepting data into the page cache (so applications weren't failing yet), but nothing was reaching disk. A filesystem check (`fsck`) and remount fixed it.

**Lesson:** `nr_writeback = 0` with `nr_dirty` rising is not a sign of low write activity. It can mean writes are completely stopped. Always cross-check with `dmesg` when you see this pattern.

---

## What to do when it's abnormal

**Case A: `nr_writeback` high and `nr_dirty` rising (storage saturated)**
```sh
# Identify the saturated device
iostat -x 1 5
# Look for %util near 100%, high await

# Which process is writing?
iotop -aoP

# Reduce write pressure temporarily
ionice -c 3 -p <PID>   # lower priority for heavy writers
```

**Case B: `nr_writeback` near zero with `nr_dirty` rising (flush stopped)**
```sh
# Check for filesystem errors
dmesg | tail -30 | grep -iE "error|EIO|timeout|reset|abort"

# Is the device still healthy?
smartctl -a /dev/sda

# Check for stuck mounts (NFS?)
df -h   # hangs if NFS is unresponsive
mount | grep nfs
```

**Case C: Both periodically spike then drain (flush storms)**
```sh
# These are triggered by dirty_ratio thresholds
sysctl vm.dirty_background_ratio vm.dirty_ratio

# Reduce burst size with more aggressive background flushing
sysctl vm.dirty_background_ratio=5   # default 10
```

---

## Common mistakes

**Ignoring it because it "looks low."** `nr_writeback = 0` during a write workload is unusual, not reassuring. It may mean flushing has stopped.

**Looking at `nr_writeback` without `nr_dirty`.** The two must be read together. `nr_writeback` of 5,000 with `nr_dirty` of 1,000 means the flush is winning. The same `nr_writeback` with `nr_dirty` of 50,000 and rising means you're losing.

**Assuming high writeback = bad.** During a `sync` or fsync-heavy workload, `nr_writeback` will be high and that's expected. Look at application-level symptoms (write latency, blocking calls) to determine if it's a problem.

---

## See also

- `vmstat.nr_dirty` — dirty pages waiting to be flushed; always read with nr_writeback
- `vmstat.nr_dirty_threshold` — the computed limit before application writes stall
- `pressure/io_some_avg10` — whether I/O pressure is causing actual task stalls
- `diskstats` — per-device write throughput, utilization, and queue depth

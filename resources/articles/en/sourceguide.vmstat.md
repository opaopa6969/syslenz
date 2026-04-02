# sourceguide: vmstat

[日本語版](../ja/sourceguide.vmstat.md)

---

## What is this source?

`/proc/vmstat` is the kernel's virtual memory event log — a flat list of cumulative counters that the kernel increments each time a specific VM event occurs. Unlike `/proc/meminfo` which shows current state, `/proc/vmstat` shows *how much activity has happened* since boot.

The counters cover page reclaim, page faults, swap I/O, writeback, compaction, and more. There are often 200+ fields on a modern kernel. Most are zero on a healthy system; that's exactly why the ones that aren't deserve attention.

```
  /proc/vmstat: cumulative event counters (never reset)
  ┌─────────────────────────────────────────────────────┐
  │  nr_dirty          = 1842     (current snapshot)    │
  │  pgmajfault        = 38210    (since boot)          │
  │  pswpin            = 0        (pages swapped in)    │
  │  pswpout           = 0        (pages swapped out)   │
  │  nr_writeback      = 0        (currently flushing)  │
  └─────────────────────────────────────────────────────┘
  
  To get rate: sample twice, subtract, divide by interval
  (the `vmstat 1` command does this for you)
```

The `vmstat 1` command you probably know reads from here and shows *per-second deltas*. `/proc/vmstat` is the raw cumulative source.

---

## What questions does it answer?

- Is the kernel actively swapping pages in or out? (`pswpin`, `pswpout`)
- How often are major page faults happening? (`pgmajfault`) — each one stalls a process waiting for disk
- Is dirty data accumulating faster than it can be flushed? (`nr_dirty`, `nr_writeback`)
- Is the kernel reclaiming memory aggressively? (`pgsteal_kswapd`, `pgscand`)
- Are there memory compaction failures that indicate fragmentation? (`compactfail`)

---

## Key fields to watch

| Field | Type | What it means |
|---|---|---|
| `nr_dirty` | Snapshot | Current dirty page count. Watch for high-and-stable (flush stalled) vs high-and-falling (flushing). |
| `nr_writeback` | Snapshot | Pages actively being written to disk. Zero with high nr_dirty = flush blocked. |
| `pgmajfault` | Counter | Major page faults since boot. Each fault = a process waited for disk. Rate spike = swap or mmap pressure. |
| `pswpin` / `pswpout` | Counter | Pages swapped in / out since boot. Any nonzero rate means anonymous pages are being evicted. |
| `pgsteal_kswapd` | Counter | Pages reclaimed by kswapd background thread. High rate = sustained memory pressure. |
| `pgscand` | Counter | Pages scanned during direct reclaim. Direct reclaim blocks the faulting process — more painful than kswapd. |

---

## How to read it directly

```sh
# Single snapshot
cat /proc/vmstat | grep -E 'nr_dirty|nr_writeback|pgmajfault|pswpin|pswpout'

# Deltas every 2 seconds (the classic tool)
vmstat 2

# Watch specific counters change
watch -n 2 'grep -E "pgmajfault|pswpin|pswpout|nr_dirty" /proc/vmstat'
```

To manually compute a rate:

```sh
# Sample pgmajfault twice, 10 seconds apart
v1=$(grep pgmajfault /proc/vmstat | awk '{print $2}')
sleep 10
v2=$(grep pgmajfault /proc/vmstat | awk '{print $2}')
echo "pgmajfault rate: $(( (v2 - v1) / 10 )) per second"
```

---

## A real episode

A Java service was experiencing random 200–500ms latency spikes every few minutes, but CPU and heap metrics looked normal. `/proc/vmstat` showed `pgmajfault` incrementing at 80–120 per second during spike windows. `pswpin` was also nonzero despite the system reporting 2 GB free.

The catch: the JVM's off-heap memory (mapped files for Lucene indices) was being paged out because the kernel's page cache had grown to fill most RAM. The system had "free" memory in the sense that `MemFree` wasn't zero, but `MemAvailable` told a different story. When the JVM accessed those mmap'd segments, it faulted them back in from swap — each fault adding 5–15ms of latency.

The resolution was limiting the page cache with `vm.dirty_ratio` and adding explicit `madvise(MADV_WILLNEED)` calls in the application to hint to the kernel that those mmap regions should stay resident.

---

## See also

- `sourceguide.meminfo` — current memory state (pair with vmstat for full picture)
- `vmstat.nr_dirty` — deep dive on dirty pages, flush thresholds, and storm patterns
- `sourceguide.pressure` — PSI metrics: did processes actually stall waiting for memory or I/O?
- `sourceguide.diskstats` — disk throughput and queue depth (correlate with writeback activity)

# Memory Compaction — vmstat

[日本語版](../ja/vmstat.compact.md)

---

## What is it?

**Memory compaction** is the kernel's way of defragmenting physical memory. Over time, free pages get scattered in small islands between occupied pages. Compaction moves pages together to create large contiguous free regions — primarily to satisfy Transparent Huge Page (THP) allocation requests.

```
Before compaction:        After compaction:
[used][free][used][free]  [used][used][free][free]
[free][used][free][used]  [used][used][free][free]
         ↕                         ↕
  No 2MB block available    Contiguous 2MB block!
```

Two actors run compaction:
- **Direct compaction**: triggered when a process needs a large page right now — synchronous, stalls the process
- **kcompactd**: background daemon, runs proactively to keep memory defragmented

---

## Metrics at a glance

| Metric | What it counts |
|--------|---------------|
| `compact_stall` | Direct compaction stalls (process blocked) |
| `compact_fail` | Compaction attempts that found no large block |
| `compact_success` | Compaction attempts that freed a large block |
| `compact_migrate_scanned` | Pages scanned looking for pages to move |
| `compact_free_scanned` | Pages scanned looking for free destinations |
| `compact_isolated` | Pages temporarily isolated during compaction |
| `compact_daemon_wake` | kcompactd wakeups |
| `compact_daemon_migrate_scanned` | Pages scanned by kcompactd |
| `compact_daemon_free_scanned` | Free pages scanned by kcompactd |

---

## Why does it matter?

**`compact_stall` is your primary signal.** Each stall means a process was blocked while the kernel tried to defragment memory to fulfill a THP request. In latency-sensitive applications (databases, web servers), these stalls show up as unexplained latency spikes.

**Success rate tells the story:**
```
success / (success + fail) = compaction efficiency
```
A low efficiency means memory is too fragmented even after compaction. This means THP allocations are mostly failing, and you're paying compaction overhead with little benefit.

**kcompactd wakeups.** If `compact_daemon_wake` is climbing steadily, the background daemon is being triggered frequently — your workload is creating fragmentation faster than kcompactd can fix it.

---

## What to do

```sh
# Check current fragmentation
cat /proc/buddyinfo
# Look for big gaps between order-0 and order-10 counts

# View compaction stats live
grep compact /proc/vmstat | watch -n 2 cat /proc/vmstat

# Tune: how aggressively to compact on THP fault
cat /sys/kernel/mm/transparent_hugepage/defrag
# Options: always, defer, defer+madvise, madvise, never
echo defer+madvise > /sys/kernel/mm/transparent_hugepage/defrag
```

If `compact_stall` is impacting latency, set THP to `madvise` mode — only applications that explicitly request THP get it, eliminating background compaction pressure.

---

## See also

- `vmstat.thp` — Transparent Huge Pages (the reason compaction exists)
- `buddyinfo.zones` — physical memory fragmentation state
- `vmstat.allocstall` — allocation stalls (related but different from compact_stall)
- `sourceguide.vmstat` — full vmstat overview

# sourceguide: meminfo

[日本語版](../ja/sourceguide.meminfo.md)

---

## What is this source?

`/proc/meminfo` is the kernel's real-time memory accounting ledger. Every line is a named counter updated by the kernel's memory management subsystem — page allocator, slab allocator, swap mechanism, and page cache — without buffering or delay.

```
  Physical RAM
  ┌─────────────────────────────────────────────┐
  │  Kernel (slab, page tables, etc.)           │
  │  Page cache (file-backed)  ← Cached         │
  │    └─ Dirty pages          ← Dirty          │
  │  Anonymous pages (heap/stack) ← AnonPages   │
  │  Free                      ← MemFree        │
  └─────────────────────────────────────────────┘
        │                            │
     Swap out                    Swap in
        ▼                            ▲
  Swap space (disk)           ← SwapFree tracks remaining
```

The numbers here are what the kernel *actually knows* about RAM — not what a monitoring agent inferred.

---

## What questions does it answer?

- How much memory can a new allocation actually use? (`MemAvailable`)
- Is the system paging anonymous memory to swap? (`SwapFree` trend)
- How large is the page cache, and is it building up dirty data? (`Cached`, `Dirty`)
- Are anonymous pages (heap, stack, mmap) growing unexpectedly? (`AnonPages`)
- Is the kernel holding back memory in free lists or slab caches? (`MemFree`, `SReclaimable`)

---

## Key fields to watch

| Field | What it means | Why it matters |
|---|---|---|
| `MemAvailable` | Estimated free + reclaimable memory | The realistic "how much can I allocate" number. More useful than MemFree alone. |
| `AnonPages` | Anonymous (non-file) pages in RAM | Growing means heap/mmap expansion. Shrinking under load means swap pressure. |
| `Cached` | Page cache size (file-backed pages) | High is fine; it means I/O is being absorbed. Watch for it squeezing AnonPages. |
| `Dirty` | Pages written but not yet flushed | Persistent high values mean write backlog. Spikes precede flush storms. |
| `SwapFree` | Free swap space remaining | Falling means anonymous pages are being evicted — a serious pressure signal. |
| `SReclaimable` | Reclaimable slab memory (dentries, inodes) | Can be freed under pressure; included in MemAvailable calculation. |
| `Writeback` | Pages currently being flushed to disk | High alongside Dirty means flush is running. Zero with high Dirty means flush stalled. |

---

## How to read it directly

```sh
cat /proc/meminfo
```

Typical output on a lightly loaded 64 GB server:

```
MemTotal:       65780736 kB
MemFree:         2341024 kB
MemAvailable:   58200448 kB
Buffers:           19840 kB
Cached:         56023552 kB
SwapCached:            0 kB
AnonPages:       4312064 kB
Dirty:              2304 kB
Writeback:             0 kB
SwapTotal:       8388604 kB
SwapFree:        8388604 kB
SReclaimable:    3201024 kB
```

To watch it live and filter to the fields you care about:

```sh
watch -n 1 'grep -E "MemAvailable|AnonPages|Dirty|Writeback|SwapFree" /proc/meminfo'
```

---

## A real episode

A batch processing job ran nightly and completed successfully, but morning response times were elevated for 20–30 minutes after it finished. `/proc/meminfo` showed: during the batch job, `Cached` had grown to fill nearly all RAM. After the job ended, `AnonPages` for the web service began growing again — and `MemAvailable` was low enough that each allocation triggered reclaim from the page cache. `Dirty` was spiking during these reclaim events because the batch writes hadn't fully flushed.

The fix was not adding RAM. It was bounding the batch job's I/O with `ionice` to prevent it from monopolizing the page cache, and tuning `vm.dirty_background_ratio` to flush more aggressively during the batch window. `MemAvailable` after the batch run stayed above 4 GB, and the morning latency bump disappeared.

---

## See also

- `sourceguide.vmstat` — cumulative counters for page faults, swap I/O, and reclaim activity
- `sourceguide.pressure` — time-based pressure stall metrics (are processes actually blocked on memory?)
- `vmstat.nr_dirty` — dirty page count with thresholds and flush storm explanation
- `sourceguide.swaps` — swap area inventory and usage breakdown

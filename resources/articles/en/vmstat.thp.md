# Transparent Huge Pages (THP) — vmstat

[日本語版](../ja/vmstat.thp.md)

---

## What is it?

The Linux kernel normally manages memory in 4 KB pages. **Transparent Huge Pages (THP)** let the kernel silently use 2 MB pages for anonymous memory (heap, stack, mmap) without any application change. More data fits in the CPU's Translation Lookaside Buffer (TLB), which means fewer TLB misses and faster memory access.

```
  Normal paging:               THP:
  ┌──┐┌──┐┌──┐┌──┐            ┌────────────────────────┐
  │4k││4k││4k││4k│  ×512      │         2 MB           │
  └──┘└──┘└──┘└──┘            └────────────────────────┘
  512 TLB entries needed       1 TLB entry needed
```

The kernel tries to allocate a 2 MB page whenever a process touches a large contiguous region. If it can't (fragmentation, memory pressure), it falls back to 4 KB pages — transparently.

---

## THP metrics at a glance

| Metric | What it counts |
|--------|---------------|
| `thp_fault_alloc` | Successful 2 MB allocations on page fault |
| `thp_fault_fallback` | Fallbacks to 4 KB (couldn't get 2 MB) |
| `thp_collapse_alloc` | khugepaged successfully collapsed 4K→2M |
| `thp_collapse_alloc_failed` | khugepaged collapse failed |
| `thp_split_page` | 2 MB pages split back to 4 KB |
| `thp_deferred_split_page` | Pages queued for splitting |
| `thp_swpout` | 2 MB pages swapped out whole |
| `thp_swpout_fallback` | Split required before swap-out |
| `thp_file_alloc` | THP for file-backed mappings |
| `thp_migration_success/fail` | THP migration results |

---

## Why does it matter?

**THP is a trade-off, not a free lunch.**

**Benefits:**
- Fewer TLB misses → faster memory-intensive workloads (databases, JVMs, ML)
- Lower kernel overhead per page (fewer page table entries)

**Costs:**
- A high `thp_fault_fallback` rate means fragmentation is preventing THP — you're paying the overhead of khugepaged scans without the benefit
- `thp_split_page` rising means the kernel allocated a 2 MB page then had to split it (e.g., for partial munmap, mprotect) — wasted effort
- THP collapse pauses: khugepaged runs in the background, but collapse itself briefly stalls the process

**The database problem.** Redis and some PostgreSQL workloads perform *worse* with THP. A 2 MB page means a write to any byte dirties the entire 2 MB page — massive write amplification. This is why many DBs tell you to disable THP.

---

## A real episode

A Java service was running fine for weeks, then started hitting GC pauses of 800ms every few minutes. GC logs showed full GCs that "shouldn't be happening." The heap was 8 GB, GC was G1, everything configured correctly.

The root cause: THP + G1GC. G1 divides the heap into fixed regions. When THP splits a 2 MB page mid-region (because G1 was remapping regions), the kernel deferred the split, which caused `thp_deferred_split_page` to climb into the millions. Eventually the deferred split queue flushed during a memory allocation, causing a synchronous pause.

Fix: `echo madvise > /sys/kernel/mm/transparent_hugepage/enabled` — only use THP when the application explicitly requests it via `madvise(MADV_HUGEPAGE)`.

---

## How to read the metrics

**Healthy THP workload:**
```
thp_fault_alloc: climbing steadily
thp_fault_fallback: low relative to alloc (< 10%)
thp_split_page: low
thp_deferred_split_page: low
```

**Fragmentation problem:**
```
thp_fault_fallback: high relative to alloc
thp_collapse_alloc_failed: rising
```
→ Try `echo always > /sys/kernel/mm/transparent_hugepage/defrag` or reduce memory fragmentation.

**Over-splitting:**
```
thp_split_page: high
thp_deferred_split_page: growing
```
→ Workload has partial-page operations (mprotect, fork+exec, partial munmap). Consider `madvise` mode.

---

## Tuning

```sh
# Check current mode
cat /sys/kernel/mm/transparent_hugepage/enabled
# [always] madvise never

# Options:
echo always   > /sys/kernel/mm/transparent_hugepage/enabled  # aggressive (default)
echo madvise  > /sys/kernel/mm/transparent_hugepage/enabled  # only when requested
echo never    > /sys/kernel/mm/transparent_hugepage/enabled  # disable completely

# Defrag policy (how hard to try for 2MB on fault):
cat /sys/kernel/mm/transparent_hugepage/defrag
echo defer+madvise > /sys/kernel/mm/transparent_hugepage/defrag
```

---

## Common mistakes

**Assuming THP always helps.** Databases, Redis, and apps with lots of partial-page operations often benefit from `madvise` or `never` mode.

**Ignoring fallback rate.** A high `thp_fault_fallback / thp_fault_alloc` ratio means you're getting THP overhead without THP benefit.

**Not checking deferred splits.** `thp_deferred_split_page` growing indefinitely signals a split storm waiting to happen.

---

## See also

- `vmstat.compact` — memory compaction that enables THP allocation
- `meminfo.AnonHugePages` — current THP memory in use
- `vmstat.pgmajfault` — major faults (THP can reduce these)
- `sourceguide.vmstat` — full vmstat source overview

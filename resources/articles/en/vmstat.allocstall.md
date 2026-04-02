# Allocation Stalls (allocstall) — vmstat

[日本語版](../ja/vmstat.allocstall.md)

---

## What is it?

When the kernel cannot find a free page for a memory allocation, it enters **direct reclaim** — it synchronously frees pages from the page cache or swaps out anonymous pages before satisfying the allocation. While it's doing this, the process that triggered the allocation is *blocked*.

`allocstall_*` counts how many times this happened per memory zone:

| Metric | Zone |
|--------|------|
| `allocstall_dma32` | DMA32 zone (< 4 GB physical) |
| `allocstall_normal` | Normal zone (typical RAM) |
| `allocstall_movable` | Movable zone (hot-pluggable memory) |
| `allocstall_device` | Device zone (GPU/device memory) |

---

## Why does it matter?

Each `allocstall` is a **synchronous stall** — your application is frozen while the kernel scrambles to free memory. This is different from kswapd (which reclaims in the background). Direct reclaim stalls are felt immediately as latency.

**What to watch:**
```
# Check stall rate over time
watch -n 5 'grep allocstall /proc/vmstat'
```

Rising `allocstall_normal` means normal RAM is under pressure. Combined with high `pressure/memory_some_avg10`, this confirms applications are being stalled.

**allocstall vs compact_stall:** `compact_stall` is specifically for huge page allocation failures. `allocstall` fires for *any* allocation that can't be satisfied from free pages.

---

## What to do

1. Check `meminfo.MemAvailable` — if below 10%, you're in danger territory
2. Check `pressure/memory_some_avg10` — confirms stalls are hurting applications  
3. Identify memory consumers: `ps aux --sort=-%mem | head -20`
4. If swap is involved: `grep 'pswpin\|pswpout' /proc/vmstat`

---

## See also

- `vmstat.compact` — compact_stall (huge page specific)
- `pressure.memory_some_avg10` — kernel-measured memory stall signal
- `meminfo.MemAvailable` — available memory headroom
- `sourceguide.vmstat` — full vmstat source overview

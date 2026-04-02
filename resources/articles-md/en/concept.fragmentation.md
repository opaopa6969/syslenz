# Fragmentation

What is this?
The state where free memory exists, but not in the sizes or locations that the kernel needs.

Why it matters
Fragmentation can break large allocations, huge pages, and contiguous DMA even when total free memory looks fine.

How to use
- Look at free blocks by order, not only total free bytes
- Check whether large allocations fail while small allocations succeed
- Compare long-lived memory patterns with recent spikes

Common mistakes
- Thinking fragmentation only matters when memory is nearly full
- Ignoring high-order allocation failures
- Confusing lack of capacity with lack of contiguity

Diagnostic flow
1. Confirm whether total memory is actually low.
2. Check whether high-order blocks are scarce.
3. Check whether the workload needs large contiguous chunks.
4. Decide whether compaction, reservation, or redesign is needed.

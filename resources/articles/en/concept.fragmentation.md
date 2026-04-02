# Memory Fragmentation

[日本語版](../ja/concept.fragmentation.md)

---

Physical memory fragmentation happens when free pages are scattered, preventing large contiguous allocations.

```
  Fragmented memory:              Contiguous memory:
  [used][free][used][free]        [used][used][free][free]
  [free][used][free][used]        [used][used][free][free]
  Cannot allocate 2MB page        Can allocate 2MB page ✓
```

**Why it matters:**
- THP (Transparent Huge Pages) requires 2MB contiguous blocks → `thp_fault_fallback`
- DMA devices need contiguous memory → kernel uses bounce buffers
- Over time, allocations fragment even initially clean memory

**How to check:**
```sh
# View fragmentation by order
cat /proc/buddyinfo
# High order-0, low order-10 = fragmented

# Current compaction activity
grep compact /proc/vmstat
```

**Fixes:**
- Memory compaction: `echo 1 > /proc/sys/vm/compact_memory`
- Increase huge page pool pre-allocation (pre-allocated = contiguous)
- Set THP to madvise/never to reduce compaction pressure

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

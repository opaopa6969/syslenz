# Memory Pressure

[日本語版](../ja/concept.memory-pressure.md)

---

**Memory pressure** is the state where the kernel is having to work hard to provide memory for new allocations.

**Stages of increasing memory pressure:**

```
  1. Plenty of memory
     → cache fills up, apps run normally

  2. Light pressure (MemAvailable dropping)
     → kernel starts evicting file cache (page reclaim)
     → kswapd wakes up occasionally
     
  3. Moderate pressure
     → kernel scans LRU aggressively
     → workingset refaults begin (cache miss on evicted pages)
     → pgscan_direct rises (foreground reclaim)
     
  4. Heavy pressure  
     → allocstall fires (processes blocked waiting for pages)
     → swap activity begins
     → pressure.memory_some_avg10 rises
     
  5. Critical
     → OOM killer fires
     → system may become unresponsive
```

**The key metric at each stage:**
Stage 1-2: `meminfo.MemAvailable`
Stage 3: `workingset_refault_file` + `pgscan_direct`
Stage 4: `allocstall_normal` + `vmstat.pswpout`
Stage 5: `vmstat.oom_kill`

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

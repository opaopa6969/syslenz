# Context Switch Cost

[日本語版](../ja/concept.context-switch-cost.md)

---

A **context switch** is when the CPU stops running one process and starts running another. It costs time and cache efficiency.

**What happens in a context switch:**
1. Save current process registers and state to memory
2. Update memory mappings (TLB flush if different address space)
3. Load next process state from memory
4. Resume execution

**Cost factors:**
- Kernel overhead: ~1-10 μs
- TLB invalidation: expensive for processes switching between different address spaces
- Cache warming: the new process's data isn't in CPU cache yet

**`stat.context_switches` vs normal:**
- Idle system: a few thousand/second (mostly kernel threads)
- Busy web server: 100k-500k/second (normal)
- 1M+/second: likely excessive (lock contention, over-threading)

**When context switches are a problem:**
High context switches with low CPU utilization = threads spending more time waiting than running. Often caused by:
- Too many threads competing for a lock
- Heavy use of system calls (kernel/user switches)
- I/O bound threads (blocking then waking repeatedly)

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

# Pressure Stall Information (PSI)

[日本語版](../ja/concept.pressure-stall.md)

---

**PSI (Pressure Stall Information)** is the most accurate way to measure resource pressure in Linux. Introduced in kernel 4.20.

Unlike utilization (which measures how busy a resource is), PSI measures **how much time tasks spent waiting** for a resource.

**Three resources:** CPU, memory, I/O

**Two measurements:**
- `some`: at least one task was stalled (partial impact)
- `full`: ALL tasks were stalled (total impact — system was blocked)

**Three time windows:** avg10 (10s), avg60 (1min), avg300 (5min)

**Why PSI is better than utilization:**
CPU at 100% utilization might mean 1 busy process and 99 others running fine. PSI shows whether tasks were actually delayed — not just whether the resource was busy.

**Alert thresholds (rough guidelines):**
- `cpu_some_avg10 > 50%`: CPU overloaded
- `memory_some_avg10 > 10%`: memory pressure
- `io_some_avg10 > 30%`: I/O bottleneck
- Any `_full_avg10 > 5%`: severe — all work stalled

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

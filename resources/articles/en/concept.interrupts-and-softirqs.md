# Interrupts and Software Interrupts

[日本語版](../ja/concept.interrupts-and-softirqs.md)

---

**Hardware interrupts (IRQs)** are signals from hardware to the CPU: "I have data for you." The CPU stops current work, saves state, runs the interrupt handler, then resumes.

**Softirqs** are deferred processing: the IRQ handler does the minimum (saves data, clears interrupt), then schedules a softirq to process it later. This keeps interrupt handlers fast.

```
  NIC receives packet:
  Hardware IRQ fires → interrupt handler runs (μs)
     ↓ schedules
  NET_RX softirq → processes packet, moves to socket buffer
```

**Why it matters:**
- `/proc/interrupts`: per-CPU IRQ counts
- `/proc/softirqs`: per-CPU softirq counts
- High softirq on one CPU = IRQ affinity imbalance → `irqbalance`
- `ksoftirqd` high CPU = softirq backlog (network or block I/O saturated)

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

# Hardware Interrupts

[日本語版](../ja/sourceguide.interrupts.md)

---

## What is it?

/proc/interrupts shows interrupt counts per CPU for each IRQ. Network cards, disk controllers, and timers all generate interrupts. Useful for diagnosing interrupt imbalance (IRQ affinity).

---

## Quick start

```sh
cat /proc/interrupts
# or use syslenz to browse with descriptions
```

---

## See also

- `stat.cpu_user`
- `cpuinfo.logical_cpus`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

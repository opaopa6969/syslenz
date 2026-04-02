# I/O Memory Map

[日本語版](../ja/sourceguide.iomem.md)

---

## What is it?

/proc/iomem shows the physical memory address space layout: RAM regions, BIOS, ACPI tables, PCI device memory, and MMIO regions. Useful for hardware debugging.

---

## Quick start

```sh
cat /proc/iomem
# or use syslenz to browse with descriptions
```

---

## See also

- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

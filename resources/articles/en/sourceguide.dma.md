# DMA Channels

[日本語版](../ja/sourceguide.dma.md)

---

## What is it?

/proc/dma lists ISA DMA channels in use. Relevant mainly on legacy x86 systems with ISA slots. Modern systems using PCIe show nothing here.

---

## Quick start

```sh
cat /proc/dma
# or use syslenz to browse with descriptions
```

---

## See also

- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

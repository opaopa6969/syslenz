# I/O Path: Syscall to Device

[日本語版](../ja/concept.io-path-syscall-to-device.md)

---

Understanding the I/O path helps diagnose where latency is hiding.

```
  Application: write(fd, buf, len)
       ↓
  System call interface (kernel entry)
       ↓
  VFS layer (file permissions, path lookup)
       ↓
  Filesystem (ext4: journal, allocation)
       ↓
  Page cache (dirty page — async write)
       ↓ (on fsync or writeback)
  Block layer (I/O scheduler: merge, sort)
       ↓
  Request queue (buffer if device busy)
       ↓
  Device driver (DMA setup)
       ↓
  Hardware (NVMe/SATA command)
       ↓
  Physical media (flash/magnetic)
```

**Where to look for latency:**
- VFS/FS: `stat.cpu_system` high, `iowait` low
- Page cache dirty: `vmstat.nr_dirty` high, flush storms
- Block queue: `diskstats.io_queue_depth_distribution` large
- Device: `diskstats.read_wait_distribution` high

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

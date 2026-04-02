# Driver, Kernel, OS Boundary

[日本語版](../ja/concept.driver-kernel-os-boundary.md)

---

Understanding what happens at different layers helps identify where problems originate.

```
  Application
      ↓ system call (read/write/ioctl)
  System Call Interface
      ↓
  VFS (Virtual File System)
      ↓
  Filesystem (ext4, xfs, btrfs)
      ↓
  Block Layer (I/O scheduler, queues)
      ↓
  Device Driver
      ↓
  Hardware (NVMe, SATA, network card)
```

**Why it matters for debugging:**
- High `stat.cpu_iowait` + low disk queue → filesystem problem
- High disk queue + low iowait → I/O scheduler or driver problem
- Driver errors in `dmesg` → hardware or driver bug

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

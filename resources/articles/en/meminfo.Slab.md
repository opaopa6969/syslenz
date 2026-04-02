# Kernel Slab Memory — meminfo

[日本語版](../ja/meminfo.Slab.md)

---

## What is it?

The kernel uses a **slab allocator** to efficiently allocate and cache frequently-used kernel objects: dentries (directory entries), inodes, network socket buffers, file descriptors, and thousands of others.

Slab memory is split into:
- **Reclaimable**: can be freed under memory pressure (dentry/inode caches)
- **Unreclaimable**: kernel data structures that must stay in memory

| Metric | What it is |
|--------|-----------|
| `Slab` | Total slab memory (SReclaimable + SUnreclaim) |
| `SReclaimable` | Reclaimable slab (dentry/inode caches) |
| `SUnreclaim` | Unreclaimable slab (socket buffers, kmalloc) |
| `KReclaimable` | All reclaimable kernel memory (includes SReclaimable) |

---

## Why does it matter?

**`SReclaimable` can be large and that's fine.** A server that traverses many directories builds a large dentry cache. This is memory that works hard and frees itself when needed.

**`SUnreclaim` growing** is more concerning — it means kernel objects that can't be freed are accumulating. Common causes:
- Network connections (socket buffers)
- Lots of open files
- Memory leaks in kernel modules

**Diagnosing slab usage:**
```sh
# Top slab consumers
sudo slabtop -o | head -20

# Or from /proc
sort -k3 -rn /proc/slabinfo | head -20
```

---

## See also

- `meminfo.KReclaimable` — all reclaimable kernel memory
- `vmstat.nr_slab_reclaimable` — vmstat version
- `sourceguide.meminfo` — full meminfo source overview

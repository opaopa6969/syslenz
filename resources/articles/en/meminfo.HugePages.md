# HugePages — meminfo

[日本語版](../ja/meminfo.HugePages.md)

---

## What is it?

Linux normally uses 4 KB pages. **Huge Pages** are pre-allocated large pages (typically 2 MB on x86_64) that are **locked in RAM** — they cannot be swapped out. Applications that manage large datasets (databases, JVMs with large heaps) explicitly request huge pages to reduce TLB pressure.

Unlike **Transparent Huge Pages (THP)** which are automatic, these are **explicit** huge pages. Applications use `mmap(MAP_HUGETLB)` or `shmget(SHM_HUGETLB)` to request them. The pool is configured at boot or via `sysctl`.

```
  /proc/sys/vm/nr_hugepages = 1000     # pre-allocate 1000 × 2MB = 2 GB
  
  HugePages_Total: 1000   ← configured
  HugePages_Free:   800   ← available for allocation
  HugePages_Rsvd:   100   ← reserved (promised but not yet used)
  HugePages_Surp:     0   ← excess (above configured, from overcommit pool)
```

---

## Metrics

| Metric | What it means |
|--------|---------------|
| `HugePages_Total` | Total huge pages in the pool |
| `HugePages_Free` | Unallocated huge pages |
| `HugePages_Rsvd` | Reserved but not yet mapped (committed) |
| `HugePages_Surp` | Surplus pages (from overcommit pool) |
| `Hugepagesize` | Size of each huge page (typically 2048 kB) |
| `Hugetlb` | Total memory used by all huge page pools |
| `AnonHugePages` | THP (transparent) anonymous huge pages |
| `ShmemHugePages` | Shared memory backed by huge pages |
| `FileHugePages` | File-backed memory using huge pages |

---

## Why does it matter?

**PostgreSQL and Oracle** often require huge pages to be configured. Without them, a 128 GB database shared buffer would be mapped with 32 million 4KB page table entries — significant memory overhead and TLB thrashing.

**Misconfiguration sign:** `HugePages_Free` near zero with applications failing to allocate. Check:
```sh
grep Huge /proc/meminfo
# Then compare with what your DB config requests
```

**`HugePages_Surp` > 0** means the kernel had to create pages beyond the pool — this is from the overcommit pool (`vm.nr_overcommit_hugepages`).

---

## Tuning

```sh
# Check current state
grep Huge /proc/meminfo

# Set pool size (2GB on a system with 2MB hugepages)
sysctl vm.nr_hugepages=1024

# Persistent (in /etc/sysctl.conf)
echo 'vm.nr_hugepages=1024' >> /etc/sysctl.conf

# Mount hugetlbfs for applications
mount -t hugetlbfs none /dev/hugepages
```

---

## See also

- `vmstat.thp` — Transparent Huge Pages (automatic, no pool)
- `meminfo.AnonHugePages` — THP in use (different from HugePages pool)
- `sourceguide.meminfo` — full meminfo source overview

# Unevictable Pages — vmstat

[日本語版](../ja/vmstat.unevictable_pgs.md)

---

## What is it?

Some memory pages **cannot be evicted** — the kernel cannot write them to disk or discard them. Pages become unevictable when:
- Locked with `mlock()` system call (real-time apps, cryptography)
- Backed by `ramfs` (in-memory filesystem, never goes to disk)
- Shared memory regions with `SHM_LOCK`

The kernel tracks these separately on the **unevictable LRU list** so it doesn't waste time scanning them during reclaim.

---

## Metrics

| Metric | What it counts |
|--------|---------------|
| `unevictable_pgs_culled` | Pages moved *to* unevictable list (newly locked) |
| `unevictable_pgs_scanned` | Unevictable pages checked during reclaim (should stay 0) |
| `unevictable_pgs_rescued` | Pages moved *from* unevictable back to LRU (unlocked) |
| `unevictable_pgs_mlocked` | Pages made unevictable via mlock() |
| `unevictable_pgs_munlocked` | Pages released from mlock() |
| `unevictable_pgs_cleared` | Pages cleared from unevictable list |
| `unevictable_pgs_stranded` | Pages on wrong LRU (bug indicator) |

---

## Why does it matter?

**`unevictable_pgs_scanned` > 0** is a warning — the kernel is scanning pages it knows it can't evict. This wastes CPU during memory pressure.

**Growing `unevictable_pgs_culled` without matching `rescued`** means locked memory is accumulating. Combined with `meminfo.Mlocked` being large, this reduces effective memory available for reclaim.

**`unevictable_pgs_stranded`** appearing is a kernel bug indicator — pages stuck on the wrong list.

```sh
# Check locked memory
grep Mlocked /proc/meminfo

# See which processes have locked memory
grep -r VmLck /proc/*/status 2>/dev/null | grep -v ' 0 kB'
```

---

## See also

- `meminfo.Unevictable` — total unevictable memory
- `meminfo.Mlocked` — mlock()-locked memory
- `sourceguide.vmstat` — full vmstat source overview

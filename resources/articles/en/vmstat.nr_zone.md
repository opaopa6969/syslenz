# Per-Zone Page Counts (nr_zone_*) — vmstat

[日本語版](../ja/vmstat.nr_zone.md)

---

## What is it?

These counters mirror the global `nr_active_anon`, `nr_inactive_file`, etc. metrics but broken down **per memory zone**. Memory zones are physical address regions with different properties:

- `dma32`: physical addresses below 4 GB (legacy DMA devices)
- `normal`: main system RAM above 4 GB
- `movable`: hot-pluggable or zone-restricted memory

| Metric | What it counts |
|--------|---------------|
| `nr_zone_inactive_anon` | Inactive anonymous pages in this zone |
| `nr_zone_active_anon` | Active anonymous pages in this zone |
| `nr_zone_inactive_file` | Inactive file-backed pages in this zone |
| `nr_zone_active_file` | Active file-backed pages in this zone |
| `nr_zone_unevictable` | Unevictable pages in this zone |
| `nr_zone_write_pending` | Pages with pending writes in this zone |

---

## Why does it matter?

For most monitoring purposes, the global `nr_active_anon`, `nr_inactive_file` etc. are sufficient. These per-zone variants are useful when:

1. Debugging NUMA issues — unusual imbalance between zones
2. Investigating DMA zone pressure (embedded/IoT systems)
3. Hot-plug memory debugging

Normal servers: `nr_zone_normal_*` holds almost all pages. `nr_zone_dma32_*` should be small.

---

## See also

- `vmstat.nr_active_inactive` — global active/inactive page counts
- `buddyinfo.zones` — zone free page fragmentation
- `sourceguide.vmstat` — full vmstat source overview

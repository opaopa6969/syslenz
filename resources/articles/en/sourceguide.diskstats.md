# Disk I/O Statistics

[日本語版](../ja/sourceguide.diskstats.md)

---

## What is it?

/proc/diskstats provides per-block-device I/O statistics: reads/writes completed, sectors transferred, I/O time, and queue depths. The primary source for storage performance analysis.

---

## Quick start

```sh
cat /proc/diskstats
# or use syslenz to browse with descriptions
```

---

## See also

- `pressure.io_some_avg10`
- `vmstat.nr_dirty`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

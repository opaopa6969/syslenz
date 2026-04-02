# Memory Zone Information

[日本語版](../ja/sourceguide.zoneinfo.md)

---

## What is it?

/proc/zoneinfo provides detailed per-zone memory statistics including watermarks (min/low/high), page counts, and per-cpu page caches. More detailed than buddyinfo.

---

## Quick start

```sh
cat /proc/zoneinfo
# or use syslenz to browse with descriptions
```

---

## See also

- `buddyinfo.zones`
- `vmstat.nr_free_pages`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

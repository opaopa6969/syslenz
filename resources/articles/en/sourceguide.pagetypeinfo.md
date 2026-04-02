# Page Type Information

[日本語版](../ja/sourceguide.pagetypeinfo.md)

---

## What is it?

/proc/pagetypeinfo breaks down free pages by movability type (Unmovable, Movable, Reclaimable) at each order. Used for diagnosing fragmentation issues in detail.

---

## Quick start

```sh
cat /proc/pagetypeinfo
# or use syslenz to browse with descriptions
```

---

## See also

- `buddyinfo.zones`
- `vmstat.compact`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

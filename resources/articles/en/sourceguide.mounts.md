# Mounted Filesystems

[日本語版](../ja/sourceguide.mounts.md)

---

## What is it?

/proc/mounts (or /proc/self/mounts) lists all currently mounted filesystems with their options. Essential for understanding storage layout and checking mount options.

---

## Quick start

```sh
cat /proc/mounts
# or use syslenz to browse with descriptions
```

---

## See also

- `df.filesystems`
- `partitions.partitions`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

# Supported Filesystems

[日本語版](../ja/sourceguide.filesystems.md)

---

## What is it?

/proc/filesystems lists all filesystem types the kernel supports. Useful for checking if a needed filesystem type (overlayfs, btrfs, etc.) is available.

---

## Quick start

```sh
cat /proc/filesystems
# or use syslenz to browse with descriptions
```

---

## See also

- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

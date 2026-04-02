# Disk Partitions

[日本語版](../ja/sourceguide.partitions.md)

---

## What is it?

/proc/partitions lists all block device partitions known to the kernel: major/minor numbers, size, and name. Includes physical disks, LVM volumes, and device mapper devices.

---

## Quick start

```sh
cat /proc/partitions
# or use syslenz to browse with descriptions
```

---

## See also

- `df.filesystems`
- `diskstats.devices`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

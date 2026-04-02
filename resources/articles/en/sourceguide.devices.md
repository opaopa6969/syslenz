# Device Classes

[日本語版](../ja/sourceguide.devices.md)

---

## What is it?

/proc/devices lists the character and block device major numbers registered on this kernel. Major numbers map device drivers to device files in /dev.

---

## Quick start

```sh
cat /proc/devices
# or use syslenz to browse with descriptions
```

---

## See also

- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

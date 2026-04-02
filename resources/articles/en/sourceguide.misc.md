# Misc Character Devices

[日本語版](../ja/sourceguide.misc.md)

---

## What is it?

/proc/misc shows miscellaneous character devices registered under major number 10: /dev/random, /dev/fuse, /dev/kvm, etc. Indicates which kernel subsystems are active.

---

## Quick start

```sh
cat /proc/misc
# or use syslenz to browse with descriptions
```

---

## See also

- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

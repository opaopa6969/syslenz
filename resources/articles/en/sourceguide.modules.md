# Loaded Kernel Modules

[日本語版](../ja/sourceguide.modules.md)

---

## What is it?

/proc/modules lists all currently loaded kernel modules with their sizes, reference counts, and dependencies. Useful for security auditing and driver debugging.

---

## Quick start

```sh
cat /proc/modules
# or use syslenz to browse with descriptions
```

---

## See also

- `version.kernel_version`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

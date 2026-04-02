# Kernel Command Line

[日本語版](../ja/sourceguide.cmdline.md)

---

## What is it?

/proc/cmdline shows the parameters passed to the kernel at boot time. Useful for understanding boot configuration: hugepages, cgroup version, security modules, etc.

---

## Quick start

```sh
cat /proc/cmdline
# or use syslenz to browse with descriptions
```

---

## See also

- `meminfo.HugePages`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

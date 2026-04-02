# CPU Information

[日本語版](../ja/sourceguide.cpuinfo.md)

---

## What is it?

/proc/cpuinfo describes the CPUs on this system: vendor, model, frequency, cache sizes, NUMA topology, and feature flags. Essential for understanding hardware capabilities.

---

## Quick start

```sh
cat /proc/cpuinfo
# or use syslenz to browse with descriptions
```

---

## See also

- `vmstat.numa`
- `stat.cpu_user`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

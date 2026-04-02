# Control Groups (cgroups)

[日本語版](../ja/sourceguide.cgroups.md)

---

## What is it?

/proc/cgroups lists the cgroup controllers available on this kernel (memory, cpu, blkio, etc.) and how many cgroups are using each. Cgroups isolate and limit resources per process group.

---

## Quick start

```sh
cat /proc/cgroups
# or use syslenz to browse with descriptions
```

---

## See also

- `processes.process_count`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

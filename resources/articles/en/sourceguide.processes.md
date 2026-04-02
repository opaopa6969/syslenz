# Process List

[日本語版](../ja/sourceguide.processes.md)

---

## What is it?

Process information from /proc/[pid]/: all running processes with name, state, RSS memory, thread count, and UID. This is the data behind ps and top.

---

## Quick start

```sh
cat /proc/procesys/blockes
# or use syslenz to browse with descriptions
```

---

## See also

- `stat.procs_running`
- `meminfo.MemAvailable`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

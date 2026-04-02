# Scheduler Statistics

[日本語版](../ja/sourceguide.schedstat.md)

---

## What is it?

/proc/schedstat shows per-CPU scheduler statistics: run queue time, wait time, and context switches. Useful for diagnosing CPU scheduling fairness and wait times.

---

## Quick start

```sh
cat /proc/schedstat
# or use syslenz to browse with descriptions
```

---

## See also

- `stat.context_switches`
- `pressure.cpu_some_avg10`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

# Kernel File Locks

[日本語版](../ja/sourceguide.locks.md)

---

## What is it?

/proc/locks shows kernel-level file locks: POSIX locks (fcntl), BSD locks (flock), and mandatory locks. Useful for debugging lock contention or stuck processes.

---

## Quick start

```sh
cat /proc/locks
# or use syslenz to browse with descriptions
```

---

## See also

- `processes.process_count`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

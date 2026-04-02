# Kernel Timer List

[日本語版](../ja/sourceguide.timer_list.md)

---

## What is it?

/proc/timer_list shows all pending kernel timers. Useful for debugging timer-related issues and understanding scheduled kernel work.

---

## Quick start

```sh
cat /proc/timer_list
# or use syslenz to browse with descriptions
```

---

## See also

- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

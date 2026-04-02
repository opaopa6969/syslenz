# Swap Space

[日本語版](../ja/sourceguide.swaps.md)

---

## What is it?

/proc/swaps lists all swap devices and files: their sizes and how much is used. Multiple swap areas can be configured with different priorities.

---

## Quick start

```sh
cat /proc/swaps
# or use syslenz to browse with descriptions
```

---

## See also

- `meminfo.SwapFree`
- `vmstat.swap`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

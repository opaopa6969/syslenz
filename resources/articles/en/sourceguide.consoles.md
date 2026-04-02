# Console Devices

[日本語版](../ja/sourceguide.consoles.md)

---

## What is it?

/proc/consoles shows the console devices currently active on this system (ttyS0, tty0, hvc0, etc.). Useful for understanding serial/virtual console configuration.

---

## Quick start

```sh
cat /proc/consoles
# or use syslenz to browse with descriptions
```

---

## See also

- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

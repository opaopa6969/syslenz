# I/O Port Map

[日本語版](../ja/sourceguide.ioports.md)

---

## What is it?

/proc/ioports shows the I/O port address space used by hardware devices. Mainly relevant for legacy x86 hardware with ISA ports.

---

## Quick start

```sh
cat /proc/ioports
# or use syslenz to browse with descriptions
```

---

## See also

- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

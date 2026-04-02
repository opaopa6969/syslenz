# CPU Thermal Sensors

[日本語版](../ja/sourceguide.thermal.md)

---

## What is it?

CPU thermal zone temperatures from /sys/class/thermal/. Critical for catching thermal throttling before it impacts performance.

---

## Quick start

```sh
cat /proc/thermal
# or use syslenz to browse with descriptions
```

---

## See also

- `cpuinfo.frequency`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

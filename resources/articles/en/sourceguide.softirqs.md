# Software Interrupts

[日本語版](../ja/sourceguide.softirqs.md)

---

## What is it?

/proc/softirqs shows cumulative software interrupt counts per CPU: NET_RX, NET_TX, BLOCK, TIMER, RCU, etc. High softirq rates on one CPU indicate interrupt affinity issues.

---

## Quick start

```sh
cat /proc/softirqs
# or use syslenz to browse with descriptions
```

---

## See also

- `interrupts.interrupts`
- `stat.cpu_user`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

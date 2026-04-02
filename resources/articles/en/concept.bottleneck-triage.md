# Bottleneck Triage

[日本語版](../ja/concept.bottleneck-triage.md)

---

Finding the bottleneck requires a systematic approach. The four resource types — CPU, memory, storage I/O, and network — each have distinct signatures.

**Step 1: Check pressure stall information (PSI)**
```sh
cat /proc/pressure/cpu
cat /proc/pressure/memory
cat /proc/pressure/io
```
PSI directly tells you which resource tasks are waiting for. Non-zero `full` values mean all tasks were blocked — severe.

**Step 2: Match symptoms to resources**

| Symptom | Likely resource | Key metric |
|---------|----------------|-----------|
| High load, low CPU% | I/O wait | `stat.cpu_iowait` |
| Slow response, normal load | Memory swap | `vmstat.pswpin` |
| 100% CPU | CPU bound | `stat.cpu_user` |
| Network timeouts | Network congestion | `net/snmp.Tcp_RetransSegs` |

**Step 3: Use USE method**
For each resource: Utilization, Saturation, Errors.
- Utilization: how busy is it?
- Saturation: is there a queue forming?
- Errors: are things failing?

**Step 4: Look one level deeper**
CPU bottleneck → which process? (`processes.processes`)
Memory bottleneck → who leaked? (`meminfo.AnonPages` + `ps aux`)
I/O bottleneck → which device? (`diskstats.devices`)

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

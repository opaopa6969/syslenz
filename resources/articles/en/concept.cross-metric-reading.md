# Cross-Metric Reading

[日本語版](../ja/concept.cross-metric-reading.md)

---

Single metrics rarely tell the whole story. Experienced operators read metrics in groups.

**Memory pressure trio:**
```
meminfo.MemAvailable (dropping) +
vmstat.pswpout (rising) +
pressure.memory_some_avg10 (> 0)
= Active swap with application impact confirmed
```

**I/O problem trio:**
```
stat.cpu_iowait (high) +
pressure.io_some_avg10 (high) +
diskstats.io_queue_depth_distribution (large)
= Storage bottleneck with CPU stalls
```

**Network problem trio:**
```
net/snmp.Tcp_RetransSegs (rising) +
net/netstat.TcpExt_TCPTimeouts (rising) +
net/netstat.TcpExt_ListenDrops (rising)
= Packet loss + application-level TCP pressure
```

**The principle:** Look for *corroboration*. A single high metric could be noise. Three related metrics all pointing the same direction = signal.

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

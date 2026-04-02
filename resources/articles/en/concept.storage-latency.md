# Storage Latency

[日本語版](../ja/concept.storage-latency.md)

---

Storage latency is one of the most common sources of application performance problems.

**Latency hierarchy:**
```
  L1 cache:    ~1ns
  DRAM:        ~100ns
  NVMe SSD:    ~100μs   (1,000x slower than DRAM)
  SATA SSD:    ~500μs   (5,000x slower)
  HDD (7200):  ~10ms    (100,000x slower)
  HDD (seek):  ~15ms
```

**Where to look:**
- `diskstats.read_wait_distribution` — p99 read latency per device
- `diskstats.write_wait_distribution` — p99 write latency
- `diskstats.io_queue_depth_distribution` — queue depth (> 1 = latency impact)
- `pressure.io_some_avg10` — % time tasks stalled on I/O

**Causes of high storage latency:**
1. Device saturation (IOPS limit reached)
2. I/O queue too deep (queueing delay)
3. Write pressure (`vmstat.nr_dirty` spikes)
4. Software RAID rebuild
5. Thermal throttling (NVMe drives get hot)

```sh
# Check device latency in real-time
iostat -x 1
# Look at: await (average wait) and %util (device utilization)
```

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

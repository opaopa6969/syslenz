# CPU and I/O Throttling

[日本語版](../ja/concept.throttling.md)

---

**Throttling** is when the kernel artificially limits a process's access to a resource. Unlike saturation (resource at capacity), throttling is a configured limit.

**CPU throttling (cgroups):**
```sh
# Check if a cgroup has CPU limits
cat /sys/fs/cgroup/cpu/myservice/cpu.cfs_quota_us
# -1 = no limit, positive number = throttled
```
If a container's cpu.cfs_quota_us is hit, processes are paused — this shows as higher latency without high CPU%

**I/O throttling (blkio):**
```sh
# Check blkio limits
cat /sys/fs/cgroup/blkio/myservice/blkio.throttle.read_bps_device
```

**Write throttling:**
When `vmstat.nr_dirty` exceeds `dirty_ratio`, the kernel blocks application writes — this is implicit throttling. `nr_throttled_written` counts pages written during this.

**Thermal throttling:**
CPU reduces frequency when too hot — shows as `cpuinfo.frequency` dropping below base clock.

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

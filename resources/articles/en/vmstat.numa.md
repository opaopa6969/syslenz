# NUMA Allocation Statistics — vmstat

[日本語版](../ja/vmstat.numa.md)

---

## What is it?

On servers with multiple CPU sockets, each socket has its own bank of RAM — this is **NUMA (Non-Uniform Memory Access)**. Accessing memory on your local socket is fast (~100ns). Accessing memory on a remote socket crosses the QPI/UPI interconnect, which is 2–3× slower.

```
  Socket 0              Socket 1
  ┌─────────┐           ┌─────────┐
  │  CPU 0  │──QPI/UPI──│  CPU 1  │
  │  RAM A  │           │  RAM B  │
  └─────────┘           └─────────┘
  Local access: fast    Remote access: 2-3x slower
```

These vmstat counters tell you how many allocations hit the right node vs. the wrong one.

---

## Metrics

| Metric | What it means |
|--------|---------------|
| `numa_hit` | Allocation succeeded on the intended node |
| `numa_miss` | Allocation landed on a different node (remote) |
| `numa_foreign` | Allocation intended for this node but placed elsewhere |
| `numa_local` | Allocation on the node where the process ran |
| `numa_other` | Allocation on a node different from where the process ran |
| `numa_interleave` | Allocation using interleave policy |

**Miss rate = `numa_miss / (numa_hit + numa_miss)`**

---

## Why does it matter?

A high miss rate means your workload is accessing memory across NUMA nodes. This causes:
- Memory latency 2–3× higher than necessary
- QPI/UPI interconnect saturation on heavily loaded systems
- Inconsistent performance as processes migrate between CPUs

**Common causes:**
1. JVM with a large heap spanning both nodes
2. Application threads on CPU 0 accessing data allocated when running on CPU 1
3. `numactl` not configured, relying on first-touch allocation with CPU migration

```sh
# Check NUMA topology
numactl --hardware

# Check per-node memory allocation
numastat -m

# Run process with NUMA affinity
numactl --cpunodebind=0 --membind=0 ./myapp
```

---

## See also

- `cpuinfo.cores_per_socket` — socket/core topology
- `sourceguide.vmstat` — full vmstat source overview

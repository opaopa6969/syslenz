# Cached

[日本語版](../ja/meminfo.Cached.md)

---

## What is it?

`Cached` is the amount of RAM used as a page cache for files read from or written to disk. When your application opens a file, the kernel reads it into RAM and keeps it there. The next time anyone reads the same file, no disk access is needed — the data comes straight from RAM.

Think of it as the kernel's automatic disk speed-up. The kernel decides what to cache and what to evict without any application input. When memory pressure rises, cache pages are the first to be reclaimed — they're "clean" copies of data that exists on disk, so discarding them costs nothing except a future disk read.

```
  First read: file → disk I/O → page cache → application
  Second read: file → page cache → application  (no disk!)

  When memory is needed:
  page cache pages → evicted → MemFree grows → application gets RAM
  (the data still exists on disk, nothing is lost)
```

---

## Why does it matter?

**Large `Cached` is usually good.** A database server with 50 GB `Cached` is reading its working set from RAM, not disk. Read latency is microseconds instead of milliseconds. That 50 GB isn't "wasted" — it's doing work.

**`Cached` dropping suddenly is bad.** If another workload needs memory and the kernel evicts your cache, the next reads become slow disk I/O. This is the "cold start" effect: a service that was running at sub-millisecond latency suddenly takes 50–200ms per operation while its working set reloads.

**Typical patterns to recognize:**

| `Cached` behavior | What it means |
|-------------------|---------------|
| Large and stable | Good — working set is warm |
| Large and dropping | Memory pressure — something else needs RAM |
| Small and growing | System warming up after restart |
| Near zero | Cache evicted — expect slow I/O until it warms |

---

## How to read it

```sh
# Check cache size alongside availability
grep -E "MemTotal|MemAvailable|Cached|Buffers" /proc/meminfo

# Watch cache and available together
watch -n 2 'grep -E "MemAvailable|Cached:" /proc/meminfo'

# See how much is actually being read from disk vs cache
# (look at "pgpgin" — pages read from disk)
grep pgpgin /proc/vmstat
```

**On a database server,** the ratio of cache hits to total reads is the key metric. If `Cached` is large and your DB reports high buffer hit rates, you're in a good state.

**Rule of thumb:** on a server dedicated to one workload, having `Cached` use 60–80% of total RAM is healthy. Only worry if `MemAvailable` is dropping.

---

## A real episode

A company ran an Elasticsearch cluster. Each node had 32 GB RAM. For months, queries ran at < 10ms average. Then they added a new log ingestion pipeline that ran on the same nodes.

The ingestion pipeline wrote multi-GB log files. The kernel was caching them, which steadily evicted Elasticsearch's index pages from cache. Over two days, `Cached` content shifted from Elasticsearch indexes to log files. Query latency climbed from 10ms to 400ms. No crash, no OOM — just a slow, quiet performance erosion.

The fix was separating workloads onto different nodes. But the diagnosis required understanding that cache isn't neutral — it has content, and that content matters.

The lesson: if latency mysteriously rises on a database or search service, check whether something else is evicting its cache. `Cached` in `/proc/meminfo` tells you the size, but `fincore` or `vmtouch` can show you what's actually cached.

---

## What to do when cache drops suddenly

**Step 1: Identify what's consuming memory instead.**
```sh
# Check AnonPages — is an app consuming more RAM?
grep AnonPages /proc/meminfo

# Top RSS consumers
ps aux --sort=-%mem | head -10
```

**Step 2: Check if I/O latency has increased.**
```sh
iostat -x 1 5
# Look at %util and await columns — are disks busy?
```

**Step 3: Identify what was cached.**
If you suspect a specific file's cache was evicted, you can check with `fincore`:
```sh
# Install: apt install libfincore / fincore <filename>
fincore /var/lib/postgresql/data/base/...
```

**Step 4: If workload competition is the cause,** consider cgroup memory limits to reserve cache for critical services, or separate workloads onto different hosts.

---

## Common mistakes

**Trying to "free up" cache to get more memory.** The kernel does this automatically when memory is needed. Manually dropping cache (`drop_caches`) just causes unnecessary cache misses.

**Confusing `Cached` with `Buffers`.** `Cached` is page cache for file content. `Buffers` is block device metadata (filesystem structures, not file data). They're different pools with different behavior.

**Mistaking cold cache for a crash.** After a server restart, `Cached` starts near zero. Services will be slow until cache warms. This is normal — not a bug.

**Ignoring cache as a performance signal.** Falling `Cached` on a database server is an early warning. Don't wait until latency spikes — watch `Cached` trend over time.

---

## See also

- `meminfo.MemAvailable` — the actual memory headroom (includes reclaimable cache)
- `meminfo.Buffers` — block device metadata cache (separate from file cache)
- `meminfo.AnonPages` — application memory that cannot be reclaimed
- `meminfo.MemFree` — truly idle memory (usually a misleading metric)
- `vmstat.pgpgin` / `pgpgout` — pages read/written from disk (cache miss rate proxy)

# MemFree

[日本語版](../ja/meminfo.MemFree.md)

---

## What is it?

`MemFree` is the amount of RAM not being used by anything — not by applications, not by the kernel, not even by file cache. It's memory that the kernel considers completely idle.

On a healthy, busy Linux server, `MemFree` is almost always low. That's intentional.

```
  Windows / macOS mindset:
  ┌──────────────────────────────────┐
  │ Used  ████████████  8 GB         │
  │ Free  ░░░░░░░░░░░░  8 GB  ← good │
  └──────────────────────────────────┘

  Linux reality:
  ┌──────────────────────────────────┐
  │ Apps  ████  4 GB                 │
  │ Cache ████████████  11 GB ← good │
  │ Free  ░  1 GB        ← also fine │
  └──────────────────────────────────┘
```

Linux aggressively uses idle RAM as file cache. The kernel's philosophy is: **free memory is wasted memory**. If a page isn't caching something useful, it's a missed opportunity to speed up the next disk read.

---

## Why does it matter?

Mostly, it doesn't — and that's the point.

`MemFree` becomes meaningful only in a few specific situations:

**Genuine memory exhaustion.** If both `MemFree` and `MemAvailable` are near zero simultaneously, the system is out of memory. But this is a last resort signal — `MemAvailable` will warn you long before `MemFree` reaches zero.

**After a `drop_caches`.** If you forcibly evict the page cache, `MemFree` will spike. That's just the cache returning to free memory — it will be consumed again shortly.

**On systems with no swap and tiny cache.** A container with 512 MB RAM and no swap may legitimately have a tiny `MemFree` — here it does indicate pressure.

In the vast majority of cases, if you're looking at `MemFree` and worried, **you should be looking at `MemAvailable` instead**.

---

## How to read it

```sh
# Don't read MemFree alone. Read this:
grep -E "MemTotal|MemFree|MemAvailable|Cached" /proc/meminfo

# Or use free(1) — "available" is what matters
free -h
```

The rule: if `MemAvailable` is healthy (> 20% of MemTotal), a low `MemFree` is completely normal.

---

## A real episode

A junior engineer joined a company running a busy PostgreSQL server. His first task was a health check. He ran `free -m`, saw `MemFree: 247 MB` on a 64 GB machine, and filed an urgent incident: "Server is critically low on memory."

The DBA on call looked at it and pointed out that `MemAvailable` was 42 GB — the system had 42 GB ready to hand out. The "missing" memory was 58 GB of PostgreSQL's working set sitting in page cache, ready to serve queries without touching disk.

The junior engineer had fallen into the most common Linux memory mistake: confusing "free" with "available." He fixed his mental model and eventually became a good systems engineer. But for one tense hour, everyone thought the database was about to crash.

The lesson: install the Linux memory model before you install the monitoring tool.

---

## Common mistakes

**Using `MemFree` as a health indicator.** It isn't one. Use `MemAvailable`.

**Freeing cache to "fix" low MemFree.** `echo 3 > /proc/sys/vm/drop_caches` will make `MemFree` jump, but it destroys cache that was actively helping performance. You haven't fixed anything — you've made the system slower.

**Alerting on low MemFree.** This generates constant false alarms on healthy systems. Alert on `MemAvailable` percentage instead.

**Confusing `MemFree` with unused capacity.** The cache is capacity — it just looks "used." If an application needs memory, the kernel will reclaim cache pages in milliseconds.

---

## See also

- `meminfo.MemAvailable` — the metric you actually want to watch
- `meminfo.Cached` — the file cache consuming most of what looks like "used" memory
- `meminfo.AnonPages` — actual application memory (not reclaimable)

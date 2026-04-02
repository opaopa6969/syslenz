# AnonPages

[日本語版](../ja/meminfo.AnonPages.md)

---

## What is it?

`AnonPages` is the memory used by application heap, stack, and private memory-mapped regions — anything that isn't backed by a file on disk. When your application calls `malloc()`, allocates a Python object, or grows its heap, those pages show up here.

"Anonymous" means there's no file behind the page. If the kernel needs to reclaim it, it can't just discard it (there's no disk copy) — it must write it to swap first, or kill the process.

```
  File cache (Cached):
    App reads /etc/config → page backed by disk → evictable for free

  Anonymous pages (AnonPages):
    App does malloc(1MB) → page backed by nothing → NOT evictable
    (must be swapped or the process killed to reclaim)
```

`AnonPages` is the closest thing `/proc/meminfo` has to "memory actually consumed by applications."

---

## Why does it matter?

**It's what the OOM killer is really looking at.** When memory runs out, the kernel can't evict anonymous pages without swapping or killing. High `AnonPages` means the kernel has less maneuvering room.

**It's your leak detector.** If `AnonPages` grows steadily over hours while request load is flat, something is leaking. `Cached` fluctuates naturally; `AnonPages` should be relatively stable on a stable workload.

**It tells you whether MemAvailable's drop is real.** If `MemAvailable` is falling and `AnonPages` is rising at the same rate, applications are genuinely consuming more memory. If `MemAvailable` is falling but `AnonPages` is stable, cache behavior is likely the cause.

---

## How to read it

```sh
# Watch AnonPages trend
watch -n 5 'grep -E "AnonPages|MemAvailable|SwapFree" /proc/meminfo'

# Check per-process anonymous memory (VmRSS ≈ anon + file pages)
cat /proc/<pid>/status | grep -E "VmRSS|VmAnon|RssAnon"

# All processes, sorted by RSS
ps aux --sort=-%rss | head -20
```

**Interpreting trends:**

| AnonPages behavior | What it means |
|--------------------|---------------|
| Stable | Healthy — apps using steady memory |
| Slowly rising over hours | Possible leak — correlate with request count |
| Rapidly rising | Load spike or runaway allocation |
| Falling | App freed memory, or process exited |

---

## A real episode

A Python web service processed image uploads. The developer had profiled memory usage under normal load — everything looked fine. After deploying to production, `AnonPages` grew 200 MB/hour.

The team watched `MemAvailable` fall for a day before someone noticed the trend. By that point, `AnonPages` had grown from 2 GB to 7 GB. They looked at per-process RSS with `ps` and found the web worker processes were all holding large amounts of memory.

The culprit: a PIL (Pillow) image object was being cached in a module-level dict for "performance." The cache had no eviction policy. Every unique image size that came through created a permanent entry. The dict grew forever.

Fixing it: add a max-size cache with LRU eviction. `AnonPages` stabilized within minutes of deployment.

The lesson: `AnonPages` growing on a stable workload is a leak. Find it with per-process RSS tracking over time, not just a snapshot.

---

## How to find a memory leak

**Step 1: Confirm it's not load-driven.**
```sh
# Compare AnonPages growth against request rate
# If AnonPages grows even when requests are flat, it's a leak
grep AnonPages /proc/meminfo  # sample every few minutes
```

**Step 2: Find the leaking process.**
```sh
# Snapshot RSS of all processes, wait 10 minutes, snapshot again
ps -eo pid,comm,rss --sort=-rss | head -20
# Which process is growing?
```

**Step 3: Confirm with /proc/<pid>/status.**
```sh
watch -n 30 'cat /proc/<pid>/status | grep -E "VmRSS|VmSize|RssAnon"'
```
`RssAnon` is exactly what `AnonPages` counts for that process.

**Step 4: Profile the leaking process.**
For interpreted languages (Python, Ruby, Node.js), use their built-in heap profilers. For C/C++, use `valgrind --leak-check=full` or `heaptrack`. For Go, use `pprof`.

---

## Common mistakes

**Confusing `AnonPages` with total RSS.** A process's RSS includes both anonymous pages and file-backed pages (shared libraries, mmapped files). `RssAnon` in `/proc/<pid>/status` shows just the anonymous portion.

**Blaming cache for a memory problem.** If `MemAvailable` is low but `AnonPages` is small, the issue is cache behavior, not a leak. Don't restart services until you've confirmed which is which.

**Taking a single snapshot.** Memory leaks are trends, not values. A one-time `AnonPages = 5 GB` tells you nothing without context.

**Ignoring swap.** If `AnonPages` is high and swap is being used, your applications are effectively running in slow motion. Check `vmstat`'s `si`/`so` fields.

---

## See also

- `meminfo.MemAvailable` — overall memory headroom
- `meminfo.SwapFree` — if AnonPages is high, swap is the safety net (and it's slow)
- `meminfo.Cached` — file-backed memory that CAN be reclaimed (contrast with AnonPages)
- `vmstat.pgpgout` — pages written to disk (includes swap-outs of anonymous pages)

# Buffers

[日本語版](../ja/meminfo.Buffers.md)

---

## What is it?

`Buffers` is the memory used to cache metadata about block devices — things like directory entries (dentries), inode tables, and filesystem structure information. It does NOT include the content of files (that's `Cached`).

Think of `Buffers` as the kernel's map of the filesystem, while `Cached` is the actual file contents. When the kernel needs to look up where a file lives on disk, it checks `Buffers` first. When it needs the file's data, it checks `Cached`.

```
  Filesystem lookup: "where is /var/log/app.log on disk?"
    → Check Buffers (dentry/inode cache)
    → Found: inode 4821, block 0x1F3A...

  File read: "give me the content of block 0x1F3A"
    → Check Cached (page cache)
    → Found in RAM, no disk read needed
```

On modern Linux kernels, `Buffers` primarily holds inode and dentry metadata. The older distinction between "block layer buffers" and "page cache" has largely merged — but `Buffers` still appears as a separate line in `/proc/meminfo`.

---

## Why does it matter?

**Mostly, it's background noise.** `Buffers` is typically small — a few hundred MB at most on most systems — and reclaimable. The kernel frees it under pressure just like file cache.

**What to watch for:**

- **Normal range**: a few MB to a few hundred MB. On servers with many files and directories (like a build server or fileserver), a few GB is possible but uncommon.
- **Abnormally large Buffers** (multi-GB on a typical app server): something is scanning many files or directories, keeping metadata warm. Backup jobs, file indexing, or runaway directory traversal are common causes.
- **Buffers ≠ Cached**: a common confusion. If you see total RAM consumption that doesn't add up, check both.

---

## How to read it

```sh
# Check Buffers alongside Cached and MemAvailable
grep -E "MemAvailable|Buffers|Cached" /proc/meminfo

# Note: "Cached:" in /proc/meminfo excludes Buffers
# free(1) shows "buff/cache" which combines both
free -h
```

**The `free` output:**

```
              total        used        free      shared  buff/cache   available
Mem:           15Gi       4.2Gi       1.1Gi       256Mi       9.7Gi        10Gi
Swap:           4Gi          0B        4Gi
```

`buff/cache` combines `Buffers + Cached`. Both are reclaimable. `available` (MemAvailable) already accounts for both.

---

## A real episode

A team ran a nightly backup job that used `rsync` to sync a large NFS share — about 2 million files across 500,000 directories. After the job ran, `Buffers` jumped from 80 MB to 4.2 GB. The monitoring system flagged it as "high memory usage."

The on-call engineer restarted several services unnecessarily. The memory pressure wasn't real: `MemAvailable` had barely moved. The `Buffers` growth was just the dentry/inode cache warming up from scanning 2.5 million filesystem entries. Within an hour of the backup finishing, `Buffers` drained back to normal as the kernel reclaimed those pages for other uses.

The lesson: check `MemAvailable`, not individual components. Large `Buffers` from a filesystem scan is transient and harmless.

---

## Common mistakes

**Confusing `Buffers` with `Cached`.** `Buffers` = filesystem metadata. `Cached` = file content. The `free` command combines them as "buff/cache" which is correct for "how much is reclaimable" but obscures the distinction.

**Alarming on `Buffers` size alone.** A few GB of `Buffers` on a file-heavy workload is normal. Check `MemAvailable` to see if it's actually impacting headroom.

**Trying to control `Buffers` directly.** You can't tune it separately. The kernel manages dentry/inode cache size through `vfs_cache_pressure` (`/proc/sys/vm/vfs_cache_pressure`). Default is 100; higher values make the kernel reclaim inode/dentry cache more aggressively.

---

## See also

- `meminfo.Cached` — file content cache (the larger reclaimable pool; often confused with Buffers)
- `meminfo.MemAvailable` — total available memory including both Buffers and Cached
- `meminfo.SReclaimable` — reclaimable slab memory (includes dentry/inode cache tracked via slab)

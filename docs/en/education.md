---
version: v1.0.0
lang: en
---

# Education: Category Guides and Help Levels

[🇯🇵 日本語版](../ja/education.md)

[<- Prev: Diagnostics](diagnostics.md) | [Index](index.md) | [Next: Remote Monitoring ->](remote.md)


## Table of Contents

- [Overview](#overview)
- [Help Levels](#help-levels)
- [Category Guide](#category-guide)
- [Memory Story](#memory-story)
- [CPU Story](#cpu-story)
- [Network Story](#network-story)
- [Storage Story](#storage-story)
- [Process Story](#process-story)
- [Learning Paths](#learning-paths)

## Overview

syslenz is not just a monitoring tool -- it is a teaching tool. The built-in educational system helps you learn Linux internals by providing structured narratives, diagnostic flowcharts, and common issue patterns for each major subsystem. Combined with live data from your own system, this makes abstract kernel concepts concrete.

## Help Levels

Press `?` to cycle through four help levels:

| Level | Label | What It Shows |
|-------|-------|---------------|
| OFF | `OFF` | No help panel |
| NORMAL | `NORMAL` | Basic keybinding reference and current view description |
| DETAILED | `DETAILED` | Field descriptions, units, and interpretation tips for the selected source |
| EXTRA | `EXTRA` | Extended explanation including kernel background, related files, and cross-references to other sources |

The help panel appears at the bottom of the screen and scrolls with `j`/`k` when focused. Use `PageUp`/`PageDown` for faster scrolling.

## Category Guide

Press `C` to open the Category Guide view. This provides deep educational content organized by subsystem:

| Category | Icon | Related Sources |
|----------|------|----------------|
| **Memory** | MEM | meminfo, vmstat, swaps, buddyinfo, pressure, zoneinfo, slabinfo, pagetypeinfo |
| **CPU / Load** | CPU | stat, loadavg, cpuinfo, pressure, schedstat, softirqs, interrupts |
| **Network** | NET | net/dev, net/tcp, net/udp, net/unix, net/arp, net/route, net/sockstat, net/snmp, net/netstat, net/wireless, dns, conntrack, ip_route, ip_neighbor, ss_summary |
| **Storage** | DSK | diskstats, df, mounts, partitions, pressure, locks |
| **Process** | PRC | processes, file-nr, stat |

### Navigation

| Key | Action |
|-----|--------|
| `h` / `l` or Arrow Left/Right | Switch between categories |
| `j` / `k` or Arrow Down/Up | Scroll content |
| `PageUp` / `PageDown` | Scroll by page |
| `Backspace` | Return to previous view |

Each category page contains four sections:
1. **Overview** -- What this subsystem does and why it matters
2. **Story** -- A narrative walkthrough ("Where did all my RAM go?")
3. **Diagnostic Flow** -- Step-by-step decision tree for troubleshooting
4. **Common Issues** -- Frequently encountered problems and solutions

## Memory Story

**"Where did all my RAM go?"**

The Memory guide walks you through the most misunderstood aspect of Linux: memory management.

**Key insights:**
- On a healthy Linux system, nearly all RAM appears "used" because the kernel aggressively caches disk data
- "Used" does not mean "unavailable" -- `MemAvailable` (not `MemFree`) is the true indicator
- When `MemAvailable` drops low, the kernel begins swapping
- `vmstat` fields `si` (swap-in) and `so` (swap-out) confirm active swapping
- PSI `memory_some_avg10 > 0` means tasks are actually stalling on memory
- `buddyinfo` reveals memory fragmentation at each allocation order

**Diagnostic flow:**
1. Check `meminfo` -> `MemAvailable`. If > 10% of `MemTotal`, stop (memory is OK)
2. Check `Cached + Buffers`. If large, caches are using RAM (normal)
3. Check `swaps` -> `SwapUsed`. If growing, active swapping is occurring
4. Check `vmstat` -> `si`, `so`, `pgmajfault`. Non-zero means swap thrashing
5. Check `pressure` -> `memory_some_avg10`. If > 0, tasks are stalling

**Common issues:** OOM Killer, false "RAM is full" alarms, memory leaks (growing RSS), swap storms, slab cache growth.

## CPU Story

**"Why is my server slow?"**

The CPU guide explains load averages, CPU utilization breakdown, and pressure stall information.

**Key insights:**
- Load average measures demand (run queue + I/O wait), not utilization
- Load < core count means the system is not CPU-overloaded
- `stat` breaks down CPU time: user, system, idle, iowait, steal
- High `iowait` looks like CPU is busy, but it actually means the CPU is waiting for slow storage
- PSI `cpu_some_avg10` is the most direct measure of CPU contention
- `schedstat` shows per-CPU scheduling statistics
- High `context_switches` in `stat` may indicate too many competing threads

**Diagnostic flow:**
1. Check `loadavg` -> `load1` vs CPU count. If load < cores, CPU is not the bottleneck
2. Check `stat` -> `cpu_iowait` (high = I/O problem), `cpu_user` (high = app CPU-bound), `cpu_system` (high = kernel overhead)
3. Check `pressure` -> `cpu_some_avg10`. If > 0, tasks are stalling for CPU
4. Check `stat` -> `context_switches` for excessive thread competition

## Network Story

**"Why are connections failing?"**

The Network guide covers TCP connection states, socket statistics, and interface traffic.

**Key insights:**
- TCP connection states reveal application behavior: `ESTABLISHED` (active), `TIME_WAIT` (recently closed), `CLOSE_WAIT` (FD leak), `SYN_SENT` (target unreachable)
- `net/sockstat` shows socket memory usage and allocation counts
- `net/snmp` provides protocol-level error counters (retransmissions, out-of-order segments)
- `net/dev` shows per-interface traffic with error and drop counters
- Non-zero drops indicate buffer overflow or rate limiting

**Common issues:** Socket FD leaks (growing CLOSE_WAIT), ephemeral port exhaustion (many TIME_WAIT), connection timeouts (SYN_SENT), retransmission storms.

## Storage Story

**"Why is my disk slow?"**

The Storage guide covers disk I/O, filesystem usage, and mount options.

**Key insights:**
- `diskstats` shows per-device I/O operations, bytes transferred, and time spent
- `df` provides filesystem usage percentages
- `mounts` lists all mounted filesystems with their options
- PSI `io_some_avg10` measures tasks stalled on I/O
- `locks` shows file locks held by processes

**Common issues:** Disk full (logs, temp files), I/O bottleneck (slow HDD), mount option issues (noatime, sync).

## Process Story

**"What are all these processes doing?"**

The Process guide covers process states, resource consumption, and file descriptor management.

**Key insights:**
- Process states: R (running), S (sleeping), D (uninterruptible I/O), Z (zombie), T (stopped)
- RSS (Resident Set Size) shows actual physical memory used
- Zombie processes indicate parents not calling `wait()`
- D-state processes are stuck in I/O and cannot be killed
- `file-nr` tracks system-wide file descriptor usage

## Learning Paths

### Path 1: "I am new to Linux administration"

1. Start with the **Dashboard** to understand system health at a glance
2. Open the **Category Guide** (`C`) and read the Memory story first
3. Switch to Classic mode (`O`), select `meminfo`, and compare what you read with live data
4. Enable help (`?` twice for DETAILED level) to see field descriptions
5. Try the diagnostic flow from the guide against your live system

### Path 2: "I want to debug a performance issue"

1. Open **Diagnostics** (`X`) to see automated findings
2. Check the severity -- start with CRIT items
3. Open the **Category Guide** for the relevant subsystem
4. Follow the diagnostic flow step by step
5. Use Classic mode to inspect the specific sources mentioned

### Path 3: "I want to understand Linux internals deeply"

1. Read all five Category Guide stories (Memory, CPU, Network, Storage, Process)
2. Set help to EXTRA level for maximum detail
3. Inspect each source in Classic mode, reading every field description
4. Use diff view (`d`) to watch counters change over time
5. Use graph view (`g`) to visualize trends
6. Try comparing a healthy system with a stressed one (use `--import` with saved snapshots)

---

[<- Prev: Diagnostics](diagnostics.md) | [Index](index.md) | [Next: Remote Monitoring ->](remote.md)

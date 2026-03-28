---
version: v1.0.0
lang: en
---

# Auto-Diagnostics

[<- Prev: Classic Mode](classic-mode.md) | [Index](index.md) | [Next: Education ->](education.md)

[🇯🇵 日本語版](../ja/diagnostics.md)

## Table of Contents

- [Overview](#overview)
- [Accessing Diagnostics](#accessing-diagnostics)
- [Severity Levels](#severity-levels)
- [Diagnostic Findings Format](#diagnostic-findings-format)
- [All Diagnostic Checks](#all-diagnostic-checks)
  - [Memory](#memory)
  - [Load Average](#load-average)
  - [Swap](#swap)
  - [Pressure (PSI)](#pressure-psi)
  - [Processes](#processes)
  - [Network (TCP)](#network-tcp)
  - [Disk](#disk)
  - [Temperature](#temperature)
  - [File Descriptors](#file-descriptors)
  - [DNS](#dns)
  - [Conntrack](#conntrack)
- [Copying Diagnostics](#copying-diagnostics)

## Overview

syslenz includes an automatic diagnostics engine that analyzes every snapshot and produces actionable findings. It checks for common system health issues across memory, CPU, network, storage, and more. Findings are sorted by severity so the most critical issues appear first.

The diagnostics engine runs 11 checks against each snapshot and produces findings only when thresholds are exceeded. A healthy system typically shows zero findings.

## Accessing Diagnostics

Press `X` from any view to open the Diagnostics view.

The diagnostics panel shows all current findings in a scrollable list. Each finding includes:
- Severity level
- Source (which data source triggered the finding)
- Title (brief description of the issue)
- Detail (explanation of what is happening)
- Suggestion (recommended action)

## Severity Levels

| Level | Label | Description |
|-------|-------|-------------|
| **Critical** | `CRIT` | Immediate action required. System may crash or degrade severely. |
| **Warning** | `WARN` | Attention needed. Issue is developing or has moderate impact. |
| **Info** | `INFO` | Informational. Something noteworthy but not necessarily a problem. |

Findings are sorted: Critical first, then Warning, then Info.

## Diagnostic Findings Format

Each finding is a structured object with five fields:

```
[CRIT] meminfo: Memory critical: 5.2% available
  MemAvailable (832 MiB) is below 10% of MemTotal (16.0 GiB).
  OOM Killer may activate.
  -> Run: ps aux --sort=-rss | head to find top memory consumers
```

## All Diagnostic Checks

### Memory

**Source:** `meminfo`

| Condition | Severity | Title |
|-----------|----------|-------|
| MemAvailable < 10% of MemTotal | CRIT | Memory critical: X% available |
| MemAvailable < 20% of MemTotal | WARN | Memory low: X% available |

**What it checks:** Calculates the percentage of memory available to applications (`MemAvailable / MemTotal * 100`). MemAvailable accounts for reclaimable caches, making it more accurate than MemFree.

**Suggestion for CRIT:** `ps aux --sort=-rss | head` to find top memory consumers.

**Suggestion for WARN:** Check for processes with growing RSS over time.

### Load Average

**Source:** `loadavg`, `stat` or `cpuinfo` (for CPU count)

| Condition | Severity | Title |
|-----------|----------|-------|
| load1 > 2x CPU count | CRIT | CPU overloaded: load X (Nx M CPUs) |
| load1 > 1x CPU count | WARN | CPU saturated: load X (Nx M CPUs) |

**What it checks:** Compares the 1-minute load average to the number of CPU cores. A ratio above 1.0 means more tasks want CPU time than cores are available. Above 2.0 means severe queuing.

**Suggestion for CRIT:** Check PSI pressure data. High CPU pressure = need more CPUs. High I/O pressure = disk bottleneck.

**Suggestion for WARN:** Run `top` to identify high-CPU processes.

### Swap

**Source:** `meminfo`

| Condition | Severity | Title |
|-----------|----------|-------|
| SwapFree = 0 (and SwapTotal > 0) | CRIT | Swap exhausted |
| SwapTotal = 0 | INFO | No swap configured |

**What it checks:** Detects swap exhaustion (all swap used, next allocation may trigger OOM Killer) and the absence of swap configuration.

**Suggestion for CRIT:** Investigate high-memory processes immediately.

**Suggestion for INFO:** Consider configuring swap at 1-2x RAM for production.

### Pressure (PSI)

**Source:** `pressure`

| Condition | Severity | Title |
|-----------|----------|-------|
| cpu/memory/io_some_avg10 > 50% | CRIT | X pressure: Y% (>50%) |
| cpu/memory/io_some_avg10 > 25% | WARN | X pressure: Y% (>25%) |

**What it checks:** Linux Pressure Stall Information (PSI) measures the percentage of time tasks are stalled waiting for a resource. This is the most direct measure of contention -- unlike load average, PSI measures actual stalls.

Three resources are checked:
- **CPU**: Tasks stalled waiting for CPU time
- **Memory**: Tasks stalled waiting for memory (page reclaim, swap)
- **I/O**: Tasks stalled waiting for I/O completion

**Suggestions:**
- CPU: Identify CPU-bound processes with `top`
- Memory: Check MemAvailable, possible memory leak
- I/O: Check `diskstats` await, consider SSD upgrade

### Processes

**Source:** `processes`

| Condition | Severity | Title |
|-----------|----------|-------|
| Zombie (Z-state) count > 5 | WARN | N zombie processes detected |
| D-state count > 3 | WARN | N processes in D-state |

**What it checks:**

- **Zombie processes:** Processes that have exited but whose parent has not called `wait()`. Zombies consume no resources but indicate a buggy parent process. Small numbers are normal; many zombies suggest a parent is not reaping children.

- **D-state processes:** Processes in uninterruptible sleep, typically waiting for I/O. These cannot be killed with SIGKILL. Many D-state processes suggest NFS hangs, disk failures, or kernel driver issues.

**Suggestions:**
- Zombies: Check PPID to identify the parent process
- D-state: Check `dmesg` for storage-related errors

### Network (TCP)

**Source:** `net/tcp`

| Condition | Severity | Title |
|-----------|----------|-------|
| SYN_SENT > 10 | WARN | N SYN_SENT -- targets not responding |
| CLOSE_WAIT > 20 | WARN | N CLOSE_WAIT -- socket FD leak |
| TIME_WAIT > 5000 | INFO | N TIME_WAIT connections |

**What it checks:**

- **SYN_SENT:** Outgoing connections waiting for a response. Many SYN_SENT connections suggest the target host is down, a firewall is dropping packets, or DNS resolution is slow.

- **CLOSE_WAIT:** The remote end has closed the connection, but the local application has not called `close()`. This is a classic file descriptor leak in applications. The count will grow over time until the process runs out of FDs.

- **TIME_WAIT:** Normal TCP state after closing a connection (persists for 2x MSL, typically 60 seconds). Large numbers indicate many short-lived connections, which can exhaust ephemeral ports.

**Suggestions:**
- SYN_SENT: Check target IPs/ports, test reachability with `ping`/`telnet`
- CLOSE_WAIT: Use `lsof` to find which process holds CLOSE_WAIT sockets
- TIME_WAIT: Consider setting `net.ipv4.tcp_tw_reuse = 1`

### Disk

**Source:** `df`

| Condition | Severity | Title |
|-----------|----------|-------|
| Root filesystem > 90% used | CRIT | Disk usage X% -- critically low space |
| Root filesystem > 80% used | WARN | Disk usage X% -- getting full |

**What it checks:** Monitors the root filesystem usage percentage.

**Suggestions:**
- CRIT: `du -sh /*` to find large directories. `journalctl --vacuum-size=500M` to trim logs
- WARN: Clean up old logs and caches. `df -h` to check all partitions

### Temperature

**Source:** `thermal`

| Condition | Severity | Title |
|-----------|----------|-------|
| Max CPU temp > 90C | CRIT | CPU temperature XC -- overheating |
| Max CPU temp > 75C | WARN | CPU temperature XC -- running hot |

**What it checks:** Reads thermal zone data from `/sys/class/thermal/`. Temperatures above 90C trigger thermal throttling, severely degrading performance and risking hardware damage.

**Suggestions:**
- CRIT: Check cooling system: fan operation, thermal paste, airflow
- WARN: Consider improving cooling, check CPU utilization and distribute load

### File Descriptors

**Source:** `file-nr`

| Condition | Severity | Title |
|-----------|----------|-------|
| FD usage > 80% of system max | WARN | FD usage X% -- exhaustion risk |

**What it checks:** Compares the current number of open file descriptors system-wide against the kernel maximum (`fs.file-max`). FD exhaustion prevents processes from opening files, sockets, or pipes.

**Suggestion:** `lsof | wc -l` to check open FDs. Find leaking processes. Consider raising `sysctl fs.file-max`.

### DNS

**Source:** `dns`

| Condition | Severity | Title |
|-----------|----------|-------|
| No nameservers in /etc/resolv.conf | WARN | No DNS nameservers configured |

**What it checks:** Parses `/etc/resolv.conf` and checks for nameserver entries. Without nameservers, DNS resolution fails entirely.

**Suggestion:** Add a nameserver to resolv.conf (e.g., `nameserver 8.8.8.8`).

### Conntrack

**Source:** `conntrack`

| Condition | Severity | Title |
|-----------|----------|-------|
| Conntrack table > 80% full | WARN | Conntrack usage X% -- table exhaustion risk |

**What it checks:** Monitors the Linux connection tracking table usage. When the table is full, new connections are dropped. This commonly affects firewalls, NAT gateways, and load balancers.

**Suggestion:** Increase `sysctl net.nf_conntrack_max` or add NOTRACK rules for high-traffic flows.

## Copying Diagnostics

Press `c` while in the Diagnostics view to copy the current diagnostic findings to the clipboard. This is useful for sharing findings in incident reports or chat.

---

[<- Prev: Classic Mode](classic-mode.md) | [Index](index.md) | [Next: Education ->](education.md)

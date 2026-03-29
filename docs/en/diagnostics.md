---
version: v1.3.0
lang: en
---

# Auto-Diagnostics

[🇯🇵 日本語版](../ja/diagnostics.md)

[<- Prev: Classic Mode](classic-mode.md) | [Index](index.md) | [Next: Education ->](education.md)


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
  - [Memory Leak (v1.3.0)](#memory-leak-v130)
  - [Swap Activity (v1.3.0)](#swap-activity-v130)
  - [OOM Kills (v1.3.0)](#oom-kills-v130)
  - [Network Errors (v1.3.0)](#network-errors-v130)
  - [Recent Reboot (v1.3.0)](#recent-reboot-v130)
  - [Load Trend (v1.3.0)](#load-trend-v130)
  - [High-Memory Process (v1.3.0)](#high-memory-process-v130)
  - [Orphaned TCP (v1.3.0)](#orphaned-tcp-v130)
  - [IP Forwarding (v1.3.0)](#ip-forwarding-v130)
  - [Kernel Taint (v1.3.0)](#kernel-taint-v130)
  - [Inode Pressure (v1.3.0)](#inode-pressure-v130)
  - [Context Switches (v1.3.0)](#context-switches-v130)
  - [Conntrack Rate (v1.3.0)](#conntrack-rate-v130)
  - [TCP Listen Ports (v1.3.0)](#tcp-listen-ports-v130)
- [User-Defined Alerts (v1.1.0)](#user-defined-alerts-v110)
- [Copying Diagnostics](#copying-diagnostics)

## Overview

syslenz includes an automatic diagnostics engine that analyzes every snapshot and produces actionable findings. It checks for common system health issues across memory, CPU, network, storage, and more. Findings are sorted by severity so the most critical issues appear first.

The diagnostics engine runs 25 checks against each snapshot and produces findings only when thresholds are exceeded. A healthy system typically shows zero findings.

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

### Memory Leak (v1.3.0)

**Source:** `meminfo` (time-series comparison)

| Condition | Severity | Title |
|-----------|----------|-------|
| MemAvailable decreasing steadily over 10+ snapshots | WARN | Possible memory leak: MemAvailable declining steadily |

**What it checks:** Compares MemAvailable across the snapshot history ring buffer. If MemAvailable has decreased in 10 or more consecutive snapshots, a potential memory leak is flagged.

**Suggestion:** Identify processes with growing RSS using `ps aux --sort=-rss | head`. Consider running `valgrind` or `heaptrack` on suspect processes.

### Swap Activity (v1.3.0)

**Source:** `vmstat`

| Condition | Severity | Title |
|-----------|----------|-------|
| pswpin + pswpout delta > 1000 pages/s | WARN | Active swapping detected: N pages/s |
| pswpin + pswpout delta > 5000 pages/s | CRIT | Heavy swapping: N pages/s |

**What it checks:** Measures the rate of pages swapped in and out between consecutive snapshots. Active swapping degrades performance because disk I/O is orders of magnitude slower than RAM access.

**Suggestion:** Check MemAvailable. Identify high-RSS processes. Consider adding RAM or tuning `vm.swappiness`.

### OOM Kills (v1.3.0)

**Source:** `vmstat`

| Condition | Severity | Title |
|-----------|----------|-------|
| oom_kill counter increased since last snapshot | CRIT | OOM Kill detected: N kills since last check |

**What it checks:** Monitors the `oom_kill` counter in `/proc/vmstat`. An increase means the kernel's Out-of-Memory Killer terminated one or more processes to free memory.

**Suggestion:** Check `dmesg | grep -i oom` to identify killed processes. Increase memory or configure `oom_score_adj` for critical services.

### Network Errors (v1.3.0)

**Source:** `net/dev`

| Condition | Severity | Title |
|-----------|----------|-------|
| rx_errors + tx_errors delta > 0 on any interface | WARN | Network errors on IFACE: N errors |
| rx_drops + tx_drops delta > 100 on any interface | WARN | Packet drops on IFACE: N drops |

**What it checks:** Monitors per-interface error and drop counters. Non-zero deltas indicate hardware issues, driver bugs, buffer overflows, or network congestion.

**Suggestion:** Check `ethtool -S IFACE` for detailed NIC statistics. Inspect cable connections and switch port errors.

### Recent Reboot (v1.3.0)

**Source:** `uptime`

| Condition | Severity | Title |
|-----------|----------|-------|
| Uptime < 300 seconds (5 minutes) | INFO | System recently rebooted: uptime Xm Xs |

**What it checks:** Flags that the system was recently rebooted, which may indicate a crash, kernel panic, or planned maintenance.

**Suggestion:** Check `last reboot` and `dmesg` for crash indicators. Review `/var/log/kern.log` for panic messages.

### Load Trend (v1.3.0)

**Source:** `loadavg`

| Condition | Severity | Title |
|-----------|----------|-------|
| load1 > load5 > load15 and load1 > CPU count | WARN | Load increasing: 1m X > 5m Y > 15m Z |

**What it checks:** Detects a rising load trend by comparing the three load averages. When all three are increasing and the 1-minute average exceeds CPU count, workload is growing and may soon become critical.

**Suggestion:** Identify new or growing workloads with `top`. Check for recently started batch jobs or cron tasks.

### High-Memory Process (v1.3.0)

**Source:** `processes`, `meminfo`

| Condition | Severity | Title |
|-----------|----------|-------|
| Any single process RSS > 50% of MemTotal | WARN | Process "NAME" (PID) using X% of total memory |

**What it checks:** Scans the process list for any single process consuming more than half of total physical memory.

**Suggestion:** Verify the process is expected to use that much memory. Check for memory leaks or misconfigured resource limits.

### Orphaned TCP (v1.3.0)

**Source:** `net/sockstat`

| Condition | Severity | Title |
|-----------|----------|-------|
| TCP orphan count > 1000 | WARN | N orphaned TCP sockets |

**What it checks:** Monitors the orphan count from `/proc/net/sockstat`. Orphaned sockets are TCP connections with no owning process, consuming kernel memory until they time out.

**Suggestion:** Check for application crashes that leave connections behind. Tune `net.ipv4.tcp_max_orphans` if needed.

### IP Forwarding (v1.3.0)

**Source:** `/proc/sys/net/ipv4/ip_forward`

| Condition | Severity | Title |
|-----------|----------|-------|
| ip_forward = 1 on a non-router system | INFO | IP forwarding is enabled |

**What it checks:** Detects that IP forwarding is enabled. On systems not intended to be routers, this may indicate misconfiguration or a security concern.

**Suggestion:** If this system is not a router, NAT gateway, or container host, disable with `sysctl net.ipv4.ip_forward=0`.

### Kernel Taint (v1.3.0)

**Source:** `/proc/sys/kernel/tainted`

| Condition | Severity | Title |
|-----------|----------|-------|
| Taint flags != 0 | INFO | Kernel is tainted: flags=X (meanings: ...) |

**What it checks:** Reads the kernel taint bitmask. A tainted kernel has loaded proprietary modules, experienced a machine check exception, or had other events that may make kernel bugs harder to diagnose.

**Suggestion:** Identify taint sources with `cat /proc/sys/kernel/tainted`. Common causes: proprietary GPU drivers (flag 1), out-of-tree modules (flag 4096).

### Inode Pressure (v1.3.0)

**Source:** `df` (inode mode)

| Condition | Severity | Title |
|-----------|----------|-------|
| Inode usage > 90% on any filesystem | CRIT | Inode usage X% on MOUNT -- critically low |
| Inode usage > 80% on any filesystem | WARN | Inode usage X% on MOUNT -- getting full |

**What it checks:** Monitors inode usage via `df -i`. A filesystem can run out of inodes even with free disk space, preventing new file creation. This commonly affects systems with many small files.

**Suggestion:** Find directories with many files using `find / -xdev -printf '%h\n' | sort | uniq -c | sort -rn | head`. Consider reformatting with more inodes or cleaning up.

### Context Switches (v1.3.0)

**Source:** `stat`

| Condition | Severity | Title |
|-----------|----------|-------|
| Context switch rate > 100,000/s per CPU | WARN | High context switch rate: N/s (Xk per CPU) |

**What it checks:** Calculates the per-CPU context switch rate from `/proc/stat`. Extremely high context switching indicates excessive thread contention, too many runnable threads, or a locking problem.

**Suggestion:** Profile the workload with `perf stat` to measure actual context switch overhead. Reduce thread count or fix lock contention.

### Conntrack Rate (v1.3.0)

**Source:** `conntrack`

| Condition | Severity | Title |
|-----------|----------|-------|
| Conntrack count increasing > 1000/s | WARN | Conntrack table growing rapidly: +N/s |

**What it checks:** Measures the rate of change in the conntrack table size. A rapidly growing table may indicate a DDoS attack, connection flood, or misconfigured timeout values.

**Suggestion:** Check for SYN floods with `conntrack -S`. Reduce timeouts with `sysctl net.netfilter.nf_conntrack_tcp_timeout_established`. Add NOTRACK rules for trusted high-traffic flows.

### TCP Listen Ports (v1.3.0)

**Source:** `net/tcp`

| Condition | Severity | Title |
|-----------|----------|-------|
| LISTEN socket bound to 0.0.0.0 on unexpected port | INFO | N services listening on all interfaces |

**What it checks:** Scans TCP sockets in LISTEN state bound to `0.0.0.0` (all interfaces). Services exposed to all interfaces may be unintentionally accessible from the network.

**Suggestion:** Review listening services with `ss -tlnp`. Bind services to `127.0.0.1` if they only need local access. Use firewall rules to restrict access.

## User-Defined Alerts (v1.1.0)

As of v1.1.0, syslenz supports user-defined alert rules configured via `[[alert]]` entries in `config.toml`. Alerts complement the built-in diagnostics engine by letting you set custom thresholds tailored to your environment.

When an alert fires:

- The **status bar** displays an alert count (e.g., `ALERT: 2 active`)
- The **sidebar** colors the source that triggered the alert, drawing your attention immediately
- Individual **fields** are marked with an alert indicator in the detail view

Alerts and built-in diagnostics work together -- diagnostics provide general best-practice checks, while alerts let you define precise thresholds for your specific workloads. See the [Configuration Reference](config.md#alert-v110) for the full `[[alert]]` syntax.

## Copying Diagnostics

Press `c` while in the Diagnostics view to copy the current diagnostic findings to the clipboard. This is useful for sharing findings in incident reports or chat.

---

[<- Prev: Classic Mode](classic-mode.md) | [Index](index.md) | [Next: Education ->](education.md)

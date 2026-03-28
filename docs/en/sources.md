---
version: v1.1.0
lang: en
---

# Data Sources Reference

[🇯🇵 日本語版](../ja/sources.md)

[<- Prev: OpenTelemetry](otel.md) | [Index](index.md)


## Table of Contents

- [Overview](#overview)
- [Memory](#memory)
- [CPU and Scheduling](#cpu-and-scheduling)
- [System](#system)
- [Storage](#storage)
- [Network (proc)](#network-proc)
- [Network (system)](#network-system)
- [Process](#process)
- [Hardware](#hardware)
- [Security and Kernel](#security-and-kernel)
- [Plugins](#plugins)

## Overview

syslenz reads from 55+ data sources organized across `/proc`, `/sys`, system config files, and command outputs. Each source is parsed into structured, typed fields. This page documents every source, what it reads, its key fields, and when to use it.

## Memory

### meminfo

| | |
|---|---|
| **Reads** | `/proc/meminfo` |
| **Key fields** | MemTotal (Bytes), MemFree (Bytes), MemAvailable (Bytes), Buffers (Bytes), Cached (Bytes), SwapTotal (Bytes), SwapFree (Bytes), SReclaimable (Bytes), SUnreclaim (Bytes) |
| **When to use** | First stop for any memory investigation. Check MemAvailable as percentage of MemTotal. |

### vmstat

| | |
|---|---|
| **Reads** | `/proc/vmstat` |
| **Key fields** | pgpgin (Integer), pgpgout (Integer), pswpin (Integer), pswpout (Integer), pgmajfault (Integer), pgfault (Integer) |
| **When to use** | Diagnosing swap activity (`pswpin`/`pswpout`) and page fault rates. |

### swaps

| | |
|---|---|
| **Reads** | `/proc/swaps` |
| **Key fields** | Table with columns: filename, type, size, used, priority |
| **When to use** | Check which swap devices are configured and how much is used on each. |

### buddyinfo

| | |
|---|---|
| **Reads** | `/proc/buddyinfo` |
| **Key fields** | Table with per-zone free page counts at each allocation order (0-10) |
| **When to use** | Diagnosing memory fragmentation. If high-order columns are all zero, large contiguous allocations will fail. |

### zoneinfo

| | |
|---|---|
| **Reads** | `/proc/zoneinfo` |
| **Key fields** | Per-zone: free (Integer), min (Integer), low (Integer), high (Integer), managed (Integer) |
| **When to use** | Deep memory debugging. Understanding per-NUMA-zone watermarks and free page distribution. |

### slabinfo

| | |
|---|---|
| **Reads** | `/proc/slabinfo` |
| **Key fields** | Table with columns: name, active_objs, num_objs, objsize, objperslab, pagesperslab |
| **When to use** | Investigating kernel slab cache growth (dentry, inode caches). Check when SUnreclaim is large in meminfo. |

### pagetypeinfo

| | |
|---|---|
| **Reads** | `/proc/pagetypeinfo` |
| **Key fields** | Table with per-zone, per-type free page counts |
| **When to use** | Advanced memory fragmentation analysis beyond buddyinfo. |

## CPU and Scheduling

### stat

| | |
|---|---|
| **Reads** | `/proc/stat` |
| **Key fields** | cpu_user (Integer), cpu_system (Integer), cpu_idle (Integer), cpu_iowait (Integer), cpu_steal (Integer), cpu_count (Integer), context_switches (Integer), processes_created (Integer), procs_running (Integer), procs_blocked (Integer) |
| **When to use** | CPU utilization breakdown. Compare user vs system vs iowait to classify workload. |

### loadavg

| | |
|---|---|
| **Reads** | `/proc/loadavg` |
| **Key fields** | load1 (Float), load5 (Float), load15 (Float), running_tasks (Integer), total_tasks (Integer) |
| **When to use** | Quick CPU demand check. Compare to CPU count. |

### cpuinfo

| | |
|---|---|
| **Reads** | `/proc/cpuinfo` |
| **Key fields** | cpu_count (Integer), model_name (Text), cpu_mhz (Float), cache_size (Text), flags (Text) |
| **When to use** | Hardware info: CPU model, core count, frequency, feature flags. |

### pressure

| | |
|---|---|
| **Reads** | `/proc/pressure/cpu`, `/proc/pressure/memory`, `/proc/pressure/io` |
| **Key fields** | cpu_some_avg10 (Float), cpu_some_avg60 (Float), cpu_some_avg300 (Float), memory_some_avg10 (Float), memory_full_avg10 (Float), io_some_avg10 (Float), io_full_avg10 (Float) |
| **When to use** | Most direct measure of resource contention. PSI > 0 means tasks are stalling. |

### schedstat

| | |
|---|---|
| **Reads** | `/proc/schedstat` |
| **Key fields** | Per-CPU: running_time (Integer), waiting_time (Integer), timeslices (Integer) |
| **When to use** | Deep scheduling analysis. High waiting_time relative to running_time indicates CPU contention. |

### interrupts

| | |
|---|---|
| **Reads** | `/proc/interrupts` |
| **Key fields** | Table with per-CPU interrupt counts by IRQ number |
| **When to use** | Hardware interrupt analysis. Check for IRQ storms or unbalanced interrupt distribution. |

### softirqs

| | |
|---|---|
| **Reads** | `/proc/softirqs` |
| **Key fields** | Table with per-CPU softirq counts (HI, TIMER, NET_TX, NET_RX, BLOCK, etc.) |
| **When to use** | High NET_RX softirqs indicate heavy network traffic. High BLOCK softirqs indicate heavy disk I/O. |

### timer_list

| | |
|---|---|
| **Reads** | `/proc/timer_list` |
| **Key fields** | Active timer entries |
| **When to use** | Debugging kernel timers and hrtimer issues. |

## System

### uptime

| | |
|---|---|
| **Reads** | `/proc/uptime` |
| **Key fields** | uptime (Duration), idle (Duration) |
| **When to use** | Check how long the system has been running. |

### version

| | |
|---|---|
| **Reads** | `/proc/version` |
| **Key fields** | kernel_version (Text) |
| **When to use** | Identify kernel version, build info. |

### cmdline

| | |
|---|---|
| **Reads** | `/proc/cmdline` |
| **Key fields** | cmdline (Text) |
| **When to use** | Check kernel boot parameters. |

### modules

| | |
|---|---|
| **Reads** | `/proc/modules` |
| **Key fields** | Table with columns: name, size, use_count, used_by, state |
| **When to use** | List loaded kernel modules. |

### filesystems

| | |
|---|---|
| **Reads** | `/proc/filesystems` |
| **Key fields** | Table with supported filesystem types |
| **When to use** | Check which filesystems the kernel supports. |

### devices

| | |
|---|---|
| **Reads** | `/proc/devices` |
| **Key fields** | Table with character and block device numbers and names |
| **When to use** | Device number to name mapping. |

### consoles

| | |
|---|---|
| **Reads** | `/proc/consoles` |
| **Key fields** | Table with console device information |
| **When to use** | Check configured console devices. |

### misc

| | |
|---|---|
| **Reads** | `/proc/misc` |
| **Key fields** | Table with miscellaneous device registrations |
| **When to use** | Check misc character device minor numbers. |

### dma

| | |
|---|---|
| **Reads** | `/proc/dma` |
| **Key fields** | Table with DMA channel assignments |
| **When to use** | Hardware DMA channel inspection. |

## Storage

### diskstats

| | |
|---|---|
| **Reads** | `/proc/diskstats` |
| **Key fields** | Table with per-device: reads_completed, reads_merged, sectors_read, read_time_ms, writes_completed, writes_merged, sectors_written, write_time_ms, io_in_progress, io_time_ms |
| **When to use** | Disk I/O analysis. Check io_time_ms and in-progress I/O for bottlenecks. |

### df

| | |
|---|---|
| **Reads** | `df` command output (parsed) |
| **Key fields** | root_use_pct (Float), and per-filesystem table |
| **When to use** | Filesystem usage monitoring. Diagnostics checks root_use_pct. |

### mounts

| | |
|---|---|
| **Reads** | `/proc/mounts` |
| **Key fields** | Table with columns: device, mountpoint, fstype, options |
| **When to use** | Check mount options (noatime, sync, bind), verify expected mounts. |

### partitions

| | |
|---|---|
| **Reads** | `/proc/partitions` |
| **Key fields** | Table with columns: major, minor, blocks, name |
| **When to use** | List block devices and their partition sizes. |

### locks

| | |
|---|---|
| **Reads** | `/proc/locks` |
| **Key fields** | Table with active file locks (type, mode, PID, inode) |
| **When to use** | Debugging file locking issues between processes. |

## Network (proc)

### net/dev

| | |
|---|---|
| **Reads** | `/proc/net/dev` |
| **Key fields** | Table with per-interface: rx_bytes, rx_packets, rx_errors, rx_drops, tx_bytes, tx_packets, tx_errors, tx_drops |
| **When to use** | Interface traffic monitoring. Non-zero errors/drops indicate problems. |

### net/tcp

| | |
|---|---|
| **Reads** | `/proc/net/tcp` |
| **Key fields** | Table with columns: local_addr, remote_addr, state, tx_queue, rx_queue, uid, inode |
| **When to use** | TCP connection analysis. Check for SYN_SENT, CLOSE_WAIT, TIME_WAIT accumulation. |

### net/udp

| | |
|---|---|
| **Reads** | `/proc/net/udp` |
| **Key fields** | Table with columns: local_addr, remote_addr, state, drops, uid, inode |
| **When to use** | UDP socket monitoring. Check drops for overloaded UDP services. |

### net/unix

| | |
|---|---|
| **Reads** | `/proc/net/unix` |
| **Key fields** | Table with Unix domain socket paths, types, states |
| **When to use** | Check Unix socket connectivity between local services. |

### net/arp

| | |
|---|---|
| **Reads** | `/proc/net/arp` |
| **Key fields** | Table with columns: IP, HW_type, Flags, HW_addr, Mask, Device |
| **When to use** | ARP table inspection. Check for stale or missing entries. |

### net/route

| | |
|---|---|
| **Reads** | `/proc/net/route` |
| **Key fields** | Table with kernel routing table (destination, gateway, mask, flags, interface) |
| **When to use** | Routing table inspection. Verify default gateway and routes. |

### net/sockstat

| | |
|---|---|
| **Reads** | `/proc/net/sockstat` |
| **Key fields** | TCP inuse (Integer), UDP inuse (Integer), TCP mem (Integer), TCP alloc (Integer), orphan (Integer) |
| **When to use** | Socket allocation overview. High orphan count indicates connection cleanup issues. |

### net/snmp

| | |
|---|---|
| **Reads** | `/proc/net/snmp` |
| **Key fields** | InSegs, OutSegs, RetransSegs, InErrs, OutRsts, and many more per protocol |
| **When to use** | Protocol-level error analysis. High RetransSegs = network congestion or packet loss. |

### net/netstat

| | |
|---|---|
| **Reads** | `/proc/net/netstat` |
| **Key fields** | Extended TCP statistics (TW, TWRecycled, TCPAbortOnTimeout, etc.) |
| **When to use** | Advanced TCP debugging beyond net/snmp. |

### net/wireless

| | |
|---|---|
| **Reads** | `/proc/net/wireless` |
| **Key fields** | Table with per-interface: status, link, level, noise |
| **When to use** | WiFi signal quality monitoring. |

## Network (system)

### dns

| | |
|---|---|
| **Reads** | `/etc/resolv.conf` |
| **Key fields** | nameservers (Table), search domains (Table) |
| **When to use** | DNS configuration validation. Diagnostics checks for missing nameservers. |

### conntrack

| | |
|---|---|
| **Reads** | `/proc/sys/net/netfilter/nf_conntrack_*` |
| **Key fields** | count (Integer), max (Integer), usage_pct (Float) |
| **When to use** | Connection tracking table monitoring for firewalls and NAT. |

### ip_route

| | |
|---|---|
| **Reads** | `ip route` command output |
| **Key fields** | Table with routing entries |
| **When to use** | Modern routing table view (supplements net/route). |

### ip_neighbor

| | |
|---|---|
| **Reads** | `ip neighbor` command output |
| **Key fields** | Table with neighbor entries (IP, MAC, state) |
| **When to use** | Modern ARP/NDP table view (supplements net/arp). |

### ss_summary

| | |
|---|---|
| **Reads** | `ss -s` command output |
| **Key fields** | Socket statistics summary |
| **When to use** | Quick socket count overview. |

## Process

### processes

| | |
|---|---|
| **Reads** | `/proc/[pid]/stat`, `/proc/[pid]/status` for all PIDs |
| **Key fields** | Table with columns: PID, name, state, RSS, threads, UID |
| **When to use** | Process list. Check for zombies (Z), D-state, high RSS consumers. |

### file-nr

| | |
|---|---|
| **Reads** | `/proc/sys/fs/file-nr` |
| **Key fields** | allocated_fds (Integer), max_fds (Integer), fd_usage_pct (Float) |
| **When to use** | System-wide FD usage. Diagnostics alerts at 80%. |

## Hardware

### thermal

| | |
|---|---|
| **Reads** | `/sys/class/thermal/thermal_zone*/temp` |
| **Key fields** | max_temp (Float), per-zone temperatures |
| **When to use** | CPU temperature monitoring. Diagnostics alerts at 75C and 90C. |

## Security and Kernel

### crypto

| | |
|---|---|
| **Reads** | `/proc/crypto` |
| **Key fields** | Table with registered cryptographic algorithms |
| **When to use** | Check available kernel crypto algorithms. |

### cgroups

| | |
|---|---|
| **Reads** | `/proc/cgroups` |
| **Key fields** | Table with cgroup controllers (name, hierarchy, num_cgroups, enabled) |
| **When to use** | Check which cgroup controllers are available and enabled. |

### iomem

| | |
|---|---|
| **Reads** | `/proc/iomem` |
| **Key fields** | Table with I/O memory mappings (address range, description) |
| **When to use** | Hardware memory map inspection. |

### ioports

| | |
|---|---|
| **Reads** | `/proc/ioports` |
| **Key fields** | Table with I/O port assignments |
| **When to use** | Hardware port assignment inspection. |

## Plugins

Plugin sources appear with the `plugin/` prefix:

| | |
|---|---|
| **Reads** | Executable output from `~/.config/syslenz/plugins/` |
| **Key fields** | Defined by the plugin (any valid FieldValue types) |
| **When to use** | Custom data sources. See the [Plugins guide](plugins.md). |

---

[<- Prev: OpenTelemetry](otel.md) | [Index](index.md)

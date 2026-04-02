# sourceguide: stat

[日本語版](../ja/sourceguide.stat.md)

---

## What is this source?

`/proc/stat` is the kernel's CPU time accounting file — cumulative tick counts showing how much time each CPU has spent in each mode since boot. It also includes system-wide counters for interrupts, context switches, and process creation.

```
$ cat /proc/stat
cpu  428291 0 183742 12847392 14832 0 3291 0 0 0
cpu0 107432 0 46038  3212832  3621  0 824  0 0 0
cpu1 106847 0 46209  3212432  3609  0 820  0 0 0
...
intr 48293847 ...
ctxt 29483920
btime 1711234567
processes 48291
procs_running 2
procs_blocked 0
```

The first `cpu` line is the sum of all CPUs. Each number is in USER_HZ ticks (typically 1/100 second on Linux). To get percentages, you must take two snapshots and calculate the delta.

```
  CPU time fields (in order):
  user  - time in user mode
  nice  - time in user mode with low priority (niced)
  system - time in kernel mode
  idle  - time doing nothing
  iowait - time waiting for I/O to complete
  irq   - time servicing hardware interrupts
  softirq - time servicing software interrupts
  steal - time "stolen" by hypervisor for other VMs  ← critical in VMs
  guest - time running virtual CPUs
  guest_nice - time running low-priority virtual CPUs
```

---

## What questions does it answer?

- Where is CPU time actually going — user code, kernel code, or I/O waiting?
- Is this a virtualized host having its CPU time stolen by the hypervisor? (`steal`)
- How many processes are running or blocked on I/O right now? (`procs_running`, `procs_blocked`)
- Is the system spending too much time in kernel mode compared to user mode? (`system` ratio)
- How fast is the system creating new processes? (`processes` counter rate)

---

## Key fields to watch

| Field | What it means | When to worry |
|---|---|---|
| `user` | Time running application code | Expected to be dominant on compute workloads |
| `system` | Time in kernel (syscalls, scheduling) | Persistently >20% suggests syscall-heavy or lock-contended code |
| `iowait` | Time CPUs idle waiting for I/O | >10% means I/O is a bottleneck. Correlate with diskstats. |
| `steal` | Time hypervisor served other VMs | **Any sustained steal% in a VM means you're being throttled.** Even 5–10% steal causes latency jitter. |
| `idle` | Genuine idle time | Low idle + low steal + low iowait = CPU saturation |
| `procs_blocked` | Processes in D state right now | Nonzero means threads are stuck in uninterruptible wait (often I/O or kernel locks) |

---

## How to read it directly

```sh
cat /proc/stat

# Traditional tool that computes percentages from /proc/stat
mpstat 1 5

# Or with vmstat
vmstat 1
# columns: us sy id wa st
#          user system idle iowait steal
```

To compute CPU% manually from two snapshots:

```sh
# Read twice, 5 seconds apart
s1=$(grep '^cpu ' /proc/stat)
sleep 5
s2=$(grep '^cpu ' /proc/stat)
# Subtract each field; divide non-idle by total to get busy%
```

For steal specifically on cloud VMs:

```sh
# Check current steal from vmstat
vmstat 1 | awk '{print "steal:", $17}'

# Or from top: look for %st in CPU line
top -bn1 | grep '%Cpu'
```

---

## A real episode

A web service running on a cloud VM had p99 latency varying by 40–80ms between requests with identical CPU profiles. The application team profiled extensively — no hot paths, no GC pauses, no unusual system calls.

`/proc/stat` showed `steal` averaging 12–18% throughout the day, with spikes to 30%+ every few minutes. The VM was on a noisy-neighbor host: other tenants on the same physical machine were doing I/O-intensive batch jobs, and the hypervisor was regularly preempting this VM's CPUs to service them.

The fix was not application tuning. Moving the VM to a dedicated-host instance (no shared CPU allocation) dropped steal to 0% and p99 latency normalized to under 20ms. The signal was in `/proc/stat` all along; it just wasn't on anyone's dashboard.

---

## See also

- `sourceguide.loadavg` — the derived load signal that `/proc/stat` helps explain
- `sourceguide.pressure` — PSI separates CPU scheduling delay from I/O stall more cleanly than stat
- `sourceguide.schedstat` — per-CPU scheduler statistics including runqueue wait time
- `sourceguide.processes` — current process and thread state, including D-state count

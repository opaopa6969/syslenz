# Syscall vs. Application Time

[日本語版](../ja/concept.syscall-vs-application-time.md)

---

CPU time is split between **user space** (application code) and **kernel space** (system calls).

```
  stat.cpu_user:   time running application code
  stat.cpu_system: time in kernel (syscalls)
  stat.cpu_iowait: time waiting for I/O
  stat.cpu_idle:   time doing nothing
```

**High cpu_system relative to cpu_user:**
Application is spending more time in kernel than doing actual work. Common causes:
- Excessive system calls (logging, small I/O operations)
- Lock contention (futex syscalls)
- Memory mapping operations (mmap, mprotect)
- Network operations (lots of small send/recv)

```sh
# Find which syscalls are taking time
strace -cp <pid>   # syscall count and time
perf top           # CPU hot spots
```

**High cpu_iowait:**
CPU is idle waiting for I/O completion. The application is I/O bound. The CPU could be doing work, but storage is the bottleneck.

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

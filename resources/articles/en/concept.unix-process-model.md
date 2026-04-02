# Unix Process Model

[日本語版](../ja/concept.unix-process-model.md)

---

Understanding the Unix process model helps interpret process-related metrics.

**Process states:**
```
  R (Running/Runnable): on CPU or ready to run
  S (Sleeping):         waiting for event (I/O, timer, signal)
  D (Uninterruptible):  waiting for I/O — cannot be interrupted
  Z (Zombie):           exited but parent not called wait()
  T (Stopped):          suspended (SIGSTOP or debugger)
```

**D state is important:**
`stat.procs_blocked` counts D-state processes. Stuck D-state means I/O is not completing. Common causes: NFS hang, disk failure, kernel bug.

**Zombie processes:**
`Z` state processes waste a PID but no memory. Parent needs to call `wait()`. Common in applications that don't properly handle child processes.

**Fork/exec model:**
- `fork()`: creates a copy of current process (copy-on-write)
- `exec()`: replaces process image with new program
- `stat.forks_total`: tracks rate of process creation

**Thread vs. Process:**
Linux uses the same mechanism for both — "tasks". Threads share address space; processes don't. `processes.process_count` shows all tasks.

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

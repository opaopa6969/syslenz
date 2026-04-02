# System Call Lifecycle

[日本語版](../ja/concept.system-call-lifecycle.md)

---

When a program calls `read()`, `write()`, or `socket()`, it makes a **system call** — a controlled switch from user space to kernel space.

```
  User code: int n = read(fd, buf, 1024);
       ↓  (syscall instruction, privilege switch)
  Kernel:
    1. Validate arguments
    2. Check file descriptor
    3. Check file permissions
    4. Find page in page cache or submit I/O
    5. Copy data to user buffer
    6. Return to user space with byte count
  User code: continues with n = number of bytes read
```

**Cost factors:**
- Context switch overhead (~1μs)
- Cache effects (kernel code brought in, app data pushed out)
- Actual work done (I/O wait, lock acquisition)

**Reducing syscall overhead:**
- Batch I/O (write larger chunks, use writev/readv)
- Use io_uring for async I/O (avoids context switches)
- Use `splice()` for zero-copy between file descriptors

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

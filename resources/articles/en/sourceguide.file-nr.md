# File Descriptor Usage

[日本語版](../ja/sourceguide.file-nr.md)

---

## What is it?

/proc/sys/fs/file-nr shows system-wide file descriptor usage: allocated, free (in cache), and maximum. Running out of file descriptors causes 'too many open files' errors.

---

## Quick start

```sh
cat /proc/file-nr
# or use syslenz to browse with descriptions
```

---

## See also

- `net/sockstat.sockets_used`
- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

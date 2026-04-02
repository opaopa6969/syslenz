# Unix Domain Sockets

[日本語版](../ja/sourceguide.net/unix.md)

---

## What is it?

/proc/net/unix lists all Unix domain sockets (local IPC): their types (STREAM, DGRAM), states, and paths. Useful for debugging local service communication.

---

## Quick start

```sh
cat /proc/net/unix
# or use syslenz to browse with descriptions
```

---

## See also

- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

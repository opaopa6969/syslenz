# Kernel Crypto Algorithms

[日本語版](../ja/sourceguide.crypto.md)

---

## What is it?

/proc/crypto lists all cryptographic algorithms registered in the kernel: hash functions, ciphers, and compression algorithms. Useful for auditing available crypto and verifying hardware acceleration.

---

## Quick start

```sh
cat /proc/crypto
# or use syslenz to browse with descriptions
```

---

## See also

- `sourceguide.vmstat` — vmstat memory statistics
- `sourceguide.meminfo` — memory information

# nr_writeback

What is this?
Pages currently being written to storage.

Why it matters
This is the in-flight part of writeback. If it stays elevated, the storage path cannot keep up.

How to read
- A small amount is normal.
- Sustained elevation points to slow disks or a heavy dirty-page backlog.
- Pair it with PSI to decide whether users are seeing stalls.

Next check
Compare with diskstats.active_devices and pressure.io_some_avg10.
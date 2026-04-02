# nr_dirty

What is this?
Pages that have been modified in RAM but not written back to storage yet.

Why it matters
A growing dirty page count means the system is building writeback pressure. That can turn into stalls later.

How to read
- Short spikes are normal during bursts.
- Sustained growth means writeback is behind.
- If pressure.io_some_avg10 also rises, the storage path is feeling it.

Next check
Watch nr_writeback and diskstats.active_devices.
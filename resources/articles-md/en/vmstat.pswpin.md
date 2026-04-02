# pswpin

What is this?
Pages brought back from swap into RAM.

Why it matters
Any sustained pswpin means the system is paying swap I/O to recover working sets.

How to read
- Occasional spikes can be harmless after a burst.
- Repeated growth means memory pressure is real.
- If pswpout also rises, the system is thrashing between RAM and swap.

Next check
Compare with MemAvailable and pswpout.
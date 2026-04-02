# Major Fault Distribution

What is this?
A grouped article for major page fault counts, useful for spotting disk-backed memory demand.

Why it matters
Major faults cause synchronous disk reads and can amplify latency under load.

How to read
- count: fault volume
- max: worst burst
- buckets: whether faults are clustered in a few phases

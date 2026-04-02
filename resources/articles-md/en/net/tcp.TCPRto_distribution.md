# TCP RTO Distribution

What is this?
A grouped article for TCP retransmission timeout values, best read through min/max/count and bucketed ranges.

Why it matters
A widening timeout distribution usually means the network path is getting slower or more variable.

How to read
- count: how many timeout samples were seen
- max: worst observed timeout
- buckets: whether the tail is getting heavier

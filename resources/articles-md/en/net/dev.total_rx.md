# total_rx

What is this?
Cumulative received bytes across all network interfaces.

Why it matters
This is the simplest host-level network throughput signal. Use deltas between snapshots to estimate receive rate.

How to read
- Growth over time is expected on active hosts.
- Compare it with total_tx to see whether the host is mostly consuming or serving data.
- If packet drops also rise, traffic quality is the issue, not just volume.

Next check
Compare with total_tx and Tcp_RetransSegs.
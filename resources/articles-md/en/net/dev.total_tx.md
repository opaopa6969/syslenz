# total_tx

What is this?
Cumulative transmitted bytes across all network interfaces.

Why it matters
This shows outbound network volume. It is useful for spotting serving workloads, backups, or unexpected exfiltration.

How to read
- High TX with low RX often means the host is serving data.
- Compare with total_rx to understand traffic direction.
- Sudden jumps deserve attention if the workload did not change.

Next check
Compare with total_rx and Udp_InErrors.
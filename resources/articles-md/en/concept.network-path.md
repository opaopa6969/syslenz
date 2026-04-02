# Network Path

What is this?
The full path from application socket to NIC, switch, remote host, and back again.

Why it matters
Network issues often appear as app slowness, timeouts, or retries long before packet loss becomes obvious.

How to use
- Break the path into local stack, link, and remote path
- Check drops, retransmits, and RTT together
- Compare throughput with small-request latency

Common mistakes
- Blaming the network for an app backlog
- Looking at bandwidth when the problem is retransmission
- Forgetting that DNS, TLS, and connect time are part of network latency

Diagnostic flow
1. See whether the pain is connection setup, payload transfer, or packet recovery.
2. Check interface errors and retransmits.
3. Check whether one peer or all peers are affected.
4. Decide whether the limit is host, link, switch, or remote service.

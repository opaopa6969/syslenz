# Tcp_RetransSegs

What is this?
Total TCP segments retransmitted because acknowledgements did not arrive in time.

Why it matters
Retransmissions usually mean packet loss, congestion, or an overloaded peer.

How to read
- Use the rate, not the absolute value.
- A small background rate is normal on busy networks.
- Rising retransmits plus listen drops can mean multiple network bottlenecks at once.

Next check
Compare with Udp_InErrors and TcpExt_ListenDrops.
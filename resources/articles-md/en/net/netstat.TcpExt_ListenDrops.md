# TcpExt_ListenDrops

What is this?
Connections dropped because the listen queue was full.

Why it matters
This is an application-facing failure mode. Clients are turned away when the server cannot accept fast enough.

How to read
- Any non-zero rate is a problem to investigate.
- Traffic spikes can overwhelm a too-small backlog.
- Garbage-collection pauses or slow accept loops can show up here.

Next check
Compare with Tcp_RetransSegs and procs_running.
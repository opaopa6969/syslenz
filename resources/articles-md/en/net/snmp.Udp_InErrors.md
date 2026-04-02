# Udp_InErrors

What is this?
UDP datagrams that could not be delivered successfully.

Why it matters
UDP has no retransmission. Once packets are dropped here, the application never gets them.

How to read
- Any persistent growth deserves a look.
- Receive-buffer exhaustion is a common cause.
- Check this alongside packet rate and application ingest speed.

Next check
Compare with Tcp_RetransSegs and listen queue drops.
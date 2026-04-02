# cpu_user

What is this?
Time spent running application code in user mode.

Why it matters
This is the useful work side of CPU usage. It is better than staring at load alone.

How to read
- Rising user time with stable latency is usually fine.
- User time plus rising loadavg can mean the host is simply busy.
- User time without matching throughput can point to inefficient code.

Next check
Compare with loadavg.load_1min and pressure.cpu_some_avg10.
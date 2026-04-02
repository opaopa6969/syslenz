# cpu_system

What is this?
Time spent inside kernel mode on behalf of workloads.

Why it matters
A high system share can mean network, storage, or syscall-heavy activity.

How to read
- System time rising together with iowait often means storage pressure.
- System time with high packet rates can mean network processing overhead.
- System time alone is not bad if throughput also rises.

Next check
Look at cpu_iowait, net/snmp counters, and loadavg.
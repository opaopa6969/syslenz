# Process Contention

What is this?
The situation where processes compete for CPU, locks, file descriptors, or memory inside user space.

Why it matters
An application can look healthy at the process level while one hot thread or lock stalls the whole service.

How to use
- Check process counts, thread counts, and file descriptor growth
- Look for lock contention and context-switch spikes
- Compare a hot process with the system-wide picture

Common mistakes
- Assuming more processes means more throughput
- Looking only at total RSS or total CPU
- Missing a single lock that serializes everything

Diagnostic flow
1. Find the hottest process.
2. Check whether it is CPU-bound, lock-bound, or IO-bound.
3. Check whether other processes are blocked behind it.
4. Check whether the limit is application design or operating system resources.

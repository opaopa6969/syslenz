# Queueing vs Saturation

What is this?
Two different failure shapes: work piling up in line, or the resource itself being full.

Why it matters
Queueing can exist before saturation. By the time saturation is obvious, tail latency may already be bad.

How to use
- Queueing shows up as waiting time, run queue, backlog, or retry accumulation
- Saturation shows up as a lack of headroom, throttling, or blocked progress
- The same utilization level can be safe on one workload and unstable on another

Common mistakes
- Equating any queue with overload
- Equating high utilization with saturation
- Ignoring service time changes that turn a small queue into a large one

Diagnostic flow
1. Check whether work is waiting.
2. Check whether the system still has spare capacity.
3. Check whether retries are increasing the queue.
4. Check whether the queue is a symptom or the cause.

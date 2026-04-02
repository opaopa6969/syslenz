# Latency Analysis

What is this?
The practice of breaking latency into service time, queue time, retry time, and waiting time.

Why it matters
Average latency hides tail pain. The user feels the slowest requests, not the median.

How to use
- Separate work from waiting
- Compare average with p95/p99 or max
- Look for queue growth before throughput collapse
- Check retries and timeouts, because they amplify tail latency

Common mistakes
- Optimizing mean while tail grows
- Ignoring retries that multiply load
- Confusing fast failures with healthy service

Diagnostic flow
1. Confirm whether the pain is in compute, queueing, storage, or network.
2. Compare mean and tail together.
3. Check pressure and saturation on the suspected resource.
4. Correlate with recent change and retry behavior.

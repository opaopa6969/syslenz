# Cross-metric Reading

What is this?
The habit of reading metrics in pairs or triples instead of in isolation.

Why it matters
Every metric has blind spots. Pairing counters reveals whether a value is healthy, noisy, or misleading.

How to use
- Capacity + pressure: MemAvailable with PSI, utilization with queue depth
- Average + tail: mean latency with p95/p99 or max
- Count + rate: event count with per-second growth
- Symptom + cause: user latency with CPU, memory, IO, or network signals

Common mistakes
- Looking only at a single gauge
- Comparing values with different units
- Reading a rising count without checking rate

Diagnostic flow
1. Start with the symptom metric.
2. Add a pressure metric to test contention.
3. Add a capacity metric to test headroom.
4. Add a rate or tail metric to test burstiness.

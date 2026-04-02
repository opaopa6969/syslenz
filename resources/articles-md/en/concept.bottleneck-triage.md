# Bottleneck Triage

What is this?
A fast method for separating CPU, memory, storage, network, and process contention.

Why it matters
If you triage in the wrong order, you waste time staring at symptoms that are downstream of the real limit.

How to use
1. Decide whether the system is slow, stuck, or dropping work.
2. Check pressure first to see whether the resource is contended.
3. Check capacity second to see whether the system still has headroom.
4. Check tails and retries last to estimate how bad the user impact is.

Common mistakes
- Starting with root-cause speculation
- Assuming one subsystem can explain every symptom
- Missing the difference between saturation and failure

Diagnostic flow
- Slow requests -> latency analysis
- High run queue -> CPU contention
- Low headroom -> memory or storage saturation
- Retries or drops -> network or IO path issues

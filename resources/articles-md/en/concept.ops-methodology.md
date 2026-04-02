# Operations Methodology

What is this?
A practical loop for turning a symptom into evidence, then evidence into action.

Why it matters
Without a method, teams jump to fixes before they know what layer is failing.

How to use
1. Name the symptom in one sentence.
2. Split the stack into app, kernel, device, and hardware.
3. Read at least one capacity metric and one pressure or latency metric.
4. Verify whether the issue is local, repeated, or workload-wide.

Common mistakes
- Starting from the noisiest graph
- Fixing symptoms without checking blast radius
- Treating a single metric as proof

Diagnostic flow
- User impact first -> look for latency and pressure
- Resource saturation -> look for headroom and queueing
- Change-related -> compare against the last deploy or config change

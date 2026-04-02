# Incident Workflow

What is this?
The sequence of steps that keeps a bad event from becoming a worse one.

Why it matters
An incident is about control and learning, not only about the first fix.

How to use
- Detect: decide whether the issue is active and user-visible
- Stabilize: reduce blast radius, roll back, throttle, or isolate
- Diagnose: gather the smallest set of metrics that can explain the symptom
- Recover: restore service before perfect root cause analysis
- Learn: record change, cause, mitigation, and follow-up owner

Common mistakes
- Spending too long on root cause before service is stable
- Skipping timestamps and change context
- Forgetting to write down what was actually observed

Diagnostic flow
1. Protect users first.
2. Capture evidence second.
3. Restore service third.
4. Write the postmortem last.

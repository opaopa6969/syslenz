# SLO Thinking

What is this?
Thinking in service level objectives, not just in infrastructure metrics.

Why it matters
Infrastructure health matters because it affects user outcomes. SLOs keep the conversation tied to user impact.

How to use
- Define the service that users actually care about
- Choose SLIs that reflect correctness, latency, or availability
- Set an objective that leaves room for normal variance
- Track error budget burn so you know when to pause risky change

Common mistakes
- Using internal metrics as the objective itself
- Setting objectives without a failure budget
- Treating every alert as an SLO violation

Diagnostic flow
1. Is the user-facing SLI degraded?
2. Is the error budget burning fast or slow?
3. Which subsystem is the best explanation for the SLI movement?
4. Should you ship, pause, or mitigate?

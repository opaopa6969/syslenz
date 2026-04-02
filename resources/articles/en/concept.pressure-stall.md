# Pressure Stall Information (PSI)

## NAME
`concept.pressure-stall` - foundational systems concept for diagnosis

## WHY NOW
Read this when the team needs a shared systems model before touching production knobs.
If discussion is opinion-heavy, use this article to convert claims into measurable checks.

## EVIDENCE ORDER
1. Fix user-visible symptom and time window first.
2. Verify this article's primary signal trend.
3. Cross-check one sibling signal and one cross-layer signal.
4. Apply reversible mitigation and confirm trend recovery.

## SEE ALSO
Use related links in the article overlay to continue the same evidence chain.

## Why PSI Exists
CPU%, memory usage, and I/O throughput tell you *how much* work happens. PSI tells you *how long work had to wait* because a resource was unavailable.

## Core Interpretation
- `some`: at least one task stalled.
- `full`: all runnable tasks in that scope stalled.
- `avg10/avg60/avg300`: moving averages by window.
- `total`: cumulative stalled time.

## Episode
A service showed moderate CPU and memory usage, yet user latency kept degrading. PSI `some` and `total` rose continuously, proving contention pressure despite acceptable utilization charts.

## Reading Protocol (Systems Lens)
1. Start with `avg10` for immediate pressure pulses.
2. Compare with `avg60/avg300` to detect persistence.
3. Use `total` growth rate to validate chronic stall accumulation.
4. Cross-check with runqueue/reclaim/queue-depth indicators.

## Operational Heuristic
If utilization looks normal but PSI keeps rising, you are likely in a hidden contention phase. Treat it early before full saturation appears.

## Common Mistakes
- Reading PSI without workload phase context.
- Ignoring `total` because averages seem small.
- Treating PSI as CPU-only signal (it spans CPU/memory/I/O pressure domains).

## Systems Narrative (Systems)

This signal (concept.pressure-stall) is not only a number; it is an exposed edge of kernel state transitions.
This source becomes far more reliable when read as part of a cross-layer evidence chain.

### Episode: Dashboard Confidence vs User Pain (Systems)
- The dashboard looked green because systems averages stayed normal.
- User-facing latency regressed only in systems burst windows.
- This systems field moved first, and neighboring fields confirmed direction.
- The winning move was not a large systems tuning change, but narrowing uncertainty quickly.

### Cross-Layer Translation (Systems)
1. Translate field movement into a probable kernel path.
2. Verify whether scheduler delay or I/O wait explains wall-clock loss.
3. Separate demand growth from service-time growth.
4. Confirm post-change recovery in both symptom and mechanism.

### What Senior Reviewers Usually Ask (Systems)
- Which systems counter moved first in time order?
- Which systems counter looked persuasive but was later demoted to a side effect?
- Which systems execution path likely carried the user-visible penalty?
- Which systems mitigation was reversible and what rollback trigger was defined?

### Combining With Unix Internals (Systems)
- Process model (Systems lens): did runnable tasks increase, or did blocked tasks accumulate?
- Syscall lifecycle (Systems lens): where did request time shift (entry, sleep, wakeup, return)?
- Interrupt path (Systems lens): did wakeup delivery or softirq backlog alter tail behavior?
- Scheduler (Systems lens): did fairness protect throughput while harming tail latency?

### Practical Mentor Notes (Systems)
Treat pressure-stall as one scene in a longer diagnostic narrative.
The systems narrative quality matters more than single-point precision: strong incidents are solved by ordered evidence, explicit assumptions, and controlled experiments.

## Incident Lab (Systems)

### Drill A: First-Mover Detection (Systems)
1. Pick one incident window and annotate first movement among three related signals.
2. Record one wrong hypothesis that looked plausible at first.
3. Explain why time order invalidated that hypothesis.

### Drill B: Reversible Mitigation Design (Systems)
1. Define one mitigation that can be rolled back in less than five minutes.
2. Define one explicit rollback condition before applying it.
3. Track symptom and mechanism separately after the change.

### Drill C: Evidence Compression (Systems)
1. Write a six-line narrative: symptom, first signal, second signal, action, reaction, conclusion.
2. Remove adjectives and keep only testable statements.
3. Hand the narrative to another engineer and check if they can reproduce your reasoning.

### Review Outcome (Systems)
If your team can replay this systems article as a short diagnostic script, the article is operationally useful.

## Quick Checklist (Systems)
- Identify one systems-affected user-facing symptom and timestamp.
- Identify one first-moving systems signal.
- Identify one cross-layer systems confirmation signal.
- State one reversible systems action.
- State one systems rollback condition.
- Verify systems trend recovery after action.

## Advanced Practice Notes

### Focus
Conceptual model calibration around pressure-stall.

### Operator Exercise
1. Write one failure narrative where this article was primary evidence.
2. Write one narrative where this article was only a supporting clue.
3. Compare both and identify the earliest divergence point.

### Escalation Boundary
- Escalate when symptom severity rises while this signal remains ambiguous.
- Avoid escalation if one more cross-layer check can resolve uncertainty in minutes.

### Coaching Prompt
Ask a junior engineer to explain this article without charts.
If they can state cause candidates, disproof steps, and rollback-safe action, the understanding is operational.

## Micro Episodes

### Episode 1
Capacity looked sufficient until reclaim timing collided with workload burst shape.

### Episode 2
Changing allocation rhythm reduced pressure faster than increasing total memory.

### Use In Review
In post-incident review, ask which episode pattern is closer to the observed timeline and which evidence is still missing.

## Incident Forensics

### Evidence Capture
- Convert this concept into one measurable hypothesis and one disproof step before incident action.
- Record where this concept changed team decisions; that history becomes reusable engineering capital.

### Decision Record
- Primary claim: concept.pressure-stall indicated a meaningful state transition.
- Disproof attempt: identify one alternate cause and log why it failed.
- Action note: pressure-stall was treated as evidence in a chain, not a singleton verdict.

## Man-Page Crosswalk
- Process lens: Process: express this concept as a runnable or blocked state hypothesis.
- Syscall lens: Syscall: identify one boundary where this concept becomes measurable.
- Scheduler lens: Scheduler: define one fairness or contention check.
- Interrupt or IO lens: Interrupt or IO: state one external signal that can falsify your claim.
- Field anchor: pressure-stall
- Source anchor: concept

## Failure Archetype Matrix
- Archetype A: concept understood verbally but not translated into measurable checks.
- Archetype B: one-layer optimization applied to a cross-layer bottleneck.
- Archetype C: plausible narrative chosen without explicit disproof path.
- Field in focus: pressure-stall

## Counterfactual Branches
1. If this concept were wrong, which metric pair would falsify it first?
2. If a junior engineer follows this concept, what single mistake is most likely?
3. What conceptual mismatch in evidence would invalidate your current mitigation immediately?

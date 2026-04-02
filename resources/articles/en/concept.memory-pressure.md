# Memory Pressure

## NAME
`concept.memory-pressure` - foundational systems concept for diagnosis

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

## Opening Scene
An on-call page arrives when dashboards are noisy and explanations conflict. This article gives a mental model that turns raw numbers into decisions under time pressure.

## Core Thesis
`Memory Pressure` is less about one metric and more about choosing the right layer, timeline, and comparison set.
- Layer: application, kernel, driver, hardware
- Timeline: immediate spike, sustained drift, recovery curve
- Comparison: local field, sibling fields, user-facing symptom

## Field Episode (Systems Lens)
A realistic incident pattern:
1. The first chart looked alarming.
2. The obvious hypothesis was wrong.
3. Cross-checking two neighboring signals changed the diagnosis.
4. Service recovered after a smaller, reversible action than expected.

The lesson: correctness in the first 10 minutes comes from structure, not heroics.

## Practical Reading Protocol
1. Define the symptom in user terms first (latency, error, throughput).
2. Pick 3-5 signals that can falsify your first hypothesis.
3. Separate cause, amplifier, and side effect.
4. Record timestamps and actions while investigating.
5. Prefer reversible mitigations before deep invasive changes.

## Common Failure Modes
- Treating one high value as root cause.
- Ignoring trend direction and only reading absolute numbers.
- Mixing layers (app blame for driver behavior, or reverse).
- Optimizing internals while users are still impacted.

## Engineer's Checklist
- What changed first?
- What changed second because of it?
- Which metric reflects user pain most directly?
- What action reduces risk immediately?
- What should be documented for the next incident?

## Practice Drill
Take one recent alert and replay it using this framework. Write a 10-line mini postmortem focused on evidence order, not intuition order.

## Systems Narrative (Memory)

This signal (concept.memory-pressure) is not only a number; it is an exposed edge of kernel state transitions.
This source becomes far more reliable when read as part of a cross-layer evidence chain.

### Episode: Dashboard Confidence vs User Pain (Memory)
- The dashboard looked green because memory averages stayed normal.
- User-facing latency regressed only in memory burst windows.
- This memory field moved first, and neighboring fields confirmed direction.
- The winning move was not a large memory tuning change, but narrowing uncertainty quickly.

### Cross-Layer Translation (Memory)
1. Translate field movement into a probable kernel path.
2. Verify whether scheduler delay or I/O wait explains wall-clock loss.
3. Separate demand growth from service-time growth.
4. Confirm post-change recovery in both symptom and mechanism.

### What Senior Reviewers Usually Ask (Memory)
- Which memory counter moved first in time order?
- Which memory counter looked persuasive but was later demoted to a side effect?
- Which memory execution path likely carried the user-visible penalty?
- Which memory mitigation was reversible and what rollback trigger was defined?

### Combining With Unix Internals (Memory)
- Process model (Memory lens): did runnable tasks increase, or did blocked tasks accumulate?
- Syscall lifecycle (Memory lens): where did request time shift (entry, sleep, wakeup, return)?
- Interrupt path (Memory lens): did wakeup delivery or softirq backlog alter tail behavior?
- Scheduler (Memory lens): did fairness protect throughput while harming tail latency?

### Practical Mentor Notes (Memory)
Treat memory-pressure as one scene in a longer diagnostic narrative.
The memory narrative quality matters more than single-point precision: strong incidents are solved by ordered evidence, explicit assumptions, and controlled experiments.

## Incident Lab (Memory)

### Drill A: First-Mover Detection (Memory)
1. Pick one incident window and annotate first movement among three related signals.
2. Record one wrong hypothesis that looked plausible at first.
3. Explain why time order invalidated that hypothesis.

### Drill B: Reversible Mitigation Design (Memory)
1. Define one mitigation that can be rolled back in less than five minutes.
2. Define one explicit rollback condition before applying it.
3. Track symptom and mechanism separately after the change.

### Drill C: Evidence Compression (Memory)
1. Write a six-line narrative: symptom, first signal, second signal, action, reaction, conclusion.
2. Remove adjectives and keep only testable statements.
3. Hand the narrative to another engineer and check if they can reproduce your reasoning.

### Review Outcome (Memory)
If your team can replay this memory article as a short diagnostic script, the article is operationally useful.

## Quick Checklist (Memory)
- Identify one memory-affected user-facing symptom and timestamp.
- Identify one first-moving memory signal.
- Identify one cross-layer memory confirmation signal.
- State one reversible memory action.
- State one memory rollback condition.
- Verify memory trend recovery after action.

## Advanced Practice Notes

### Focus
Conceptual model calibration around memory-pressure.

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
- Primary claim: concept.memory-pressure indicated a meaningful state transition.
- Disproof attempt: identify one alternate cause and log why it failed.
- Action note: memory-pressure was treated as evidence in a chain, not a singleton verdict.

## Man-Page Crosswalk
- Process lens: Process: express this concept as a runnable or blocked state hypothesis.
- Syscall lens: Syscall: identify one boundary where this concept becomes measurable.
- Scheduler lens: Scheduler: define one fairness or contention check.
- Interrupt or IO lens: Interrupt or IO: state one external signal that can falsify your claim.
- Field anchor: memory-pressure
- Source anchor: concept

## Failure Archetype Matrix
- Archetype A: concept understood verbally but not translated into measurable checks.
- Archetype B: one-layer optimization applied to a cross-layer bottleneck.
- Archetype C: plausible narrative chosen without explicit disproof path.
- Field in focus: memory-pressure

## Counterfactual Branches
1. If this concept were wrong, which metric pair would falsify it first?
2. If a junior engineer follows this concept, what single mistake is most likely?
3. What conceptual mismatch in evidence would invalidate your current mitigation immediately?

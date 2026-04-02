# max_temp

## NAME
`thermal.max_temp` - metric signal from `thermal` (max_temp)

## WHY NOW
Read this when noisy telemetry must be turned into a concrete operational decision.
If cause, symptom, and side effect are mixed, use this article to structure next checks.

## EVIDENCE ORDER
1. Fix user-visible symptom and time window first.
2. Verify this article's primary signal trend.
3. Cross-check one sibling signal and one cross-layer signal.
4. Apply reversible mitigation and confirm trend recovery.

## SEE ALSO
Use related links in the article overlay to continue the same evidence chain.

## Metric Snapshot
- ID: `thermal.max_temp`
- Source: `thermal`
- Field: `max_temp`
- Domain: system behavior

## Why This Metric Is Operationally Valuable
This metric helps separate normal workload expansion from unstable behavior. It is most reliable when read with neighboring fields and time trend.

## Episode
A familiar operations pattern: one metric looked abnormal, but correlated signals changed the diagnosis and mitigation plan.

In this incident pattern, this metric appears early in the evidence chain, but becomes actionable only after cross-source validation.

## Reading Strategy
1. Check current value and direction.
2. Compare short-term trend in Diff/Graph.
3. Pair one sibling metric in `thermal` plus one pressure/queue metric from another source.
4. Map movement to user-impact hypothesis.

## Decision Signals
- Low risk: load-proportional movement with fast recovery.
- Warning: trend persists after load normalization.
- Critical: related metrics co-move and recovery slope degrades.

## Misread Patterns
- Absolute-value judgment without workload context.
- Ignoring recovery slope after mitigation.
- Mixing symptom metrics and causal metrics.

## Action Loop
1. State a falsifiable hypothesis.
2. Apply reversible mitigation.
3. Verify with 2-3 correlated metrics.
4. Keep concise evidence notes for postmortem reuse.

## Unix Internals Lens

This field is a manifestation of **Unix kernel execution side effects**.

- Think in terms of layer: process, syscall, scheduler, memory, I/O, interrupt.
- Identify where this field sits in that layer map.
- Validate with one neighboring field and one cross-layer field.

## Systems Narrative (Systems)

This signal (thermal.max_temp) is not only a number; it is an exposed edge of kernel state transitions.
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
Treat max_temp as one scene in a longer diagnostic narrative.
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

## Incident Forensics

### Evidence Capture
- Anchor analysis on time order, not magnitude alone.
- Prefer reversible action and explicit rollback guardrails while uncertainty remains.

### Decision Record
- Primary claim: thermal.max_temp indicated a meaningful state transition.
- Disproof attempt: identify one alternate cause and log why it failed.
- Action note: max_temp was treated as evidence in a chain, not a singleton verdict.

## Man-Page Crosswalk
- Process lens: Process: decide whether this is demand growth or service degradation.
- Syscall lens: Syscall: mark one candidate path for time attribution.
- Scheduler lens: Scheduler: validate runqueue and wake behavior before tuning.
- Interrupt or IO lens: Interrupt or IO: cross-check one hardware-adjacent signal.
- Field anchor: max_temp
- Source anchor: thermal

## Failure Archetype Matrix
- Archetype A: magnitude-focused reading without sequence context.
- Archetype B: mitigation overreach under high uncertainty.
- Archetype C: symptom-mechanism mismatch after partial recovery.
- Field in focus: max_temp

## Counterfactual Branches
1. If this signal is secondary, what primary signal should have moved first?
2. If mitigation is rolled back, which metric should recover first and why?
3. What source-specific observation would invalidate your current mitigation immediately?

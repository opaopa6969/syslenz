# filesystems

## NAME
`df.filesystems` - metric signal from `df` (filesystems)

## WHY NOW
Read this when storage symptoms appear inconsistent between average throughput and tail latency.
If you cannot tell queueing from device slowdown, use this article before changing filesystem knobs.

## EVIDENCE ORDER
1. Fix user-visible symptom and time window first.
2. Verify this article's primary signal trend.
3. Cross-check one sibling signal and one cross-layer signal.
4. Apply reversible mitigation and confirm trend recovery.

## SEE ALSO
Use related links in the article overlay to continue the same evidence chain.

## Metric Snapshot
- ID: `df.filesystems`
- Source: `df`
- Field: `filesystems`
- Domain: system behavior

## Why This Metric Is Operationally Valuable
This metric helps separate normal workload expansion from unstable behavior. It is most reliable when read with neighboring fields and time trend.

## Episode
A familiar operations pattern: one metric looked abnormal, but correlated signals changed the diagnosis and mitigation plan.

In this incident pattern, this metric appears early in the evidence chain, but becomes actionable only after cross-source validation.

## Reading Strategy
1. Check current value and direction.
2. Compare short-term trend in Diff/Graph.
3. Pair one sibling metric in `df` plus one pressure/queue metric from another source.
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

## Systems Narrative (Storage)

This signal (df.filesystems) is not only a number; it is an exposed edge of kernel state transitions.
This source becomes far more reliable when read as part of a cross-layer evidence chain.

### Episode: Dashboard Confidence vs User Pain (Storage)
- The dashboard looked green because storage averages stayed normal.
- User-facing latency regressed only in storage burst windows.
- This storage field moved first, and neighboring fields confirmed direction.
- The winning move was not a large storage tuning change, but narrowing uncertainty quickly.

### Cross-Layer Translation (Storage)
1. Translate field movement into a probable kernel path.
2. Verify whether scheduler delay or I/O wait explains wall-clock loss.
3. Separate demand growth from service-time growth.
4. Confirm post-change recovery in both symptom and mechanism.

### What Senior Reviewers Usually Ask (Storage)
- Which storage counter moved first in time order?
- Which storage counter looked persuasive but was later demoted to a side effect?
- Which storage execution path likely carried the user-visible penalty?
- Which storage mitigation was reversible and what rollback trigger was defined?

### Combining With Unix Internals (Storage)
- Process model (Storage lens): did runnable tasks increase, or did blocked tasks accumulate?
- Syscall lifecycle (Storage lens): where did request time shift (entry, sleep, wakeup, return)?
- Interrupt path (Storage lens): did wakeup delivery or softirq backlog alter tail behavior?
- Scheduler (Storage lens): did fairness protect throughput while harming tail latency?

### Practical Mentor Notes (Storage)
Treat filesystems as one scene in a longer diagnostic narrative.
The storage narrative quality matters more than single-point precision: strong incidents are solved by ordered evidence, explicit assumptions, and controlled experiments.

## Incident Lab (Storage)

### Drill A: First-Mover Detection (Storage)
1. Pick one incident window and annotate first movement among three related signals.
2. Record one wrong hypothesis that looked plausible at first.
3. Explain why time order invalidated that hypothesis.

### Drill B: Reversible Mitigation Design (Storage)
1. Define one mitigation that can be rolled back in less than five minutes.
2. Define one explicit rollback condition before applying it.
3. Track symptom and mechanism separately after the change.

### Drill C: Evidence Compression (Storage)
1. Write a six-line narrative: symptom, first signal, second signal, action, reaction, conclusion.
2. Remove adjectives and keep only testable statements.
3. Hand the narrative to another engineer and check if they can reproduce your reasoning.

### Review Outcome (Storage)
If your team can replay this storage article as a short diagnostic script, the article is operationally useful.

## Quick Checklist (Storage)
- Identify one storage-affected user-facing symptom and timestamp.
- Identify one first-moving storage signal.
- Identify one cross-layer storage confirmation signal.
- State one reversible storage action.
- State one storage rollback condition.
- Verify storage trend recovery after action.

## Incident Forensics

### Evidence Capture
- Anchor analysis on time order, not magnitude alone.
- Prefer reversible action and explicit rollback guardrails while uncertainty remains.

### Decision Record
- Primary claim: df.filesystems indicated a meaningful state transition.
- Disproof attempt: identify one alternate cause and log why it failed.
- Action note: filesystems was treated as evidence in a chain, not a singleton verdict.

## Man-Page Crosswalk
- Process lens: Process: identify writers and flush cadence.
- Syscall lens: Syscall: map fsync or write bursts to queue behavior.
- Scheduler lens: Scheduler: separate waiting-to-run from waiting-for-device.
- Interrupt or IO lens: Interrupt: check completion cadence under burst windows.
- Field anchor: filesystems
- Source anchor: df

## Failure Archetype Matrix
- Archetype A: queue-depth inflation before visible throughput loss.
- Archetype B: flush cadence mismatch between producer and device service rhythm.
- Archetype C: tail-latency pain hidden by healthy average throughput.
- Field in focus: filesystems

## Counterfactual Branches
1. If write burst amplitude were halved, would wait distribution still stay heavy?
2. If queue depth dropped but p99 stayed high, what path outside storage explains it?
3. What storage-side observation would invalidate your current mitigation immediately?

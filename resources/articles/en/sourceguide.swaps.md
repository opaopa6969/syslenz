# Source Guide: swaps

## NAME
`sourceguide.swaps` - source-level operational reading guide

## WHY NOW
Read this when you need to interpret one source without losing cross-source context.
If a single source looks convincing, use this article to validate what it cannot prove alone.

## EVIDENCE ORDER
1. Fix user-visible symptom and time window first.
2. Verify this article's primary signal trend.
3. Cross-check one sibling signal and one cross-layer signal.
4. Apply reversible mitigation and confirm trend recovery.

## SEE ALSO
Use related links in the article overlay to continue the same evidence chain.

## Why This Source Exists
`swaps` is a protocol-oriented telemetry surface. It helps connect socket behavior, packet lifecycle, and user-visible latency.

## Episode From Operations
A service looked healthy at CPU and memory level, but protocol-level counters in this source exposed retry and state-transition anomalies that explained intermittent timeouts.

## How To Read This Source
1. Start from stable baseline counters.
2. Track which counters move first during load spikes.
3. Compare trend with sibling network sources (`ss`, conntrack, pressure).
4. Map counter drift to connection lifecycle stages.

## Pattern Library
- Healthy: counters scale with traffic and settle quickly.
- Warning: specific error/retry counters trend independently of traffic growth.
- Critical: multiple error paths rise while successful progression stalls.

## Suggested Workflow
1. Mark first anomaly timestamp.
2. Cross-check one socket-level and one pressure-level source.
3. Decide mitigation (rate shaping, timeout tuning, retry policy adjustments).
4. Document evidence chain for repeat incidents.

## Unix Internals Lens

This field is a manifestation of **Unix kernel execution side effects**.

- Think in terms of layer: process, syscall, scheduler, memory, I/O, interrupt.
- Identify where this field sits in that layer map.
- Validate with one neighboring field and one cross-layer field.

## Systems Narrative (Memory)

This signal (sourceguide.swaps) is not only a number; it is an exposed edge of kernel state transitions.
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
Treat swaps as one scene in a longer diagnostic narrative.
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
Source-level operational literacy for swaps.

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
This source became useful only after timeline alignment with a sibling source and one cross-layer signal.

### Episode 2
The strongest decision came from sequence consistency, not absolute magnitude.

### Use In Review
In post-incident review, ask which episode pattern is closer to the observed timeline and which evidence is still missing.

## Incident Forensics

### Evidence Capture
- Use this source as an index into neighboring sources rather than a standalone authority.
- The reading quality improves when you can explain what this source cannot prove by itself.

### Decision Record
- Primary claim: sourceguide.swaps indicated a meaningful state transition.
- Disproof attempt: identify one alternate cause and log why it failed.
- Action note: swaps was treated as evidence in a chain, not a singleton verdict.

## Man-Page Crosswalk
- Process lens: Process: decide whether this is demand growth or service degradation.
- Syscall lens: Syscall: mark one candidate path for time attribution.
- Scheduler lens: Scheduler: validate runqueue and wake behavior before tuning.
- Interrupt or IO lens: Interrupt or IO: cross-check one hardware-adjacent signal.
- Field anchor: swaps
- Source anchor: sourceguide

## Failure Archetype Matrix
- Archetype A: source treated as verdict instead of index into neighboring evidence.
- Archetype B: threshold crossing fixation while trend-shape evidence is ignored.
- Archetype C: mitigation attempted before confirming cross-layer sequence.
- Field in focus: swaps

## Counterfactual Branches
1. If this source is removed, which two sources can reconstruct the same conclusion?
2. If values stay normal but user pain grows, what trend clue was likely missed?
3. What neighboring-source observation would invalidate your current mitigation immediately?

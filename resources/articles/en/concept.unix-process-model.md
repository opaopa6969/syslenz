# Unix Process Model: Fork, Exec, Wait

## NAME
`concept.unix-process-model` - foundational systems concept for diagnosis

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

## Why This Matters For /proc Reading
Many `/proc` fields are snapshots of process lifecycle side effects. If you understand `fork -> exec -> wait`, process counters stop being abstract numbers and become an execution story.

## Mental Model
- `fork`: duplicate process state (cheap in modern kernels via copy-on-write).
- `exec`: replace process image with a new program.
- `wait`: parent collects child exit status to avoid zombies.

In production, incidents often appear as abnormal ratios between these phases, not as one single bad value.

## Episode: "Everything Is Slow, CPU Is Fine"
A service reported latency spikes while CPU stayed below 50%.
- `process_count` and `forks_total` climbed sharply.
- `runqueue` was elevated but not pegged.
- Logs showed frequent short-lived worker restarts.

Root cause: process churn (`fork/exec` storm), not raw compute shortage.

## What To Check In syslenz
1. `stat.process_count` and `stat.forks_total` trend.
2. scheduler-related indicators (`schedstat.*`, runqueue distributions).
3. memory side effects (page faults, reclaim activity) when churn is high.
4. context switches and softirq behavior if process creation drives kernel work.

## Common Misreads
- "CPU low means healthy": false when process churn dominates overhead.
- "High load means CPU shortage": maybe blocked tasks or startup storms.
- "Process count up = memory leak": sometimes just rollout/restart behavior.

## Practical Triage Protocol
1. Confirm user symptom (latency/error/throughput).
2. Verify process lifecycle anomaly exists over multiple snapshots.
3. Separate expected scaling from pathological churn.
4. Mitigate with backoff/restart policy/worker model adjustments.
5. Re-check trend recovery after mitigation.

## Deeper Engineering Notes
- `fork` cost moved from memory copy to page-table and TLB overhead in many workloads.
- `exec` cost depends on binary size, dynamic linking, filesystem cache state.
- orphan/zombie patterns can bias process counters without obvious CPU impact.

## Drill
Take one incident where "CPU wasn't high but service was slow" and map evidence into:
- lifecycle anomaly
- scheduler consequence
- user impact chain

## Systems Narrative (Process)

This signal (concept.unix-process-model) is not only a number; it is an exposed edge of kernel state transitions.
This source becomes far more reliable when read as part of a cross-layer evidence chain.

### Episode: Dashboard Confidence vs User Pain (Process)
- The dashboard looked green because process averages stayed normal.
- User-facing latency regressed only in process burst windows.
- This process field moved first, and neighboring fields confirmed direction.
- The winning move was not a large process tuning change, but narrowing uncertainty quickly.

### Cross-Layer Translation (Process)
1. Translate field movement into a probable kernel path.
2. Verify whether scheduler delay or I/O wait explains wall-clock loss.
3. Separate demand growth from service-time growth.
4. Confirm post-change recovery in both symptom and mechanism.

### What Senior Reviewers Usually Ask (Process)
- Which process counter moved first in time order?
- Which process counter looked persuasive but was later demoted to a side effect?
- Which process execution path likely carried the user-visible penalty?
- Which process mitigation was reversible and what rollback trigger was defined?

### Combining With Unix Internals (Process)
- Process model (Process lens): did runnable tasks increase, or did blocked tasks accumulate?
- Syscall lifecycle (Process lens): where did request time shift (entry, sleep, wakeup, return)?
- Interrupt path (Process lens): did wakeup delivery or softirq backlog alter tail behavior?
- Scheduler (Process lens): did fairness protect throughput while harming tail latency?

### Practical Mentor Notes (Process)
Treat unix-process-model as one scene in a longer diagnostic narrative.
The process narrative quality matters more than single-point precision: strong incidents are solved by ordered evidence, explicit assumptions, and controlled experiments.

## Incident Lab (Process)

### Drill A: First-Mover Detection (Process)
1. Pick one incident window and annotate first movement among three related signals.
2. Record one wrong hypothesis that looked plausible at first.
3. Explain why time order invalidated that hypothesis.

### Drill B: Reversible Mitigation Design (Process)
1. Define one mitigation that can be rolled back in less than five minutes.
2. Define one explicit rollback condition before applying it.
3. Track symptom and mechanism separately after the change.

### Drill C: Evidence Compression (Process)
1. Write a six-line narrative: symptom, first signal, second signal, action, reaction, conclusion.
2. Remove adjectives and keep only testable statements.
3. Hand the narrative to another engineer and check if they can reproduce your reasoning.

### Review Outcome (Process)
If your team can replay this process article as a short diagnostic script, the article is operationally useful.

## Quick Checklist (Process)
- Identify one process-affected user-facing symptom and timestamp.
- Identify one first-moving process signal.
- Identify one cross-layer process confirmation signal.
- State one reversible process action.
- State one process rollback condition.
- Verify process trend recovery after action.

## Incident Forensics

### Evidence Capture
- Convert this concept into one measurable hypothesis and one disproof step before incident action.
- Record where this concept changed team decisions; that history becomes reusable engineering capital.

### Decision Record
- Primary claim: concept.unix-process-model indicated a meaningful state transition.
- Disproof attempt: identify one alternate cause and log why it failed.
- Action note: unix-process-model was treated as evidence in a chain, not a singleton verdict.

## Man-Page Crosswalk
- Process lens: Process: express this concept as a runnable or blocked state hypothesis.
- Syscall lens: Syscall: identify one boundary where this concept becomes measurable.
- Scheduler lens: Scheduler: define one fairness or contention check.
- Interrupt or IO lens: Interrupt or IO: state one external signal that can falsify your claim.
- Field anchor: unix-process-model
- Source anchor: concept

## Failure Archetype Matrix
- Archetype A: concept understood verbally but not translated into measurable checks.
- Archetype B: one-layer optimization applied to a cross-layer bottleneck.
- Archetype C: plausible narrative chosen without explicit disproof path.
- Field in focus: unix-process-model

## Counterfactual Branches
1. If this concept were wrong, which metric pair would falsify it first?
2. If a junior engineer follows this concept, what single mistake is most likely?
3. What conceptual mismatch in evidence would invalidate your current mitigation immediately?

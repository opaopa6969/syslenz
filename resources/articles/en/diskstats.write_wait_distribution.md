# write_wait_distribution

## NAME
`diskstats.write_wait_distribution` - metric signal from `diskstats` (write_wait_distribution)

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
- ID: `diskstats.write_wait_distribution`
- Source: `diskstats`
- Field: `write_wait_distribution`
- Domain: device queueing and storage latency
- Signal family: I/O queue dynamics

## Operational Meaning (Storage Lens)
This field is strongest for distinguishing queueing growth from device-service deterioration before averages hide the tail risk.

## Field Episode (Storage Lens)
Queue-related counters moved first; throughput remained high but waiting time distribution worsened.

For `write_wait_distribution`, the practical value comes from ordering evidence in time: what moved first, what followed, and what changed after mitigation.

## Reading Protocol (Storage Lens)
1. Confirm current direction (rising/falling/flat) and short-term slope.
2. Compare against sibling fields in `diskstats` to avoid single-metric bias.
3. Cross-check one queue/stall/pressure metric from another source.
4. Map the movement to user impact (latency, error, throughput) before acting.

## Decision Heuristic (Storage Family)
If this field rises while sibling pressure fields stay flat, verify whether the change is workload-shape related rather than capacity failure.

## Failure Patterns To Avoid (Storage Family)
- Root-cause declaration from one snapshot of queue or wait state.
- Ignoring post-mitigation recovery shape in storage timeline.
- Confusing cross-layer storage correlation with causation.

## Action Loop
1. State a falsifiable hypothesis for `write_wait_distribution`.
2. Apply reversible mitigation.
3. Validate with 2-3 correlated fields over multiple refreshes.
4. Keep concise evidence notes for postmortem reuse.

## Unix Internals Lens

This field is a manifestation of **block-layer queue and completion behavior**.

- Kernel path: bio -> request queue -> driver -> device completion.
- Typical trigger: queue depth growth, mixed R/W contention, firmware variance.
- Cross-check: iowait, writeback, and application timeout patterns.

## Casebook (Storage Family)

### Incident Slice 1 (Storage)
Case C (retries hurt): Retry policy increased queue occupancy and lengthened recovery.

### Incident Slice 2 (Storage)
Case A (queue-first failure): Queue depth and wait distribution degraded before error rate changed. Early queue control prevented customer-visible outage.

### Incident Slice 3 (Storage)
Case B (throughput illusion): High throughput masked severe fairness loss between request classes.

## Failure Branches (Storage Family)
- Branch 1: Symptom improves but storage-trend does not. -> Revisit storage causal layer assumption.
- Branch 2: Storage-trend improves but symptom does not. -> Inspect parallel CPU or network bottleneck chain.
- Branch 3: Both worsen after mitigation. -> Roll back quickly and preserve storage evidence snapshot.
- Branch 4: Short recovery then relapse. -> Check queue and retry feedback loops.

## Runbook Drill (Storage Lens)
1. Pick a 15-minute incident window and annotate T0/T1/T2 events.
2. Build a three-signal chain: primary field, sibling field, cross-layer field.
3. Write one falsifiable hypothesis and one rollback-safe mitigation.
4. Define success as trend recovery + user symptom recovery, not one chart turning green.

## MAN Notes (Storage Lens)
- This section mirrors man-page flow for storage latency: definition -> queue context -> failure branches -> evidence order.
- Prefer explicit timestamps and queue-depth notes; storage narratives drift without queue context.
- If uncertain, follow SEE ALSO links before changing production storage knobs.

## Deep Appendix: Counterfactuals and Review Prompts (Storage Family)

### Counterfactual Questions (Storage)
- If traffic had stayed constant, would queue and wait transitions still move this field the same way?
- If this field had remained flat, which non-storage signal could still explain the symptom?
- If mitigation was delayed by 10 minutes, which storage-sensitive user metric would have crossed first?
- If only one layer could be instrumented, which storage-adjacent layer would preserve most explanatory power?

### Timeline Template (Storage Incident)
- T-10m: baseline snapshot with queue-depth annotation
- T-5m: first wait-distribution anomaly candidate
- T0: user symptom confirmed with storage-side context
- T+3m: first hypothesis written with queue assumption
- T+6m: cross-source validation including scheduler or pressure
- T+10m: mitigation applied with rollback guard
- T+15m: trend reaction checked for wait normalization
- T+30m: recovery confidence decision with tail-latency confirmation

### Evidence Quality Rubric (Storage)
- Strong: ordered, cross-layer, and queue-consistent trend evidence
- Medium: correlated movement without queue/service-time separation
- Weak: isolated value with no wait-distribution context

### Postmortem Questions (Storage)
1. What evidence changed the team decision most?
2. Which metric looked convincing but was later proven secondary?
3. What assumption was left implicit and should be made explicit next time?
4. Which alert would have triggered earlier with lower noise?

### Anti-Drift Checklist (Storage)
- Keep one baseline note per environment, workload phase, and queue profile.
- Revalidate storage thresholds after release, firmware, or IO-pattern changes.
- Avoid cargo-cult tuning; require before and after evidence with queue and wait context.
- Link this article to at least two neighboring articles in your runbook.

## Incident Forensics

### Evidence Capture
- Capture a 30-minute timeline and mark the first structural shift before user-impact alerts.
- Pair this field with one scheduler signal and one storage or network signal to test cross-layer consistency.

### Decision Record
- Primary claim: diskstats.write_wait_distribution indicated a meaningful state transition.
- Disproof attempt: identify one alternate cause and log why it failed.
- Action note: write_wait_distribution was treated as evidence in a chain, not a singleton verdict.

## Man-Page Crosswalk
- Process lens: Process: identify writers and flush cadence.
- Syscall lens: Syscall: map fsync or write bursts to queue behavior.
- Scheduler lens: Scheduler: separate waiting-to-run from waiting-for-device.
- Interrupt or IO lens: Interrupt: check completion cadence under burst windows.
- Field anchor: write_wait_distribution
- Source anchor: diskstats

## Source Drillbook (Storage Family)

### Drill Steps
1. Track queue depth, wait distribution, and service rhythm in one timeline.
2. Distinguish bursty producer behavior from persistent device-side slowdown.
3. Validate whether p99 pain comes from merge inefficiency or flush cadence.
4. Apply one reversible burst-smoothing action before heavy tuning.
5. Capture before and after traces to prevent tuning folklore.

### Debrief Questions
- Which storage-side step produced the highest confidence gain?
- Which step failed to separate producer burst from device slowdown?
- What storage instrumentation change would accelerate this drill next time?

### Anchor (Storage)
Field under practice: write_wait_distribution

## Failure Archetype Matrix
- Archetype A: queue-depth inflation before visible throughput loss.
- Archetype B: flush cadence mismatch between producer and device service rhythm.
- Archetype C: tail-latency pain hidden by healthy average throughput.
- Field in focus: write_wait_distribution

## Counterfactual Branches
1. If write burst amplitude were halved, would wait distribution still stay heavy?
2. If queue depth dropped but p99 stayed high, what path outside storage explains it?
3. What storage-side observation would invalidate your current mitigation immediately?

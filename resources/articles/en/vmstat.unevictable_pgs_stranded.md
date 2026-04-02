# unevictable_pgs_stranded

## NAME
`vmstat.unevictable_pgs_stranded` - metric signal from `vmstat` (unevictable_pgs_stranded)

## WHY NOW
Read this when vmstat reclaim and scan behavior around `unevictable_pgs_stranded` is noisy and you need a fast, defensible decision.
If you cannot tell reclaim pressure around `unevictable_pgs_stranded` from side effect, use this article to order evidence before tuning.

## EVIDENCE ORDER
1. Fix user-visible symptom and time window first.
2. Verify this article's primary signal trend.
3. Cross-check one sibling signal and one cross-layer signal.
4. Apply reversible mitigation and confirm trend recovery.

## SEE ALSO
Use related links in the article overlay to continue the same evidence chain.

## Metric Snapshot
- ID: `vmstat.unevictable_pgs_stranded`
- Source: `vmstat` (`unevictable_pgs_stranded`)
- Field: `unevictable_pgs_stranded`
- Domain: virtual memory and reclaim behavior
- Signal family: baseline behavior and drift detection

## Operational Meaning (VMStat Lens)
This field is strongest for separating vmstat reclaim signal around `unevictable_pgs_stranded` from side-effect noise when scan and reclaim trends diverge.

## Field Episode (VMStat Lens)
The field looked quiet in isolation, but trend context changed the operational decision.

For `unevictable_pgs_stranded`, the practical value comes from ordering evidence in time: what moved first, what followed, and what changed after mitigation.

## Reading Protocol (VMStat Lens)
1. Confirm current direction (rising/falling/flat) and short-term slope.
2. Compare against sibling fields in `vmstat` to avoid single-metric bias.
3. Cross-check one queue/stall/pressure metric from another source.
4. Map the movement to user impact (latency, error, throughput) before acting.

## Decision Heuristic (VMStat Family)
If mitigation changes user latency but not this field trend, re-check the assumed causal layer.

## Failure Patterns To Avoid (VMStat Family)
- Root-cause declaration from one vmstat reclaim/scan snapshot.
- Ignoring post-mitigation recovery shape in vmstat timeline.
- Confusing cross-layer vmstat correlation with causation.

## Action Loop
1. State a falsifiable hypothesis for `unevictable_pgs_stranded`.
2. Apply reversible mitigation.
3. Validate with 2-3 correlated fields over multiple refreshes.
4. Keep concise evidence notes for postmortem reuse.

## Unix Internals Lens

This field is a manifestation of **virtual memory state transitions (`unevictable_pgs_stranded`)**.

- Kernel path: reclaim, compaction, page fault handling, swap policy.
- Typical trigger (`unevictable_pgs_stranded`): memory pressure or workload phase shift.
- Cross-check: scheduler delay and storage writeback side effects.

## Casebook (VMStat Family)

### Incident Slice 1 (VMStat)
Case A: First anomaly came from this field trend, not absolute value.

### Incident Slice 2 (VMStat)
Case B: Cross-source correlation reversed the initial diagnosis.

### Incident Slice 3 (VMStat)
Case C: Reversible mitigation provided faster learning than invasive change.

## Failure Branches (VMStat Family)
- Branch 1: Symptom improves but vmstat trend does not. -> Revisit vmstat causal layer assumption.
- Branch 2: vmstat trend improves but symptom does not. -> Inspect parallel CPU or IO bottleneck chain.
- Branch 3: Both worsen after mitigation. -> Roll back quickly and preserve vmstat evidence snapshot.
- Branch 4: Short recovery then relapse. -> Check vmstat reclaim and retry feedback loops.

## Runbook Drill (VMStat Lens)
1. Pick a 15-minute incident window and annotate T0/T1/T2 events.
2. Build a three-signal chain: primary field, sibling field, cross-layer field.
3. Write one falsifiable hypothesis and one rollback-safe mitigation.
4. Define success as trend recovery + user symptom recovery, not one chart turning green.

## MAN Notes (VMStat Lens)
- This section mirrors man-page flow for vmstat reclaim analysis of `unevictable_pgs_stranded`: definition -> scan/reclaim context -> failure branches -> evidence order.
- Prefer explicit timestamps and vmstat reclaim-phase notes; narratives drift without phase context.
- If uncertain, follow SEE ALSO links before changing production vmstat-related memory knobs.

## Deep Appendix: Counterfactuals and Review Prompts (VMStat Family)

### Counterfactual Questions (VMStat)
- If traffic had stayed constant, would vmstat reclaim and scan transitions still move this field the same way?
- If this field had remained flat, which non-vmstat signal could still explain the symptom?
- If mitigation was delayed by 10 minutes, which vmstat-linked user metric would have crossed first?
- If only one layer could be instrumented, which vmstat-adjacent layer would preserve most explanatory power?

### Timeline Template (VMStat Incident)
- T-10m: baseline snapshot with vmstat reclaim/scan phase annotation for `unevictable_pgs_stranded`
- T-5m: first vmstat reclaim or scan anomaly candidate around `unevictable_pgs_stranded`
- T0: user symptom confirmed with vmstat-side context around `unevictable_pgs_stranded`
- T+3m: first hypothesis written with vmstat reclaim assumption for `unevictable_pgs_stranded`
- T+6m: cross-source validation including scheduler or writeback for `unevictable_pgs_stranded`
- T+10m: mitigation applied with rollback guard in the vmstat path for `unevictable_pgs_stranded`
- T+15m: trend reaction checked for vmstat reclaim stabilization around `unevictable_pgs_stranded`
- T+30m: recovery confidence decision with vmstat and pressure confirmation for `unevictable_pgs_stranded`

### Evidence Quality Rubric (VMStat)
- Strong: ordered, cross-layer, and vmstat reclaim-consistent trend evidence for `unevictable_pgs_stranded`
- Medium: correlated movement without clear vmstat reclaim/scan boundary
- Weak: isolated value with no vmstat reclaim/scan context around `unevictable_pgs_stranded`

### Postmortem Questions (VMStat)
1. What evidence changed the team decision most?
2. Which metric looked convincing but was later proven secondary?
3. What assumption was left implicit and should be made explicit next time?
4. Which alert would have triggered earlier with lower noise?

### Anti-Drift Checklist (VMStat)
- Keep one baseline note per environment, workload phase, and vmstat reclaim mode.
- Revalidate vmstat-related thresholds after release, kernel, or allocator-behavior changes.
- Avoid cargo-cult tuning; require before and after evidence with vmstat+pressure context.
- Link this article to at least two neighboring vmstat or memory-path articles in your runbook.

## Incident Forensics

### Evidence Capture
- Capture a 30-minute timeline and mark the first reclaim or scan inflection before user-impact alerts.
- Pair this field with one scheduler signal and one writeback or swap signal to test cross-layer consistency.

### Decision Record
- Primary claim: vmstat.unevictable_pgs_stranded indicated a meaningful state transition.
- Disproof attempt: identify one alternate cause and log why it failed.
- Action note: unevictable_pgs_stranded was treated as evidence in a chain, not a singleton verdict.

## Man-Page Crosswalk
- Process lens: Process: check runnable vs blocked expansion around vmstat inflection windows.
- Syscall lens: Syscall: identify page-fault and memory-touching call bursts aligned with the vmstat transitions for `unevictable_pgs_stranded`.
- Scheduler lens: Scheduler: verify whether reclaim-side waiting inflated wake latency during vmstat spikes.
- Interrupt or IO lens: Storage or IO: validate writeback or page-in side effects around vmstat shifts.
- Field anchor: unevictable_pgs_stranded
- Source anchor: vmstat (`unevictable_pgs_stranded`)

## Source Drillbook (VMStat Family)

### Drill Steps
1. Run a 20-minute replay with three synchronized views: primary field, sibling field, and user symptom.
2. Mark one point where trend direction changed without alert threshold crossing.
3. Explain whether the change suggests reclaim pressure, allocation phase shift, or scheduler-side delay.
4. Write one rollback-safe mitigation and predefine stop conditions.
5. Document what evidence would force you to reject your first hypothesis.

### Debrief Questions
- Which vmstat-side step around `unevictable_pgs_stranded` produced the highest confidence gain?
- Which step failed to reduce uncertainty about reclaim pressure around `unevictable_pgs_stranded` versus side effect?
- What vmstat instrumentation change around `unevictable_pgs_stranded` would accelerate this drill next time?

### Anchor (VMStat)
Field under practice: unevictable_pgs_stranded

## Failure Archetype Matrix
- Archetype A (`unevictable_pgs_stranded`): silent vmstat reclaim pressure where utilization looks safe but stalls rise.
- Archetype B (`unevictable_pgs_stranded`): vmstat reclaim oscillation causing repeated short recoveries and relapses.
- Archetype C (`unevictable_pgs_stranded`): vmstat scan/reclaim phase shift with delayed user-impact visibility.
- Field in focus: unevictable_pgs_stranded

## Counterfactual Branches
1. If traffic had been flat, would this field still drift in the same direction?
2. If reclaim signals stabilized but latency stayed bad, which non-memory path becomes primary?
3. What memory-side observation would invalidate your current mitigation immediately?

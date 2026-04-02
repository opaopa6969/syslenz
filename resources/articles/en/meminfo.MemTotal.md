# MemTotal

## NAME
`meminfo.MemTotal` - metric signal from `meminfo` (MemTotal)

## WHY NOW
Read this when meminfo headroom and reclaimability behavior around `MemTotal` is noisy and you need a fast, defensible decision.
If you cannot tell headroom erosion around `MemTotal` from side effect, use this article to order evidence before tuning.

## EVIDENCE ORDER
1. Fix user-visible symptom and time window first.
2. Verify this article's primary signal trend.
3. Cross-check one sibling signal and one cross-layer signal.
4. Apply reversible mitigation and confirm trend recovery.

## SEE ALSO
Use related links in the article overlay to continue the same evidence chain.

## Metric Snapshot
- ID: `meminfo.MemTotal`
- Source: `meminfo`
- Field: `MemTotal`
- Domain: memory capacity and reclaim headroom
- Signal family: stall-time accumulation

## Operational Meaning (MemInfo Lens)
This field is strongest for separating meminfo headroom signal around `MemTotal` from side-effect noise when free/reclaimable trends diverge.

## Field Episode (MemInfo Lens)
Average pressure looked acceptable while cumulative stall time explained tail-latency degradation.

For `MemTotal`, the practical value comes from ordering evidence in time: what moved first, what followed, and what changed after mitigation.

## Reading Protocol (MemInfo Lens)
1. Confirm current direction (rising/falling/flat) and short-term slope.
2. Compare against sibling fields in `meminfo` to avoid single-metric bias.
3. Cross-check one queue/stall/pressure metric from another source.
4. Map the movement to user impact (latency, error, throughput) before acting.

## Decision Heuristic (MemInfo Family)
If this field normalizes quickly after load drop, bias toward burst explanation; if not, investigate persistent contention.

## Failure Patterns To Avoid (MemInfo Family)
- Root-cause declaration from one meminfo headroom/reclaimability snapshot.
- Ignoring post-mitigation recovery shape in meminfo timeline.
- Confusing cross-layer meminfo correlation with causation.

## Action Loop
1. State a falsifiable hypothesis for `MemTotal`.
2. Apply reversible mitigation.
3. Validate with 2-3 correlated fields over multiple refreshes.
4. Keep concise evidence notes for postmortem reuse.

## Unix Internals Lens

This field is a manifestation of **memory accounting surfaces (MemTotal)**.

- Kernel path (MemTotal): allocator, page cache, slab, reclaim boundaries.
- Typical trigger (MemTotal): cache growth/shrink, allocation bursts, background reclaim.
- Cross-check (MemTotal): vmstat reclaim counters and pressure metrics.

## Casebook (MemInfo Family)

### Incident Slice 1 (MemInfo)
Case B (burst storm): Short periodic stalls were invisible in coarse dashboards but aligned perfectly with user complaints.

### Incident Slice 2 (MemInfo)
Case C (cross-layer confusion): Team blamed network; stall evidence showed memory contention as first mover.

### Incident Slice 3 (MemInfo)
Case A (utilization trap): CPU utilization looked moderate while PSI total climbed steadily. Hidden waiting, not execution, consumed SLO budget.

## Failure Branches (MemInfo Family)
- Branch 1: Symptom improves but meminfo trend does not. -> Revisit meminfo causal layer assumption.
- Branch 2: meminfo trend improves but symptom does not. -> Inspect parallel CPU or IO bottleneck chain.
- Branch 3: Both worsen after mitigation. -> Roll back quickly and preserve meminfo evidence snapshot.
- Branch 4: Short recovery then relapse. -> Check meminfo headroom and retry feedback loops.

## Runbook Drill (MemInfo Lens)
1. Pick a 15-minute incident window and annotate T0/T1/T2 events.
2. Build a three-signal chain: primary field, sibling field, cross-layer field.
3. Write one falsifiable hypothesis and one rollback-safe mitigation.
4. Define success as trend recovery + user symptom recovery, not one chart turning green.

## MAN Notes (MemInfo Lens)
- This section mirrors man-page flow for meminfo analysis: definition -> headroom context -> failure branches -> evidence order.
- Prefer explicit timestamps and meminfo headroom notes; narratives drift without headroom context.
- If uncertain, follow SEE ALSO links before changing production meminfo-related memory knobs.

## Deep Appendix: Counterfactuals and Review Prompts (MemInfo Family)

### Counterfactual Questions (MemInfo)
- If traffic had stayed constant, would meminfo headroom transitions still move this field the same way?
- If this field had remained flat, which non-meminfo signal could still explain the symptom?
- If mitigation was delayed by 10 minutes, which meminfo-linked user metric would have crossed first?
- If only one layer could be instrumented, which meminfo-adjacent layer would preserve most explanatory power?

### Timeline Template (MemInfo Incident)
- T-10m: baseline snapshot with meminfo headroom annotation
- T-5m: first meminfo headroom or reclaimability anomaly candidate
- T0: user symptom confirmed with meminfo-side context
- T+3m: first hypothesis written with meminfo headroom assumption
- T+6m: cross-source validation including scheduler or pressure
- T+10m: mitigation applied with rollback guard in meminfo path
- T+15m: trend reaction checked for meminfo headroom stabilization
- T+30m: recovery confidence decision with meminfo and pressure confirmation

### Evidence Quality Rubric (MemInfo)
- Strong: ordered, cross-layer, and meminfo headroom-consistent trend evidence
- Medium: correlated movement without clear meminfo headroom/reclaimability boundary
- Weak: isolated value with no meminfo headroom/reclaimability context

### Postmortem Questions (MemInfo)
1. What evidence changed the team decision most?
2. Which metric looked convincing but was later proven secondary?
3. What assumption was left implicit and should be made explicit next time?
4. Which alert would have triggered earlier with lower noise?

### Anti-Drift Checklist (MemInfo)
- Keep one baseline note per environment, workload phase, and meminfo headroom profile.
- Revalidate meminfo-related thresholds after release, kernel, or allocator-behavior changes.
- Avoid cargo-cult tuning; require before and after evidence with meminfo+pressure context.
- Link this article to at least two neighboring meminfo or memory-path articles in your runbook.

## Incident Forensics

### Evidence Capture
- Capture a 30-minute timeline and mark the first headroom or reclaimability inflection before user-impact alerts.
- Pair this field with one scheduler signal and one pressure or vmstat signal to test cross-layer consistency.

### Decision Record
- Primary claim: meminfo.MemTotal indicated a meaningful state transition.
- Disproof attempt: identify one alternate cause and log why it failed.
- Action note: MemTotal was treated as evidence in a chain, not a singleton verdict.

## Man-Page Crosswalk
- Process lens: Process: check runnable vs blocked expansion around meminfo inflection windows.
- Syscall lens: Syscall: identify allocation and memory-touching call bursts aligned with meminfo transitions.
- Scheduler lens: Scheduler: verify whether headroom-side waiting inflated wake latency during meminfo shifts.
- Interrupt or IO lens: Storage or IO: validate writeback or page-in side effects around meminfo shifts.
- Field anchor: MemTotal
- Source anchor: meminfo

## Source Drillbook (MemInfo Family)

### Drill Steps
1. Run a 20-minute replay with three synchronized views: primary field, sibling field, and user symptom.
2. Mark one point where trend direction changed without alert threshold crossing.
3. Explain whether the change suggests reclaim pressure, allocation phase shift, or scheduler-side delay.
4. Write one rollback-safe mitigation and predefine stop conditions.
5. Document what evidence would force you to reject your first hypothesis.

### Debrief Questions
- Which meminfo-side step produced the highest confidence gain?
- Which step failed to reduce uncertainty about headroom versus side effect?
- What meminfo instrumentation change would accelerate this drill next time?

### Anchor (MemInfo)
Field under practice: MemTotal

## Failure Archetype Matrix
- Archetype A: silent meminfo headroom erosion while utilization appears safe.
- Archetype B: meminfo reserve/reclaimability oscillation causing short recoveries and relapses.
- Archetype C: meminfo headroom phase shift with delayed user-impact visibility.
- Field in focus: MemTotal

## Counterfactual Branches
1. If traffic had been flat, would this field still drift in the same direction?
2. If reclaim signals stabilized but latency stayed bad, which non-memory path becomes primary?
3. What memory-side observation would invalidate your current mitigation immediately?

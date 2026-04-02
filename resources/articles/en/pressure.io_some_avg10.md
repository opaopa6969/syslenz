# io_some_avg10

## NAME
`pressure.io_some_avg10` - metric signal from `pressure` (io_some_avg10)

## WHY NOW
Read this when pressure stalled-time behavior around `io_some_avg10` is noisy and you need a fast, defensible decision.
If you cannot tell pressure stall signal around `io_some_avg10` from side effect, use this article to order evidence before tuning.

## EVIDENCE ORDER
1. Fix user-visible symptom and time window first.
2. Verify this article's primary signal trend.
3. Cross-check one sibling signal and one cross-layer signal.
4. Apply reversible mitigation and confirm trend recovery.

## SEE ALSO
Use related links in the article overlay to continue the same evidence chain.

## Metric Snapshot
- ID: `pressure.io_some_avg10`
- Source: `pressure`
- Field: `io_some_avg10`
- Domain: stall accumulation and backpressure
- Signal family: stall-time accumulation

## Operational Meaning (Pressure Lens)
This field is strongest for separating pressure stalled-time signal around `io_some_avg10` from side-effect noise when some/full trends diverge.

## Field Episode (Pressure Lens)
Average pressure looked acceptable while cumulative stall time explained tail-latency degradation.

For `io_some_avg10`, the practical value comes from ordering evidence in time: what moved first, what followed, and what changed after mitigation.

## Reading Protocol (Pressure Lens)
1. Confirm current direction (rising/falling/flat) and short-term slope.
2. Compare against sibling fields in `pressure` to avoid single-metric bias.
3. Cross-check one queue/stall/pressure metric from another source.
4. Map the movement to user impact (latency, error, throughput) before acting.

## Decision Heuristic (Pressure Family)
If this field normalizes quickly after load drop, bias toward burst explanation; if not, investigate persistent contention.

## Failure Patterns To Avoid (Pressure Family)
- Root-cause declaration from one pressure some/full snapshot.
- Ignoring post-mitigation recovery shape in pressure timeline.
- Confusing cross-layer pressure correlation with causation.

## Action Loop
1. State a falsifiable hypothesis for `io_some_avg10`.
2. Apply reversible mitigation.
3. Validate with 2-3 correlated fields over multiple refreshes.
4. Keep concise evidence notes for postmortem reuse.

## Unix Internals Lens

This field is a manifestation of **stall-time accumulation in kernel subsystems (io_some_avg10)**.

- Kernel path: runnable wait, memory reclaim stalls, I/O wait chains.
- Typical trigger (io_some_avg10): resource contention that does not always raise CPU%.
- Cross-check (io_some_avg10): runqueue, reclaim, and queue-depth signals.

## Casebook (Pressure Family)

### Incident Slice 1 (Pressure)
Case A (utilization trap): CPU utilization looked moderate while PSI total climbed steadily. Hidden waiting, not execution, consumed SLO budget.

### Incident Slice 2 (Pressure)
Case B (burst storm): Short periodic stalls were invisible in coarse dashboards but aligned perfectly with user complaints.

### Incident Slice 3 (Pressure)
Case C (cross-layer confusion): Team blamed network; stall evidence showed memory contention as first mover.

## Failure Branches (Pressure Family)
- Branch 1: Symptom improves but pressure trend does not. -> Revisit pressure causal layer assumption.
- Branch 2: pressure trend improves but symptom does not. -> Inspect parallel CPU or IO bottleneck chain.
- Branch 3: Both worsen after mitigation. -> Roll back quickly and preserve pressure evidence snapshot.
- Branch 4: Short recovery then relapse. -> Check pressure stall and retry feedback loops.

## Runbook Drill (Pressure Lens)
1. Pick a 15-minute incident window and annotate T0/T1/T2 events.
2. Build a three-signal chain: primary field, sibling field, cross-layer field.
3. Write one falsifiable hypothesis and one rollback-safe mitigation.
4. Define success as trend recovery + user symptom recovery, not one chart turning green.

## MAN Notes (Pressure Lens)
- This section mirrors man-page flow for pressure analysis: definition -> stalled-time context -> failure branches -> evidence order.
- Prefer explicit timestamps and pressure-phase notes; narratives drift without stall-phase context.
- If uncertain, follow SEE ALSO links before changing production pressure-related knobs.

## Deep Appendix: Counterfactuals and Review Prompts (Pressure Family)

### Counterfactual Questions (Pressure)
- If traffic had stayed constant, would pressure some/full transitions still move this field the same way?
- If this field had remained flat, which non-pressure signal could still explain the symptom?
- If mitigation was delayed by 10 minutes, which pressure-linked user metric would have crossed first?
- If only one layer could be instrumented, which pressure-adjacent layer would preserve most explanatory power?

### Timeline Template (Pressure Incident)
- T-10m: baseline snapshot with pressure some/full annotation
- T-5m: first pressure stalled-time anomaly candidate
- T0: user symptom confirmed with pressure-side context
- T+3m: first hypothesis written with pressure-stall assumption
- T+6m: cross-source validation including scheduler or capacity source
- T+10m: mitigation applied with rollback guard in pressure path
- T+15m: trend reaction checked for pressure stall stabilization
- T+30m: recovery confidence decision with pressure and symptom confirmation

### Evidence Quality Rubric (Pressure)
- Strong: ordered, cross-layer, and pressure stalled-time-consistent trend evidence
- Medium: correlated movement without clear pressure some/full boundary
- Weak: isolated value with no pressure some/full context

### Postmortem Questions (Pressure)
1. What evidence changed the team decision most?
2. Which metric looked convincing but was later proven secondary?
3. What assumption was left implicit and should be made explicit next time?
4. Which alert would have triggered earlier with lower noise?

### Anti-Drift Checklist (Pressure)
- Keep one baseline note per environment, workload phase, and pressure profile.
- Revalidate pressure-related thresholds after release, kernel, or allocator-behavior changes.
- Avoid cargo-cult tuning; require before and after evidence with pressure+symptom context.
- Link this article to at least two neighboring pressure or memory-path articles in your runbook.

## Incident Forensics

### Evidence Capture
- Capture a 30-minute timeline and mark the first stalled-time inflection before user-impact alerts.
- Pair this field with one scheduler signal and one source-specific capacity signal to test cross-layer consistency.

### Decision Record
- Primary claim: pressure.io_some_avg10 indicated a meaningful state transition.
- Disproof attempt: identify one alternate cause and log why it failed.
- Action note: io_some_avg10 was treated as evidence in a chain, not a singleton verdict.

## Man-Page Crosswalk
- Process lens: Process: check runnable vs blocked expansion around pressure inflection windows.
- Syscall lens: Syscall: identify blocking-call bursts aligned with pressure stalled-time transitions.
- Scheduler lens: Scheduler: verify whether wait-to-run inflation amplified pressure stalled-time spikes.
- Interrupt or IO lens: Storage or IO: validate writeback or page-in side effects around pressure shifts.
- Field anchor: io_some_avg10
- Source anchor: pressure

## Source Drillbook (Pressure Family)

### Drill Steps
1. Run a 20-minute replay with three synchronized views: primary field, sibling field, and user symptom.
2. Mark one point where trend direction changed without alert threshold crossing.
3. Explain whether the change suggests reclaim pressure, allocation phase shift, or scheduler-side delay.
4. Write one rollback-safe mitigation and predefine stop conditions.
5. Document what evidence would force you to reject your first hypothesis.

### Debrief Questions
- Which pressure-side step produced the highest confidence gain?
- Which step failed to reduce uncertainty about stalled-time versus side effect?
- What pressure instrumentation change would accelerate this drill next time?

### Anchor (Pressure)
Field under practice: io_some_avg10

## Failure Archetype Matrix
- Archetype A: silent pressure rise where utilization appears safe but stalled time climbs.
- Archetype B: pressure some/full oscillation causing short recoveries and relapses.
- Archetype C: pressure phase shift with delayed user-impact visibility.
- Field in focus: io_some_avg10

## Counterfactual Branches
1. If traffic had been flat, would this field still drift in the same direction?
2. If reclaim signals stabilized but latency stayed bad, which non-memory path becomes primary?
3. What memory-side observation would invalidate your current mitigation immediately?

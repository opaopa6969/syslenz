# Icmp_OutAddrMaskReps

## NAME
`net/snmp.Icmp_OutAddrMaskReps` - metric signal from `net/snmp` (Icmp_OutAddrMaskReps)

## WHY NOW
Read this when IP-layer counters and user symptoms diverge across segments.
If aggregate traffic looks healthy but IP/ICMP anomaly signs rise, use this article to localize path instability.

## EVIDENCE ORDER
1. Fix user-visible symptom and time window first.
2. Verify this article's primary signal trend.
3. Cross-check one sibling signal and one cross-layer signal.
4. Apply reversible mitigation and confirm trend recovery.

## SEE ALSO
Use related links in the article overlay to continue the same evidence chain.

## Metric Snapshot
- ID: `net/snmp.Icmp_OutAddrMaskReps`
- Source: `net/snmp`
- Field: `Icmp_OutAddrMaskReps`
- Domain: protocol-layer network health
- Signal family: baseline behavior and drift detection

## Operational Meaning (ICMP Network Lens)
This field is strongest for localizing transport-path instability in SNMP counter surfaces when user impact worsens first.

## Field Episode (ICMP Network Lens)
The field looked quiet in isolation, but trend context changed the operational decision.

For `Icmp_OutAddrMaskReps`, the practical value comes from ordering evidence in time: what moved first, what followed, and what changed after mitigation.

## Reading Protocol (ICMP Network Lens)
1. Confirm current direction (rising/falling/flat) and short-term slope.
2. Compare against sibling fields in `net/snmp` to avoid single-metric bias.
3. Cross-check one queue/stall/pressure metric from another source.
4. Map the movement to user impact (latency, error, throughput) before acting.

## Decision Heuristic (ICMP Network Family)
If this field and a queue/stall indicator co-move for multiple snapshots, treat it as actionable, not transient noise.

## Failure Patterns To Avoid (ICMP Network Family)
- Root-cause declaration from one snapshot of ICMP control-plane counters.
- Ignoring post-mitigation recovery shape in ICMP control timeline.
- Confusing cross-layer ICMP correlation with causation.

## Action Loop
1. State a falsifiable hypothesis for `Icmp_OutAddrMaskReps`.
2. Apply reversible mitigation.
3. Validate with 2-3 correlated fields over multiple refreshes.
4. Keep concise evidence notes for postmortem reuse.

## Unix Internals Lens

This field is a manifestation of **transport and protocol control-loop signals**.

- Kernel path: TCP/UDP state machine transitions and retry control.
- Typical trigger: loss, reorder, queueing, endpoint backpressure.
- Cross-check: ss states, conntrack occupancy, latency tails.

## Casebook (ICMP Network Family)

### Incident Slice 1 (ICMP Network)
Case B: Cross-source correlation reversed the initial diagnosis.

### Incident Slice 2 (ICMP Network)
Case C: Reversible mitigation provided faster learning than invasive change.

### Incident Slice 3 (ICMP Network)
Case A: First anomaly came from this field trend, not absolute value.

## Failure Branches (ICMP Network Family)
- Branch 1: Symptom improves but ICMP trend does not. -> Revisit the assumed causal layer.
- Branch 2: ICMP trend improves but symptom does not. -> Inspect a parallel CPU or storage bottleneck chain.
- Branch 3: Both worsen after mitigation. -> Roll back quickly and preserve ICMP evidence snapshot.
- Branch 4: Short recovery then relapse. -> Check retry and wake-delay loops in the ICMP path.

## Runbook Drill (ICMP Network Lens)
1. Pick a 15-minute incident window and annotate T0/T1/T2 events.
2. Build a three-signal chain: primary field, sibling field, cross-layer field.
3. Write one falsifiable hypothesis and one rollback-safe mitigation.
4. Define success as trend recovery + user symptom recovery, not one chart turning green.

## MAN Notes (ICMP Network Lens)
- This section mirrors man-page flow for SNMP transport diagnosis: definition -> path context -> failure branches -> evidence order.
- Prefer explicit timestamps and segment notes in SNMP context; narratives drift without path scoping.
- If uncertain, follow SEE ALSO links before changing production ICMP knobs.

## Deep Appendix: Counterfactuals and Review Prompts (ICMP Network Family)

### Counterfactual Questions (ICMP)
- If packet volume had stayed constant, would ICMP control-path transitions still move this field the same way?
- If this field had remained flat, which non-ICMP signal could still explain the symptom?
- If mitigation was delayed by 10 minutes, which timeout or unreachable-rate user metric would have crossed first?
- If only one layer could be instrumented, which control-plane layer would preserve most explanatory power?

### Timeline Template (ICMP Incident)
- T-10m: baseline snapshot with SNMP segment annotation
- T-5m: first SNMP transport anomaly candidate
- T0: user symptom confirmed with SNMP network-path context
- T+3m: first hypothesis written around control-message transition
- T+6m: cross-source validation including route, ACL, and scheduler evidence
- T+10m: mitigation applied with rollback guard in SNMP path
- T+15m: trend reaction checked for control-path stabilization
- T+30m: recovery confidence decision with unreachable/timeout trend confirmation

### Evidence Quality Rubric (ICMP)
- Strong: ordered, cross-layer, and SNMP segment-consistent trend evidence
- Medium: correlated movement without SNMP segment or zone isolation
- Weak: isolated value with no SNMP transport-state context

### Postmortem Questions (ICMP)
1. What evidence changed the team decision most?
2. Which metric looked convincing but was later proven secondary?
3. What assumption was left implicit and should be made explicit next time?
4. Which alert would have triggered earlier with lower noise?

### Anti-Drift Checklist (ICMP)
- Keep one baseline note per zone, policy set, and control-message profile.
- Revalidate SNMP-related thresholds after release, topology, or retry-policy changes.
- Avoid cargo-cult tuning; require before and after evidence with ICMP control-path context.
- Link this article to at least two neighboring ICMP and route-policy articles in your runbook.

## Incident Forensics

### Evidence Capture
- Split evidence by segment and routing domain; global aggregates hide IP-local faults.
- Track anomaly movement and queue states before changing timeout or routing-adjacent policy.

### Decision Record
- Primary claim: net/snmp.Icmp_OutAddrMaskReps indicated a meaningful state transition.
- Disproof attempt: identify one alternate cause and log why it failed.
- Action note: Icmp_OutAddrMaskReps was treated as evidence in a chain, not a singleton verdict.

## Man-Page Crosswalk
- Process lens: Process: isolate worker pools handling packet routing and control-plane interactions.
- Syscall lens: Syscall: inspect recv and send cadence while correlating with routing/lookup behavior.
- Scheduler lens: Scheduler: confirm whether wake delays amplify IP-path anomaly propagation.
- Interrupt or IO lens: Interrupt and softirq: align packet-control handling windows with latency spikes.
- Field anchor: Icmp_OutAddrMaskReps
- Source anchor: net/snmp

## Source Drillbook (ICMP Network Family)

### Drill Steps
1. Split counters by transport phase: setup, steady transfer, retry, and close.
2. Correlate retransmit-like growth with queue occupancy and socket-state transitions.
3. Check whether anomalies are cluster-wide or segment-local before broad policy changes.
4. Define one targeted mitigation for the smallest affected segment.
5. Record the exact evidence that justifies widening scope beyond that segment.

### Debrief Questions
- Which SNMP-path step produced the highest confidence gain?
- Which step failed to localize the SNMP anomaly by segment or zone?
- What SNMP instrumentation change would accelerate this drill next time?

### Anchor (ICMP Network)
Field under practice: Icmp_OutAddrMaskReps

## Failure Archetype Matrix
- Archetype A: localized IP-path instability hidden inside globally healthy aggregates.
- Archetype B (Network `Icmp_OutAddrMaskReps`): anomaly amplification driven by wake-delay and queue coupling.
- Archetype C: control-plane side effects mistaken for network capacity failure.
- Field in focus: Icmp_OutAddrMaskReps

## Counterfactual Branches
1. If segment scoping is applied, does anomaly remain global or collapse locally?
2. If retries are capped, does user error improve or simply shift to latency?
3. What network-path observation would invalidate your current mitigation immediately?

# Tcp_EstabResets

## NAME
`net/snmp.Tcp_EstabResets` - metric signal from `net/snmp` (Tcp_EstabResets)

## WHY NOW
Read this when TCP-path counters and user symptoms diverge across segments.
If global dashboards look healthy but TCP retries rise, use this article to localize path instability.

## EVIDENCE ORDER
1. Fix user-visible symptom and time window first.
2. Verify this article's primary signal trend.
3. Cross-check one sibling signal and one cross-layer signal.
4. Apply reversible mitigation and confirm trend recovery.

## SEE ALSO
Use related links in the article overlay to continue the same evidence chain.

## Metric Snapshot
- ID: `net/snmp.Tcp_EstabResets`
- Source: `net/snmp`
- Field: `Tcp_EstabResets`
- Domain: protocol-layer network health
- Signal family: transport reliability and retry behavior

## Operational Meaning (TCP Network Lens)
This field is strongest for localizing transport-path instability in SNMP counter surfaces when user impact worsens first.

## Field Episode (TCP Network Lens)
Transport counters indicated retry amplification before request-level failures became obvious.

For `Tcp_EstabResets`, the practical value comes from ordering evidence in time: what moved first, what followed, and what changed after mitigation.

## Reading Protocol (TCP Network Lens)
1. Confirm current direction (rising/falling/flat) and short-term slope.
2. Compare against sibling fields in `net/snmp` to avoid single-metric bias.
3. Cross-check one queue/stall/pressure metric from another source.
4. Map the movement to user impact (latency, error, throughput) before acting.

## Decision Heuristic (TCP Network Family)
If this field and a queue/stall indicator co-move for multiple snapshots, treat it as actionable, not transient noise.

## Failure Patterns To Avoid (TCP Network Family)
- Root-cause declaration from one snapshot of TCP state counters.
- Ignoring post-mitigation recovery shape in TCP state timeline.
- Confusing cross-layer TCP correlation with causation.

## Action Loop
1. State a falsifiable hypothesis for `Tcp_EstabResets`.
2. Apply reversible mitigation.
3. Validate with 2-3 correlated fields over multiple refreshes.
4. Keep concise evidence notes for postmortem reuse.

## Unix Internals Lens

This field is a manifestation of **transport and protocol control-loop signals**.

- Kernel path: TCP/UDP state machine transitions and retry control.
- Typical trigger: loss, reorder, queueing, endpoint backpressure.
- Cross-check: ss states, conntrack occupancy, latency tails.

## Casebook (TCP Network Family)

### Incident Slice 1 (TCP Network)
Case C (partial mitigation): Timeout increase reduced errors briefly but worsened queue memory footprint.

### Incident Slice 2 (TCP Network)
Case A (retry shadow): Interface traffic looked normal; transport counters showed retransmission escalation and timer backoff.

### Incident Slice 3 (TCP Network)
Case B (state skew): Connection-state imbalance preceded login failures by minutes.

## Failure Branches (TCP Network Family)
- Branch 1: Symptom improves but TCP trend does not. -> Revisit the assumed causal layer.
- Branch 2: TCP trend improves but symptom does not. -> Inspect a parallel CPU or storage bottleneck chain.
- Branch 3: Both worsen after mitigation. -> Roll back quickly and preserve TCP evidence snapshot.
- Branch 4: Short recovery then relapse. -> Check retry and wake-delay loops in the TCP path.

## Runbook Drill (TCP Network Lens)
1. Pick a 15-minute incident window and annotate T0/T1/T2 events.
2. Build a three-signal chain: primary field, sibling field, cross-layer field.
3. Write one falsifiable hypothesis and one rollback-safe mitigation.
4. Define success as trend recovery + user symptom recovery, not one chart turning green.

## MAN Notes (TCP Network Lens)
- This section mirrors man-page flow for SNMP transport diagnosis: definition -> path context -> failure branches -> evidence order.
- Prefer explicit timestamps and segment notes in SNMP context; narratives drift without path scoping.
- If uncertain, follow SEE ALSO links before changing production TCP knobs.

## Deep Appendix: Counterfactuals and Review Prompts (TCP Network Family)

### Counterfactual Questions (TCP)
- If packet volume had stayed constant, would TCP state transitions still move this field the same way?
- If this field had remained flat, which non-TCP signal could still explain the symptom?
- If mitigation was delayed by 10 minutes, which RTT or retransmission user metric would have crossed first?
- If only one layer could be instrumented, which socket-state layer would preserve most explanatory power?

### Timeline Template (TCP Incident)
- T-10m: baseline snapshot with SNMP segment annotation
- T-5m: first SNMP transport anomaly candidate
- T0: user symptom confirmed with SNMP network-path context
- T+3m: first hypothesis written around connection-state transition
- T+6m: cross-source validation including socket states and scheduler run queue
- T+10m: mitigation applied with rollback guard in SNMP path
- T+15m: trend reaction checked for retransmission and cwnd stabilization
- T+30m: recovery confidence decision with handshake/reset trend confirmation

### Evidence Quality Rubric (TCP)
- Strong: ordered, cross-layer, and SNMP segment-consistent trend evidence
- Medium: correlated movement without SNMP segment or zone isolation
- Weak: isolated value with no SNMP transport-state context

### Postmortem Questions (TCP)
1. What evidence changed the team decision most?
2. Which metric looked convincing but was later proven secondary?
3. What assumption was left implicit and should be made explicit next time?
4. Which alert would have triggered earlier with lower noise?

### Anti-Drift Checklist (TCP)
- Keep one baseline note per service, workload phase, and connection class.
- Revalidate SNMP-related thresholds after release, topology, or retry-policy changes.
- Avoid cargo-cult tuning; require before and after evidence with TCP state context.
- Link this article to at least two neighboring TCP lifecycle articles in your runbook.

## Incident Forensics

### Evidence Capture
- Split evidence by segment and connection phase; global aggregates hide TCP-local faults.
- Track retransmit movement and queue states before changing retry or timeout policy.

### Decision Record
- Primary claim: net/snmp.Tcp_EstabResets indicated a meaningful state transition.
- Disproof attempt: identify one alternate cause and log why it failed.
- Action note: Tcp_EstabResets was treated as evidence in a chain, not a singleton verdict.

## Man-Page Crosswalk
- Process lens: Process: isolate worker pools handling connection establishment and stream progression.
- Syscall lens: Syscall: inspect accept, recv, and send cadence against queue occupancy.
- Scheduler lens: Scheduler: confirm whether wake delays amplify TCP retransmit patterns.
- Interrupt or IO lens: Interrupt and softirq: align packet-completion windows with tail spikes.
- Field anchor: Tcp_EstabResets
- Source anchor: net/snmp

## Source Drillbook (TCP Network Family)

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

### Anchor (TCP Network)
Field under practice: Tcp_EstabResets

## Failure Archetype Matrix
- Archetype A: localized TCP instability hidden inside globally healthy aggregates.
- Archetype B: retransmit amplification driven by wake-delay and queue coupling.
- Archetype C: timeout-policy side effects mistaken for transport capacity failure.
- Field in focus: Tcp_EstabResets

## Counterfactual Branches
1. If segment scoping is applied, does anomaly remain global or collapse locally?
2. If retries are capped, does user error improve or simply shift to latency?
3. What network-path observation would invalidate your current mitigation immediately?

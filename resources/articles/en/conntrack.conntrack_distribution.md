# Connection tracking distribution

## NAME
`conntrack.conntrack_distribution` - group article covering conntrack table counts and headroom limits

## WHY NOW
Use this group when retries, drops, or timeout storms do not match any single counter. A dissonant signal suggests the table is simultaneously near capacity (`conntrack_max`) and trending toward exhaustion (`conntrack_count`). This article turns that dissonance into a structured proof.

## EVIDENCE ORDER
1. Capture the symptom window and verify downstream client impact (timeouts, rate limit triggers, SYN drops).
2. Confirm `conntrack_count` trend and how close the table is to `conntrack_max`.
3. Measure whether short-lived state churn or long-lived binds dominate the delta.
4. Apply reversible headroom relief (adjust timeout, cgroup `netfilter` limits, or NIC offload) and check for cross-layer recovery.

## SEE ALSO
- `sourceguide.conntrack`
- `concept.cross-metric-reading`
- `net/snmp.Tcp_Rto_distribution`

## Distribution components
- **conntrack_count**: current number of tracked connections; trending linearly when new flows outpace cleanup.
- **conntrack_max**: kernel tunable limit; elevated value indicates intentional headroom, while an unchanged value means static table depth.
- **conntrack.usage_pct**: derived ratio; removing noise from raw count by scaling against the long-term max/carved limit.

## Operational narrative
Conntrack tables are shared resources between kernel, NAT helpers, and connection-oriented workloads. Incidents often follow this script: workloads with many short-lived flows fill the table quickly (`conntrack_count` spikes) while the configured `conntrack_max` stays constant, and `usage_pct` crosses yellow thresholds. Teams either fail open (timeouts) or fail close (drops) depending on whether kernel cleanup keeps pace. The winning playbook treats this as a distribution problem, not a single gauge drama.

## Reading protocol
1. Visualize `conntrack_count` and `conntrack.usage_pct` on a trend chart to reveal the slope and jitter.
2. Overlay the `conntrack_max` line to see how far headroom remains; a flat max with a rising count signals inevitable throttling.
3. Correlate with SYN drops in `net/snmp.Tcp_SynDropped` or `net/stat` transitions for explanatory power.
4. Use `conntrack.usage_pct` to filter out ephemeral bursts, letting you detect persistent drift.

## Decision signals
- **Low risk**: count growth is proportional to observed client load and resets quickly after the burst.
- **Warning**: usage_pct records multiple passes above 80% without symptom improvement.
- **Critical**: actual quota breaches (`conntrack_count == conntrack_max`) coincide with `TCPSynRetrans` or `conntrack.usage_pct` saturating.

## Misread patterns
- Tuning `conntrack_max` without confirming why the count increased (capacity vs. leak).
- Treating spikes as anomalies when they represent legitimate batch jobs (validate with request timeline).
- Disabling checksum offload or NAT helpers without referencing the usage distribution.

## Action loop
1. Hypothesize whether the table is throttling due to short-lived churn or sticky sessions.
2. Release unused states (`conntrack -F` in a controlled window) or extend timeouts temporarily.
3. Re-measure `conntrack_count` slope and usage_pct after each change.
4. Document the evidence chain for the next incident.

## Unix internals lens
- **Process layer**: each socket pair increments conntrack state; if helpers hold onto entries, cleanup lags.
- **Syscall layer**: connection setup/teardown (accept, close) influences both count delta and the aging queue.
- **Scheduler lens**: backlog and softirq delay can postpone `nf_conntrack` cleanup queues, amplifying headroom loss.
- **I/O and interrupt lens**: bursts from hardware offloads (VXLAN, GRO) stress the table via high packet rates.

## Incident lab
### Drill A: Capacity story
1. Pick a burst window from load tests.
2. Annotate conntrack count, max, and usage_pct along the timeline.
3. Explain whether the drift is due to demand growth or an aging backlog of states.

### Drill B: Reversible relief
1. Choose one knob (timeout, max, blocklist) to relax gracefully.
2. Apply the change with a rollback plan (# of seconds or failed packets) and observe the distribution.
3. If symptoms do not ease, restore the original knob and collect supporting metrics.

### Drill C: Evidence compression
1. Summarize the incident in 6 lines: symptom, primary metric, secondary metric, action, reaction, conclusion.
2. Share with the team to validate whether the story matches what they observed.

## Quick checklist
- Timestamp of the first conntrack warning entry.
- Distance between `conntrack_count` and `conntrack_max` during incident.
- Evidence that cleanup or timeout values were appropriate for the workload.
- Action taken (e.g., `conntrack_max` bump + cleanup script) and rollback guard.
- Confirmation that the slope or jitter normalized afterward.

## Incident forensics
### Evidence capture
- Archive the `conntrack_count` vs `conntrack_max` graph with the event window annotated.
- Include supporting `net/snmp` retransmission or drop counters.

### Decision record
- Claim: the table distribution, not a single counter, limited throughput.
- Disproof attempt: what non-distribution explanation (e.g., NIC limit) could also fit?
- Action note: actions were reversible (timeout tweak) before committing to kernel recompile.

## Failure archetype matrix
- **Archetype A**: capacity is misread as leakage because the count keeps climbing after cleanup.
- **Archetype B**: `conntrack_max` is increased blindly, leaving no headroom for helper backlogs.
- **Archetype C**: `usage_pct` saturates at 100% but teams chase unrelated firewall rules.

## Counterfactual branches
1. If the table had been larger, would client timeouts still have spiked? (Use usage_pct to reason.)
2. If cleanup was delayed by 500 ms, which metric would have risen first? (Hint: `conntrack_count`.)
3. If we could instrument state lifetime, which bucket would we throttle next?

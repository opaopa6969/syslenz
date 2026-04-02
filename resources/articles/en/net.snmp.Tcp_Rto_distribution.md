# TCP RTO distribution

## NAME
`net/snmp.Tcp_Rto_distribution` - multi-metric overview pairing minimum and maximum retransmission timeout settings with retransmission signals

## WHY NOW
Open this article when TCP retransmissions, retransmission timeouts, or connection stalls do not line up with a single RTO stat. Traffic often looks healthy even though the min/max pair are chasing each other; this group exposes their interplay and the socket pressure they create.

## EVIDENCE ORDER
1. Pin the symptom window and measure the experience impact (retransmission counts, tail latency, application retries).
2. Track `Tcp_RtoMin` and `Tcp_RtoMax` to see whether RTO is collapsing (responsive) or expanding (backoff).
3. Confirm whether `Tcp_RetransSegs` or `Tcp_RtoAlgorithm` hints at adaptive behavior.
4. Adjust timeouts, pacing, or congestion controls cautiously and validate cross-layer recovery.

## SEE ALSO
- `net/snmp.Tcp_RetransSegs`
- `concept.latency-analysis`
- `concept.bottleneck-triage`

## Distribution components
- **Tcp_RtoMin**: negotiated minimum retransmission timeout; moving baseline indicates handshake or load changes.
- **Tcp_RtoMax**: upper cap; rising max shows widening jitter, often due to queueing or hardware stalls.
- **Tcp_RetransSegs**: tells you how many segments needed retransmission; use to validate whether RTO values were the right alarms.
- **Tcp_RtoAlgorithm**: reveals whether the kernel is in `RTO_RFC6298` or `RTO_LP` style mode (Algorithm 0 vs 1) and what behavior to expect.

## Operational narrative
RTO ranges are not static—they are adaptive knobs that shift based on latency, congestion, and historical retransmission history. When `Tcp_RtoMin` shrinks but retransmissions stay high, the kernel is trying to probe aggressively but starving the retransmission queue. When `Tcp_RtoMax` balloons while `Tcp_RetransSegs` hikes, underlying latency or NIC stalls have moved the target. This group forces you to articulate the narrative: is the RTO spectrum chasing a noisy path, or is it protecting clients from excessive retries?

## Reading protocol
1. Overlay the min and max lines on the same chart to see relative alignment.
2. Observe `Tcp_RetransSegs` spikes; if they line up with min/max drift, the retransmission logic is reactive.
3. Check `Tcp_RtoAlgorithm` to know whether the kernel is using exponential backoff or pacing-based heuristics.
4. Combine with queue and pressure metrics (`net/stat`, `queues/TCP`) before tuning.

## Decision signals
- **Keep calm**: min/max move in step, retransmissions stay within expected rate, and RTT variance is stable.
- **Investigate**: max jumps while min stays flat, or min shrinks yet retrans metrics climb.
- **Act**: both min and max diverge aggressively, retransmissions stay high, and latency spikes persist despite pacing.

## Misread patterns
- Fixing retransmissions by lowering `Tcp_RtoMin` alone, ignoring the capacity of downstream queues.
- Raising `Tcp_RtoMax` because max drifted, without verifying whether the stack already retried enough.
- Blaming the network when `Tcp_RtoAlgorithm` shows the kernel is in aggressive `RTO_ADAPTIVE` mode due to past packet loss.

## Action loop
1. Hypothesize whether the signal is congestion-shed (max drift) or noise chasing (min oscillates).
2. Adjust pacing (e.g., `net.ipv4.tcp_low_latency`) or congestion control quickly.
3. Revalidate with `Tcp_RetransSegs` and client-reported tail latency.
4. Keep the entire run documented (timing, knobs, rollback conditions).

## Unix internals lens
- **Process layer**: retransmission counts expose how userspace sockets see burstiness, but the kernel controls RTO boundaries.
- **Syscall layer**: slow handshake completion or blocked `connect()` when `Tcp_RtoMax` is large.
- **Scheduler lens**: delayed softirq handling can artificially extend RTT, pushing RTO max upward.
- **Interrupt or I/O lens**: hardware queue saturation (NIC, disk) feeds into the RTO decision tree.

## Incident lab
### Drill A: Distribution map
1. Annotate a 10-minute window showing RTT, `Tcp_RtoMin`, `Tcp_RtoMax`, and `Tcp_RetransSegs`.
2. Identify whether min or max moved first.
3. Decide if adaptive behavior (algorithm change) or network jitter explains the gap.

### Drill B: Guarded tuning
1. Apply a low-impact change (e.g., pacing toggles or small `tcp_moderate_rcvbuf`) with rollback time.
2. Monitor distribution for 2-3 windows.
3. If retransmissions remain, restore the previous knob and record why.

### Drill C: Evidence compression
1. Write one short narrative: symptom, min-max trend, action, recovery.
2. Share with the on-call to ensure the story matches the observed impact.

## Quick checklist
- Symptom timestamp and client-facing metric.
- Relative gap between `Tcp_RtoMin` and `Tcp_RtoMax` at T0.
- Spike count of `Tcp_RetransSegs` around the gap.
- Kernel algorithm state (0 or 1) and whether it shifted during the incident.
- Action taken and rollback guard.

## Incident forensics
### Evidence capture
- Save min/max trend with retransmission spikes annotated.
- Attach `tcp_rto_algorithm` and queue/pressure notes.

### Decision record
- Claim: the RTO distribution, not a single metric, was the explanatory anchor.
- Disproof: what hardware or scheduling delay could mimic the same gap?
- Action: e.g., temporary pacing toggle with firm rollback instructions.

## Failure archetype matrix
- **Archetype A**: The min line oscillates but max stays constant; teams chase alarms instead of backlog.
- **Archetype B**: The max line accelerates, but min is static; the kernel is handing off to bigger timeouts.
- **Archetype C**: Retransmissions climb yet RTO lines remain calm; the real issue is NIC latency, not TCP.

## Counterfactual branches
1. If min/max had stayed aligned, would retransmissions still explain the tail latency?
2. If we delayed action by 30 seconds, which line would have broken first—the min or the retrans count?
3. If we could instrument the algorithm switch, what would we change next?

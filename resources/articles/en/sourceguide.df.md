# Source Guide: df

## NAME
`sourceguide.df` - source-level operational reading guide

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
`df` is a protocol-oriented telemetry surface. It helps connect socket behavior, packet lifecycle, and user-visible latency.

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

## Systems Narrative (Storage)

This signal (sourceguide.df) is not only a number; it is an exposed edge of kernel state transitions.
This source becomes far more reliable when read as part of a cross-layer evidence chain.

### Episode: Dashboard Confidence vs User Pain (Storage)
- The dashboard looked green because storage averages stayed normal.
- User-facing latency regressed only in storage burst windows.
- This storage field moved first, and neighboring fields confirmed direction.
- The winning move was not a large storage tuning change, but narrowing uncertainty quickly.

### Cross-Layer Translation (Storage)
1. Translate field movement into a probable kernel path.
2. Verify whether scheduler delay or I/O wait explains wall-clock loss.
3. Separate demand growth from service-time growth.
4. Confirm post-change recovery in both symptom and mechanism.

### What Senior Reviewers Usually Ask (Storage)
- Which storage counter moved first in time order?
- Which storage counter looked persuasive but was later demoted to a side effect?
- Which storage execution path likely carried the user-visible penalty?
- Which storage mitigation was reversible and what rollback trigger was defined?

### Combining With Unix Internals (Storage)
- Process model (Storage lens): did runnable tasks increase, or did blocked tasks accumulate?
- Syscall lifecycle (Storage lens): where did request time shift (entry, sleep, wakeup, return)?
- Interrupt path (Storage lens): did wakeup delivery or softirq backlog alter tail behavior?
- Scheduler (Storage lens): did fairness protect throughput while harming tail latency?

### Practical Mentor Notes (Storage)
Treat Filesystem                                 1K-blocks        Used   Available Use% Mounted on
/dev/sdd                                  1055762868    69485972   932573424   7% /
rootfs                                      32679036        2720    32676316   1% /init
tmpfs                                       32684076           0    32684076   0% /dev
none                                        32684076        2464    32681612   1% /run
none                                        32684076           0    32684076   0% /run/lock
none                                        32684076           0    32684076   0% /run/shm
none                                        32684076         112    32683964   1% /run/user
tmpfs                                        6536812           8     6536804   1% /run/user/0
tmpfs                                        6536812           8     6536804   1% /run/user/1000
drivers                                   3999836156  1648965312  2350870844  42% /usr/lib/wsl/drivers
none                                        32684076           0    32684076   0% /usr/lib/wsl/lib
none                                        32684076           4    32684072   1% /mnt/wsl
none                                        32684076         612    32683464   1% /mnt/wsl/docker-desktop/shared-sockets/host-services
/dev/sde                                      117339       59741       48378  56% /mnt/wsl/docker-desktop/docker-desktop-user-distro
/dev/loop0                                    671964      671964           0 100% /mnt/wsl/docker-desktop/cli-tools
none                                        32684076         692    32683384   1% /mnt/wslg/versions.txt
none                                        32684076         692    32683384   1% /mnt/wslg/doc
none                                        32684076           0    32684076   0% /usr/lib/modules/6.6.87.2-microsoft-standard-WSL2
C:\                                       3999836156  1648965312  2350870844  42% /mnt/c
D:\                                      19531806712  4508932392 15022874320  24% /mnt/d
E:\                                      19531806712 19418647624   113159088 100% /mnt/e
F:\                                       1982823420   207212128  1775611292  11% /mnt/f
G:\                                        975628284    84715792   890912492   9% /mnt/g
snapfuse                                         128         128           0 100% /snap/bare/5
snapfuse                                       75776       75776           0 100% /snap/core22/2339
snapfuse                                       75776       75776           0 100% /snap/core22/2411
snapfuse                                       93952       93952           0 100% /snap/gtk-common-themes/1535
snapfuse                                       49280       49280           0 100% /snap/snapd/25935
snapfuse                                       49536       49536           0 100% /snap/snapd/26382
snapfuse                                      134272      134272           0 100% /snap/ubuntu-desktop-installer/1276
snapfuse                                      134912      134912           0 100% /snap/ubuntu-desktop-installer/1286
C:\Program Files\Docker\Docker\resources  3999836156  1648965312  2350870844  42% /Docker/host as one scene in a longer diagnostic narrative.
The storage narrative quality matters more than single-point precision: strong incidents are solved by ordered evidence, explicit assumptions, and controlled experiments.

## Incident Lab (Storage)

### Drill A: First-Mover Detection (Storage)
1. Pick one incident window and annotate first movement among three related signals.
2. Record one wrong hypothesis that looked plausible at first.
3. Explain why time order invalidated that hypothesis.

### Drill B: Reversible Mitigation Design (Storage)
1. Define one mitigation that can be rolled back in less than five minutes.
2. Define one explicit rollback condition before applying it.
3. Track symptom and mechanism separately after the change.

### Drill C: Evidence Compression (Storage)
1. Write a six-line narrative: symptom, first signal, second signal, action, reaction, conclusion.
2. Remove adjectives and keep only testable statements.
3. Hand the narrative to another engineer and check if they can reproduce your reasoning.

### Review Outcome (Storage)
If your team can replay this storage article as a short diagnostic script, the article is operationally useful.

## Incident Forensics

### Evidence Capture
- Use this source as an index into neighboring sources rather than a standalone authority.
- The reading quality improves when you can explain what this source cannot prove by itself.

### Decision Record
- Primary claim: sourceguide.df indicated a meaningful state transition.
- Disproof attempt: identify one alternate cause and log why it failed.
- Action note: df was treated as evidence in a chain, not a singleton verdict.

## Man-Page Crosswalk
- Process lens: Process: decide whether this is demand growth or service degradation.
- Syscall lens: Syscall: mark one candidate path for time attribution.
- Scheduler lens: Scheduler: validate runqueue and wake behavior before tuning.
- Interrupt or IO lens: Interrupt or IO: cross-check one hardware-adjacent signal.
- Field anchor: df
- Source anchor: sourceguide

## Failure Archetype Matrix
- Archetype A: source treated as verdict instead of index into neighboring evidence.
- Archetype B: threshold crossing fixation while trend-shape evidence is ignored.
- Archetype C: mitigation attempted before confirming cross-layer sequence.
- Field in focus: df

## Counterfactual Branches
1. If this source is removed, which two sources can reconstruct the same conclusion?
2. If values stay normal but user pain grows, what trend clue was likely missed?
3. What neighboring-source observation would invalidate your current mitigation immediately?

# Hardware Components

What is this?
A map of the physical pieces that bound system performance: CPU, memory, storage, NICs, power, and thermals.

Why it matters
If you do not know the component, you cannot tell whether the limit is silicon, firmware, or software.

How to use
- CPU: cores, clocks, cache, SMT, thermal headroom
- Memory: RAM size, bandwidth, fragmentation, swap exposure
- Storage: device latency, queue depth, media type, endurance
- Network: NIC speed, drops, offload behavior, packet path

Common mistakes
- Treating all hardware as interchangeable
- Assuming faster storage fixes every IO problem
- Ignoring thermals and power limits on busy systems

Diagnostic flow
- CPU symptoms -> check frequency, throttling, and run queue
- Memory symptoms -> check headroom, reclaim, and fragmentation
- IO symptoms -> check device queue and tail latency
- Network symptoms -> check drops, retransmits, and link speed

# Memory Pressure

What is this?
The point where memory demand starts causing reclaim, stalls, or swap activity.

Why it matters
Memory pressure is often visible before an OOM event. PSI and swap activity show impact earlier than a crash.

How to use
- Check MemAvailable for headroom
- Check Cached and Slab for reclaimable memory
- Check swap usage and PSI for user-visible stalls

Common mistakes
- Assuming low MemFree means pressure
- Ignoring cache reclaimability
- Watching only total used memory

Diagnostic flow
1. Confirm whether MemAvailable is falling.
2. Check whether swap is growing.
3. Check whether memory PSI is non-zero.
4. Decide whether to add RAM, reduce demand, or tune reclaim behavior.

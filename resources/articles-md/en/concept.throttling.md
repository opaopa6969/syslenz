# Throttling

What is this?
An intentional limit that slows execution to stay within thermal, power, cgroup, or policy boundaries.

Why it matters
Throttling can look like random slowness unless you check the limiter itself.

How to use
- Check whether CPU frequency falls under load
- Check whether thermal or power limits are engaged
- Check whether cgroup or quota limits are active

Common mistakes
- Treating throttling as normal latency
- Ignoring power and thermal envelopes on laptops and dense servers
- Looking only at average CPU usage when frequency is capped

Diagnostic flow
1. See whether performance drops with rising heat or power draw.
2. Check whether the cap is imposed by hardware, firmware, or policy.
3. Check whether latency follows the cap exactly.
4. Decide whether to cool, reconfigure, or move the workload.

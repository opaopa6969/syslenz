# Capacity vs Utilization

What is this?
The difference between how busy a resource looks and how much safe room it really has.

Why it matters
High utilization does not always mean trouble, and low utilization does not always mean safety.

How to use
- Capacity asks: how much headroom remains?
- Utilization asks: how much is currently in use?
- Pressure asks: is anyone waiting because of contention?

Common mistakes
- Using utilization alone to predict failure
- Ignoring burstiness and queue growth
- Assuming a flat average means safe headroom

Diagnostic flow
1. Measure headroom.
2. Measure contention.
3. Compare recent trend with peak behavior.
4. Decide whether to add capacity, rebalance, or reduce demand.

# TCP Retransmissions Distribution

What is this?
A grouped article for retransmission counters that should be read as a distribution: min, max, count, and related buckets.

Why it matters
A single spike can be harmless, but repeated retransmits point to loss, congestion, or path instability.

How to read
- count: how often retransmits happen
- max: the worst burst or peak window
- min: the quiet baseline

Common mistake
Checking only the peak and missing whether the pattern is persistent.

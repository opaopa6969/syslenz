# pswpout

What is this?
Pages evicted from RAM to swap.

Why it matters
Growing pswpout is a direct sign that RAM pressure has crossed from warning into action.

How to read
- Some historical swap-out can be fine.
- New growth while the workload is active means reclaim is failing to keep up.
- If pressure.memory_full_avg10 is non-zero, the impact is visible.

Next check
Look at MemAvailable and pressure.memory_full_avg10.
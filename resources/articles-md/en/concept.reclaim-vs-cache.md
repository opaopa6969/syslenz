# Reclaim vs Cache

What is this?
The difference between memory that is immediately free, memory that can be reclaimed, and memory that is genuinely committed.

Why it matters
Linux uses RAM aggressively for cache. A large page cache is usually good, not a leak.

How to use
- Treat Cached and Buffers as potential headroom
- Treat Slab as partly reclaimable, partly sticky
- Treat MemAvailable as the best quick estimate

Common mistakes
- Calling every rise in used memory a leak
- Restarting services because MemFree looks low
- Ignoring whether reclaim is cheap or expensive

Diagnostic flow
1. Check whether the memory is free, cached, or committed.
2. Check whether reclaim cost is growing.
3. Check whether swap is being used.
4. Decide whether the issue is cache growth, true leak, or workload increase.

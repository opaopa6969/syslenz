# CPU Run Queue

What is this?
The set of runnable tasks waiting for CPU time.

Why it matters
A growing run queue means work is ready but cannot execute immediately. That is a direct signal of contention.

How to use
- Compare runnable tasks with CPU count
- Look for context-switch churn and load average growth
- Check whether the load is compute-bound or waiting on something else

Common mistakes
- Treating load average as the same thing as CPU usage
- Ignoring runnable tasks on machines with many cores
- Missing steal time or throttling on virtualized systems

Diagnostic flow
1. Check load and runnable count.
2. Check CPU pressure and utilization.
3. Check whether a single process or all tasks are waiting.
4. Check whether the real issue is CPU, IO, or lock contention.

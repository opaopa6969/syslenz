# SLO Thinking for Infrastructure

[日本語版](../ja/concept.slo-thinking.md)

---

**Service Level Objectives (SLOs)** turn vague "it should be fast" into measurable targets.

**Common infrastructure SLOs:**
- P99 request latency < 100ms
- Availability > 99.9% (8.7 hours downtime/year)
- Error rate < 0.1%

**Translating to Linux metrics:**
Instead of alerting on CPU > 80%, ask: what CPU level causes SLO violation?

```
SLO: p99 latency < 200ms
  ↓ test: at what CPU% does p99 breach 200ms?
  ↓ answer: consistently above 70% over 5min
  → Alert threshold: cpu_some_avg300 > 50%
     (leave headroom before breach)
```

**Error budget thinking:**
- 99.9% availability = 43.8 minutes downtime/month error budget
- High `vmstat.oom_kill` events consume error budget
- When to page vs. when to ticket: does this consume error budget?

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

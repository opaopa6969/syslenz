# Incident Response Workflow

[日本語版](../ja/concept.incident-workflow.md)

---

A structured approach to system incidents reduces time-to-resolution.

**1. Establish symptoms** (what is the user impact?)
- Which service/endpoint is affected?
- When did it start?
- Is it getting better or worse?

**2. Check the most important indicators first**
```sh
# 30-second overview
uptime                    # load
free -h                   # memory
df -h                     # disk
cat /proc/pressure/cpu    # CPU stall
cat /proc/pressure/memory # memory stall
cat /proc/pressure/io     # I/O stall
```

**3. Hypothesize and test**
Form a hypothesis about the cause, find the metric that would confirm or deny it.

**4. Mitigate, then investigate root cause**
Don't let perfect be the enemy of good. Restore service first. Then find the root cause with more time.

**5. Document the timeline**
What changed, when, what fixed it. For post-mortems.

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

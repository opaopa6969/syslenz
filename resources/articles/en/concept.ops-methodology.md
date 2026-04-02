# Operations Methodology

[日本語版](../ja/concept.ops-methodology.md)

---

Two complementary methodologies for system analysis:

**USE Method** (Brendan Gregg) — for resources:
- **U**tilization: how busy is the resource? (%)
- **S**aturation: is there a queue? (run queue, disk queue)
- **E**rrors: are there errors?

Apply to each resource: CPU, memory, disk, network.

**RED Method** — for services:
- **R**ate: requests per second
- **E**rrors: error rate
- **D**uration: response time/latency

**Golden Signals** (Google SRE):
1. Latency — time to serve request
2. Traffic — demand on the system
3. Errors — rate of failing requests
4. Saturation — how close to capacity

For infrastructure (Linux system metrics), USE method is most applicable. For service monitoring, RED or Golden Signals.

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

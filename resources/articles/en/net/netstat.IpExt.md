# IpExt — Extended IP Statistics

[日本語版](../../ja/net/netstat.IpExt.md)

---

## What is it?

`IpExt` contains extended IP-layer counters from `/proc/net/netstat` that supplement the basic RFC MIB stats in `/proc/net/snmp`.

| Metric | What it counts |
|--------|---------------|
| `InOctets` / `OutOctets` | Total bytes in/out (including headers) |
| `InMcastPkts` / `OutMcastPkts` | Multicast packets |
| `InBcastPkts` / `OutBcastPkts` | Broadcast packets |
| `InCsumErrors` | Inbound IP checksum errors |
| `InNoRoutes` | Packets dropped — no route to destination |
| `InCEPkts` | ECN Congestion Experienced packets |
| `InECT0Pkts` / `InECT1Pkts` | ECN-capable transport packets |
| `ReasmOverlaps` | Overlapping IP fragments (possible attack) |

---

## Key signals

**`InNoRoutes` rising**: packets arriving with no route — routing table misconfiguration or someone is sending to the wrong destination.

**`InCsumErrors`**: corrupted IP packets arriving — usually hardware issue or buggy NIC driver.

**`ReasmOverlaps`**: overlapping IP reassembly fragments. Old technique used in evasion attacks. Should be near zero.

**ECN metrics** (`InCEPkts` etc.): if your network supports ECN, these track congestion signaling. High `InCEPkts` means routers are seeing congestion and signaling back.

---

## See also

- `net/snmp.Ip` — basic IP statistics
- `sourceguide.net/netstat` — full source overview

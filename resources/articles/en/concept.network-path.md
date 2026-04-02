# Network Path

[日本語版](../ja/concept.network-path.md)

---

Understanding the network path helps isolate network problems.

```
  Application: send(fd, "GET /", 5)
       ↓
  Socket buffer (send queue)
       ↓
  TCP layer (segmentation, sequencing)
       ↓
  IP layer (routing, fragmentation)
       ↓
  NIC driver (DMA to NIC ring buffer)
       ↓
  NIC hardware (frame creation, PHY)
       ↓
  Physical link (cable/fiber/wireless)
       ... 
  Remote NIC → remote IP → remote TCP → remote socket
```

**Where to look for problems:**
- Socket buffer full → `net/netstat.TcpExt_TCPBacklogDrop`
- TCP congestion → `net/netstat.TcpExt_TCPTimeouts`
- IP routing → `net/snmp.Ip_OutNoRoutes`
- NIC errors → `net/dev.interfaces` (errors column)

---

## See also

- `sourceguide.vmstat` — vmstat overview
- `sourceguide.meminfo` — memory info overview
- `sourceguide.pressure` — PSI pressure stall information

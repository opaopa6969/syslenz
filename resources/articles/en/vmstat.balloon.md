# Memory Balloon (balloon) — vmstat

[日本語版](../ja/vmstat.balloon.md)

---

## What is it?

**Memory ballooning** is a virtualization technique that lets the hypervisor reclaim memory from a guest VM without shutting it down. The guest's balloon driver "inflates" (grabs pages from the guest OS and returns them to the host) or "deflates" (releases pages back to the guest).

```
  Host hypervisor says: "I need 1 GB from guest VM"
  
  Guest balloon driver inflates:
  [Guest app memory] [balloon: 1 GB] ← balloon holds pages "for host"
  
  Host can now use those physical pages for other VMs
```

| Metric | What it counts |
|--------|---------------|
| `balloon_inflate` | Pages added to balloon (memory taken from guest) |
| `balloon_deflate` | Pages removed from balloon (memory returned to guest) |
| `balloon_migrate` | Balloon pages migrated (live migration) |

---

## Why does it matter?

**`balloon_inflate` rising** means the hypervisor is actively taking memory from your VM. From the guest's perspective, RAM is shrinking. This can trigger swap and memory pressure.

**`balloon_deflate`** after an inflate means the host is giving memory back — a good sign that the pressure was temporary.

If you see `balloon_inflate` climbing and `meminfo.MemAvailable` dropping simultaneously, the hypervisor is the cause of your memory pressure.

```sh
# Check if balloon driver is loaded
lsmod | grep balloon

# Watch balloon activity
watch -n 5 'grep balloon /proc/vmstat'
```

This metric is zero on bare-metal hosts — only relevant in VMs with virtio-balloon or VMware balloon drivers.

---

## See also

- `meminfo.MemAvailable` — available memory (affected by ballooning)
- `vmstat.allocstall` — allocation stalls that follow memory removal
- `sourceguide.vmstat` — full vmstat source overview

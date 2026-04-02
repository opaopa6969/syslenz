# TcpExt — Extended TCP Statistics

[日本語版](../../ja/net/netstat.TcpExt.md)

---

## What is it?

`TcpExt` is a set of ~130 extended TCP counters that Linux tracks beyond the basic RFC MIB statistics. They're in `/proc/net/netstat` and expose the internals of the TCP stack: SYN cookies, congestion control, retransmit details, memory pressure, Fast Open, and more.

Think of `/proc/net/snmp` as the TCP summary, and `TcpExt` as the TCP debug log.

---

## The most important counters

**Connection attacks / SYN flood:**
| Metric | Signal |
|--------|--------|
| `SyncookiesSent` | SYN flood protection active |
| `SyncookiesFailed` | Invalid cookies (spoofed SYN flood) |
| `ListenOverflows` | Accept queue full — server can't keep up |
| `ListenDrops` | SYNs dropped — critical if rising |

**Retransmits and losses:**
| Metric | Signal |
|--------|--------|
| `TCPFastRetrans` | Fast retransmits (SACK-based, good response to loss) |
| `TCPTimeouts` | RTO timeouts (slow, costly — means real packet loss) |
| `TCPLostRetransmit` | Lost retransmits (very bad — retransmit was itself lost) |
| `TCPRetransFail` | Failed retransmit attempts |

**Memory pressure:**
| Metric | Signal |
|--------|--------|
| `TCPMemoryPressures` | TCP socket entered memory pressure mode |
| `TCPAbortOnMemory` | Connection aborted due to memory pressure |
| `TCPBacklogDrop` | Segments dropped from socket backlog |

**TIME-WAIT management:**
| Metric | Signal |
|--------|--------|
| `TW` | TIME-WAIT sockets recycled normally |
| `TCPTimeWaitOverflow` | TIME-WAIT table overflowed |

---

## A real episode

A Node.js API server handling 50k req/s suddenly started dropping 0.5% of connections at 2 AM. The error was "ECONNREFUSED". Looking at TcpExt:

```
ListenOverflows: rising by ~300/second
ListenDrops: rising by ~300/second
```

The accept queue was full. The application was accepting connections but its event loop was blocking on a database query that had gone slow. New connections were queuing up, overflowing, and being dropped.

Fix: `sysctl net.core.somaxconn=4096` (was 128), `sysctl net.ipv4.tcp_max_syn_backlog=4096`. This bought time while fixing the slow query. The real fix was adding a connection pool timeout.

---

## How to watch TcpExt

```sh
# See all TcpExt counters
grep TcpExt /proc/net/netstat | awk 'NR==1{split($0,h)} NR==2{for(i=2;i<=NF;i++) if($i>0) print h[i], $i}'

# Watch specific counters for changes
watch -n 2 'grep TcpExt /proc/net/netstat | tr " " "\n" | paste - -'
```

---

## See also

- `net/snmp.Tcp` — simpler TCP summary (RFC MIB)
- `net/netstat.IpExt` — extended IP statistics
- `sourceguide.net/netstat` — full source overview
- `ss.tcp_established` — current connection count

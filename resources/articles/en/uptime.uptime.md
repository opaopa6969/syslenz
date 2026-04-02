# uptime

[日本語版](../ja/uptime.uptime.md)

---

## What is it?

`uptime` is the number of seconds the system has been running since it last booted. It comes from `/proc/uptime` — the first number in that file. Simple to read, easy to misread.

Think of it as the system's age since its last birthday (reboot). A server with 86,400 uptime seconds is 1 day old. One with 31,536,000 seconds has been running for a year without a reboot.

```
  /proc/uptime example:
  1234567.89  4987654.32
  ^            ^
  │            └─ total idle time across all CPUs (seconds)
  └─ uptime in seconds (this is what syslenz reports)

  1,234,567 seconds = ~14.3 days
```

---

## Why does it matter?

**Unexpected reboots.** If uptime suddenly resets to a small value, the system rebooted. This might be planned (kernel update), accidental (power failure, OOM kill cascade), or hostile (someone rebooted your database server at 3am). Monitoring uptime gives you a tripwire.

**Stale patches.** A server running for 400+ days has almost certainly missed kernel security patches. Linux requires a reboot to apply kernel updates. High uptime sounds good ("system is stable!") but often means "system is running a kernel from 2022 with known CVEs."

**Stability signal.** A server that keeps rebooting (uptime resets every few hours) has a serious problem — kernel panic, hardware fault, or a crash loop. Low uptime on a production server demands immediate investigation.

```
  Uptime as a health signal:

  New system:     [1 day]   <- normal post-deployment
  Healthy:        [7-90 days] <- patched regularly
  Patch debt:     [180+ days] <- security risk
  Crash loop:     resets repeatedly <- critical
```

---

## How to read it

```sh
# Human-readable uptime
uptime
# Example: 14:32:01 up 14 days, 3:17,  2 users,  load average: 0.52, 0.58, 0.61

# Raw seconds from /proc/uptime
awk '{print $1}' /proc/uptime

# When did the system boot?
who -b
# or
last reboot | head -3
```

| Uptime | Interpretation |
|--------|---------------|
| < 1 hour | Just rebooted — verify it was intentional |
| 1–7 days | Recent deployment or patch cycle |
| 7–90 days | Normal operational range for patched systems |
| 90–180 days | Review patch status; plan maintenance window |
| 180+ days | Likely security debt; kernel patches pending |
| Repeated resets | Crash loop or instability — investigate immediately |

**Check if a reboot was expected:**
```sh
# Was there a scheduled maintenance in the logs?
journalctl -b -1 | head -20   # previous boot's last messages
last reboot                    # reboot history
```

---

## A real episode

A fintech startup's payment processing service had been running for 270 days with no issues — the team was proud of the stability. Then a security audit flagged the server: it was running a kernel version with a local privilege escalation CVE that had been patched 8 months ago.

The team realized they had never set up automated kernel patching because "the system was stable and they didn't want to break anything." The 270-day uptime was a badge of honor that had become a liability.

Worse: when they finally planned the patching window, they discovered the system had been using features deprecated in newer kernels. The patch brought a breaking change. The "stable" system had accumulated 9 months of hidden technical debt.

**Lesson:** High uptime is not the same as healthy uptime. Track uptime not to maximize it, but to detect unexpected drops and enforce a maximum ceiling for security patches.

---

## What to do when it resets unexpectedly

**Step 1: Confirm the reboot happened.**
```sh
last reboot | head -5
who -b
```

**Step 2: Find out why.**
```sh
# Check kernel messages from the previous boot
journalctl -b -1 -p err | tail -50

# Was it a kernel panic?
journalctl -b -1 | grep -i "panic\|oops\|BUG\|killed"

# Was it an OOM kill?
journalctl -b -1 | grep "oom\|Out of memory"

# Check hardware errors
dmesg | grep -iE "hardware error|mce|edac"
```

**Step 3: Check for intentional causes.**
```sh
# Scheduled reboot (systemd)
systemctl list-timers | grep reboot

# Was a kernel update pending?
needs-restarting -r   # (RHEL/CentOS)
```

**What to do when uptime is too high (patch debt):**
```sh
# Check current kernel version
uname -r

# Compare with available updates (Debian/Ubuntu)
apt list --upgradable | grep linux-image

# Check for CVEs in current kernel (RHEL/CentOS)
yum updateinfo list security | grep kernel
```

---

## Common mistakes

**Maximizing uptime as a goal.** "999 days uptime!" is not a success metric for a Linux server. It usually means unpatched CVEs. Set a maximum uptime policy: "reboot for kernel patches within 30 days of release."

**Not alerting on unexpected reboots.** A server that reboots at 3am unannounced may have had a kernel panic. Without an alert on uptime reset, you might not know for days.

**Confusing uptime with reliability.** A server can have 365 days uptime while silently degrading — disk slowly filling, memory leaking, connections accumulating. Uptime tells you it hasn't rebooted, not that it's healthy.

**Ignoring the idle time field.** The second number in `/proc/uptime` is total CPU idle time. On a 4-core machine with 1000s uptime, an idle time of 3800s means the CPUs were 95% idle on average. Useful sanity check.

---

## See also

- `uptime.idle` — total CPU idle time from the same file; sanity-checks load average
- `stat.procs_running` — if high after a reboot, something is not starting cleanly
- `loadavg` — load average at 1/5/15 minutes; reboot resets this too
- `journalctl -b -1` — logs from the previous boot session for reboot root-cause analysis

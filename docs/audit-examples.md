# Audit and Compliance Examples

Practical workflows for using syslenz in security audits, compliance checks, and system inventory tasks.

## Table of Contents

- [Quick Reference](#quick-reference)
- [1. Security Audit: Open Network Connections](#1-security-audit-open-network-connections)
- [2. Kernel Module Inventory](#2-kernel-module-inventory)
- [3. Compliance: Cgroup Policy Verification](#3-compliance-cgroup-policy-verification)
- [4. System Inventory Across Hosts](#4-system-inventory-across-hosts)
- [5. Performance Baseline Comparison](#5-performance-baseline-comparison)
- [6. Mounted Filesystem Audit](#6-mounted-filesystem-audit)
- [7. Process Audit](#7-process-audit)
- [8. Crypto and Security Configuration](#8-crypto-and-security-configuration)
- [9. Automated CI/CD Compliance Gate](#9-automated-cicd-compliance-gate)

---

## Quick Reference

```bash
# Export full system snapshot
syslenz --export snapshot.json

# Export from remote host
syslenz --ssh user@host --export host-snapshot.json

# Export from Docker container
syslenz --docker my-container --export container-snapshot.json

# All examples below use jq to query the JSON export
```

The JSON export contains every data source with typed fields. The structure is:

```json
{
  "timestamp": "2026-03-29T12:00:00Z",
  "hostname": "web-server-01",
  "entries": {
    "meminfo": {
      "fields": [
        { "name": "MemTotal", "value": { "Bytes": 16777216000 }, "description": "..." },
        ...
      ]
    },
    "net/tcp": { ... },
    "modules": { ... },
    ...
  }
}
```

---

## 1. Security Audit: Open Network Connections

Identify all TCP connections, look for unexpected listeners or connections to unknown hosts.

```bash
# Export snapshot
syslenz --export audit.json

# List all TCP connections with state
jq '.entries["net/tcp"].fields[] | select(.name | startswith("conn_")) | {name, value}' audit.json

# Find ESTABLISHED connections (potential data exfiltration)
jq '.entries["net/tcp"].fields[] | select(.value.Text? // "" | contains("ESTABLISHED"))' audit.json

# Find LISTEN sockets (unexpected services)
jq '.entries["net/tcp"].fields[] | select(.value.Text? // "" | contains("LISTEN"))' audit.json

# Check for suspicious SYN_SENT connections (outbound connection attempts)
jq '.entries["net/tcp"].fields[] | select(.value.Text? // "" | contains("SYN_SENT"))' audit.json

# Check UDP listeners
jq '.entries["net/udp"].fields[] | select(.value.Text? // "" | contains("LISTEN"))' audit.json

# Socket summary
jq '.entries["net/sockstat"].fields[] | {name: .name, value: .value}' audit.json
```

### What to look for

- LISTEN ports that are not in your approved service list
- ESTABLISHED connections to IP ranges outside your expected set
- SYN_SENT floods (possible malware beaconing)
- CLOSE_WAIT accumulation (connection leak)
- TIME_WAIT excess (port exhaustion risk)

---

## 2. Kernel Module Inventory

Verify that only approved kernel modules are loaded.

```bash
# List all loaded kernel modules
jq '.entries["modules"].fields[] | {name: .name, value: .value}' audit.json

# Extract just module names
jq -r '.entries["modules"].fields[] | .name' audit.json | sort

# Compare against an approved module list
jq -r '.entries["modules"].fields[] | .name' audit.json | sort > loaded-modules.txt
diff approved-modules.txt loaded-modules.txt

# Check for known-risky modules
RISKY_MODULES="vboxdrv nf_conntrack_ftp ip_vs"
for mod in $RISKY_MODULES; do
    if jq -e ".entries[\"modules\"].fields[] | select(.name == \"$mod\")" audit.json > /dev/null 2>&1; then
        echo "WARNING: Risky module loaded: $mod"
    fi
done
```

---

## 3. Compliance: Cgroup Policy Verification

Ensure resource limits are properly configured for containerized workloads.

```bash
# List all cgroup controllers
jq '.entries["cgroups"].fields[] | {name: .name, value: .value}' audit.json

# Verify memory limits exist
jq '.entries["cgroups"].fields[] | select(.name | contains("memory"))' audit.json

# Check CPU quotas
jq '.entries["cgroups"].fields[] | select(.name | contains("cpu"))' audit.json
```

---

## 4. System Inventory Across Hosts

Capture and compare system state across multiple hosts.

```bash
# Capture snapshots from multiple hosts
for host in web-01 web-02 web-03 db-01; do
    syslenz --ssh user@$host --export "inventory-${host}.json"
done

# Compare kernel versions
for f in inventory-*.json; do
    HOST=$(basename "$f" .json | sed 's/inventory-//')
    VERSION=$(jq -r '.entries["version"].fields[0].value.Text // "unknown"' "$f")
    echo "$HOST: $VERSION"
done

# Compare memory across hosts
for f in inventory-*.json; do
    HOST=$(basename "$f" .json | sed 's/inventory-//')
    MEM=$(jq '.entries["meminfo"].fields[] | select(.name == "MemTotal") | .value.Bytes' "$f")
    echo "$HOST: $(echo "scale=1; $MEM / 1073741824" | bc) GB"
done

# Find hosts with swap disabled
for f in inventory-*.json; do
    HOST=$(basename "$f" .json | sed 's/inventory-//')
    SWAP=$(jq '.entries["meminfo"].fields[] | select(.name == "SwapTotal") | .value.Bytes' "$f")
    if [ "$SWAP" = "0" ]; then
        echo "WARNING: $HOST has no swap configured"
    fi
done

# Compare loaded modules between hosts
diff <(jq -r '.entries["modules"].fields[] | .name' inventory-web-01.json | sort) \
     <(jq -r '.entries["modules"].fields[] | .name' inventory-web-02.json | sort)
```

---

## 5. Performance Baseline Comparison

Capture baseline metrics, then compare after changes.

```bash
# Capture baseline before deployment
syslenz --export baseline.json

# ... deploy changes ...

# Capture post-deployment snapshot
syslenz --export after-deploy.json

# Compare memory usage
echo "=== Memory ==="
for field in MemAvailable MemFree Cached Buffers; do
    BEFORE=$(jq ".entries[\"meminfo\"].fields[] | select(.name == \"$field\") | .value.Bytes" baseline.json)
    AFTER=$(jq ".entries[\"meminfo\"].fields[] | select(.name == \"$field\") | .value.Bytes" after-deploy.json)
    DELTA=$(echo "$AFTER - $BEFORE" | bc)
    echo "$field: $(echo "scale=1; $DELTA / 1048576" | bc) MB change"
done

# Compare load averages
echo "=== Load ==="
for field in load1 load5 load15; do
    BEFORE=$(jq ".entries[\"loadavg\"].fields[] | select(.name == \"$field\") | .value.Float" baseline.json)
    AFTER=$(jq ".entries[\"loadavg\"].fields[] | select(.name == \"$field\") | .value.Float" after-deploy.json)
    echo "$field: $BEFORE -> $AFTER"
done

# Compare network connection counts
echo "=== Connections ==="
for field in tcp_established tcp_time_wait tcp_close_wait; do
    BEFORE=$(jq ".entries[\"net/sockstat\"].fields[] | select(.name == \"$field\") | .value.Integer // 0" baseline.json)
    AFTER=$(jq ".entries[\"net/sockstat\"].fields[] | select(.name == \"$field\") | .value.Integer // 0" after-deploy.json)
    echo "$field: $BEFORE -> $AFTER"
done

# Use syslenz built-in diff (in TUI)
# Import baseline, then compare with live data:
syslenz --import baseline.json
# Press 'd' for diff view, '[' and ']' for time-travel
```

---

## 6. Mounted Filesystem Audit

Verify mount options meet security requirements.

```bash
# List all mounted filesystems
jq '.entries["mounts"].fields[] | {name: .name, value: .value}' audit.json

# Check for filesystems without noexec on /tmp
jq -r '.entries["mounts"].fields[] | select(.value.Text? // "" | contains("/tmp")) | .value.Text' audit.json

# Check disk usage
jq '.entries["df"].fields[] | {name: .name, value: .value}' audit.json

# Find nearly-full filesystems (> 80%)
jq '.entries["df"].fields[] | select(.name | endswith("_use_pct")) | select(.value.Float > 80) | {name, pct: .value.Float}' audit.json
```

---

## 7. Process Audit

Review running processes for anomalies.

```bash
# List all processes
jq '.entries["processes"].fields[] | {name: .name, value: .value}' audit.json

# Find zombie processes
jq '.entries["processes"].fields[] | select(.value.Text? // "" | contains("zombie"))' audit.json

# Find processes in D-state (uninterruptible sleep -- possible I/O hang)
jq '.entries["processes"].fields[] | select(.value.Text? // "" | contains("D ("))' audit.json

# High-memory processes
jq '.entries["processes"].fields[] | select(.value.Bytes? > 1073741824) | {name, bytes: .value.Bytes}' audit.json
```

---

## 8. Crypto and Security Configuration

Audit available cryptographic algorithms and security settings.

```bash
# List available crypto algorithms
jq '.entries["crypto"].fields[] | {name: .name, value: .value}' audit.json

# Check for weak algorithms
WEAK_ALGOS="md4 md5 des rc4"
for algo in $WEAK_ALGOS; do
    if jq -e ".entries[\"crypto\"].fields[] | select(.name | ascii_downcase | contains(\"$algo\"))" audit.json > /dev/null 2>&1; then
        echo "INFO: Weak crypto algorithm available: $algo"
    fi
done

# I/O memory map (check for unexpected memory-mapped devices)
jq '.entries["iomem"].fields[] | {name: .name, value: .value}' audit.json

# I/O ports
jq '.entries["ioports"].fields[] | {name: .name, value: .value}' audit.json
```

---

## 9. Automated CI/CD Compliance Gate

Use syslenz in a CI pipeline to verify system configuration before deployment.

```bash
#!/bin/bash
# compliance-check.sh -- run as a CI step
set -e

syslenz --export /tmp/audit.json

FAIL=0

# Check: swap must be configured
SWAP=$(jq '.entries["meminfo"].fields[] | select(.name == "SwapTotal") | .value.Bytes' /tmp/audit.json)
if [ "$SWAP" = "0" ]; then
    echo "FAIL: No swap configured"
    FAIL=1
fi

# Check: disk usage must be under 80%
FULL_DISKS=$(jq '[.entries["df"].fields[] | select(.name | endswith("_use_pct")) | select(.value.Float > 80)] | length' /tmp/audit.json)
if [ "$FULL_DISKS" -gt "0" ]; then
    echo "FAIL: $FULL_DISKS filesystem(s) over 80% usage"
    FAIL=1
fi

# Check: no zombie processes
ZOMBIES=$(jq '[.entries["processes"].fields[] | select(.value.Text? // "" | contains("zombie"))] | length' /tmp/audit.json)
if [ "$ZOMBIES" -gt "0" ]; then
    echo "FAIL: $ZOMBIES zombie process(es) found"
    FAIL=1
fi

# Check: memory available > 10%
MEM_TOTAL=$(jq '.entries["meminfo"].fields[] | select(.name == "MemTotal") | .value.Bytes' /tmp/audit.json)
MEM_AVAIL=$(jq '.entries["meminfo"].fields[] | select(.name == "MemAvailable") | .value.Bytes' /tmp/audit.json)
PCT=$(echo "scale=0; $MEM_AVAIL * 100 / $MEM_TOTAL" | bc)
if [ "$PCT" -lt "10" ]; then
    echo "FAIL: Memory available only ${PCT}%"
    FAIL=1
fi

if [ "$FAIL" -eq "0" ]; then
    echo "PASS: All compliance checks passed"
else
    echo "FAIL: One or more compliance checks failed"
    exit 1
fi
```

---

## Tips

- Use `syslenz --ssh user@host --export` to audit remote hosts without installing syslenz on them (syslenz copies itself via SSH).
- Use the TUI `X` (Diagnostics) view for an interactive audit -- it automatically detects 25+ anomaly patterns.
- Schedule periodic snapshots with cron and compare with `diff` or `jq` for drift detection.
- Combine with `--prometheus` for continuous compliance monitoring via alerting rules.

---

See also: [Getting Started](en/getting-started.md) | [Data Sources](en/sources.md) | [Diagnostics](en/diagnostics.md)

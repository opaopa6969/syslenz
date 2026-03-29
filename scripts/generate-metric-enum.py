#!/usr/bin/env python3
"""
Generate a MetricField enum from syslenz source files.

Scans Rust parser modules under src/proc/ to extract (source, field) pairs,
then emits a Rust enum to stdout.

Usage:
    python3 scripts/generate-metric-enum.py > src/metric_field_generated.rs

The generated enum covers all fields discovered in parsers. Each variant
encodes the source name and field name so that the original string pair
can be recovered via source() and field() methods.

This is a *discovery* tool: it reads the existing codebase to find what
metrics exist. The authoritative enum definitions live in metric_kind.rs
and common_metric.rs (hand-written). This script helps bootstrap the
full per-platform MetricField enum.
"""

import os
import re
import sys
from collections import defaultdict
from pathlib import Path


def find_project_root() -> Path:
    """Walk up from script location to find Cargo.toml."""
    p = Path(__file__).resolve().parent
    while p != p.parent:
        if (p / "Cargo.toml").exists():
            return p
        p = p.parent
    print("ERROR: Could not find Cargo.toml in parent directories", file=sys.stderr)
    sys.exit(1)


def scan_parser_files(proc_dir: Path) -> dict[str, list[str]]:
    """
    Scan .rs files under src/proc/ for field name patterns.

    Looks for patterns like:
        name: "FieldName".into()
        Field { name: "FieldName".into(), ...
        "source_name" => "field_name"

    Returns { source_name: [field_name, ...] }
    """
    metrics: dict[str, list[str]] = defaultdict(list)

    # Map file stems to source names (some differ)
    stem_to_source = {
        "meminfo": "meminfo",
        "uptime": "uptime",
        "loadavg": "loadavg",
        "version": "version",
        "mounts": "mounts",
        "partitions": "partitions",
        "cpuinfo": "cpuinfo",
        "stat": "stat",
        "net_dev": "net/dev",
        "diskstats": "diskstats",
        "processes": "processes",
        "swaps": "swaps",
        "buddyinfo": "buddyinfo",
        "cgroups": "cgroups",
        "cmdline": "cmdline",
        "consoles": "consoles",
        "crypto": "crypto",
        "devices": "devices",
        "filesystems": "filesystems",
        "interrupts": "interrupts",
        "iomem": "iomem",
        "ioports": "ioports",
        "locks": "locks",
        "modules": "modules",
        "vmstat": "vmstat",
        "zoneinfo": "zoneinfo",
        "softirqs": "softirqs",
        "misc": "misc",
        "pressure": "pressure",
        "net_tcp": "net/tcp",
        "net_udp": "net/udp",
        "net_unix": "net/unix",
        "net_arp": "net/arp",
        "net_route": "net/route",
        "net_sockstat": "net/sockstat",
        "net_snmp": "net/snmp",
        "net_netstat": "net/netstat",
        "net_wireless": "net/wireless",
        "slabinfo": "slabinfo",
        "pagetypeinfo": "pagetypeinfo",
        "schedstat": "schedstat",
        "dma": "dma",
        "timer_list": "timer_list",
    }

    # Pattern: Field { name: "something".into(), ...
    field_pat = re.compile(r'name:\s*"([^"]+)"\.into\(\)')
    # Pattern: entries.insert("source".into(), ...)  -- for platform files
    insert_pat = re.compile(r'entries\.insert\("([^"]+)"\.into\(\)')

    for rs_file in sorted(proc_dir.glob("*.rs")):
        stem = rs_file.stem
        if stem in ("mod", "platform_macos", "platform_windows"):
            continue
        source = stem_to_source.get(stem, stem)

        try:
            content = rs_file.read_text(encoding="utf-8")
        except Exception:
            continue

        for m in field_pat.finditer(content):
            field_name = m.group(1)
            if field_name not in metrics[source]:
                metrics[source].append(field_name)

    return dict(metrics)


def source_field_to_variant(source: str, field: str) -> str:
    """
    Convert a (source, field) pair into a Rust enum variant name.

    Examples:
        ("meminfo", "MemTotal") -> "MeminfoMemTotal"
        ("net/dev", "rx_bytes") -> "NetDevRxBytes"
        ("stat", "cpu_usage_pct") -> "StatCpuUsagePct"
        ("file-nr", "fd_usage_pct") -> "FileNrFdUsagePct"
    """
    def to_pascal(s: str) -> str:
        """Convert snake_case/kebab-case/slash-separated to PascalCase."""
        parts = re.split(r'[_/\-]+', s)
        return ''.join(p.capitalize() for p in parts if p)

    source_part = to_pascal(source)
    # If field is already PascalCase, keep it; otherwise convert
    if '_' in field or '-' in field:
        field_part = to_pascal(field)
    else:
        # Already PascalCase (like "MemTotal") — keep as-is
        field_part = field

    return source_part + field_part


def guess_kind(source: str, field: str) -> str:
    """Guess the MetricKind based on source/field names."""
    mem_sources = {"meminfo", "vmstat", "swaps", "buddyinfo", "zoneinfo", "slabinfo", "pagetypeinfo"}
    cpu_sources = {"stat", "loadavg", "cpuinfo", "schedstat", "softirqs", "interrupts", "thermal"}
    net_sources = {"net/dev", "net/tcp", "net/udp", "net/unix", "net/arp",
                   "net/route", "net/sockstat", "net/snmp", "net/netstat", "net/wireless"}
    storage_sources = {"diskstats", "df", "mounts", "partitions", "locks"}
    process_sources = {"processes", "file-nr"}
    security_sources = {"crypto"}
    system_sources = {"uptime", "version", "cmdline", "cgroups", "consoles", "devices",
                      "filesystems", "iomem", "ioports", "misc", "dma", "timer_list", "modules"}

    if source in mem_sources:
        return "MetricKind::Memory"
    if source in cpu_sources:
        return "MetricKind::Cpu"
    if source in net_sources:
        return "MetricKind::Network"
    if source in storage_sources:
        return "MetricKind::Storage"
    if source in process_sources:
        return "MetricKind::Process"
    if source in security_sources:
        return "MetricKind::Security"
    if source in system_sources:
        return "MetricKind::System"

    # Fallback: check field name heuristics
    fl = field.lower()
    if "mem" in fl or "swap" in fl or "cache" in fl or "buffer" in fl:
        return "MetricKind::Memory"
    if "cpu" in fl or "load" in fl:
        return "MetricKind::Cpu"
    if "net" in fl or "tcp" in fl or "udp" in fl or "rx" in fl or "tx" in fl:
        return "MetricKind::Network"
    if "disk" in fl or "mount" in fl:
        return "MetricKind::Storage"
    if "process" in fl or "pid" in fl or "fd" in fl:
        return "MetricKind::Process"

    return "MetricKind::System"


def guess_unit(source: str, field: str) -> str:
    """Guess the MetricUnit based on field naming conventions."""
    fl = field.lower()
    if fl.endswith("_pct") or fl.endswith("_percent") or "usage_pct" in fl or "percent" in fl:
        return "MetricUnit::Percent"
    if any(kw in fl for kw in ["bytes", "memtotal", "memfree", "memavailable",
                                "buffers", "cached", "swaptotal", "swapfree",
                                "rx_bytes", "tx_bytes", "shmem", "slab",
                                "committed_as", "writeback", "dirty",
                                "active_anon", "inactive_anon", "active_file", "inactive_file"]):
        return "MetricUnit::Bytes"
    if "temp" in fl or "celsius" in fl:
        return "MetricUnit::Celsius"
    if fl in ("uptime",) or "seconds" in fl or "duration" in fl:
        return "MetricUnit::Seconds"
    if "per_sec" in fl or "persec" in fl:
        return "MetricUnit::CountPerSec"
    if any(kw in fl for kw in ["count", "total", "nr", "number"]):
        return "MetricUnit::Count"
    return "MetricUnit::None"


def generate_enum(metrics: dict[str, list[str]]) -> str:
    """Generate the Rust enum code."""
    lines = []
    lines.append("// AUTO-GENERATED by scripts/generate-metric-enum.py")
    lines.append("// Do not edit manually. Re-run the script to regenerate.")
    lines.append("//")
    lines.append(f"// Total: {{count}} variants from {{sources}} sources")
    lines.append("")
    lines.append("use crate::metric_kind::MetricKind;")
    lines.append("use crate::common_metric::MetricUnit;")
    lines.append("use crate::proc::{FieldValue, Snapshot};")
    lines.append("")
    lines.append("/// All known Linux metric fields, auto-generated from parser source code.")
    lines.append("///")
    lines.append("/// Each variant uniquely identifies a (source, field) pair.")
    lines.append("/// Use `source()` and `field()` to recover the string pair,")
    lines.append("/// `kind()` for the category, and `extract()` to pull a value")
    lines.append("/// from a Snapshot.")
    lines.append("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")
    lines.append("#[allow(non_camel_case_types)]")
    lines.append("pub enum MetricField {")

    total = 0
    for source in sorted(metrics.keys()):
        fields = metrics[source]
        if not fields:
            continue
        lines.append(f"    // --- {source} ---")
        for field in fields:
            variant = source_field_to_variant(source, field)
            lines.append(f"    {variant},")
            total += 1
    lines.append("}")
    lines.append("")

    # source() method
    lines.append("impl MetricField {")
    lines.append("    /// The source name (e.g. \"meminfo\", \"net/tcp\").")
    lines.append("    pub fn source(&self) -> &'static str {")
    lines.append("        match self {")
    for source in sorted(metrics.keys()):
        fields = metrics[source]
        if not fields:
            continue
        variants = [source_field_to_variant(source, f) for f in fields]
        variant_list = " | ".join(f"MetricField::{v}" for v in variants)
        # Split long lines
        if len(variant_list) > 90:
            lines.append(f"            // {source}")
            for v in variants:
                lines.append(f"            MetricField::{v} |")
            # Fix last line: remove trailing |
            lines[-1] = lines[-1].rstrip(" |")
            lines.append(f"                => \"{source}\",")
        else:
            lines.append(f"            {variant_list} => \"{source}\",")
    lines.append("        }")
    lines.append("    }")
    lines.append("")

    # field() method
    lines.append("    /// The field name (e.g. \"MemTotal\", \"cpu_usage_pct\").")
    lines.append("    pub fn field(&self) -> &'static str {")
    lines.append("        match self {")
    for source in sorted(metrics.keys()):
        for field in metrics[source]:
            variant = source_field_to_variant(source, field)
            lines.append(f"            MetricField::{variant} => \"{field}\",")
    lines.append("        }")
    lines.append("    }")
    lines.append("")

    # kind() method
    lines.append("    /// The metric category.")
    lines.append("    pub fn kind(&self) -> MetricKind {")
    lines.append("        match self {")
    # Group by kind for cleaner output
    kind_groups: dict[str, list[str]] = defaultdict(list)
    for source in sorted(metrics.keys()):
        for field in metrics[source]:
            variant = source_field_to_variant(source, field)
            kind = guess_kind(source, field)
            kind_groups[kind].append(variant)
    for kind in sorted(kind_groups.keys()):
        variants = kind_groups[kind]
        if len(variants) <= 3:
            variant_list = " | ".join(f"MetricField::{v}" for v in variants)
            lines.append(f"            {variant_list} => {kind},")
        else:
            lines.append(f"            // {kind}")
            for v in variants:
                lines.append(f"            MetricField::{v} |")
            lines[-1] = lines[-1].rstrip(" |")
            lines.append(f"                => {kind},")
    lines.append("        }")
    lines.append("    }")
    lines.append("")

    # unit() method
    lines.append("    /// The unit of measurement.")
    lines.append("    pub fn unit(&self) -> MetricUnit {")
    lines.append("        match self {")
    unit_groups: dict[str, list[str]] = defaultdict(list)
    for source in sorted(metrics.keys()):
        for field in metrics[source]:
            variant = source_field_to_variant(source, field)
            unit = guess_unit(source, field)
            unit_groups[unit].append(variant)
    for unit in sorted(unit_groups.keys()):
        variants = unit_groups[unit]
        if len(variants) <= 3:
            variant_list = " | ".join(f"MetricField::{v}" for v in variants)
            lines.append(f"            {variant_list} => {unit},")
        else:
            lines.append(f"            // {unit}")
            for v in variants:
                lines.append(f"            MetricField::{v} |")
            lines[-1] = lines[-1].rstrip(" |")
            lines.append(f"                => {unit},")
    lines.append("        }")
    lines.append("    }")
    lines.append("")

    # extract() method
    lines.append("    /// Extract this metric's value from a Snapshot.")
    lines.append("    pub fn extract<'a>(&self, snapshot: &'a Snapshot) -> Option<&'a FieldValue> {")
    lines.append("        snapshot")
    lines.append("            .entries")
    lines.append("            .get(self.source())")
    lines.append("            .and_then(|entry| entry.fields.iter().find(|f| f.name == self.field()))")
    lines.append("            .map(|f| &f.value)")
    lines.append("    }")
    lines.append("")

    # all() method
    lines.append("    /// All known metric fields.")
    lines.append("    pub fn all() -> &'static [MetricField] {")
    lines.append("        &[")
    for source in sorted(metrics.keys()):
        for field in metrics[source]:
            variant = source_field_to_variant(source, field)
            lines.append(f"            MetricField::{variant},")
    lines.append("        ]")
    lines.append("    }")
    lines.append("}")

    # Fill in count placeholder
    source_count = len([s for s in metrics if metrics[s]])
    result = "\n".join(lines)
    result = result.replace("{count}", str(total)).replace("{sources}", str(source_count))
    return result


def main():
    root = find_project_root()
    proc_dir = root / "src" / "proc"

    if not proc_dir.is_dir():
        print(f"ERROR: {proc_dir} is not a directory", file=sys.stderr)
        sys.exit(1)

    metrics = scan_parser_files(proc_dir)

    total_fields = sum(len(v) for v in metrics.values())
    print(f"// Scanned {len(metrics)} sources, found {total_fields} fields", file=sys.stderr)

    code = generate_enum(metrics)
    print(code)


if __name__ == "__main__":
    main()

"""Data models mirroring the Rust Snapshot / ProcEntry / Field structures.

These are plain Python classes with no external dependencies.  They are
constructed from the JSON that syslenz sends over TCP in response to a
``SNAPSHOT`` command.

Rust reference (src/proc/mod.rs)::

    pub struct Snapshot {
        pub timestamp: SystemTime,          // ISO-8601 string in JSON
        pub entries: BTreeMap<String, ProcEntry>,
    }
    pub struct ProcEntry {
        pub source: String,
        pub fields: Vec<Field>,
    }
    pub struct Field {
        pub name: String,
        pub value: FieldValue,
        pub unit: Option<String>,
        pub description: String,
    }
    pub enum FieldValue {
        Bytes(u64),
        Integer(i64),
        Float(f64),
        Text(String),
        Duration(f64),
        Table(Vec<Vec<String>>),
    }
"""

from __future__ import annotations

from typing import Any, Dict, List, Optional, Union


class FieldValue:
    """Typed metric value, corresponding to the Rust ``FieldValue`` enum.

    Each instance has a ``kind`` (one of ``"Bytes"``, ``"Integer"``,
    ``"Float"``, ``"Text"``, ``"Duration"``, ``"Table"``) and a ``raw``
    value holding the native Python representation.

    Attributes:
        kind: The variant name (e.g. ``"Bytes"``).
        raw:  The unwrapped value -- ``int``, ``float``, ``str``, or
              ``list[list[str]]`` for ``Table``.
    """

    __slots__ = ("kind", "raw")

    def __init__(self, kind: str, raw: Any) -> None:
        self.kind = kind
        self.raw = raw

    # -- Convenience accessors ------------------------------------------------

    def as_int(self) -> int:
        """Return the value as an ``int``.  Works for Bytes, Integer."""
        return int(self.raw)

    def as_float(self) -> float:
        """Return the value as a ``float``.  Works for Float, Duration."""
        return float(self.raw)

    def as_str(self) -> str:
        """Return a human-readable string representation."""
        if self.kind == "Bytes":
            return _format_bytes(int(self.raw))
        if self.kind == "Duration":
            return _format_duration(float(self.raw))
        if self.kind == "Table":
            rows = self.raw  # type: ignore[assignment]
            return f"[{len(rows)} rows]"
        return str(self.raw)

    # -- Construction from JSON -----------------------------------------------

    @classmethod
    def from_json(cls, obj: Any) -> "FieldValue":
        """Parse a FieldValue from the JSON representation.

        The Rust serialisation uses a tagged enum, so the JSON looks like
        ``{"Bytes": 1024}`` or ``{"Text": "hello"}``.
        """
        if isinstance(obj, dict):
            for variant in ("Bytes", "Integer", "Float", "Text", "Duration", "Table"):
                if variant in obj:
                    return cls(variant, obj[variant])
            # Fallback: treat the whole dict as Text
            return cls("Text", str(obj))
        # Bare scalar (shouldn't happen with the Rust serialiser, but be safe)
        if isinstance(obj, int):
            return cls("Integer", obj)
        if isinstance(obj, float):
            return cls("Float", obj)
        return cls("Text", str(obj))

    # -- Dunder ---------------------------------------------------------------

    def __repr__(self) -> str:
        return f"FieldValue({self.kind!r}, {self.raw!r})"

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, FieldValue):
            return NotImplemented
        return self.kind == other.kind and self.raw == other.raw


class Field:
    """A single named metric inside a :class:`ProcEntry`.

    Attributes:
        name:        Metric name (e.g. ``"mem_total"``).
        value:       Typed value as a :class:`FieldValue`.
        unit:        Optional unit string (e.g. ``"bytes"``, ``"%"``).
        description: Human-readable description.
    """

    __slots__ = ("name", "value", "unit", "description")

    def __init__(
        self,
        name: str,
        value: FieldValue,
        unit: Optional[str],
        description: str,
    ) -> None:
        self.name = name
        self.value = value
        self.unit = unit
        self.description = description

    @classmethod
    def from_json(cls, obj: Dict[str, Any]) -> "Field":
        return cls(
            name=obj.get("name", ""),
            value=FieldValue.from_json(obj.get("value")),
            unit=obj.get("unit"),
            description=obj.get("description", ""),
        )

    def __repr__(self) -> str:
        return f"Field({self.name!r}, {self.value!r})"


class ProcEntry:
    """A group of metrics from a single source (e.g. a process).

    Attributes:
        source: Source identifier (e.g. ``"system"``).
        fields: List of :class:`Field` objects.
    """

    __slots__ = ("source", "fields")

    def __init__(self, source: str, fields: List[Field]) -> None:
        self.source = source
        self.fields = fields

    def get_field(self, name: str) -> Optional[Field]:
        """Look up a field by name.  Returns ``None`` if not found."""
        for f in self.fields:
            if f.name == name:
                return f
        return None

    def field_names(self) -> List[str]:
        """Return a list of all field names in this entry."""
        return [f.name for f in self.fields]

    @classmethod
    def from_json(cls, obj: Dict[str, Any]) -> "ProcEntry":
        fields = [Field.from_json(f) for f in obj.get("fields", [])]
        return cls(source=obj.get("source", ""), fields=fields)

    def __repr__(self) -> str:
        return f"ProcEntry({self.source!r}, {len(self.fields)} fields)"


class Snapshot:
    """A point-in-time system snapshot returned by syslenz.

    Attributes:
        timestamp: ISO-8601 timestamp string (e.g. ``"2025-01-15T10:30:00Z"``).
        entries:   Ordered dict mapping entry names to :class:`ProcEntry` objects.
        raw:       The original parsed JSON dict, for advanced access.
    """

    __slots__ = ("timestamp", "entries", "raw")

    def __init__(
        self,
        timestamp: str,
        entries: Dict[str, ProcEntry],
        raw: Dict[str, Any],
    ) -> None:
        self.timestamp = timestamp
        self.entries = entries
        self.raw = raw

    # -- Convenience helpers --------------------------------------------------

    def get_field(
        self, field_name: str, entry_name: Optional[str] = None
    ) -> Optional[Field]:
        """Find a field by name, optionally scoped to a specific entry.

        If *entry_name* is ``None``, the first matching field across all
        entries is returned.
        """
        if entry_name is not None:
            entry = self.entries.get(entry_name)
            return entry.get_field(field_name) if entry else None
        for entry in self.entries.values():
            field = entry.get_field(field_name)
            if field is not None:
                return field
        return None

    def get_value(
        self, field_name: str, entry_name: Optional[str] = None
    ) -> Optional[Any]:
        """Shortcut: return the raw value of a field, or ``None``."""
        field = self.get_field(field_name, entry_name)
        return field.value.raw if field else None

    def entry_names(self) -> List[str]:
        """Return a list of all entry names in this snapshot."""
        return list(self.entries.keys())

    @classmethod
    def from_json(cls, obj: Dict[str, Any]) -> "Snapshot":
        """Construct a Snapshot from parsed JSON.

        Handles both the full Snapshot format (``{timestamp, entries}``)
        and the single-ProcEntry format used by the Java SDK
        (``{source, fields}``).
        """
        timestamp = obj.get("timestamp", "")
        raw_entries = obj.get("entries", {})

        # Full Snapshot format: entries is a dict of ProcEntry objects
        if raw_entries and isinstance(raw_entries, dict):
            entries = {
                k: ProcEntry.from_json(v) for k, v in raw_entries.items()
            }
            return cls(timestamp=timestamp, entries=entries, raw=obj)

        # Single ProcEntry format (e.g. from Java SDK or simple providers)
        if "source" in obj and "fields" in obj:
            entry = ProcEntry.from_json(obj)
            return cls(
                timestamp=timestamp,
                entries={entry.source: entry},
                raw=obj,
            )

        # Fallback: empty snapshot
        return cls(timestamp=timestamp, entries={}, raw=obj)

    def __repr__(self) -> str:
        return (
            f"Snapshot(timestamp={self.timestamp!r}, "
            f"{len(self.entries)} entries)"
        )


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def _format_bytes(b: int) -> str:
    """Human-readable byte count (e.g. ``1.5 GiB``)."""
    for unit in ("B", "KiB", "MiB", "GiB", "TiB"):
        if abs(b) < 1024.0:
            return f"{b:.1f} {unit}"
        b_f = b / 1024.0  # type: ignore[assignment]
        b = int(b_f) if unit != "TiB" else b  # keep iterating
    return f"{b:.1f} TiB"


def _format_duration(secs: float) -> str:
    """Human-readable duration."""
    if secs < 60:
        return f"{secs:.1f}s"
    if secs < 3600:
        return f"{secs / 60:.1f}m"
    if secs < 86400:
        return f"{secs / 3600:.1f}h"
    return f"{secs / 86400:.1f}d"

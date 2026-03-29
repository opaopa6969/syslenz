"""syslenz4py - Python client for syslenz system monitoring.

Connect to a syslenz instance running in --serve mode and retrieve
system snapshots over TCP using the syslenz protocol.

Usage::

    from syslenz4py import SyslenzClient

    client = SyslenzClient("localhost", 9100)
    client.connect()
    snapshot = client.get_snapshot()
    print(snapshot.timestamp)
    for name, entry in snapshot.entries.items():
        print(f"  {entry.source}: {len(entry.fields)} fields")
    client.close()

No external dependencies -- uses only the Python standard library.
"""

__version__ = "1.0.0"

from syslenz4py.models import FieldValue, Field, ProcEntry, Snapshot
from syslenz4py.client import SyslenzClient

__all__ = [
    "SyslenzClient",
    "Snapshot",
    "ProcEntry",
    "Field",
    "FieldValue",
]

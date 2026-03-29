"""TCP client for the syslenz ``--serve`` protocol.

Protocol summary (see ``src/serve.rs``):

1. Client opens a TCP connection to the syslenz server.
2. Client sends ``SNAPSHOT\\n``.
3. Server responds with a single-line JSON snapshot followed by ``\\n``.
4. Connection can be reused or closed by either side.

The ``METRICS`` command is also supported and returns Prometheus-format
text, but this client focuses on ``SNAPSHOT`` (JSON).
"""

from __future__ import annotations

import json
import socket
from typing import Any, Dict, Optional

from syslenz4py.models import Field, Snapshot


class SyslenzError(Exception):
    """Base exception for syslenz client errors."""


class ConnectionError(SyslenzError):
    """Raised when a connection to syslenz cannot be established."""


class ProtocolError(SyslenzError):
    """Raised when the server response cannot be parsed."""


class SyslenzClient:
    """Client for connecting to a syslenz instance in ``--serve`` mode.

    Basic usage::

        client = SyslenzClient("localhost", 9100)
        client.connect()
        snap = client.get_snapshot()
        print(snap)
        client.close()

    Context-manager usage::

        with SyslenzClient("localhost", 9100) as client:
            snap = client.get_snapshot()

    Parameters:
        host:    Hostname or IP of the syslenz server.
        port:    TCP port (default 9100).
        timeout: Socket timeout in seconds (default 10).
    """

    DEFAULT_PORT = 9100
    DEFAULT_TIMEOUT = 10.0

    def __init__(
        self,
        host: str = "localhost",
        port: int = DEFAULT_PORT,
        timeout: float = DEFAULT_TIMEOUT,
    ) -> None:
        self.host = host
        self.port = port
        self.timeout = timeout
        self._sock: Optional[socket.socket] = None

    # -- Connection lifecycle -------------------------------------------------

    def connect(self) -> "SyslenzClient":
        """Open a TCP connection to the syslenz server.

        Returns *self* so calls can be chained::

            snap = SyslenzClient("host").connect().get_snapshot()

        Raises:
            ConnectionError: If the connection fails.
        """
        if self._sock is not None:
            return self
        try:
            sock = socket.create_connection(
                (self.host, self.port), timeout=self.timeout
            )
            sock.settimeout(self.timeout)
            self._sock = sock
        except OSError as exc:
            raise ConnectionError(
                f"Failed to connect to {self.host}:{self.port}: {exc}"
            ) from exc
        return self

    def close(self) -> None:
        """Close the TCP connection (idempotent)."""
        sock = self._sock
        if sock is not None:
            self._sock = None
            try:
                sock.close()
            except OSError:
                pass

    @property
    def connected(self) -> bool:
        """``True`` if a socket is currently open."""
        return self._sock is not None

    # -- Context manager ------------------------------------------------------

    def __enter__(self) -> "SyslenzClient":
        self.connect()
        return self

    def __exit__(self, *exc: Any) -> None:
        self.close()

    # -- Snapshot retrieval ---------------------------------------------------

    def get_snapshot(self) -> Snapshot:
        """Send ``SNAPSHOT`` and return the parsed :class:`Snapshot`.

        A new TCP connection is created for each call (matching the
        server's one-request-per-connection behaviour in ``serve.rs``).

        Raises:
            ConnectionError: If not connected and auto-connect fails.
            ProtocolError:   If the response is not valid JSON.
        """
        raw = self._request("SNAPSHOT")
        try:
            data: Dict[str, Any] = json.loads(raw)
        except json.JSONDecodeError as exc:
            raise ProtocolError(f"Invalid JSON from server: {exc}") from exc

        if "error" in data:
            raise ProtocolError(f"Server error: {data['error']}")

        return Snapshot.from_json(data)

    def get_snapshot_raw(self) -> Dict[str, Any]:
        """Send ``SNAPSHOT`` and return the raw parsed JSON dict.

        Useful when you want to inspect the full structure without the
        model layer.
        """
        raw = self._request("SNAPSHOT")
        try:
            return json.loads(raw)
        except json.JSONDecodeError as exc:
            raise ProtocolError(f"Invalid JSON from server: {exc}") from exc

    def get_field(
        self, field_name: str, entry_name: Optional[str] = None
    ) -> Optional[Field]:
        """Convenience: fetch a snapshot and extract a single field.

        Parameters:
            field_name:  The metric name to look up.
            entry_name:  Optional entry to scope the search.

        Returns:
            The :class:`Field`, or ``None`` if not found.
        """
        snap = self.get_snapshot()
        return snap.get_field(field_name, entry_name)

    def get_value(
        self, field_name: str, entry_name: Optional[str] = None
    ) -> Optional[Any]:
        """Convenience: fetch a snapshot and return a field's raw value."""
        snap = self.get_snapshot()
        return snap.get_value(field_name, entry_name)

    # -- Internal helpers -----------------------------------------------------

    def _request(self, command: str) -> str:
        """Send a command and read a single-line response.

        The syslenz serve protocol closes after one request, so we open
        a fresh connection for each call.
        """
        try:
            sock = socket.create_connection(
                (self.host, self.port), timeout=self.timeout
            )
            sock.settimeout(self.timeout)
        except OSError as exc:
            raise ConnectionError(
                f"Failed to connect to {self.host}:{self.port}: {exc}"
            ) from exc

        try:
            # Send command
            sock.sendall((command + "\n").encode("utf-8"))

            # Read response (single line terminated by \n)
            buf = bytearray()
            while True:
                chunk = sock.recv(65536)
                if not chunk:
                    break
                buf.extend(chunk)
                if b"\n" in buf:
                    break

            response = buf.decode("utf-8").strip()
            if not response:
                raise ProtocolError("Empty response from server")
            return response
        finally:
            try:
                sock.close()
            except OSError:
                pass

    def __repr__(self) -> str:
        state = "connected" if self.connected else "disconnected"
        return f"SyslenzClient({self.host!r}, {self.port}, {state})"

    def __del__(self) -> None:
        self.close()

/**
 * SyslenzClient — TCP client for syslenz --serve mode.
 *
 * Protocol:
 *   1. Client connects to the syslenz TCP server.
 *   2. Client sends "SNAPSHOT\n".
 *   3. Server responds with a single-line JSON (Snapshot format).
 *   4. Connection can be reused or closed.
 *
 * The client can operate in two modes:
 *   - One-shot: connect, fetch one snapshot, close.
 *   - Persistent: connect once, fetch multiple snapshots, close when done.
 */

import { createConnection } from "node:net";
import { unwrapFieldValue, formatFieldValue } from "./models.js";

/** Default connection timeout in milliseconds. */
const DEFAULT_TIMEOUT_MS = 10_000;

/** Default port for syslenz --serve. */
const DEFAULT_PORT = 9100;

export class SyslenzClient {
  #host;
  #port;
  #timeoutMs;
  #socket;
  #connected;

  /**
   * Create a new SyslenzClient.
   *
   * @param {object} [options]
   * @param {string} [options.host="localhost"] - Host to connect to
   * @param {number} [options.port=9100] - Port to connect to
   * @param {number} [options.timeout=10000] - Connection/read timeout in ms
   */
  constructor(options = {}) {
    this.#host = options.host ?? "localhost";
    this.#port = options.port ?? DEFAULT_PORT;
    this.#timeoutMs = options.timeout ?? DEFAULT_TIMEOUT_MS;
    this.#socket = null;
    this.#connected = false;
  }

  /**
   * Connect to the syslenz TCP server.
   *
   * @returns {Promise<void>}
   * @throws {Error} If the connection fails or times out
   */
  connect() {
    return new Promise((resolve, reject) => {
      if (this.#connected) {
        resolve();
        return;
      }

      const socket = createConnection(
        { host: this.#host, port: this.#port },
        () => {
          this.#socket = socket;
          this.#connected = true;
          resolve();
        }
      );

      socket.setTimeout(this.#timeoutMs);

      socket.on("timeout", () => {
        socket.destroy();
        this.#connected = false;
        this.#socket = null;
        reject(new Error(`Connection to ${this.#host}:${this.#port} timed out`));
      });

      socket.on("error", (err) => {
        this.#connected = false;
        this.#socket = null;
        reject(new Error(`Failed to connect to ${this.#host}:${this.#port}: ${err.message}`));
      });

      socket.on("close", () => {
        this.#connected = false;
        this.#socket = null;
      });
    });
  }

  /**
   * Fetch a snapshot from the server.
   *
   * If not connected, establishes a new connection first. After receiving
   * the response the connection is closed (syslenz uses one-request-per-
   * connection semantics by default).
   *
   * @returns {Promise<object>} Parsed Snapshot JSON
   * @throws {Error} On connection or parse errors
   */
  async getSnapshot() {
    // Always open a fresh connection — syslenz server breaks after one
    // SNAPSHOT response (see serve.rs: `break` after response).
    this.close();
    await this.connect();

    return new Promise((resolve, reject) => {
      const socket = this.#socket;
      if (!socket) {
        reject(new Error("Not connected"));
        return;
      }

      let data = "";

      socket.on("data", (chunk) => {
        data += chunk.toString("utf-8");
      });

      socket.on("end", () => {
        this.#connected = false;
        this.#socket = null;
        try {
          const snapshot = JSON.parse(data.trim());
          resolve(snapshot);
        } catch (err) {
          reject(
            new Error(`Failed to parse snapshot JSON: ${err.message}\n(first 200 chars: ${data.slice(0, 200)})`)
          );
        }
      });

      socket.on("error", (err) => {
        this.#connected = false;
        this.#socket = null;
        reject(new Error(`Error reading snapshot: ${err.message}`));
      });

      // Send the SNAPSHOT command
      socket.write("SNAPSHOT\n");
    });
  }

  /**
   * Fetch a snapshot and extract a single field by name.
   *
   * Searches across all entries in the snapshot for a field with
   * the given name.
   *
   * @param {string} fieldName - The field name to look up (e.g. "cpu_total")
   * @returns {Promise<{ entry: string, field: object, value: *, type: string } | null>}
   */
  async getField(fieldName) {
    const snapshot = await this.getSnapshot();

    if (!snapshot.entries) return null;

    for (const [entryName, entry] of Object.entries(snapshot.entries)) {
      if (!entry.fields) continue;
      for (const field of entry.fields) {
        if (field.name === fieldName) {
          const { type, value } = unwrapFieldValue(field.value);
          return {
            entry: entryName,
            field,
            value,
            type,
            display: formatFieldValue(field.value),
          };
        }
      }
    }

    return null;
  }

  /**
   * Close the connection. Safe to call even if not connected.
   */
  close() {
    if (this.#socket) {
      this.#socket.destroy();
      this.#socket = null;
    }
    this.#connected = false;
  }

  /**
   * Whether the client is currently connected.
   *
   * @returns {boolean}
   */
  get isConnected() {
    return this.#connected;
  }

  /**
   * The address this client connects to.
   *
   * @returns {string}
   */
  get address() {
    return `${this.#host}:${this.#port}`;
  }
}

/**
 * Convenience function: fetch a single snapshot without managing a client.
 *
 * @param {string} [addr="localhost:9100"] - Address in "host:port" format
 * @param {number} [timeout=10000] - Timeout in ms
 * @returns {Promise<object>} Parsed Snapshot JSON
 */
export async function fetchSnapshot(addr = "localhost:9100", timeout = DEFAULT_TIMEOUT_MS) {
  const [host, portStr] = addr.split(":");
  const port = parseInt(portStr, 10) || DEFAULT_PORT;
  const client = new SyslenzClient({ host, port, timeout });
  try {
    return await client.getSnapshot();
  } finally {
    client.close();
  }
}

/**
 * Poll snapshots at a fixed interval.
 *
 * Returns an AsyncGenerator that yields snapshots. Transient errors are
 * silently skipped (up to maxConsecutiveFailures). The generator runs
 * until the caller breaks out or the failure limit is exceeded.
 *
 * @param {object} [options]
 * @param {string} [options.host="localhost"]
 * @param {number} [options.port=9100]
 * @param {number} [options.interval=2000] - Polling interval in ms
 * @param {number} [options.maxConsecutiveFailures=5]
 * @yields {object} Snapshot JSON
 */
export async function* streamSnapshots(options = {}) {
  const interval = options.interval ?? 2000;
  const maxFail = options.maxConsecutiveFailures ?? 5;
  const client = new SyslenzClient(options);
  let consecutiveFailures = 0;

  try {
    while (true) {
      try {
        const snapshot = await client.getSnapshot();
        consecutiveFailures = 0;
        yield snapshot;
      } catch {
        consecutiveFailures++;
        if (consecutiveFailures >= maxFail) {
          throw new Error(
            `Streaming stopped after ${maxFail} consecutive failures`
          );
        }
      }
      await new Promise((resolve) => setTimeout(resolve, interval));
    }
  } finally {
    client.close();
  }
}

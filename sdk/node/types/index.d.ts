/**
 * TypeScript type definitions for syslenz4node.
 */

// ---------------------------------------------------------------------------
// FieldValue variants (tagged enum, serialized as e.g. { "Bytes": 1024 })
// ---------------------------------------------------------------------------

export interface FieldValueBytes {
  Bytes: number;
}

export interface FieldValueInteger {
  Integer: number;
}

export interface FieldValueFloat {
  Float: number;
}

export interface FieldValueText {
  Text: string;
}

export interface FieldValueDuration {
  Duration: number;
}

export interface FieldValueTable {
  Table: string[][];
}

export type FieldValue =
  | FieldValueBytes
  | FieldValueInteger
  | FieldValueFloat
  | FieldValueText
  | FieldValueDuration
  | FieldValueTable;

export type FieldValueType =
  | "Bytes"
  | "Integer"
  | "Float"
  | "Text"
  | "Duration"
  | "Table"
  | "unknown";

// ---------------------------------------------------------------------------
// Core data structures
// ---------------------------------------------------------------------------

export interface Field {
  name: string;
  value: FieldValue;
  unit: string | null;
  description: string;
}

export interface ProcEntry {
  source: string;
  fields: Field[];
}

export interface Snapshot {
  timestamp: string;
  entries: Record<string, ProcEntry>;
}

// ---------------------------------------------------------------------------
// Field lookup result
// ---------------------------------------------------------------------------

export interface FieldResult {
  entry: string;
  field: Field;
  value: number | string | string[][];
  type: FieldValueType;
  display: string;
}

// ---------------------------------------------------------------------------
// Unwrapped field value
// ---------------------------------------------------------------------------

export interface UnwrappedFieldValue {
  type: FieldValueType;
  value: number | string | string[][] | null;
}

// ---------------------------------------------------------------------------
// SyslenzClient
// ---------------------------------------------------------------------------

export interface SyslenzClientOptions {
  host?: string;
  port?: number;
  timeout?: number;
}

export class SyslenzClient {
  constructor(options?: SyslenzClientOptions);

  /** Connect to the syslenz TCP server. */
  connect(): Promise<void>;

  /** Fetch a full snapshot from the server. */
  getSnapshot(): Promise<Snapshot>;

  /**
   * Fetch a snapshot and extract a single field by name.
   * Returns null if the field is not found.
   */
  getField(fieldName: string): Promise<FieldResult | null>;

  /** Close the connection. Safe to call even if not connected. */
  close(): void;

  /** Whether the client is currently connected. */
  readonly isConnected: boolean;

  /** The address this client connects to (host:port). */
  readonly address: string;
}

// ---------------------------------------------------------------------------
// Convenience functions
// ---------------------------------------------------------------------------

/**
 * Fetch a single snapshot without managing a client instance.
 *
 * @param addr - Address in "host:port" format (default: "localhost:9100")
 * @param timeout - Timeout in milliseconds (default: 10000)
 */
export function fetchSnapshot(
  addr?: string,
  timeout?: number
): Promise<Snapshot>;

export interface StreamOptions extends SyslenzClientOptions {
  /** Polling interval in milliseconds (default: 2000). */
  interval?: number;
  /** Stop after this many consecutive failures (default: 5). */
  maxConsecutiveFailures?: number;
}

/**
 * Poll snapshots at a fixed interval. Yields an async iterable of Snapshots.
 */
export function streamSnapshots(
  options?: StreamOptions
): AsyncGenerator<Snapshot, void, unknown>;

// ---------------------------------------------------------------------------
// Model utilities
// ---------------------------------------------------------------------------

/** Extract the raw value and type name from a FieldValue. */
export function unwrapFieldValue(fieldValue: FieldValue): UnwrappedFieldValue;

/** Format a FieldValue for human-readable display. */
export function formatFieldValue(fieldValue: FieldValue): string;

/** Format bytes into a human-readable string (e.g. "1.5 GiB"). */
export function formatBytes(bytes: number): string;

/** Format seconds into a human-readable duration string. */
export function formatDuration(secs: number): string;

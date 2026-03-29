/**
 * Data models for syslenz snapshot JSON.
 *
 * These classes mirror the Rust types:
 *   Snapshot { timestamp, entries: Map<string, ProcEntry> }
 *   ProcEntry { source, fields: Field[] }
 *   Field { name, value: FieldValue, unit, description }
 *   FieldValue = { Bytes: number } | { Integer: number } | { Float: number }
 *               | { Text: string } | { Duration: number } | { Table: string[][] }
 */

/**
 * Extract the raw value from a FieldValue object.
 *
 * FieldValue is a tagged enum serialized as `{ "Bytes": 1024 }` etc.
 * This function returns the inner value regardless of variant.
 *
 * @param {object} fieldValue - The FieldValue object
 * @returns {{ type: string, value: * }}
 */
export function unwrapFieldValue(fieldValue) {
  if (fieldValue == null) return { type: "unknown", value: null };

  for (const type of ["Bytes", "Integer", "Float", "Text", "Duration", "Table"]) {
    if (type in fieldValue) {
      return { type, value: fieldValue[type] };
    }
  }
  return { type: "unknown", value: fieldValue };
}

/**
 * Format a FieldValue for display (mirrors Rust FieldValue::display).
 *
 * @param {object} fieldValue - The FieldValue object
 * @returns {string}
 */
export function formatFieldValue(fieldValue) {
  const { type, value } = unwrapFieldValue(fieldValue);

  switch (type) {
    case "Bytes":
      return formatBytes(value);
    case "Integer":
      return String(value);
    case "Float":
      return value.toFixed(2);
    case "Text":
      return value;
    case "Duration":
      return formatDuration(value);
    case "Table":
      return `[${value.length} rows]`;
    default:
      return String(value);
  }
}

/**
 * Format bytes into a human-readable string (e.g. "1.5 GiB").
 *
 * @param {number} bytes
 * @returns {string}
 */
export function formatBytes(bytes) {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KiB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MiB`;
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GiB`;
}

/**
 * Format seconds into a human-readable duration string.
 *
 * @param {number} secs
 * @returns {string}
 */
export function formatDuration(secs) {
  if (secs < 60) return `${secs.toFixed(1)}s`;
  if (secs < 3600) return `${Math.floor(secs / 60)}m ${Math.floor(secs % 60)}s`;
  const hours = Math.floor(secs / 3600);
  const mins = Math.floor((secs % 3600) / 60);
  return `${hours}h ${mins}m`;
}

/**
 * syslenz4node — Node.js client for syslenz TCP servers.
 *
 * @module syslenz4node
 *
 * @example
 * import { SyslenzClient, fetchSnapshot } from "syslenz4node";
 *
 * // One-shot
 * const snapshot = await fetchSnapshot("localhost:9100");
 *
 * // Persistent client
 * const client = new SyslenzClient({ host: "localhost", port: 9100 });
 * const snap = await client.getSnapshot();
 * const cpu = await client.getField("cpu_total");
 * client.close();
 */

export { SyslenzClient, fetchSnapshot, streamSnapshots } from "./client.js";
export {
  unwrapFieldValue,
  formatFieldValue,
  formatBytes,
  formatDuration,
} from "./models.js";

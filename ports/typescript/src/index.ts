/**
 * naipes — Spanish card games on the command line.
 *
 * Public programmatic API. For CLI usage, install globally and run `naipes`.
 */

export * from "./core/index.js";
export * as brisca from "./games/brisca/index.js";
export { AVAILABLE_GAMES } from "./games/index.js";
export type { AvailableGame } from "./games/index.js";
export { replay, type ReplayResult } from "./cli/loop.js";

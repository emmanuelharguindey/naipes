#!/usr/bin/env node
import { main } from "./root.js";

main().then(
  (code) => process.exit(code),
  (err) => {
    if (err instanceof Error && err.message === "readline was closed") {
      process.exit(130);
    }
    process.stderr.write(`naipes: ${err instanceof Error ? err.message : String(err)}\n`);
    process.exit(1);
  },
);

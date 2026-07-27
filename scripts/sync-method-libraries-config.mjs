#!/usr/bin/env node
// Compatibility wrapper — prefer scripts/sync-kpar-libraries-config.mjs
import { spawnSync } from "node:child_process";
import path from "node:path";
import process from "node:process";

const script = path.join(process.cwd(), "scripts", "sync-kpar-libraries-config.mjs");
const result = spawnSync(process.execPath, [script, ...process.argv.slice(2)], {
  stdio: "inherit",
});
process.exit(result.status ?? 1);

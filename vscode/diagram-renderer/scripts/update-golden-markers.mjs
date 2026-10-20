#!/usr/bin/env node
/** Regenerate shared structural-marker fixtures through the owning native renderer.
 * Run from any directory: node vscode/diagram-renderer/scripts/update-golden-markers.mjs
 * The webview mounts native SVG; it must not maintain a second drawing implementation.
 */
import { spawnSync } from "node:child_process";
import { dirname, resolve, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const root = resolve(here, "../../..");
const fixtures = join(root, "crates/diagram_draw/tests/fixtures");
const golden = resolve(here, "../src/test-support/golden-parity");
for (const [fixture, summary] of [
  ["general-view", "general-view"],
  ["interconnection-view-fidelity", "interconnection-view"],
]) {
  const result = spawnSync("cargo", ["run", "--offline", "-p", "diagram_draw", "--example", "render_draw_input", "--",
    join(fixtures, `${fixture}.laid-out.json`), join(fixtures, `${fixture}.rendered.svg`),
    join(golden, `${summary}.markers.json`)], { cwd: root, stdio: "inherit" });
  if (result.error) throw result.error;
  if (result.status !== 0) process.exit(result.status ?? 1);
  console.log(`updated ${summary}.markers.json`);
}

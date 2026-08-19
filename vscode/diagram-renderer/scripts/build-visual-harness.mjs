#!/usr/bin/env node
/**
 * Builds the deterministic Chrome visual-review harness into `visual-out/` (git-ignored).
 *
 * Corpus sources:
 *  - every checked-in repository diagram product (`tests/snapshots/generation/diagram_*.md`,
 *    exact `## diagram.json` section), so all seven view kinds are covered for regressions;
 *  - the authored node-chrome stress cases in `visual/synthetic-cases.ts`.
 *
 * Output is a byte-reproducible function of those checked-in inputs: no clocks, no network, no
 * absolute paths in the emitted bundle.
 *
 *   node scripts/build-visual-harness.mjs
 *   npx http-server visual-out   (or any static server)
 */
import { build } from "esbuild";
import { mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const rendererDir = resolve(here, "..");
const repoRoot = resolve(rendererDir, "..", "..");
const snapshotDir = join(repoRoot, "tests", "snapshots", "generation");
const outDir = join(rendererDir, "visual-out");

function productFromSnapshot(file) {
  const text = readFileSync(join(snapshotDir, file), "utf8");
  const generated = text.split("# GENERATED\n", 2)[1];
  const match = generated?.match(/## diagram\.json\n~~~json\n([\s\S]*?)\n~~~/);
  if (!match) return null;
  const product = JSON.parse(match[1]);
  if (product.schemaVersion !== 3) {
    throw new Error(`${file}: expected schema-v3 diagram product, saw ${product.schemaVersion}`);
  }
  return product;
}

function viewKindOf(file) {
  const text = readFileSync(join(snapshotDir, file), "utf8");
  return text.match(/^viewKind=(.+)$/m)?.[1] ?? "unknown-view";
}

function collectProductCases() {
  const files = readdirSync(snapshotDir)
    .filter((name) => name.startsWith("diagram_") && name.endsWith(".md"))
    .sort();
  const cases = [];
  for (const file of files) {
    const product = productFromSnapshot(file);
    if (!product) continue;
    const id = `product:${file.replace(/^diagram_/, "").replace(/\.md$/, "")}`;
    cases.push({ id, title: `${file} (${viewKindOf(file)})`, view: viewKindOf(file), product });
  }
  return cases;
}

const HTML = `<!doctype html>
<meta charset="utf-8">
<title>spec42 diagram visual harness</title>
<style>
  :root { color-scheme: light; --harness-bg: #ffffff; --harness-fg: #111827; --harness-muted: #6b7280; --harness-line: #e5e7eb; }
  :root[data-harness-theme="dark"] { color-scheme: dark; --harness-bg: #1e1e1e; --harness-fg: #e5e5e5; --harness-muted: #a3a3a3; --harness-line: #3a3a3a; }
  html, body { margin: 0; padding: 0; height: 100%; background: var(--harness-bg); color: var(--harness-fg); }
  body { display: flex; flex-direction: column; font: 12px/1.4 ui-sans-serif, system-ui, -apple-system, "Segoe UI", sans-serif; }
  #caption { flex: 0 0 auto; padding: 6px 10px; border-bottom: 1px solid var(--harness-line); color: var(--harness-muted); font-size: 11px; }
  #diagram { flex: 1 1 auto; min-height: 0; position: relative; }
  #diagram svg { display: block; width: 100%; height: 100%; }
  .case-index { columns: 3; padding: 12px 24px; }
  .incomplete { padding: 24px; color: var(--harness-muted); font-family: ui-monospace, SFMono-Regular, monospace; }
  .case-index a { color: inherit; }
</style>
<div id="caption"></div>
<div id="diagram"></div>
<script src="harness.js"></script>
`;

async function main() {
  rmSync(outDir, { recursive: true, force: true });
  mkdirSync(outDir, { recursive: true });
  const productCases = collectProductCases();
  await build({
    entryPoints: [join(rendererDir, "visual", "harness.ts")],
    outfile: join(outDir, "harness.js"),
    bundle: true,
    platform: "browser",
    format: "iife",
    target: "es2020",
    sourcemap: false,
    define: { __SPEC42_PRODUCT_CASES__: JSON.stringify(productCases) },
    logLevel: "info",
  });
  writeFileSync(join(outDir, "index.html"), HTML);
  writeFileSync(join(outDir, "harness.html"), HTML);
  writeFileSync(
    join(outDir, "cases.json"),
    `${JSON.stringify(productCases.map((entry) => ({ id: entry.id, view: entry.view })), null, 2)}\n`,
  );
  console.log(`visual harness written to visual-out/ (${productCases.length} product cases)`);
}

await main();

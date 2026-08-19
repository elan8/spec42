#!/usr/bin/env node
/**
 * Regenerates the CLI-vs-webview golden structural-marker fixtures under
 * `src/test-support/golden-parity/`.
 *
 * The fixtures are derived artifacts of the renderer's own output, so they are rebuilt through
 * this script rather than hand-edited. Run it only when a node-chrome or marker change is
 * intentional, then review the diff and re-run `npx vitest run`.
 *
 *   node scripts/update-golden-markers.mjs
 */
import { build } from "esbuild";
import { JSDOM } from "jsdom";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const rendererDir = resolve(here, "..");
const goldenDir = join(rendererDir, "src", "test-support", "golden-parity");

const ENTRY = `
import { prepareViewData } from "../src/prepare";
import { renderVisualization } from "../src/renderer";
import { generalViewGoldenPayload, interconnectionViewGoldenPayload } from "../src/test-support/golden-parity-payloads";
import { summarizeSvgMarkers } from "../src/test-support/svg-markers";

export async function summarize(payload) {
  const target = document.createElement("div");
  Object.defineProperty(target, "clientWidth", { value: 1280, configurable: true });
  Object.defineProperty(target, "clientHeight", { value: 900, configurable: true });
  const prepared = prepareViewData(payload);
  const controller = await renderVisualization(target, prepared, { theme: { colorScheme: "light" } });
  const svg = controller.exportSvg();
  controller.destroy();
  return summarizeSvgMarkers(svg);
}

export const payloads = { generalViewGoldenPayload, interconnectionViewGoldenPayload };
`;

const workDir = mkdtempSync(join(tmpdir(), "spec42-golden-"));
try {
  const entryPath = join(rendererDir, "scripts", ".golden-entry.generated.mjs");
  writeFileSync(entryPath, ENTRY);
  const outfile = join(workDir, "golden-entry.mjs");
  await build({
    entryPoints: [entryPath],
    outfile,
    bundle: true,
    platform: "browser",
    format: "esm",
    target: "es2020",
    logLevel: "error",
  });
  rmSync(entryPath, { force: true });

  const dom = new JSDOM("<!doctype html><html><body></body></html>", { pretendToBeVisual: true });
  globalThis.window = dom.window;
  globalThis.document = dom.window.document;
  Object.defineProperty(globalThis, "navigator", { value: dom.window.navigator, configurable: true });
  globalThis.SVGElement = dom.window.SVGElement;
  globalThis.XMLSerializer = dom.window.XMLSerializer;
  globalThis.MouseEvent = dom.window.MouseEvent;
  globalThis.Node = dom.window.Node;

  const mod = await import(pathToFileURL(outfile).href);
  const targets = [
    ["general-view.markers.json", mod.payloads.generalViewGoldenPayload],
    ["interconnection-view.markers.json", mod.payloads.interconnectionViewGoldenPayload],
  ];
  for (const [file, payload] of targets) {
    const summary = await mod.summarize(payload);
    // Key-sorted so the committed fixture diff is reviewable and independent of paint order.
    const sorted = {
      classCounts: Object.fromEntries(
        Object.entries(summary.classCounts).sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0)),
      ),
      markerIds: summary.markerIds,
    };
    writeFileSync(join(goldenDir, file), `${JSON.stringify(sorted, null, 2)}\n`);
    console.log(`updated ${file}`);
  }
} finally {
  rmSync(workDir, { recursive: true, force: true });
}
process.exit(0);

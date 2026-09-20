// @vitest-environment jsdom
import { readFileSync, readdirSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { parseDiagramProduct } from "../../src/diagram/diagramViewerCore";
import { prepareViewData } from "./prepare";
import { isNativeDiagramView, renderVisualization } from "./renderer";
import type { PreparedView } from "./prepare/types";

/**
 * Catalog views still draw in this package. Native views are covered by `diagram_draw` goldens
 * and `spec42/draw` integration tests — they have no client drawing fallback here.
 */
const SNAPSHOT_DIR = resolve(process.cwd(), "../../tests/snapshots/generation");

function productFiles(): string[] {
  return readdirSync(SNAPSHOT_DIR)
    .filter((name) => name.startsWith("diagram_") && name.endsWith(".md"))
    .sort();
}

function preparedFromSnapshot(file: string): PreparedView {
  const text = readFileSync(resolve(SNAPSHOT_DIR, file), "utf8");
  const generated = text.split("# GENERATED\n", 2)[1];
  const match = generated?.match(/## diagram\.json\n~~~json\n([\s\S]*?)\n~~~/);
  if (!match) throw new Error(`${file} has no exact diagram.json product`);
  return prepareViewData(parseDiagramProduct(match[1]) as unknown as Record<string, unknown>);
}

function host(width: number, height: number): HTMLElement {
  const target = document.createElement("div");
  Object.defineProperty(target, "clientWidth", { value: width, configurable: true });
  Object.defineProperty(target, "clientHeight", { value: height, configurable: true });
  return target;
}

describe("visual review corpus", () => {
  it.each(productFiles())("renders catalog views from %s", async (file) => {
    const prepared = preparedFromSnapshot(file);
    if (isNativeDiagramView(prepared.view)) return;
    const target = host(1280, 800);
    const controller = await renderVisualization(target, prepared, { theme: { colorScheme: "light" } });
    expect(target.querySelector("svg")).toBeTruthy();
    expect(controller.exportSvg()).toContain("<svg");
    controller.destroy();
  }, 30_000);
});

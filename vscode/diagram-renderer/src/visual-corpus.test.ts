// @vitest-environment jsdom
import { readFileSync, readdirSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { parseDiagramProduct } from "../../src/diagram/diagramViewerCore";
import { prepareViewData } from "./prepare";
import { renderVisualization } from "./renderer";
import { nodeBodyChromeStyle, notationRoleFromAttributes, resolveNodeChrome } from "./node-notation";
import { DISCLOSURE_TARGET_SIZE } from "./sysml-node-builder";
import { SYNTHETIC_CASES } from "../visual/synthetic-cases";
import type { PreparedView } from "./prepare/types";

/**
 * Deterministic counterpart of the Chrome visual-review harness (`visual/`, built by
 * `scripts/build-visual-harness.mjs`): the same corpus -- every checked-in repository diagram
 * product plus the authored node-chrome stress cases -- rendered through the same code path, with
 * the geometry invariants the screenshots are reviewed for asserted mechanically.
 */
const SNAPSHOT_DIR = resolve(process.cwd(), "../../tests/snapshots/generation");

const VIEWPORTS: Array<{ name: string; width: number; height: number }> = [
  { name: "narrow", width: 720, height: 900 },
  { name: "normal", width: 1280, height: 800 },
  { name: "wide", width: 1920, height: 1080 },
];

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
  // Schema-v3 validation stays on the product path the webview uses.
  return prepareViewData(parseDiagramProduct(match[1]) as unknown as Record<string, unknown>);
}

function host(width: number, height: number): HTMLElement {
  const target = document.createElement("div");
  Object.defineProperty(target, "clientWidth", { value: width, configurable: true });
  Object.defineProperty(target, "clientHeight", { value: height, configurable: true });
  return target;
}

function pathPoints(d: string): Array<{ x: number; y: number }> {
  const points: Array<{ x: number; y: number }> = [];
  let cursor = { x: 0, y: 0 };
  for (const match of d.matchAll(/([MALHV])([^MALHVZ]*)/gi)) {
    const command = match[1].toUpperCase();
    const numbers = (match[2].match(/-?\d+(?:\.\d+)?/g) ?? []).map(Number);
    if (command === "M" || command === "L") cursor = { x: numbers[0], y: numbers[1] };
    else if (command === "H") cursor = { x: numbers[0], y: cursor.y };
    else if (command === "V") cursor = { x: cursor.x, y: numbers[0] };
    else if (command === "A") cursor = { x: numbers[numbers.length - 2], y: numbers[numbers.length - 1] };
    points.push({ ...cursor });
  }
  return points;
}

/** Invariants every reviewed screenshot is expected to show, asserted on the drawn geometry. */
function assertNodeChromeInvariants(target: HTMLElement, label: string): void {
  const svg = target.querySelector("svg")!;
  expect(svg.outerHTML, `${label}: no non-finite coordinates`).not.toContain("NaN");

  for (const node of Array.from(target.querySelectorAll<SVGGElement>(".general-node"))) {
    const bounds = (node.getAttribute("data-bounds") ?? "").split(",").map(Number);
    const [, , width, height] = bounds;
    if (!Number.isFinite(width) || !Number.isFinite(height)) continue;
    const attributes = { notationRole: node.classList.contains("viz-node--definition")
      ? "definition"
      : node.classList.contains("viz-node--reference")
        ? "reference-usage"
        : node.classList.contains("viz-node--usage")
          ? "usage"
          : "unsupported" };
    const body = nodeBodyChromeStyle(resolveNodeChrome(notationRoleFromAttributes(attributes)), {
      generalView: true,
      selected: node.classList.contains("is-selected"),
    });
    const inset = body.strokeWidthPx / 2;
    const id = node.getAttribute("data-node-id");

    const background = node.querySelector(".sysml-node-bg");
    expect(Number(background?.getAttribute("width")), `${label} ${id}: body width`).toBe(width);
    expect(Number(background?.getAttribute("height")), `${label} ${id}: body height`).toBe(height);

    const fill = node.querySelector(".sysml-header-compartment");
    if (fill) {
      expect(fill.tagName.toLowerCase(), `${label} ${id}: header fill is a clipped path`).toBe("path");
      for (const point of pathPoints(fill.getAttribute("d") ?? "")) {
        expect(point.x, `${label} ${id}: header fill left of border`).toBeGreaterThanOrEqual(inset - 1e-6);
        expect(point.x, `${label} ${id}: header fill right of border`).toBeLessThanOrEqual(width - inset + 1e-6);
        expect(point.y, `${label} ${id}: header fill above border`).toBeGreaterThanOrEqual(inset - 1e-6);
        expect(point.y, `${label} ${id}: header fill below node`).toBeLessThanOrEqual(height + 1e-6);
      }
    }

    for (const divider of Array.from(node.querySelectorAll(".sysml-compartment-divider"))) {
      expect(Number(divider.getAttribute("x1")), `${label} ${id}: divider start`).toBeGreaterThanOrEqual(inset - 1e-6);
      expect(Number(divider.getAttribute("x2")), `${label} ${id}: divider end`).toBeLessThanOrEqual(width - inset + 1e-6);
      expect(Number(divider.getAttribute("y1")), `${label} ${id}: divider inside node`).toBeLessThanOrEqual(height);
    }

    const control = node.querySelector(".general-node-toggle .sysml-disclosure-target");
    if (control) {
      expect(Number(control.getAttribute("width")), `${label} ${id}: pointer target width`)
        .toBeGreaterThanOrEqual(DISCLOSURE_TARGET_SIZE);
      expect(Number(control.getAttribute("height")), `${label} ${id}: pointer target height`)
        .toBeGreaterThanOrEqual(DISCLOSURE_TARGET_SIZE);
      const badge = node.querySelector(".general-hidden-relationships rect");
      if (badge) {
        const controlRight = Number(control.getAttribute("x")) + Number(control.getAttribute("width"));
        expect(controlRight, `${label} ${id}: control clear of badge`)
          .toBeLessThanOrEqual(Number(badge.getAttribute("x")));
        expect(Number(badge.getAttribute("x")) + Number(badge.getAttribute("width")), `${label} ${id}: badge inside node`)
          .toBeLessThanOrEqual(width - inset + 1e-6);
      }
    }
  }
}

describe("visual review corpus", () => {
  it.each(productFiles())("renders %s without chrome defects", async (file) => {
    const prepared = preparedFromSnapshot(file);
    for (const scheme of ["light", "dark"] as const) {
      const target = host(1280, 800);
      const controller = await renderVisualization(target, prepared, { theme: { colorScheme: scheme } });
      assertNodeChromeInvariants(target, `${file}/${scheme}`);
      expect(controller.exportSvg()).toContain("<svg");
      controller.destroy();
    }
  }, 30_000);

  it.each(SYNTHETIC_CASES.map((entry) => entry.id))(
    "renders the %s stress case at every viewport in both schemes",
    async (id) => {
      const entry = SYNTHETIC_CASES.find((candidate) => candidate.id === id)!;
      for (const viewport of VIEWPORTS) {
        for (const scheme of ["light", "dark"] as const) {
          const target = host(viewport.width, viewport.height);
          const prepared = JSON.parse(JSON.stringify(entry.prepared)) as PreparedView;
          const controller = await renderVisualization(target, prepared, { theme: { colorScheme: scheme } });
          for (const nodeId of entry.expand ?? []) {
            const control = target.querySelector(`[data-node-id="${nodeId}"] .general-node-toggle`);
            control?.dispatchEvent(new MouseEvent("click", { bubbles: true }));
            for (let attempt = 0; attempt < 80; attempt += 1) {
              const after = target.querySelector(`[data-node-id="${nodeId}"] .general-node-toggle`);
              if (after?.getAttribute("aria-expanded") === "true") break;
              await new Promise((resolve) => setTimeout(resolve, 5));
            }
          }
          assertNodeChromeInvariants(target, `${id}/${viewport.name}/${scheme}`);
          controller.destroy();
        }
      }
    },
    30_000,
  );
});

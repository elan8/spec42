import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { prepareViewData } from "../prepare";
import { layoutPrepared } from "../render/layout";
import { exportHeadlessSvg } from "../headless-export";
import type { UnknownRecord } from "../prepare/types";
import { generalViewGoldenPayload } from "./golden-parity-payloads";

/**
 * Regenerable fixtures for the Rust drawing spike (`crates/diagram_draw`, spec42 #176 phase 1):
 *
 * - The already-laid-out General View graph -- the same `LaidOutNode[]`/`LaidOutEdge[]` shape
 *   `drawNodes`/`drawEdges` draw from -- dumped from the real `prepareViewData` + `layoutPrepared`
 *   pipeline, so the Rust drawing port has a production-shaped input rather than a hand-authored
 *   one.
 * - The actual SVG the existing TS/D3 pipeline renders from that same graph (via the identical
 *   `exportHeadlessSvg` call `headless-export.golden-parity.test.ts` uses), so the Rust output can
 *   be checked against real rendered content -- geometry, text, attributes -- not just the
 *   marker/class-count summary `svg-markers.ts` produces.
 *
 * Two payloads are dumped:
 *
 * - `generalViewGoldenPayload` (shared with the TS golden-parity suite and its
 *   `general-view.markers.json` -- left untouched here, not widened, so those tests keep meaning
 *   what they already mean).
 * - `generalViewFidelityPayload` below, a diagram_draw-only payload that exists specifically to
 *   exercise drawing paths the golden payload's two-node/one-edge graph never reaches: a
 *   collapsed inherited-attributes compartment (disclosure chrome, true-vs-shown member count), a
 *   named non-generic edge label (`data-connector-id` on the label text), and a `specializes`
 *   edge (a marker style whose branch also re-sets `stroke-width`, so it separately exercises
 *   `Element`'s set-vs-append attribute semantics). `P::vehicle` additionally carries an
 *   `attributes` compartment so its usage-role node (non-zero corner radius) also gets a header
 *   fill, exercising the corner-radius/stroke-inset math a zero-radius definition node cannot.
 *
 * Regenerate with `UPDATE_DIAGRAM_DRAW_FIXTURES=1 npx vitest run
 * src/test-support/dump-general-view-fixture.test.ts` whenever either payload,
 * `prepareViewData`, `layoutPrepared`, or the drawing pipeline change in a way that affects the
 * General View shape or rendering.
 */
const FIXTURES_DIR = join(dirname(fileURLToPath(import.meta.url)), "../../../../crates/diagram_draw/tests/fixtures");

const basePayload = {
  version: 1,
  workspaceRootUri: "file:///demo",
  modelReady: true,
  viewCandidates: [],
  selectedView: null,
  emptyStateMessage: null,
  packageGroups: null,
  workspaceModel: null,
  ibd: null,
  interconnectionScene: null,
  stats: null,
  projectionHints: null,
  graph: null,
  generalViewGraph: null,
  activityDiagrams: null,
  activityDiagramCandidates: null,
  sequenceDiagrams: null,
  sequenceDiagramCandidates: null,
  stateMachines: null,
  stateMachineCandidates: null,
};

export const generalViewFidelityPayload: UnknownRecord = {
  ...basePayload,
  view: "general-view",
  selectedViewName: "General",
  graph: {
    nodes: [
      { id: "P::Vehicle", name: "Vehicle", type: "part def", attributes: { attributes: ["mass"] } },
      {
        id: "P::vehicle",
        name: "vehicle",
        type: "part",
        attributes: { partType: "Vehicle", attributes: ["speed"] },
      },
      {
        id: "P::Wheel",
        name: "Wheel",
        type: "part def",
        attributes: { generalViewInheritedAttributes: ["radius", "material"] },
      },
    ],
    edges: [
      { id: "typed", source: "P::vehicle", target: "P::Vehicle", type: "typing", name: "typing" },
      { id: "has-wheel", source: "P::vehicle", target: "P::Wheel", type: "specializes", name: "has-wheel" },
    ],
  },
};

const FIXTURES: Array<{ name: string; payload: UnknownRecord }> = [
  { name: "general-view", payload: generalViewGoldenPayload },
  { name: "general-view-fidelity", payload: generalViewFidelityPayload },
];

describe("diagram_draw fixture dump", () => {
  it("writes the laid-out General View graphs and their rendered SVGs when UPDATE_DIAGRAM_DRAW_FIXTURES=1", async () => {
    if (process.env.UPDATE_DIAGRAM_DRAW_FIXTURES !== "1") {
      expect(true).toBe(true);
      return;
    }
    mkdirSync(FIXTURES_DIR, { recursive: true });
    for (const { name, payload } of FIXTURES) {
      const prepared = prepareViewData(payload);
      const layout = await layoutPrepared(prepared);
      writeFileSync(
        join(FIXTURES_DIR, `${name}.laid-out.json`),
        `${JSON.stringify(
          { title: prepared.title, view: prepared.view, meta: prepared.meta ?? null, nodes: layout.nodes, edges: layout.edges },
          null,
          2,
        )}\n`,
        "utf8",
      );
      const svg = await exportHeadlessSvg(payload, { width: 1280, height: 900, colorScheme: "light" });
      writeFileSync(join(FIXTURES_DIR, `${name}.rendered.svg`), `${svg}\n`, "utf8");
    }
  });
});

import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { prepareViewData } from "../prepare";
import { exportHeadlessSvg } from "../headless-export";
import { layoutBehaviorGraph } from "../views/behavior-common";
import type { UnknownRecord } from "../prepare/types";

/**
 * Regenerable fixture for the Rust drawing port of action-flow-view (spec42 #176 phase 2). Same
 * two-part shape as `dump-state-transition-view-fixture.test.ts`: the `PreparedView` plus the
 * `BehaviorLayoutResult` `layoutBehaviorGraph` produces for it (`Map`s converted to plain objects).
 *
 * The payload is sized to exercise: initial/final nodes, a decision (two outgoing edges), a fork
 * with two outgoing edges converging at a join, a plain action with input/output parameter
 * badges, a `perform` action (dashed border + stereotype), two swim lanes, and three edge-kind
 * branches (succession via `guard: "first"`, streaming via `guard: "flow"`, conditional via a
 * `condition` string).
 *
 * Regenerate with `UPDATE_DIAGRAM_DRAW_FIXTURES=1 npx vitest run
 * src/test-support/dump-action-flow-view-fixture.test.ts` whenever this payload,
 * `layoutBehaviorGraph`, or `renderActionFlowView` change in a way that affects
 * action-flow-view's drawn shape.
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

export const actionFlowViewGoldenPayload: UnknownRecord = {
  ...basePayload,
  view: "action-flow-view",
  selectedViewName: "Fulfillment",
  activityDiagrams: [
    {
      name: "Fulfillment",
      nodes: [
        { id: "start", kind: "initial" },
        { id: "pack", kind: "action", name: "Pack Order", attributes: { inputs: ["order"], outputs: ["package"], swimLane: "Warehouse" } },
        { id: "decide", kind: "decision" },
        { id: "ship", kind: "perform", name: "Ship Package", attributes: { swimLane: "Logistics" } },
        { id: "split", kind: "fork" },
        { id: "notify", kind: "action", name: "Notify Customer", attributes: { swimLane: "Logistics" } },
        { id: "archive", kind: "action", name: "Archive Order", attributes: { swimLane: "Warehouse" } },
        { id: "join", kind: "join" },
        { id: "end", kind: "final" },
      ],
      flows: [
        { from: "start", to: "pack", guard: "first" },
        { from: "pack", to: "decide" },
        { from: "decide", to: "ship", condition: "approved" },
        { from: "decide", to: "end", condition: "rejected" },
        { from: "ship", to: "split", guard: "flow" },
        { from: "split", to: "notify" },
        { from: "split", to: "archive" },
        { from: "notify", to: "join" },
        { from: "archive", to: "join" },
        { from: "join", to: "end" },
      ],
    },
  ],
};

function mapToObject<V>(map: Map<string, V>): Record<string, V> {
  return Object.fromEntries(map.entries());
}

describe("diagram_draw action-flow fixture dump", () => {
  it("writes the prepared view, its behavior layout, and its rendered SVG when UPDATE_DIAGRAM_DRAW_FIXTURES=1", async () => {
    if (process.env.UPDATE_DIAGRAM_DRAW_FIXTURES !== "1") {
      expect(true).toBe(true);
      return;
    }
    mkdirSync(FIXTURES_DIR, { recursive: true });
    const prepared = prepareViewData(actionFlowViewGoldenPayload);
    const horizontal = String(prepared.meta?.layoutDirection ?? "").toLowerCase() === "horizontal";
    const layout = await layoutBehaviorGraph(prepared, { horizontal, mode: "action" });
    writeFileSync(
      join(FIXTURES_DIR, "action-flow-view.prepared.json"),
      `${JSON.stringify(
        {
          prepared: { title: prepared.title, view: prepared.view, nodes: prepared.nodes, edges: prepared.edges, meta: prepared.meta ?? null },
          behaviorLayout: {
            positions: mapToObject(layout.positions),
            edgeSectionsById: mapToObject(layout.edgeSectionsById),
            edgeLabelsById: mapToObject(layout.edgeLabelsById),
          },
        },
        null,
        2,
      )}\n`,
      "utf8",
    );
    const svg = await exportHeadlessSvg(actionFlowViewGoldenPayload, { width: 1280, height: 900, colorScheme: "light" });
    writeFileSync(join(FIXTURES_DIR, "action-flow-view.rendered.svg"), `${svg}\n`, "utf8");
  });
});

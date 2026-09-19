import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { prepareViewData } from "../prepare";
import { exportHeadlessSvg } from "../headless-export";
import { layoutBehaviorGraph } from "../views/behavior-common";
import type { UnknownRecord } from "../prepare/types";

/**
 * Regenerable fixture for the Rust drawing port of state-transition-view (spec42 #176 phase 2).
 * Unlike sequence-view, this view has a real ELK layout step
 * (`layoutBehaviorGraph(prepared, {horizontal, mode: "state"})`), so the dump has two parts: the
 * `PreparedView` itself, and the `BehaviorLayoutResult` that layout call produces (its `Map`
 * fields converted to plain objects, since `JSON.stringify` drops real `Map`s silently).
 *
 * The payload is sized to exercise: an initial state, a plain state with entry/do/exit actions, a
 * composite state with nested regions, a final state, a self-loop transition (`source === target`),
 * and labeled transitions.
 *
 * Regenerate with `UPDATE_DIAGRAM_DRAW_FIXTURES=1 npx vitest run
 * src/test-support/dump-state-transition-view-fixture.test.ts` whenever this payload,
 * `layoutBehaviorGraph`, or `renderStateTransitionView` change in a way that affects
 * state-transition-view's drawn shape.
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

export const stateTransitionViewGoldenPayload: UnknownRecord = {
  ...basePayload,
  view: "state-transition-view",
  selectedViewName: "Order",
  stateDiagrams: [
    {
      name: "Order",
      states: [
        { id: "init", name: "Initial", type: "initial" },
        { id: "processing", name: "Processing", type: "state", entry: "begin", do: "process", exit: "finish" },
        { id: "shipped", name: "Shipped", type: "composite" },
        { id: "done", name: "Done", type: "final" },
      ],
      regions: [
        { id: "r1", name: "packing", parentId: "shipped" },
        { id: "r2", name: "labeling", parentId: "shipped" },
      ],
      transitions: [
        { source: "init", target: "processing", label: "start" },
        { source: "processing", target: "processing", guard: "timeout" },
        { source: "processing", target: "shipped", effect: "notify" },
        { source: "shipped", target: "done", accept: "delivered", send: "archive" },
      ],
    },
  ],
};

function mapToObject<V>(map: Map<string, V>): Record<string, V> {
  return Object.fromEntries(map.entries());
}

describe("diagram_draw state-transition fixture dump", () => {
  it("writes the prepared view, its behavior layout, and its rendered SVG when UPDATE_DIAGRAM_DRAW_FIXTURES=1", async () => {
    if (process.env.UPDATE_DIAGRAM_DRAW_FIXTURES !== "1") {
      expect(true).toBe(true);
      return;
    }
    mkdirSync(FIXTURES_DIR, { recursive: true });
    const prepared = prepareViewData(stateTransitionViewGoldenPayload);
    const layoutMode = String(prepared.meta?.layoutDirection ?? "horizontal").toLowerCase();
    const horizontal = layoutMode !== "vertical" && layoutMode !== "force";
    const layout = await layoutBehaviorGraph(prepared, { horizontal, mode: "state" });
    writeFileSync(
      join(FIXTURES_DIR, "state-transition-view.prepared.json"),
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
    const svg = await exportHeadlessSvg(stateTransitionViewGoldenPayload, { width: 1280, height: 900, colorScheme: "light" });
    writeFileSync(join(FIXTURES_DIR, "state-transition-view.rendered.svg"), `${svg}\n`, "utf8");
  });
});

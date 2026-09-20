import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { prepareViewData } from "../prepare";
import { exportHeadlessSvg } from "../headless-export";
import type { UnknownRecord } from "../prepare/types";

/**
 * Regenerable fixture for the Rust drawing port of sequence-view (spec42 #176 phase 2), mirroring
 * `dump-general-view-fixture.test.ts`'s pattern: sequence-view has no ELK/layout dependency at
 * all, so unlike action-flow/state-transition there is no separate layout-result fixture -- the
 * dumped `PreparedView` (with `meta.sequenceDiagram` carrying `{lifelines, messages, activations,
 * fragments}`) is the entire drawing input.
 *
 * The payload below is sized to exercise every branch `renderSequenceView` has: a normal
 * cross-lifeline message, a self-message (cubic-bezier path), a dashed "reply" (return) message,
 * an activation bar spanning multiple messages, and a fragment with a guard.
 *
 * Regenerate with `UPDATE_DIAGRAM_DRAW_FIXTURES=1 npx vitest run
 * src/test-support/dump-sequence-view-fixture.test.ts` whenever this payload or
 * `renderSequenceView` change in a way that affects sequence-view's drawn shape.
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

export const sequenceViewGoldenPayload: UnknownRecord = {
  ...basePayload,
  view: "sequence-view",
  selectedViewName: "Checkout",
  sequenceDiagrams: [
    {
      name: "Checkout",
      nodes: [
        { id: "buyer", name: "Buyer" },
        { id: "store", name: "Store" },
      ],
      lifelines: [
        { id: "buyer", name: "Buyer" },
        { id: "store", name: "Store" },
      ],
      messages: [
        { id: "m1", source: "buyer", target: "store", name: "placeOrder", order: 1 },
        { id: "m2", source: "store", target: "store", name: "validate", order: 2 },
        { id: "m3", source: "store", target: "buyer", name: "confirmed", kind: "reply", order: 3 },
      ],
      activations: [{ on_lifeline: "store", start_message: "m1", finish_message: "m3" }],
      fragments: [{ kind: "alt", operands: [{ guard: "success", message_ids: ["m1"] }] }],
    },
  ],
};

describe("diagram_draw sequence fixture dump", () => {
  it("writes the prepared sequence view and its rendered SVG when UPDATE_DIAGRAM_DRAW_FIXTURES=1", async () => {
    if (process.env.UPDATE_DIAGRAM_DRAW_FIXTURES !== "1") {
      expect(true).toBe(true);
      return;
    }
    mkdirSync(FIXTURES_DIR, { recursive: true });
    const prepared = prepareViewData(sequenceViewGoldenPayload);
    writeFileSync(
      join(FIXTURES_DIR, "sequence-view.prepared.json"),
      `${JSON.stringify({ title: prepared.title, view: prepared.view, nodes: prepared.nodes, edges: prepared.edges, meta: prepared.meta ?? null }, null, 2)}\n`,
      "utf8",
    );
    const svg = await exportHeadlessSvg(sequenceViewGoldenPayload, { width: 1280, height: 900, colorScheme: "light" });
    writeFileSync(join(FIXTURES_DIR, "sequence-view.rendered.svg"), `${svg}\n`, "utf8");
  });
});

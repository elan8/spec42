import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { prepareViewData } from "../prepare";
import { layoutPrepared } from "../render/layout";
import { exportHeadlessSvg } from "../headless-export";
import type { UnknownRecord } from "../prepare/types";

/**
 * Regenerable fixture for the Rust drawing port of interconnection-view (spec42 #176 phase 2).
 * Same `layoutPrepared` entry point General View uses (it dispatches to
 * `layoutInterconnectionPrepared` for this view), so the dump shape mirrors
 * `dump-general-view-fixture.test.ts` exactly, plus `layout.interconnectionLayout` (port anchors/
 * draw order, route points, containers) which that dumper doesn't need.
 *
 * The existing `interconnectionViewGoldenPayload` in `golden-parity-payloads.ts` (2 nodes, 1
 * unlabeled `flow` edge, no containers, no ports on both sides) is too thin to exercise this
 * view's distinct branches, so this is its own richer payload: a synthetic package container
 * grouping two of three nodes, ports on both the west and east side, an `interface`-kind edge
 * with a real (non-generic) label, and a `flow`-kind edge with no label.
 *
 * Regenerate with `UPDATE_DIAGRAM_DRAW_FIXTURES=1 npx vitest run
 * src/test-support/dump-interconnection-view-fixture.test.ts` whenever this payload or the
 * interconnection drawing pipeline change in a way that affects interconnection-view's drawn shape.
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

export const interconnectionViewFidelityPayload: UnknownRecord = {
  ...basePayload,
  view: "interconnection-view",
  selectedViewName: "Powertrain",
  interconnectionScene: {
    schemaVersion: 3,
    view: { id: "powertrain", name: "Powertrain", type: "InterconnectionView", rootIds: ["powertrain"] },
    nodes: [
      { id: "controller", name: "controller", kind: "part", qualifiedName: "controller", semanticId: "controller", definitionId: "Controller", typeName: "Controller" },
      { id: "engine", name: "engine", kind: "part", qualifiedName: "engine", semanticId: "engine", definitionId: "Engine", typeName: "Engine" },
      { id: "wheel", name: "wheel", kind: "part", qualifiedName: "wheel", semanticId: "wheel", definitionId: "Wheel", typeName: "Wheel" },
    ],
    ports: [
      { id: "controller.cmdOut", ownerNodeId: "controller", name: "cmdOut", semanticId: "controller.cmdOut", direction: "out", typeName: "Command", sideHint: "east" },
      { id: "engine.cmdIn", ownerNodeId: "engine", name: "cmdIn", semanticId: "engine.cmdIn", direction: "in", typeName: "Command", sideHint: "west" },
      { id: "engine.powerOut", ownerNodeId: "engine", name: "powerOut", semanticId: "engine.powerOut", direction: "out", typeName: "Torque", sideHint: "east" },
      { id: "wheel.powerIn", ownerNodeId: "wheel", name: "powerIn", semanticId: "wheel.powerIn", direction: "in", typeName: "Torque", sideHint: "west" },
    ],
    edges: [
      {
        id: "e-control",
        kind: "interface",
        sourcePortId: "controller.cmdOut",
        targetPortId: "engine.cmdIn",
        sourceNodeId: "controller",
        targetNodeId: "engine",
        semanticId: "e-control",
        label: "ControlBus",
      },
      {
        id: "e-power",
        kind: "flow",
        sourcePortId: "engine.powerOut",
        targetPortId: "wheel.powerIn",
        sourceNodeId: "engine",
        targetNodeId: "wheel",
        semanticId: "e-power",
      },
    ],
    containers: [{ id: "powertrain", label: "Powertrain", memberNodeIds: ["engine", "wheel"], depth: 1 }],
    diagnostics: [],
  },
};

describe("diagram_draw interconnection fixture dump", () => {
  it("writes the laid-out interconnection graph and its rendered SVG when UPDATE_DIAGRAM_DRAW_FIXTURES=1", async () => {
    if (process.env.UPDATE_DIAGRAM_DRAW_FIXTURES !== "1") {
      expect(true).toBe(true);
      return;
    }
    mkdirSync(FIXTURES_DIR, { recursive: true });
    const prepared = prepareViewData(interconnectionViewFidelityPayload);
    const layout = await layoutPrepared(prepared);
    writeFileSync(
      join(FIXTURES_DIR, "interconnection-view-fidelity.laid-out.json"),
      `${JSON.stringify(
        {
          title: prepared.title,
          view: prepared.view,
          meta: prepared.meta ?? null,
          nodes: layout.nodes,
          edges: layout.edges,
          interconnectionLayout: layout.interconnectionLayout ?? null,
        },
        null,
        2,
      )}\n`,
      "utf8",
    );
    const svg = await exportHeadlessSvg(interconnectionViewFidelityPayload, { width: 1280, height: 900, colorScheme: "light" });
    writeFileSync(join(FIXTURES_DIR, "interconnection-view-fidelity.rendered.svg"), `${svg}\n`, "utf8");
  });
});

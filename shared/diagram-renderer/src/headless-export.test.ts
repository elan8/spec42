import { describe, expect, it } from "vitest";
import { exportHeadlessSvg } from "./headless-export";

const basePayload = {
  version: 1,
  workspaceRootUri: "file:///demo",
  modelReady: true,
  viewCandidates: [],
  selectedView: null,
  selectedViewName: "Headless",
  emptyStateMessage: null,
  packageGroups: null,
  workspaceModel: null,
  ibd: null,
  interconnectionScene: null,
  stats: null,
  projectionHints: null,
};

describe("headless SVG export", () => {
  it("exports General View with shared renderer notation", async () => {
    const svg = await exportHeadlessSvg({
      ...basePayload,
      view: "general-view",
      graph: {
        nodes: [
          { id: "Vehicle", name: "Vehicle", type: "part def", attributes: { attributes: ["mass"] } },
          { id: "vehicle", name: "vehicle", type: "part", attributes: { partType: "Vehicle" } },
        ],
        edges: [{ id: "typed", source: "vehicle", target: "Vehicle", type: "typing", name: "typing" }],
      },
      generalViewGraph: null,
      activityDiagrams: null,
      sequenceDiagrams: null,
      stateMachines: null,
    });

    expect(svg).toContain("<svg");
    expect(svg).toContain("viz-node--definition");
    expect(svg).toContain("viz-node--usage");
    expect(svg).toContain("general-d3-specializes");
    expect(svg).toContain("sysml-header-compartment");
  });

  it("exports Interconnection View with IBD notation", async () => {
    const svg = await exportHeadlessSvg({
      ...basePayload,
      view: "interconnection-view",
      graph: null,
      generalViewGraph: null,
      activityDiagrams: null,
      sequenceDiagrams: null,
      stateMachines: null,
      interconnectionScene: {
        schemaVersion: 2,
        view: { id: "v", name: "Connections", type: "InterconnectionView", rootIds: ["a", "b"] },
        nodes: [
          { id: "a", name: "a", kind: "part", qualifiedName: "a", semanticId: "a", definitionId: "A", typeName: "A" },
          { id: "b", name: "b", kind: "part", qualifiedName: "b", semanticId: "b", definitionId: "B", typeName: "B" },
        ],
        ports: [
          { id: "a.p", ownerNodeId: "a", name: "p", direction: "out", typeName: "Power", sideHint: "east" },
          { id: "b.p", ownerNodeId: "b", name: "p", direction: "in", typeName: "Power", sideHint: "west" },
        ],
        edges: [{ id: "e", sourceNodeId: "a", targetNodeId: "b", sourcePortId: "a.p", targetPortId: "b.p", kind: "flow", label: "flow", semanticId: "e" }],
        containers: [],
        diagnostics: [],
      },
    });

    expect(svg).toContain("ibd-connector");
    expect(svg).toContain("ibd-flow-arrow");
    expect(svg).toContain("port-icon");
    expect(svg).toContain("viz-node--usage");
  });

  it("exports behavior views", async () => {
    const actionSvg = await exportHeadlessSvg({
      ...basePayload,
      view: "action-flow-view",
      graph: null,
      generalViewGraph: null,
      activityDiagrams: [{ id: "a", name: "Action", nodes: [{ id: "start", name: "start", kind: "initial" }, { id: "do", name: "Do", kind: "action" }], edges: [{ id: "e", source: "start", target: "do", label: "then" }] }],
      sequenceDiagrams: null,
      stateMachines: null,
    });
    expect(actionSvg).toContain("activity-action");
    expect(actionSvg).toContain("action-flow-arrow");

    const stateSvg = await exportHeadlessSvg({
      ...basePayload,
      view: "state-transition-view",
      graph: null,
      generalViewGraph: null,
      activityDiagrams: null,
      sequenceDiagrams: null,
      stateMachines: [{ id: "s", name: "State", states: [{ id: "idle", name: "Idle", kind: "state" }, { id: "done", name: "Done", kind: "final" }], transitions: [{ id: "t", source: "idle", target: "done", label: "finish" }] }],
    });
    expect(stateSvg).toContain("state-node");
    expect(stateSvg).toContain("state-transition-arrow");

    const sequenceSvg = await exportHeadlessSvg({
      ...basePayload,
      view: "sequence-view",
      graph: null,
      generalViewGraph: null,
      activityDiagrams: null,
      stateMachines: null,
      sequenceDiagrams: [{ id: "q", name: "Seq", lifelines: [{ id: "a", name: "A" }, { id: "b", name: "B" }], messages: [{ id: "m", source: "a", target: "b", label: "call" }] }],
    });
    expect(sequenceSvg).toContain("sequence-lifeline");
    expect(sequenceSvg).toContain("sequence-message");
  });
});

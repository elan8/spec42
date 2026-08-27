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
        schemaVersion: 3,
        view: { id: "v", name: "Connections", type: "InterconnectionView", rootIds: ["a", "b"] },
        nodes: [
          { id: "a", name: "a", kind: "part", qualifiedName: "a", semanticId: "a", definitionId: "A", typeName: "A" },
          { id: "b", name: "b", kind: "part", qualifiedName: "b", semanticId: "b", definitionId: "B", typeName: "B" },
        ],
        ports: [
          { id: "a.p", semanticId: "Demo::a::p", ownerNodeId: "a", name: "p", direction: "out", multiplicity: "[0..1]", typeName: "Power", sideHint: "east" },
          { id: "b.p", semanticId: "Demo::b::p", ownerNodeId: "b", name: "p", direction: "in", multiplicity: "[1]", typeName: "~Power", sideHint: "west" },
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
    expect(svg).toContain("<title>p");
    expect(svg).toContain("Type: ~Power");
    expect(svg).toContain("Multiplicity: [0..1]");
    expect(svg).toContain("Resolved source: a.p");
    expect(svg).toContain("viz-edge-hit-target");
    expect(svg).not.toContain("sysml-diagram-tooltip");
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

  it("exports Browser View as an indented expandable hierarchy", async () => {
    const svg = await exportHeadlessSvg({
      ...basePayload,
      view: "browser-view",
      selectedViewName: "Structure Browser",
      projectionHints: {
        browserLayout: "hierarchy",
        treeRoots: ["root"],
      },
      graph: null,
      generalViewGraph: {
        nodes: [
          { id: "root", name: "Root", type: "part def", parent_id: "", parentId: "" },
          { id: "child", name: "Child", type: "part", parent_id: "", parentId: "root" },
        ],
        edges: [],
      },
      activityDiagrams: null,
      sequenceDiagrams: null,
      stateMachines: null,
    });

    expect(svg).toContain("browser-toggle");
    expect(svg).toContain('data-node-id="child"');
    expect(svg).not.toContain("provisional SysML notation");
  });
});

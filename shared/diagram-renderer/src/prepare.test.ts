import { describe, expect, it } from "vitest";
import { normalizeEdgeKind } from "./graph-normalization";
import { prepareViewData } from "./prepare";

describe("shared graph normalization", () => {
  it("normalizes known relationship kinds", () => {
    expect(normalizeEdgeKind("typing")).toBe("typing");
    expect(normalizeEdgeKind("specialization")).toBe("specializes");
    expect(normalizeEdgeKind("owns")).toBe("hierarchy");
    expect(normalizeEdgeKind("")).toBe("relationship");
  });
});

describe("shared prepareViewData", () => {
  it("maps general graph payload and omits package nodes", () => {
    const prepared = prepareViewData({
      view: "general-view",
      selectedViewName: "General",
      graph: {
        nodes: [
          { id: "pkg", name: "Pkg", type: "package" },
          { id: "a", name: "A", type: "part_def" },
          { id: "b", name: "B", type: "part_def" },
        ],
        edges: [{ id: "rel", source: "a", target: "b", type: "typing" }],
      },
    });
    expect(prepared.nodes.map((n) => n.id)).toEqual(["a", "b"]);
    expect(prepared.edges).toHaveLength(1);
    expect(prepared.edges[0].edgeKind).toBe("typing");
  });

  it("maps interconnection payload and filters invalid connectors", () => {
    const prepared = prepareViewData({
      view: "interconnection-view",
      selectedViewName: "Root",
      ibd: {
        parts: [
          { id: "p1", name: "Engine", type: "part" },
          { id: "p2", name: "Controller", type: "part" },
        ],
        connectors: [
          { id: "c1", sourcePartId: "p1", targetPartId: "p2", name: "connect" },
          { id: "c2", sourcePartId: "p1", targetPartId: "missing", name: "bad" },
        ],
      },
    });
    expect(prepared.nodes).toHaveLength(2);
    expect(prepared.edges).toHaveLength(1);
    expect(prepared.edges[0].id).toBe("c1");
    expect(prepared.meta?.selectedRoot).toBe("Root");
  });

  it("keeps interconnection container metadata for renderer parity", () => {
    const prepared = prepareViewData({
      view: "interconnection-view",
      selectedViewName: "ConnectedBlocks",
      ibd: {
        rootCandidates: ["ConnectedBlocks", "IT"],
        packageContainerGroups: [
          { id: "pkg1", name: "ConnectedBlocks", memberIds: ["p1", "p2"] },
        ],
        parts: [
          { id: "p1", name: "Engine", type: "part" },
          { id: "p2", name: "Controller", type: "part" },
        ],
        connectors: [
          { id: "c1", sourcePartId: "p1", targetPartId: "p2", sourceId: "Engine.out", targetId: "Controller.in", type: "flow" },
        ],
      },
    });
    expect(prepared.meta?.rootCandidates).toEqual(["ConnectedBlocks", "IT"]);
    expect(Array.isArray(prepared.meta?.packageContainerGroups)).toBe(true);
    expect(prepared.edges[0].attributes?.relationType).toBe("flow");
  });

  it("adds synthetic initial state when missing", () => {
    const prepared = prepareViewData({
      view: "state-transition-view",
      selectedViewName: "TimerStateMachine",
      synthesizeInitialState: true,
      graph: {
        nodes: [
          { id: "s1", name: "Idle", type: "state" },
          { id: "s2", name: "Running", type: "state" },
        ],
        edges: [{ source: "s1", target: "s2", type: "transition", name: "start" }],
      },
    });
    expect(prepared.nodes[0].id).toBe("__synthetic_initial__");
    expect(prepared.edges.some((edge) => edge.id === "transition-synthetic-initial")).toBe(true);
    expect(prepared.meta?.syntheticInitial).toBe(true);
  });

  it("prefers richest action-flow candidate", () => {
    const prepared = prepareViewData({
      view: "action-flow-view",
      activityDiagrams: [
        { id: "d1", name: "Simple", nodes: [{ id: "n1", name: "A" }], edges: [] },
        {
          id: "d2",
          name: "Rich",
          nodes: [{ id: "n1", name: "A" }, { id: "n2", name: "B" }],
          edges: [{ id: "e1", source: "n1", target: "n2", name: "flow" }],
        },
      ],
    });
    expect(prepared.title).toBe("Rich");
    expect(prepared.meta?.selectedDiagramId).toBe("d2");
  });
});

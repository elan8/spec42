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
  });
});

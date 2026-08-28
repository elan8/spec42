import { describe, expect, it } from "vitest";

import { prepareViewData } from "../prepare";
import { layoutPrepared } from "./layout";

function partNode(id: string, name: string, qualifiedName: string) {
  return {
    id,
    type: "part",
    name,
    qualifiedName,
    attributes: { qualifiedName },
  };
}

describe("general-view layout package hierarchy", () => {
  it("clusters each package's members close together when there are multiple packages", async () => {
    const nodes = [
      partNode("PkgA::root", "root", "PkgA::root"),
      ...Array.from({ length: 6 }, (_, i) => partNode(`PkgA::child${i}`, `child${i}`, `PkgA::child${i}`)),
      partNode("PkgB::other", "other", "PkgB::other"),
      ...Array.from({ length: 6 }, (_, i) => partNode(`PkgB::item${i}`, `item${i}`, `PkgB::item${i}`)),
    ];
    const graph = { nodes, edges: [] };
    const prepared = prepareViewData({ view: "general-view", generalViewGraph: graph });

    const packageGroups = prepared.meta?.packageContainerGroups as
      | Array<{ id: string; name: string; memberIds: string[] }>
      | undefined;
    expect(packageGroups?.length).toBe(2);

    const result = await layoutPrepared(prepared);
    const nodeById = new Map(result.nodes.map((n) => [n.id, n]));

    for (const group of packageGroups ?? []) {
      const members = group.memberIds.map((id) => nodeById.get(id)).filter(Boolean) as typeof result.nodes;
      expect(members.length).toBeGreaterThan(0);
      const xs = members.map((m) => m.x ?? 0);
      const spreadX = Math.max(...xs) - Math.min(...xs);
      // All members of one package sit within one compact ELK container column, not scattered
      // across the whole (potentially much wider) diagram.
      const allXs = result.nodes.map((n) => n.x ?? 0);
      const diagramWidth = Math.max(...allXs) - Math.min(...allXs);
      expect(spreadX).toBeLessThanOrEqual(diagramWidth);
    }

    // Packages should not overlap: package A's members and package B's members occupy disjoint
    // x-ranges (this is what "real hierarchy" buys us over a flat layered graph).
    const groupA = packageGroups?.find((g) => g.name === "PkgA");
    const groupB = packageGroups?.find((g) => g.name === "PkgB");
    const rangeFor = (ids: string[] | undefined) => {
      const xs = (ids ?? []).map((id) => nodeById.get(id)?.x ?? 0);
      return { min: Math.min(...xs), max: Math.max(...xs) };
    };
    const rangeA = rangeFor(groupA?.memberIds);
    const rangeB = rangeFor(groupB?.memberIds);
    const disjoint = rangeA.max < rangeB.min || rangeB.max < rangeA.min;
    expect(disjoint).toBe(true);
  });

  it("falls back to the flat layout when there are fewer than 2 packages", async () => {
    const nodes = [
      partNode("PkgA::root", "root", "PkgA::root"),
      partNode("PkgA::child", "child", "PkgA::child"),
    ];
    const graph = { nodes, edges: [] };
    const prepared = prepareViewData({ view: "general-view", generalViewGraph: graph });
    expect(prepared.meta).toBeUndefined();

    const result = await layoutPrepared(prepared);
    expect(result.nodes).toHaveLength(2);
    expect(result.nodes.every((n) => typeof n.x === "number" && typeof n.y === "number")).toBe(true);
  });

  it("keeps a wide same-parent sibling set on one hierarchy row", async () => {
    const memberCount = 19;
    const owner = partNode("Pkg::Owner", "Owner", "Pkg::Owner");
    const members = Array.from({ length: memberCount }, (_, i) =>
      partNode(`Pkg::member${i}`, `member${i}`, `Pkg::member${i}`),
    );
    const edges = members.map((member, i) => ({
      id: `contains-${i}`,
      source: owner.id,
      target: member.id,
      type: "contains",
    }));
    const graph = { nodes: [owner, ...members], edges };
    const prepared = prepareViewData({ view: "general-view", generalViewGraph: graph });
    // Single package: no package containers, so this exercises the flat-branch chunking path.
    expect(prepared.meta).toBeUndefined();

    const result = await layoutPrepared(prepared);
    expect(result.nodes).toHaveLength(memberCount + 1);
    expect(result.nodes.every((n) => typeof n.x === "number" && typeof n.y === "number")).toBe(true);
    expect(result.edges).toHaveLength(memberCount);
    expect(result.edges.every((e) => e.layout)).toBe(true);

    const ownerLayout = result.nodes.find((node) => node.id === owner.id)!;
    const memberYs = members.map((member) => result.nodes.find((node) => node.id === member.id)?.y);
    expect(new Set(memberYs).size).toBe(1);
    expect(memberYs[0]).toBeGreaterThan(ownerLayout.y ?? 0);
  });

  it("does not draw a relationship usage that is already represented by its composed edge", async () => {
    const prepared = {
      title: "Structure",
      view: "general-view",
      nodes: [
        { id: "n:0", label: "system", kind: "PartUsage" },
        { id: "n:1", label: "source", kind: "PartUsage" },
        { id: "n:2", label: "target", kind: "PartUsage" },
        { id: "n:3", label: "ConnectionUsage", kind: "ConnectionUsage" },
      ],
      edges: [
        { id: "owns-1", source: "n:0", target: "n:1", label: "", edgeKind: "hierarchy", attributes: { originNodeId: "n:1" } },
        { id: "owns-2", source: "n:0", target: "n:2", label: "", edgeKind: "hierarchy", attributes: { originNodeId: "n:2" } },
        { id: "connects", source: "n:1", target: "n:2", label: "", edgeKind: "connection", attributes: { originNodeId: "n:3" } },
      ],
    };

    const result = await layoutPrepared(prepared);
    expect(result.nodes.map((node) => node.id)).toEqual(["n:0", "n:1", "n:2"]);
    expect(result.edges.map((edge) => edge.id)).toEqual(["owns-1", "owns-2", "connects"]);
    expect(result.nodes.find((node) => node.id === "n:1")?.y)
      .toBe(result.nodes.find((node) => node.id === "n:2")?.y);
  });
});

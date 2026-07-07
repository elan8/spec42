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
});

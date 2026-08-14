import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

import { prepareInterconnectionScene } from "./interconnection-scene";

const fixtureDir = join(dirname(fileURLToPath(import.meta.url)), "../../test-fixtures/interconnection");

describe("prepareInterconnectionScene", () => {
  it("maps canonical scene without string owner inference", () => {
    const payload = JSON.parse(readFileSync(join(fixtureDir, "scene-two-part-chain.json"), "utf8"));
    const prepared = prepareInterconnectionScene(payload.interconnectionScene, payload);
    expect(prepared.meta?.canonicalScene).toBe(true);
    expect(prepared.edges[0]).toMatchObject({
      source: "occ:Demo.Source",
      target: "occ:Demo.Target",
      attributes: {
        sourcePortId: "occ:Demo.Source.out",
        targetPortId: "occ:Demo.Target.in",
      },
    });
  });

  it("selects the materialized instance root, prunes ancestor packages, and preserves bindings", () => {
    const scene = {
      schemaVersion: 2,
      view: {
        id: "architecture",
        name: "Architecture",
        type: "InterconnectionView",
        rootIds: ["occ:Demo.SystemDef.child", "occ:Demo.system"],
      },
      nodes: [
        { id: "occ:Demo.system.left", semanticId: "Demo.system.left", qualifiedName: "Demo.system.left", name: "left", kind: "part" },
        { id: "occ:Demo.system.right", semanticId: "Demo.system.right", qualifiedName: "Demo.system.right", name: "right", kind: "part" },
      ],
      ports: [
        { id: "occ:Demo.system.left.out", semanticId: "Demo.system.left.out", ownerNodeId: "occ:Demo.system.left", name: "out", sideHint: "east" },
        { id: "occ:Demo.system.right.in", semanticId: "Demo.system.right.in", ownerNodeId: "occ:Demo.system.right", name: "in", sideHint: "west" },
      ],
      edges: [{
        id: "binding-1",
        kind: "binding",
        sourcePortId: "occ:Demo.system.left.out",
        targetPortId: "occ:Demo.system.right.in",
        sourceNodeId: "occ:Demo.system.left",
        targetNodeId: "occ:Demo.system.right",
      }],
      containers: [
        { id: "occ:Demo", label: "Demo", memberNodeIds: ["occ:Demo.system.left", "occ:Demo.system.right"], depth: 0 },
        { id: "occ:Demo.system", label: "system", memberNodeIds: ["occ:Demo.system.left", "occ:Demo.system.right"], depth: 1 },
      ],
      diagnostics: [],
    };

    const prepared = prepareInterconnectionScene(scene, { interconnectionScene: scene });
    expect(prepared.meta?.selectedRoot).toBe("occ:Demo.system");
    expect(prepared.nodes.some((node) => node.id === "occ:Demo")).toBe(false);
    expect(prepared.nodes.some((node) => node.id === "occ:Demo.system")).toBe(true);
    expect(prepared.edges).toHaveLength(1);
    expect(prepared.edges[0]?.edgeKind).toBe("bind");
  });
});

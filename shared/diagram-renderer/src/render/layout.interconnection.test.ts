import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

import { prepareViewData } from "../prepare";
import { buildInterconnectionElkGraph, buildInterconnectionElkGraphInput } from "./interconnection-layout";
import { layoutInterconnectionPrepared, layoutPrepared } from "./layout";
import {
  assertNoDetachedEndpoints,
  assertWithinBounds,
  assessRouteQuality,
} from "./route-quality";

const fixtureDir = join(dirname(fileURLToPath(import.meta.url)), "../../test-fixtures/interconnection");

function loadFixture(name: string): unknown {
  return JSON.parse(readFileSync(join(fixtureDir, name), "utf8"));
}

function loadSceneFixture(name: string): unknown {
  return JSON.parse(readFileSync(join(fixtureDir, name), "utf8"));
}

function prepareGridSceneFixture(name: string) {
  return prepareViewData({
    view: "interconnection-view",
    interconnectionScene: loadSceneFixture(name),
  });
}

// Regenerate fixtures: cargo test -p semantic_core --test view_expose_powersystems_interconnection export_powersystems -- --nocapture
// Regenerate ELK input goldens: UPDATE_ELK_FIXTURES=1 npm test -- layout.interconnection

function maybeWriteElkGolden(fixtureBaseName: string, payload: unknown): void {
  if (process.env.UPDATE_ELK_FIXTURES !== "1") {
    return;
  }
  const prepared = prepareViewData(payload);
  const elkInput = buildInterconnectionElkGraphInput(prepared);
  writeFileSync(
    join(fixtureDir, `${fixtureBaseName}-elk-input.json`),
    `${JSON.stringify(elkInput, null, 2)}\n`,
    "utf8",
  );
}

async function maybeWriteLayoutGolden(fixtureBaseName: string, payload: unknown): Promise<void> {
  if (process.env.UPDATE_LAYOUT_FIXTURES !== "1") {
    return;
  }
  const prepared = prepareViewData(payload);
  const layout = await layoutPrepared(prepared);
  const positions = (layout.interconnectionLayout?.nodes ?? []).map((node) => ({
    id: node.id,
    x: node.x,
    y: node.y,
    width: node.width,
    height: node.height,
  }));
  writeFileSync(
    join(fixtureDir, `${fixtureBaseName}-elk-layout.json`),
    `${JSON.stringify(positions, null, 2)}\n`,
    "utf8",
  );
}

function edgeSignatures(graph: Record<string, unknown>): Array<[string, string, string]> {
  const edges = Array.isArray(graph.edges) ? graph.edges : [];
  return edges.map((edge) => {
    const record = edge as { id?: string; sources?: string[]; targets?: string[] };
    return [
      String(record.id ?? ""),
      String(record.sources?.[0] ?? ""),
      String(record.targets?.[0] ?? ""),
    ];
  });
}

describe("interconnection layout fixtures", () => {
  it("exports ELK input golden fixtures when UPDATE_ELK_FIXTURES=1", () => {
    maybeWriteElkGolden("scene-two-part-chain", loadFixture("scene-two-part-chain.json"));
    maybeWriteElkGolden("nested-ring-minimal", loadFixture("nested-ring-minimal.json"));
    expect(true).toBe(true);
  });

  it("exports ELK layout position golden fixtures when UPDATE_LAYOUT_FIXTURES=1", async () => {
    await maybeWriteLayoutGolden("scene-two-part-chain", loadFixture("scene-two-part-chain.json"));
    await maybeWriteLayoutGolden("nested-ring-minimal", loadFixture("nested-ring-minimal.json"));
    expect(true).toBe(true);
  });

  it("matches ELK input golden for nested ring when present", () => {
    const fixturePath = join(fixtureDir, "nested-ring-minimal-elk-input.json");
    if (!existsSync(fixturePath)) {
      return;
    }
    const golden = JSON.parse(readFileSync(fixturePath, "utf8")) as Record<string, unknown>;
    const prepared = prepareViewData(loadFixture("nested-ring-minimal.json"));
    const elkInput = buildInterconnectionElkGraphInput(prepared);
    expect(edgeSignatures(elkInput)).toEqual(edgeSignatures(golden));
  });

  it("snapshots ELK input graph for canonical scene fixture", () => {
    const prepared = prepareViewData(loadFixture("scene-two-part-chain.json"));
    expect(prepared.meta?.canonicalScene).toBe(true);
    expect(buildInterconnectionElkGraph(prepared)).toMatchObject({
      id: "root",
      canonicalScene: true,
      roots: ["node:Demo.Source", "node:Demo.Target"],
      edges: [
        {
          id: "edge:Demo.Source.out->Demo.Target.in:0",
          sourcePortId: "port:Demo.Source.out",
          targetPortId: "port:Demo.Target.in",
        },
      ],
    });
  });

  it("passes route quality checks for canonical two-part chain", async () => {
    const prepared = prepareViewData(loadFixture("scene-two-part-chain.json"));
    const layout = await layoutPrepared(prepared);
    expect(layout.interconnectionLayout).toBeDefined();
    const layoutDto = layout.interconnectionLayout!;
    expect(layoutDto.nodes.length).toBeGreaterThanOrEqual(2);
    expect(layoutDto.edges).toHaveLength(1);
    expect(layoutDto.edges[0]?.routePoints.length).toBeGreaterThanOrEqual(2);
    for (const node of layoutDto.nodes) {
      expect(node.portDrawOrder).toBeDefined();
      expect(Object.keys(node.portAnchors).length).toBeGreaterThan(0);
    }
    for (const node of layout.nodes) {
      expect((node.attributes as Record<string, unknown> | undefined)?._portAnchors).toBeUndefined();
      expect((node.attributes as Record<string, unknown> | undefined)?._portDrawOrder).toBeUndefined();
    }
    const report = assessRouteQuality(layout.edges, layout.nodes, { maxLengthRatio: 6 });
    expect(layout.nodes.length).toBeGreaterThanOrEqual(2);
    expect(layout.edges.length).toBe(1);
    expect(report.passed, report.violations.join("; ")).toBe(true);
  });

  it("prepares nested ring fixture with resolved nested target owner", () => {
    const prepared = prepareViewData(loadFixture("nested-ring-minimal.json"));
    expect(prepared.edges).toHaveLength(1);
    expect(prepared.edges[0].target).toBe("node:Grid.northSouthRing.ringSegmentBtoC");
  });

  it("passes route quality checks for nested ring scene fixture", async () => {
    const prepared = prepareViewData(loadFixture("nested-ring-minimal.json"));
    const layout = await layoutPrepared(prepared);
    expect(layout.interconnectionLayout?.containers.length).toBeGreaterThan(0);
    const report = assessRouteQuality(layout.edges, layout.nodes, { maxLengthRatio: 6 });
    expect(layout.edges).toHaveLength(1);
    expect(assertNoDetachedEndpoints(report), report.violations.join("; ")).toEqual([]);
    expect(assertWithinBounds(report), report.violations.join("; ")).toEqual([]);
    expect(report.passed, report.violations.join("; ")).toBe(true);
  });

  it("passes route quality checks for grid systemContext scene fixture", async () => {
    const fixtureName = "grid-system-context-scene.json";
    if (!existsSync(join(fixtureDir, fixtureName))) {
      return;
    }
    const prepared = prepareGridSceneFixture(fixtureName);
    expect(prepared.meta?.canonicalScene).toBe(true);
    expect(prepared.edges.length).toBeGreaterThanOrEqual(17);
    const layout = await layoutPrepared(prepared);
    expect(layout.interconnectionLayout).toBeDefined();
    const layoutDto = layout.interconnectionLayout!;
    for (const edge of layoutDto.edges) {
      expect(edge.routePoints.length).toBeGreaterThanOrEqual(2);
    }
    for (const node of layoutDto.nodes) {
      if (Object.keys(node.portAnchors).length > 0) {
        expect(node.portDrawOrder).toBeDefined();
      }
    }
    const report = assessRouteQuality(layout.edges, layout.nodes, { maxLengthRatio: 8 });
    expect(layout.edges.length).toBeGreaterThanOrEqual(17);
    expect(assertNoDetachedEndpoints(report), report.violations.join("; ")).toEqual([]);
    expect(assertWithinBounds(report), report.violations.join("; ")).toEqual([]);
    expect(report.passed, report.violations.join("; ")).toBe(true);
  });

  it("passes route quality checks for grid gridConnections scene fixture", async () => {
    const fixtureName = "grid-connections-scene.json";
    if (!existsSync(join(fixtureDir, fixtureName))) {
      return;
    }
    const prepared = prepareGridSceneFixture(fixtureName);
    expect(prepared.meta?.canonicalScene).toBe(true);
    expect(prepared.edges.length).toBeGreaterThanOrEqual(15);
    expect(
      prepared.edges.every((edge) => {
        const semanticId = String(edge.attributes?.semanticId ?? "");
        return !semanticId.includes(".Variants.") && !semanticId.includes(".expansionAlternatives.");
      }),
    ).toBe(true);
    const layout = await layoutPrepared(prepared);
    expect(layout.interconnectionLayout).toBeDefined();
    const layoutDto = layout.interconnectionLayout!;
    for (const edge of layoutDto.edges) {
      expect(edge.routePoints.length).toBeGreaterThanOrEqual(2);
    }
    const report = assessRouteQuality(layout.edges, layout.nodes, { maxLengthRatio: 8 });
    expect(layout.edges.length).toBeGreaterThanOrEqual(15);
    expect(assertNoDetachedEndpoints(report), report.violations.join("; ")).toEqual([]);
    expect(assertWithinBounds(report), report.violations.join("; ")).toEqual([]);
    expect(report.passed, report.violations.join("; ")).toBe(true);
  });
});

// @vitest-environment jsdom
import { dirname, join } from "node:path";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import * as d3 from "d3";

import { prepareViewData } from "../prepare";
import { resolveDiagramTheme } from "../theme";
import { buildInterconnectionLayoutLookup } from "./interconnection-layout-dto";
import { drawEdges, interconnectionEdgeLabelAnchor, pathForIbdEdge } from "./drawing";
import { layoutPrepared } from "./layout";
import type { LaidOutEdge, LaidOutNode } from "./types";

const fixtureDir = join(dirname(fileURLToPath(import.meta.url)), "../../test-fixtures/interconnection");

describe("interconnection drawing from layout DTO", () => {
  it("resolves edge paths from layoutDto without attribute fallback", async () => {
    const prepared = prepareViewData(
      JSON.parse(readFileSync(join(fixtureDir, "scene-two-part-chain.json"), "utf8")),
    );
    const layout = await layoutPrepared(prepared);
    const layoutDto = layout.interconnectionLayout;
    expect(layoutDto).toBeDefined();
    const lookup = buildInterconnectionLayoutLookup(layoutDto!);
    const edge = layout.edges[0];
    expect(edge).toBeDefined();

    const pathFromDto = pathForIbdEdge(edge, lookup);
    expect(pathFromDto).toBeTruthy();

    const edgeWithoutAttrs = {
      ...edge,
      attributes: { ...(edge.attributes ?? {}) },
    };
    delete (edgeWithoutAttrs.attributes as Record<string, unknown>).layoutRoutePoints;
    const pathWithoutAttrs = pathForIbdEdge(edgeWithoutAttrs, lookup);
    expect(pathWithoutAttrs).toBe(pathFromDto);

    const pathWithoutLookup = pathForIbdEdge(edgeWithoutAttrs);
    expect(pathWithoutLookup).toBeTruthy();
  });

  it("places a label on the longest horizontal route segment instead of a crowded bend", () => {
    const node = (id: string): LaidOutNode => ({
      id,
      label: id,
      kind: "part",
      x: 0,
      y: 0,
      width: 100,
      height: 80,
    });
    const edge: LaidOutEdge = {
      id: "power",
      source: "source",
      target: "target",
      label: "Power",
      sourceNode: node("source"),
      targetNode: node("target"),
    };
    const lookup = buildInterconnectionLayoutLookup({
      nodes: [],
      containers: [],
      diagnostics: [],
      edges: [{
        id: "power",
        routePoints: [
          { x: 0, y: 0 },
          { x: 16, y: 0 },
          { x: 16, y: 100 },
          { x: 216, y: 100 },
          { x: 216, y: 124 },
        ],
      }],
    });

    expect(interconnectionEdgeLabelAnchor(edge, lookup)).toEqual({
      x: 116,
      y: 100,
      textAnchor: "middle",
      dy: "-0.55em",
    });
  });

  it("paints every connector before the top label layer and gives labels a background halo", () => {
    const svg = d3.select(document.body).append("svg");
    const root = svg.append("g");
    const source: LaidOutNode = {
      id: "source",
      label: "source",
      kind: "part",
      x: 0,
      y: 0,
      width: 100,
      height: 80,
    };
    const target: LaidOutNode = {
      id: "target",
      label: "target",
      kind: "part",
      x: 240,
      y: 0,
      width: 100,
      height: 80,
    };
    const edges: LaidOutEdge[] = ["power", "control"].map((id, index) => ({
      id,
      source: source.id,
      target: target.id,
      label: id,
      edgeKind: "flow",
      attributes: { itemType: id, relationType: "flow" },
      sourceNode: source,
      targetNode: target,
      layout: {
        sections: [{
          startPoint: { x: 100, y: 30 + index * 20 },
          endPoint: { x: 240, y: 30 + index * 20 },
        }],
      },
    }));
    const theme = resolveDiagramTheme({ colorScheme: "dark" });

    drawEdges(root, edges, true, theme);

    const layers = Array.from(root.node()!.children);
    expect(layers.map((layer) => layer.getAttribute("class"))).toEqual([
      "viz-edges",
      "viz-edge-labels",
    ]);
    expect(layers[0].querySelectorAll("path")).toHaveLength(2);
    expect(layers[1].querySelectorAll("text")).toHaveLength(2);
    const label = layers[1].querySelector("text");
    expect(label?.getAttribute("paint-order")).toBe("stroke fill");
    expect(label?.getAttribute("stroke")).toBe(theme.canvasBackground);
    expect(label?.getAttribute("stroke-width")).toBe("4");
  });
});

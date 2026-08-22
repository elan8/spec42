import { describe, expect, it } from "vitest";

import { resolveIbdRoutePoints, snapRouteEndpoints } from "./ibd-route";
import type { LaidOutEdge } from "./types";

function expectOrthogonal(points: Array<{ x: number; y: number }>): void {
  for (let index = 1; index < points.length; index += 1) {
    const previous = points[index - 1];
    const current = points[index];
    expect(previous.x === current.x || previous.y === current.y).toBe(true);
  }
}

describe("IBD route endpoint snapping", () => {
  it("preserves orthogonal routing when snapping ELK routes to port centers", () => {
    const route = snapRouteEndpoints(
      [
        { x: 100, y: 70 },
        { x: 180, y: 70 },
        { x: 180, y: 220 },
      ],
      { x: 80, y: 70 },
      { x: 210, y: 220 },
    );

    expect(route[0]).toEqual({ x: 80, y: 70 });
    expect(route[route.length - 1]).toEqual({ x: 210, y: 220 });
    expect(route).toContainEqual({ x: 180, y: 70 });
    expect(route).toContainEqual({ x: 180, y: 220 });
    expectOrthogonal(route);
  });

  it("selects container offset for nested-port ELK sections", () => {
    const edge = {
      id: "edge:nested",
      source: "node:source",
      target: "node:target",
      label: "",
      sourceNode: { id: "node:source", label: "source", kind: "part", x: 900, y: 200 },
      targetNode: {
        id: "node:target",
        label: "target",
        kind: "part",
        x: 520,
        y: 260,
        attributes: { containerId: "node:container" },
      },
      layout: {
        sections: [
          {
            startPoint: { x: 360, y: 40 },
            bendPoints: [{ x: 420, y: 40 }, { x: 420, y: 70 }],
            endPoint: { x: 470, y: 70 },
          },
        ],
        edgeOwnerOffset: { x: 0, y: 0 },
        lcaOffset: { x: 500, y: 220 },
      },
      attributes: {
        _sourcePortCenter: { x: 900, y: 270 },
        _targetPortCenter: { x: 520, y: 295 },
      },
    } satisfies LaidOutEdge;

    const points = resolveIbdRoutePoints(edge);
    expect(points).not.toBeNull();
    expect(points?.[0]).toEqual({ x: 900, y: 270 });
    expect(points?.[points!.length - 1]).toEqual({ x: 520, y: 295 });
  });
});

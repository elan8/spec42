import { describe, expect, it } from "vitest";

import { snapRouteEndpoints } from "./ibd-route";

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
        { x: 100, y: 40 },
        { x: 180, y: 40 },
        { x: 180, y: 190 },
      ],
      { x: 80, y: 70 },
      { x: 210, y: 220 },
    );

    expect(route[0]).toEqual({ x: 80, y: 70 });
    expect(route[route.length - 1]).toEqual({ x: 210, y: 220 });
    expect(route).toContainEqual({ x: 100, y: 70 });
    expect(route).toContainEqual({ x: 180, y: 220 });
    expectOrthogonal(route);
  });
});

import { describe, expect, it } from "vitest";

import {
  assertNoDetachedEndpoints,
  assertWithinBounds,
  assessRouteQuality,
  endpointDetached,
  manhattanDistance,
  polylineLength,
  routePointsOutsideBounds,
} from "./route-quality";

describe("route quality helpers", () => {
  it("computes polyline length and Manhattan distance", () => {
    const points = [
      { x: 0, y: 0 },
      { x: 10, y: 0 },
      { x: 10, y: 5 },
    ];
    expect(polylineLength(points)).toBe(15);
    expect(manhattanDistance({ x: 0, y: 0 }, { x: 10, y: 5 })).toBe(15);
  });

  it("detects detached endpoints and out-of-bounds points", () => {
    expect(endpointDetached({ x: 0, y: 0 }, { x: 1, y: 1 }, 2)).toBe(false);
    expect(endpointDetached({ x: 0, y: 0 }, { x: 10, y: 10 }, 2)).toBe(true);
    expect(
      routePointsOutsideBounds(
        [{ x: 200, y: 50 }],
        { minX: 0, minY: 0, maxX: 100, maxY: 100 },
      ),
    ).toBe(true);
  });

  it("flags node-boundary fallback when port ids are present", () => {
    const report = assessRouteQuality(
      [
        {
          id: "e1",
          source: "a",
          target: "b",
          label: "link",
          attributes: {
            sourcePortId: "port:a.out",
            targetPortId: "port:b.in",
          },
          layout: {
            sections: [
              {
                startPoint: { x: 0, y: 0 },
                endPoint: { x: 10, y: 0 },
              },
            ],
            edgeOwnerOffset: { x: 0, y: 0 },
            lcaOffset: { x: 0, y: 0 },
          },
        },
      ],
      [],
      { maxLengthRatio: 10 },
    );
    expect(report.violations.some((item) => item.includes("node-boundary fallback"))).toBe(true);
  });

  it("collects detached endpoint and out-of-bounds violations separately", () => {
    const report = assessRouteQuality(
      [
        {
          id: "e1",
          source: "a",
          target: "b",
          label: "link",
          attributes: {
            _sourcePortCenter: { x: 0, y: 0 },
            _targetPortCenter: { x: 100, y: 0 },
          },
          layout: {
            sections: [
              {
                startPoint: { x: 50, y: 0 },
                endPoint: { x: 100, y: 0 },
              },
            ],
            edgeOwnerOffset: { x: 0, y: 0 },
            lcaOffset: { x: 0, y: 0 },
          },
        },
      ],
      [{ id: "a", label: "A", kind: "part", x: 0, y: 0, width: 40, height: 40 }],
      { maxLengthRatio: 10, maxDetachedDistance: 2 },
    );
    expect(assertNoDetachedEndpoints(report)).toEqual(["e1: detached route endpoint"]);
    expect(assertWithinBounds(report)).toEqual([]);
  });
});

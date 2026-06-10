import { describe, expect, it } from "vitest";
import { edgeLabelPositionFromSections } from "./elk-label-utils";

describe("sequence layout helpers", () => {
  it("edgeLabelPositionFromSections returns midpoint for simple section", () => {
    const position = edgeLabelPositionFromSections([
      {
        startPoint: { x: 0, y: 0 },
        endPoint: { x: 100, y: 0 },
      },
    ]);
    expect(position?.x).toBe(50);
    expect(typeof position?.y).toBe("number");
  });
});

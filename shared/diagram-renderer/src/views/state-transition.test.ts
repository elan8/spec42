import { describe, expect, it } from "vitest";
import { nodeSupportsSourceNavigation } from "./behavior-interaction";

describe("state-transition interaction", () => {
  it("nodeSupportsSourceNavigation requires uri or range", () => {
    expect(nodeSupportsSourceNavigation({ id: "s1", label: "", kind: "state" })).toBe(false);
    expect(
      nodeSupportsSourceNavigation({
        id: "s1",
        label: "Idle",
        kind: "state",
        uri: "file:///model.sysml",
      }),
    ).toBe(true);
  });
});

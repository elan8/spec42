import { describe, expect, it } from "vitest";
import { truncateLabel } from "./behavior-common";

describe("action-flow helpers", () => {
  it("truncateLabel shortens long labels", () => {
    expect(truncateLabel("short", 20)).toBe("short");
    expect(truncateLabel("abcdefghijklmnopqrstuvwxyz", 10)).toBe("abcdefgh..");
  });
});

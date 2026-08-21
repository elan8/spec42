import { describe, expect, it } from "vitest";
import { normalizeEdgeKind } from "./graph-normalization";

describe("normalizeEdgeKind", () => {
  it("maps BNF connector names for interconnection view", () => {
    expect(normalizeEdgeKind("binding-connection")).toBe("bind");
    expect(normalizeEdgeKind("interface-connection")).toBe("interface");
    expect(normalizeEdgeKind("flow-on-connection")).toBe("flow");
  });

  it("maps BNF relationship names for general view", () => {
    expect(normalizeEdgeKind("binary-dependency")).toBe("dependency");
    expect(normalizeEdgeKind("redefinition")).toBe("redefinition");
    expect(normalizeEdgeKind("usage")).toBe("usage");
  });
});

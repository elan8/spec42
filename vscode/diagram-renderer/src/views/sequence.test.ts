import { describe, expect, it } from "vitest";
import { edgeLabelPositionFromSections } from "./elk-label-utils";
import {
  asArray,
  asRecord,
  asString,
  findPreparedLifeline,
  messageRef,
  messageRow,
} from "./sequence";
import type { PreparedNode } from "../prepare";

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

describe("asRecord", () => {
  it("passes through plain objects", () => {
    const obj = { a: 1, b: "two" };
    expect(asRecord(obj)).toBe(obj);
  });

  it("returns empty record for null", () => {
    expect(asRecord(null)).toEqual({});
  });

  it("returns empty record for undefined", () => {
    expect(asRecord(undefined)).toEqual({});
  });

  it("returns empty record for strings", () => {
    expect(asRecord("hello")).toEqual({});
  });

  it("returns empty record for numbers", () => {
    expect(asRecord(42)).toEqual({});
  });

  it("passes through arrays (arrays are objects)", () => {
    const arr = [1, 2, 3];
    expect(asRecord(arr)).toBe(arr);
  });
});

describe("asArray", () => {
  it("passes through arrays", () => {
    const arr = [1, 2, 3];
    expect(asArray(arr)).toBe(arr);
  });

  it("returns empty array for null", () => {
    expect(asArray(null)).toEqual([]);
  });

  it("returns empty array for undefined", () => {
    expect(asArray(undefined)).toEqual([]);
  });

  it("returns empty array for objects", () => {
    expect(asArray({ a: 1 })).toEqual([]);
  });

  it("returns empty array for strings", () => {
    expect(asArray("hello")).toEqual([]);
  });

  it("returns empty array for numbers", () => {
    expect(asArray(42)).toEqual([]);
  });
});

describe("asString", () => {
  it("passes through strings", () => {
    expect(asString("hello")).toBe("hello");
    expect(asString("")).toBe("");
  });

  it("converts numbers to string", () => {
    expect(asString(42)).toBe("42");
    expect(asString(0)).toBe("0");
    expect(asString(-1)).toBe("-1");
  });

  it("converts booleans to string", () => {
    expect(asString(true)).toBe("true");
    expect(asString(false)).toBe("false");
  });

  it("returns empty string fallback for null", () => {
    expect(asString(null)).toBe("");
  });

  it("returns empty string fallback for undefined", () => {
    expect(asString(undefined)).toBe("");
  });

  it("returns empty string fallback for objects", () => {
    expect(asString({ a: 1 })).toBe("");
  });

  it("uses custom fallback when provided", () => {
    expect(asString(null, "default")).toBe("default");
    expect(asString(undefined, "fallback")).toBe("fallback");
    expect(asString({}, "none")).toBe("none");
  });

  it("does not use fallback for valid values", () => {
    expect(asString("real", "fallback")).toBe("real");
    expect(asString(0, "fallback")).toBe("0");
  });
});

describe("messageRow", () => {
  // LIFELINE_TOP = 118, MESSAGE_GAP = 78; row = LIFELINE_TOP + 58 + (max(1,order) - 1) * MESSAGE_GAP
  it("returns correct Y for order 1", () => {
    expect(messageRow(1)).toBe(176); // 118 + 58 + 0
  });

  it("returns correct Y for order 2", () => {
    expect(messageRow(2)).toBe(254); // 118 + 58 + 78
  });

  it("returns correct Y for order 3", () => {
    expect(messageRow(3)).toBe(332); // 118 + 58 + 156
  });

  it("treats order 0 same as order 1 (clamps to 1)", () => {
    expect(messageRow(0)).toBe(messageRow(1));
  });

  it("treats negative order same as order 1", () => {
    expect(messageRow(-5)).toBe(messageRow(1));
  });

  it("rows are evenly spaced by MESSAGE_GAP", () => {
    expect(messageRow(2) - messageRow(1)).toBe(78);
    expect(messageRow(5) - messageRow(4)).toBe(78);
  });
});

describe("messageRef", () => {
  it("returns id when present", () => {
    expect(messageRef({ id: "msg-1", name: "call", label: "call()" })).toBe("msg-1");
  });

  it("falls back to name when id is absent", () => {
    expect(messageRef({ name: "call", label: "call()" })).toBe("call");
  });

  it("falls back to label when id and name are absent", () => {
    expect(messageRef({ label: "call()" })).toBe("call()");
  });

  it("returns empty string when nothing is present", () => {
    expect(messageRef({})).toBe("");
  });

  it("returns empty string when all relevant fields are null", () => {
    expect(messageRef({ id: null, name: null, label: null })).toBe("");
  });
});

describe("findPreparedLifeline", () => {
  const makeNode = (overrides: Partial<PreparedNode>): PreparedNode => ({
    id: "node-1",
    label: "Component",
    kind: "part",
    edges: [],
    attributes: {},
    ...overrides,
  });

  it("finds a node by matching id", () => {
    const node = makeNode({ id: "ll-a" });
    const result = findPreparedLifeline([node], { id: "ll-a" });
    expect(result).toBe(node);
  });

  it("finds a node by matching label to lifeline name", () => {
    const node = makeNode({ id: "node-1", label: "Sensor" });
    const result = findPreparedLifeline([node], { name: "Sensor" });
    expect(result).toBe(node);
  });

  it("finds a node by qualifiedName matching lifeline id", () => {
    const node = makeNode({ id: "node-1", label: "X", attributes: { qualifiedName: "Pkg::Sensor" } });
    const result = findPreparedLifeline([node], { id: "Pkg::Sensor" });
    expect(result).toBe(node);
  });

  it("finds a node by qualifiedName matching lifeline name", () => {
    const node = makeNode({ id: "node-1", label: "X", attributes: { qualifiedName: "Pkg::Sensor" } });
    const result = findPreparedLifeline([node], { name: "Pkg::Sensor" });
    expect(result).toBe(node);
  });

  it("returns undefined when no node matches", () => {
    const node = makeNode({ id: "node-1", label: "Actuator" });
    const result = findPreparedLifeline([node], { id: "unknown-lifeline" });
    expect(result).toBeUndefined();
  });

  it("returns undefined for empty node list", () => {
    const result = findPreparedLifeline([], { id: "ll-a" });
    expect(result).toBeUndefined();
  });

  it("returns first matching node when multiple match", () => {
    const node1 = makeNode({ id: "ll-a", label: "A" });
    const node2 = makeNode({ id: "ll-a", label: "B" });
    const result = findPreparedLifeline([node1, node2], { id: "ll-a" });
    expect(result).toBe(node1);
  });
});

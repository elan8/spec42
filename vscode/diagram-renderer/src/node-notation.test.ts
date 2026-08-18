import { describe, expect, it } from "vitest";
import {
  isDefinitionKind,
  isReferenceKind,
  nodeBodyChromeStyle,
  nodeBodyStrokeDasharray,
  resolveNodeChrome,
} from "./node-notation";

describe("isDefinitionKind", () => {
  it("detects part def and requirement def", () => {
    expect(isDefinitionKind("part def")).toBe(true);
    expect(isDefinitionKind("part_def")).toBe(true);
    expect(isDefinitionKind("requirement definition")).toBe(true);
  });

  it("does not treat reference or composite usage as definition", () => {
    expect(isDefinitionKind("part")).toBe(false);
    expect(isDefinitionKind("ref")).toBe(false);
    expect(isDefinitionKind("part-ref")).toBe(false);
  });
});

describe("isReferenceKind", () => {
  it("detects ref usages", () => {
    expect(isReferenceKind("ref")).toBe(true);
    expect(isReferenceKind("part-ref")).toBe(true);
    expect(isReferenceKind("port ref")).toBe(true);
  });

  it("does not match refine", () => {
    expect(isReferenceKind("refine")).toBe(false);
  });
});

describe("resolveNodeChrome", () => {
  it("uses solid sharp corners for definitions", () => {
    const chrome = resolveNodeChrome("part def");
    expect(chrome.isDefinition).toBe(true);
    expect(chrome.cornerRadius).toBe(0);
    expect(chrome.strokeDasharray).toBeNull();
    expect(chrome.structureClass).toBe("viz-node--definition");
  });

  it("uses solid rounded corners for composite usages", () => {
    const chrome = resolveNodeChrome("part");
    expect(chrome.isDefinition).toBe(false);
    expect(chrome.isReference).toBe(false);
    expect(chrome.cornerRadius).toBe(8);
    expect(chrome.strokeDasharray).toBeNull();
    expect(chrome.structureClass).toBe("viz-node--usage");
  });

  it("uses dotted rounded corners for reference usages", () => {
    const chrome = resolveNodeChrome("ref");
    expect(chrome.isReference).toBe(true);
    expect(chrome.cornerRadius).toBe(8);
    expect(chrome.strokeDasharray).toBe("2,4");
    expect(chrome.structureClass).toBe("viz-node--reference");
  });

  it("uses dashed frame for containers", () => {
    const chrome = resolveNodeChrome("part_usage", { isContainer: true });
    expect(chrome.isContainer).toBe(true);
    expect(chrome.strokeDasharray).toBe("4,4");
    expect(chrome.structureClass).toBe("viz-node--container");
  });

  it("reference wins over definition when both hinted", () => {
    const chrome = resolveNodeChrome("part def", { isDefinition: true, isReference: true });
    expect(chrome.isReference).toBe(true);
    expect(chrome.isDefinition).toBe(false);
  });

  it("uses extra rounding for requirement usages", () => {
    expect(resolveNodeChrome("requirement").cornerRadius).toBe(16);
    expect(resolveNodeChrome("requirement def").cornerRadius).toBe(0);
  });

  it("keeps package containers solid while layout containers are dashed", () => {
    const layout = resolveNodeChrome("part_usage", { isContainer: true });
    const pkg = resolveNodeChrome("package", { isContainer: true, isPackageContainer: true });
    expect(nodeBodyStrokeDasharray(layout)).toBe("4,4");
    expect(nodeBodyStrokeDasharray(pkg, true)).toBe("none");
  });

  it("nodeBodyChromeStyle matches general vs ibd stroke widths", () => {
    const def = resolveNodeChrome("part def");
    expect(nodeBodyChromeStyle(def, { generalView: true }).strokeWidthPx).toBe(3);
    expect(nodeBodyChromeStyle(def, { generalView: false }).strokeWidthPx).toBe(2);
  });
});

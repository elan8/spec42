import { describe, expect, it } from "vitest";
import {
  nodeBodyChromeStyle,
  nodeBodyStrokeDasharray,
  resolveNodeChrome,
} from "./node-notation";

describe("resolveNodeChrome", () => {
  it("uses solid sharp corners for definitions", () => {
    const chrome = resolveNodeChrome("definition");
    expect(chrome.isDefinition).toBe(true);
    expect(chrome.cornerRadius).toBe(0);
    expect(chrome.strokeDasharray).toBeNull();
    expect(chrome.structureClass).toBe("viz-node--definition");
  });

  it("uses solid rounded corners for composite usages", () => {
    const chrome = resolveNodeChrome("usage");
    expect(chrome.isDefinition).toBe(false);
    expect(chrome.isReference).toBe(false);
    expect(chrome.cornerRadius).toBe(8);
    expect(chrome.strokeDasharray).toBeNull();
    expect(chrome.structureClass).toBe("viz-node--usage");
  });

  it("uses dotted rounded corners for reference usages", () => {
    const chrome = resolveNodeChrome("reference-usage");
    expect(chrome.isReference).toBe(true);
    expect(chrome.cornerRadius).toBe(8);
    expect(chrome.strokeDasharray).toBe("2,4");
    expect(chrome.structureClass).toBe("viz-node--reference");
  });

  it("uses dashed frame for containers", () => {
    const chrome = resolveNodeChrome("usage", { isContainer: true });
    expect(chrome.isContainer).toBe(true);
    expect(chrome.strokeDasharray).toBe("4,4");
    expect(chrome.structureClass).toBe("viz-node--container");
  });

  it("marks an unsupported role without normative chrome", () => {
    const chrome = resolveNodeChrome("unsupported");
    expect(chrome.structureClass).toBe("viz-node--unsupported");
    expect(chrome.strokeDasharray).toBe("3,3");
  });

  it("keeps package containers solid while layout containers are dashed", () => {
    const layout = resolveNodeChrome("usage", { isContainer: true });
    const pkg = resolveNodeChrome("namespace", { isContainer: true, isPackageContainer: true });
    expect(nodeBodyStrokeDasharray(layout)).toBe("4,4");
    expect(nodeBodyStrokeDasharray(pkg, true)).toBe("none");
  });

  it("nodeBodyChromeStyle matches general vs ibd stroke widths", () => {
    const def = resolveNodeChrome("definition");
    expect(nodeBodyChromeStyle(def, { generalView: true }).strokeWidthPx).toBe(3);
    expect(nodeBodyChromeStyle(def, { generalView: false }).strokeWidthPx).toBe(2);
  });
});

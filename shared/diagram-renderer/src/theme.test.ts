// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import {
  detectColorScheme,
  resolveDiagramTheme,
  strokeColorForEdge,
  strokeColorForNode,
} from "./theme";

describe("notation diagram theme", () => {
  it("resolves distinct light and dark token tables", () => {
    const light = resolveDiagramTheme({ colorScheme: "light" });
    const dark = resolveDiagramTheme({ colorScheme: "dark" });
    expect(light.nodeBorder).not.toBe(dark.nodeBorder);
    expect(light.nodeFill).not.toBe(dark.nodeFill);
    expect(light.edge.default).toBe(light.nodeBorder);
    expect(dark.edge.default).toBe(dark.nodeBorder);
  });

  it("uses one ink color for all element kinds", () => {
    const theme = resolveDiagramTheme({ colorScheme: "light" });
    expect(strokeColorForNode(theme)).toBe(theme.nodeBorder);
    expect(strokeColorForEdge("flow", theme)).toBe(theme.edge.default);
    expect(strokeColorForEdge("satisfy", theme)).toBe(theme.edge.default);
    expect(strokeColorForEdge("allocate", theme)).toBe(theme.edge.default);
  });

  it("vscode mode uses CSS variables without hex node maps", () => {
    const theme = resolveDiagramTheme({ colorScheme: "vscode" });
    expect(theme.nodeBorder).toContain("var(--vscode-editor-foreground)");
    expect(theme.edge.default).toContain("var(--vscode-editor-foreground)");
    expect("node" in theme).toBe(false);
  });

  it("detectColorScheme reads data-color-scheme from SVG ancestor", () => {
    const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    svg.setAttribute("class", "sysml-viz-svg");
    svg.setAttribute("data-color-scheme", "dark");
    const host = document.createElement("div");
    svg.appendChild(host);
    expect(detectColorScheme(host)).toBe("dark");
  });
});

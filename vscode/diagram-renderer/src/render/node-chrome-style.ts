import type * as d3 from "d3";
import type { DiagramTheme } from "../theme";

/**
 * Interaction styling for node chrome controls.
 *
 * Hover and focus states cannot be expressed through per-element inline styles, so the renderer
 * emits one stylesheet built from the resolved theme tokens. It lives inside the SVG, which keeps
 * exported SVG self-contained and keeps the controls legible in every colour scheme.
 */
export function nodeChromeStyleSheet(theme: DiagramTheme): string {
  return [
    ".sysml-disclosure { cursor: pointer; outline: none; }",
    ".sysml-disclosure .sysml-disclosure-box,",
    ".sysml-disclosure .sysml-disclosure-glyph { pointer-events: none; }",
    // The hit target carries an inline `fill: transparent` so it stays invisible even when this
    // stylesheet is absent (standalone `renderSysMLNode` callers); the hover and focus fills
    // therefore have to override that inline declaration.
    `.sysml-disclosure:hover .sysml-disclosure-target { fill: ${theme.controlHoverFill} !important; fill-opacity: 0.55; }`,
    `.sysml-disclosure:hover .sysml-disclosure-box { fill: ${theme.controlHoverFill}; stroke: ${theme.controlForeground}; }`,
    `.sysml-disclosure:focus-visible .sysml-disclosure-target { stroke: ${theme.focusRing}; stroke-width: 2px; fill: ${theme.controlHoverFill} !important; fill-opacity: 0.35; }`,
    `.sysml-disclosure:focus-visible .sysml-disclosure-box { stroke: ${theme.focusRing}; }`,
    ".sysml-compartment-label text { user-select: none; }",
    ".viz-node text { user-select: none; }",
  ].join("\n");
}

export function installNodeChromeStyles(
  svg: d3.Selection<SVGSVGElement, unknown, null, undefined>,
  theme: DiagramTheme,
): void {
  svg.append("style").attr("class", "sysml-node-chrome-style").text(nodeChromeStyleSheet(theme));
}

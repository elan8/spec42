import * as d3 from "d3";
import type { Selection } from "d3";
import type { PreparedNode } from "../prepare";
import type { RenderOptions } from "../renderer";
import type { DiagramTheme } from "../theme";

export interface BehaviorJumpPayload {
  name: string;
  id?: string;
  uri?: string;
  range?: PreparedNode["range"];
  parentContext?: string;
}

export function nodeSupportsSourceNavigation(node: PreparedNode): boolean {
  const attrs = (node.attributes ?? {}) as Record<string, unknown>;
  const qualifiedName = asString(attrs.qualifiedName ?? node.id);
  const uri = asString(node.uri ?? node.sourcePath);
  const range = node.range;
  const hasRange = Boolean(range?.start && typeof range.start.line === "number");
  return Boolean((uri && hasRange) || qualifiedName.includes("::") || node.label.trim());
}

export function jumpPayloadFromNode(node: PreparedNode, parentContext?: string): BehaviorJumpPayload {
  const attrs = (node.attributes ?? {}) as Record<string, unknown>;
  const qualifiedName = asString(attrs.qualifiedName ?? node.id);
  const looksQualified = qualifiedName.includes("::");
  const uri = asString(node.uri ?? node.sourcePath) || undefined;
  return {
    name: node.label || node.id,
    id: looksQualified ? qualifiedName : undefined,
    uri,
    range: node.range ?? undefined,
    parentContext: parentContext || undefined,
  };
}

export function clearBehaviorHighlights(root: Selection<SVGGElement, unknown, null, undefined>): void {
  root.selectAll(".highlighted-element").each(function () {
    const group = d3.select(this);
    group.classed("highlighted-element", false);
    group.select(".node-background").each(function () {
      const el = d3.select(this);
      const origStroke = el.attr("data-original-stroke");
      const origWidth = el.attr("data-original-width");
      if (origStroke) {
        el.style("stroke", origStroke).style("stroke-width", origWidth);
      }
    });
  });
}

export function attachBehaviorNodeClick(
  nodeGroup: Selection<SVGGElement, unknown, null, undefined>,
  node: PreparedNode,
  theme: DiagramTheme,
  options: RenderOptions,
  root: Selection<SVGGElement, unknown, null, undefined>,
): void {
  nodeGroup
    .style("cursor", options.onNodeClick && nodeSupportsSourceNavigation(node) ? "pointer" : "")
    .on("click", (event: Event) => {
      if (!options.onNodeClick || !nodeSupportsSourceNavigation(node)) {
        return;
      }
      event.stopPropagation();
      clearBehaviorHighlights(root);
      nodeGroup.classed("highlighted-element", true);
      const background = nodeGroup.select(".node-background");
      if (!background.empty()) {
        background.style("stroke", theme.highlight).style("stroke-width", "3px");
      }
      options.onNodeClick(node);
    });
}

function asString(value: unknown, fallback = ""): string {
  if (typeof value === "string") return value;
  if (typeof value === "number" || typeof value === "boolean") return String(value);
  return fallback;
}

import * as d3 from "d3";
import type { PreparedNode } from "../prepare";
import type { DiagramTheme } from "../theme";
import { attachBehaviorNodeClick } from "./behavior-interaction";
import {
  BehaviorSceneContext,
  edgeLabelPositionFromSections,
  fallbackEdgePath,
  layoutBehaviorGraph,
  nodeKind,
  pathFromSections,
  truncateLabel,
} from "./behavior-common";

function isInitial(kind: string): boolean {
  return kind.includes("initial") || kind.includes("start");
}
function isFinal(kind: string): boolean {
  return kind.includes("final") || kind.includes("done") || kind.includes("end");
}
function isFlowFinal(kind: string): boolean {
  return kind.includes("flow-final") || kind.includes("flow final") || kind.includes("terminate");
}
function isDecision(kind: string): boolean {
  return kind.includes("decision") || kind.includes("merge");
}
function isFork(kind: string): boolean {
  return kind.includes("fork") || kind.includes("join");
}

function drawActionNode(
  group: d3.Selection<SVGGElement, unknown, null, undefined>,
  node: PreparedNode,
  layout: { x: number; y: number; width: number; height: number },
  theme: DiagramTheme,
): d3.Selection<SVGGElement, unknown, null, undefined> {
  const kind = nodeKind(node);
  const attrs = (node.attributes ?? {}) as Record<string, unknown>;
  const inputs = Array.isArray(attrs.inputs) ? attrs.inputs : Array.isArray(attrs.inputParameters) ? attrs.inputParameters : [];
  const outputs = Array.isArray(attrs.outputs) ? attrs.outputs : Array.isArray(attrs.outputParameters) ? attrs.outputParameters : [];
  const isPerform = kind.includes("perform") || String(attrs.actionType ?? attrs.type ?? "").toLowerCase().includes("perform");
  const g = group
    .append("g")
    .attr("class", `activity-action action-flow-node${isPerform ? " perform-action-node" : ""}`)
    .attr("data-node-id", node.id)
    .attr("transform", `translate(${layout.x},${layout.y})`);

  if (isInitial(kind) || isFinal(kind)) {
    g.append("circle")
      .attr("class", "node-background")
      .attr("data-original-stroke", theme.nodeBorder)
      .attr("data-original-width", "2px")
      .attr("cx", layout.width / 2)
      .attr("cy", layout.height / 2)
      .attr("r", layout.width / 2 - 2)
      .style("fill", isInitial(kind) ? theme.edge.default : theme.canvasBackground)
      .style("stroke", theme.nodeBorder)
      .style("stroke-width", "2px");
    if (isFinal(kind) && !isFlowFinal(kind)) {
      g.append("circle")
        .attr("cx", layout.width / 2)
        .attr("cy", layout.height / 2)
        .attr("r", 10)
        .style("fill", theme.edge.default)
        .style("stroke", "none");
    }
    if (isFlowFinal(kind)) {
      g.append("path")
        .attr("class", "flow-final-x")
        .attr("d", `M${layout.width / 2 - 8},${layout.height / 2 - 8} L${layout.width / 2 + 8},${layout.height / 2 + 8} M${layout.width / 2 + 8},${layout.height / 2 - 8} L${layout.width / 2 - 8},${layout.height / 2 + 8}`)
        .style("stroke", theme.edge.default)
        .style("stroke-width", "2px");
    }
  } else if (isDecision(kind)) {
    const cx = layout.width / 2;
    const cy = layout.height / 2;
    g.append("path")
      .attr("class", "node-background")
      .attr("data-original-stroke", theme.edge.default)
      .attr("data-original-width", "2px")
      .attr("d", `M${cx},0 L${layout.width},${cy} L${cx},${layout.height} L0,${cy} Z`)
      .style("fill", theme.canvasBackground)
      .style("stroke", theme.edge.default)
      .style("stroke-width", "2px");
  } else if (isFork(kind)) {
    g.append("rect")
      .attr("class", "node-background")
      .attr("data-original-stroke", "none")
      .attr("data-original-width", "0px")
      .attr("width", layout.width)
      .attr("height", layout.height)
      .attr("rx", 3)
      .style("fill", theme.nodeBorder)
      .style("stroke", "none");
  } else {
    g.append("rect")
      .attr("class", "node-background")
      .attr("data-original-stroke", theme.nodeBorder)
      .attr("data-original-width", "2px")
      .attr("width", layout.width)
      .attr("height", layout.height)
      .attr("rx", 8)
      .style("fill", theme.nodeFill)
      .style("stroke", theme.nodeBorder)
      .style("stroke-width", "2px")
      .style("stroke-dasharray", isPerform ? "5,3" : "none");
    g.append("rect")
      .attr("width", layout.width)
      .attr("height", 6)
      .attr("rx", 8)
      .style("fill", theme.nodeBorder)
      .style("stroke", "none");
    if (isPerform) {
      g.append("text")
        .attr("class", "perform-action-stereotype")
        .attr("x", layout.width / 2)
        .attr("y", 20)
        .attr("text-anchor", "middle")
        .style("font-size", "9px")
        .style("fill", theme.textSecondary)
        .text("perform");
    }
  }

  const labelY = isFork(kind) ? layout.height + 14 : layout.height / 2 + (isPerform ? 12 : 4);
  g.append("text")
    .attr("x", layout.width / 2)
    .attr("y", labelY)
    .attr("text-anchor", "middle")
    .style("font-size", "12px")
    .style("font-weight", "600")
    .style("fill", theme.textPrimary)
    .text(truncateLabel(node.label, 24));

  const drawParameter = (items: unknown[], side: "input" | "output") => {
    items.slice(0, 4).forEach((item, index) => {
      const parameter = item && typeof item === "object" ? item as Record<string, unknown> : { name: String(item) };
      const name = String(parameter.name ?? parameter.label ?? item ?? "");
      const y = 20 + index * 14;
      const x = side === "input" ? -9 : layout.width + 9;
      g.append("circle")
        .attr("class", `action-parameter-badge action-parameter-${side}`)
        .attr("cx", x)
        .attr("cy", y)
        .attr("r", 5)
        .style("fill", theme.canvasBackground)
        .style("stroke", theme.nodeBorder)
        .style("stroke-width", "1.5px");
      g.append("text")
        .attr("class", `action-parameter-label action-parameter-${side}-label`)
        .attr("x", side === "input" ? x - 8 : x + 8)
        .attr("y", y + 3)
        .attr("text-anchor", side === "input" ? "end" : "start")
        .style("font-size", "8px")
        .style("fill", theme.textSecondary)
        .text(truncateLabel(name, 14));
    });
  };
  if (!isInitial(kind) && !isFinal(kind) && !isDecision(kind) && !isFork(kind)) {
    drawParameter(inputs, "input");
    drawParameter(outputs, "output");
  }

  return g;
}

export async function renderActionFlowView(ctx: BehaviorSceneContext): Promise<{ minX: number; minY: number; maxX: number; maxY: number }> {
  const horizontal = String(ctx.prepared.meta?.layoutDirection ?? "").toLowerCase() === "horizontal";
  const layout = await layoutBehaviorGraph(ctx.prepared, { horizontal, mode: "action" });
  const renderOptions = ctx.options ?? {};

  ctx.root
    .append("text")
    .attr("x", 24)
    .attr("y", 28)
    .style("font-size", "14px")
    .style("font-weight", "700")
    .style("fill", ctx.theme.textPrimary)
    .text(ctx.prepared.title || "Action Flow");

  const flowLayer = ctx.root.append("g").attr("class", "activity-flows");
  const nodeLayer = ctx.root.append("g").attr("class", "activity-actions");

  for (const edge of ctx.prepared.edges) {
    const source = layout.positions.get(edge.source);
    const target = layout.positions.get(edge.target);
    if (!source || !target) continue;
    const sections = layout.edgeSectionsById.get(edge.id);
    const fallback = fallbackEdgePath(source, target, horizontal);
    flowLayer
      .append("path")
      .attr("class", "activity-flow action-flow-edge")
      .attr("d", pathFromSections(sections) || fallback.path)
      .style("fill", "none")
      .style("stroke", ctx.theme.edge.default)
      .style("stroke-width", "2px")
      .style("marker-end", "url(#action-flow-arrow)");
    const label = truncateLabel(edge.label, 20);
    if (label && !["flow", "first", "bind"].includes(label.toLowerCase())) {
      const elkLabel = layout.edgeLabelsById.get(edge.id)?.[0];
      const labelFromSections = edgeLabelPositionFromSections(sections);
      const labelPosition = elkLabel
        ? { x: elkLabel.x + elkLabel.width / 2, y: elkLabel.y + elkLabel.height / 2 }
        : (labelFromSections ?? { x: fallback.labelX, y: fallback.labelY });
      const displayLabel = label.startsWith("[") ? label : `[${label}]`;
      if (elkLabel) {
        flowLayer
          .append("rect")
          .attr("x", elkLabel.x)
          .attr("y", elkLabel.y)
          .attr("width", elkLabel.width)
          .attr("height", elkLabel.height)
          .attr("rx", 3)
          .style("fill", ctx.theme.canvasBackground)
          .style("stroke", ctx.theme.edge.default)
          .style("stroke-width", "1px");
      }
      flowLayer
        .append("text")
        .attr("x", labelPosition.x)
        .attr("y", labelPosition.y + (elkLabel ? 3 : 0))
        .attr("text-anchor", "middle")
        .style("font-size", "10px")
        .style("fill", ctx.theme.textSecondary)
        .text(displayLabel);
    }
  }

  for (const node of ctx.prepared.nodes) {
    const position = layout.positions.get(node.id);
    if (!position) continue;
    const nodeGroup = drawActionNode(nodeLayer, node, position, ctx.theme);
    attachBehaviorNodeClick(nodeGroup, node, ctx.theme, renderOptions, ctx.root);
  }

  let minX = 0;
  let minY = 0;
  let maxX = ctx.width;
  let maxY = ctx.height;
  layout.positions.forEach((rect) => {
    minX = Math.min(minX, rect.x);
    minY = Math.min(minY, rect.y);
    maxX = Math.max(maxX, rect.x + rect.width);
    maxY = Math.max(maxY, rect.y + rect.height + 20);
  });
  return { minX: minX - 40, minY: minY - 40, maxX: maxX + 40, maxY: maxY + 40 };
}

export function addActionFlowMarkers(defs: d3.Selection<SVGDefsElement, unknown, null, undefined>, theme: DiagramTheme): void {
  defs.selectAll("#action-flow-arrow").remove();
  defs
    .append("marker")
    .attr("id", "action-flow-arrow")
    .attr("viewBox", "0 -5 10 10")
    .attr("refX", 8)
    .attr("refY", 0)
    .attr("markerWidth", 6)
    .attr("markerHeight", 6)
    .attr("orient", "auto")
    .append("path")
    .attr("d", "M0,-5L10,0L0,5")
    .style("fill", theme.edge.default);
}

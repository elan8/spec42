import * as d3 from "d3";
import type { PreparedNode } from "../prepare";
import type { DiagramTheme } from "../theme";
import {
  BehaviorSceneContext,
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
): void {
  const kind = nodeKind(node);
  const g = group
    .append("g")
    .attr("class", "activity-action action-flow-node")
    .attr("data-node-id", node.id)
    .attr("transform", `translate(${layout.x},${layout.y})`);

  if (isInitial(kind) || isFinal(kind)) {
    g.append("circle")
      .attr("cx", layout.width / 2)
      .attr("cy", layout.height / 2)
      .attr("r", layout.width / 2 - 2)
      .style("fill", isInitial(kind) ? theme.edge.default : theme.canvasBackground)
      .style("stroke", theme.nodeBorder)
      .style("stroke-width", "2px");
    if (isFinal(kind)) {
      g.append("circle")
        .attr("cx", layout.width / 2)
        .attr("cy", layout.height / 2)
        .attr("r", 10)
        .style("fill", theme.edge.default)
        .style("stroke", "none");
    }
  } else if (isDecision(kind)) {
    const cx = layout.width / 2;
    const cy = layout.height / 2;
    g.append("path")
      .attr("d", `M${cx},0 L${layout.width},${cy} L${cx},${layout.height} L0,${cy} Z`)
      .style("fill", theme.canvasBackground)
      .style("stroke", theme.edge.default)
      .style("stroke-width", "2px");
  } else if (isFork(kind)) {
    g.append("rect")
      .attr("width", layout.width)
      .attr("height", layout.height)
      .attr("rx", 3)
      .style("fill", theme.nodeBorder)
      .style("stroke", "none");
  } else {
    g.append("rect")
      .attr("width", layout.width)
      .attr("height", layout.height)
      .attr("rx", 8)
      .style("fill", theme.nodeFill)
      .style("stroke", theme.nodeBorder)
      .style("stroke-width", "2px");
    g.append("rect")
      .attr("width", layout.width)
      .attr("height", 6)
      .attr("rx", 8)
      .style("fill", theme.nodeBorder)
      .style("stroke", "none");
  }

  const labelY = isFork(kind) ? layout.height + 14 : layout.height / 2 + 4;
  g.append("text")
    .attr("x", layout.width / 2)
    .attr("y", labelY)
    .attr("text-anchor", "middle")
    .style("font-size", "12px")
    .style("font-weight", "600")
    .style("fill", theme.textPrimary)
    .text(truncateLabel(node.label, 24));
}

export async function renderActionFlowView(ctx: BehaviorSceneContext): Promise<{ minX: number; minY: number; maxX: number; maxY: number }> {
  const horizontal = String(ctx.prepared.meta?.layoutDirection ?? "").toLowerCase() === "horizontal";
  const layout = await layoutBehaviorGraph(ctx.prepared, { horizontal, mode: "action" });

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
      flowLayer
        .append("text")
        .attr("x", fallback.labelX)
        .attr("y", fallback.labelY)
        .attr("text-anchor", "middle")
        .style("font-size", "10px")
        .style("fill", ctx.theme.textSecondary)
        .text(label.startsWith("[") ? label : `[${label}]`);
    }
  }

  for (const node of ctx.prepared.nodes) {
    const position = layout.positions.get(node.id);
    if (!position) continue;
    drawActionNode(nodeLayer, node, position, ctx.theme);
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

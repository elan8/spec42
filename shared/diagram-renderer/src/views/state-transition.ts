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

function drawStateNode(
  group: d3.Selection<SVGGElement, unknown, null, undefined>,
  node: PreparedNode,
  layout: { x: number; y: number; width: number; height: number },
  theme: DiagramTheme,
): void {
  const kind = nodeKind(node);
  const g = group
    .append("g")
    .attr("class", "state-node state-transition-node")
    .attr("data-node-id", node.id)
    .attr("transform", `translate(${layout.x},${layout.y})`);

  if (kind.includes("initial")) {
    g.append("circle")
      .attr("cx", layout.width / 2)
      .attr("cy", layout.height / 2)
      .attr("r", layout.width / 2 - 2)
      .style("fill", theme.edge.default)
      .style("stroke", theme.nodeBorder)
      .style("stroke-width", "2px");
  } else if (kind.includes("final")) {
    g.append("circle")
      .attr("cx", layout.width / 2)
      .attr("cy", layout.height / 2)
      .attr("r", layout.width / 2 - 2)
      .style("fill", theme.canvasBackground)
      .style("stroke", theme.nodeBorder)
      .style("stroke-width", "2px");
    g.append("circle")
      .attr("cx", layout.width / 2)
      .attr("cy", layout.height / 2)
      .attr("r", 10)
      .style("fill", theme.edge.default)
      .style("stroke", "none");
  } else {
    g.append("rect")
      .attr("width", layout.width)
      .attr("height", layout.height)
      .attr("rx", kind.includes("composite") ? 10 : 14)
      .style("fill", theme.nodeFill)
      .style("stroke", theme.nodeBorder)
      .style("stroke-width", "2px");
    g.append("text")
      .attr("x", layout.width / 2)
      .attr("y", 22)
      .attr("text-anchor", "middle")
      .style("font-size", "12px")
      .style("font-weight", "700")
      .style("fill", theme.textPrimary)
      .text(truncateLabel(node.label, 28));
  }
}

export async function renderStateTransitionView(ctx: BehaviorSceneContext): Promise<{ minX: number; minY: number; maxX: number; maxY: number }> {
  const layoutMode = String(ctx.prepared.meta?.layoutDirection ?? "horizontal").toLowerCase();
  const horizontal = layoutMode !== "vertical" && layoutMode !== "force";
  const layout = await layoutBehaviorGraph(ctx.prepared, { horizontal, mode: "state" });

  ctx.root
    .append("text")
    .attr("x", 24)
    .attr("y", 28)
    .style("font-size", "14px")
    .style("font-weight", "700")
    .style("fill", ctx.theme.textPrimary)
    .text(ctx.prepared.title || "State Transition");

  const edgeLayer = ctx.root.append("g").attr("class", "state-transitions");
  const nodeLayer = ctx.root.append("g").attr("class", "state-nodes");

  for (const edge of ctx.prepared.edges) {
    const source = layout.positions.get(edge.source);
    const target = layout.positions.get(edge.target);
    if (!source || !target) continue;
    const sections = layout.edgeSectionsById.get(edge.id);
    const fallback = fallbackEdgePath(source, target, horizontal);
    edgeLayer
      .append("path")
      .attr("class", "state-transition-edge")
      .attr("d", pathFromSections(sections) || fallback.path)
      .style("fill", "none")
      .style("stroke", ctx.theme.edge.default)
      .style("stroke-width", "2px")
      .style("marker-end", "url(#state-transition-arrow)");
    const label = truncateLabel(edge.label, 24);
    if (label) {
      edgeLayer
        .append("text")
        .attr("x", fallback.labelX)
        .attr("y", fallback.labelY)
        .attr("text-anchor", "middle")
        .style("font-size", "10px")
        .style("fill", ctx.theme.textSecondary)
        .text(label);
    }
  }

  for (const node of ctx.prepared.nodes) {
    const position = layout.positions.get(node.id);
    if (!position) continue;
    drawStateNode(nodeLayer, node, position, ctx.theme);
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

export function addStateTransitionMarkers(defs: d3.Selection<SVGDefsElement, unknown, null, undefined>, theme: DiagramTheme): void {
  defs.selectAll("#state-transition-arrow").remove();
  defs
    .append("marker")
    .attr("id", "state-transition-arrow")
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

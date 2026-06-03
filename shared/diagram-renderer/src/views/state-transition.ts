import * as d3 from "d3";
import type { PreparedNode } from "../prepare";
import type { DiagramTheme } from "../theme";
import { attachBehaviorNodeClick } from "./behavior-interaction";
import {
  BehaviorSceneContext,
  buildSelfLoopPath,
  edgeLabelPositionFromSections,
  fallbackEdgePath,
  layoutBehaviorGraph,
  nodeKind,
  pathFromSections,
  truncateLabel,
} from "./behavior-common";

function transitionDisplayLabel(label: string): string {
  const trimmed = label.trim();
  if (!trimmed || trimmed.toLowerCase() === "entry") return "";
  return trimmed;
}

function drawStateNode(
  group: d3.Selection<SVGGElement, unknown, null, undefined>,
  node: PreparedNode,
  layout: { x: number; y: number; width: number; height: number },
  theme: DiagramTheme,
): d3.Selection<SVGGElement, unknown, null, undefined> {
  const kind = nodeKind(node);
  const attrs = (node.attributes ?? {}) as Record<string, unknown>;
  const regions = Array.isArray(attrs.regions) ? attrs.regions : Array.isArray(attrs.children) ? attrs.children : [];
  const entry = String(attrs.entry ?? attrs.entryAction ?? "").trim();
  const doAction = String(attrs.do ?? attrs.doAction ?? "").trim();
  const exit = String(attrs.exit ?? attrs.exitAction ?? "").trim();
  const isComposite = kind.includes("composite") || regions.length > 0;
  const isTerminate = kind.includes("terminate");
  const g = group
    .append("g")
    .attr("class", "state-node state-transition-node")
    .attr("data-node-id", node.id)
    .attr("transform", `translate(${layout.x},${layout.y})`);

  if (kind.includes("initial")) {
    g.append("circle")
      .attr("class", "node-background")
      .attr("data-original-stroke", theme.nodeBorder)
      .attr("data-original-width", "2px")
      .attr("cx", layout.width / 2)
      .attr("cy", layout.height / 2)
      .attr("r", layout.width / 2 - 2)
      .style("fill", theme.edge.default)
      .style("stroke", theme.nodeBorder)
      .style("stroke-width", "2px");
  } else if (kind.includes("final") || isTerminate) {
    g.append("circle")
      .attr("class", "node-background")
      .attr("data-original-stroke", theme.nodeBorder)
      .attr("data-original-width", "2px")
      .attr("cx", layout.width / 2)
      .attr("cy", layout.height / 2)
      .attr("r", layout.width / 2 - 2)
      .style("fill", theme.canvasBackground)
      .style("stroke", theme.nodeBorder)
      .style("stroke-width", "2px");
    if (isTerminate) {
      g.append("path")
        .attr("class", "terminate-state-x")
        .attr("d", `M${layout.width / 2 - 9},${layout.height / 2 - 9} L${layout.width / 2 + 9},${layout.height / 2 + 9} M${layout.width / 2 + 9},${layout.height / 2 - 9} L${layout.width / 2 - 9},${layout.height / 2 + 9}`)
        .style("stroke", theme.edge.default)
        .style("stroke-width", "2px");
    } else {
      g.append("circle")
        .attr("cx", layout.width / 2)
        .attr("cy", layout.height / 2)
        .attr("r", 10)
        .style("fill", theme.edge.default)
        .style("stroke", "none");
    }
  } else {
    g.append("rect")
      .attr("class", "node-background")
      .attr("data-original-stroke", theme.nodeBorder)
      .attr("data-original-width", "2px")
      .attr("width", layout.width)
      .attr("height", layout.height)
      .attr("rx", isComposite ? 10 : 14)
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
    const actionLines = [
      entry ? `entry / ${entry}` : "",
      doAction ? `do / ${doAction}` : "",
      exit ? `exit / ${exit}` : "",
    ].filter(Boolean);
    if (actionLines.length > 0 || isComposite) {
      g.append("line")
        .attr("class", "state-compartment-divider")
        .attr("x1", 0)
        .attr("x2", layout.width)
        .attr("y1", 34)
        .attr("y2", 34)
        .style("stroke", theme.nodeBorder)
        .style("stroke-width", "1px");
    }
    actionLines.forEach((line, index) => {
      g.append("text")
        .attr("class", "state-action-compartment")
        .attr("x", 12)
        .attr("y", 54 + index * 16)
        .style("font-size", "10px")
        .style("fill", theme.textSecondary)
        .text(truncateLabel(line, 34));
    });
    if (isComposite) {
      const regionTop = Math.max(80, 52 + actionLines.length * 16);
      const regionHeight = Math.max(32, (layout.height - regionTop - 14) / Math.max(1, regions.length || 1));
      const regionList = regions.length > 0 ? regions : [{ name: "region" }];
      regionList.slice(0, 4).forEach((region, index) => {
        const item = region && typeof region === "object" ? region as Record<string, unknown> : { name: String(region) };
        const y = regionTop + index * regionHeight;
        g.append("rect")
          .attr("class", "state-region")
          .attr("x", 12)
          .attr("y", y)
          .attr("width", layout.width - 24)
          .attr("height", Math.max(24, regionHeight - 8))
          .attr("rx", 5)
          .style("fill", "none")
          .style("stroke", theme.nodeBorder)
          .style("stroke-dasharray", "4,3");
        g.append("text")
          .attr("class", "state-region-label")
          .attr("x", 20)
          .attr("y", y + 17)
          .style("font-size", "9px")
          .style("fill", theme.textSecondary)
          .text(truncateLabel(String(item.name ?? item.label ?? `region ${index + 1}`), 28));
      });
    }
  }

  return g;
}

export async function renderStateTransitionView(ctx: BehaviorSceneContext): Promise<{ minX: number; minY: number; maxX: number; maxY: number }> {
  const layoutMode = String(ctx.prepared.meta?.layoutDirection ?? "horizontal").toLowerCase();
  const horizontal = layoutMode !== "vertical" && layoutMode !== "force";
  const layout = await layoutBehaviorGraph(ctx.prepared, { horizontal, mode: "state" });
  const renderOptions = ctx.options ?? {};

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
    const selfLoop = Boolean(edge.attributes?.selfLoop) || edge.source === edge.target;
    const fallback = selfLoop ? buildSelfLoopPath(source) : fallbackEdgePath(source, target, horizontal);
    const path = selfLoop ? fallback.path : (pathFromSections(sections) || fallback.path);
    edgeLayer
      .append("path")
      .attr("class", "state-transition-edge")
      .attr("d", path)
      .style("fill", "none")
      .style("stroke", ctx.theme.edge.default)
      .style("stroke-width", "2px")
      .style("marker-end", "url(#state-transition-arrow)");

    const label = transitionDisplayLabel(edge.label);
    if (label) {
      const elkLabel = layout.edgeLabelsById.get(edge.id)?.[0];
      const labelFromSections = edgeLabelPositionFromSections(sections);
      const labelPosition = elkLabel
        ? { x: elkLabel.x + elkLabel.width / 2, y: elkLabel.y + elkLabel.height / 2 }
        : (labelFromSections ?? { x: fallback.labelX, y: fallback.labelY });
      const labelWidth = elkLabel?.width ?? Math.max(42, label.length * 6 + 10);
      const labelHeight = elkLabel?.height ?? 18;

      edgeLayer
        .append("rect")
        .attr("x", elkLabel ? elkLabel.x : labelPosition.x - labelWidth / 2)
        .attr("y", elkLabel ? elkLabel.y : labelPosition.y - 10)
        .attr("width", labelWidth)
        .attr("height", labelHeight)
        .attr("rx", 4)
        .style("fill", ctx.theme.canvasBackground)
        .style("stroke", ctx.theme.edge.default)
        .style("stroke-width", "1px");

      edgeLayer
        .append("text")
        .attr("x", labelPosition.x)
        .attr("y", labelPosition.y + 3)
        .attr("text-anchor", "middle")
        .style("font-size", "10px")
        .style("font-weight", "500")
        .style("fill", ctx.theme.edge.default)
        .text(label);
    }
  }

  for (const node of ctx.prepared.nodes) {
    const position = layout.positions.get(node.id);
    if (!position) continue;
    const nodeGroup = drawStateNode(nodeLayer, node, position, ctx.theme);
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

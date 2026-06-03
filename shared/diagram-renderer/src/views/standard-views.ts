import * as d3 from "d3";
import type { PreparedNode } from "../prepare";
import type { DiagramTheme } from "../theme";
import { attachBehaviorNodeClick } from "./behavior-interaction";
import type { BehaviorSceneContext } from "./behavior-common";
import { truncateLabel } from "./behavior-common";

function asRecord(value: unknown): Record<string, unknown> {
  return value && typeof value === "object" ? (value as Record<string, unknown>) : {};
}

function asArray(value: unknown): unknown[] {
  return Array.isArray(value) ? value : [];
}

function asString(value: unknown, fallback = ""): string {
  if (typeof value === "string") return value;
  if (typeof value === "number" || typeof value === "boolean") return String(value);
  return fallback;
}

function drawProvisionalBadge(
  root: d3.Selection<SVGGElement, unknown, null, undefined>,
  theme: DiagramTheme,
  label = "provisional SysML notation",
): void {
  const badge = root.append("g").attr("class", "provisional-view-badge");
  badge
    .append("rect")
    .attr("x", 22)
    .attr("y", 42)
    .attr("width", 176)
    .attr("height", 24)
    .attr("rx", 5)
    .style("fill", theme.canvasBackground)
    .style("stroke", theme.edge.default)
    .style("stroke-dasharray", "4,3");
  badge
    .append("text")
    .attr("x", 34)
    .attr("y", 58)
    .style("font-size", "10px")
    .style("fill", theme.textSecondary)
    .text(label);
}

function nodeFromMeta(row: Record<string, unknown>, fallback: PreparedNode[]): PreparedNode | undefined {
  const id = asString(row.id);
  return fallback.find((node) => node.id === id || node.label === asString(row.label ?? row.name));
}

export function renderBrowserView(ctx: BehaviorSceneContext): { minX: number; minY: number; maxX: number; maxY: number } {
  const rows = asArray(ctx.prepared.meta?.rows).map(asRecord);
  const sourceRows = rows.length > 0 ? rows : ctx.prepared.nodes.map((node) => ({ id: node.id, label: node.label, kind: node.kind }));
  const rowHeight = 28;
  const left = 52;
  const top = 88;
  const width = Math.max(520, Math.min(920, ctx.width - 120));
  ctx.root
    .append("text")
    .attr("x", 24)
    .attr("y", 28)
    .style("font-size", "14px")
    .style("font-weight", "700")
    .style("fill", ctx.theme.textPrimary)
    .text(ctx.prepared.title || "Browser View");
  drawProvisionalBadge(ctx.root, ctx.theme);

  const layer = ctx.root.append("g").attr("class", "browser-view-rows");
  sourceRows.forEach((row, index) => {
    const y = top + index * rowHeight;
    const qualified = asString(row.qualifiedName);
    const depth = Math.max(0, qualified.split("::").filter(Boolean).length - 1);
    const preparedNode = nodeFromMeta(row, ctx.prepared.nodes);
    const item = layer
      .append("g")
      .attr("class", "browser-row")
      .attr("data-node-id", preparedNode?.id ?? asString(row.id, `browser-row-${index}`))
      .attr("transform", `translate(${left},${y})`);
    item
      .append("rect")
      .attr("class", "node-background")
      .attr("data-original-stroke", ctx.theme.nodeBorder)
      .attr("data-original-width", "1px")
      .attr("width", width)
      .attr("height", rowHeight - 3)
      .attr("rx", 4)
      .style("fill", index % 2 === 0 ? ctx.theme.nodeFill : ctx.theme.canvasBackground)
      .style("stroke", ctx.theme.nodeBorder)
      .style("stroke-width", "1px")
      .style("opacity", 0.9);
    item
      .append("text")
      .attr("x", 14 + depth * 16)
      .attr("y", 18)
      .style("font-size", "11px")
      .style("font-weight", "600")
      .style("fill", ctx.theme.textPrimary)
      .text(truncateLabel(asString(row.label ?? row.name ?? row.id, "Unnamed"), 48));
    item
      .append("text")
      .attr("x", width - 14)
      .attr("y", 18)
      .attr("text-anchor", "end")
      .style("font-size", "10px")
      .style("fill", ctx.theme.textSecondary)
      .text(truncateLabel(asString(row.kind, "element"), 24));
    if (preparedNode) {
      attachBehaviorNodeClick(item, preparedNode, ctx.theme, ctx.options ?? {}, ctx.root);
    }
  });

  return { minX: 0, minY: 0, maxX: left + width + 80, maxY: top + sourceRows.length * rowHeight + 80 };
}

export function renderGridView(ctx: BehaviorSceneContext): { minX: number; minY: number; maxX: number; maxY: number } {
  const cells = asArray(ctx.prepared.meta?.cells).map(asRecord);
  const rows = cells.length > 0 ? cells : ctx.prepared.nodes.map((node) => asRecord(node.attributes));
  const left = 52;
  const top = 92;
  const columns = [
    { key: "name", label: "Name", width: 220 },
    { key: "kind", label: "Kind", width: 150 },
    { key: "attributeCount", label: "Attrs", width: 80 },
    { key: "partCount", label: "Parts", width: 80 },
    { key: "portCount", label: "Ports", width: 80 },
  ];
  const tableWidth = columns.reduce((sum, column) => sum + column.width, 0);
  const rowHeight = 30;
  ctx.root
    .append("text")
    .attr("x", 24)
    .attr("y", 28)
    .style("font-size", "14px")
    .style("font-weight", "700")
    .style("fill", ctx.theme.textPrimary)
    .text(ctx.prepared.title || "Grid View");
  drawProvisionalBadge(ctx.root, ctx.theme);

  const table = ctx.root.append("g").attr("class", "grid-view-table").attr("transform", `translate(${left},${top})`);
  let x = 0;
  columns.forEach((column) => {
    table
      .append("rect")
      .attr("class", "grid-header-cell")
      .attr("x", x)
      .attr("width", column.width)
      .attr("height", rowHeight)
      .style("fill", ctx.theme.nodeBorder)
      .style("stroke", ctx.theme.nodeBorder);
    table
      .append("text")
      .attr("x", x + 10)
      .attr("y", 20)
      .style("font-size", "11px")
      .style("font-weight", "700")
      .style("fill", ctx.theme.canvasBackground)
      .text(column.label);
    x += column.width;
  });
  rows.forEach((row, rowIndex) => {
    x = 0;
    const preparedNode = nodeFromMeta(row, ctx.prepared.nodes);
    const group = table
      .append("g")
      .attr("class", "grid-row")
      .attr("data-node-id", preparedNode?.id ?? asString(row.id, `grid-row-${rowIndex}`))
      .attr("transform", `translate(0,${(rowIndex + 1) * rowHeight})`);
    columns.forEach((column) => {
      group
        .append("rect")
        .attr("class", "grid-cell")
        .attr("x", x)
        .attr("width", column.width)
        .attr("height", rowHeight)
        .style("fill", rowIndex % 2 === 0 ? ctx.theme.nodeFill : ctx.theme.canvasBackground)
        .style("stroke", ctx.theme.nodeBorder)
        .style("stroke-width", "1px");
      group
        .append("text")
        .attr("x", x + 10)
        .attr("y", 20)
        .style("font-size", "10px")
        .style("fill", ctx.theme.textPrimary)
        .text(truncateLabel(asString(row[column.key]), column.width > 100 ? 28 : 8));
      x += column.width;
    });
    if (preparedNode) {
      attachBehaviorNodeClick(group, preparedNode, ctx.theme, ctx.options ?? {}, ctx.root);
    }
  });
  return { minX: 0, minY: 0, maxX: left + tableWidth + 80, maxY: top + (rows.length + 2) * rowHeight + 80 };
}

export function renderGeometryView(ctx: BehaviorSceneContext): { minX: number; minY: number; maxX: number; maxY: number } {
  const elements = asArray(ctx.prepared.meta?.elements).map(asRecord);
  const nodes = elements.length > 0 ? elements : ctx.prepared.nodes.map((node) => ({ id: node.id, label: node.label, kind: node.kind }));
  const centerX = Math.max(280, ctx.width / 2);
  const centerY = Math.max(240, ctx.height / 2 + 30);
  const radius = Math.max(130, Math.min(300, nodes.length * 22));
  ctx.root
    .append("text")
    .attr("x", 24)
    .attr("y", 28)
    .style("font-size", "14px")
    .style("font-weight", "700")
    .style("fill", ctx.theme.textPrimary)
    .text(ctx.prepared.title || "Geometry View");
  drawProvisionalBadge(ctx.root, ctx.theme, "provisional geometry notation");

  const layer = ctx.root.append("g").attr("class", "geometry-view-scene");
  layer
    .append("circle")
    .attr("cx", centerX)
    .attr("cy", centerY)
    .attr("r", radius + 44)
    .style("fill", "none")
    .style("stroke", ctx.theme.frame.stroke)
    .style("stroke-dasharray", "8,6");
  nodes.forEach((node, index) => {
    const angle = nodes.length <= 1 ? -Math.PI / 2 : (index / nodes.length) * Math.PI * 2 - Math.PI / 2;
    const x = centerX + Math.cos(angle) * radius;
    const y = centerY + Math.sin(angle) * radius;
    const preparedNode = nodeFromMeta(node, ctx.prepared.nodes);
    const item = layer
      .append("g")
      .attr("class", "geometry-object")
      .attr("data-node-id", preparedNode?.id ?? asString(node.id, `geometry-node-${index}`))
      .attr("transform", `translate(${x},${y})`);
    item
      .append("rect")
      .attr("class", "node-background")
      .attr("data-original-stroke", ctx.theme.nodeBorder)
      .attr("data-original-width", "1.5px")
      .attr("x", -54)
      .attr("y", -24)
      .attr("width", 108)
      .attr("height", 48)
      .attr("rx", 6)
      .style("fill", ctx.theme.nodeFill)
      .style("stroke", ctx.theme.nodeBorder)
      .style("stroke-width", "1.5px");
    item
      .append("text")
      .attr("text-anchor", "middle")
      .attr("y", -2)
      .style("font-size", "10px")
      .style("font-weight", "700")
      .style("fill", ctx.theme.textPrimary)
      .text(truncateLabel(asString(node.label ?? node.name ?? node.id), 16));
    item
      .append("text")
      .attr("text-anchor", "middle")
      .attr("y", 14)
      .style("font-size", "8px")
      .style("fill", ctx.theme.textSecondary)
      .text(truncateLabel(asString(node.kind, "element"), 18));
    if (preparedNode) {
      attachBehaviorNodeClick(item, preparedNode, ctx.theme, ctx.options ?? {}, ctx.root);
    }
  });
  return {
    minX: centerX - radius - 120,
    minY: centerY - radius - 120,
    maxX: centerX + radius + 120,
    maxY: centerY + radius + 120,
  };
}

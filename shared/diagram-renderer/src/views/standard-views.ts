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

function shortMatrixLabel(id: string): string {
  const segments = id.split("::").filter(Boolean);
  return truncateLabel(segments[segments.length - 1] ?? id, 10);
}

export function renderBrowserView(ctx: BehaviorSceneContext): { minX: number; minY: number; maxX: number; maxY: number } {
  const rows = asArray(ctx.prepared.meta?.rows).map(asRecord);
  const sourceRows = rows.length > 0 ? rows : ctx.prepared.nodes.map((node) => ({ id: node.id, label: node.label, kind: node.kind }));
  const hierarchyLayout = Boolean(ctx.prepared.meta?.hierarchyLayout);
  const rowHeight = 28;
  const left = 52;
  const top = 88;
  const width = Math.max(520, Math.min(920, ctx.width - 120));
  const collapsed = new Set<string>();

  ctx.root
    .append("text")
    .attr("x", 24)
    .attr("y", 28)
    .style("font-size", "14px")
    .style("font-weight", "700")
    .style("fill", ctx.theme.textPrimary)
    .text(ctx.prepared.title || "Browser View");
  if (!hierarchyLayout) {
    drawProvisionalBadge(ctx.root, ctx.theme);
  }

  const layer = ctx.root.append("g").attr("class", "browser-view-rows");

  const isRowVisible = (row: Record<string, unknown>, index: number): boolean => {
    if (!hierarchyLayout) return true;
    const parentId = asString(row.parentId);
    if (!parentId) return true;
    for (let cursor = index - 1; cursor >= 0; cursor -= 1) {
      const ancestor = asRecord(sourceRows[cursor]);
      if (asString(ancestor.id) !== parentId) continue;
      if (!isRowVisible(ancestor, cursor) || collapsed.has(parentId)) {
        return false;
      }
      return true;
    }
    return !collapsed.has(parentId);
  };

  const redraw = (): void => {
    layer.selectAll("*").remove();
    let visibleIndex = 0;
    sourceRows.forEach((row, index) => {
      if (!isRowVisible(row, index)) return;
      const y = top + visibleIndex * rowHeight;
      visibleIndex += 1;
      const depth = hierarchyLayout ? Number(row.depth ?? 0) : Math.max(0, asString(row.qualifiedName).split("::").filter(Boolean).length - 1);
      const hasChildren = Boolean(row.hasChildren);
      const preparedNode = nodeFromMeta(row, ctx.prepared.nodes);
      const rowId = preparedNode?.id ?? asString(row.id, `browser-row-${index}`);
      const item = layer
        .append("g")
        .attr("class", "browser-row")
        .attr("data-node-id", rowId)
        .attr("transform", `translate(${left},${y})`);
      item
        .append("rect")
        .attr("class", "node-background")
        .attr("data-original-stroke", ctx.theme.nodeBorder)
        .attr("data-original-width", "1px")
        .attr("width", width)
        .attr("height", rowHeight - 3)
        .attr("rx", 4)
        .style("fill", visibleIndex % 2 === 0 ? ctx.theme.nodeFill : ctx.theme.canvasBackground)
        .style("stroke", ctx.theme.nodeBorder)
        .style("stroke-width", "1px")
        .style("opacity", 0.9);
      if (hasChildren) {
        const toggle = item
          .append("text")
          .attr("x", 8 + depth * 16)
          .attr("y", 18)
          .attr("class", "browser-toggle")
          .style("font-size", "11px")
          .style("font-weight", "700")
          .style("fill", ctx.theme.textSecondary)
          .style("cursor", "pointer")
          .text(collapsed.has(rowId) ? "▸" : "▾");
        toggle.on("click", (event) => {
          event.stopPropagation();
          if (collapsed.has(rowId)) {
            collapsed.delete(rowId);
          } else {
            collapsed.add(rowId);
          }
          redraw();
        });
      }
      item
        .append("text")
        .attr("x", 14 + depth * 16 + (hasChildren ? 12 : 0))
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
  };

  redraw();
  const visibleCount = hierarchyLayout
    ? sourceRows.filter((row, index) => isRowVisible(row, index)).length
    : sourceRows.length;
  return { minX: 0, minY: 0, maxX: left + width + 80, maxY: top + visibleCount * rowHeight + 80 };
}

export function renderGridView(ctx: BehaviorSceneContext): { minX: number; minY: number; maxX: number; maxY: number } {
  const relationshipMatrix = Boolean(ctx.prepared.meta?.relationshipMatrix);
  if (relationshipMatrix) {
    return renderRelationshipMatrix(ctx);
  }

  const cells = asArray(ctx.prepared.meta?.cells).map(asRecord);
  const rows = cells.length > 0 ? cells : ctx.prepared.nodes.map((node) => asRecord(node.attributes));
  const left = 52;
  const top = 92;
  const traceabilityTable = Boolean(ctx.prepared.meta?.traceabilityTable);
  const columns = traceabilityTable
    ? [
        { key: "name", label: "Name", width: 240 },
        { key: "kind", label: "Kind", width: 130 },
        { key: "package", label: "Package", width: 170 },
        { key: "linkCount", label: "Links", width: 70 },
      ]
    : [
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
  if (Boolean(ctx.prepared.meta?.provisional)) {
    drawProvisionalBadge(ctx.root, ctx.theme);
  }

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

function renderRelationshipMatrix(ctx: BehaviorSceneContext): { minX: number; minY: number; maxX: number; maxY: number } {
  const rowIds = asArray(ctx.prepared.meta?.matrixRowIds).map((value) => asString(value)).filter(Boolean);
  const colIds = asArray(ctx.prepared.meta?.matrixColIds).map((value) => asString(value)).filter(Boolean);
  const matrixCells = asArray(ctx.prepared.meta?.matrixCells).map(asRecord);
  const cellSize = 34;
  const headerSize = 120;
  const left = 180;
  const top = 92;

  ctx.root
    .append("text")
    .attr("x", 24)
    .attr("y", 28)
    .style("font-size", "14px")
    .style("font-weight", "700")
    .style("fill", ctx.theme.textPrimary)
    .text(ctx.prepared.title || "Relationship Matrix");

  const layer = ctx.root.append("g").attr("class", "grid-relationship-matrix").attr("transform", `translate(${left},${top})`);
  colIds.forEach((colId, colIndex) => {
    layer
      .append("text")
      .attr("x", headerSize + colIndex * cellSize + cellSize / 2)
      .attr("y", 16)
      .attr("text-anchor", "middle")
      .attr("transform", `rotate(-35, ${headerSize + colIndex * cellSize + cellSize / 2}, 16)`)
      .style("font-size", "9px")
      .style("fill", ctx.theme.textSecondary)
      .text(shortMatrixLabel(colId));
  });
  rowIds.forEach((rowId, rowIndex) => {
    layer
      .append("text")
      .attr("x", headerSize - 8)
      .attr("y", headerSize + rowIndex * cellSize + cellSize / 2 + 4)
      .attr("text-anchor", "end")
      .style("font-size", "10px")
      .style("fill", ctx.theme.textPrimary)
      .text(shortMatrixLabel(rowId));
    colIds.forEach((colId, colIndex) => {
      const cell = matrixCells.find(
        (entry) => asString(entry.source) === rowId && asString(entry.target) === colId,
      );
      const present = Boolean(cell?.present);
      const x = headerSize + colIndex * cellSize;
      const y = headerSize + rowIndex * cellSize;
      layer
        .append("rect")
        .attr("x", x)
        .attr("y", y)
        .attr("width", cellSize - 2)
        .attr("height", cellSize - 2)
        .style("fill", present ? ctx.theme.nodeFill : ctx.theme.canvasBackground)
        .style("stroke", ctx.theme.nodeBorder)
        .style("stroke-width", "1px");
      if (present) {
        layer
          .append("text")
          .attr("x", x + (cellSize - 2) / 2)
          .attr("y", y + (cellSize - 2) / 2 + 4)
          .attr("text-anchor", "middle")
          .style("font-size", "12px")
          .style("font-weight", "700")
          .style("fill", ctx.theme.edge.default)
          .text("●");
      }
    });
  });

  const width = headerSize + colIds.length * cellSize + 40;
  const height = headerSize + rowIds.length * cellSize + 40;
  return { minX: 0, minY: 0, maxX: left + width, maxY: top + height };
}

export function renderGeometryView(ctx: BehaviorSceneContext): { minX: number; minY: number; maxX: number; maxY: number } {
  const elements = asArray(ctx.prepared.meta?.elements).map(asRecord);
  const nodes = elements.length > 0 ? elements : ctx.prepared.nodes.map((node) => ({ id: node.id, label: node.label, kind: node.kind }));
  const geometryMode = asString(ctx.prepared.meta?.geometryMode, "2d");
  const geometryProjection = asString(ctx.prepared.meta?.geometryProjection, "orthographic");
  const left = 64;
  const top = 88;
  const cellWidth = 128;
  const cellHeight = 72;
  const columns = Math.max(1, Math.ceil(Math.sqrt(nodes.length)));

  ctx.root
    .append("text")
    .attr("x", 24)
    .attr("y", 28)
    .style("font-size", "14px")
    .style("font-weight", "700")
    .style("fill", ctx.theme.textPrimary)
    .text(ctx.prepared.title || "Geometry View");
  if (Boolean(ctx.prepared.meta?.provisional)) {
    drawProvisionalBadge(ctx.root, ctx.theme, `${geometryMode} ${geometryProjection} preview`);
  }

  const layer = ctx.root.append("g").attr("class", "geometry-view-scene").attr("transform", `translate(${left},${top})`);
  layer
    .append("rect")
    .attr("width", columns * cellWidth + 24)
    .attr("height", Math.ceil(nodes.length / columns) * cellHeight + 24)
    .attr("rx", 8)
    .style("fill", "none")
    .style("stroke", ctx.theme.frame.stroke)
    .style("stroke-dasharray", "8,6");

  nodes.forEach((node, index) => {
    const col = index % columns;
    const row = Math.floor(index / columns);
    const x = col * cellWidth + 12;
    const y = row * cellHeight + 12;
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
      .attr("width", cellWidth - 20)
      .attr("height", cellHeight - 16)
      .attr("rx", 6)
      .style("fill", ctx.theme.nodeFill)
      .style("stroke", ctx.theme.nodeBorder)
      .style("stroke-width", "1.5px");
    item
      .append("text")
      .attr("x", (cellWidth - 20) / 2)
      .attr("y", 24)
      .attr("text-anchor", "middle")
      .style("font-size", "10px")
      .style("font-weight", "700")
      .style("fill", ctx.theme.textPrimary)
      .text(truncateLabel(asString(node.label ?? node.name ?? node.id), 16));
    item
      .append("text")
      .attr("x", (cellWidth - 20) / 2)
      .attr("y", 42)
      .attr("text-anchor", "middle")
      .style("font-size", "8px")
      .style("fill", ctx.theme.textSecondary)
      .text(truncateLabel(asString(node.kind, "element"), 18));
    if (preparedNode) {
      attachBehaviorNodeClick(item, preparedNode, ctx.theme, ctx.options ?? {}, ctx.root);
    }
  });

  const width = columns * cellWidth + 80;
  const height = Math.ceil(nodes.length / columns) * cellHeight + 120;
  return { minX: 0, minY: 0, maxX: left + width, maxY: top + height };
}

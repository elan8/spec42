import * as d3 from "d3";
import type { DiagramTheme } from "../theme";
import { BehaviorSceneContext, truncateLabel } from "./behavior-common";

const HEADER_Y = 64;
const LIFELINE_TOP = 118;
const LIFELINE_GAP = 220;
const MESSAGE_GAP = 78;
const LIFELINE_BOX_WIDTH = 132;
const LIFELINE_BOX_HEIGHT = 38;

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

function messageRow(order: number): number {
  return LIFELINE_TOP + 58 + (Math.max(1, order) - 1) * MESSAGE_GAP;
}

function addSequenceMarkers(defs: d3.Selection<SVGDefsElement, unknown, null, undefined>, theme: DiagramTheme): void {
  defs.selectAll("#sequence-arrow-sync").remove();
  defs
    .append("marker")
    .attr("id", "sequence-arrow-sync")
    .attr("viewBox", "0 -5 10 10")
    .attr("refX", 9)
    .attr("refY", 0)
    .attr("markerWidth", 8)
    .attr("markerHeight", 8)
    .attr("orient", "auto")
    .append("path")
    .attr("d", "M0,-5L10,0L0,5")
    .style("fill", theme.edge.default);
}

export function renderSequenceView(ctx: BehaviorSceneContext): { minX: number; minY: number; maxX: number; maxY: number } {
  const diagram = asRecord(ctx.prepared.meta?.sequenceDiagram);
  const lifelines = asArray(diagram.lifelines).map(asRecord);
  const messages = asArray(diagram.messages)
    .map(asRecord)
    .sort((a, b) => Number(a.order ?? 0) - Number(b.order ?? 0));

  ctx.root
    .append("text")
    .attr("x", ctx.width / 2)
    .attr("y", 32)
    .attr("text-anchor", "middle")
    .style("font-size", "14px")
    .style("font-weight", "700")
    .style("fill", ctx.theme.textPrimary)
    .text(ctx.prepared.title || "Sequence");

  if (lifelines.length === 0 || messages.length === 0) {
    ctx.root
      .append("text")
      .attr("x", ctx.width / 2)
      .attr("y", ctx.height / 2)
      .attr("text-anchor", "middle")
      .style("fill", ctx.theme.textSecondary)
      .text("No sequence lifelines or messages in payload");
    return { minX: 0, minY: 0, maxX: ctx.width, maxY: ctx.height };
  }

  const xOffset = Math.max(80, (ctx.width - ((Math.max(0, lifelines.length - 1) * LIFELINE_GAP) + LIFELINE_BOX_WIDTH)) / 2);
  const lifelineX = new Map<string, number>();
  lifelines.forEach((lifeline, index) => {
    lifelineX.set(asString(lifeline.id ?? lifeline.name), xOffset + index * LIFELINE_GAP);
  });

  const lastMessageY = messages.length ? messageRow(Number(messages[messages.length - 1].order ?? messages.length)) : LIFELINE_TOP + 100;
  const lifelineBottom = lastMessageY + 140;

  const lifelineLayer = ctx.root.append("g").attr("class", "sequence-lifelines");
  for (const lifeline of lifelines) {
    const id = asString(lifeline.id ?? lifeline.name);
    const x = lifelineX.get(id) ?? xOffset;
    const label = truncateLabel(asString(lifeline.name ?? lifeline.label ?? id), 18);
    lifelineLayer
      .append("rect")
      .attr("x", x - LIFELINE_BOX_WIDTH / 2)
      .attr("y", HEADER_Y)
      .attr("width", LIFELINE_BOX_WIDTH)
      .attr("height", LIFELINE_BOX_HEIGHT)
      .attr("rx", 6)
      .style("fill", ctx.theme.nodeFill)
      .style("stroke", ctx.theme.nodeBorder)
      .style("stroke-width", "1.5px");
    lifelineLayer
      .append("text")
      .attr("x", x)
      .attr("y", HEADER_Y + 24)
      .attr("text-anchor", "middle")
      .style("font-size", "11px")
      .style("fill", ctx.theme.textPrimary)
      .text(label);
    lifelineLayer
      .append("line")
      .attr("x1", x)
      .attr("y1", LIFELINE_TOP)
      .attr("x2", x)
      .attr("y2", lifelineBottom)
      .style("stroke", ctx.theme.nodeBorder)
      .style("stroke-dasharray", "6,4");
  }

  const messageLayer = ctx.root.append("g").attr("class", "sequence-messages");
  for (const message of messages) {
    const sourceId = asString(message.source ?? message.from);
    const targetId = asString(message.target ?? message.to);
    const sourceX = lifelineX.get(sourceId);
    const targetX = lifelineX.get(targetId);
    if (sourceX == null || targetX == null) continue;
    const y = messageRow(Number(message.order ?? 1));
    messageLayer
      .append("line")
      .attr("class", "sequence-message")
      .attr("x1", sourceX)
      .attr("y1", y)
      .attr("x2", targetX)
      .attr("y2", y)
      .style("stroke", ctx.theme.edge.default)
      .style("stroke-width", "1.8px")
      .style("marker-end", "url(#sequence-arrow-sync)");
    const label = truncateLabel(asString(message.name ?? message.label), 28);
    if (label) {
      messageLayer
        .append("text")
        .attr("x", (sourceX + targetX) / 2)
        .attr("y", y - 8)
        .attr("text-anchor", "middle")
        .style("font-size", "10px")
        .style("fill", ctx.theme.textSecondary)
        .text(label);
    }
  }

  return {
    minX: 0,
    minY: 0,
    maxX: xOffset + lifelines.length * LIFELINE_GAP + 80,
    maxY: lifelineBottom + 60,
  };
}

export { addSequenceMarkers };

import * as d3 from "d3";
import type { PreparedNode } from "../prepare";
import type { DiagramTheme } from "../theme";
import { attachBehaviorNodeClick } from "./behavior-interaction";
import { BehaviorSceneContext, truncateLabel } from "./behavior-common";

const HEADER_Y = 64;
const LIFELINE_TOP = 118;
const LIFELINE_GAP = 220;
const MESSAGE_GAP = 78;
const LIFELINE_BOX_WIDTH = 132;
const LIFELINE_BOX_HEIGHT = 38;

export function asRecord(value: unknown): Record<string, unknown> {
  return value && typeof value === "object" ? (value as Record<string, unknown>) : {};
}

export function asArray(value: unknown): unknown[] {
  return Array.isArray(value) ? value : [];
}

export function asString(value: unknown, fallback = ""): string {
  if (typeof value === "string") return value;
  if (typeof value === "number" || typeof value === "boolean") return String(value);
  return fallback;
}

export function messageRow(order: number): number {
  return LIFELINE_TOP + 58 + (Math.max(1, order) - 1) * MESSAGE_GAP;
}

export function messageRef(message: Record<string, unknown>): string {
  return asString(message.id ?? message.name ?? message.label);
}

export function findPreparedLifeline(preparedNodes: PreparedNode[], lifeline: Record<string, unknown>): PreparedNode | undefined {
  const id = asString(lifeline.id ?? lifeline.name);
  const name = asString(lifeline.name ?? lifeline.label);
  return preparedNodes.find((node) => {
    const qualifiedName = asString(node.attributes?.qualifiedName);
    if (id && (node.id === id || qualifiedName === id)) return true;
    if (name && (node.label === name || qualifiedName === name)) return true;
    return false;
  });
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
  const activations = asArray(diagram.activations).map(asRecord);
  const fragments = asArray(diagram.fragments).map(asRecord);
  const renderOptions = ctx.options ?? {};

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
    const preparedNode = findPreparedLifeline(ctx.prepared.nodes, lifeline);
    const group = lifelineLayer
      .append("g")
      .attr("class", "sequence-lifeline")
      .attr("data-node-id", preparedNode?.id ?? id);

    group
      .append("rect")
      .attr("class", "node-background")
      .attr("data-original-stroke", ctx.theme.nodeBorder)
      .attr("data-original-width", "1.5px")
      .attr("x", x - LIFELINE_BOX_WIDTH / 2)
      .attr("y", HEADER_Y)
      .attr("width", LIFELINE_BOX_WIDTH)
      .attr("height", LIFELINE_BOX_HEIGHT)
      .attr("rx", 6)
      .style("fill", ctx.theme.nodeFill)
      .style("stroke", ctx.theme.nodeBorder)
      .style("stroke-width", "1.5px");
    group
      .append("text")
      .attr("x", x)
      .attr("y", HEADER_Y + 24)
      .attr("text-anchor", "middle")
      .style("font-size", "11px")
      .style("fill", ctx.theme.textPrimary)
      .text(label);
    group
      .append("line")
      .attr("x1", x)
      .attr("y1", LIFELINE_TOP)
      .attr("x2", x)
      .attr("y2", lifelineBottom)
      .style("stroke", ctx.theme.nodeBorder)
      .style("stroke-dasharray", "6,4");

    if (preparedNode) {
      attachBehaviorNodeClick(group, preparedNode, ctx.theme, renderOptions, ctx.root);
    }
  }

  const messageLayer = ctx.root.append("g").attr("class", "sequence-messages");
  const messagePosition = new Map<string, { sourceX: number; targetX: number; y: number }>();
  for (const message of messages) {
    const sourceId = asString(message.source ?? message.from);
    const targetId = asString(message.target ?? message.to);
    const sourceX = lifelineX.get(sourceId);
    const targetX = lifelineX.get(targetId);
    if (sourceX == null || targetX == null) continue;
    const y = messageRow(Number(message.order ?? 1));
    messagePosition.set(messageRef(message), { sourceX, targetX, y });
    const kind = asString(message.kind ?? message.type).toLowerCase();
    const isReturn = kind.includes("return") || kind.includes("reply");
    const isSelf = sourceId === targetId;
    if (isSelf) {
      const path = `M${sourceX},${y} C${sourceX + 84},${y - 18} ${sourceX + 84},${y + 34} ${sourceX},${y + 28}`;
      messageLayer
        .append("path")
        .attr("class", `sequence-message sequence-message-self${isReturn ? " sequence-message-return" : ""}`)
        .attr("d", path)
        .style("fill", "none")
        .style("stroke", ctx.theme.edge.default)
        .style("stroke-width", "1.8px")
        .style("stroke-dasharray", isReturn ? "6,4" : "none")
        .style("marker-end", "url(#sequence-arrow-sync)");
    } else {
      messageLayer
        .append("line")
      .attr("class", `sequence-message${isReturn ? " sequence-message-return" : ""}`)
      .attr("x1", sourceX)
      .attr("y1", y)
      .attr("x2", targetX)
      .attr("y2", y)
      .style("stroke", ctx.theme.edge.default)
      .style("stroke-width", "1.8px")
      .style("stroke-dasharray", isReturn ? "6,4" : "none")
      .style("marker-end", "url(#sequence-arrow-sync)");
    }
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

  const activationLayer = ctx.root.append("g").attr("class", "sequence-activations");
  for (const activation of activations) {
    const lifelineId = asString(activation.on_lifeline ?? activation.onLifeline ?? activation.lifeline ?? activation.on);
    const x = lifelineX.get(lifelineId);
    if (x == null) continue;
    const startRef = asString(activation.start_message ?? activation.startMessage ?? activation.start);
    const finishRef = asString(activation.finish_message ?? activation.finishMessage ?? activation.finish);
    const startY = messagePosition.get(startRef)?.y ?? LIFELINE_TOP + 36;
    const finishY = messagePosition.get(finishRef)?.y ?? startY + MESSAGE_GAP;
    activationLayer
      .append("rect")
      .attr("class", "sequence-activation")
      .attr("x", x - 7)
      .attr("y", startY + 6)
      .attr("width", 14)
      .attr("height", Math.max(34, finishY - startY + 18))
      .attr("rx", 3)
      .style("fill", ctx.theme.nodeFill)
      .style("stroke", ctx.theme.nodeBorder)
      .style("stroke-width", "1px");
  }

  const fragmentLayer = ctx.root.insert("g", ".sequence-messages").attr("class", "sequence-fragments");
  for (const fragment of fragments) {
    const kind = asString(fragment.kind ?? fragment.type, "fragment");
    const operands = asArray(fragment.operands).map(asRecord);
    const referencedMessages = new Set<string>();
    for (const operand of operands) {
      asArray(operand.message_ids ?? operand.messageIds ?? operand.messages).forEach((id) => referencedMessages.add(asString(id)));
    }
    const matching = [...referencedMessages].map((id) => messagePosition.get(id)).filter((value): value is { sourceX: number; targetX: number; y: number } => Boolean(value));
    if (matching.length === 0 && messages.length > 0) {
      matching.push(...[...messagePosition.values()]);
    }
    if (matching.length === 0) continue;
    const minX = Math.min(...matching.map((item) => Math.min(item.sourceX, item.targetX))) - 58;
    const maxX = Math.max(...matching.map((item) => Math.max(item.sourceX, item.targetX))) + 58;
    const minY = Math.min(...matching.map((item) => item.y)) - 34;
    const maxY = Math.max(...matching.map((item) => item.y)) + 34 + Math.max(0, operands.length - 1) * 28;
    const box = fragmentLayer.append("g").attr("class", `sequence-fragment sequence-fragment-${kind.toLowerCase()}`);
    box.append("rect")
      .attr("x", minX)
      .attr("y", minY)
      .attr("width", maxX - minX)
      .attr("height", maxY - minY)
      .attr("rx", 4)
      .style("fill", "none")
      .style("stroke", ctx.theme.nodeBorder)
      .style("stroke-dasharray", "7,4");
    box.append("path")
      .attr("d", `M${minX},${minY + 24} L${minX + 72},${minY + 24} L${minX + 90},${minY} L${minX},${minY}`)
      .style("fill", ctx.theme.canvasBackground)
      .style("stroke", ctx.theme.nodeBorder);
    box.append("text")
      .attr("x", minX + 10)
      .attr("y", minY + 16)
      .style("font-size", "10px")
      .style("font-weight", "700")
      .style("fill", ctx.theme.textPrimary)
      .text(kind);
    operands.forEach((operand, index) => {
      const guard = asString(operand.guard ?? operand.condition);
      if (index > 0) {
        const y = minY + 28 + index * 28;
        box.append("line")
          .attr("x1", minX)
          .attr("x2", maxX)
          .attr("y1", y)
          .attr("y2", y)
          .style("stroke", ctx.theme.nodeBorder)
          .style("stroke-dasharray", "4,3");
      }
      if (guard) {
        box.append("text")
          .attr("class", "sequence-fragment-guard")
          .attr("x", minX + 12)
          .attr("y", minY + 44 + index * 28)
          .style("font-size", "9px")
          .style("fill", ctx.theme.textSecondary)
          .text(`[${truncateLabel(guard, 28)}]`);
      }
    });
  }

  return {
    minX: 0,
    minY: 0,
    maxX: xOffset + lifelines.length * LIFELINE_GAP + 80,
    maxY: lifelineBottom + 60,
  };
}

export { addSequenceMarkers };

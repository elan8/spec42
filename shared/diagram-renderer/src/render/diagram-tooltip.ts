import * as d3 from "d3";

import type { PreparedEdge, PreparedNode, PreparedView } from "../prepare";
import type { DiagramTheme } from "../theme";
import type { PreparedPort } from "./types";

export interface DiagramTooltipDescriptor {
  title: string;
  rows: Array<{ label: string; value: string }>;
  technicalRows?: Array<{ label: string; value: string }>;
}

type SvgGroup = d3.Selection<SVGGElement, unknown, null, undefined>;
const activeTooltipControllers = new WeakMap<HTMLElement, () => void>();

function text(value: unknown): string {
  return String(value ?? "").trim();
}

function humanize(value: string): string {
  const normalized = value.replace(/[_-]+/g, " ").replace(/([a-z])([A-Z])/g, "$1 $2").trim();
  return normalized ? normalized.replace(/^./, (first) => first.toUpperCase()) : "Relationship";
}

function pushRow(rows: DiagramTooltipDescriptor["rows"], label: string, value: unknown): void {
  const normalized = text(value);
  if (normalized) rows.push({ label, value: normalized });
}

export function portTooltipDescriptor(port: PreparedPort): DiagramTooltipDescriptor {
  const rows: DiagramTooltipDescriptor["rows"] = [];
  const technicalRows: DiagramTooltipDescriptor["rows"] = [];
  pushRow(rows, "Type", port.portType ?? port.attributes?.portType);
  pushRow(rows, "Direction", port.direction ?? port.attributes?.direction);
  const multiplicity = text(port.multiplicity ?? port.attributes?.multiplicity ?? "[1]");
  if (multiplicity && multiplicity !== "[1]") pushRow(rows, "Multiplicity", multiplicity);
  pushRow(technicalRows, "Multiplicity", multiplicity);
  pushRow(technicalRows, "Qualified name", port.semanticId ?? port.attributes?.semanticId ?? port.id ?? port.attributes?.scenePortId);
  return { title: port.name, rows, technicalRows };
}

function nodeLabel(nodes: Map<string, PreparedNode>, id: string): string {
  const node = nodes.get(id);
  return node?.label || id;
}

function compactEndpoint(value: unknown): string {
  const raw = text(value).replace(/^occ:/, "");
  const segments = raw.split(/::|\./).filter(Boolean);
  return segments.length > 2 ? segments.slice(-2).join(".") : raw.replace(/::/g, ".");
}

export function edgeTooltipDescriptor(edge: PreparedEdge, prepared: PreparedView): DiagramTooltipDescriptor {
  const attributes = edge.attributes ?? {};
  const nodes = new Map(prepared.nodes.map((node) => [node.id, node]));
  const view = prepared.view;
  const rawKind = view === "sequence-view"
    ? "message"
    : view === "state-transition-view"
      ? "transition"
      : view === "action-flow-view"
        ? (attributes.succession === true || text(attributes.flowKind).toLowerCase() === "succession" ? "succession" : "flow")
        : text(view === "interconnection-view"
          ? (text(attributes.relationType).toLowerCase() === "binding" ? "bind" : attributes.relationType ?? edge.edgeKind ?? edge.label)
          : attributes.relationType ?? attributes.kind ?? edge.edgeKind ?? edge.label);
  const title = humanize(rawKind || (view === "sequence-view" ? "message" : "relationship"));
  const rows: DiagramTooltipDescriptor["rows"] = [];
  const technicalRows: DiagramTooltipDescriptor["rows"] = [];
  const genericLabels = new Set(["", "bind", "binding", "connect", "connection", "flow", "succession", "transition", "message", rawKind.toLowerCase()]);
  if (!genericLabels.has(text(edge.label).toLowerCase())) pushRow(rows, "Name", edge.label);

  if (view === "interconnection-view") {
    pushRow(rows, "From", compactEndpoint(attributes.sourceExpression ?? nodeLabel(nodes, edge.source)));
    pushRow(rows, "To", compactEndpoint(attributes.targetExpression ?? nodeLabel(nodes, edge.target)));
    pushRow(technicalRows, "Resolved source", attributes.sourcePortId ?? attributes.sourceId);
    pushRow(technicalRows, "Resolved target", attributes.targetPortId ?? attributes.targetId);
  } else {
    pushRow(rows, "From", nodeLabel(nodes, edge.source));
    pushRow(rows, "To", nodeLabel(nodes, edge.target));
    pushRow(technicalRows, "Source ID", edge.source);
    pushRow(technicalRows, "Target ID", edge.target);
  }

  if (view === "action-flow-view") {
    const guard = text(attributes.guard);
    if (guard && !["flow", "first", "succession", "succession flow"].includes(guard.toLowerCase())) {
      pushRow(rows, "Guard", guard);
    }
    pushRow(rows, "Condition", attributes.condition);
  } else if (view === "state-transition-view") {
    pushRow(rows, "Trigger", attributes.trigger);
    pushRow(rows, "Accept", attributes.accept);
    pushRow(rows, "Guard", attributes.guard);
    pushRow(rows, "Effect", attributes.effect);
    pushRow(rows, "Send", attributes.send);
  } else if (view === "sequence-view") {
    pushRow(rows, "Message kind", attributes.messageKind ?? attributes.kind);
    pushRow(rows, "Order", attributes.order);
  }
  pushRow(technicalRows, "Semantic ID", attributes.semanticId);
  return { title, rows, technicalRows };
}

export function tooltipText(descriptor: DiagramTooltipDescriptor): string {
  return [descriptor.title, ...descriptor.rows.map((row) => `${row.label}: ${row.value}`)].join("\n");
}

export function tooltipFallbackText(descriptor: DiagramTooltipDescriptor): string {
  const technicalRows = (descriptor.technicalRows ?? [])
    .filter((technical) => !descriptor.rows.some(
      (row) => row.label === technical.label && row.value === technical.value,
    ));
  return [
    descriptor.title,
    ...descriptor.rows.map((row) => `${row.label}: ${row.value}`),
    ...technicalRows.map((row) => `${row.label}: ${row.value}`),
  ].join("\n");
}

function renderHtmlTooltip(
  tooltip: HTMLDivElement,
  descriptor: DiagramTooltipDescriptor,
  theme: DiagramTheme,
): void {
  tooltip.replaceChildren();
  const heading = tooltip.ownerDocument.createElement("div");
  heading.className = "sysml-diagram-tooltip-title";
  heading.textContent = descriptor.title;
  Object.assign(heading.style, {
    fontSize: "13px",
    fontWeight: "600",
    color: theme.textPrimary,
    marginBottom: descriptor.rows.length > 0 ? "6px" : "0",
  });
  tooltip.appendChild(heading);
  if (descriptor.rows.length === 0) return;

  const grid = tooltip.ownerDocument.createElement("div");
  grid.className = "sysml-diagram-tooltip-grid";
  Object.assign(grid.style, {
    display: "grid",
    gridTemplateColumns: "max-content minmax(0, 1fr)",
    columnGap: "10px",
    rowGap: "3px",
    alignItems: "start",
  });
  for (const row of descriptor.rows) {
    const label = tooltip.ownerDocument.createElement("span");
    label.className = "sysml-diagram-tooltip-label";
    label.textContent = row.label;
    label.style.color = theme.textSecondary;
    const value = tooltip.ownerDocument.createElement("span");
    value.className = "sysml-diagram-tooltip-value";
    value.textContent = row.value;
    Object.assign(value.style, {
      color: theme.textPrimary,
      overflowWrap: "anywhere",
    });
    grid.append(label, value);
  }
  tooltip.appendChild(grid);
}

export function appendPathEdgeHitTarget(layer: SvgGroup, path: string, edgeId: string): void {
  layer
    .append("path")
    .attr("class", "viz-edge-hit-target")
    .attr("data-tooltip-kind", "edge")
    .attr("data-tooltip-id", edgeId)
    .attr("d", path)
    .style("fill", "none")
    .style("stroke", "transparent")
    .style("stroke-width", "12px")
    .style("pointer-events", "stroke");
}

export function appendLineEdgeHitTarget(
  layer: SvgGroup,
  edgeId: string,
  x1: number,
  y1: number,
  x2: number,
  y2: number,
): void {
  layer
    .append("line")
    .attr("class", "viz-edge-hit-target")
    .attr("data-tooltip-kind", "edge")
    .attr("data-tooltip-id", edgeId)
    .attr("x1", x1)
    .attr("y1", y1)
    .attr("x2", x2)
    .attr("y2", y2)
    .style("stroke", "transparent")
    .style("stroke-width", "12px")
    .style("pointer-events", "stroke");
}

export function markVisibleEdge<T extends SVGGeometryElement>(
  edge: d3.Selection<T, unknown, null, undefined>,
  edgeId: string,
  strokeWidth: number,
): void {
  edge
    .attr("data-edge-id", edgeId)
    .attr("data-base-stroke-width", String(strokeWidth));
}

export function installDiagramTooltips(
  target: HTMLElement,
  prepared: PreparedView,
  theme: DiagramTheme,
): () => void {
  activeTooltipControllers.get(target)?.();
  const descriptors = new Map<string, DiagramTooltipDescriptor>();
  for (const node of prepared.nodes) {
    const details = Array.isArray(node.attributes?.portDetails) ? node.attributes?.portDetails as PreparedPort[] : [];
    for (const port of details) {
      const id = text(port.id ?? port.attributes?.scenePortId);
      if (id) descriptors.set(`port:${id}`, portTooltipDescriptor(port));
    }
  }
  for (const edge of prepared.edges) {
    descriptors.set(`edge:${edge.id}`, edgeTooltipDescriptor(edge, prepared));
  }

  const previousPosition = target.style.position;
  if (!previousPosition) target.style.position = "relative";
  const tooltip = target.ownerDocument.createElement("div");
  tooltip.className = "sysml-diagram-tooltip";
  Object.assign(tooltip.style, {
    position: "absolute",
    display: "none",
    pointerEvents: "none",
    zIndex: "20",
    width: "max-content",
    maxWidth: "340px",
    padding: "9px 11px",
    borderRadius: "5px",
    border: `1px solid ${theme.nodeBorder}`,
    background: theme.nodeFill,
    color: theme.textPrimary,
    boxShadow: "0 4px 14px rgba(0, 0, 0, 0.35)",
    fontFamily: "system-ui, sans-serif",
    fontSize: "12px",
    lineHeight: "1.35",
  });
  target.appendChild(tooltip);

  const marked = Array.from(target.querySelectorAll<SVGElement>("[data-tooltip-kind]"))
    .filter((element) => Boolean(element.dataset?.tooltipId ?? element.getAttribute("data-tooltip-id")));
  for (const element of marked) {
    const kind = element.dataset?.tooltipKind ?? element.getAttribute("data-tooltip-kind") ?? "";
    const id = element.dataset?.tooltipId ?? element.getAttribute("data-tooltip-id") ?? "";
    const descriptor = descriptors.get(`${kind}:${id}`);
    if (!descriptor) continue;
    const fullText = tooltipFallbackText(descriptor);
    element.setAttribute("aria-label", fullText.replace(/\n/g, "; "));
    element.setAttribute("data-tooltip-title", fullText);
    const title = target.ownerDocument.createElementNS("http://www.w3.org/2000/svg", "title");
    title.textContent = fullText;
    element.appendChild(title);
  }

  let activeEdgeId = "";
  let suppressedNativeTitle: { element: SVGElement; title: SVGTitleElement } | null = null;
  const restoreNativeTitle = (): void => {
    if (!suppressedNativeTitle) return;
    suppressedNativeTitle.element.appendChild(suppressedNativeTitle.title);
    suppressedNativeTitle = null;
  };
  const suppressNativeTitle = (element: SVGElement): void => {
    restoreNativeTitle();
    const title = Array.from(element.children)
      .find((child): child is SVGTitleElement => child.tagName.toLowerCase() === "title");
    if (!title) return;
    title.remove();
    suppressedNativeTitle = { element, title };
  };
  const highlightEdge = (edgeId: string, active: boolean): void => {
    for (const edgeElement of Array.from(target.querySelectorAll<SVGGeometryElement>("[data-edge-id]"))) {
      if (edgeElement.getAttribute("data-edge-id") !== edgeId) continue;
      const base = Number(edgeElement.getAttribute("data-base-stroke-width") || 2);
      edgeElement.classList.toggle("viz-edge-hovered", active);
      edgeElement.style.strokeWidth = `${active ? base + 1.5 : base}px`;
      edgeElement.style.opacity = active ? "1" : "0.9";
    }
  };
  const positionTooltip = (event: MouseEvent): void => {
    const bounds = target.getBoundingClientRect();
    const width = tooltip.offsetWidth || 280;
    const height = tooltip.offsetHeight || 80;
    const left = Math.max(4, Math.min(event.clientX - bounds.left + 14, bounds.width - width - 4));
    const top = Math.max(4, Math.min(event.clientY - bounds.top + 14, bounds.height - height - 4));
    tooltip.style.left = `${left}px`;
    tooltip.style.top = `${top}px`;
  };
  const tooltipElement = (event: Event): SVGElement | null => {
    const candidate = event.target instanceof Element
      ? event.target.closest<SVGElement>("[data-tooltip-kind][data-tooltip-id]")
      : null;
    return candidate && target.contains(candidate) ? candidate : null;
  };
  const show = (event: MouseEvent): void => {
    const element = tooltipElement(event);
    if (!element) return;
    const descriptor = descriptors.get(`${element.dataset.tooltipKind}:${element.dataset.tooltipId}`);
    if (!descriptor) return;
    suppressNativeTitle(element);
    renderHtmlTooltip(tooltip, descriptor, theme);
    tooltip.style.display = "block";
    positionTooltip(event);
    if (element.dataset.tooltipKind === "edge") {
      const nextEdgeId = element.dataset.tooltipId ?? "";
      if (activeEdgeId && activeEdgeId !== nextEdgeId) highlightEdge(activeEdgeId, false);
      activeEdgeId = nextEdgeId;
      highlightEdge(activeEdgeId, true);
    }
  };
  const move = (event: MouseEvent): void => {
    if (tooltip.style.display !== "none") positionTooltip(event);
  };
  const hide = (): void => {
    tooltip.style.display = "none";
    restoreNativeTitle();
    if (activeEdgeId) highlightEdge(activeEdgeId, false);
    activeEdgeId = "";
  };
  const leave = (event: MouseEvent): void => {
    const current = tooltipElement(event);
    if (!current) return;
    const next = event.relatedTarget instanceof Element
      ? event.relatedTarget.closest<SVGElement>("[data-tooltip-kind][data-tooltip-id]")
      : null;
    if (next?.dataset.tooltipKind === current.dataset.tooltipKind
      && next?.dataset.tooltipId === current.dataset.tooltipId) return;
    hide();
  };
  target.addEventListener("mouseover", show);
  target.addEventListener("mousemove", move);
  target.addEventListener("mouseout", leave);
  target.addEventListener("mouseleave", hide);

  const cleanup = () => {
    hide();
    target.removeEventListener("mouseover", show);
    target.removeEventListener("mousemove", move);
    target.removeEventListener("mouseout", leave);
    target.removeEventListener("mouseleave", hide);
    tooltip.remove();
    if (!previousPosition) target.style.position = "";
    if (activeTooltipControllers.get(target) === cleanup) activeTooltipControllers.delete(target);
  };
  activeTooltipControllers.set(target, cleanup);
  return cleanup;
}

import * as d3 from "d3";
import { normalizeEdgeKind } from "../graph-normalization";
import { nodeBodyChromeStyle, resolveNodeChrome } from "../node-notation";
import { collectCompartments, computeNodeHeight, renderSysMLNode } from "../sysml-node-builder";
import { strokeColorForEdge, strokeColorForNode, type DiagramTheme } from "../theme";
import type { InterconnectionLayoutDto } from "../prepare";
import type { PreparedView } from "../prepare";
import { nodeSupportsSourceNavigation } from "../views/behavior-interaction";
import { buildInterconnectionLayoutLookup, type InterconnectionLayoutLookup } from "./interconnection-layout-dto";
import { resolveIbdRoutePoints } from "./ibd-route";
import {
  ibdNodeHeight,
  ibdNodeWidth,
  nodeHeight,
  nodeWidth,
  type ContentBounds,
  type EdgeSection,
  type LaidOutEdge,
  type LaidOutNode,
  type PreparedPort,
  type RenderOptions,
} from "./types";

function truncate(value: string, max: number): string {
  const text = String(value || "");
  return text.length > max ? `${text.slice(0, max - 1)}...` : text;
}

export function drawEdges(
  root: d3.Selection<SVGGElement, unknown, null, undefined>,
  edges: LaidOutEdge[],
  isInterconnectionView: boolean,
  theme: DiagramTheme,
  layoutDto?: InterconnectionLayoutDto,
): void {
  const layoutLookup = layoutDto ? buildInterconnectionLayoutLookup(layoutDto) : undefined;
  const group = root.append("g").attr("class", "viz-edges");
  for (const edge of edges) {
    if (!edge.sourceNode || !edge.targetNode) continue;
    const path = isInterconnectionView
      ? pathForIbdEdge(edge, layoutLookup)
      : pathFromSimpleSection(edge.layout?.sections?.[0]);
    if (!path) continue;
    const edgeKind = edge.edgeKind ?? normalizeEdgeKind(edge.label);
    const displayLabel = edgeDisplayLabel(edge, edgeKind, isInterconnectionView);
    const stroke = strokeColorForEdge(edgeKind, theme);
    const strokeWidth = edgeKind === "hierarchy" ? 1.4 : isInterconnectionView ? 2 : 1.8;
    const pathSelection = group
      .append("path")
      .attr("class", `${isInterconnectionView ? "ibd-connector" : "general-connector"} viz-edge viz-edge--${edgeKind}`)
      .attr("d", path)
      .attr("data-connector-id", edge.id)
      .attr("data-source", edge.source)
      .attr("data-target", edge.target)
      .attr("data-type", String((edge.attributes?.relationType as string) || edgeKind || "relationship"))
      .style("fill", "none")
      .style("stroke", stroke)
      .style("stroke-width", strokeWidth)
      .style("opacity", 0.9);
    applyEdgeMarker(pathSelection, edgeKind, isInterconnectionView, theme);
    if (shouldRenderEdgeLabel(edge, edgeKind, isInterconnectionView)) {
      const midpoint = edgeMidpoint(edge, isInterconnectionView, layoutLookup);
      group
        .append("text")
        .attr("class", `viz-edge-label viz-edge-label--${edgeKind}`)
        .attr("x", midpoint.x)
        .attr("y", midpoint.y)
        .attr("text-anchor", "middle")
        .attr("dy", "-0.35em")
        .attr("fill", theme.textPrimary)
        .attr("font-size", 11)
        .text(truncate(displayLabel, 18));
    }
  }
}

function shouldRenderEdgeLabel(edge: LaidOutEdge, edgeKind: string, isInterconnectionView: boolean): boolean {
  return edgeDisplayLabel(edge, edgeKind, isInterconnectionView).length > 0;
}

function edgeDisplayLabel(edge: LaidOutEdge, edgeKind: string, isInterconnectionView: boolean): string {
  return isInterconnectionView ? ibdEdgeDisplayLabel(edge, edgeKind) : generalEdgeDisplayLabel(edge, edgeKind);
}

function generalEdgeDisplayLabel(edge: LaidOutEdge, edgeKind: string): string {
  const label = String(edge.label ?? "").trim();
  const relationType = String(edge.attributes?.relationType ?? "").trim();
  const generic = new Set([
    "",
    "relationship",
    "edge",
    "connect",
    "connection",
    "dependency",
    "specializes",
    "specialization",
    "typing",
    "defined_by",
    "defined by",
    "definition",
    "hierarchy",
    "contains",
    "owns",
    "ownership",
    "containment",
    "allocate",
    "allocation",
    "satisfy",
    "verify",
    "bind",
    "binding",
  ]);
  const lowerLabel = label.toLowerCase();
  if (generic.has(lowerLabel)) return "";
  if (lowerLabel === relationType.toLowerCase() || lowerLabel === edgeKind.toLowerCase()) return "";
  return label;
}

function ibdEdgeDisplayLabel(edge: LaidOutEdge, edgeKind: string): string {
  const itemType = String(edge.attributes?.itemType ?? "").trim();
  if (edgeKind === "flow" && itemType) return itemType;
  const interfaceName = String(edge.attributes?.interfaceName ?? "").trim();
  if (edgeKind === "interface" && interfaceName) return interfaceName;
  const label = String(edge.label ?? "").trim();
  const relationType = String(edge.attributes?.relationType ?? "").trim();
  const generic = new Set(["", "connect", "connection", "flow", "interface", "binding", "bind", "reference", "ref", "relationship"]);
  if (generic.has(label.toLowerCase()) || generic.has(relationType.toLowerCase())) return "";
  return label;
}

export function drawNodes(
  root: d3.Selection<SVGGElement, unknown, null, undefined>,
  nodes: LaidOutNode[],
  options: RenderOptions,
  isInterconnectionView: boolean,
  theme: DiagramTheme,
  layoutDto?: InterconnectionLayoutDto,
): void {
  const layoutLookup = layoutDto ? buildInterconnectionLayoutLookup(layoutDto) : undefined;
  const renderNodes = isInterconnectionView ? orderIbdNodesForPaint(nodes) : nodes;
  const groups = root
    .append("g")
    .attr("class", "viz-nodes")
    .selectAll("g")
    .data(renderNodes)
    .enter()
    .append("g")
    .attr("class", (d: LaidOutNode) => {
      const clickable = options.onNodeClick && nodeSupportsSourceNavigation(d) ? "is-clickable" : "";
      const selected = options.selectedNodeId && d.id === options.selectedNodeId ? "is-selected" : "";
      const legacyClass = isInterconnectionView ? "ibd-part" : "general-node";
      const attrs = (d.attributes ?? {}) as Record<string, unknown>;
      const isLayoutContainer = Boolean(
        attrs.isSyntheticContainer || attrs.isPackageContainer || attrs._isLayoutContainer,
      );
      const structureClass = resolveNodeChrome(d.kind || "part", {
        ...(typeof attrs.isDefinition === "boolean" ? { isDefinition: attrs.isDefinition } : {}),
        ...(typeof attrs.isReference === "boolean" ? { isReference: attrs.isReference } : {}),
        isContainer: isLayoutContainer,
        isPackageContainer: Boolean(attrs.isPackageContainer),
      }).structureClass;
      return `${legacyClass} viz-node ${structureClass} ${clickable} ${selected}`.trim();
    })
    .attr("transform", (d: LaidOutNode) => `translate(${d.x || 0},${d.y || 0})`)
    .attr("data-node-id", (d: LaidOutNode) => d.id)
    .attr("data-element-name", (d: LaidOutNode) => d.label)
    .attr("data-bounds", (d: LaidOutNode) =>
      [d.x || 0, d.y || 0, d.width || (isInterconnectionView ? ibdNodeWidth : nodeWidth), d.height || (isInterconnectionView ? ibdNodeHeight : nodeHeight)].join(",")
    )
    .style("cursor", (d: LaidOutNode) =>
      options.onNodeClick && nodeSupportsSourceNavigation(d) ? "pointer" : null,
    )
    .on("click", (event: unknown, d: LaidOutNode) => {
      if (!options.onNodeClick || !nodeSupportsSourceNavigation(d)) {
        return;
      }
      (event as Event).stopPropagation?.();
      options.onNodeClick?.(d);
    });

  if (!isInterconnectionView) {
    groups.each(function (d: LaidOutNode) {
      const group = d3.select(this);
      group.selectAll("*").remove();
      const compartments = d.compartments ?? collectCompartments(d);
      const attrs = (d.attributes ?? {}) as Record<string, unknown>;
      const chrome = resolveNodeChrome(d.kind, {
        ...(typeof attrs.isDefinition === "boolean" ? { isDefinition: attrs.isDefinition } : {}),
        ...(typeof attrs.isReference === "boolean" ? { isReference: attrs.isReference } : {}),
      });
      renderSysMLNode(group as any, compartments, {
        x: 0,
        y: 0,
        width: d.width || nodeWidth,
        height: d.height || computeNodeHeight(compartments, { maxLinesPerCompartment: 8 }),
        nodeClass: "",
        dataElementName: d.label,
        strokeColor: strokeColorForNode(theme),
        kind: d.kind,
        chrome,
        selected: Boolean(options.selectedNodeId && d.id === options.selectedNodeId),
        config: { maxLinesPerCompartment: 8 },
        theme,
      });
    });
    return;
  }

  groups.each(function (d: LaidOutNode) {
    const group = d3.select(this);
    group.selectAll("*").remove();
    try {
      renderIbdNode(
        group as any,
        d,
        Boolean(options.selectedNodeId && d.id === options.selectedNodeId),
        theme,
        layoutLookup?.nodesById.get(d.id),
      );
    } catch (error) {
      console.error("[IBD] failed to render node", d.id, error);
    }
  });
  return;

  groups
    .append("rect")
    .attr("width", (d: LaidOutNode) => d.width || nodeWidth)
    .attr("height", (d: LaidOutNode) => d.height || nodeHeight)
    .attr("rx", 8)
    .attr("fill", "var(--vscode-editor-background, #1e1e1e)")
    .attr("stroke", "var(--vscode-panel-border, #666)")
    .attr("stroke-width", 1.6);

  if (isInterconnectionView) {
    groups
      .append("text")
      .attr("class", "viz-node-kind")
      .attr("x", 14)
      .attr("y", 22)
      .attr("text-anchor", "start")
      .attr("fill", "var(--vscode-descriptionForeground, #a8a8a8)")
      .attr("font-size", 11)
      .text((d: LaidOutNode) => `<<${truncate(d.kind, 24)}>>`);
    groups
      .append("text")
      .attr("class", "viz-node-name")
      .attr("x", 14)
      .attr("y", 44)
      .attr("text-anchor", "start")
      .attr("fill", "var(--vscode-editor-foreground, #d0d0d0)")
      .attr("font-size", 12)
      .text((d: LaidOutNode) => truncate(d.label, 34));
    groups
      .append("line")
      .attr("x1", 10)
      .attr("x2", (d: LaidOutNode) => (d.width || ibdNodeWidth) - 10)
      .attr("y1", 56)
      .attr("y2", 56)
      .attr("stroke", "currentColor")
      .attr("opacity", 0.18);
    groups
      .append("text")
      .attr("class", "viz-node-kind")
      .attr("x", 14)
      .attr("y", 74)
      .attr("text-anchor", "start")
      .attr("fill", "var(--vscode-descriptionForeground, #a8a8a8)")
      .attr("font-size", 10)
      .text((d: LaidOutNode) => {
        const ports = Array.isArray((d.attributes as Record<string, unknown> | undefined)?.ports)
          ? ((d.attributes as Record<string, unknown>).ports as unknown[])
          : [];
        if (ports.length === 0) return "ports: —";
        return `ports: ${ports.slice(0, 6).map((value) => String(value)).join(", ")}${ports.length > 6 ? "..." : ""}`;
      });
    return;
  }

  groups
    .append("text")
    .attr("class", "viz-node-kind")
    .attr("x", nodeWidth / 2)
    .attr("y", 22)
    .attr("text-anchor", "middle")
    .attr("fill", "var(--vscode-descriptionForeground, #a8a8a8)")
    .attr("font-size", 11)
    .text((d: LaidOutNode) => `<<${truncate(d.kind, 24)}>>`);
  groups
    .append("text")
    .attr("class", "viz-node-name")
    .attr("x", nodeWidth / 2)
    .attr("y", 48)
    .attr("text-anchor", "middle")
    .attr("fill", "var(--vscode-editor-foreground, #d0d0d0)")
    .attr("font-size", 12)
    .text((d: LaidOutNode) => truncate(d.label, 30));

  groups
    .append("line")
    .attr("x1", 10)
    .attr("x2", (d: LaidOutNode) => (d.width || nodeWidth) - 10)
    .attr("y1", 58)
    .attr("y2", 58)
    .attr("stroke", "var(--vscode-panel-border, #666)")
    .attr("opacity", 0.5);

  groups
    .append("text")
    .attr("class", "viz-node-attrs")
    .attr("x", 12)
    .attr("y", 74)
    .attr("text-anchor", "start")
    .attr("fill", "var(--vscode-descriptionForeground, #a8a8a8)")
    .attr("font-size", 10)
    .text((d: LaidOutNode) => formatCompartmentSummary(d.attributes));
}

function orderIbdNodesForPaint(nodes: LaidOutNode[]): LaidOutNode[] {
  return nodes.slice().sort((a, b) => {
    const aContainer = Boolean((a.attributes ?? {})._isLayoutContainer || (a.attributes ?? {}).isSyntheticContainer || (a.attributes ?? {}).isPackageContainer);
    const bContainer = Boolean((b.attributes ?? {})._isLayoutContainer || (b.attributes ?? {}).isSyntheticContainer || (b.attributes ?? {}).isPackageContainer);
    if (aContainer !== bContainer) return aContainer ? -1 : 1;
    const aDepth = Number((a.attributes ?? {})._layoutDepth ?? 0);
    const bDepth = Number((b.attributes ?? {})._layoutDepth ?? 0);
    if (aContainer && bContainer && aDepth !== bDepth) return aDepth - bDepth;
    if (!aContainer && !bContainer && aDepth !== bDepth) return aDepth - bDepth;
    return nodes.indexOf(a) - nodes.indexOf(b);
  });
}

function applyEdgeMarker(
  path: d3.Selection<SVGPathElement, unknown, null, undefined>,
  edgeKind: string,
  isInterconnectionView: boolean,
  theme: DiagramTheme,
): void {
  if (isInterconnectionView) {
    if (edgeKind === "flow") {
      path.attr("stroke", strokeColorForEdge(edgeKind, theme)).attr("stroke-width", 2.5).style("marker-end", "url(#ibd-flow-arrow)");
    } else if (edgeKind === "interface") {
      path.attr("stroke", strokeColorForEdge(edgeKind, theme)).style("stroke-dasharray", "8,4").style("marker-end", "url(#ibd-interface-arrow)");
    } else if (edgeKind === "bind" || edgeKind === "binding") {
      path.attr("stroke", strokeColorForEdge("bind", theme)).style("stroke-dasharray", "6,4").style("marker-start", "url(#ibd-connection-dot)").style("marker-end", "url(#ibd-connection-dot)");
    } else if (edgeKind === "reference") {
      path.attr("stroke", strokeColorForEdge(edgeKind, theme)).attr("stroke-width", 1.6).style("stroke-dasharray", "4,4").style("marker-start", "url(#ibd-connection-dot)").style("marker-end", "url(#ibd-connection-dot)");
    } else if (edgeKind === "connection" || edgeKind === "relationship") {
      path.attr("stroke", strokeColorForEdge("connection", theme)).attr("stroke-width", 2).style("marker-start", "url(#ibd-connection-dot)").style("marker-end", "url(#ibd-connection-dot)");
    } else {
      path.style("marker-start", "url(#ibd-connection-dot)").style("marker-end", "url(#ibd-connection-dot)");
    }
    return;
  }
  if (edgeKind === "specializes") {
    path.attr("stroke", strokeColorForEdge(edgeKind, theme)).style("marker-end", "url(#general-d3-specializes)").style("stroke-width", "1.7px");
  } else if (edgeKind === "typing") {
    path.attr("stroke", strokeColorForEdge(edgeKind, theme)).style("marker-end", "url(#general-d3-arrow-open)").style("stroke-dasharray", "5,3");
  } else if (edgeKind === "hierarchy") {
    path.attr("stroke", strokeColorForEdge(edgeKind, theme)).style("marker-start", "url(#general-d3-diamond)").style("marker-end", "none");
  } else if (edgeKind === "bind") {
    path.attr("stroke", strokeColorForEdge(edgeKind, theme)).style("stroke-dasharray", "2,2").style("marker-end", "none");
  } else if (edgeKind === "allocate") {
    path.attr("stroke", strokeColorForEdge(edgeKind, theme)).style("marker-end", "url(#general-d3-arrow)").style("stroke-dasharray", "8,4");
  } else if (edgeKind === "dependency" || edgeKind === "usage") {
    path.attr("stroke", strokeColorForEdge(edgeKind, theme)).style("marker-end", "url(#general-d3-arrow-open)").style("stroke-dasharray", "4,4");
  } else if (edgeKind === "redefinition") {
    path.attr("stroke", strokeColorForEdge(edgeKind, theme)).style("marker-end", "url(#general-d3-specializes)").style("stroke-dasharray", "5,3");
  } else if (edgeKind === "composition") {
    path.attr("stroke", strokeColorForEdge(edgeKind, theme)).style("marker-start", "url(#general-d3-diamond)").style("marker-end", "none").style("stroke-dasharray", "6,3");
  } else if (edgeKind === "satisfy" || edgeKind === "verify" || edgeKind === "derivation") {
    path.attr("stroke", strokeColorForEdge(edgeKind, theme)).style("marker-end", "url(#general-d3-arrow-open)").style("stroke-dasharray", "7,4");
  } else {
    path.style("marker-end", "url(#general-d3-arrow)");
  }
}

function renderIbdNode(
  group: d3.Selection<SVGGElement, LaidOutNode, null, undefined>,
  node: LaidOutNode,
  selected: boolean,
  theme: DiagramTheme,
  layoutNode?: InterconnectionLayoutDto["nodes"][number],
): void {
  const attrs = (node.attributes ?? {}) as Record<string, unknown>;
  const kind = (node.kind || "part").toLowerCase();
  const isContainer = Boolean(attrs.isSyntheticContainer) || Boolean(attrs.isPackageContainer) || Boolean(attrs._isLayoutContainer);
  const width = node.width ?? ibdNodeWidth;
  const height = node.height ?? ibdNodeHeight;
  const chrome = resolveNodeChrome(kind, {
    ...(typeof attrs.isDefinition === "boolean" ? { isDefinition: attrs.isDefinition } : {}),
    ...(typeof attrs.isReference === "boolean" ? { isReference: attrs.isReference } : {}),
    isContainer,
    isPackageContainer: Boolean(attrs.isPackageContainer),
  });
  const body = nodeBodyChromeStyle(chrome, {
    selected,
    isContainer,
    isPackageContainer: Boolean(attrs.isPackageContainer),
  });
  const stroke = selected ? theme.highlight : theme.nodeBorder;
  const headerHeight = isContainer ? 28 : attrs.partType ? 41 : 33;
  group.classed("ibd-container", isContainer);

  group
    .append("rect")
    .attr("width", width)
    .attr("height", height)
    .attr("rx", body.cornerRadius)
    .attr("class", "graph-node-background")
    .attr("data-original-stroke", theme.nodeBorder)
    .attr("data-original-width", `${body.strokeWidthPx}px`)
    .style("fill", theme.nodeFill)
    .style("stroke", stroke)
    .style("stroke-width", `${body.strokeWidthPx}px`)
    .style("stroke-dasharray", body.strokeDasharray);

  group
    .append("rect")
    .attr("width", width)
    .attr("height", headerHeight)
    .attr("rx", 6)
    .style("fill", theme.panelBackground);

  if (isContainer) {
    group
      .append("text")
      .attr("x", width / 2)
      .attr("y", headerHeight / 2 + 4)
      .attr("text-anchor", "middle")
      .text(node.label)
      .style("font-size", "11px")
      .style("font-weight", "bold")
      .style("fill", theme.textPrimary);
    drawIbdPorts(group, node, width, headerHeight, theme, layoutNode);
    return;
  }

  const stereo = kind.includes("part def") ? "part def" : kind.includes("part") ? "part" : (node.kind || "part").replace(/_/g, " ");
  group
    .append("text")
    .attr("x", width / 2)
    .attr("y", 17)
    .attr("text-anchor", "middle")
    .text(`\u00ab${stereo}\u00bb`)
    .style("font-size", "9px")
    .style("fill", theme.textPrimary);

  group
    .append("text")
    .attr("class", "node-name-text viz-node-name")
    .attr("x", width / 2)
    .attr("y", 31)
    .attr("text-anchor", "middle")
    .text(truncate(node.label, 18))
    .style("font-size", "11px")
    .style("font-weight", "bold")
    .style("fill", theme.textPrimary);

  const typedBy = String(attrs.partType || "");
  if (typedBy) {
    group
      .append("text")
      .attr("x", width / 2)
      .attr("y", 43)
      .attr("text-anchor", "middle")
      .text(`: ${truncate(typedBy, 18)}`)
      .style("font-size", "10px")
      .style("font-style", "italic")
      .style("fill", theme.textPrimary);
  }

  const contentStartY = typedBy ? 50 : 38;
  const children = Array.isArray(attrs.children) ? attrs.children : [];
  children.slice(0, 8).forEach((child, index) => {
    const childRecord = child && typeof child === "object" ? child as Record<string, unknown> : {};
    const childType = String(childRecord.type || "").toLowerCase();
    const prefix = childType.includes("attribute") ? "[attr] " : childType.includes("state") ? "[state] " : childType.includes("part") ? "[part] " : "";
    const name = String(childRecord.name || "");
    if (!name) return;
    group
      .append("text")
      .attr("x", 6)
      .attr("y", contentStartY + 8 + index * 12)
      .text(truncate(`${prefix}${name}`, 28))
      .style("font-size", "9px")
      .style("fill", theme.textSecondary);
  });

  drawIbdPorts(group, node, width, contentStartY + 20, theme, layoutNode);
}

function drawIbdPorts(
  group: d3.Selection<SVGGElement, LaidOutNode, null, undefined>,
  node: LaidOutNode,
  width: number,
  fallbackStartY: number,
  theme: DiagramTheme,
  layoutNode?: InterconnectionLayoutDto["nodes"][number],
): void {
  const attrs = (node.attributes ?? {}) as Record<string, unknown>;
  const details = Array.isArray(attrs.portDetails) ? attrs.portDetails as PreparedPort[] : [];
  const drawOrder = layoutNode?.portDrawOrder ?? null;
  const portNames = drawOrder
    ? [...(drawOrder.west ?? []), ...(drawOrder.east ?? [])]
    : details.length > 0
      ? details.map((port) => port.name)
      : Array.isArray(attrs.ports) ? (attrs.ports as unknown[]).map((port) => String(port)) : [];
  const anchors = layoutNode?.portAnchors ?? {};
  const portSize = 10;
  const fallbackSpacing = 26;
  const drawPort = (name: string, sideIndex: number, side: "WEST" | "EAST") => {
    const detail = details.find((port) => port.name === name);
    const sanitized = name.replace(/[^A-Za-z0-9_.-]/g, "_");
    const anchor = anchors[sanitized] ?? anchors[name];
    const resolvedSide = (anchor?.side === "WEST" || anchor?.side === "EAST" ? anchor.side : side) as "WEST" | "EAST";
    const x = anchor?.x ?? (resolvedSide === "WEST" ? 0 : width);
    const y = anchor?.y ?? (fallbackStartY + sideIndex * fallbackSpacing);
    const color = theme.nodeBorder;
    group
      .append("rect")
      .attr("class", "port-icon")
      .attr("data-port-name", name)
      .attr("data-port-side", resolvedSide)
      .attr("x", x - portSize / 2)
      .attr("y", y - portSize / 2)
      .attr("width", portSize)
      .attr("height", portSize)
      .style("fill", "none")
      .style("stroke", color)
      .style("stroke-width", "1.8px");
    group
      .append("text")
      .attr("x", resolvedSide === "WEST" ? Math.min(width - 10, x + 16) : Math.max(10, x - 16))
      .attr("y", y + 3)
      .attr("text-anchor", resolvedSide === "WEST" ? "start" : "end")
      .text(truncate(formatIbdPortLabel(name, detail), 24))
      .style("font-size", "8px")
      .style("font-weight", "500")
      .style("fill", color);
  };

  if (drawOrder) {
    (drawOrder.west ?? []).forEach((name: string, index: number) => drawPort(name, index, "WEST"));
    (drawOrder.east ?? []).forEach((name: string, index: number) => drawPort(name, index, "EAST"));
    return;
  }

  portNames.forEach((name, index) => {
    const sanitized = name.replace(/[^A-Za-z0-9_.-]/g, "_");
    const anchor = anchors[sanitized] ?? anchors[name];
    const side: "WEST" | "EAST" =
      anchor?.side === "WEST" ? "WEST" : anchor?.side === "EAST" ? "EAST" : name.toLowerCase().startsWith("in") ? "WEST" : "EAST";
    drawPort(name, index, side);
  });
}

function formatIbdPortLabel(name: string, detail?: PreparedPort): string {
  const direction = String(detail?.direction || "").trim();
  const directionPrefix = direction ? `${direction} ` : "";
  const type = String(detail?.portType || detail?.attributes?.portType || "").trim();
  if (!type) return `${directionPrefix}${name}`;
  const conjugated = type.startsWith("~");
  const cleanType = type.replace(/^~/, "").split(/::|\./).pop() || type.replace(/^~/, "");
  return `${directionPrefix}${name}: ${conjugated ? "~" : ""}${cleanType}`;
}

function formatCompartmentSummary(attributes: Record<string, unknown> | undefined): string {
  if (!attributes) return "";
  const parts = Array.isArray(attributes.parts) ? attributes.parts : [];
  const ports = Array.isArray(attributes.ports) ? attributes.ports : [];
  const attrs = Array.isArray(attributes.attributes) ? attributes.attributes : [];
  const summary: string[] = [];
  if (attrs.length > 0) summary.push(`attrs:${attrs.length}`);
  if (parts.length > 0) summary.push(`parts:${parts.length}`);
  if (ports.length > 0) summary.push(`ports:${ports.length}`);
  return summary.join("  ");
}

export function drawGeneralPackageContainers(
  root: d3.Selection<SVGGElement, unknown, null, undefined>,
  prepared: PreparedView,
  nodes: LaidOutNode[],
  theme: DiagramTheme,
): void {
  const packageGroups = ((prepared.meta?.packageContainerGroups as unknown[]) || []) as Array<Record<string, unknown>>;
  if (packageGroups.length === 0) return;
  const nodeById = new Map(nodes.map((node) => [node.id, node]));
  const layer = root.append("g").attr("class", "general-package-containers");
  for (const group of packageGroups) {
    const memberIds = Array.isArray(group.memberIds) ? group.memberIds.map((value) => String(value)) : [];
    const label = String(group.name || group.label || group.id || "");
    const memberNodes = memberIds.map((id) => nodeById.get(id)).filter((value): value is LaidOutNode => Boolean(value));
    if (memberNodes.length === 0) continue;
    const minX = Math.min(...memberNodes.map((node) => node.x || 0));
    const minY = Math.min(...memberNodes.map((node) => (node.y || 0)));
    const maxX = Math.max(...memberNodes.map((node) => (node.x || 0) + (node.width || nodeWidth)));
    const maxY = Math.max(...memberNodes.map((node) => (node.y || 0) + (node.height || nodeHeight)));
    const padding = 28;
    const x = minX - padding;
    const y = minY - padding;
    const width = maxX - minX + padding * 2;
    const height = maxY - minY + padding * 2;
    layer
      .append("rect")
      .attr("class", "general-package-frame")
      .attr("x", x)
      .attr("y", y)
      .attr("width", width)
      .attr("height", height)
      .attr("rx", 18)
      .style("fill", "transparent")
      .style("stroke", theme.nodeBorder)
      .style("stroke-width", "1.5px")
      .style("opacity", 0.9);
    layer
      .append("text")
      .attr("class", "general-package-label")
      .attr("x", x + 14)
      .attr("y", y + 21)
      .style("font-size", "11px")
      .style("font-weight", "700")
      .style("fill", theme.nodeBorder)
      .text(label);
  }
}

export function drawInterconnectionContainers(
  root: d3.Selection<SVGGElement, unknown, null, undefined>,
  prepared: PreparedView,
  nodes: LaidOutNode[],
  theme: DiagramTheme,
  layoutDto?: InterconnectionLayoutDto,
): void {
  const layoutContainers = layoutDto?.containers ?? [];
  if (layoutContainers.length > 0) {
    const layer = root.append("g").attr("class", "ibd-containers");
    for (const container of layoutContainers) {
      const label = container.label;
      const groupG = layer
        .append("g")
        .attr("class", "ibd-part ibd-container")
        .attr("transform", `translate(${container.x},${container.y})`)
        .attr("data-element-name", label);
      groupG
        .append("rect")
        .attr("width", container.width)
        .attr("height", container.height)
        .attr("rx", 14)
        .attr("fill", "none")
        .attr("stroke", theme.nodeBorder)
        .attr("stroke-width", 1.4)
        .attr("stroke-dasharray", "6,4")
        .attr("opacity", 0.7);
      groupG
        .append("text")
        .attr("x", 12)
        .attr("y", 20)
        .attr("fill", theme.textSecondary)
        .attr("font-size", 11)
        .text(label);
    }
    return;
  }
  const packageGroups = ((prepared.meta?.packageContainerGroups as unknown[]) || []) as Array<Record<string, unknown>>;
  if (packageGroups.length === 0) return;
  const nodeById = new Map(nodes.map((node) => [node.id, node]));
  const layer = root.append("g").attr("class", "ibd-containers");
  for (const group of packageGroups) {
    const memberIds = Array.isArray(group.memberIds) ? group.memberIds.map((value) => String(value)) : [];
    const label = String(group.name || group.label || group.id || "");
    const memberNodes = memberIds.map((id) => nodeById.get(id)).filter((value): value is LaidOutNode => Boolean(value));
    if (memberNodes.length === 0) continue;
    const minX = Math.min(...memberNodes.map((node) => node.x || 0));
    const minY = Math.min(...memberNodes.map((node) => node.y || 0));
    const maxX = Math.max(...memberNodes.map((node) => (node.x || 0) + (node.width || ibdNodeWidth)));
    const maxY = Math.max(...memberNodes.map((node) => (node.y || 0) + (node.height || ibdNodeHeight)));
    const padding = 26;
    const x = minX - padding;
    const y = minY - padding;
    const width = (maxX - minX) + (padding * 2);
    const height = (maxY - minY) + (padding * 2);
    const groupG = layer
      .append("g")
      .attr("class", "ibd-part ibd-container")
      .attr("transform", `translate(${x},${y})`)
      .attr("data-element-name", label);
    groupG
      .append("rect")
      .attr("width", width)
      .attr("height", height)
      .attr("rx", 14)
      .attr("fill", "none")
      .attr("stroke", theme.nodeBorder)
      .attr("stroke-width", 1.4)
      .attr("stroke-dasharray", "6,4")
      .attr("opacity", 0.7);
    groupG
      .append("text")
      .attr("x", 12)
      .attr("y", 20)
      .attr("fill", theme.textSecondary)
      .attr("font-size", 11)
      .text(label);
  }
}

export function shouldDrawIbdViewFrame(prepared: PreparedView): boolean {
  return !prepared.nodes.some((node) => Boolean((node.attributes ?? {}).isDiagramRoot));
}

export function drawIbdViewFrame(
  root: d3.Selection<SVGGElement, unknown, null, undefined>,
  prepared: PreparedView,
  bounds: ContentBounds,
  theme: DiagramTheme,
): void {
  const label = String(prepared.meta?.selectedRoot || prepared.title || "").trim();
  if (!label || bounds.width <= 0 || bounds.height <= 0) return;
  const padding = 20;
  const headerHeight = 18;
  const x = bounds.x - padding;
  const y = bounds.y - padding - headerHeight;
  const width = bounds.width + padding * 2;
  const height = bounds.height + padding * 2 + headerHeight;
  const frame = root
    .append("g")
    .attr("class", "ibd-view-frame")
    .attr("data-view-name", label);
  frame
    .append("rect")
    .attr("x", x)
    .attr("y", y)
    .attr("width", width)
    .attr("height", height)
    .attr("rx", 6)
    .style("fill", "none")
    .style("stroke", theme.frame.stroke)
    .style("stroke-width", "1.5px");
  frame
    .append("text")
    .attr("x", x + width / 2)
    .attr("y", y + 13)
    .attr("text-anchor", "middle")
    .style("font-size", "11px")
    .style("font-weight", "bold")
    .style("fill", theme.frame.text)
    .text(label);
}

function pointsToPathD(points: Array<{ x: number; y: number }>): string {
  if (points.length < 2) return "";
  return d3.line<{ x: number; y: number }>().x((d) => d.x).y((d) => d.y)(points) || "";
}

function pathFromSimpleSection(section: EdgeSection | undefined): string | null {
  if (!section) return null;
  const points = [section.startPoint, ...(section.bendPoints || []), section.endPoint].filter(Boolean) as Array<{ x: number; y: number }>;
  if (points.length < 2) return null;
  return pointsToPathD(points);
}

export function pathForIbdEdge(
  edge: LaidOutEdge,
  layoutLookup?: InterconnectionLayoutLookup,
): string | null {
  const layoutEdge = layoutLookup?.edgesById.get(edge.id);
  const points = layoutEdge && layoutEdge.routePoints.length >= 2
    ? layoutEdge.routePoints
    : resolveIbdRoutePoints(edge);
  if (!points || points.length < 2) return null;
  return pointsToPathD(points);
}

function edgeMidpoint(
  edge: LaidOutEdge,
  isInterconnectionView: boolean,
  layoutLookup?: InterconnectionLayoutLookup,
): { x: number; y: number } {
  if (isInterconnectionView) {
    const layoutEdge = layoutLookup?.edgesById.get(edge.id);
    const routePoints = layoutEdge?.routePoints?.length
      ? layoutEdge.routePoints
      : resolveIbdRoutePoints(edge);
    if (routePoints && routePoints.length > 0) {
      const index = Math.floor((routePoints.length - 1) / 2);
      return routePoints[index];
    }
  } else {
    const section = edge.layout?.sections?.[0];
    if (section) {
      const points = [section.startPoint, ...(section.bendPoints || []), section.endPoint].filter(Boolean) as Array<{
        x: number;
        y: number;
      }>;
      if (points.length > 0) {
        const index = Math.floor((points.length - 1) / 2);
        return points[index];
      }
    }
  }
  const sourceNode = edge.sourceNode;
  const targetNode = edge.targetNode;
  if (sourceNode && targetNode) {
    const width = isInterconnectionView ? ibdNodeWidth : nodeWidth;
    const height = isInterconnectionView ? ibdNodeHeight : nodeHeight;
    return {
      x: ((sourceNode.x || 0) + (targetNode.x || 0) + width) / 2,
      y: ((sourceNode.y || 0) + (targetNode.y || 0) + height) / 2,
    };
  }
  return { x: 0, y: 0 };
}
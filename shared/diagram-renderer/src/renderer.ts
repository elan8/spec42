import * as d3 from "d3";
import ELK from "elkjs/lib/elk.bundled.js";
import { nodeAccentClass, type PreparedNode, type PreparedView } from "./prepare";
import { normalizeEdgeKind } from "./graph-normalization";

const elk = new ELK();
const nodeWidth = 220;
const nodeHeight = 86;
const ibdNodeWidth = 280;
const ibdNodeHeight = 140;

export interface RenderOptions {
  onNodeClick?: (node: PreparedNode) => void;
  selectedNodeId?: string | null;
}

export interface RenderController {
  reset: () => void;
  exportSvg: () => string;
  destroy: () => void;
}

interface LaidOutNode extends PreparedNode {
  x?: number;
  y?: number;
  width?: number;
  height?: number;
}

interface EdgeSection {
  startPoint?: { x: number; y: number };
  bendPoints?: Array<{ x: number; y: number }>;
  endPoint?: { x: number; y: number };
}

interface LaidOutEdge {
  id: string;
  source: string;
  target: string;
  label: string;
  edgeKind?: string;
  attributes?: Record<string, unknown>;
  sourceNode?: LaidOutNode;
  targetNode?: LaidOutNode;
  layout?: {
    sections?: EdgeSection[];
  };
}

interface LayoutResult {
  nodes: LaidOutNode[];
  edges: LaidOutEdge[];
}

interface ContentBounds {
  x: number;
  y: number;
  width: number;
  height: number;
}

export async function renderVisualization(
  target: HTMLElement,
  prepared: PreparedView,
  options: RenderOptions = {},
): Promise<RenderController> {
  target.innerHTML = "";
  const width = Math.max(720, target.clientWidth || 960);
  const height = Math.max(480, target.clientHeight || 640);
  const svg = d3
    .select(target)
    .append("svg")
    .attr("class", "sysml-viz-svg")
    .attr("width", "100%")
    .attr("height", "100%")
    .attr("viewBox", `0 0 ${width} ${height}`)
    .attr("role", "img")
    .attr("aria-label", prepared.title || "SysML view");
  svg.append("rect").attr("class", "viz-bg").attr("width", width).attr("height", height);
  svg
    .select(".viz-bg")
    .attr("fill", "var(--vscode-editor-background, transparent)");
  addMarkers(svg);

  const root = svg.append("g").attr("class", "viz-root");
  const zoom = d3.zoom<SVGSVGElement, unknown>().scaleExtent([0.08, 5]).on("zoom", (event: any) => {
    root.attr("transform", event.transform.toString());
  });
  svg.call(zoom);

  const isInterconnectionView = prepared.view === "interconnection-view";
  const layout = await layoutPrepared(prepared);
  drawEdges(root, layout.edges);
  drawNodes(root, layout.nodes, options, isInterconnectionView);
  const bounds = contentBounds(layout);
  fit(svg, zoom, bounds, width, height, isInterconnectionView);

  return {
    reset: () => fit(svg, zoom, bounds, width, height, isInterconnectionView),
    exportSvg: () => exportSvg(svg.node() as SVGSVGElement, bounds),
    destroy: () => {
      target.innerHTML = "";
    }
  };
}

async function layoutPrepared(prepared: PreparedView): Promise<LayoutResult> {
  if (!prepared.nodes.length) return { nodes: [], edges: [] };
  const isInterconnectionView = prepared.view === "interconnection-view";
  if (isInterconnectionView) {
    return layoutInterconnectionPrepared(prepared);
  }
  const width = isInterconnectionView ? ibdNodeWidth : nodeWidth;
  const height = isInterconnectionView ? ibdNodeHeight : nodeHeight;
  const graph = {
    id: "root",
    layoutOptions: {
      "elk.algorithm": "layered",
      "elk.direction": isInterconnectionView ? "RIGHT" : "DOWN",
      "elk.spacing.nodeNode": isInterconnectionView ? "80" : "48",
      "elk.layered.spacing.nodeNodeBetweenLayers": isInterconnectionView ? "110" : "72",
      "elk.edgeRouting": "ORTHOGONAL"
    },
    children: prepared.nodes.map((node) => ({ id: node.id, width, height })),
    edges: prepared.edges.map((edge) => ({ id: edge.id, sources: [edge.source], targets: [edge.target] }))
  };
  try {
    const laidOut = await elk.layout(graph);
    const byId = new Map(prepared.nodes.map((node) => [node.id, node]));
    const layouts = new Map((laidOut.children || []).map((node: any) => [String(node.id), node]));
    return {
      nodes: prepared.nodes.map((node) => ({ ...node, ...(layouts.get(node.id) || {}) })),
      edges: prepared.edges.map((edge) => ({
        ...edge,
        sourceNode: byId.get(edge.source),
        targetNode: byId.get(edge.target),
        layout: (laidOut.edges || []).find((item: any) => item.id === edge.id) as LaidOutEdge["layout"]
      }))
    };
  } catch {
    return fallbackLayout(prepared);
  }
}

async function layoutInterconnectionPrepared(prepared: PreparedView): Promise<LayoutResult> {
  const nodesById = new Map(prepared.nodes.map((node) => [node.id, node]));
  const childrenByParent = new Map<string, PreparedNode[]>();
  const roots: PreparedNode[] = [];

  for (const node of prepared.nodes) {
    const attrs = (node.attributes ?? {}) as Record<string, unknown>;
    const parentId = typeof attrs.containerId === "string" ? attrs.containerId : "";
    if (parentId && nodesById.has(parentId)) {
      const current = childrenByParent.get(parentId) ?? [];
      current.push(node);
      childrenByParent.set(parentId, current);
    } else {
      roots.push(node);
    }
  }

  const sanitizeId = (value: string) => value.replace(/[^A-Za-z0-9_.-]/g, "_");
  const portIdFor = (nodeId: string, portName: string) => `${sanitizeId(nodeId)}__port__${sanitizeId(portName)}`;
  const connectorPortName = (node: PreparedNode, endpoint: unknown): string | null => {
    const endpointText = String(endpoint ?? "").trim();
    if (!endpointText) return null;
    const endpointLeaf = endpointText.split(".").pop()?.split("::").pop()?.trim() ?? "";
    if (!endpointLeaf) return null;
    const ports = Array.isArray((node.attributes as Record<string, unknown>)?.ports)
      ? (((node.attributes as Record<string, unknown>).ports as unknown[]).map((value) => String(value)))
      : [];
    const matched = ports.find((port) => port === endpointLeaf || endpointText.endsWith(`.${port}`));
    return matched ?? null;
  };

  const sideForPort = (name: string): "WEST" | "EAST" => {
    const lower = name.toLowerCase();
    if (lower.endsWith("in") || lower.includes("input") || lower.startsWith("in")) return "WEST";
    return "EAST";
  };

  const toElkNode = (node: PreparedNode): any => {
    const ports = Array.isArray((node.attributes as Record<string, unknown>)?.ports)
      ? (((node.attributes as Record<string, unknown>).ports as unknown[]).map((value) => String(value)))
      : [];
    const children = (childrenByParent.get(node.id) ?? []).map((child) => toElkNode(child));
    const baseWidth = children.length > 0 ? 420 : ibdNodeWidth;
    const width = Math.max(
      baseWidth,
      180 + Math.max(node.label.length * 6, ...ports.map((item) => item.length * 5), 0),
    );
    const height = children.length > 0 ? 120 : Math.max(ibdNodeHeight, 90 + ports.length * 20);
    return {
      id: node.id,
      width,
      height,
      ports: ports.map((port, index) => ({
        id: portIdFor(node.id, port),
        width: 10,
        height: 10,
        layoutOptions: {
          "org.eclipse.elk.port.side": sideForPort(port),
          "org.eclipse.elk.port.index": String(index),
        },
      })),
      children,
      layoutOptions: children.length
        ? {
            "elk.padding": "[top=52,left=18,bottom=18,right=18]",
            "elk.direction": "RIGHT",
            "org.eclipse.elk.portConstraints": "FIXED_ORDER",
          }
        : {
            "org.eclipse.elk.portConstraints": "FIXED_ORDER",
          },
    };
  };

  const elkEdges = prepared.edges
    .map((edge) => {
      const sourceNode = nodesById.get(edge.source);
      const targetNode = nodesById.get(edge.target);
      if (!sourceNode || !targetNode) return null;
      const sourceEndpoint = (edge.attributes as Record<string, unknown> | undefined)?.sourceId;
      const targetEndpoint = (edge.attributes as Record<string, unknown> | undefined)?.targetId;
      const sourcePortName = connectorPortName(sourceNode, sourceEndpoint);
      const targetPortName = connectorPortName(targetNode, targetEndpoint);
      return {
        id: edge.id,
        sources: [sourcePortName ? portIdFor(sourceNode.id, sourcePortName) : sourceNode.id],
        targets: [targetPortName ? portIdFor(targetNode.id, targetPortName) : targetNode.id],
        sourcePortId: sourcePortName ? portIdFor(sourceNode.id, sourcePortName) : undefined,
        targetPortId: targetPortName ? portIdFor(targetNode.id, targetPortName) : undefined,
      };
    })
    .filter((edge): edge is NonNullable<typeof edge> => Boolean(edge));

  const graph = {
    id: "root",
    layoutOptions: {
      "elk.algorithm": "layered",
      "elk.hierarchyHandling": "INCLUDE_CHILDREN",
      "elk.direction": "RIGHT",
      "elk.spacing.nodeNode": "150",
      "elk.layered.spacing.nodeNodeBetweenLayers": "220",
      "elk.spacing.edgeNode": "110",
      "elk.spacing.edgeEdge": "90",
      "elk.edgeRouting": "ORTHOGONAL",
      "elk.layered.nodePlacement.strategy": "NETWORK_SIMPLEX",
      "elk.layered.crossingMinimization.strategy": "LAYER_SWEEP",
      "elk.separateConnectedComponents": "true",
      "elk.padding": "[top=70,left=70,bottom=70,right=70]",
      "org.eclipse.elk.portConstraints": "FIXED_ORDER",
      "org.eclipse.elk.portAlignment.default": "CENTER",
      "org.eclipse.elk.json.edgeCoords": "ROOT",
    },
    children: roots.map((node) => toElkNode(node)),
    edges: elkEdges.map((edge) => ({ id: edge.id, sources: edge.sources, targets: edge.targets })),
  };

  try {
    const laidOut = await elk.layout(graph);
    const laidOutNodes = new Map<string, LaidOutNode>();
    const portCenters = new Map<string, { x: number; y: number }>();

    const visit = (elkNode: any, ox: number, oy: number) => {
      const absX = ox + (elkNode.x ?? 0);
      const absY = oy + (elkNode.y ?? 0);
      const base = nodesById.get(String(elkNode.id));
      if (base) {
        laidOutNodes.set(base.id, {
          ...base,
          x: absX,
          y: absY,
          width: elkNode.width ?? ibdNodeWidth,
          height: elkNode.height ?? ibdNodeHeight,
        });
      }
      for (const port of elkNode.ports ?? []) {
        const pw = port.width ?? 10;
        const ph = port.height ?? 10;
        const side = port?.layoutOptions?.["org.eclipse.elk.port.side"];
        const x =
          side === "WEST"
            ? absX + (port.x ?? 0)
            : side === "EAST"
              ? absX + (port.x ?? 0) + pw
              : absX + (port.x ?? 0) + pw / 2;
        const y = absY + (port.y ?? 0) + ph / 2;
        portCenters.set(String(port.id), { x, y });
      }
      for (const child of elkNode.children ?? []) {
        visit(child, absX, absY);
      }
    };

    for (const child of laidOut.children ?? []) {
      visit(child, 0, 0);
    }

    const edgeLayout = new Map<string, any>();
    const visitEdges = (elkNode: any) => {
      for (const edge of elkNode.edges ?? []) edgeLayout.set(String(edge.id), edge);
      for (const child of elkNode.children ?? []) visitEdges(child);
    };
    visitEdges(laidOut);

    const nodes = prepared.nodes
      .map((node) => laidOutNodes.get(node.id))
      .filter((value): value is LaidOutNode => Boolean(value));

    const edges = prepared.edges.map((edge) => {
      const layout = edgeLayout.get(edge.id);
      const elkEdge = elkEdges.find((item) => item.id === edge.id);
      return {
        ...edge,
        sourceNode: nodesById.get(edge.source),
        targetNode: nodesById.get(edge.target),
        layout: layout ? { sections: layout.sections as EdgeSection[] } : undefined,
        attributes: {
          ...(edge.attributes ?? {}),
          _sourcePortCenter: elkEdge?.sourcePortId ? portCenters.get(elkEdge.sourcePortId) : undefined,
          _targetPortCenter: elkEdge?.targetPortId ? portCenters.get(elkEdge.targetPortId) : undefined,
        },
      } satisfies LaidOutEdge;
    });

    return { nodes, edges };
  } catch {
    return fallbackLayout(prepared);
  }
}

function fallbackLayout(prepared: PreparedView): LayoutResult {
  const isInterconnectionView = prepared.view === "interconnection-view";
  const width = isInterconnectionView ? ibdNodeWidth : nodeWidth;
  const height = isInterconnectionView ? ibdNodeHeight : nodeHeight;
  const columns = Math.max(1, Math.ceil(Math.sqrt(prepared.nodes.length || 1)));
  const nodes = prepared.nodes.map((node, index) => ({
    ...node,
    x: (index % columns) * (width + 60),
    y: Math.floor(index / columns) * (height + 64),
    width,
    height
  }));
  const byId = new Map(nodes.map((node) => [node.id, node]));
  return { nodes, edges: prepared.edges.map((edge) => ({ ...edge, sourceNode: byId.get(edge.source), targetNode: byId.get(edge.target) })) };
}

function drawEdges(root: d3.Selection<SVGGElement, unknown, null, undefined>, edges: LaidOutEdge[]): void {
  const group = root.append("g").attr("class", "viz-edges");
  for (const edge of edges) {
    if (!edge.sourceNode || !edge.targetNode) continue;
    const path = edge.layout?.sections?.[0] ? pathFromSection(edge.layout.sections[0]) : pathFallback(edge);
    const edgeKind = edge.edgeKind ?? normalizeEdgeKind(edge.label);
    group
      .append("path")
      .attr("class", `viz-edge viz-edge--${edgeKind}`)
      .attr("d", path)
      .attr("fill", "none")
      .attr("stroke", "var(--vscode-editor-foreground, #d0d0d0)")
      .attr("stroke-width", edgeKind === "hierarchy" ? 1.4 : 1.8)
      .attr("opacity", 0.85);
    if (edgeKind !== "hierarchy" && edge.label) {
      const midpoint = edgeMidpoint(edge);
      group
        .append("text")
        .attr("class", `viz-edge-label viz-edge-label--${edgeKind}`)
        .attr("x", midpoint.x)
        .attr("y", midpoint.y)
        .attr("text-anchor", "middle")
        .attr("dy", "-0.35em")
        .attr("fill", "var(--vscode-editor-foreground, #d0d0d0)")
        .attr("font-size", 11)
        .text(truncate(edge.label, 18));
    }
  }
}

function drawNodes(
  root: d3.Selection<SVGGElement, unknown, null, undefined>,
  nodes: LaidOutNode[],
  options: RenderOptions,
  isInterconnectionView = false,
): void {
  const groups = root
    .append("g")
    .attr("class", "viz-nodes")
    .selectAll("g")
    .data(nodes)
    .enter()
    .append("g")
    .attr("class", (d: LaidOutNode) => {
      const clickable = options.onNodeClick ? "is-clickable" : "";
      const selected = options.selectedNodeId && d.id === options.selectedNodeId ? "is-selected" : "";
      return `viz-node ${nodeAccentClass(d.kind)} ${clickable} ${selected}`.trim();
    })
    .attr("transform", (d: LaidOutNode) => `translate(${d.x || 0},${d.y || 0})`)
    .attr("data-node-id", (d: LaidOutNode) => d.id)
    .attr("data-element-name", (d: LaidOutNode) => d.label)
    .on("click", (_event: unknown, d: LaidOutNode) => options.onNodeClick?.(d));
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
}

function pathFromSection(section: EdgeSection): string {
  const points = [section.startPoint, ...(section.bendPoints || []), section.endPoint].filter(Boolean);
  return d3.line<{ x: number; y: number }>().x((d) => d.x).y((d) => d.y)(points as { x: number; y: number }[]) || "";
}

function edgeMidpoint(edge: LaidOutEdge): { x: number; y: number } {
  const section = edge.layout?.sections?.[0];
  if (section) {
    const points = [section.startPoint, ...(section.bendPoints || []), section.endPoint].filter(Boolean) as {
      x: number;
      y: number;
    }[];
    if (points.length > 0) {
      const index = Math.floor((points.length - 1) / 2);
      return points[index];
    }
  }
  const sourceNode = edge.sourceNode;
  const targetNode = edge.targetNode;
  if (sourceNode && targetNode) {
    return {
      x: ((sourceNode.x || 0) + (targetNode.x || 0) + nodeWidth) / 2,
      y: ((sourceNode.y || 0) + (targetNode.y || 0) + nodeHeight) / 2,
    };
  }
  return { x: 0, y: 0 };
}

function pathFallback(edge: LaidOutEdge): string {
  const sourceNode = edge.sourceNode;
  const targetNode = edge.targetNode;
  if (!sourceNode || !targetNode) return "";
  const sourceWidth = sourceNode.width || nodeWidth;
  const sourceHeight = sourceNode.height || nodeHeight;
  const targetWidth = targetNode.width || nodeWidth;
  const targetHeight = targetNode.height || nodeHeight;
  const sourcePortCenter = ((edge.attributes as Record<string, unknown> | undefined)?._sourcePortCenter ??
    null) as { x: number; y: number } | null;
  const targetPortCenter = ((edge.attributes as Record<string, unknown> | undefined)?._targetPortCenter ??
    null) as { x: number; y: number } | null;
  const sx = sourcePortCenter?.x ?? (sourceNode.x || 0) + sourceWidth;
  const sy = sourcePortCenter?.y ?? (sourceNode.y || 0) + sourceHeight / 2;
  const tx = targetPortCenter?.x ?? (targetNode.x || 0);
  const ty = targetPortCenter?.y ?? (targetNode.y || 0) + targetHeight / 2;
  if (!sourcePortCenter && !targetPortCenter && (targetNode.x || 0) + targetWidth < (sourceNode.x || 0)) {
    const sxLeft = sourceNode.x || 0;
    const txRight = (targetNode.x || 0) + targetWidth;
    const mid = (sxLeft + txRight) / 2;
    return `M${sxLeft},${sy} L${mid},${sy} L${mid},${ty} L${txRight},${ty}`;
  }
  const mid = (sx + tx) / 2;
  return `M${sx},${sy} L${mid},${sy} L${mid},${ty} L${tx},${ty}`;
}

function contentBounds(layout: LayoutResult): ContentBounds {
  if (!layout.nodes.length) return { x: 0, y: 0, width: 100, height: 100 };
  const minX = Math.min(...layout.nodes.map((node) => node.x || 0));
  const minY = Math.min(...layout.nodes.map((node) => node.y || 0));
  const maxX = Math.max(...layout.nodes.map((node) => (node.x || 0) + (node.width || nodeWidth)));
  const maxY = Math.max(...layout.nodes.map((node) => (node.y || 0) + (node.height || nodeHeight)));
  return { x: minX, y: minY, width: maxX - minX, height: maxY - minY };
}

function fit(
  svg: d3.Selection<SVGSVGElement, unknown, null, undefined>,
  zoom: d3.ZoomBehavior<SVGSVGElement, unknown>,
  bounds: ContentBounds,
  width: number,
  height: number,
  isInterconnectionView = false,
): void {
  const padding = 48;
  const minScale = isInterconnectionView ? 0.2 : 0.08;
  const maxScale = isInterconnectionView ? 1.1 : 1.3;
  const scale = Math.min(
    maxScale,
    Math.max(minScale, Math.min((width - padding * 2) / bounds.width, (height - padding * 2) / bounds.height)),
  );
  const tx = (width - bounds.width * scale) / 2 - bounds.x * scale;
  const ty = (height - bounds.height * scale) / 2 - bounds.y * scale;
  svg.transition().duration(180).call(zoom.transform, d3.zoomIdentity.translate(tx, ty).scale(scale));
}

function addMarkers(svg: d3.Selection<SVGSVGElement, unknown, null, undefined>): void {
  svg.append("defs").append("marker").attr("id", "viz-arrow").attr("markerWidth", 10).attr("markerHeight", 10).attr("refX", 9).attr("refY", 3).attr("orient", "auto").attr("markerUnits", "strokeWidth").append("path").attr("d", "M0,0 L0,6 L9,3 z");
}

function exportSvg(svgNode: SVGSVGElement, bounds: ContentBounds): string {
  const clone = svgNode.cloneNode(true) as SVGSVGElement;
  clone.setAttribute("xmlns", "http://www.w3.org/2000/svg");
  clone.setAttribute("viewBox", `${bounds.x - 40} ${bounds.y - 40} ${bounds.width + 80} ${bounds.height + 80}`);
  return new XMLSerializer().serializeToString(clone);
}

function truncate(value: string, max: number): string {
  const text = String(value || "");
  return text.length > max ? `${text.slice(0, max - 1)}...` : text;
}

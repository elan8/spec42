import * as d3 from "d3";
import ELK from "elkjs/lib/elk.bundled.js";
import { nodeBodyChromeStyle, resolveNodeChrome } from "./node-notation";
import { type PreparedNode, type PreparedView } from "./prepare";
import { isOverviewVisualElementType, normalizeEdgeKind } from "./graph-normalization";
import { collectCompartments, computeNodeHeight, renderSysMLNode } from "./sysml-node-builder";
import {
  resolveDiagramTheme,
  strokeColorForEdge,
  strokeColorForNode,
  type DiagramTheme,
  type DiagramThemeOverrides,
} from "./theme";
import { addActionFlowMarkers, renderActionFlowView } from "./views/action-flow";
import { renderSequenceView, addSequenceMarkers } from "./views/sequence";
import { addStateTransitionMarkers, renderStateTransitionView } from "./views/state-transition";

const elk = new ELK();
const nodeWidth = 200;
const nodeHeight = 70;
const ibdNodeWidth = 280;
const ibdNodeHeight = 140;

export interface RenderOptions {
  onNodeClick?: (node: PreparedNode) => void;
  selectedNodeId?: string | null;
  theme?: DiagramThemeOverrides;
  /** When true, skip internal d3.zoom; host (e.g. VS Code webview) attaches pan/zoom to the SVG. */
  delegateZoom?: boolean;
}

export interface RenderController {
  reset: () => void;
  exportSvg: () => string;
  destroy: () => void;
  /** Last fit-to-view transform (for hosts that delegate pan/zoom). */
  getFitTransform: () => d3.ZoomTransform;
}

interface LaidOutNode extends PreparedNode {
  x?: number;
  y?: number;
  width?: number;
  height?: number;
  compartments?: ReturnType<typeof collectCompartments>;
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

interface ContentExtents {
  minX: number;
  minY: number;
  maxX: number;
  maxY: number;
}

function contentBoundsFromExtents(extents: ContentExtents): ContentBounds {
  const width = extents.maxX - extents.minX;
  const height = extents.maxY - extents.minY;
  return {
    x: extents.minX,
    y: extents.minY,
    width: width > 0 ? width : 1,
    height: height > 0 ? height : 1,
  };
}

interface PreparedPort {
  id?: string;
  name: string;
  direction?: string;
  portType?: string;
  portSide?: string;
  attributes?: Record<string, unknown>;
}

export async function renderVisualization(
  target: HTMLElement,
  prepared: PreparedView,
  options: RenderOptions = {},
): Promise<RenderController> {
  target.innerHTML = "";
  const theme = resolveDiagramTheme(options.theme);
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
    .attr("aria-label", prepared.title || "SysML view")
    .style("touch-action", "none")
    .style("cursor", "grab");
  if (theme.colorScheme === "light" || theme.colorScheme === "dark" || theme.colorScheme === "auto") {
    const scheme =
      theme.colorScheme === "auto"
        ? typeof window !== "undefined" && window.matchMedia?.("(prefers-color-scheme: dark)")?.matches
          ? "dark"
          : "light"
        : theme.colorScheme;
    svg.attr("data-color-scheme", scheme);
  }
  svg.append("rect").attr("class", "viz-bg").attr("width", width).attr("height", height);
  svg
    .select(".viz-bg")
    .attr("fill", theme.canvasBackground);
  addMarkers(svg, theme);

  const root = svg.append("g").attr("class", "viz-root");
  const delegateZoom = options.delegateZoom === true;
  const zoom = d3.zoom<SVGSVGElement, unknown>()
    .scaleExtent([0.08, 5])
    .on("start", () => svg.style("cursor", "grabbing"))
    .on("zoom", (event: any) => {
      root.attr("transform", event.transform.toString());
    })
    .on("end", () => svg.style("cursor", "grab"));
  if (!delegateZoom) {
    svg
      .call(zoom)
      .on("dblclick.zoom", null)
      .on("wheel.zoom", function(event: WheelEvent) {
        event.preventDefault();
        event.stopPropagation();
        const mouse = d3.pointer(event, this as SVGSVGElement);
        const currentTransform = d3.zoomTransform(this as SVGSVGElement);
        const factor = event.deltaY > 0 ? 0.7 : 1.45;
        const newScale = Math.min(Math.max(currentTransform.k * factor, 0.08), 5);
        const translateX = mouse[0] - (mouse[0] - currentTransform.x) * (newScale / currentTransform.k);
        const translateY = mouse[1] - (mouse[1] - currentTransform.y) * (newScale / currentTransform.k);
        d3.select(this as SVGSVGElement)
          .transition()
          .duration(50)
          .call(zoom.transform, d3.zoomIdentity.translate(translateX, translateY).scale(newScale));
      });
  }

  const view = prepared.view;
  const isInterconnectionView = view === "interconnection-view";
  const isBehaviorView =
    view === "action-flow-view" || view === "state-transition-view" || view === "sequence-view";

  let bounds: ContentBounds;
  if (view === "action-flow-view") {
    addActionFlowMarkers(svg.select("defs").empty() ? svg.append("defs") : svg.select("defs"), theme);
    bounds = contentBoundsFromExtents(await renderActionFlowView({ root, prepared, theme, width, height }));
  } else if (view === "state-transition-view") {
    addStateTransitionMarkers(svg.select("defs").empty() ? svg.append("defs") : svg.select("defs"), theme);
    bounds = contentBoundsFromExtents(await renderStateTransitionView({ root, prepared, theme, width, height }));
  } else if (view === "sequence-view") {
    addSequenceMarkers(svg.select("defs").empty() ? svg.append("defs") : svg.select("defs"), theme);
    bounds = contentBoundsFromExtents(renderSequenceView({ root, prepared, theme, width, height }));
  } else {
    const layout = await layoutPrepared(prepared);
    if (isInterconnectionView) {
      if (shouldDrawIbdViewFrame(prepared)) {
        drawIbdViewFrame(root, prepared, contentBounds(layout), theme);
      }
      drawInterconnectionContainers(root, prepared, layout.nodes, theme);
      drawNodes(root, layout.nodes, options, isInterconnectionView, theme);
      drawEdges(root, layout.edges, isInterconnectionView, theme);
    } else {
      drawEdges(root, layout.edges, isInterconnectionView, theme);
      drawNodes(root, layout.nodes, options, isInterconnectionView, theme);
    }
    bounds = contentBounds(layout);
  }

  let lastFitTransform = d3.zoomIdentity;
  const fitView = () => {
    lastFitTransform = applyFit(
      svg,
      zoom,
      root,
      bounds,
      width,
      height,
      isInterconnectionView || isBehaviorView,
      delegateZoom,
    );
  };
  fitView();

  return {
    reset: () => fitView(),
    getFitTransform: () => lastFitTransform,
    exportSvg: () => exportSvg(svg.node() as SVGSVGElement, bounds),
    destroy: () => {
      target.innerHTML = "";
    },
  };
}

async function layoutPrepared(prepared: PreparedView): Promise<LayoutResult> {
  if (!prepared.nodes.length) return { nodes: [], edges: [] };
  const isInterconnectionView = prepared.view === "interconnection-view";
  if (
    prepared.view === "action-flow-view" ||
    prepared.view === "state-transition-view" ||
    prepared.view === "sequence-view"
  ) {
    return { nodes: [], edges: [] };
  }
  if (isInterconnectionView) {
    return layoutInterconnectionPrepared(prepared);
  }
  const diagramNodes = prepared.nodes.filter((node) => isOverviewVisualElementType(node.kind));
  const visibleIds = new Set(diagramNodes.map((node) => node.id));
  const diagramEdges = prepared.edges.filter(
    (edge) => visibleIds.has(edge.source) && visibleIds.has(edge.target),
  );
  if (!diagramNodes.length) return { nodes: [], edges: [] };
  const width = isInterconnectionView ? ibdNodeWidth : nodeWidth;
  const height = isInterconnectionView ? ibdNodeHeight : nodeHeight;
  const graph = {
    id: "root",
    layoutOptions: {
      "elk.algorithm": "layered",
      "elk.direction": isInterconnectionView ? "RIGHT" : "DOWN",
      "elk.spacing.nodeNode": isInterconnectionView ? "80" : "220",
      "elk.layered.spacing.nodeNodeBetweenLayers": isInterconnectionView ? "110" : "280",
      "elk.spacing.edgeNode": isInterconnectionView ? "80" : "120",
      "elk.spacing.edgeEdge": isInterconnectionView ? "60" : "120",
      "elk.edgeRouting": "ORTHOGONAL",
      "elk.layered.nodePlacement.strategy": "NETWORK_SIMPLEX",
      "elk.separateConnectedComponents": "true",
      "elk.aspectRatio": isInterconnectionView ? "1.6" : "1.4",
      "elk.padding": isInterconnectionView ? "[top=70,left=70,bottom=70,right=70]" : "[top=100,left=100,bottom=100,right=100]",
      "org.eclipse.elk.portConstraints": "FIXED_SIDE",
      "org.eclipse.elk.json.edgeCoords": "ROOT"
    },
    children: diagramNodes.map((node) => {
      const compartments = collectCompartments(node);
      return {
        id: node.id,
        width,
        height: Math.max(height, computeNodeHeight(compartments, { maxLinesPerCompartment: 8 })),
      };
    }),
    edges: diagramEdges.map((edge) => ({ id: edge.id, sources: [edge.source], targets: [edge.target] }))
  };
  try {
    const laidOut = await elk.layout(graph);
    const byId = new Map(diagramNodes.map((node) => [node.id, node]));
    const layouts = new Map((laidOut.children || []).map((node: any) => [String(node.id), node]));
    return {
      nodes: diagramNodes.map((node) => {
        const compartments = collectCompartments(node);
        return { ...node, compartments, ...(layouts.get(node.id) || {}) };
      }),
      edges: diagramEdges.map((edge) => ({
        ...edge,
        sourceNode: byId.get(edge.source),
        targetNode: byId.get(edge.target),
        layout: (laidOut.edges || []).find((item: any) => item.id === edge.id) as LaidOutEdge["layout"]
      }))
    };
  } catch {
    return fallbackLayout({ ...prepared, nodes: diagramNodes, edges: diagramEdges });
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
  const portDetailsFor = (node: PreparedNode): PreparedPort[] => {
    const attrs = (node.attributes ?? {}) as Record<string, unknown>;
    const details = Array.isArray(attrs.portDetails) ? attrs.portDetails : [];
    if (details.length > 0) {
      return details
        .map((item) => item && typeof item === "object" ? item as PreparedPort : null)
        .filter((item): item is PreparedPort => Boolean(item?.name));
    }
    return Array.isArray(attrs.ports)
      ? (attrs.ports as unknown[]).map((name) => ({ name: String(name) }))
      : [];
  };
  const normalizeEndpoint = (value: unknown): string => String(value ?? "").replace(/::/g, ".").trim();
  const portLayoutKey = (node: PreparedNode, port: PreparedPort): string => {
    const attrs = (node.attributes ?? {}) as Record<string, unknown>;
    const explicit = normalizeEndpoint(port.id);
    if (explicit) return explicit;
    const parent = normalizeEndpoint(port.attributes?.parentId ?? (port as Record<string, unknown>).parentId ?? attrs.qualifiedName ?? node.id ?? node.label);
    return parent ? `${parent}.${port.name}` : normalizeEndpoint(port.name);
  };
  const portUsage = new Map<string, { sourceCount: number; targetCount: number }>();
  const bumpPortUsage = (endpoint: unknown, role: "sourceCount" | "targetCount") => {
    const normalized = normalizeEndpoint(endpoint);
    if (!normalized) return;
    const current = portUsage.get(normalized) ?? { sourceCount: 0, targetCount: 0 };
    current[role] += 1;
    portUsage.set(normalized, current);
  };
  for (const edge of prepared.edges) {
    bumpPortUsage(edge.attributes?.sourceId ?? edge.source, "sourceCount");
    bumpPortUsage(edge.attributes?.targetId ?? edge.target, "targetCount");
  }
  const usageForPort = (node: PreparedNode, port: PreparedPort): { sourceCount: number; targetCount: number } => {
    const key = portLayoutKey(node, port);
    const explicit = portUsage.get(key);
    if (explicit) return explicit;
    const attrs = (node.attributes ?? {}) as Record<string, unknown>;
    const parent = normalizeEndpoint(attrs.qualifiedName ?? node.id ?? node.label);
    const fallback = portUsage.get(`${parent}.${normalizeEndpoint(port.name)}`);
    return fallback ?? { sourceCount: 0, targetCount: 0 };
  };
  const connectorPortName = (node: PreparedNode, endpoint: unknown): string | null => {
    const endpointText = String(endpoint ?? "").trim();
    if (!endpointText) return null;
    const endpointLeaf = endpointText.split(".").pop()?.split("::").pop()?.trim() ?? "";
    if (!endpointLeaf) return null;
    const ports = portDetailsFor(node).map((port) => port.name);
    const matched = ports.find((port) => port === endpointLeaf || endpointText.endsWith(`.${port}`));
    return matched ?? null;
  };

  const sideForPort = (port: PreparedPort, node: PreparedNode): "WEST" | "EAST" => {
    const explicit = String(port.portSide || port.attributes?.portSide || "").toLowerCase();
    if (explicit === "left" || explicit === "west") return "WEST";
    if (explicit === "right" || explicit === "east") return "EAST";
    const direction = String(port.direction || "").toLowerCase();
    if (direction === "in") return "WEST";
    if (direction === "out") return "EAST";
    const usage = usageForPort(node, port);
    if (usage.targetCount > usage.sourceCount) return "WEST";
    if (usage.sourceCount > usage.targetCount) return "EAST";
    const lower = port.name.toLowerCase();
    const portType = String(port.portType || port.attributes?.portType || "").toLowerCase();
    if (lower.endsWith("in") || lower.includes("input") || lower.startsWith("in")) return "WEST";
    if (lower.endsWith("out") || lower.startsWith("out")) return "EAST";
    if (portType.startsWith("~") && /(powerport|telemetryport|sensordataport|gimbalcommandport|cameracontrolport)/.test(portType)) {
      return "WEST";
    }
    if (!portType.startsWith("~") && /(powerport|telemetryport|sensordataport)/.test(portType)) {
      return "EAST";
    }
    const nodeText = `${node.label} ${String(node.attributes?.qualifiedName || "")}`.toLowerCase();
    const prefersLeft = /(sensor|imu|barometer|gnss|receiver|battery|input|telemetryin|videoin|c2in|rcin|sensorin)/.test(nodeText)
      || /(cmd$|control$|input|telemetryin|videoin|sensorin|mainpower)/.test(lower);
    const prefersRight = /(camera|gimbal|propulsion|motor|radio|communication|distribution|controller|payload|actuator)/.test(nodeText)
      || /(videoout|telemetryout|regulated|pwr|cmd|ctrl)/.test(lower);
    if (prefersLeft && !prefersRight) return "WEST";
    if (prefersRight && !prefersLeft) return "EAST";
    return "EAST";
  };

  const toElkNode = (node: PreparedNode): any => {
    const ports = portDetailsFor(node);
    const children = (childrenByParent.get(node.id) ?? []).map((child) => toElkNode(child));
    const isContainer = Boolean((node.attributes ?? {}).isSyntheticContainer) || children.length > 0;
    const baseWidth = isContainer ? 420 : ibdNodeWidth;
    const width = Math.max(
      baseWidth,
      180 + Math.max(node.label.length * 6, ...ports.map((item) => item.name.length * 5), 0),
    );
    const height = isContainer ? 92 : Math.max(ibdNodeHeight, 90 + ports.length * 26);
    return {
      id: node.id,
      width,
      height,
      ports: ports.map((port, index) => ({
        id: portIdFor(node.id, port.name),
        width: 10,
        height: 10,
        layoutOptions: {
          "org.eclipse.elk.port.side": sideForPort(port, node),
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
    const nodePortAnchors = new Map<string, Record<string, { x: number; y: number; side: string }>>();

    const visit = (elkNode: any, ox: number, oy: number, depth: number) => {
      const absX = ox + (elkNode.x ?? 0);
      const absY = oy + (elkNode.y ?? 0);
      const base = nodesById.get(String(elkNode.id));
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
        if (base) {
          const portName = String(port.id).split("__port__").pop() ?? String(port.id);
          const anchors = nodePortAnchors.get(base.id) ?? {};
          anchors[portName] = { x: x - absX, y: y - absY, side: String(side || "") };
          nodePortAnchors.set(base.id, anchors);
        }
      }
      if (base) {
        const hasLayoutChildren = Array.isArray(elkNode.children) && elkNode.children.length > 0;
        laidOutNodes.set(base.id, {
          ...base,
          x: absX,
          y: absY,
          width: elkNode.width ?? ibdNodeWidth,
          height: elkNode.height ?? ibdNodeHeight,
          attributes: {
            ...(base.attributes ?? {}),
            _isLayoutContainer: hasLayoutChildren,
            _layoutDepth: depth,
            _portAnchors: nodePortAnchors.get(base.id),
          },
        });
      }
      for (const child of elkNode.children ?? []) {
        visit(child, absX, absY, depth + 1);
      }
    };

    for (const child of laidOut.children ?? []) {
      visit(child, 0, 0, 0);
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
        sourceNode: laidOutNodes.get(edge.source),
        targetNode: laidOutNodes.get(edge.target),
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
    compartments: collectCompartments(node),
    x: (index % columns) * (width + 60),
    y: Math.floor(index / columns) * (height + 64),
    width,
    height: Math.max(height, computeNodeHeight(collectCompartments(node), { maxLinesPerCompartment: 8 })),
  }));
  const byId = new Map(nodes.map((node) => [node.id, node]));
  return { nodes, edges: prepared.edges.map((edge) => ({ ...edge, sourceNode: byId.get(edge.source), targetNode: byId.get(edge.target) })) };
}

function drawEdges(
  root: d3.Selection<SVGGElement, unknown, null, undefined>,
  edges: LaidOutEdge[],
  isInterconnectionView: boolean,
  theme: DiagramTheme,
): void {
  const group = root.append("g").attr("class", "viz-edges");
  for (const edge of edges) {
    if (!edge.sourceNode || !edge.targetNode) continue;
    const path = edge.layout?.sections?.[0] ? pathFromSection(edge.layout.sections[0]) : pathFallback(edge);
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
      const midpoint = edgeMidpoint(edge);
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

function drawNodes(
  root: d3.Selection<SVGGElement, unknown, null, undefined>,
  nodes: LaidOutNode[],
  options: RenderOptions,
  isInterconnectionView: boolean,
  theme: DiagramTheme,
): void {
  const renderNodes = isInterconnectionView ? orderIbdNodesForPaint(nodes) : nodes;
  const groups = root
    .append("g")
    .attr("class", "viz-nodes")
    .selectAll("g")
    .data(renderNodes)
    .enter()
    .append("g")
    .attr("class", (d: LaidOutNode) => {
      const clickable = options.onNodeClick ? "is-clickable" : "";
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
    .on("click", (_event: unknown, d: LaidOutNode) => options.onNodeClick?.(d));

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
      renderIbdNode(group as any, d, Boolean(options.selectedNodeId && d.id === options.selectedNodeId), theme);
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
  } else if (edgeKind === "composition") {
    path.attr("stroke", strokeColorForEdge(edgeKind, theme)).style("marker-start", "url(#general-d3-diamond)").style("marker-end", "none").style("stroke-dasharray", "6,3");
  } else if (edgeKind === "satisfy" || edgeKind === "verify") {
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
    drawIbdPorts(group, node, width, headerHeight, theme);
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

  drawIbdPorts(group, node, width, contentStartY + 20, theme);
}

function drawIbdPorts(
  group: d3.Selection<SVGGElement, LaidOutNode, null, undefined>,
  node: LaidOutNode,
  width: number,
  fallbackStartY: number,
  theme: DiagramTheme,
): void {
  const attrs = (node.attributes ?? {}) as Record<string, unknown>;
  const details = Array.isArray(attrs.portDetails) ? attrs.portDetails as PreparedPort[] : [];
  const portNames = details.length > 0
    ? details.map((port) => port.name)
    : Array.isArray(attrs.ports) ? (attrs.ports as unknown[]).map((port) => String(port)) : [];
  const anchors = (attrs._portAnchors && typeof attrs._portAnchors === "object" ? attrs._portAnchors : {}) as Record<string, { x: number; y: number; side: string }>;
  const portSize = 10;
  const fallbackSpacing = 26;
  portNames.forEach((name, index) => {
    const detail = details.find((port) => port.name === name);
    const sanitized = name.replace(/[^A-Za-z0-9_.-]/g, "_");
    const anchor = anchors[sanitized] ?? anchors[name];
    const side = anchor?.side || (name.toLowerCase().startsWith("in") ? "WEST" : "EAST");
    const x = anchor?.x ?? (side === "WEST" ? 0 : width);
    const y = anchor?.y ?? (fallbackStartY + index * fallbackSpacing);
    const color = theme.nodeBorder;
    group
      .append("rect")
      .attr("class", "port-icon")
      .attr("data-port-name", name)
      .attr("data-port-side", side)
      .attr("x", x - portSize / 2)
      .attr("y", y - portSize / 2)
      .attr("width", portSize)
      .attr("height", portSize)
      .style("fill", "none")
      .style("stroke", color)
      .style("stroke-width", "1.8px");
    group
      .append("text")
      .attr("x", side === "WEST" ? Math.min(width - 10, x + 16) : Math.max(10, x - 16))
      .attr("y", y + 3)
      .attr("text-anchor", side === "WEST" ? "start" : "end")
      .text(truncate(formatIbdPortLabel(name, detail), 24))
      .style("font-size", "8px")
      .style("font-weight", "500")
      .style("fill", color);
  });
}

function formatIbdPortLabel(name: string, detail?: PreparedPort): string {
  const type = String(detail?.portType || detail?.attributes?.portType || "").trim();
  if (!type) return name;
  const conjugated = type.startsWith("~");
  const cleanType = type.replace(/^~/, "").split(/::|\./).pop() || type.replace(/^~/, "");
  return `${name}: ${conjugated ? "~" : ""}${cleanType}`;
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

function drawInterconnectionContainers(
  root: d3.Selection<SVGGElement, unknown, null, undefined>,
  prepared: PreparedView,
  nodes: LaidOutNode[],
  theme: DiagramTheme,
): void {
  if (prepared.nodes.some((node) => Boolean((node.attributes ?? {}).isSyntheticContainer))) return;
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

function shouldDrawIbdViewFrame(prepared: PreparedView): boolean {
  return !prepared.nodes.some((node) => Boolean((node.attributes ?? {}).isDiagramRoot));
}

function drawIbdViewFrame(
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

function applyFit(
  svg: d3.Selection<SVGSVGElement, unknown, null, undefined>,
  zoom: d3.ZoomBehavior<SVGSVGElement, unknown>,
  root: d3.Selection<SVGGElement, unknown, null, undefined>,
  bounds: ContentBounds,
  width: number,
  height: number,
  isInterconnectionView = false,
  delegateZoom = false,
): d3.ZoomTransform {
  const padding = 48;
  const minScale = isInterconnectionView ? 0.2 : 0.08;
  const maxScale = isInterconnectionView ? 1.1 : 1.3;
  const scale = Math.min(
    maxScale,
    Math.max(minScale, Math.min((width - padding * 2) / bounds.width, (height - padding * 2) / bounds.height)),
  );
  const tx = (width - bounds.width * scale) / 2 - bounds.x * scale;
  const ty = (height - bounds.height * scale) / 2 - bounds.y * scale;
  const transform = d3.zoomIdentity.translate(tx, ty).scale(scale);
  if (delegateZoom) {
    // Host applies this via d3.zoom; keep attr in sync for first paint before host wiring.
    root.attr("transform", transform.toString());
    return transform;
  }
  svg.transition().duration(180).call(zoom.transform, transform);
  return transform;
}

function addMarkers(svg: d3.Selection<SVGSVGElement, unknown, null, undefined>, theme: DiagramTheme): void {
  const defs = svg.append("defs");
  defs.append("marker").attr("id", "viz-arrow").attr("markerWidth", 10).attr("markerHeight", 10).attr("refX", 9).attr("refY", 3).attr("orient", "auto").attr("markerUnits", "strokeWidth").append("path").attr("d", "M0,0 L0,6 L9,3 z").attr("fill", theme.edge.default);
  defs.append("marker").attr("id", "general-d3-arrow").attr("viewBox", "0 -5 10 10").attr("refX", 8).attr("refY", 0).attr("markerWidth", 5).attr("markerHeight", 5).attr("orient", "auto").append("path").attr("d", "M0,-4L10,0L0,4").style("fill", theme.edge.default);
  defs.append("marker").attr("id", "general-d3-arrow-open").attr("viewBox", "0 -5 10 10").attr("refX", 9).attr("refY", 0).attr("markerWidth", 8).attr("markerHeight", 8).attr("orient", "auto").append("path").attr("d", "M0,-4L10,0L0,4").style("fill", "none").style("stroke", theme.edge.default).style("stroke-width", "1.3");
  defs.append("marker").attr("id", "general-d3-specializes").attr("viewBox", "0 -6 12 12").attr("refX", 11).attr("refY", 0).attr("markerWidth", 8).attr("markerHeight", 8).attr("orient", "auto").append("path").attr("d", "M0,0L10,-4L10,4Z").style("fill", theme.nodeFill).style("stroke", theme.edge.default).style("stroke-width", "1.2");
  defs.append("marker").attr("id", "general-d3-diamond").attr("viewBox", "0 -6 12 12").attr("refX", 2).attr("refY", 0).attr("markerWidth", 7).attr("markerHeight", 7).attr("orient", "auto").append("path").attr("d", "M0,0L5,-4L10,0L5,4Z").style("fill", theme.edge.default);
  defs.append("marker").attr("id", "ibd-connection-dot").attr("viewBox", "-5 -5 10 10").attr("refX", 0).attr("refY", 0).attr("markerWidth", 5).attr("markerHeight", 5).attr("orient", "auto").append("circle").attr("r", 3).style("fill", theme.nodeFill).style("stroke", theme.edge.default).style("stroke-width", "1.5");
  defs.append("marker").attr("id", "ibd-flow-arrow").attr("viewBox", "0 -5 10 10").attr("refX", 10).attr("refY", 0).attr("markerWidth", 8).attr("markerHeight", 8).attr("orient", "auto").append("path").attr("d", "M0,-4L10,0L0,4Z").style("fill", theme.edge.default);
  defs.append("marker").attr("id", "ibd-interface-arrow").attr("viewBox", "0 -5 10 10").attr("refX", 10).attr("refY", 0).attr("markerWidth", 8).attr("markerHeight", 8).attr("orient", "auto").append("path").attr("d", "M0,-4L10,0L0,4Z").style("fill", "none").style("stroke", theme.edge.default).style("stroke-width", "1.5");
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

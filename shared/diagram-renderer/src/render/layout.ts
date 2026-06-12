import ELK from "elkjs/lib/elk.bundled.js";
import { isOverviewVisualElementType, normalizeEdgeKind } from "../graph-normalization";
import { collectCompartments, computeNodeHeight } from "../sysml-node-builder";
import type { InterconnectionLayoutDto, PreparedNode, PreparedView } from "../prepare";
import { lcaOffsetForNodes, pointsFromElkSections, resolveIbdRoutePoints } from "./ibd-route";
import {
  computeIbdLeafHeight,
  ibdNodeHeight,
  ibdNodeWidth,
  nodeHeight,
  nodeWidth,
  splitIbdPortsBySide,
  type EdgeSection,
  type LaidOutEdge,
  type LaidOutNode,
  type LayoutResult,
  type PreparedPort,
} from "./types";

const elk = new ELK();

export async function layoutPrepared(prepared: PreparedView): Promise<LayoutResult> {
  if (!prepared.nodes.length) return { nodes: [], edges: [] };
  const isInterconnectionView = prepared.view === "interconnection-view";
  if (
    prepared.view === "action-flow-view" ||
    prepared.view === "state-transition-view" ||
    prepared.view === "sequence-view" ||
    prepared.view === "browser-view" ||
    prepared.view === "grid-view" ||
    prepared.view === "geometry-view"
  ) {
    return { nodes: [], edges: [] };
  }
  if (isInterconnectionView) {
    const layout = await layoutInterconnectionPrepared(prepared);
    if (prepared.meta?.canonicalScene) {
      const layoutDto: InterconnectionLayoutDto = {
        nodes: layout.nodes.map((node) => ({
          id: node.id,
          x: node.x ?? 0,
          y: node.y ?? 0,
          width: node.width ?? 0,
          height: node.height ?? 0,
          portAnchors:
            ((node.attributes ?? {})._portAnchors as Record<string, { x: number; y: number; side: string }>) ??
            {},
        })),
        edges: layout.edges.map((edge) => ({
          id: edge.id,
          routePoints: resolveIbdRoutePoints(edge) ?? [],
          sourcePortId: String(edge.attributes?.sourcePortId ?? ""),
          targetPortId: String(edge.attributes?.targetPortId ?? ""),
        })),
        diagnostics: [],
      };
      for (const edge of layout.edges) {
        const routePoints = layoutDto.edges.find((item) => item.id === edge.id)?.routePoints;
        if (routePoints?.length) {
          edge.attributes = { ...(edge.attributes ?? {}), layoutRoutePoints: routePoints };
        }
      }
      return { ...layout, interconnectionLayout: layoutDto };
    }
    return layout;
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
    // Match interconnection policy: no heuristic grid when ELK fails.
    return { nodes: [], edges: [] };
  }
}

export async function layoutInterconnectionPrepared(prepared: PreparedView): Promise<LayoutResult> {
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
  const elkIdFor = (preparedNodeId: string) => sanitizeId(preparedNodeId);
  const preparedIdForElkId = new Map<string, string>();
  const registerElkId = (preparedId: string) => {
    const elkId = elkIdFor(preparedId);
    preparedIdForElkId.set(elkId, preparedId);
    return elkId;
  };
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
  const portLayoutKeys = (node: PreparedNode, port: PreparedPort): string[] => {
    const attrs = (node.attributes ?? {}) as Record<string, unknown>;
    const keys: string[] = [];
    const explicit = normalizeEndpoint(port.id);
    if (explicit) keys.push(explicit);
    const parent = normalizeEndpoint(
      port.attributes?.parentId ??
        (port as unknown as Record<string, unknown>).parentId ??
        attrs.qualifiedName ??
        node.id ??
        node.label,
    );
    if (parent) keys.push(`${parent}.${port.name}`);
    keys.push(normalizeEndpoint(port.name));
    return [...new Set(keys.filter(Boolean))];
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
    for (const key of portLayoutKeys(node, port)) {
      const explicit = portUsage.get(key);
      if (explicit) return explicit;
    }
    const attrs = (node.attributes ?? {}) as Record<string, unknown>;
    const parent = normalizeEndpoint(attrs.qualifiedName ?? node.id ?? node.label);
    const fallback = portUsage.get(`${parent}.${normalizeEndpoint(port.name)}`);
    if (fallback) return fallback;

    const aliases = [
      normalizeEndpoint(node.id),
      normalizeEndpoint(node.label),
      normalizeEndpoint(attrs.qualifiedName),
    ].filter(Boolean);
    const portName = normalizeEndpoint(port.name);
    const usage = { sourceCount: 0, targetCount: 0 };
    for (const [endpoint, counts] of portUsage) {
      if (!endpoint.endsWith(`.${portName}`) && endpoint !== portName) continue;
      const owner = endpoint === portName ? "" : endpoint.slice(0, -portName.length - 1);
      const matchesOwner = aliases.some((alias) =>
        owner === alias ||
        owner.endsWith(`.${alias}`) ||
        alias.endsWith(`.${owner}`) ||
        owner.endsWith(`.${node.label}`),
      );
      if (!matchesOwner) continue;
      usage.sourceCount += counts.sourceCount;
      usage.targetCount += counts.targetCount;
    }
    return usage;
  };
  const connectorPortName = (node: PreparedNode, endpoint: unknown): string | null => {
    const endpointText = String(endpoint ?? "").trim();
    if (!endpointText) return null;
    const ports = portDetailsFor(node);
    const canonicalMatch = ports.find(
      (port) =>
        port.id === endpointText ||
        port.attributes?.scenePortId === endpointText ||
        port.attributes?.semanticId === endpointText,
    );
    if (canonicalMatch) return canonicalMatch.name;
    const endpointLeaf = endpointText.split(".").pop()?.split("::").pop()?.trim() ?? "";
    if (!endpointLeaf) return null;
    const portNames = ports.map((port) => port.name);
    const matched = portNames.find((port) => port === endpointLeaf || endpointText.endsWith(`.${port}`));
    return matched ?? null;
  };

  const sideForPort = (port: PreparedPort, node: PreparedNode): "WEST" | "EAST" => {
    const sideHint = String(port.attributes?.sideHint || "").toLowerCase();
    if (sideHint === "west") return "WEST";
    if (sideHint === "east") return "EAST";
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

  const rootHeaderHeight = 28;
  const containerTopInset = rootHeaderHeight + 20;

  const toElkNode = (node: PreparedNode): any => {
    const ports = portDetailsFor(node);
    const { west: westPorts, east: eastPorts } = splitIbdPortsBySide(node, ports, sideForPort, usageForPort);
    const portRows = Math.max(westPorts.length, eastPorts.length, ports.length > 0 ? 1 : 0);
    const children = (childrenByParent.get(node.id) ?? []).map((child) => toElkNode(child));
    const attrs = (node.attributes ?? {}) as Record<string, unknown>;
    const isSyntheticPackage = Boolean(attrs.isSyntheticPackage);
    const isContainer = Boolean(attrs.isSyntheticContainer) || children.length > 0;
    const baseWidth = isContainer ? 420 : ibdNodeWidth;
    let width = Math.max(
      baseWidth,
      180 + Math.max(node.label.length * 6, ...ports.map((item) => item.name.length * 5), 0),
    );
    let height = isContainer
      ? rootHeaderHeight + 72
      : computeIbdLeafHeight(node, ports, portRows);
    if (isContainer && children.length > 0) {
      const childWidthSum = children.reduce((sum: number, child: { width?: number }) => sum + (child.width ?? ibdNodeWidth), 0);
      width = isSyntheticPackage
        ? Math.max(width, Math.min(980, childWidthSum + children.length * 44))
        : Math.max(width, Math.min(1040, childWidthSum + children.length * 72));
      height = isSyntheticPackage
        ? rootHeaderHeight + 72
        : rootHeaderHeight + Math.max(72, Math.min(132, 58 + children.length * 14));
    }
    const buildElkPort = (port: PreparedPort, side: "WEST" | "EAST", index: number) => ({
      id: portIdFor(node.id, port.name),
      width: 10,
      height: 10,
      layoutOptions: {
        "org.eclipse.elk.port.side": side,
        "org.eclipse.elk.port.index": String(index),
      },
    });
    return {
      id: registerElkId(node.id),
      width,
      height,
      ports: [
        ...westPorts.map((port, index) => buildElkPort(port, "WEST", index)),
        ...eastPorts.map((port, index) => buildElkPort(port, "EAST", index)),
      ],
      children,
      layoutOptions: children.length
        ? {
            "elk.padding": isSyntheticPackage
              ? `[top=${rootHeaderHeight + 12},left=16,bottom=16,right=16]`
              : `[top=${containerTopInset},left=24,bottom=24,right=24]`,
            "elk.direction": isSyntheticPackage ? "DOWN" : "RIGHT",
            "org.eclipse.elk.portConstraints": "FIXED_ORDER",
            "org.eclipse.elk.portAlignment.default": "CENTER",
          }
        : {
            "org.eclipse.elk.portConstraints": "FIXED_ORDER",
            "org.eclipse.elk.portAlignment.default": "CENTER",
          },
    };
  };

  const elkEdges = prepared.edges
    .map((edge) => {
      const sourceNode = nodesById.get(edge.source);
      const targetNode = nodesById.get(edge.target);
      if (!sourceNode || !targetNode) return null;
      const edgeAttrs = edge.attributes as Record<string, unknown> | undefined;
      const sourceEndpoint = edgeAttrs?.sourcePortId ?? edgeAttrs?.sourceId;
      const targetEndpoint = edgeAttrs?.targetPortId ?? edgeAttrs?.targetId;
      const sourcePortName = connectorPortName(sourceNode, sourceEndpoint);
      const targetPortName = connectorPortName(targetNode, targetEndpoint);
      return {
        id: edge.id,
        sources: [sourcePortName ? portIdFor(sourceNode.id, sourcePortName) : elkIdFor(sourceNode.id)],
        targets: [targetPortName ? portIdFor(targetNode.id, targetPortName) : elkIdFor(targetNode.id)],
        sourcePortId: sourcePortName ? portIdFor(sourceNode.id, sourcePortName) : undefined,
        targetPortId: targetPortName ? portIdFor(targetNode.id, targetPortName) : undefined,
      };
    })
    .filter((edge): edge is NonNullable<typeof edge> => Boolean(edge));

  const nodeBoundaryPoint = (node: LaidOutNode, role: "source" | "target"): { x: number; y: number } => ({
    x: (node.x ?? 0) + (role === "source" ? (node.width ?? ibdNodeWidth) : 0),
    y: (node.y ?? 0) + (node.height ?? ibdNodeHeight) / 2,
  });
  const fallbackEdgeSections = (
    sourceNode: LaidOutNode | undefined,
    targetNode: LaidOutNode | undefined,
    sourcePortCenter?: { x: number; y: number },
    targetPortCenter?: { x: number; y: number },
  ): EdgeSection[] | undefined => {
    if (!sourceNode || !targetNode) return undefined;
    const startPoint = sourcePortCenter ?? nodeBoundaryPoint(sourceNode, "source");
    const endPoint = targetPortCenter ?? nodeBoundaryPoint(targetNode, "target");
    const midX = (startPoint.x + endPoint.x) / 2;
    return [
      {
        startPoint,
        bendPoints: [
          { x: midX, y: startPoint.y },
          { x: midX, y: endPoint.y },
        ],
        endPoint,
      },
    ];
  };

  const elkGraphInput = {
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
    const laidOut = await elk.layout(elkGraphInput);
    const laidOutNodes = new Map<string, LaidOutNode>();
    const portCenters = new Map<string, { x: number; y: number }>();
    const nodePortAnchors = new Map<string, Record<string, { x: number; y: number; side: string }>>();

    const visit = (elkNode: any, ox: number, oy: number, depth: number) => {
      const absX = ox + (elkNode.x ?? 0);
      const absY = oy + (elkNode.y ?? 0);
      const preparedId = preparedIdForElkId.get(String(elkNode.id)) ?? String(elkNode.id);
      const base = nodesById.get(preparedId);
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
            _portDrawOrder: (() => {
              const ports = portDetailsFor(base);
              const { west, east } = splitIbdPortsBySide(base, ports, sideForPort, usageForPort);
              return { west: west.map((port) => port.name), east: east.map((port) => port.name) };
            })(),
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

    const edgeLayout = new Map<string, { edge: any; offset: { x: number; y: number } }>();
    const collectElkEdgesWithOffsets = (
      elkNode: any,
      containerOffset: { x: number; y: number },
    ) => {
      for (const elkEdge of elkNode.edges ?? []) {
        const edgeId = String(elkEdge?.id ?? "");
        if (!edgeId) continue;
        edgeLayout.set(edgeId, { edge: elkEdge, offset: containerOffset });
      }
      for (const child of elkNode.children ?? []) {
        collectElkEdgesWithOffsets(child, {
          x: containerOffset.x + (child.x ?? 0),
          y: containerOffset.y + (child.y ?? 0),
        });
      }
    };
    const rootElkEdges = laidOut.edges ?? [];
    if (rootElkEdges.length > 0) {
      for (const elkEdge of rootElkEdges) {
        const edgeId = String(elkEdge?.id ?? "");
        if (!edgeId) continue;
        edgeLayout.set(edgeId, { edge: elkEdge, offset: { x: 0, y: 0 } });
      }
    } else {
      collectElkEdgesWithOffsets(laidOut, { x: 0, y: 0 });
    }

    const nodes = prepared.nodes
      .map((node) => laidOutNodes.get(node.id))
      .filter((value): value is LaidOutNode => Boolean(value));

    const edges = prepared.edges.map((edge) => {
      const layoutRecord = edgeLayout.get(edge.id);
      const elkEdge = elkEdges.find((item) => item.id === edge.id);
      const sourceNode = laidOutNodes.get(edge.source);
      const targetNode = laidOutNodes.get(edge.target);
      const sourcePortCenter = elkEdge?.sourcePortId ? portCenters.get(elkEdge.sourcePortId) : undefined;
      const targetPortCenter = elkEdge?.targetPortId ? portCenters.get(elkEdge.targetPortId) : undefined;
      if (
        prepared.meta?.canonicalScene &&
        (edge.attributes?.sourcePortId || edge.attributes?.targetPortId) &&
        (!sourcePortCenter || !targetPortCenter)
      ) {
        console.warn("[spec42][interconnection-layout] node-boundary fallback", {
          edgeId: edge.id,
          sourcePortId: edge.attributes?.sourcePortId,
          targetPortId: edge.attributes?.targetPortId,
        });
      }
      return {
        ...edge,
        sourceNode,
        targetNode,
        layout: layoutRecord?.edge.sections?.length
          ? {
              sections: layoutRecord.edge.sections as EdgeSection[],
              edgeOwnerOffset: prepared.meta?.canonicalScene ? { x: 0, y: 0 } : layoutRecord.offset,
              lcaOffset: prepared.meta?.canonicalScene
                ? { x: 0, y: 0 }
                : (() => {
                    const sourceNode = laidOutNodes.get(edge.source);
                    const targetNode = laidOutNodes.get(edge.target);
                    return sourceNode && targetNode
                      ? lcaOffsetForNodes(sourceNode, targetNode, laidOutNodes)
                      : { x: 0, y: 0 };
                  })(),
            }
          : {
              sections: fallbackEdgeSections(sourceNode, targetNode, sourcePortCenter, targetPortCenter),
              edgeOwnerOffset: { x: 0, y: 0 },
              lcaOffset: { x: 0, y: 0 },
            },
        attributes: {
          ...(edge.attributes ?? {}),
          _sourcePortCenter: sourcePortCenter,
          _targetPortCenter: targetPortCenter,
        },
      } satisfies LaidOutEdge;
    });

    return { nodes, edges };
  } catch {
    // Interconnection notation must not degrade into a heuristic layout if ELK fails.
    return { nodes: [], edges: [] };
  }
}

export function buildInterconnectionElkGraph(prepared: PreparedView): Record<string, unknown> {
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
  return {
    id: "root",
    roots: roots.map((node) => node.id),
    edges: prepared.edges.map((edge) => ({
      id: edge.id,
      source: edge.source,
      target: edge.target,
      sourcePortId: edge.attributes?.sourcePortId,
      targetPortId: edge.attributes?.targetPortId,
    })),
    canonicalScene: Boolean(prepared.meta?.canonicalScene),
  };
}

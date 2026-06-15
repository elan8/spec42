import {
  asInterconnectionPrepared,
  type InterconnectionPreparedNode,
  type InterconnectionPreparedView,
  type PreparedNode,
  type PreparedView,
} from "../prepare";
import {
  computeIbdLeafHeight,
  ibdNodeHeight,
  ibdNodeWidth,
  splitIbdPortsBySide,
  type PreparedPort,
} from "./types";

export interface InterconnectionElkEdgeSpec {
  id: string;
  sources: string[];
  targets: string[];
  sourcePortId?: string;
  targetPortId?: string;
}

export interface InterconnectionPortDrawOrder {
  west: string[];
  east: string[];
}

export interface InterconnectionElkBuild {
  elkGraphInput: Record<string, unknown>;
  elkEdges: InterconnectionElkEdgeSpec[];
  nodesById: Map<string, InterconnectionPreparedNode>;
  preparedIdForElkId: Map<string, string>;
  portDrawOrderFor: (node: InterconnectionPreparedNode) => InterconnectionPortDrawOrder;
}

export function buildInterconnectionElkBuild(prepared: InterconnectionPreparedView): InterconnectionElkBuild {
  const nodesById = new Map(prepared.nodes.map((node) => [node.id, node]));
  const childrenByParent = new Map<string, InterconnectionPreparedNode[]>();
  const roots: InterconnectionPreparedNode[] = [];

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
        port.attributes?.scenePortId === endpointText,
    );
    return canonicalMatch?.name ?? null;
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

  const toElkNode = (node: PreparedNode): Record<string, unknown> => {
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
      const childWidthSum = children.reduce((sum: number, child) => sum + Number((child as { width?: number }).width ?? ibdNodeWidth), 0);
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
    .filter((edge): edge is NonNullable<typeof edge> => edge !== null);

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

  const portDrawOrderFor = (node: PreparedNode): InterconnectionPortDrawOrder => {
    const ports = portDetailsFor(node);
    const { west, east } = splitIbdPortsBySide(node, ports, sideForPort, usageForPort);
    return { west: west.map((port) => port.name), east: east.map((port) => port.name) };
  };

  return { elkGraphInput, elkEdges, nodesById, preparedIdForElkId, portDrawOrderFor };
}

/** Full ELK graph input for parity tests and pipeline export. */
export function buildInterconnectionElkGraphInput(prepared: PreparedView): Record<string, unknown> {
  return buildInterconnectionElkBuild(asInterconnectionPrepared(prepared)).elkGraphInput;
}

import { normalizeEdgeKind } from "./graph-normalization";

export interface PreparedNode {
  id: string;
  label: string;
  kind: string;
  sourcePath?: string | null;
  range?: { start?: { line?: number } } | null;
  attributes?: Record<string, unknown>;
}

export interface PreparedEdge {
  id: string;
  source: string;
  target: string;
  label: string;
  edgeKind?: string;
  attributes?: Record<string, unknown>;
}

export interface PreparedView {
  title: string;
  view: string;
  nodes: PreparedNode[];
  edges: PreparedEdge[];
}

type UnknownRecord = Record<string, unknown>;
type UnknownArray = UnknownRecord[];

interface VisualizationPayload extends UnknownRecord {
  view?: string;
  selectedViewName?: string;
  selectedView?: string;
  graph?: UnknownRecord;
  generalViewGraph?: UnknownRecord;
  ibd?: UnknownRecord;
  activityDiagrams?: UnknownArray;
  sequenceDiagrams?: UnknownArray;
}

function asRecord(value: unknown): UnknownRecord {
  return value && typeof value === "object" ? (value as UnknownRecord) : {};
}

function asArray(value: unknown): unknown[] {
  return Array.isArray(value) ? value : [];
}

function asString(value: unknown, fallback = ""): string {
  if (typeof value === "string") return value;
  if (typeof value === "number" || typeof value === "boolean") return String(value);
  return fallback;
}

function isPackage(node: UnknownRecord): boolean {
  return asString(node.type ?? node.element_type).toLowerCase() === "package";
}

export function nodeAccentClass(kind: string): string {
  const normalized = kind.toLowerCase();
  if (normalized.includes("verification")) return "viz-node--verification";
  if (normalized.includes("analysis")) return "viz-node--analysis";
  if (normalized.includes("requirement")) return "viz-node--requirement";
  if (normalized.includes("part_usage")) return "viz-node--part-usage";
  if (normalized.includes("part")) return "viz-node--part";
  if (normalized.includes("port")) return "viz-node--port";
  if (normalized.includes("interface")) return "viz-node--interface";
  if (normalized.includes("action")) return "viz-node--action";
  if (normalized.includes("connection")) return "viz-node--connection";
  if (normalized.includes("state")) return "viz-node--state";
  return "viz-node--default";
}

export function rendererLabel(view: string): string {
  switch (view) {
    case "interconnection-view":
      return "Interconnection";
    case "action-flow-view":
      return "Action Flow";
    case "state-transition-view":
      return "State Transition";
    case "sequence-view":
      return "Sequence";
    default:
      return "General";
  }
}

export function prepareViewData(visualizationInput: unknown): PreparedView {
  const visualization = asRecord(visualizationInput) as VisualizationPayload;
  const view = visualization?.view || "general-view";
  if (view === "interconnection-view") return prepareInterconnection(visualization);
  if (view === "action-flow-view") return prepareActivity(visualization);
  if (view === "state-transition-view") return prepareState(visualization);
  if (view === "sequence-view") return prepareSequence(visualization);
  return prepareGraph(visualization?.graph, visualization);
}

function prepareGraph(graphInput: unknown, visualization: VisualizationPayload): PreparedView {
  const graph = asRecord(graphInput);
  const rawNodes = asArray(graph.nodes).map(asRecord);
  const sourceNodes = rawNodes.filter((node) => !isPackage(node));
  const nodeIds = new Set(sourceNodes.map((node) => asString(node.id)));
  const nodes = sourceNodes.map((node) => ({
    id: asString(node.id),
    label: asString(node.name ?? node.qualifiedName ?? node.id, "Unnamed"),
    kind: asString(node.type ?? node.element_type, "Element"),
    sourcePath: asString(node.sourcePath ?? node.source_path) || null,
    range: (node.range as { start?: { line?: number } } | null | undefined) ?? null,
    attributes: asRecord(node.attributes)
  }));
  const edges = asArray(graph.edges)
    .map(asRecord)
    .filter((edge) => nodeIds.has(asString(edge.source)) && nodeIds.has(asString(edge.target)))
    .map((edge, index) => {
      const label = asString(edge.name ?? edge.type ?? edge.rel_type, "");
      return {
        id: asString(edge.id, `edge-${index}`),
        source: asString(edge.source),
        target: asString(edge.target),
        label,
        edgeKind: normalizeEdgeKind(label),
      };
    });
  return {
    title: visualization?.selectedViewName || "SysML View",
    view: visualization?.view || "general-view",
    nodes,
    edges
  };
}

function prepareInterconnection(visualization: VisualizationPayload): PreparedView {
  const ibd = asRecord(visualization.ibd);
  const selectedName = asString(visualization.selectedViewName ?? ibd.defaultRoot);
  const rootViews = asRecord(ibd.rootViews);
  const rootKeys = Object.keys(rootViews);
  const scopedName =
    selectedName && rootViews[selectedName]
      ? selectedName
      : rootKeys.length > 0
        ? rootKeys[0]
        : selectedName;
  const scoped = scopedName && rootViews[scopedName] ? asRecord(rootViews[scopedName]) : ibd;
  const parts = asArray(scoped.parts ?? ibd.parts).map(asRecord);
  const ports = asArray(scoped.ports ?? ibd.ports).map(asRecord);
  const connectors = asArray(scoped.connectors ?? ibd.connectors).map(asRecord);
  const nodes = parts.map((part) => {
    const partId = asString(part.id ?? part.name);
    const parent = asString(part.containerId ?? part.parentId, "");
    return {
      id: partId,
      label: asString(part.name ?? part.id, "Unnamed"),
      kind: asString(part.type, "part"),
      sourcePath: asString(part.sourcePath) || null,
      range: (part.range as { start?: { line?: number } } | null | undefined) ?? null,
      attributes: {
        ...asRecord(part.attributes),
        containerId: parent || null,
        qualifiedName: asString(part.qualifiedName),
        ports: portsForPart(ports, part),
      },
    };
  });
  const nodeIds = new Set(nodes.map((node) => node.id));
  const edges = connectors
    .map((connector, index) => {
      const source = firstPresent(connector.sourcePartId, connector.source, connector.sourceId, connector.sourcePortPartId);
      const target = firstPresent(connector.targetPartId, connector.target, connector.targetId, connector.targetPortPartId);
      return {
        id: asString(connector.id, `connector-${index}`),
        source: source ? asString(source) : "",
        target: target ? asString(target) : "",
        label: asString(connector.name ?? connector.type, "connect"),
        attributes: {
          sourceId: asString(connector.sourceId ?? connector.source),
          targetId: asString(connector.targetId ?? connector.target),
        },
      };
    })
    .filter((edge) => nodeIds.has(edge.source) && nodeIds.has(edge.target));
  return { title: scopedName || selectedName || "Interconnection View", view: "interconnection-view", nodes, edges };
}

function prepareActivity(visualization: VisualizationPayload): PreparedView {
  const selected = selectNamedDiagram(visualization?.activityDiagrams, visualization?.selectedViewName);
  const selectedRecord = asRecord(selected);
  const nodes = asArray(selectedRecord.nodes ?? selectedRecord.actions ?? selectedRecord.steps).map((nodeRaw, index) => {
    const node = asRecord(nodeRaw);
    return {
      id: asString(node.id ?? node.name, `action-${index}`),
      label: asString(node.name ?? node.label ?? node.id, `Action ${index + 1}`),
      kind: asString(node.type, "action"),
      sourcePath: asString(node.sourcePath) || null,
      range: (node.range as { start?: { line?: number } } | null | undefined) ?? null,
      attributes: asRecord(node.attributes)
    };
  });
  const nodeIds = new Set(nodes.map((node) => node.id));
  const edges = asArray(selectedRecord.edges ?? selectedRecord.flows ?? selectedRecord.transitions)
    .map((edgeRaw, index) => {
      const edge = asRecord(edgeRaw);
      return {
        id: asString(edge.id, `flow-${index}`),
        source: asString(edge.source ?? edge.from ?? edge.sourceId, ""),
        target: asString(edge.target ?? edge.to ?? edge.targetId, ""),
        label: asString(edge.name ?? edge.label ?? edge.type, "")
      };
    })
    .filter((edge) => nodeIds.has(edge.source) && nodeIds.has(edge.target));
  return { title: asString(selectedRecord.name ?? visualization?.selectedViewName, "Action Flow View"), view: "action-flow-view", nodes, edges };
}

function prepareState(visualization: VisualizationPayload): PreparedView {
  const graph = asRecord(visualization?.graph);
  const stateNodes = asArray(graph.nodes)
    .map(asRecord)
    .filter((node) => asString(node.type ?? node.element_type).toLowerCase().includes("state"));
  const ids = new Set(stateNodes.map((node) => asString(node.id)));
  const nodes = stateNodes.map((node) => ({
    id: asString(node.id),
    label: asString(node.name ?? node.id, "State"),
    kind: asString(node.type ?? node.element_type, "state"),
    sourcePath: asString(node.sourcePath) || null,
    range: (node.range as { start?: { line?: number } } | null | undefined) ?? null,
    attributes: asRecord(node.attributes)
  }));
  const edges = asArray(graph.edges)
    .map((edgeRaw, index) => {
      const edge = asRecord(edgeRaw);
      return {
        id: `transition-${index}`,
        source: asString(edge.source),
        target: asString(edge.target),
        label: asString(edge.name ?? edge.type ?? edge.rel_type, "")
      };
    })
    .filter((edge) => ids.has(edge.source) && ids.has(edge.target));
  return { title: asString(visualization?.selectedViewName, "State Transition View"), view: "state-transition-view", nodes, edges };
}

function prepareSequence(visualization: VisualizationPayload): PreparedView {
  const selected = selectNamedDiagram(visualization?.sequenceDiagrams, visualization?.selectedViewName);
  if (selected) return diagramToPrepared(selected, "sequence-view", "Sequence View");
  return prepareGraph(visualization?.graph, visualization);
}

function diagramToPrepared(diagramInput: unknown, view: string, fallbackTitle: string): PreparedView {
  const diagram = asRecord(diagramInput);
  const nodes = asArray(diagram.nodes ?? diagram.states).map((nodeRaw, index) => {
    const node = asRecord(nodeRaw);
    return {
      id: asString(node.id ?? node.name, `node-${index}`),
      label: asString(node.name ?? node.label ?? node.id, `Node ${index + 1}`),
      kind: asString(node.type, view),
      sourcePath: asString(node.sourcePath) || null,
      range: (node.range as { start?: { line?: number } } | null | undefined) ?? null,
      attributes: asRecord(node.attributes)
    };
  });
  const ids = new Set(nodes.map((node) => node.id));
  const edges = asArray(diagram.edges ?? diagram.transitions)
    .map((edgeRaw, index) => {
      const edge = asRecord(edgeRaw);
      return {
        id: asString(edge.id, `edge-${index}`),
        source: asString(edge.source ?? edge.from ?? edge.sourceId, ""),
        target: asString(edge.target ?? edge.to ?? edge.targetId, ""),
        label: asString(edge.name ?? edge.label ?? edge.type, "")
      };
    })
    .filter((edge) => ids.has(edge.source) && ids.has(edge.target));
  return { title: asString(diagram.name, fallbackTitle), view, nodes, edges };
}

function selectNamedDiagram(diagramsInput: unknown, selectedName: string | undefined): UnknownRecord | null {
  const diagrams = asArray(diagramsInput).map(asRecord);
  if (diagrams.length === 0) return null;
  if (!selectedName) return diagrams[0];
  return (
    diagrams.find(
      (diagram) =>
        asString(diagram.id) === selectedName || asString(diagram.name) === selectedName,
    ) ?? null
  );
}

function firstPresent(...values: unknown[]): unknown {
  return values.find((value) => value != null && asString(value).trim() !== "");
}

function portsForPart(ports: UnknownRecord[], part: UnknownRecord): string[] {
  const id = asString(part.id ?? part.name);
  return ports
    .filter((port) => asString(port.partId ?? port.ownerId ?? port.containerId, "") === id)
    .map((port) => asString(port.name ?? port.id))
    .filter(Boolean)
    .slice(0, 8);
}

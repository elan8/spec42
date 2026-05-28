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
  meta?: Record<string, unknown>;
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
  synthesizeInitialState?: boolean;
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
  return prepareGraph(visualization?.generalViewGraph ?? visualization?.graph, visualization);
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
        attributes: {
          relationType: normalizeEdgeKind(label),
        },
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
  const containerGroups = asArray(scoped.containerGroups ?? ibd.containerGroups).map(asRecord);
  const packageContainerGroups = asArray(scoped.packageContainerGroups ?? ibd.packageContainerGroups).map(asRecord);
  const baseNodes = parts.map((part) => {
    const partId = asString(part.id ?? part.name);
    const parent = asString(part.containerId ?? part.parentId, "");
    const portDetails = portsForPart(ports, part);
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
        partType: firstPresent(
          asRecord(part.attributes).partType,
          asRecord(part.attributes).type,
          asRecord(part.attributes).typedBy,
          part.partType,
        ),
        children: asArray(part.children),
        ports: portDetails.map((port) => port.name),
        portDetails,
      },
    };
  });
  const nodes = synthesizeInterconnectionContainers(baseNodes, containerGroups, packageContainerGroups);
  const nodeIds = new Set(nodes.map((node) => node.id));
  const nodeById = new Map(nodes.map((node) => [node.id, node]));
  const concreteNodes = nodes.filter((node) => !asRecord(node.attributes).isSyntheticContainer);
  const resolveEndpointPartId = (explicit: unknown, endpoint: unknown): string => {
    const explicitText = asString(explicit);
    if (explicitText && nodeIds.has(explicitText)) return explicitText;
    const endpointText = asString(endpoint).replace(/::/g, ".").trim();
    if (!endpointText) return explicitText;
    const direct = concreteNodes.find((node) => {
      const attrs = asRecord(node.attributes);
      return [node.id, node.label, asString(attrs.qualifiedName).replace(/::/g, ".")]
        .filter(Boolean)
        .includes(endpointText);
    });
    if (direct) return direct.id;
    const best = concreteNodes
      .map((node) => {
        const qn = asString(asRecord(node.attributes).qualifiedName, node.label).replace(/::/g, ".").trim();
        const aliases = [qn, node.label, node.id].filter(Boolean);
        const matched = aliases
          .filter((alias) => endpointText === alias || endpointText.startsWith(`${alias}.`))
          .sort((a, b) => b.length - a.length)[0];
        return matched ? { node, score: matched.length } : null;
      })
      .filter((value): value is { node: (typeof nodes)[number]; score: number } => Boolean(value))
      .sort((a, b) => b.score - a.score)[0];
    return best?.node.id ?? explicitText;
  };
  const edges = connectors
    .map((connector, index) => {
      const sourceEndpoint = firstPresent(connector.sourceId, connector.source);
      const targetEndpoint = firstPresent(connector.targetId, connector.target);
      const source = resolveEndpointPartId(firstPresent(connector.sourcePartId, connector.sourcePortPartId), sourceEndpoint);
      const target = resolveEndpointPartId(firstPresent(connector.targetPartId, connector.targetPortPartId), targetEndpoint);
      const type = ibdConnectorKind(connector);
      const label = ibdConnectorLabel(connector, type);
      return {
        id: asString(connector.id, `connector-${index}`),
        source,
        target,
        label,
        edgeKind: normalizeEdgeKind(type),
        attributes: {
          ...asRecord(connector.attributes),
          sourceId: asString(sourceEndpoint),
          targetId: asString(targetEndpoint),
          itemType: asString(connector.itemType),
          interfaceName: asString(connector.interfaceName ?? connector.interfaceType ?? connector.interfaceDefinition),
          relationType: type,
        },
      };
    })
    .filter((edge) => nodeById.has(edge.source) && nodeById.has(edge.target));
  const rootCandidates = asArray(ibd.rootCandidates).map((value) => asString(value)).filter(Boolean);
  return {
    title: scopedName || selectedName || "Interconnection View",
    view: "interconnection-view",
    nodes,
    edges,
    meta: {
      selectedRoot: scopedName || null,
      rootCandidates,
      containerGroups,
      packageContainerGroups,
    },
  };
}

function prepareActivity(visualization: VisualizationPayload): PreparedView {
  const selected = visualization?.selectedViewName
    ? selectNamedDiagram(visualization?.activityDiagrams, visualization?.selectedViewName)
    : null;
  const fallbackDiagram = bestBehaviorDiagram(asArray(visualization?.activityDiagrams).map(asRecord));
  const effective = selected ?? fallbackDiagram;
  const selectedRecord = asRecord(effective);
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
  return {
    title: asString(selectedRecord.name ?? visualization?.selectedViewName, "Action Flow View"),
    view: "action-flow-view",
    nodes,
    edges,
    meta: {
      selectedDiagramId: asString(selectedRecord.id),
      nodeCount: nodes.length,
      edgeCount: edges.length,
    },
  };
}

function prepareState(visualization: VisualizationPayload): PreparedView {
  const selectedStateDiagram = selectNamedDiagram((visualization as UnknownRecord).stateDiagrams, visualization?.selectedViewName);
  if (selectedStateDiagram) {
    const diagram = asRecord(selectedStateDiagram);
    const prepared = diagramToPrepared(diagram, "state-transition-view", "State Transition View");
    return {
      ...prepared,
      meta: {
        selectedDiagramId: asString(diagram.id),
        selectedDiagramName: asString(diagram.name),
      },
    };
  }
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
  const synthesizeInitial = visualization?.synthesizeInitialState === true;
  const hasInitial = nodes.some((node) => node.kind.toLowerCase().includes("initial") || node.label.toLowerCase() === "initial");
  const withSyntheticInitial = synthesizeInitial && !hasInitial && nodes.length > 0
    ? [{ id: "__synthetic_initial__", label: "Initial", kind: "initial", attributes: { synthetic: true } }, ...nodes]
    : nodes;
  const idsWithInitial = new Set(withSyntheticInitial.map((node) => node.id));
  const edgesWithInitial =
    !hasInitial && withSyntheticInitial.length > 1
      ? [
          {
            id: "transition-synthetic-initial",
            source: "__synthetic_initial__",
            target: withSyntheticInitial[1].id,
            label: "initial",
          },
          ...edges,
        ]
      : edges;
  return {
    title: asString(visualization?.selectedViewName, "State Transition View"),
    view: "state-transition-view",
    nodes: withSyntheticInitial.filter((node) => idsWithInitial.has(node.id)),
    edges: edgesWithInitial.filter((edge) => idsWithInitial.has(edge.source) && idsWithInitial.has(edge.target)),
    meta: {
      syntheticInitial: synthesizeInitial && !hasInitial && nodes.length > 0,
    },
  };
}

function prepareSequence(visualization: VisualizationPayload): PreparedView {
  const selected = selectNamedDiagram(visualization?.sequenceDiagrams, visualization?.selectedViewName);
  if (selected) {
    const prepared = diagramToPrepared(selected, "sequence-view", "Sequence View");
    return {
      ...prepared,
      meta: {
        selectedDiagramName: asString(asRecord(selected).name),
      },
    };
  }
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

function bestBehaviorDiagram(diagrams: UnknownRecord[]): UnknownRecord | null {
  if (diagrams.length === 0) return null;
  const score = (diagram: UnknownRecord): number => {
    const nodes = asArray(diagram.nodes ?? diagram.actions ?? diagram.steps);
    const edges = asArray(diagram.edges ?? diagram.flows ?? diagram.transitions);
    return (nodes.length * 10) + edges.length;
  };
  return diagrams
    .slice()
    .sort((a, b) => score(b) - score(a))[0] ?? null;
}

function firstPresent(...values: unknown[]): unknown {
  return values.find((value) => value != null && asString(value).trim() !== "");
}

function ibdConnectorKind(connector: UnknownRecord): string {
  const type = asString(connector.type ?? connector.relationType ?? connector.rel_type).trim();
  const name = asString(connector.name ?? connector.label).trim();
  const itemType = asString(connector.itemType).trim();
  const interfaceName = asString(connector.interfaceName ?? connector.interfaceType ?? connector.interfaceDefinition).trim();
  const source = `${type} ${name}`.toLowerCase();
  if (source.includes("binding") || source.includes("bind")) return "binding";
  if (source.includes("reference") || source.includes("ref")) return "reference";
  if (source.includes("interface") || interfaceName) return "interface";
  if (source.includes("flow") || itemType) return "flow";
  return type || "connection";
}

function ibdConnectorLabel(connector: UnknownRecord, type: string): string {
  const name = asString(connector.name ?? connector.label).trim();
  const interfaceName = asString(connector.interfaceName ?? connector.interfaceType ?? connector.interfaceDefinition).trim();
  const itemType = asString(connector.itemType).trim();
  const normalized = type.toLowerCase();
  if (normalized.includes("flow") && itemType) return itemType;
  if (normalized.includes("interface") && interfaceName) return interfaceName;
  return name || type || "connection";
}

function synthesizeInterconnectionContainers(
  baseNodes: Array<PreparedNode>,
  containerGroups: UnknownRecord[],
  packageContainerGroups: UnknownRecord[],
): PreparedNode[] {
  const byId = new Map(baseNodes.map((node) => [node.id, node]));
  const nodes = [...baseNodes];
  const resolveMember = (memberId: string): PreparedNode | undefined => {
    const normalized = memberId.replace(/::/g, ".");
    return (
      byId.get(memberId) ??
      baseNodes.find((node) => {
        const attrs = asRecord(node.attributes);
        const qualifiedName = asString(attrs.qualifiedName).replace(/::/g, ".");
        return node.id === memberId || node.label === memberId || qualifiedName === normalized;
      })
    );
  };
  const addGroup = (group: UnknownRecord, packageGroup: boolean) => {
    const id = asString(group.id || group.qualifiedPackage || group.qualifiedName || group.label || group.name);
    if (!id || byId.has(id)) return;
    const label = asString(group.label || group.name || group.qualifiedPackage || group.qualifiedName || id, "package");
    const memberIds = groupMemberIds(group);
    const parentId = asString(group.parentId);
    const node: PreparedNode = {
      id,
      label,
      kind: "package",
      attributes: {
        isSyntheticContainer: true,
        isPackageContainer: packageGroup,
        containerId: parentId || null,
        memberIds,
        qualifiedName: asString(group.qualifiedPackage || group.qualifiedName || id),
      },
    };
    byId.set(id, node);
    nodes.push(node);
    for (const memberId of memberIds) {
      const member = resolveMember(memberId);
      if (!member) continue;
      const attrs = member.attributes ?? {};
      member.attributes = {
        ...attrs,
        containerId: attrs.containerId || id,
        _fallbackContainerId: id,
      };
    }
  };
  packageContainerGroups.forEach((group) => addGroup(group, true));
  containerGroups.forEach((group) => addGroup(group, false));
  resolveInterconnectionContainerIds(nodes);
  return pruneEmptySyntheticContainers(nodes);
}

function resolveInterconnectionContainerIds(nodes: PreparedNode[]): void {
  const aliases = new Map<string, string>();
  const addAlias = (alias: unknown, id: string) => {
    const text = asString(alias).trim();
    if (!text) return;
    aliases.set(text, id);
    aliases.set(text.replace(/::/g, "."), id);
    aliases.set(text.replace(/\./g, "::"), id);
  };
  for (const node of nodes) {
    const attrs = asRecord(node.attributes);
    addAlias(node.id, node.id);
    addAlias(node.label, node.id);
    addAlias(attrs.qualifiedName, node.id);
  }

  for (const node of nodes) {
    const attrs = asRecord(node.attributes);
    const rawParent = asString(attrs.containerId);
    const fallbackParent = asString(attrs._fallbackContainerId);
    const resolved = aliases.get(rawParent) ?? aliases.get(rawParent.replace(/::/g, ".")) ?? aliases.get(fallbackParent);
    if (resolved && resolved !== node.id) {
      node.attributes = { ...attrs, containerId: resolved };
    } else if (rawParent) {
      node.attributes = { ...attrs, containerId: null };
    }
  }
}

function groupMemberIds(group: UnknownRecord): string[] {
  return asArray(group.memberPartIds ?? group.memberIds ?? group.nodeIds)
    .map((value) => asString(value))
    .filter(Boolean);
}

function pruneEmptySyntheticContainers(nodes: PreparedNode[]): PreparedNode[] {
  let current = nodes;
  let changed = true;
  while (changed) {
    changed = false;
    const childCount = new Map<string, number>();
    for (const node of current) {
      const parentId = asString(asRecord(node.attributes).containerId);
      if (parentId) childCount.set(parentId, (childCount.get(parentId) ?? 0) + 1);
    }
    const next = current.filter((node) => {
      const attrs = asRecord(node.attributes);
      const emptySynthetic = Boolean(attrs.isSyntheticContainer) && !childCount.has(node.id);
      if (emptySynthetic) changed = true;
      return !emptySynthetic;
    });
    if (changed) {
      const ids = new Set(next.map((node) => node.id));
      for (const node of next) {
        const attrs = asRecord(node.attributes);
        const parentId = asString(attrs.containerId);
        if (parentId && !ids.has(parentId)) {
          node.attributes = { ...attrs, containerId: null };
        }
      }
    }
    current = next;
  }
  return current;
}

function portsForPart(ports: UnknownRecord[], part: UnknownRecord): Array<Record<string, unknown> & { name: string }> {
  const id = asString(part.id ?? part.name);
  const name = asString(part.name);
  const qualifiedName = asString(part.qualifiedName).replace(/::/g, ".");
  return ports
    .filter((port) => {
      const parent = asString(port.partId ?? port.ownerId ?? port.containerId ?? port.parentId, "").replace(/::/g, ".");
      return parent === id || parent === name || parent === qualifiedName;
    })
    .map((port) => ({
      ...port,
      name: asString(port.name ?? port.id),
      id: asString(port.id ?? port.name),
      parentId: asString(port.parentId ?? port.partId ?? port.ownerId ?? port.containerId),
    }))
    .filter((port) => Boolean(port.name))
    .slice(0, 8);
}

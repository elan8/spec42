import {
  isOverviewVisualElementType,
  isPackageElementType,
  normalizeEdgeKind,
} from "./graph-normalization";
import {
  isDefinitionKind,
  isReferenceKind,
  resolveNodeChrome,
} from "./node-notation";

export interface PreparedNode {
  id: string;
  label: string;
  kind: string;
  sourcePath?: string | null;
  uri?: string | null;
  range?: { start?: { line?: number; character?: number }; end?: { line?: number; character?: number } } | null;
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
  /** Normalized activity diagrams from VS Code prepareDataForView (nodes + resolved flows). */
  diagrams?: UnknownArray;
  activityDiagrams?: UnknownArray;
  sequenceDiagrams?: UnknownArray;
  stateMachines?: UnknownArray;
  stateDiagrams?: UnknownArray;
  synthesizeInitialState?: boolean;
  activityLayoutDirection?: string;
  stateLayoutDirection?: string;
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
  return isPackageElementType(elementTypeOf(node));
}

function elementTypeOf(node: UnknownRecord): string {
  const attrs = asRecord(node.attributes);
  return asString(
    node.type ??
      node.element_type ??
      node.element_kind ??
      attrs.element_type ??
      attrs.element_kind ??
      attrs.elementKind,
  );
}

/** @deprecated Use nodeStructureClass */
export function nodeAccentClass(kind: string): string {
  return nodeStructureClass(kind, kind.toLowerCase().includes("def"));
}

/** Structure-only CSS classes (definition / usage / reference / container); no per-kind color. */
export function nodeStructureClass(
  kind: string,
  isDefinition?: boolean,
  isReference?: boolean,
): string {
  return resolveNodeChrome(kind, { isDefinition, isReference }).structureClass;
}

export { isDefinitionKind, isReferenceKind, resolveNodeChrome } from "./node-notation";

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

function isGeneralViewDiagramNode(node: UnknownRecord): boolean {
  if (isSyntheticPackage(node)) {
    return false;
  }
  const elementType = elementTypeOf(node);
  // General / structure view: packages are namespace containers, not diagram nodes.
  return isOverviewVisualElementType(elementType);
}

function prepareGraph(graphInput: unknown, visualization: VisualizationPayload): PreparedView {
  const graph = asRecord(graphInput);
  const rawNodes = asArray(graph.nodes).map(asRecord);
  const sourceNodes = rawNodes.filter((node) => isGeneralViewDiagramNode(node));
  const nodeIds = new Set(sourceNodes.map((node) => asString(node.id)));
  const nodes = sourceNodes.map((node) => ({
    id: asString(node.id),
    label: asString(node.name ?? node.qualifiedName ?? node.id, "Unnamed"),
    kind: elementTypeOf(node) || "Element",
    sourcePath: asString(node.sourcePath ?? node.source_path) || null,
    uri: nodeUri(node),
    range: (node.range as { start?: { line?: number } } | null | undefined) ?? null,
    attributes: {
      ...asRecord(node.attributes),
      qualifiedName: asString(node.qualifiedName ?? asRecord(node.attributes).qualifiedName),
      isPackage: isPackage(node),
      isDefinition: isDefinitionKind(asString(node.type ?? node.element_type, "")),
      isReference: isReferenceKind(asString(node.type ?? node.element_type, "")),
    },
  }));
  const edges = asArray(graph.edges)
    .map(asRecord)
    .filter((edge) => nodeIds.has(asString(edge.source)) && nodeIds.has(asString(edge.target)))
    .map((edge, index) => {
      const relationType = asString(edge.type ?? edge.rel_type ?? edge.relationType ?? edge.name, "");
      const label = asString(edge.name ?? edge.label ?? edge.type ?? edge.rel_type, "");
      return {
        id: asString(edge.id, `edge-${index}`),
        source: asString(edge.source),
        target: asString(edge.target),
        label,
        edgeKind: normalizeEdgeKind(relationType),
        attributes: {
          ...asRecord(edge.attributes),
          relationType: normalizeEdgeKind(relationType),
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
    const partKind = asString(part.type, "part");
    return {
      id: partId,
      label: asString(part.name ?? part.id, "Unnamed"),
      kind: partKind,
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
        isDefinition: isDefinitionKind(partKind),
        isReference: isReferenceKind(partKind),
      },
    };
  });
  const scopedRootPart = scopedName ? findScopedRootPart(baseNodes, scopedName) : undefined;
  const scopedContainerGroups = scopedRootPart
    ? filterContainerGroupsForScopedRoot(containerGroups, baseNodes, scopedName)
    : containerGroups;
  const scopedPackageGroups = scopedRootPart ? [] : packageContainerGroups;
  let nodes = synthesizeInterconnectionContainers(baseNodes, scopedContainerGroups, scopedPackageGroups);
  nodes = scopedRootPart ? collapseRedundantOuterBoundaries(nodes, scopedName) : nodes;
  const nodeIds = new Set(nodes.map((node) => node.id));
  const nodeById = new Map(nodes.map((node) => [node.id, node]));
  const concreteNodes = nodes.filter((node) => !asRecord(node.attributes).isSyntheticContainer);
  const resolveEndpointPartId = (explicit: unknown, endpoint: unknown): string => {
    const explicitText = asString(explicit).replace(/::/g, ".").trim();
    if (explicitText) {
      const directById = concreteNodes.find((node) => {
        const attrs = asRecord(node.attributes);
        const aliases = [node.id, node.label, asString(attrs.qualifiedName)]
          .filter(Boolean)
          .map((alias) => alias.replace(/::/g, "."));
        return aliases.includes(explicitText);
      });
      if (directById) return directById.id;
      if (nodeIds.has(explicitText)) return explicitText;
    }
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

function nodeUri(node: UnknownRecord): string | null {
  return asString(node.uri ?? node.sourcePath ?? node.source_path) || null;
}

function nodeRange(node: UnknownRecord): PreparedNode["range"] {
  return (node.range as PreparedNode["range"]) ?? null;
}

function buildBehaviorNode(
  node: UnknownRecord,
  index: number,
  defaults: { id: string; label: string; kind: string },
): PreparedNode {
  const attrs = asRecord(node.attributes);
  const qualifiedName = asString(node.qualifiedName ?? attrs.qualifiedName ?? node.id);
  return {
    id: asString(node.id ?? node.name, defaults.id),
    label: asString(node.name ?? node.label ?? node.id, defaults.label),
    kind: defaults.kind,
    sourcePath: nodeUri(node),
    uri: nodeUri(node),
    range: nodeRange(node),
    attributes: {
      ...attrs,
      ...(qualifiedName ? { qualifiedName } : {}),
      ...(node.parentId != null ? { parentId: node.parentId } : {}),
      ...(node.parent != null ? { parent: node.parent } : {}),
    },
  };
}

function isSyntheticPackage(node: UnknownRecord): boolean {
  if (!isPackage(node)) return false;
  const attrs = asRecord(node.attributes);
  return Boolean(node.synthetic ?? node.isSynthetic ?? attrs.synthetic ?? attrs.isSyntheticContainer);
}

function activityDiagramCatalog(visualization: VisualizationPayload): UnknownRecord[] {
  const normalized = asArray(visualization.diagrams).map(asRecord);
  if (normalized.length > 0) {
    return normalized;
  }
  return asArray(visualization.activityDiagrams).map(asRecord);
}

function collectActivityNodes(diagram: UnknownRecord): PreparedNode[] {
  const allowedKinds = new Set(["action", "perform", "decision", "merge", "fork", "join", "initial", "final"]);
  const decisions = asArray(diagram.decisions).map((nodeRaw, index) => {
    const node = asRecord(nodeRaw);
    return buildBehaviorNode(node, index, {
      id: `decision-${index}`,
      label: "Decision",
      kind: "decision",
    });
  });
  const states = asArray(diagram.states)
    .map((nodeRaw, index) => {
      const node = asRecord(nodeRaw);
      const kind = asString(node.type ?? node.stateType ?? node.kind, "state").toLowerCase();
      return buildBehaviorNode(node, index, {
        id: `state-${index}`,
        label: `State ${index + 1}`,
        kind,
      });
    })
    .filter((node) =>
      ["initial", "final", "decision", "merge", "fork", "join"].some((token) => node.kind.includes(token)),
    );
  const actions = asArray(diagram.nodes ?? diagram.actions ?? diagram.steps).map((nodeRaw, index) => {
    const node = asRecord(nodeRaw);
    const kind = asString(node.kind ?? node.type ?? node.action_type, "action").toLowerCase();
    const normalizedKind = kind.includes("perform")
      ? "perform"
      : kind.includes("decision")
        ? "decision"
        : kind.includes("merge")
          ? "merge"
          : kind.includes("fork")
            ? "fork"
            : kind.includes("join")
              ? "join"
              : kind.includes("initial")
                ? "initial"
                : kind.includes("final")
                  ? "final"
                  : "action";
    return buildBehaviorNode(node, index, {
      id: `action-${index}`,
      label: `Action ${index + 1}`,
      kind: normalizedKind,
    });
  });
  return [...actions, ...decisions, ...states].filter((node) => allowedKinds.has(node.kind));
}

function buildActivityNodeAliasMap(nodes: PreparedNode[]): Map<string, string> {
  const aliases = new Map<string, string>();
  const register = (alias: unknown, nodeId: string) => {
    const key = asString(alias).trim();
    if (!key) {
      return;
    }
    if (!aliases.has(key)) {
      aliases.set(key, nodeId);
    }
    const normalized = key.replace(/::/g, ".");
    if (!aliases.has(normalized)) {
      aliases.set(normalized, nodeId);
    }
    const lastSegment = normalized.split(".").filter(Boolean).pop();
    if (lastSegment && !aliases.has(lastSegment)) {
      aliases.set(lastSegment, nodeId);
    }
  };
  for (const node of nodes) {
    const nodeId = node.id;
    register(node.id, nodeId);
    register(node.label, nodeId);
    register(asRecord(node.attributes).qualifiedName, nodeId);
  }
  return aliases;
}

function resolveActivityNodeRef(value: unknown, aliases: Map<string, string>): string {
  const key = asString(value).trim();
  if (!key) {
    return "";
  }
  const normalized = key.replace(/::/g, ".");
  const segments = normalized.split(".").filter(Boolean);
  const last = segments[segments.length - 1] || "";
  const first = segments[0] || "";
  return (
    aliases.get(key) ??
    aliases.get(normalized) ??
    (last ? aliases.get(last) : undefined) ??
    (first ? aliases.get(first) : undefined) ??
    key
  );
}

function prepareActivity(visualization: VisualizationPayload): PreparedView {
  const catalog = activityDiagramCatalog(visualization);
  const selected = selectNamedDiagram(catalog, visualization?.selectedViewName, visualization?.selectedView);
  const effective = selected ?? bestBehaviorDiagram(catalog);
  const diagram = asRecord(effective);
  const nodes = collectActivityNodes(diagram);
  const nodeIds = new Set(nodes.map((node) => node.id));
  const aliases = buildActivityNodeAliasMap(nodes);
  const edges = asArray(diagram.flows ?? diagram.edges ?? diagram.transitions)
    .map((edgeRaw, index) => {
      const edge = asRecord(edgeRaw);
      const source = resolveActivityNodeRef(edge.from ?? edge.source ?? edge.sourceId, aliases);
      const target = resolveActivityNodeRef(edge.to ?? edge.target ?? edge.targetId, aliases);
      return {
        id: asString(edge.id, `flow-${index}`),
        source,
        target,
        label: asString(edge.name ?? edge.label ?? edge.guard ?? edge.type, ""),
      };
    })
    .filter(
      (edge) =>
        edge.source &&
        edge.target &&
        edge.source !== edge.target &&
        nodeIds.has(edge.source) &&
        nodeIds.has(edge.target),
    );
  return {
    title: asString(diagram.name ?? visualization?.selectedViewName, "Action Flow View"),
    view: "action-flow-view",
    nodes,
    edges,
    meta: {
      selectedDiagramId: asString(diagram.id),
      nodeCount: nodes.length,
      edgeCount: edges.length,
      layoutDirection: asString(visualization?.activityLayoutDirection, "vertical"),
      activityDiagram: effective,
      parentContext: asString(diagram.name),
    },
  };
}

function stateMachineCatalog(visualization: VisualizationPayload): UnknownRecord[] {
  const normalized = asArray(visualization.stateMachines).map(asRecord);
  if (normalized.length > 0) {
    return normalized;
  }
  return asArray(visualization.stateDiagrams).map(asRecord);
}

function collectStateMachineNodes(machine: UnknownRecord): PreparedNode[] {
  return asArray(machine.states).map((stateRaw, index) => {
    const state = asRecord(stateRaw);
    const element = asRecord(state.element);
    const merged = {
      ...element,
      ...state,
      id: state.id ?? element.id,
      name: state.name ?? element.name,
      range: element.range ?? state.range,
      uri: element.uri ?? state.uri ?? element.sourcePath ?? state.sourcePath,
      qualifiedName: state.qualifiedName ?? element.qualifiedName ?? state.id,
    };
    const kind = asString(state.kind ?? state.type ?? element.type, "state").toLowerCase();
    return buildBehaviorNode(asRecord(merged), index, {
      id: `state-${index}`,
      label: "State",
      kind: kind.includes("initial")
        ? "initial"
        : kind.includes("final")
          ? "final"
          : kind.includes("composite")
            ? "composite"
            : "state",
    });
  });
}

function prepareStateMachine(machine: UnknownRecord, visualization: VisualizationPayload): PreparedView {
  const nodes = collectStateMachineNodes(machine);
  const nodeIds = new Set(nodes.map((node) => node.id));
  const aliases = buildActivityNodeAliasMap(nodes);
  const edges = asArray(machine.transitions)
    .map((edgeRaw, index) => {
      const edge = asRecord(edgeRaw);
      const source = resolveActivityNodeRef(edge.source ?? edge.sourceName ?? edge.from, aliases);
      const target = resolveActivityNodeRef(edge.target ?? edge.targetName ?? edge.to, aliases);
      const label = asString(edge.label ?? edge.name ?? edge.guard, "");
      return {
        id: asString(edge.id, `transition-${index}`),
        source,
        target,
        label: label === "entry" ? "" : label,
        attributes: {
          selfLoop: Boolean(edge.selfLoop ?? source === target),
        },
      };
    })
    .filter(
      (edge) =>
        edge.source &&
        edge.target &&
        nodeIds.has(edge.source) &&
        nodeIds.has(edge.target),
    );
  return {
    title: asString(machine.name ?? visualization?.selectedViewName, "State Transition View"),
    view: "state-transition-view",
    nodes,
    edges,
    meta: {
      selectedDiagramId: asString(machine.id),
      selectedDiagramName: asString(machine.name),
      layoutDirection: asString(visualization?.stateLayoutDirection, "horizontal"),
      stateMachine: machine,
      parentContext: asString(machine.name),
    },
  };
}

function prepareState(visualization: VisualizationPayload): PreparedView {
  const catalog = stateMachineCatalog(visualization);
  if (catalog.length > 0) {
    const selected = selectNamedDiagram(catalog, visualization?.selectedViewName, visualization?.selectedView);
    const effective = selected ?? catalog[0];
    if (effective) {
      return prepareStateMachine(asRecord(effective), visualization);
    }
  }
  const selectedStateDiagram = selectNamedDiagram(
    visualization.stateDiagrams,
    visualization?.selectedViewName,
    visualization?.selectedView,
  );
  if (selectedStateDiagram) {
    const diagram = asRecord(selectedStateDiagram);
    const prepared = diagramToPrepared(diagram, "state-transition-view", "State Transition View");
    return {
      ...prepared,
      meta: {
        selectedDiagramId: asString(diagram.id),
        selectedDiagramName: asString(diagram.name),
        layoutDirection: asString(visualization?.stateLayoutDirection, "horizontal"),
        stateDiagram: diagram,
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
  const selected = selectNamedDiagram(
    visualization?.sequenceDiagrams,
    visualization?.selectedViewName,
    visualization?.selectedView,
  );
  const fallbackDiagram = asArray(visualization?.sequenceDiagrams).map(asRecord)[0] ?? null;
  const effective = selected ?? fallbackDiagram;
  if (effective) {
    const prepared = diagramToPrepared(effective, "sequence-view", "Sequence View");
    return {
      ...prepared,
      meta: {
        selectedDiagramName: asString(asRecord(effective).name),
        sequenceDiagram: effective,
        parentContext: asString(asRecord(effective).name),
      },
    };
  }
  return prepareGraph(visualization?.graph, visualization);
}

function diagramToPrepared(diagramInput: unknown, view: string, fallbackTitle: string): PreparedView {
  const diagram = asRecord(diagramInput);
  let nodes = asArray(diagram.nodes ?? diagram.states).map((nodeRaw, index) => {
    const node = asRecord(nodeRaw);
    return buildBehaviorNode(node, index, {
      id: `node-${index}`,
      label: `Node ${index + 1}`,
      kind: asString(node.type ?? node.kind, view),
    });
  });
  let edges = asArray(diagram.edges ?? diagram.transitions)
    .map((edgeRaw, index) => {
      const edge = asRecord(edgeRaw);
      return {
        id: asString(edge.id, `edge-${index}`),
        source: asString(edge.source ?? edge.from ?? edge.sourceId, ""),
        target: asString(edge.target ?? edge.to ?? edge.targetId, ""),
        label: asString(edge.name ?? edge.label ?? edge.type, "")
      };
    });
  // Sequence DTOs use lifelines/messages instead of nodes/edges.
  if (view === "sequence-view" && nodes.length === 0) {
    nodes = asArray(diagram.lifelines).map((lifelineRaw, index) => {
      const lifeline = asRecord(lifelineRaw);
      return buildBehaviorNode(lifeline, index, {
        id: `lifeline-${index}`,
        label: `Lifeline ${index + 1}`,
        kind: "lifeline",
      });
    });
    edges = asArray(diagram.messages).map((messageRaw, index) => {
      const message = asRecord(messageRaw);
      return {
        id: asString(message.id, `message-${index}`),
        source: asString(message.source ?? message.from ?? message.sourceId, ""),
        target: asString(message.target ?? message.to ?? message.targetId, ""),
        label: asString(message.name ?? message.label ?? message.type, ""),
      };
    });
  }
  const ids = new Set(nodes.map((node) => node.id));
  edges = edges.filter((edge) => ids.has(edge.source) && ids.has(edge.target));
  return { title: asString(diagram.name, fallbackTitle), view, nodes, edges };
}

function normalizeDiagramKey(value: string): string {
  return value.replace(/::/g, ".").trim().toLowerCase();
}

function diagramSimpleName(value: string): string {
  const normalized = value.replace(/::/g, ".");
  const segments = normalized.split(".").filter(Boolean);
  return segments[segments.length - 1] ?? normalized;
}

function diagramMatchesSelection(
  diagram: UnknownRecord,
  selectedName?: string,
  selectedViewId?: string,
): boolean {
  const selectors = [selectedName, selectedViewId].filter((value): value is string => Boolean(value?.trim()));
  if (selectors.length === 0) return false;

  const diagramKeys = [
    asString(diagram.id),
    asString(diagram.name),
    `${asString(diagram.package_path)}::${asString(diagram.name)}`.replace(/^::+/, ""),
  ].filter(Boolean);

  return selectors.some((selector) => {
    const selectorKey = normalizeDiagramKey(selector);
    const selectorSimple = diagramSimpleName(selector).toLowerCase();
    return diagramKeys.some((candidate) => {
      const candidateKey = normalizeDiagramKey(candidate);
      const candidateSimple = diagramSimpleName(candidate).toLowerCase();
      return (
        candidateKey === selectorKey ||
        candidateSimple === selectorSimple ||
        candidateKey.endsWith(`.${selectorKey}`) ||
        selectorKey.endsWith(`.${candidateKey}`) ||
        candidateKey.includes(selectorSimple) ||
        selectorKey.includes(candidateSimple)
      );
    });
  });
}

function selectNamedDiagram(
  diagramsInput: unknown,
  selectedName?: string,
  selectedViewId?: string,
): UnknownRecord | null {
  const diagrams = asArray(diagramsInput).map(asRecord);
  if (diagrams.length === 0) return null;
  if (!selectedName && !selectedViewId) return null;
  const matched = diagrams.find((diagram) => diagramMatchesSelection(diagram, selectedName, selectedViewId));
  if (matched) return matched;
  return diagrams.length === 1 ? diagrams[0] : null;
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

function findScopedRootPart(nodes: PreparedNode[], selectedRoot: string): PreparedNode | undefined {
  const normalized = selectedRoot.trim().toLowerCase();
  if (!normalized) return undefined;
  return nodes.find((node) => {
    if (asRecord(node.attributes).isSyntheticContainer) return false;
    const label = node.label.trim().toLowerCase();
    if (label === normalized || node.id.trim().toLowerCase() === normalized) return true;
    const qualifiedName = asString(asRecord(node.attributes).qualifiedName).replace(/::/g, ".").toLowerCase();
    return qualifiedName === normalized || qualifiedName.endsWith(`.${normalized}`);
  });
}

function filterContainerGroupsForScopedRoot(
  containerGroups: UnknownRecord[],
  baseNodes: PreparedNode[],
  scopedName: string,
): UnknownRecord[] {
  if (!scopedName.trim()) return containerGroups;
  const root = findScopedRootPart(baseNodes, scopedName);
  if (!root) return containerGroups;
  const rootQualifiedName = asString(asRecord(root.attributes).qualifiedName).replace(/::/g, ".");
  const packagePrefix = rootQualifiedName.includes(".") ? rootQualifiedName.split(".")[0] : "";
  if (!packagePrefix) return containerGroups;
  return containerGroups.filter((group) => {
    const groupQualifiedName = asString(group.qualifiedName ?? group.label).replace(/::/g, ".");
    const groupLabel = asString(group.label ?? group.name);
    return !(groupQualifiedName === packagePrefix || groupLabel === packagePrefix);
  });
}

/** Remove package wrappers and mark the scoped instance as the single diagram root. */
function collapseRedundantOuterBoundaries(nodes: PreparedNode[], selectedRoot: string): PreparedNode[] {
  const root = findScopedRootPart(nodes, selectedRoot);
  if (!root) return nodes;

  const removedSyntheticIds = new Set<string>();
  let parentId = asString(asRecord(root.attributes).containerId);
  while (parentId) {
    const parent = nodes.find((node) => node.id === parentId);
    if (!parent || !asRecord(parent.attributes).isSyntheticContainer) break;
    removedSyntheticIds.add(parentId);
    parentId = asString(asRecord(parent.attributes).containerId);
  }
  if (removedSyntheticIds.size === 0) {
    root.attributes = { ...asRecord(root.attributes), isDiagramRoot: true };
    return nodes;
  }

  const rootQualifiedName = asString(asRecord(root.attributes).qualifiedName).replace(/::/g, ".");
  const resolveContainerId = (node: PreparedNode): string | null => {
    const attrs = asRecord(node.attributes);
    const current = asString(attrs.containerId);
    if (!current || !removedSyntheticIds.has(current)) return current || null;
    if (node.id === root.id) return null;
    const qualifiedName = asString(attrs.qualifiedName).replace(/::/g, ".");
    if (rootQualifiedName && (qualifiedName === rootQualifiedName || qualifiedName.startsWith(`${rootQualifiedName}.`))) {
      return root.id;
    }
    return null;
  };

  return nodes
    .filter((node) => !removedSyntheticIds.has(node.id))
    .map((node) => {
      const attrs = asRecord(node.attributes);
      const nextContainerId = resolveContainerId(node);
      const nextAttributes: Record<string, unknown> = {
        ...attrs,
        containerId: nextContainerId,
      };
      delete nextAttributes._fallbackContainerId;
      if (node.id === root.id) {
        nextAttributes.isDiagramRoot = true;
      }
      return { ...node, attributes: nextAttributes };
    });
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
    .filter((port) => Boolean(port.name));
}

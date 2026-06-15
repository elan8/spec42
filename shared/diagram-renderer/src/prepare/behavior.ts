import type { PreparedNode, PreparedView, UnknownRecord, VisualizationPayload } from "./types";
import { bestBehaviorDiagram, diagramToPrepared, selectNamedDiagram } from "./diagram-select";
import { prepareGraph } from "./graph";
import { asArray, asRecord, asString, buildBehaviorNode } from "./util";

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
  const enriched = [...actions, ...decisions, ...states].map((node) => {
    const attrs = asRecord(node.attributes);
    const swimLane = asString(attrs.swimLane ?? attrs.swim_lane, "");
    if (!swimLane) {
      return node;
    }
    return {
      ...node,
      attributes: {
        ...(node.attributes ?? {}),
        swimLane,
      },
    };
  });
  return enriched.filter((node) => allowedKinds.has(node.kind));
}

function buildActivityNodeAliasMap(nodes: PreparedNode[]): Map<string, string> {
  const aliases = new Map<string, string>();
  const register = (alias: unknown, nodeId: string) => {
    const key = asString(alias).trim();
    if (!key) return;
    if (!aliases.has(key)) aliases.set(key, nodeId);
    const normalized = key.replace(/::/g, ".");
    if (!aliases.has(normalized)) aliases.set(normalized, nodeId);
    const lastSegment = normalized.split(".").filter(Boolean).pop();
    if (lastSegment && !aliases.has(lastSegment)) aliases.set(lastSegment, nodeId);
  };
  for (const node of nodes) {
    register(node.id, node.id);
    register(node.label, node.id);
    register(asRecord(node.attributes).qualifiedName, node.id);
  }
  return aliases;
}

function resolveActivityNodeRef(value: unknown, aliases: Map<string, string>): string {
  const key = asString(value).trim();
  if (!key) return "";
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

export function prepareActivity(visualization: VisualizationPayload): PreparedView {
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
      const guard = asString(edge.guard ?? edge.type, "");
      const condition = asString(edge.condition, "");
      const guardLower = guard.toLowerCase();
      const succession = guardLower === "flow" || guardLower === "first" || guardLower === "succession";
      const conditional =
        condition.length > 0
        || (guard.length > 0 && !["flow", "first", "bind", "perform", "succession"].includes(guardLower));
      return {
        id: asString(edge.id, `flow-${index}`),
        source,
        target,
        label: asString(edge.name ?? edge.label ?? condition ?? guard, ""),
        attributes: {
          ...(guard ? { guard } : {}),
          ...(condition ? { condition } : {}),
          succession,
          conditional,
        },
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
  const swimLanes = Array.from(
    new Set(
      nodes
        .map((node) => asString(asRecord(node.attributes).swimLane, ""))
        .filter(Boolean),
    ),
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
      swimLanes,
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

function attachCompositeRegions(nodes: PreparedNode[]): PreparedNode[] {
  const childrenByRegion = new Map<string, PreparedNode[]>();
  for (const node of nodes) {
    const regionId = asString(asRecord(node.attributes).regionId, "");
    if (!regionId) continue;
    const bucket = childrenByRegion.get(regionId) ?? [];
    bucket.push(node);
    childrenByRegion.set(regionId, bucket);
  }
  return nodes.map((node) => {
    if (!node.kind.includes("composite")) {
      return node;
    }
    const children = childrenByRegion.get(node.id) ?? [];
    if (children.length === 0) {
      return node;
    }
    return {
      ...node,
      attributes: {
        ...(node.attributes ?? {}),
        regions: children.map((child) => ({ name: child.label, id: child.id })),
      },
    };
  });
}

function collectStateMachineNodes(machine: UnknownRecord): PreparedNode[] {
  const nodes = asArray(machine.states).map((stateRaw, index) => {
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
      entry: state.entry ?? element.entry,
      do: state.do ?? element.do,
      exit: state.exit ?? element.exit,
      parentId: state.parentId ?? state.parent_id ?? element.parentId,
      regionId: state.regionId ?? state.region_id ?? element.regionId,
    };
    const kind = asString(state.kind ?? state.type ?? element.type, "state").toLowerCase();
    const behaviorNode = buildBehaviorNode(asRecord(merged), index, {
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
    const entry = asString(merged.entry, "");
    const doAction = asString(merged.do, "");
    const exit = asString(merged.exit, "");
    const regionId = asString(merged.regionId, "");
    if (entry || doAction || exit || regionId) {
      behaviorNode.attributes = {
        ...(behaviorNode.attributes ?? {}),
        ...(entry ? { entry } : {}),
        ...(doAction ? { do: doAction } : {}),
        ...(exit ? { exit } : {}),
        ...(regionId ? { regionId } : {}),
      };
    }
    return behaviorNode;
  });
  return attachCompositeRegions(nodes);
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
      const labelParts = [
        asString(edge.label, ""),
        asString(edge.guard, ""),
        asString(edge.effect, ""),
        asString(edge.accept, ""),
        asString(edge.name, ""),
      ].filter((part) => part && part.toLowerCase() !== "entry");
      const label = Array.from(new Set(labelParts)).join(" / ");
      return {
        id: asString(edge.id, `transition-${index}`),
        source,
        target,
        label,
        attributes: {
          selfLoop: Boolean(edge.selfLoop ?? source === target),
          guard: edge.guard,
          effect: edge.effect,
          accept: edge.accept,
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

export function prepareState(visualization: VisualizationPayload): PreparedView {
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
    attributes: asRecord(node.attributes),
  }));
  const edges = asArray(graph.edges)
    .map((edgeRaw, index) => {
      const edge = asRecord(edgeRaw);
      return {
        id: `transition-${index}`,
        source: asString(edge.source),
        target: asString(edge.target),
        label: asString(edge.name ?? edge.type ?? edge.rel_type, ""),
      };
    })
    .filter((edge) => ids.has(edge.source) && ids.has(edge.target));
  const synthesizeInitial = visualization?.synthesizeInitialState === true;
  const hasInitial = nodes.some((node) => node.kind.toLowerCase().includes("initial") || node.label.toLowerCase() === "initial");
  const withSyntheticInitial =
    synthesizeInitial && !hasInitial && nodes.length > 0
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

export function prepareSequence(visualization: VisualizationPayload): PreparedView {
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

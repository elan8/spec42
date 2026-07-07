import type { PreparedNode, PreparedView, UnknownRecord, VisualizationPayload } from "../types";
import { diagramToPrepared, selectNamedDiagram } from "../diagram-select";
import { asArray, asRecord, asString, buildBehaviorNode } from "../util";
import { buildActivityNodeAliasMap, resolveActivityNodeRef } from "./common";

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

function attachExplicitRegions(nodes: PreparedNode[], machine: UnknownRecord): PreparedNode[] {
  const regions = asArray(machine.regions).map(asRecord);
  if (regions.length === 0) {
    return attachCompositeRegions(nodes);
  }
  const regionsByParent = new Map<string, UnknownRecord[]>();
  for (const region of regions) {
    const parentId = asString(region.parentId ?? region.parent_id, "");
    if (!parentId) continue;
    const bucket = regionsByParent.get(parentId) ?? [];
    bucket.push(region);
    regionsByParent.set(parentId, bucket);
  }
  return nodes.map((node) => {
    if (!node.kind.includes("composite")) {
      return node;
    }
    const explicit = regionsByParent.get(node.id) ?? [];
    if (explicit.length === 0) {
      return node;
    }
    return {
      ...node,
      attributes: {
        ...(node.attributes ?? {}),
        regions: explicit.map((region, index) => ({
          id: asString(region.id, `region-${index}`),
          name: asString(region.name, `region ${index + 1}`),
        })),
      },
    };
  });
}

function formatStateTransitionLabel(edge: UnknownRecord): string {
  const parts: string[] = [];
  const guard = asString(edge.guard, "").trim();
  const effect = asString(edge.effect, "").trim();
  const accept = asString(edge.accept, "").trim();
  const send = asString(edge.send, "").trim();
  const label = asString(edge.label, "").trim();
  if (guard) parts.push(`[${guard}]`);
  if (effect) parts.push(effect);
  if (accept) parts.push(`accept ${accept}`);
  if (send) parts.push(`send ${send}`);
  if (parts.length > 0) {
    return parts.join(" / ");
  }
  return label;
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
        : kind.includes("terminate")
          ? "terminate"
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
  return attachExplicitRegions(nodes, machine);
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
      const label = formatStateTransitionLabel(edge);
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
          send: edge.send,
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

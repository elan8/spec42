import type { PreparedNode, PreparedView, UnknownRecord, VisualizationPayload } from "../types";
import { bestBehaviorDiagram, selectNamedDiagram } from "../diagram-select";
import { asArray, asRecord, asString, buildBehaviorNode } from "../util";
import { buildActivityNodeAliasMap, resolveActivityNodeRef } from "./common";

function activityDiagramCatalog(visualization: VisualizationPayload): UnknownRecord[] {
  const normalized = asArray(visualization.diagrams).map(asRecord);
  if (normalized.length > 0) {
    return normalized;
  }
  return asArray(visualization.activityDiagrams).map(asRecord);
}

function collectActivityNodes(diagram: UnknownRecord): PreparedNode[] {
  const allowedKinds = new Set([
    "action",
    "perform",
    "assign",
    "for-loop",
    "decision",
    "merge",
    "fork",
    "join",
    "initial",
    "final",
    "terminate",
    "accept",
    "send",
  ]);
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
      ["initial", "final", "decision", "merge", "fork", "join", "assign", "for-loop", "terminate", "accept", "send"].some(
        (token) => node.kind.includes(token),
      ),
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
              : kind.includes("assign")
                ? "assign"
                : kind.includes("for-loop") || kind.includes("forloop")
                  ? "for-loop"
                  : kind.includes("terminate")
                    ? "terminate"
                    : kind.includes("accept")
                      ? "accept"
                      : kind.includes("send")
                        ? "send"
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

import { resolveNodeChrome } from "../node-notation";
import { prepareActivity, prepareSequence, prepareState } from "./behavior";
import { normalizeVisualizationPayload } from "./normalize-payload";
import { prepareGraph } from "./graph";
import { prepareInterconnection } from "./interconnection";
import { prepareBrowser, prepareGeometry, prepareGrid } from "./standard-views";
import type { PreparedEdge, PreparedNode, PreparedView, VisualizationPayload } from "./types";
import { asArray, asRecord, asString } from "./util";

export type {
  InterconnectionLayoutDto,
  InterconnectionPreparedEdge,
  InterconnectionPreparedNode,
  InterconnectionPreparedView,
  PreparedEdge,
  PreparedNode,
  PreparedView,
} from "./types";
export {
  asInterconnectionPrepared,
  interconnectionPreparedForLayout,
  isInterconnectionPreparedView,
} from "./types";
export { resolveNodeChrome } from "../node-notation";

/** Structure-only CSS classes (definition / usage / reference / container); no per-kind color. */
export function nodeStructureClass(
  kind: string,
  isDefinition?: boolean,
  isReference?: boolean,
): string {
  return resolveNodeChrome(isReference ? "reference-usage" : isDefinition ? "definition" : "usage").structureClass;
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
    case "browser-view":
      return "Browser";
    case "grid-view":
      return "Grid";
    case "geometry-view":
      return "Geometry";
    default:
      return "General";
  }
}

export function prepareViewData(visualizationInput: unknown): PreparedView {
  const typed = prepareTypedDiagramProduct(visualizationInput);
  if (typed) return typed;
  const passthrough = asRecord(visualizationInput).preparedView;
  if (passthrough && typeof passthrough === "object") {
    const candidate = asRecord(passthrough) as unknown as PreparedView;
    if (typeof candidate.view === "string" && Array.isArray(candidate.nodes) && Array.isArray(candidate.edges)) {
      return candidate;
    }
  }
  const normalized = normalizeVisualizationPayload(asRecord(visualizationInput) as Record<string, unknown>);
  const visualization = asRecord(normalized) as VisualizationPayload;
  const view = visualization?.view || "general-view";
  if (view === "interconnection-view") return prepareInterconnection(visualization);
  if (view === "action-flow-view") return prepareActivity(visualization);
  if (view === "state-transition-view") return prepareState(visualization);
  if (view === "sequence-view") return prepareSequence(visualization);
  if (view === "browser-view") return prepareBrowser(visualization);
  if (view === "grid-view") return prepareGrid(visualization);
  if (view === "geometry-view") return prepareGeometry(visualization);
  return prepareGraph(visualization?.generalViewGraph ?? visualization?.graph, visualization);
}

/**
 * Adapt a schema-5 `sequence-view` projection into the `{ lifelines, messages }` shape the
 * sequence renderer consumes.
 *
 * The projection gives us: `metadata.participants` (lifeline node indices),
 * `metadata.messages` (message node indices), `flow` edges (send event -> receive event,
 * `origin` = the message), `succession` edges (message -> message, authored order), and
 * `containment` edges (lifeline -> ... -> event). We climb containment to map each message
 * end to its lifeline, and topologically order the messages by succession.
 */
function sequenceDiagramFromProjection(
  name: string,
  projection: Record<string, unknown>,
  nodes: PreparedNode[],
  metadata: Record<string, unknown>,
): Record<string, unknown> | undefined {
  const rawEdges = asArray(projection.edges).map(asRecord);
  const asIndex = (value: unknown): number | undefined =>
    typeof value === "number" && nodes[value] !== undefined ? value : undefined;
  const participantIdx = asArray(metadata.participants)
    .map(asIndex)
    .filter((value): value is number => value !== undefined);
  const messageIdx = asArray(metadata.messages)
    .map(asIndex)
    .filter((value): value is number => value !== undefined);
  if (participantIdx.length === 0 || messageIdx.length === 0) return undefined;

  const participantSet = new Set(participantIdx);
  const ownerOf = new Map<number, number>();
  for (const edge of rawEdges) {
    if (asString(edge.kind) !== "containment") continue;
    const owner = asIndex(edge.source);
    const child = asIndex(edge.target);
    if (owner !== undefined && child !== undefined) ownerOf.set(child, owner);
  }
  const lifelineOf = (node: number): number | undefined => {
    let current: number | undefined = node;
    const seen = new Set<number>();
    while (current !== undefined && !seen.has(current)) {
      if (participantSet.has(current)) return current;
      seen.add(current);
      current = ownerOf.get(current);
    }
    return undefined;
  };

  const ends = new Map<number, { from?: number; to?: number }>();
  for (const edge of rawEdges) {
    if (asString(edge.kind) !== "flow") continue;
    const origin = asIndex(edge.origin);
    const source = asIndex(edge.source);
    const target = asIndex(edge.target);
    if (origin === undefined) continue;
    ends.set(origin, {
      from: source === undefined ? undefined : lifelineOf(source),
      to: target === undefined ? undefined : lifelineOf(target),
    });
  }

  // Authored order: a chain of `succession` edges between message nodes. Messages with no
  // incoming succession start; ties and unordered messages keep declaration order.
  const nextOf = new Map<number, number[]>();
  const hasIncoming = new Set<number>();
  for (const edge of rawEdges) {
    if (asString(edge.kind) !== "succession") continue;
    const from = asIndex(edge.source);
    const to = asIndex(edge.target);
    if (from === undefined || to === undefined) continue;
    const successors = nextOf.get(from) ?? [];
    successors.push(to);
    nextOf.set(from, successors);
    hasIncoming.add(to);
  }
  const ordered: number[] = [];
  const placed = new Set<number>();
  const visit = (node: number): void => {
    if (placed.has(node) || !messageIdx.includes(node)) return;
    placed.add(node);
    ordered.push(node);
    for (const next of nextOf.get(node) ?? []) visit(next);
  };
  for (const message of messageIdx) if (!hasIncoming.has(message)) visit(message);
  for (const message of messageIdx) visit(message);

  const lifelines = participantIdx.map((index) => ({
    id: nodes[index].id,
    name: nodes[index].label || nodes[index].kind,
  }));
  const messages = ordered.map((index, position) => {
    const end = ends.get(index) ?? {};
    return {
      id: nodes[index].id,
      name: nodes[index].label,
      source: end.from === undefined ? undefined : nodes[end.from].id,
      target: end.to === undefined ? undefined : nodes[end.to].id,
      kind: nodes[index].kind,
      order: position + 1,
    };
  }).filter((message) => message.source !== undefined && message.target !== undefined);

  return { name, lifelines, messages, activations: [], fragments: [] };
}

function prepareTypedDiagramProduct(input: unknown): PreparedView | null {
  const product = asRecord(input);
  if (product.schemaVersion !== 5) return null;
  const selected = asRecord(product.selectedView);
  const projection = asRecord(product.projection);
  const documents = Array.isArray(product.documents) ? product.documents.map(asRecord) : [];
  const sources = Array.isArray(product.sources) ? product.sources.map(asRecord) : [];
  const references = Array.isArray(product.references) ? product.references : [];
  if (typeof selected.kind !== "string" || projection.kind !== selected.kind ||
      typeof selected.name !== "string" || !Array.isArray(projection.nodes) ||
      !Array.isArray(projection.edges)) return null;
  const navigation = (index: unknown) => {
    const source = typeof index === "number" ? sources[index] : undefined;
    const document = source && typeof source.document === "number" ? documents[source.document] : undefined;
    const range = source && Array.isArray(source.range) ? source.range : [];
    return {
      uri: document && typeof document.uri === "string" ? document.uri : null,
      range: range.length === 4 ? {
        start: { line: range[0], character: range[1] },
        end: { line: range[2], character: range[3] },
      } : {},
    };
  };
  if (selected.kind === "state-transition-view") {
    const scene = asRecord(projection.scene);
    if (scene.kind !== "state-transition" || !Array.isArray(scene.vertices) || !Array.isArray(scene.transitions)) return null;
    const frame = asRecord(scene.frame);
    const nodes = scene.vertices.map((raw, index): PreparedNode => {
      const vertex = asRecord(raw);
      const source = navigation(vertex.navigation);
      const semanticId = typeof vertex.id === "string" && vertex.id ? vertex.id : String(index);
      return {
        id: `state:${semanticId}`,
        label: typeof vertex.label === "string" ? vertex.label : "",
        kind: String(vertex.kind ?? "state"),
        uri: source.uri,
        range: source.range as PreparedNode["range"],
        attributes: { semanticSceneId: vertex.id },
      };
    });
    const featureLabel = (value: unknown): string => {
      const feature = asRecord(value);
      return feature.status === "supported" && typeof feature.label === "string" ? feature.label : "";
    };
    const triggerLabel = (value: unknown): string => {
      const trigger = asRecord(value);
      return trigger.status === "accept" && typeof trigger.label === "string" ? trigger.label : "";
    };
    const edges = scene.transitions.map((raw, index): PreparedEdge => {
      const transition = asRecord(raw);
      const sourceIndex = transition.source as number;
      const targetIndex = transition.target as number;
      const trigger = triggerLabel(transition.trigger);
      const guard = featureLabel(transition.guard);
      const effect = featureLabel(transition.effect);
      const sourceNavigation = navigation(transition.navigation);
      return {
        id: `transition:${index}`,
        source: nodes[sourceIndex].id,
        target: nodes[targetIndex].id,
        label: [trigger, guard ? `[${guard}]` : "", effect].filter(Boolean).join(" / ") || String(transition.label ?? ""),
        edgeKind: "transition",
        attributes: {
          semanticSceneId: transition.id,
          relationType: "transition",
          selfLoop: transition.source === transition.target,
          trigger,
          guard,
          effect,
          provenance: transition.provenance,
          sourceNavigation,
        },
      };
    });
    return {
      title: typeof frame.label === "string" ? frame.label : selected.name,
      view: selected.kind,
      nodes,
      edges,
      meta: {
        sceneKind: scene.kind,
        frame,
        layoutDirection: "horizontal",
      },
    };
  }
  const nodes = projection.nodes.map((raw, index): PreparedNode => {
    const element = asRecord(raw);
    const source = navigation(element.source);
    const typing = asRecord(element.typing);
    const typeLabels = Array.isArray(typing.types)
      ? typing.types.map(asRecord).map((type) => type.label).filter((label): label is string => typeof label === "string")
      : [];
    return {
      id: `n:${index}`,
      label: typeof element.name === "string" ? element.name : String(element.metaclass ?? ""),
      kind: String(element.metaclass ?? "Unrecognized"),
      uri: source.uri,
      range: source.range as PreparedNode["range"],
      attributes: {
        notationRole: element.notationRole,
        semanticReference: typeof element.reference === "number" ? references[element.reference] : undefined,
        owner: element.owner,
        typingStatus: typing.status,
        typedByName: (typing.status === "resolved" || typing.status === "partial") && typeLabels.length > 0
          ? typeLabels.join(" & ")
          : undefined,
      },
    };
  });
  for (const [ownerIndex, raw] of projection.nodes.entries()) {
    const element = asRecord(raw);
    const compartments = Array.isArray(element.compartments) ? element.compartments.map(asRecord) : [];
    nodes[ownerIndex].attributes = {
      ...nodes[ownerIndex].attributes,
      typedCompartments: compartments.map((compartment) => ({
        kind: String(compartment.kind ?? "members"),
        provenance: String(compartment.provenance ?? "direct"),
        members: (Array.isArray(compartment.members) ? compartment.members : [])
          .filter((member): member is number => typeof member === "number" && nodes[member] !== undefined)
          .map((member) => ({
            id: nodes[member].id,
            name: nodes[member].label,
            kind: nodes[member].kind,
            typeName: typeof nodes[member].attributes?.typedByName === "string"
              ? nodes[member].attributes?.typedByName
              : undefined,
          })),
      })),
    };
  }
  const edges = projection.edges.map((raw, index): PreparedEdge => {
    const edge = asRecord(raw);
    return {
      id: `e:${index}`,
      source: `n:${String(edge.source ?? "")}`,
      target: `n:${String(edge.target ?? "")}`,
      label: "",
      edgeKind: String(edge.kind ?? "relationship"),
      attributes: {
        semanticReference: typeof edge.reference === "number" ? references[edge.reference] : undefined,
        provenance: edge.provenance,
        sourceNavigation: edge.navigation === null ? null : navigation(edge.navigation),
      },
    };
  });
  const metadata = asRecord(projection.metadata);
  const sequenceDiagram = selected.kind === "sequence-view"
    ? sequenceDiagramFromProjection(selected.name, projection, nodes, metadata)
    : undefined;
  const gridRows = Array.isArray(metadata.rows)
    ? metadata.rows.filter((value): value is number => typeof value === "number" && nodes[value] !== undefined)
    : [];
  const gridColumns = Array.isArray(metadata.columns)
    ? metadata.columns.filter((value): value is string => typeof value === "string")
    : [];
  const gridRelationships = Array.isArray(metadata.cells) ? metadata.cells.map(asRecord) : [];
  const gridCells = selected.kind === "grid-view"
    ? gridRows.map((nodeIndex) => {
        const node = nodes[nodeIndex];
        const values = Object.fromEntries(gridColumns.map((column) => [
          `relationship:${column}`,
          gridRelationships.some((cell) => cell.row === nodeIndex && cell.column === column) ? "✓" : "",
        ]));
        return { id: node.id, name: node.label, kind: node.kind, ...values };
      })
    : undefined;
  return {
    title: selected.name,
    view: selected.kind,
    nodes,
    edges,
    meta: {
      selectedDiagramReference: typeof selected.reference === "number" ? references[selected.reference] : undefined,
      exposedRoots: Array.isArray(projection.exposedRoots)
        ? projection.exposedRoots.map((index) => `n:${String(index)}`)
        : [],
      viewMetadata: projection.metadata,
      ...(sequenceDiagram ? { sequenceDiagram } : {}),
      ...(selected.kind === "grid-view" ? {
        cells: gridCells,
        columns: [
          { key: "name", label: "Element", notationStatus: "normative" },
          ...gridColumns.map((column) => ({
            key: `relationship:${column}`,
            label: column,
            notationStatus: "normative",
          })),
        ],
        provisional: false,
      } : {}),
    },
  };
}

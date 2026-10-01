import { resolveNodeChrome } from "../node-notation";
import { normalizeEdgeKind } from "../graph-normalization";
import type { DiagramProduct } from "../../../src/generated/diagram-product/DiagramProduct";
import type { ProjectionFeature } from "../../../src/generated/diagram-product/ProjectionFeature";
import type { SequenceScene } from "../../../src/generated/diagram-product/SequenceScene";
import type { TransitionTrigger } from "../../../src/generated/diagram-product/TransitionTrigger";
import { prepareActivity, prepareSequence, prepareState } from "./behavior";
import { normalizeVisualizationPayload } from "./normalize-payload";
import { prepareGraph } from "./graph";
import { prepareInterconnection } from "./interconnection";
import { prepareInterconnectionFromTypedProjection } from "./interconnection-typed";
import { prepareBrowser, prepareGeometry, prepareGrid } from "./standard-views";
import type { PreparedEdge, PreparedNode, PreparedView, VisualizationPayload } from "./types";
import { asRecord } from "./util";

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
 * Adapt the authority-owned schema-5 sequence scene into the legacy shape the renderer consumes.
 * Endpoint ownership and temporal ordering are semantic facts already settled by the scene.
 */
function sequenceDiagramFromScene(
  name: string,
  nodes: PreparedNode[],
  scene: SequenceScene,
): Record<string, unknown> | undefined {
  const asIndex = (value: unknown): number | undefined =>
    typeof value === "number" && nodes[value] !== undefined ? value : undefined;
  const participantIdx = scene.lifelines
    .map(asIndex)
    .filter((value): value is number => value !== undefined);

  const lifelines = participantIdx.map((index) => ({
    id: nodes[index].id,
    name: nodes[index].label || nodes[index].kind,
  }));
  const messages = scene.messages.map((message) => {
    const index = asIndex(message.node);
    const sourceIndex = message.source.status === "resolved" ? asIndex(message.source.lifeline) : undefined;
    const targetIndex = message.target.status === "resolved" ? asIndex(message.target.lifeline) : undefined;
    return {
      id: index === undefined ? undefined : nodes[index].id,
      name: message.label,
      source: sourceIndex === undefined ? undefined : nodes[sourceIndex].id,
      target: targetIndex === undefined ? undefined : nodes[targetIndex].id,
      kind: index === undefined ? "FlowUsage" : nodes[index].kind,
      order: message.order.status === "resolved" ? message.order.value : undefined,
    };
  }).filter((message) => message.id !== undefined && message.source !== undefined &&
    message.target !== undefined && message.order !== undefined);

  return { name, lifelines, messages, activations: [], fragments: [] };
}

function isDiagramProduct(input: unknown): input is DiagramProduct {
  const product = asRecord(input);
  const selected = asRecord(product.selectedView);
  const projection = asRecord(product.projection);
  return product.schemaVersion === 5 && typeof product.modelDigest === "string" &&
    Array.isArray(product.documents) && Array.isArray(product.sources) && Array.isArray(product.references) &&
    typeof selected.kind === "string" && typeof selected.name === "string" &&
    projection.kind === selected.kind && Array.isArray(projection.exposedRoots) &&
    Array.isArray(projection.nodes) && Array.isArray(projection.relationships) &&
    Array.isArray(projection.edges) && typeof projection.metadata === "object" &&
    typeof projection.scene === "object";
}

function prepareTypedDiagramProduct(input: unknown): PreparedView | null {
  if (!isDiagramProduct(input)) return null;
  const { selectedView: selected, projection, documents, sources, references } = input;
  const navigation = (index: unknown) => {
    const source = typeof index === "number" ? sources[index] : undefined;
    const document = source ? documents[source.document] : undefined;
    const range = source?.range;
    return {
      uri: document?.uri ?? null,
      range: range ? {
        start: { line: range[0], character: range[1] },
        end: { line: range[2], character: range[3] },
      } : {},
    };
  };
  if (projection.kind === "state-transition-view") {
    const { scene } = projection;
    const frame = scene.frame;
    const nodes = scene.vertices.map((vertex, index): PreparedNode => {
      const source = navigation(vertex.navigation);
      const semanticId = vertex.id || String(index);
      return {
        id: `state:${semanticId}`,
        label: vertex.label,
        kind: vertex.kind,
        uri: source.uri,
        range: source.range as PreparedNode["range"],
        attributes: { semanticSceneId: vertex.id },
      };
    });
    const featureLabel = (feature: ProjectionFeature): string =>
      feature.status === "supported" ? feature.label : "";
    const triggerLabel = (trigger: TransitionTrigger): string =>
      trigger.status === "accept" ? trigger.label : "";
    const edges = scene.transitions.map((transition, index): PreparedEdge => {
      const sourceIndex = transition.source;
      const targetIndex = transition.target;
      const trigger = triggerLabel(transition.trigger);
      const guard = featureLabel(transition.guard);
      const effect = featureLabel(transition.effect);
      const sourceNavigation = navigation(transition.navigation);
      return {
        id: `transition:${index}`,
        source: nodes[sourceIndex].id,
        target: nodes[targetIndex].id,
        label: [trigger, guard ? `[${guard}]` : "", effect].filter(Boolean).join(" / ") || transition.label || "",
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
      title: frame?.label ?? selected.name,
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
  if (projection.kind === "interconnection-view") {
    return prepareInterconnectionFromTypedProjection({
      name: selected.name,
      nodes: projection.nodes,
      edges: projection.edges,
      exposedRoots: projection.exposedRoots,
      metadata: projection.metadata,
      references,
      navigation,
    });
  }
  const nodes = projection.nodes.map((element, index): PreparedNode => {
    const source = navigation(element.source);
    const typing = element.typing;
    const typeLabels = typing.status === "resolved" || typing.status === "partial"
      ? typing.types.map((type) => type.label)
      : [];
    return {
      id: `n:${index}`,
      label: element.name ?? element.metaclass,
      kind: element.metaclass,
      uri: source.uri,
      range: source.range as PreparedNode["range"],
      attributes: {
        notationRole: element.notationRole,
        semanticReference: references[element.reference],
        owner: element.owner,
        typingStatus: typing.status,
        typedByName: (typing.status === "resolved" || typing.status === "partial") && typeLabels.length > 0
          ? typeLabels.join(" & ")
          : undefined,
      },
    };
  });
  for (const [ownerIndex, element] of projection.nodes.entries()) {
    const compartments = element.compartments;
    nodes[ownerIndex].attributes = {
      ...nodes[ownerIndex].attributes,
      typedCompartments: compartments.map((compartment) => ({
        kind: compartment.kind,
        provenance: compartment.provenance,
        members: compartment.members
          .filter((member) => nodes[member] !== undefined)
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
  const edges = projection.edges.map((edge, index): PreparedEdge => {
    const origin = `n:${edge.origin}`;
    return {
      id: `e:${index}`,
      source: `n:${edge.source}`,
      target: `n:${edge.target}`,
      label: "",
      edgeKind: normalizeEdgeKind(String(edge.kind ?? "relationship")),
      attributes: {
        originNodeId: origin,
        semanticReference: references[edge.reference],
        provenance: edge.provenance,
        sourceNavigation: edge.navigation === null ? null : navigation(edge.navigation),
      },
    };
  });
  const sequenceDiagram = projection.kind === "sequence-view"
    ? sequenceDiagramFromScene(selected.name, nodes, projection.scene)
    : undefined;
  const gridRows = projection.kind === "grid-view"
    ? projection.metadata.rows.filter((value) => nodes[value] !== undefined)
    : [];
  const gridColumns = projection.kind === "grid-view" ? projection.metadata.columns : [];
  const gridRelationships = projection.kind === "grid-view" ? projection.metadata.cells : [];
  const gridCells = projection.kind === "grid-view"
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
      selectedDiagramReference: references[selected.reference],
      exposedRoots: projection.exposedRoots.map((index) => `n:${index}`),
      viewMetadata: projection.metadata,
      ...(sequenceDiagram ? { sequenceDiagram } : {}),
      ...(projection.kind === "grid-view" ? {
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

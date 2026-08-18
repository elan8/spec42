import { resolveNodeChrome } from "../node-notation";
import { prepareActivity, prepareSequence, prepareState } from "./behavior";
import { normalizeVisualizationPayload } from "./normalize-payload";
import { prepareGraph } from "./graph";
import { prepareInterconnection } from "./interconnection";
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
export { isDefinitionKind, isReferenceKind, resolveNodeChrome } from "../node-notation";

/** Structure-only CSS classes (definition / usage / reference / container); no per-kind color. */
export function nodeStructureClass(
  kind: string,
  isDefinition?: boolean,
  isReference?: boolean,
): string {
  return resolveNodeChrome(kind, { isDefinition, isReference }).structureClass;
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

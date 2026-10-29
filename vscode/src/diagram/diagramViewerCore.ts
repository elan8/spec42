import { DIAGRAM_PRODUCT_SCHEMA_VERSION } from "../generated/diagram-product/schemaVersion";
import * as path from "path";
import type { DiagramProduct } from "../generated/diagram-product/DiagramProduct";

export type { DiagramProduct } from "../generated/diagram-product/DiagramProduct";

export type SourceNavigation = {
  uri: string;
  startLine: number;
  startCharacter: number;
  endLine: number;
  endCharacter: number;
};

export function visibleSourceColumn(
  uri: string,
  editors: readonly { uri: string; viewColumn?: number }[],
): number | undefined {
  return editors.find((editor) => editor.uri === uri)?.viewColumn;
}

export const DIAGRAM_VIEWS = [
  { id: "general-view", label: "General View", queryStatus: "implemented" },
  { id: "interconnection-view", label: "Interconnection View", queryStatus: "implemented" },
  { id: "action-flow-view", label: "Action Flow View", queryStatus: "implemented" },
  { id: "state-transition-view", label: "State Transition View", queryStatus: "implemented" },
  { id: "sequence-view", label: "Sequence View", queryStatus: "implemented" },
  { id: "browser-view", label: "Browser View", queryStatus: "implemented" },
  { id: "grid-view", label: "Grid View", queryStatus: "implemented" },
  { id: "geometry-view", label: "Geometry View", queryStatus: "implemented" },
] as const;

export type DiagramViewId = typeof DIAGRAM_VIEWS[number]["id"];

export type DiagramSemanticReference =
  | { kind: "qualified-name"; document: string; qualifiedName: string; sourceDomain: string }
  | { kind: "tooling-element-id"; elementId: string; sourceDomain: string }
  | { kind: "source-anchor"; document: string; ownerQualifiedName: string | null; metaclass: string; sourceDomain: string; range: unknown }
  | { kind: "relationship"; document: string; sourceQualifiedName: string; relationshipKind: string; ordinal: number; sourceDomain: string };

const NOTATION_ROLES = new Set([
  "definition", "usage", "reference-usage", "namespace", "annotation", "unsupported",
]);

/** `spec42/diagram`: the natively built diagram product of one catalog view. */
export type LspDiagramResult = {
  modelDigest: string;
  productJson: string;
  durationUs: number;
};

export type StateTransitionViewChoice = {
  handle: string;
  semanticId: string;
  name: string;
  exposedMachine: { semanticId: string; label: string };
  source: { uri: string };
};

export type StateTransitionViewCatalog = {
  modelDigest: string;
  views: StateTransitionViewChoice[];
};

export type DiagramViewCatalog = {
  modelDigest: string;
  views: Array<{
    handle: string;
    kind: DiagramViewId;
    reference: DiagramSemanticReference;
    name: string;
    namespace?: string | null;
    source: { uri: string; range?: { startLine: number; startCharacter: number; endLine: number; endCharacter: number } };
  }>;
};

export function parseDiagramViewCatalog(value: unknown): DiagramViewCatalog {
  if (!value || typeof value !== "object") throw new Error("Spec42 returned an invalid diagram catalog.");
  const candidate = value as Record<string, unknown>;
  if (typeof candidate.modelDigest !== "string" || !Array.isArray(candidate.views)) {
    throw new Error("Spec42 diagram catalog is missing its model identity or views.");
  }
  const views = candidate.views.map((entry) => {
    if (!entry || typeof entry !== "object") throw new Error("Spec42 returned an invalid diagram view.");
    const view = entry as Record<string, unknown>;
    const source = view.source as Record<string, unknown> | undefined;
    if (typeof view.handle !== "string" || !DIAGRAM_VIEWS.some((known) => known.id === view.kind) || !isDiagramSemanticReference(view.reference) ||
        typeof view.name !== "string" || !source || typeof source.uri !== "string") {
      throw new Error("Spec42 returned malformed diagram view identity.");
    }
    if (view.namespace !== undefined && view.namespace !== null && typeof view.namespace !== "string") {
      throw new Error("Spec42 returned an invalid view namespace.");
    }
    const range = source.range as DiagramViewCatalog["views"][number]["source"]["range"];
    if (range !== undefined && !parseSourceNavigation({ ...range, uri: source.uri })) {
      throw new Error("Spec42 returned an invalid view definition range.");
    }
    return { handle: view.handle, kind: view.kind as DiagramViewId, reference: view.reference, name: view.name,
      ...(view.namespace !== undefined ? { namespace: view.namespace as string | null } : {}),
      source: { uri: source.uri, ...(range ? {range} : {}) } };

  });
  return { modelDigest: candidate.modelDigest, views };
}

export function diagramViewsForDocument(
  catalog: DiagramViewCatalog,
  documentUri: string
): Array<typeof DIAGRAM_VIEWS[number]> {
  const authoredKinds = new Set(catalog.views
    .filter((view) => view.source.uri === documentUri)
    .map((view) => view.kind));
  return DIAGRAM_VIEWS.filter((view) => authoredKinds.has(view.id));
}

/** One selectable entry in the diagram view's dropdown. */
export type DiagramViewOption = {
  handle: string;
  kind: DiagramViewId;
  name: string;
  label: string;
  /** Canonical definition document; grouping uses its semantic owner namespace. */
  documentUri: string;
  group: string;
  kindLabel?: string;
  definition?: SourceNavigation;
};

const DIAGRAM_VIEW_LABELS = new Map<DiagramViewId, string>(
  DIAGRAM_VIEWS.map((view) => [view.id, view.label]),
);

function basename(uri: string): string {
  const trimmed = uri.replace(/[/\\]+$/, "");
  const slash = Math.max(trimmed.lastIndexOf("/"), trimmed.lastIndexOf("\\"));
  const tail = slash >= 0 ? trimmed.slice(slash + 1) : trimmed;
  try {
    return decodeURIComponent(tail);
  } catch {
    return tail;
  }
}

function toOption(view: DiagramViewCatalog["views"][number]): DiagramViewOption {
  return {
    handle: view.handle,
    kind: view.kind,
    name: view.name,
    label: view.name,
    kindLabel: DIAGRAM_VIEW_LABELS.get(view.kind) ?? view.kind,
    documentUri: view.source.uri,
    group: view.namespace === null ? "(Root namespace)"
      : view.namespace === "" ? "(Unnamed namespace)"
      : view.namespace ?? "(Namespace unavailable)",
    ...(view.source.range ? { definition: { ...view.source.range, uri: view.source.uri } } : {}),
  };
}

/** Namespace groups and view names sort alphabetically, independently of kind and file.
 * Older servers without ownership metadata get an explicit unavailable group, not a guessed owner. */
export function allDiagramViewOptions(catalog: DiagramViewCatalog): DiagramViewOption[] {
  const options = catalog.views.map(toOption);
  const counts = new Map<string, number>();
  for (const option of options) {
    for (const key of [JSON.stringify([option.group, option.name]), JSON.stringify([option.group, option.name, option.kind])]) {
      counts.set(key, (counts.get(key) ?? 0) + 1);
    }
  }
  for (const option of options) {
    if ((counts.get(JSON.stringify([option.group, option.name])) ?? 0) > 1) {
      option.label = `${option.name} · ${option.kindLabel}`;
      if ((counts.get(JSON.stringify([option.group, option.name, option.kind])) ?? 0) > 1) {
        option.label += ` · ${basename(option.documentUri)}`;
      }
    }
  }
  const alphabetically = (a: string, b: string) => a.localeCompare(b, undefined, {sensitivity:"base", numeric:true}) || a.localeCompare(b);
  return options.sort((a, b) => alphabetically(a.group,b.group) || alphabetically(a.name,b.name) || a.kind.localeCompare(b.kind) || a.documentUri.localeCompare(b.documentUri) || a.handle.localeCompare(b.handle));
}

/** The subset authored in `documentUri` (used to pick a sensible default when a file is active). */
export function authoredDiagramViewOptions(
  catalog: DiagramViewCatalog,
  documentUri: string,
): DiagramViewOption[] {
  return allDiagramViewOptions(catalog).filter((option) => option.documentUri === documentUri);
}

export function diagramViewKindForHandle(
  catalog: DiagramViewCatalog,
  handle: string,
): DiagramViewId | undefined {
  return catalog.views.find((view) => view.handle === handle)?.kind;
}

/**
 * Which handle a regeneration should target, given the panel's remembered handle and a freshly
 * fetched catalog. Keeps the current selection when it still exists; otherwise falls back to the
 * first authored view so a renamed or removed view does not dead-end the panel.
 */
export function resolveDiagramSelection(
  options: DiagramViewOption[],
  rememberedHandle: string | undefined,
): DiagramViewOption | undefined {
  if (rememberedHandle) {
    const kept = options.find((option) => option.handle === rememberedHandle);
    if (kept) return kept;
  }
  return options[0];
}

/**
 * Backoff for the diagram view's reconcile loop, which runs while the view is visible and its
 * render does not yet match the publication. `attempt` is the 0-based consecutive-failure count;
 * `settled` is set once an answer looks final (no model files, or a genuinely empty catalog) so
 * the loop drops to a slow keep-checking cadence instead of a spinner-speed retry.
 */
export function reconcileDelayMs(attempt: number, settled: boolean): number {
  if (settled) return 45_000;
  const capped = Math.min(Math.max(0, Math.trunc(attempt)), 6);
  return Math.min(15_000, 600 * 2 ** capped);
}

/**
 * Whether a `spec42/publicationChanged` notification means the panel's current render is stale.
 * A render with no known digest (nothing drawn yet) is not "stale" — it just has nothing to
 * compare — so this returns false and the caller leaves the panel alone.
 */
export function diagramRenderIsStale(
  renderedModelDigest: string | undefined,
  publishedModelDigest: string,
): boolean {
  return renderedModelDigest !== undefined && renderedModelDigest !== publishedModelDigest;
}

export function parseStateTransitionViewCatalog(value: unknown): StateTransitionViewCatalog {
  if (!value || typeof value !== "object") throw new Error("Spec42 returned an invalid state-transition catalog.");
  const candidate = value as Record<string, unknown>;
  if (typeof candidate.modelDigest !== "string" || !Array.isArray(candidate.views)) {
    throw new Error("Spec42 state-transition catalog is missing its model identity or views.");
  }
  const views = candidate.views.map((entry) => {
    if (!entry || typeof entry !== "object") throw new Error("Spec42 returned an invalid state-transition view.");
    const view = entry as Record<string, unknown>;
    const semanticId = view.semanticId ?? view.semantic_id;
    const machine = (view.exposedMachine ?? view.exposed_machine) as Record<string, unknown> | undefined;
    const source = view.source as Record<string, unknown> | undefined;
    const machineSemanticId = machine?.semanticId ?? machine?.semantic_id;
    if (typeof view.handle !== "string" || typeof semanticId !== "string" || typeof view.name !== "string" ||
        !machine || typeof machineSemanticId !== "string" || typeof machine.label !== "string" ||
        !source || typeof source.uri !== "string") {
      throw new Error("Spec42 returned malformed state-transition view identity.");
    }
    return {
      handle: view.handle,
      semanticId,
      name: view.name,
      exposedMachine: { semanticId: machineSemanticId, label: machine.label },
      source: { uri: source.uri },
    };
  });
  return { modelDigest: candidate.modelDigest, views };
}

// Says how big the view is, so the author can tell what to narrow rather than only that it failed.
export function oversizedProductMessage(text: string, bytes: number): string {
  const mib = (value: number) => (value / (1024 * 1024)).toFixed(1);
  let counts = "";
  try {
    const projection = (JSON.parse(text) as { projection?: Record<string, unknown> }).projection ?? {};
    const length = (value: unknown) => (Array.isArray(value) ? value.length : 0);
    counts = ` (${length(projection.nodes)} elements, ${length(projection.edges)} edges, ` +
      `${length(projection.relationships)} relationships)`;
  } catch { /* The size alone still explains the refusal. */ }
  return `This view's diagram product is ${mib(bytes)} MiB${counts}, over the ` +
    `${mib(MAX_DIAGRAM_PRODUCT_BYTES)} MiB viewer limit. Narrow what the view exposes, ` +
    "or split it into several views.";
}

export function parseLspDiagramResult(value: unknown): LspDiagramResult {
  if (!value || typeof value !== "object") throw new Error("Spec42 returned an invalid diagram result.");
  const candidate = value as Record<string, unknown>;
  if (typeof candidate.modelDigest !== "string" || typeof candidate.productJson !== "string" ||
      typeof candidate.durationUs !== "number") {
    throw new Error("Spec42 diagram result is missing its model digest or product.");
  }
  return {
    modelDigest: candidate.modelDigest,
    productJson: candidate.productJson,
    durationUs: candidate.durationUs,
  };
}

export const MAX_DIAGRAM_PRODUCT_BYTES = 16 * 1024 * 1024;

/** The sequence reasons name projected nodes; their explanations index them without guards. */
function isCompletenessReasonShape(reason: Record<string, unknown>, nodeCount: number): boolean {
  const nodes = (value: unknown) => Array.isArray(value) && value.every((index) => indexIn(index, nodeCount));
  switch (reason.code) {
    case "sequence-content-omitted":
      return nodes(reason.elements);
    case "sequence-ordering-cycle":
      return Array.isArray(reason.witness) && reason.witness.every((step) => {
        if (!step || typeof step !== "object") return false;
        const candidate = step as Record<string, unknown>;
        return indexIn(candidate.source, nodeCount) && indexIn(candidate.target, nodeCount) &&
          ["authored", "implied"].includes(String(candidate.provenance)) && nodes(candidate.events);
      });
    default:
      return true;
  }
}

export function parseDiagramProduct(text: string): DiagramProduct {
  const bytes = Buffer.byteLength(text, "utf8");
  if (bytes > MAX_DIAGRAM_PRODUCT_BYTES) throw new Error(oversizedProductMessage(text, bytes));
  let value: unknown;
  try { value = JSON.parse(text); }
  catch { throw new Error("Generated diagram product is not valid JSON."); }
  if (!value || typeof value !== "object") throw new Error("Generated diagram product is not an object.");
  const product = value as Record<string, unknown>;
  const view = product.selectedView as Record<string, unknown> | undefined;
  const completeness = product.completeness as Record<string, unknown> | undefined;
  const projection = product.projection as Record<string, unknown> | undefined;
  const documents = product.documents;
  const sources = product.sources;
  const references = product.references;
  if (product.schemaVersion !== DIAGRAM_PRODUCT_SCHEMA_VERSION || typeof product.modelDigest !== "string" ||
      !Array.isArray(documents) || !documents.every(isDiagramDocument) ||
      !Array.isArray(sources) || !sources.every((source) => isDiagramSource(source, documents.length)) ||
      !Array.isArray(references) || !references.every((reference) => isProductReference(reference, documents.length, sources.length, references.length)) ||
      !view || !DIAGRAM_VIEWS.some((candidate) => candidate.id === view.kind) ||
      !indexIn(view.reference, references.length) || !indexIn(view.source, sources.length) || typeof view.name !== "string" ||
      !completeness || !["complete", "incomplete"].includes(String(completeness.status)) || !Array.isArray(completeness.reasons) ||
      !projection || projection.kind !== view.kind || !Array.isArray(projection.exposedRoots) ||
      !Array.isArray(projection.nodes) || !projection.exposedRoots.every((index) => indexIn(index, (projection.nodes as unknown[]).length)) ||
      !projection.nodes.every((node) => isDiagramNode(node, references.length, sources.length, (projection.nodes as unknown[]).length)) ||
      !Array.isArray(projection.relationships) ||
      !Array.isArray(projection.edges) || !projection.metadata || typeof projection.metadata !== "object" ||
      !isDiagramScene(projection.scene, String(view.kind), sources.length)) {
    throw new Error(`Generated diagram product does not match schema version ${DIAGRAM_PRODUCT_SCHEMA_VERSION}.`);
  }
  const presentationKeys = (projection.nodes as Array<{ presentationKey: string }>).map(node => node.presentationKey);
  if (new Set(presentationKeys).size !== presentationKeys.length) {
    throw new Error("Generated diagram product has duplicate presentation keys.");
  }
  for (const reason of completeness.reasons) {
    if (!reason || typeof reason !== "object") throw new Error("Generated diagram product has a malformed completeness reason.");
    const candidate = reason as Record<string, unknown>;
    if (typeof candidate.code !== "string") {
      throw new Error("Generated diagram product has a malformed completeness reason.");
    }
    if (candidate.exposure !== undefined && !indexIn(candidate.exposure, references.length)) {
      throw new Error("Generated diagram product has an out-of-range completeness reference.");
    }
    if (!isCompletenessReasonShape(candidate, (projection.nodes as unknown[]).length)) {
      throw new Error("Generated diagram product has a malformed completeness reason.");
    }
  }
  if (!(projection.edges as unknown[]).every((edge) => isDiagramEdge(edge, references.length, sources.length, (projection.nodes as unknown[]).length)) ||
      !(projection.relationships as unknown[]).every((relationship) => isDiagramRelationship(relationship, references.length, sources.length, (projection.nodes as unknown[]).length))) {
    throw new Error("Generated diagram product contains an invalid graph index.");
  }
  return value as DiagramProduct;
}

function isDiagramScene(value: unknown, viewKind: string, sourceCount: number): boolean {
  if (!value || typeof value !== "object") return false;
  const scene = value as Record<string, unknown>;
  const expected = viewKind.replace(/-view$/, "");
  if (scene.kind !== expected) return false;
  if (scene.kind !== "state-transition") return true;
  const frame = scene.frame as Record<string, unknown> | null | undefined;
  const vertices = scene.vertices;
  const transitions = scene.transitions;
  if (frame !== null && (!frame || typeof frame.id !== "string" || typeof frame.label !== "string" ||
      !indexIn(frame.navigation, sourceCount))) return false;
  if (!Array.isArray(vertices) || !Array.isArray(transitions)) return false;
  if (!vertices.every((raw) => {
    if (!raw || typeof raw !== "object") return false;
    const vertex = raw as Record<string, unknown>;
    return typeof vertex.id === "string" && typeof vertex.label === "string" &&
      ["initial", "state", "final"].includes(String(vertex.kind)) && indexIn(vertex.navigation, sourceCount);
  })) return false;
  return transitions.every((raw) => {
    if (!raw || typeof raw !== "object") return false;
    const transition = raw as Record<string, unknown>;
    const trigger = transition.trigger as Record<string, unknown> | undefined;
    const guard = transition.guard as Record<string, unknown> | undefined;
    const effect = transition.effect as Record<string, unknown> | undefined;
    return typeof transition.id === "string" && (transition.label === null || typeof transition.label === "string") &&
      indexIn(transition.source, vertices.length) && indexIn(transition.target, vertices.length) &&
      indexIn(transition.navigation, sourceCount) && ["authored", "implied"].includes(String(transition.provenance)) &&
      isSceneTrigger(trigger, sourceCount) && isSceneFeature(guard, sourceCount) && isSceneFeature(effect, sourceCount);
  });
}

function isSceneTrigger(value: Record<string, unknown> | undefined, sourceCount: number): boolean {
  if (!value) return false;
  if (["absent", "unresolved", "ambiguous"].includes(String(value.status))) return true;
  if (value.status === "unsupported") return typeof value.code === "string" && typeof value.message === "string";
  return value.status === "accept" && typeof value.label === "string" && indexIn(value.navigation, sourceCount);
}

function isSceneFeature(value: Record<string, unknown> | undefined, sourceCount: number): boolean {
  if (!value) return false;
  if (["absent", "unresolved", "ambiguous", "recovery"].includes(String(value.status))) return true;
  if (value.status === "unsupported") return typeof value.code === "string" && typeof value.message === "string";
  return value.status === "supported" && typeof value.label === "string" && indexIn(value.navigation, sourceCount);
}

function indexIn(value: unknown, length: number): value is number {
  return nonNegativeInteger(value) && value < length;
}

function isDiagramDocument(value: unknown): boolean {
  if (!value || typeof value !== "object") return false;
  const document = value as Record<string, unknown>;
  return typeof document.uri === "string" && typeof document.sourceDomain === "string";
}

function isDiagramSource(value: unknown, documentCount: number): boolean {
  if (!value || typeof value !== "object") return false;
  const source = value as Record<string, unknown>;
  return indexIn(source.document, documentCount) && Array.isArray(source.range) && source.range.length === 4 &&
    source.range.every(nonNegativeInteger) && (source.range[2] > source.range[0] ||
      (source.range[2] === source.range[0] && source.range[3] >= source.range[1]));
}

function isProductReference(value: unknown, documentCount: number, sourceCount: number, referenceCount: number): boolean {
  if (!value || typeof value !== "object") return false;
  const reference = value as Record<string, unknown>;
  if (reference.kind === "qualified-name") return indexIn(reference.document, documentCount) && typeof reference.qualifiedName === "string";
  if (reference.kind === "tooling-element-id") return typeof reference.elementId === "string" && typeof reference.sourceDomain === "string";
  if (reference.kind === "source-anchor") return indexIn(reference.source, sourceCount) && typeof reference.metaclass === "string" &&
    (reference.ownerQualifiedName === null || typeof reference.ownerQualifiedName === "string");
  return reference.kind === "relationship" && indexIn(reference.source, referenceCount) &&
    typeof reference.relationshipKind === "string" && nonNegativeInteger(reference.ordinal);
}

function isDiagramTyping(value: unknown, referenceCount: number): boolean {
  if (!value || typeof value !== "object") return false;
  const typing = value as Record<string, unknown>;
  if (["absent", "unresolved", "unsupported", "recovery", "incomplete"].includes(String(typing.status))) return true;
  if (typing.status === "ambiguous") {
    return Array.isArray(typing.candidates) && typing.candidates.every((candidate) => indexIn(candidate, referenceCount));
  }
  if (typing.status !== "resolved" && typing.status !== "partial") return false;
  return Array.isArray(typing.types) &&
    (typing.status === "partial" || typing.types.length > 0) && typing.types.every((raw) => {
    if (!raw || typeof raw !== "object") return false;
    const type = raw as Record<string, unknown>;
    return indexIn(type.reference, referenceCount) && typeof type.label === "string";
  });
}

function isDiagramNode(value: unknown, referenceCount: number, sourceCount: number, nodeCount: number): boolean {
  if (!value || typeof value !== "object") return false;
  const node = value as Record<string, unknown>;
  const compartments = node.compartments;
  return typeof node.presentationKey === "string" && node.presentationKey.length > 0 &&
    indexIn(node.reference, referenceCount) && indexIn(node.source, sourceCount) && typeof node.metaclass === "string" &&
    typeof node.notationRole === "string" && NOTATION_ROLES.has(node.notationRole) &&
    (node.name === null || typeof node.name === "string") && (node.owner === null || indexIn(node.owner, nodeCount)) &&
    isDiagramTyping(node.typing, referenceCount) &&
    Array.isArray(compartments) && compartments.every((raw) => {
      if (!raw || typeof raw !== "object") return false;
      const compartment = raw as Record<string, unknown>;
      return typeof compartment.kind === "string" && ["direct", "inherited"].includes(String(compartment.provenance)) &&
        Array.isArray(compartment.members) && compartment.members.every((member) => indexIn(member, nodeCount));
    });
}

function isDiagramEdge(value: unknown, referenceCount: number, sourceCount: number, nodeCount: number): boolean {
  if (!value || typeof value !== "object") return false;
  const edge = value as Record<string, unknown>;
  return indexIn(edge.reference, referenceCount) && indexIn(edge.source, nodeCount) && indexIn(edge.target, nodeCount) &&
    typeof edge.kind === "string" && typeof edge.provenance === "string" &&
    (edge.navigation === null || indexIn(edge.navigation, sourceCount));
}

function isDiagramRelationship(value: unknown, referenceCount: number, sourceCount: number, nodeCount: number): boolean {
  if (!value || typeof value !== "object") return false;
  const relationship = value as Record<string, unknown>;
  return indexIn(relationship.reference, referenceCount) && indexIn(relationship.source, nodeCount) &&
    typeof relationship.kind === "string" && typeof relationship.provenance === "string" &&
    (relationship.navigation === null || indexIn(relationship.navigation, sourceCount)) &&
    !!relationship.target && typeof relationship.target === "object";
}

export function isDiagramSemanticReference(value: unknown): value is DiagramSemanticReference {
  if (!value || typeof value !== "object") return false;
  const reference = value as Record<string, unknown>;
  if (typeof reference.kind !== "string" || typeof reference.sourceDomain !== "string") return false;
  if (reference.kind === "qualified-name") {
    return typeof reference.document === "string" && typeof reference.qualifiedName === "string";
  }
  if (reference.kind === "tooling-element-id") return typeof reference.elementId === "string";
  if (reference.kind === "source-anchor") {
    return typeof reference.document === "string" && typeof reference.metaclass === "string" &&
      (reference.ownerQualifiedName === null || typeof reference.ownerQualifiedName === "string") &&
      !!reference.range && typeof reference.range === "object";
  }
  return reference.kind === "relationship" && typeof reference.document === "string" &&
    typeof reference.sourceQualifiedName === "string" && typeof reference.relationshipKind === "string" &&
    nonNegativeInteger(reference.ordinal);
}

function nonNegativeInteger(value: unknown): value is number {
  return typeof value === "number" && Number.isSafeInteger(value) && value >= 0;
}

export function parseSourceNavigation(value: unknown): SourceNavigation | undefined {
  if (!value || typeof value !== "object") return undefined;
  const candidate = value as Record<string, unknown>;
  if (
    typeof candidate.uri !== "string" ||
    !nonNegativeInteger(candidate.startLine) ||
    !nonNegativeInteger(candidate.startCharacter) ||
    !nonNegativeInteger(candidate.endLine) ||
    !nonNegativeInteger(candidate.endCharacter)
  ) return undefined;
  if (
    candidate.endLine < candidate.startLine ||
    (candidate.endLine === candidate.startLine && candidate.endCharacter < candidate.startCharacter)
  ) return undefined;
  return candidate as SourceNavigation;
}

export function isPathInsideWorkspace(filePath: string, workspaceRoots: string[]): boolean {
  const resolved = path.resolve(filePath);
  return workspaceRoots.some((root) => {
    const relative = path.relative(path.resolve(root), resolved);
    return relative === "" || (!relative.startsWith(`..${path.sep}`) && relative !== "..");
  });
}

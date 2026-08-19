import * as path from "path";

export type GenerationReport = {
  status: string;
  model_digest: string;
};

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

export type DiagramProduct = {
  schemaVersion: 2;
  modelDigest: string;
  documents: Array<{ uri: string; sourceDomain: string }>;
  sources: Array<{ document: number; range: [number, number, number, number] }>;
  references: Array<Record<string, unknown>>;
  selectedView: { reference: number; kind: DiagramViewId; name: string; source: number };
  completeness: {
    status: "complete" | "incomplete";
    reasons: Array<{ code: string; [key: string]: unknown }>;
  };
  projection: {
    kind: DiagramViewId;
    exposedRoots: number[];
    nodes: unknown[];
    relationships: unknown[];
    edges: unknown[];
    metadata: Record<string, unknown>;
  };
};

const NOTATION_ROLES = new Set([
  "definition", "usage", "reference-usage", "namespace", "annotation", "unsupported",
]);

export type LspGenerationResult = {
  modelDigest: string;
  generatorDigest: string;
  artifacts: Array<{ path: string; content: number[] }>;
  timings: {
    modulePrepareMs: number;
    guestExecutionUs: number;
    preparedReused: boolean;
    compilationCacheEnabled: boolean;
    compilationCacheHits: number;
    compilationCacheMisses: number;
    compilationCacheError: string | null;
  };
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
    source: { uri: string };
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
    return { handle: view.handle, kind: view.kind as DiagramViewId, reference: view.reference, name: view.name, source: { uri: source.uri } };
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

export function parseLspGenerationResult(value: unknown): LspGenerationResult {
  if (!value || typeof value !== "object") throw new Error("Spec42 returned an invalid generation result.");
  const candidate = value as Record<string, unknown>;
  if (typeof candidate.modelDigest !== "string" || typeof candidate.generatorDigest !== "string" || !Array.isArray(candidate.artifacts)) {
    throw new Error("Spec42 generation result is missing identity or artifacts.");
  }
  const artifacts = candidate.artifacts.map((entry) => {
    if (!entry || typeof entry !== "object") throw new Error("Spec42 returned an invalid artifact.");
    const artifact = entry as Record<string, unknown>;
    if (typeof artifact.path !== "string" || !Array.isArray(artifact.content) ||
        artifact.content.some((byte) => !Number.isInteger(byte) || byte < 0 || byte > 255)) {
      throw new Error("Spec42 returned malformed artifact bytes.");
    }
    return { path: artifact.path, content: artifact.content as number[] };
  });
  const timings = candidate.timings as Record<string, unknown> | undefined;
  if (!timings || typeof timings.modulePrepareMs !== "number" || typeof timings.guestExecutionUs !== "number" ||
      typeof timings.preparedReused !== "boolean" || typeof timings.compilationCacheEnabled !== "boolean" ||
      typeof timings.compilationCacheHits !== "number" || typeof timings.compilationCacheMisses !== "number" ||
      (timings.compilationCacheError !== null && typeof timings.compilationCacheError !== "string")) {
    throw new Error("Spec42 generation result is missing timing information.");
  }
  return {
    modelDigest: candidate.modelDigest,
    generatorDigest: candidate.generatorDigest,
    artifacts,
    timings: timings as LspGenerationResult["timings"],
  };
}

export function buildGenerateArgv(
  pluginPath: string,
  modelPath: string,
  outputPath: string,
  workspaceRoot: string,
  libraryPaths: string[]
): string[] {
  const args = [
    "generate",
    pluginPath,
    modelPath,
    "--output",
    outputPath,
    "--format",
    "json",
    "--timeout-seconds",
    "30",
    "--max-files",
    "16",
    "--max-total-bytes",
    String(16 * 1024 * 1024),
  ];
  if (workspaceRoot) {
    args.push("--workspace-root", workspaceRoot);
  }
  for (const libraryPath of libraryPaths) {
    args.push("--library-path", libraryPath);
  }
  return args;
}

export function parseGenerationReport(value: unknown): GenerationReport {
  if (!value || typeof value !== "object") {
    throw new Error("Spec42 returned an invalid generation report.");
  }
  const candidate = value as Record<string, unknown>;
  if (typeof candidate.status !== "string" || typeof candidate.model_digest !== "string") {
    throw new Error("Spec42 generation report is missing status or model_digest.");
  }
  return { status: candidate.status, model_digest: candidate.model_digest };
}

export function selectSingleDiagramJson(paths: string[]): string {
  const jsonPaths = paths.filter((candidate) => path.extname(candidate).toLowerCase() === ".json" && !candidate.startsWith(".spec42-"));
  if (jsonPaths.length !== 1) {
    throw new Error(`Expected exactly one diagram JSON artifact, but generator produced ${jsonPaths.length}.`);
  }
  return jsonPaths[0];
}

export function parseDiagramProduct(text: string): DiagramProduct {
  if (Buffer.byteLength(text, "utf8") > 16 * 1024 * 1024) {
    throw new Error("Generated diagram product exceeds the 16 MiB viewer limit.");
  }
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
  if (product.schemaVersion !== 2 || typeof product.modelDigest !== "string" ||
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
      !Array.isArray(projection.edges) || !projection.metadata || typeof projection.metadata !== "object") {
    throw new Error("Generated diagram product does not match schema version 2.");
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
  }
  if (!(projection.edges as unknown[]).every((edge) => isDiagramEdge(edge, references.length, sources.length, (projection.nodes as unknown[]).length)) ||
      !(projection.relationships as unknown[]).every((relationship) => isDiagramRelationship(relationship, references.length, sources.length, (projection.nodes as unknown[]).length))) {
    throw new Error("Generated diagram product contains an invalid graph index.");
  }
  return value as DiagramProduct;
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

function isDiagramNode(value: unknown, referenceCount: number, sourceCount: number, nodeCount: number): boolean {
  if (!value || typeof value !== "object") return false;
  const node = value as Record<string, unknown>;
  return indexIn(node.reference, referenceCount) && indexIn(node.source, sourceCount) && typeof node.metaclass === "string" &&
    typeof node.notationRole === "string" && NOTATION_ROLES.has(node.notationRole) &&
    (node.name === null || typeof node.name === "string") && (node.owner === null || indexIn(node.owner, nodeCount));
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

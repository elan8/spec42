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

export const DIAGRAM_VIEWS = [
  { id: "general-view", label: "General View", queryStatus: "stubbed" },
  { id: "interconnection-view", label: "Interconnection View", queryStatus: "stubbed" },
  { id: "action-flow-view", label: "Action Flow View", queryStatus: "stubbed" },
  { id: "state-transition-view", label: "State Transition View", queryStatus: "implemented" },
  { id: "sequence-view", label: "Sequence View", queryStatus: "stubbed" },
  { id: "browser-view", label: "Browser View", queryStatus: "stubbed" },
  { id: "grid-view", label: "Grid View", queryStatus: "stubbed" },
  { id: "geometry-view", label: "Geometry View", queryStatus: "stubbed" },
] as const;

export type DiagramViewId = typeof DIAGRAM_VIEWS[number]["id"];

export type DiagramProduct = {
  schemaVersion: 1;
  modelDigest: string;
  view: { id: DiagramViewId; name: string };
  completeness: {
    status: "complete" | "incomplete";
    reasons: Array<{ code: string; message: string; requiredQuery: string }>;
  };
  preparedView: {
    title: string;
    view: DiagramViewId;
    nodes: unknown[];
    edges: unknown[];
    meta?: Record<string, unknown>;
  };
};

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
    kind: DiagramViewId;
    semanticId: string;
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
    if (!DIAGRAM_VIEWS.some((known) => known.id === view.kind) || typeof view.semanticId !== "string" ||
        typeof view.name !== "string" || !source || typeof source.uri !== "string") {
      throw new Error("Spec42 returned malformed diagram view identity.");
    }
    return { kind: view.kind as DiagramViewId, semanticId: view.semanticId, name: view.name, source: { uri: source.uri } };
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
  const view = product.view as Record<string, unknown> | undefined;
  const completeness = product.completeness as Record<string, unknown> | undefined;
  const prepared = product.preparedView as Record<string, unknown> | undefined;
  if (product.schemaVersion !== 1 || typeof product.modelDigest !== "string" ||
      !view || !DIAGRAM_VIEWS.some((candidate) => candidate.id === view.id) || typeof view.name !== "string" ||
      !completeness || !["complete", "incomplete"].includes(String(completeness.status)) || !Array.isArray(completeness.reasons) ||
      !prepared || prepared.view !== view.id || typeof prepared.title !== "string" ||
      !Array.isArray(prepared.nodes) || !Array.isArray(prepared.edges)) {
    throw new Error("Generated diagram product does not match schema version 1.");
  }
  for (const reason of completeness.reasons) {
    if (!reason || typeof reason !== "object") throw new Error("Generated diagram product has a malformed completeness reason.");
    const candidate = reason as Record<string, unknown>;
    if (typeof candidate.code !== "string" || typeof candidate.message !== "string" || typeof candidate.requiredQuery !== "string") {
      throw new Error("Generated diagram product has a malformed completeness reason.");
    }
  }
  return value as DiagramProduct;
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

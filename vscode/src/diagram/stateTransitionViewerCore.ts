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

export type SvgMetadata = { modelDigest: string; viewName: string };

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

export function selectSingleSvg(paths: string[]): string {
  const svgPaths = paths.filter((candidate) => path.extname(candidate).toLowerCase() === ".svg");
  if (svgPaths.length !== 1) {
    throw new Error(`Expected exactly one SVG artifact, but generator produced ${svgPaths.length}.`);
  }
  return svgPaths[0];
}

/** SVG is inserted into a privileged webview document, so reject active or external content. */
export function validateStandaloneSvg(svg: string): string {
  if (Buffer.byteLength(svg, "utf8") > 16 * 1024 * 1024) {
    throw new Error("Generated SVG exceeds the 16 MiB viewer limit.");
  }
  if (!/^\s*(?:<\?xml[^>]*>\s*)?<svg\b/i.test(svg)) {
    throw new Error("Generated artifact is not a standalone SVG document.");
  }
  const forbidden = [
    /<!\s*DOCTYPE\b/i,
    /<\s*script\b/i,
    /<\s*(?:foreignObject|iframe|object|embed|audio|video)\b/i,
    /\bon[a-z]+\s*=/i,
    /\bsrc\s*=/i,
    /\bhref\s*=\s*["'](?!#)/i,
    /@import\b/i,
    /url\s*\(\s*["']?\s*(?:https?:|data:|javascript:|file:|\/\/)/i,
  ];
  if (forbidden.some((pattern) => pattern.test(svg))) {
    throw new Error("Generated SVG contains active or external content and was not displayed.");
  }
  return svg;
}

export function readSvgMetadata(svg: string): SvgMetadata {
  const root = svg.match(/^\s*(?:<\?xml[^>]*>\s*)?<svg\b([^>]*)>/i)?.[1];
  const attribute = (name: string) => root?.match(new RegExp(`\\b${name}\\s*=\\s*["']([^"']+)["']`, "i"))?.[1];
  const modelDigest = attribute("data-model-digest");
  const viewName = attribute("data-view-name");
  if (!modelDigest || !viewName) {
    throw new Error("Generated SVG is missing data-model-digest or data-view-name provenance.");
  }
  return { modelDigest, viewName };
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

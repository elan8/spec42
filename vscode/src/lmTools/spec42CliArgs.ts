export type Spec42CliContext = {
  serverCommand: string;
  workspaceRoot: string;
  libraryPaths: string[];
};

export function appendLibraryPathArgs(args: string[], libraryPaths: string[]): string[] {
  const out = [...args];
  for (const lib of libraryPaths) {
    out.push("--library-path", lib);
  }
  return out;
}

export function buildCheckArgv(
  ctx: Spec42CliContext,
  targetPath: string,
  workspaceRoot?: string
): string[] {
  const args = appendLibraryPathArgs(
    ["check", targetPath, "--format", "json"],
    ctx.libraryPaths
  );
  const root = workspaceRoot ?? ctx.workspaceRoot;
  if (root) {
    args.push("--workspace-root", root);
  }
  return args;
}

export function buildDoctorArgv(ctx: Spec42CliContext): string[] {
  return appendLibraryPathArgs(["doctor", "--format", "json"], ctx.libraryPaths);
}

export function buildModelSummaryArgv(
  ctx: Spec42CliContext,
  targetPath: string,
  maxNodes: number,
  workspaceRoot?: string
): string[] {
  const args = appendLibraryPathArgs(
    ["model-summary", targetPath, "--format", "json", "--max-nodes", String(maxNodes)],
    ctx.libraryPaths
  );
  const root = workspaceRoot ?? ctx.workspaceRoot;
  if (root) {
    args.push("--workspace-root", root);
  }
  return args;
}

export function buildExplainDiagnosticArgv(
  ctx: Spec42CliContext,
  code: string,
  options?: { path?: string; line?: number; workspaceRoot?: string }
): string[] {
  const args = appendLibraryPathArgs(
    ["explain-diagnostic", "--code", code, "--format", "json"],
    ctx.libraryPaths
  );
  if (options?.path) {
    args.push("--path", options.path);
  }
  if (options?.line !== undefined) {
    args.push("--line", String(options.line));
  }
  const root = options?.workspaceRoot ?? ctx.workspaceRoot;
  if (root) {
    args.push("--workspace-root", root);
  }
  return args;
}

import * as path from "path";
import * as vscode from "vscode";

export type WorkspaceDiagnosticsSummary = {
  errors: number;
  warnings: number;
  filesWithIssues: number;
  totalFiles: number;
};

export type SummarizeDiagnosticsOptions = {
  workspaceRootPaths?: string[];
  libraryRootPaths?: string[];
};

type DiagnosticLike = {
  severity?: vscode.DiagnosticSeverity;
};

function normalizeRootPath(rootPath: string): string {
  const normalized = path.normalize(rootPath).replace(/\\/g, "/");
  return normalized.endsWith("/") ? normalized : `${normalized}/`;
}

function isSysmlOrKermlFile(filePath: string): boolean {
  const lower = filePath.toLowerCase();
  return lower.endsWith(".sysml") || lower.endsWith(".kerml");
}

function isUnderRoot(filePath: string, rootPath: string): boolean {
  const normalizedFile = path.normalize(filePath).replace(/\\/g, "/");
  const normalizedRoot = normalizeRootPath(rootPath);
  const lowerFile = normalizedFile.toLowerCase();
  const lowerRoot = normalizedRoot.toLowerCase();
  return lowerFile.startsWith(lowerRoot);
}

function isUnderAnyRoot(filePath: string, roots: string[]): boolean {
  return roots.some((root) => isUnderRoot(filePath, root));
}

export function summarizeSysmlDiagnosticEntries(
  entries: ReadonlyArray<readonly [vscode.Uri, readonly DiagnosticLike[]]>,
  options: SummarizeDiagnosticsOptions
): WorkspaceDiagnosticsSummary {
  const workspaceRoots = (options.workspaceRootPaths ?? []).filter((root) => root.length > 0);
  const libraryRoots = (options.libraryRootPaths ?? []).filter((root) => root.length > 0);

  let errors = 0;
  let warnings = 0;
  let filesWithIssues = 0;
  let totalFiles = 0;

  for (const [uri, diagnostics] of entries) {
    const filePath = uri.fsPath;
    if (!isSysmlOrKermlFile(filePath)) {
      continue;
    }
    if (workspaceRoots.length > 0 && !isUnderAnyRoot(filePath, workspaceRoots)) {
      continue;
    }
    if (libraryRoots.length > 0 && isUnderAnyRoot(filePath, libraryRoots)) {
      continue;
    }

    totalFiles += 1;
    let fileErrors = 0;
    let fileWarnings = 0;
    for (const diagnostic of diagnostics) {
      if (diagnostic.severity === vscode.DiagnosticSeverity.Error) {
        fileErrors += 1;
      } else if (diagnostic.severity === vscode.DiagnosticSeverity.Warning) {
        fileWarnings += 1;
      }
    }
    errors += fileErrors;
    warnings += fileWarnings;
    if (fileErrors > 0 || fileWarnings > 0) {
      filesWithIssues += 1;
    }
  }

  return { errors, warnings, filesWithIssues, totalFiles };
}

export function summarizeWorkspaceSysmlDiagnostics(
  options?: SummarizeDiagnosticsOptions
): WorkspaceDiagnosticsSummary {
  const workspaceRoots =
    options?.workspaceRootPaths ??
    (vscode.workspace.workspaceFolders ?? []).map((folder) => folder.uri.fsPath);
  const libraryRoots = options?.libraryRootPaths;
  const allDiagnostics = vscode.languages.getDiagnostics();
  return summarizeSysmlDiagnosticEntries(allDiagnostics, {
    workspaceRootPaths: workspaceRoots,
    libraryRootPaths: libraryRoots,
  });
}

export function summarizeActiveFileSysmlDiagnostics(
  doc: vscode.TextDocument
): WorkspaceDiagnosticsSummary {
  const diagnostics = vscode.languages.getDiagnostics(doc.uri);
  let errors = 0;
  let warnings = 0;
  for (const diagnostic of diagnostics) {
    if (diagnostic.severity === vscode.DiagnosticSeverity.Error) {
      errors += 1;
    } else if (diagnostic.severity === vscode.DiagnosticSeverity.Warning) {
      warnings += 1;
    }
  }
  return {
    errors,
    warnings,
    filesWithIssues: errors > 0 || warnings > 0 ? 1 : 0,
    totalFiles: 1,
  };
}

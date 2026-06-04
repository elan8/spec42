import * as cp from "child_process";
import * as path from "path";
import * as vscode from "vscode";

export type { Spec42CliContext } from "./spec42CliArgs";
export {
  appendLibraryPathArgs,
  buildCheckArgv,
  buildDoctorArgv,
  buildExplainDiagnosticArgv,
  buildModelSummaryArgv,
} from "./spec42CliArgs";

export function runSpec42Json(
  command: string,
  args: string[],
  cwd: string
): Promise<unknown> {
  return new Promise((resolve, reject) => {
    cp.execFile(command, args, { cwd: cwd || undefined, maxBuffer: 16 * 1024 * 1024 }, (error, stdout, stderr) => {
      if (error) {
        reject(new Error(stderr?.trim() || error.message));
        return;
      }
      try {
        resolve(JSON.parse(stdout));
      } catch (parseError) {
        reject(parseError instanceof Error ? parseError : new Error(String(parseError)));
      }
    });
  });
}

export function defaultSysmlTargetPath(workspaceRoot: string): string | undefined {
  const active = vscode.window.activeTextEditor?.document;
  if (active && (active.languageId === "sysml" || active.languageId === "kerml")) {
    return active.uri.fsPath;
  }
  const folder = vscode.workspace.workspaceFolders?.[0];
  if (folder) {
    return folder.uri.fsPath;
  }
  if (workspaceRoot) {
    return workspaceRoot;
  }
  return undefined;
}

export function resolveTargetPath(
  inputPath: string | undefined,
  workspaceRoot: string
): string {
  if (inputPath) {
    return path.isAbsolute(inputPath)
      ? inputPath
      : path.resolve(workspaceRoot || process.cwd(), inputPath);
  }
  const fallback = defaultSysmlTargetPath(workspaceRoot);
  if (!fallback) {
    throw new Error("No SysML/KerML file or workspace folder is available.");
  }
  return fallback;
}

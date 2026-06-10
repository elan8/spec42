import * as path from "path";
import * as vscode from "vscode";
import {
  summarizeActiveFileSysmlDiagnostics,
  summarizeWorkspaceSysmlDiagnostics,
} from "../diagnostics/workspaceDiagnostics";
import { hasWorkspaceFolder } from "../providers/lspModelProvider";
import type { LspModelProvider } from "../providers/lspModelProvider";
import { setVisualizationGateState } from "../visualization/visualizationGate";
import { log } from "../logger";
import {
  formatSpec42StatusBar,
  ServerHealthState,
} from "../statusBar/statusBarViewModel";
import {
  getConfigBoolean,
  getConfigStringArray,
  isSysmlDoc,
} from "./configBridge";

export type WorkspaceIndexSummary = {
  scannedFiles: number;
  loadedFiles: number;
  truncated: boolean;
  cancelled: boolean;
  failures?: number;
};

let statusItem: vscode.StatusBarItem | undefined;
let lspModelProviderForStatus: LspModelProvider | undefined;
let serverHealthState: ServerHealthState = "starting";
let serverHealthDetail = "";
let lastWorkspaceIndexSummary: WorkspaceIndexSummary | undefined;

export function getServerHealthState(): ServerHealthState {
  return serverHealthState;
}

export function getServerHealthDetail(): string {
  return serverHealthDetail;
}

export function getWorkspaceIndexSummary(): WorkspaceIndexSummary | undefined {
  return lastWorkspaceIndexSummary;
}

export function setWorkspaceIndexSummary(
  summary: WorkspaceIndexSummary | undefined
): void {
  lastWorkspaceIndexSummary = summary;
}

export function setLspModelProviderForStatus(
  provider: LspModelProvider | undefined
): void {
  lspModelProviderForStatus = provider;
}

function ensureStatusItem(context: vscode.ExtensionContext): vscode.StatusBarItem {
  if (!statusItem) {
    statusItem = vscode.window.createStatusBarItem(
      vscode.StatusBarAlignment.Right,
      100
    );
    statusItem.name = "SysML Diagnostics";
    statusItem.command = "spec42.status.showActions";
    context.subscriptions.push(statusItem);
  }
  return statusItem;
}

function getResolvedLibraryPaths(): string[] {
  const libraryPathsRaw = getConfigStringArray("libraryPaths") ?? [];
  const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath ?? "";
  const customLibraryPaths = libraryPathsRaw.map((p) =>
    path.isAbsolute(p) ? p : path.resolve(workspaceRoot, p)
  );
  return customLibraryPaths.filter(
    (value, index, all) => all.indexOf(value) === index
  );
}

export function setServerHealth(
  context: vscode.ExtensionContext,
  state: ServerHealthState,
  detail = ""
): void {
  serverHealthState = state;
  serverHealthDetail = detail;
  setVisualizationGateState({ serverHealthState: state });
  log("Server health:", state, detail);
  updateStatusBar(context);
}

export function updateStatusBar(context: vscode.ExtensionContext): void {
  const enabled = getConfigBoolean("statusBar.enabled", true);
  if (!enabled) {
    statusItem?.hide();
    return;
  }

  const editor = vscode.window.activeTextEditor;
  const doc = editor?.document;
  const showHealthWithoutDoc = serverHealthState !== "ready";
  const useWorkspaceDiagnostics =
    hasWorkspaceFolder() && serverHealthState === "ready";
  if ((!doc || !isSysmlDoc(doc)) && !showHealthWithoutDoc && !useWorkspaceDiagnostics) {
    statusItem?.hide();
    return;
  }

  const item = ensureStatusItem(context);
  const workspaceSummary = useWorkspaceDiagnostics
    ? summarizeWorkspaceSysmlDiagnostics({
        libraryRootPaths: getResolvedLibraryPaths(),
      })
    : undefined;
  const activeSummary =
    doc && isSysmlDoc(doc) ? summarizeActiveFileSysmlDiagnostics(doc) : undefined;
  const errors = workspaceSummary?.errors ?? activeSummary?.errors ?? 0;
  const warnings = workspaceSummary?.warnings ?? activeSummary?.warnings ?? 0;
  const status = formatSpec42StatusBar(
    serverHealthState,
    serverHealthDetail,
    errors,
    warnings,
    workspaceSummary,
    activeSummary
  );
  item.text = status.text;
  const baseTooltip = status.baseTooltip;
  const workspaceTooltip = lastWorkspaceIndexSummary
    ? `\n\nWorkspace indexing:\nScanned ${lastWorkspaceIndexSummary.scannedFiles} file(s)\nLoaded ${lastWorkspaceIndexSummary.loadedFiles} file(s)${(lastWorkspaceIndexSummary.failures ?? 0) > 0 ? `\nFailures: ${lastWorkspaceIndexSummary.failures}` : ""}${lastWorkspaceIndexSummary.truncated ? "\nResults may be incomplete." : ""}${lastWorkspaceIndexSummary.cancelled ? "\nLast scan was cancelled." : ""}`
    : "";
  item.tooltip = `${baseTooltip}${workspaceTooltip}`;
  item.show();

  const provider = lspModelProviderForStatus;
  if (provider) {
    provider
      .getServerStats()
      .then((stats) => {
        if (!stats || !statusItem) return;
        const uptimeStr =
          stats.uptime >= 60
            ? `${Math.floor(stats.uptime / 60)}m ${stats.uptime % 60}s`
            : `${stats.uptime}s`;
        const caches = stats.caches;
        item.tooltip = `${baseTooltip}${workspaceTooltip}\n\n-- LSP Server --\nUptime: ${uptimeStr}\nCaches: ${caches.documents} docs, ${caches.symbolTables} symbols`;
      })
      .catch(() => {});
  }
}

async function showSpec42StatusActions(): Promise<void> {
  const selected = await vscode.window.showQuickPick(
    [
      { label: "$(issues) Open Problems", command: "workbench.actions.view.problems" },
      { label: "$(list-tree) Show Model Explorer", command: "sysml.showModelExplorer" },
      { label: "$(graph) Open Visualizer", command: "sysml.showVisualizer" },
      { label: "$(star-full) Open Recommended Example", command: "spec42.examples.openRecommended" },
      { label: "$(output) Show SysML Output", command: "sysml.showOutput" },
      { label: "$(debug-restart) Restart Server", command: "sysml.restartServer" },
    ],
    { placeHolder: "Spec42 actions" }
  );
  if (selected) {
    await vscode.commands.executeCommand(selected.command);
  }
}

export function registerStatusBar(context: vscode.ExtensionContext): void {
  context.subscriptions.push(
    vscode.commands.registerCommand("spec42.status.showActions", async () => {
      await showSpec42StatusActions();
    })
  );
  context.subscriptions.push(
    vscode.languages.onDidChangeDiagnostics(() => updateStatusBar(context))
  );
  context.subscriptions.push(
    vscode.workspace.onDidChangeConfiguration((event) => {
      if (
        event.affectsConfiguration("spec42.statusBar.enabled") ||
        event.affectsConfiguration("sysml-language-server.statusBar.enabled")
      ) {
        updateStatusBar(context);
      }
    })
  );
}

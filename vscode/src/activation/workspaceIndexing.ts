import * as path from "path";
import * as vscode from "vscode";
import { NotificationType, State } from "vscode-languageclient/node";
import { log, logError, logPerfEvent } from "../logger";
import {
  graphScopesForContext,
  hasWorkspaceFolder,
  isClientNotRunningError,
} from "../providers/lspModelProvider";
import type { LspModelProvider } from "../providers/lspModelProvider";
import type { GraphNodeDTO, SemanticIndexReadyParams } from "../providers/sysmlModelTypes";
import {
  ModelExplorerProvider,
} from "../explorer/modelExplorerProvider";
import { getLastVisualizerRender } from "../visualization/renderTracker";
import { VisualizationPanel } from "../visualization/visualizationPanel";
import { rangeContainsPosition, rangeSpanScore } from "../utils/range";
import {
  activeSysmlDocument,
  getStartupWorkspaceIndexingMode,
  isSysmlDoc,
} from "./configBridge";
import {
  getLanguageClient,
  isLanguageClientReady,
  type LspClientHandles,
} from "./lspClient";
import {
  getServerHealthDetail,
  getServerHealthState,
  getWorkspaceIndexSummary,
  setServerHealth,
  setWorkspaceIndexSummary,
  updateStatusBar,
} from "./statusBar";
import {
  notifyWorkspaceLifecycleChanged,
  onWorkspaceLifecycleChanged,
  registerWorkspaceLifecycleSnapshotProvider,
  type WorkspaceLifecycleInput,
} from "./workspaceLifecycle";

type WorkspaceLoadRun = {
  runId: string;
  cts: vscode.CancellationTokenSource;
  targetViewMode: "bySemantic" | "byFile";
  discoveredFiles: vscode.Uri[];
  startedAt: number;
};

let extensionContext: vscode.ExtensionContext | undefined;
let modelExplorerProvider: ModelExplorerProvider | undefined;
let lspModelProvider: LspModelProvider | undefined;
let logStartupPhaseFn: ((phase: string, extra?: Record<string, unknown>) => void) | undefined;
let logPerfFn: ((event: string, extra?: Record<string, unknown>) => void) | undefined;
let sourceSelectionSyncTimer: ReturnType<typeof setTimeout> | undefined;
let modelExplorerSelectionSyncTimer: ReturnType<typeof setTimeout> | undefined;
let activeDocumentExplorerRefreshTimer: ReturnType<typeof setTimeout> | undefined;
let activeDocumentExplorerRefreshUri: string | undefined;
let activeDocumentExplorerRefreshGuardUntil = 0;
let lastLoadedSemanticStateVersion: number | undefined;
let lastSemanticIndexReadyWorkspaceFileCount: number | undefined;
let activeWorkspaceLoadRun: WorkspaceLoadRun | undefined;
let nextWorkspaceLoadRunId = 0;
let modelExplorerRefreshTimer: ReturnType<typeof setTimeout> | undefined;

function collectWorkspaceLifecycleInput(): WorkspaceLifecycleInput {
  const provider = modelExplorerProvider;
  const loadStatus = provider?.getWorkspaceLoadStatus();
  return {
    languageClientReady: isLanguageClientReady(),
    serverHealthState: getServerHealthState(),
    hasWorkspaceFolder: hasWorkspaceFolder(),
    semanticIndexReady: lastSemanticIndexReadyWorkspaceFileCount !== undefined,
    workspaceLoadState: loadStatus?.state ?? "idle",
    hasWorkspaceData: provider?.hasWorkspaceData() ?? false,
    workspaceLoadFailures: loadStatus?.failures,
    workspaceLoadCancelled: loadStatus?.cancelled,
    workspaceLoadTruncated: loadStatus?.truncated,
  };
}

function refreshWorkspaceLifecycleSurfaces(): void {
  const context = extensionContext;
  if (context) {
    updateStatusBar(context);
  }
  modelExplorerProvider?.refresh();
  VisualizationPanel.currentPanel?.notifyWorkspaceLifecycleChanged();
}

export type DebugExtensionState = {
  serverHealthState: import("../statusBar/statusBarViewModel").ServerHealthState;
  serverHealthDetail: string;
  workspaceIndexSummary?: import("./statusBar").WorkspaceIndexSummary;
  lastLoadedSemanticStateVersion?: number;
  lastSemanticIndexReadyWorkspaceFileCount?: number;
  modelExplorer?: {
    lastRevealedElementId?: string;
    pendingWorkspaceLoadRunId?: string;
  };
  visualizerOpen?: boolean;
  lastVisualizerRender?: {
    view: string;
    outcome: string;
    graphNodes: number;
    hasExportableSvg: boolean;
    timestampMs: number;
  };
};

function graphNodesForDocumentUri(
  nodes: GraphNodeDTO[] | undefined,
  docUri: vscode.Uri
): GraphNodeDTO[] {
  if (!nodes?.length) {
    return [];
  }
  const normalized = docUri.toString().toLowerCase();
  const scoped = nodes.filter((node) => {
    if (!node.uri) {
      return true;
    }
    try {
      return vscode.Uri.parse(node.uri).toString().toLowerCase() === normalized;
    } catch {
      return false;
    }
  });
  return scoped.length > 0 ? scoped : nodes;
}

function bestGraphNodeAtPosition(
  nodes: GraphNodeDTO[] | undefined,
  position: vscode.Position
): GraphNodeDTO | undefined {
  if (!nodes?.length) {
    return undefined;
  }
  return nodes
    .filter((node) => node.range && rangeContainsPosition(node.range, position))
    .sort((a, b) => rangeSpanScore(a.range) - rangeSpanScore(b.range))[0];
}

function nextWorkspaceLoadRunIdString(): string {
  nextWorkspaceLoadRunId += 1;
  return `workspace-load-${nextWorkspaceLoadRunId}`;
}

function isWorkspaceLoadCurrent(run: WorkspaceLoadRun | undefined): boolean {
  return !!run && activeWorkspaceLoadRun?.runId === run.runId;
}

function cancelWorkspaceLoad(
  provider: ModelExplorerProvider | undefined,
  reason: "superseded" | "mode-switch" | "user-cancel" | "deactivate",
  options?: {
    nextStatus?: "idle" | "degraded";
    cancelled?: boolean;
  }
): void {
  const run = activeWorkspaceLoadRun;
  if (!run) {
    return;
  }
  const totalMs = Date.now() - run.startedAt;
  logPerfEvent("workspaceLoad:cancelled", {
    runId: run.runId,
    reason,
    targetViewMode: run.targetViewMode,
    discoveredFiles: run.discoveredFiles.length,
    totalMs,
  });
  run.cts.cancel();
  run.cts.dispose();
  activeWorkspaceLoadRun = undefined;

  if (provider && options?.nextStatus) {
    provider.setWorkspaceLoadStatus({
      state: options.nextStatus,
      scannedFiles: run.discoveredFiles.length,
      loadedFiles:
        options.nextStatus === "idle"
          ? 0
          : provider.getWorkspaceFileUris().length,
      cancelled: options.cancelled ?? reason !== "superseded",
    });
  }
}

function startWorkspaceLoad(
  provider: ModelExplorerProvider,
  targetViewMode: "bySemantic" | "byFile"
): WorkspaceLoadRun {
  cancelWorkspaceLoad(provider, "superseded");
  const run: WorkspaceLoadRun = {
    runId: nextWorkspaceLoadRunIdString(),
    cts: new vscode.CancellationTokenSource(),
    targetViewMode,
    discoveredFiles: [],
    startedAt: Date.now(),
  };
  activeWorkspaceLoadRun = run;
  logPerfEvent("workspaceLoad:started", {
    runId: run.runId,
    targetViewMode,
  });
  return run;
}

function finishWorkspaceLoad(
  run: WorkspaceLoadRun,
  extra?: Record<string, unknown>
): void {
  if (!isWorkspaceLoadCurrent(run)) {
    return;
  }
  logPerfEvent("workspaceLoad:finished", {
    runId: run.runId,
    targetViewMode: run.targetViewMode,
    discoveredFiles: run.discoveredFiles.length,
    totalMs: Date.now() - run.startedAt,
    ...extra,
  });
  run.cts.dispose();
  activeWorkspaceLoadRun = undefined;
}

function scheduleWorkspaceExplorerPending(provider: ModelExplorerProvider): void {
  provider.setWorkspaceLoadStatus({
    state: "pending",
    scannedFiles: 0,
    loadedFiles: 0,
    truncated: false,
    cancelled: false,
    failures: 0,
  });
  refreshWorkspaceLifecycleSurfaces();
}

export function scheduleActiveDocumentExplorerRefresh(
  reason: string,
  doc?: vscode.TextDocument
): void {
  if (hasWorkspaceFolder()) {
    return;
  }
  const targetDoc = activeSysmlDocument(doc);
  if (
    !isLanguageClientReady() ||
    !targetDoc ||
    !modelExplorerProvider ||
    modelExplorerProvider.isWorkspaceBacked()
  ) {
    return;
  }
  const uri = targetDoc.uri.toString();
  const now = Date.now();
  if (
    now < activeDocumentExplorerRefreshGuardUntil &&
    activeDocumentExplorerRefreshUri === uri
  ) {
    log("scheduleActiveDocumentExplorerRefresh: skipped", reason, uri);
    return;
  }
  activeDocumentExplorerRefreshUri = uri;
  if (activeDocumentExplorerRefreshTimer) {
    clearTimeout(activeDocumentExplorerRefreshTimer);
  }
  activeDocumentExplorerRefreshTimer = setTimeout(() => {
    activeDocumentExplorerRefreshTimer = undefined;
    activeDocumentExplorerRefreshGuardUntil = Date.now() + 300;
    modelExplorerProvider
      ?.loadDocument(targetDoc)
      .then(() => {
        log("Active document explorer refresh complete", reason, uri);
      })
      .catch((error) => {
        logError(
          `Active document explorer refresh failed for ${uri} (${reason})`,
          error
        );
      });
  }, 75);
}

export function scheduleModelExplorerRefreshForCurrentMode(
  reason: string,
  doc?: vscode.TextDocument
): void {
  const provider = modelExplorerProvider;
  if (!isLanguageClientReady() || !provider) {
    return;
  }
  if ((vscode.workspace.workspaceFolders?.length ?? 0) > 0) {
    log("scheduleModelExplorerRefreshForCurrentMode: workspace", reason);
    if (lastLoadedSemanticStateVersion !== undefined) {
      void ensureWorkspaceModelLoaded(
        provider,
        extensionContext!,
        logStartupPhaseFn!,
        logPerfFn!
      ).catch((error) => {
        logError(`Workspace explorer refresh failed (${reason})`, error);
      });
    } else {
      scheduleWorkspaceExplorerPending(provider);
    }
    return;
  }
  scheduleActiveDocumentExplorerRefresh(reason, doc);
}

export function resetSemanticIndexTracking(): void {
  lastLoadedSemanticStateVersion = undefined;
  lastSemanticIndexReadyWorkspaceFileCount = undefined;
}

export function shouldShowModelExplorerContext(
  provider: ModelExplorerProvider | undefined
): boolean {
  const hasWorkspace = (vscode.workspace.workspaceFolders?.length ?? 0) > 0;
  const activeIsSysml = isSysmlDoc(vscode.window.activeTextEditor?.document);
  const hasModelData =
    (provider?.getAllElements().length ?? 0) > 0 ||
    (provider?.isWorkspaceBacked() ?? false);
  return hasWorkspace || activeIsSysml || hasModelData;
}

async function loadWorkspaceSysMLFiles(
  context: vscode.ExtensionContext,
  provider: ModelExplorerProvider,
  logStartupPhase: (phase: string, extra?: Record<string, unknown>) => void,
  logPerf: (event: string, extra?: Record<string, unknown>) => void
): Promise<void> {
  const workspaceIndexingStartedAt = Date.now();
  logStartupPhase("workspaceIndexing:start");
  const folders = vscode.workspace.workspaceFolders ?? [];
  if (folders.length === 0) {
    log("loadWorkspaceSysMLFiles: no workspace folders");
    return;
  }
  const run = startWorkspaceLoad(provider, provider.getWorkspaceViewMode());
  const cancellationToken = run.cts.token;
  const activeDocument = activeSysmlDocument();
  const anchorUri = activeDocument?.uri ?? folders[0].uri;
  run.discoveredFiles = [anchorUri];
  setServerHealth(
    context,
    "indexing",
    "Building workspace model from indexed SysML/KerML documents."
  );
  provider.setWorkspaceLoadStatus({
    state: "indexing",
    scannedFiles: 0,
    loadedFiles: 0,
    truncated: false,
    cancelled: false,
    failures: 0,
  });
  notifyWorkspaceLifecycleChanged();
  await vscode.window.withProgress(
    {
      location: vscode.ProgressLocation.Window,
      title: "SysML workspace indexing",
      cancellable: true,
    },
    async (progress, token) => {
      token.onCancellationRequested(() => {
        cancelWorkspaceLoad(provider, "user-cancel", {
          nextStatus: "degraded",
          cancelled: true,
        });
      });
      progress.report({
        message: `Requesting workspace model via ${path.basename(anchorUri.fsPath) || anchorUri.toString()}`,
      });

      if (!isWorkspaceLoadCurrent(run) || cancellationToken.isCancellationRequested) {
        setWorkspaceIndexSummary({
          scannedFiles: 0,
          loadedFiles: 0,
          truncated: false,
          cancelled: true,
        });
        provider.setWorkspaceLoadStatus({
          state: "degraded",
          scannedFiles: 0,
          loadedFiles: 0,
          truncated: false,
          cancelled: true,
        });
        setServerHealth(
          context,
          "degraded",
          "Workspace indexing was cancelled before the workspace model request completed."
        );
        return;
      }

      const loadStartedAt = Date.now();
      const result = await provider.loadWorkspaceModel([anchorUri], {
        runId: run.runId,
        token: cancellationToken,
      });
      logPerf("workspaceIndexing:modelLoadComplete", {
        runId: run.runId,
        totalMs: Date.now() - loadStartedAt,
        scannedFiles: result.fileCount,
        loadedFiles: result.loadedFiles,
        committed: result.committed,
        stale: result.stale,
        failures: result.failures,
        cancelled: result.cancelled,
      });
      if (!isWorkspaceLoadCurrent(run) || result.stale) {
        logPerfEvent("workspaceLoad:staleCompletionIgnored", {
          runId: run.runId,
          loadedFiles: result.loadedFiles,
          failures: result.failures,
          cancelled: result.cancelled,
        });
        return;
      }

      const loadedFiles = result.committed ? result.loadedFiles : 0;
      setWorkspaceIndexSummary({
        scannedFiles: result.fileCount,
        loadedFiles,
        truncated: false,
        cancelled: result.cancelled > 0 && !result.committed,
        failures: result.failures,
      });
      provider.setWorkspaceLoadStatus({
        state:
          result.failures > 0 || result.cancelled > 0 ? "degraded" : "ready",
        scannedFiles: result.fileCount,
        loadedFiles,
        truncated: false,
        cancelled: result.cancelled > 0 && !result.committed,
        failures: result.failures,
      });
      log(
        "loadWorkspaceSysMLFiles: loaded backend workspace model for",
        result.fileCount,
        "files"
      );
      logStartupPhase("workspaceIndexing:end", {
        scannedFiles: result.fileCount,
        loadedFiles,
        truncated: false,
        durationMs: Date.now() - workspaceIndexingStartedAt,
      });
      vscode.commands.executeCommand("setContext", "sysml.hasWorkspace", true);
      vscode.commands.executeCommand(
        "setContext",
        "sysml.workspaceViewMode",
        provider.getWorkspaceViewMode()
      );
      vscode.commands.executeCommand("setContext", "sysml.modelLoaded", true);
      if (result.failures > 0) {
        setServerHealth(
          context,
          "degraded",
          `Workspace indexed ${loadedFiles}/${result.fileCount} SysML/KerML file(s); ${result.failures} file(s) failed to load.`
        );
      } else if (result.cancelled > 0 && !result.committed) {
        setServerHealth(
          context,
          "degraded",
          `Workspace indexing was cancelled after loading ${loadedFiles} of ${result.fileCount} file(s).`
        );
      } else if (result.fileCount === 0) {
        setServerHealth(
          context,
          "ready",
          "No SysML/KerML files were found in the current workspace."
        );
      } else {
        setServerHealth(
          context,
          "ready",
          `Workspace indexed ${result.fileCount} SysML/KerML file(s).`
        );
      }
    }
  );
  finishWorkspaceLoad(run, {
    workspaceViewMode: provider.getWorkspaceViewMode(),
  });
  refreshWorkspaceLifecycleSurfaces();
}

async function reloadWorkspaceExplorerModel(
  context: vscode.ExtensionContext,
  params: SemanticIndexReadyParams,
  logStartupPhase: (phase: string, extra?: Record<string, unknown>) => void,
  logPerf: (event: string, extra?: Record<string, unknown>) => void
): Promise<void> {
  const provider = modelExplorerProvider;
  if (!provider || !hasWorkspaceFolder()) {
    return;
  }
  if (params.lifecycle !== "ready") {
    return;
  }
  if (params.semanticStateVersion === lastLoadedSemanticStateVersion) {
    return;
  }
  lastSemanticIndexReadyWorkspaceFileCount = params.workspaceFileCount;
  notifyWorkspaceLifecycleChanged();
  log(
    "reloadWorkspaceExplorerModel: semantic index ready",
    "version=",
    params.semanticStateVersion,
    "workspaceFiles=",
    params.workspaceFileCount
  );
  lspModelProvider?.clearModelCache();
  try {
    await loadWorkspaceSysMLFiles(context, provider, logStartupPhase, logPerf);
    lastLoadedSemanticStateVersion = params.semanticStateVersion;
  } catch (error) {
    logError("reloadWorkspaceExplorerModel failed", error);
  }
}

export async function ensureWorkspaceModelLoaded(
  provider: ModelExplorerProvider | undefined,
  context: vscode.ExtensionContext,
  logStartupPhase: (phase: string, extra?: Record<string, unknown>) => void,
  logPerf: (event: string, extra?: Record<string, unknown>) => void,
  options?: { force?: boolean }
): Promise<void> {
  if (!provider) {
    return;
  }
  const hasWorkspaceFolders =
    (vscode.workspace.workspaceFolders?.length ?? 0) > 0;
  if (!hasWorkspaceFolders) {
    provider.refresh();
    return;
  }
  if (provider.hasWorkspaceData()) {
    if (options?.force) {
      lspModelProvider?.clearModelCache();
      await loadWorkspaceSysMLFiles(context, provider, logStartupPhase, logPerf);
      return;
    }
    if (lastLoadedSemanticStateVersion !== undefined) {
      provider.refresh();
      return;
    }
    scheduleWorkspaceExplorerPending(provider);
    return;
  }
  if (lastLoadedSemanticStateVersion !== undefined) {
    await loadWorkspaceSysMLFiles(context, provider, logStartupPhase, logPerf);
    return;
  }
  scheduleWorkspaceExplorerPending(provider);
}

async function syncModelExplorerFromEditor(
  editor: vscode.TextEditor,
  position: vscode.Position,
  reason: string,
  logPerf: (event: string, extra?: Record<string, unknown>) => void,
  context: vscode.ExtensionContext,
  logStartupPhase: (phase: string, extra?: Record<string, unknown>) => void
): Promise<void> {
  if (!isLanguageClientReady() || !modelExplorerProvider || !getLanguageClient()) {
    return;
  }
  const doc = editor.document;
  if (!isSysmlDoc(doc)) {
    return;
  }
  const provider = modelExplorerProvider;
  const syncStartedAt = Date.now();
  try {
    if (hasWorkspaceFolder()) {
      await provider.awaitInFlightWorkspaceLoad();
      if (!provider.hasWorkspaceData()) {
        await loadWorkspaceSysMLFiles(
          context,
          provider,
          logStartupPhase,
          logPerf
        );
        await provider.awaitInFlightWorkspaceLoad();
      }
    } else {
      await provider.loadDocument(doc);
    }
    const result = await lspModelProvider!.getModel(
      doc.uri.toString(),
      graphScopesForContext(),
      undefined,
      reason
    );
    const scopedNodes = graphNodesForDocumentUri(result.graph?.nodes, doc.uri);
    const node = bestGraphNodeAtPosition(scopedNodes, position);
    if (node?.id) {
      await provider.revealElement(doc.uri, node.id, node.range);
      logPerf("selectionSync:modelExplorer", {
        uri: doc.uri.toString(),
        totalMs: Date.now() - syncStartedAt,
        nodeId: node.id,
      });
      return;
    }
    const treeItem = provider.findElementTreeItemAtPosition(doc.uri, position);
    if (treeItem?.element.id) {
      await provider.revealElement(
        doc.uri,
        treeItem.element.id,
        treeItem.element.range
      );
      logPerf("selectionSync:modelExplorerTreeFallback", {
        uri: doc.uri.toString(),
        totalMs: Date.now() - syncStartedAt,
        nodeId: treeItem.element.id,
      });
      return;
    }
    logPerf("selectionSync:modelExplorerNoNode", {
      uri: doc.uri.toString(),
      totalMs: Date.now() - syncStartedAt,
    });
  } catch (error) {
    if (isClientNotRunningError(error)) {
      logPerf("selectionSync:modelExplorerSkipped", {
        uri: doc.uri.toString(),
        totalMs: Date.now() - syncStartedAt,
      });
      return;
    }
    logError(`Source-to-model-explorer sync failed for ${doc.uri.toString()}`, error);
    logPerf("selectionSync:modelExplorerFailed", {
      uri: doc.uri.toString(),
      totalMs: Date.now() - syncStartedAt,
      error: error instanceof Error ? error.message : String(error),
    });
  }
}

export function registerWorkspaceIndexing(
  context: vscode.ExtensionContext,
  handles: LspClientHandles,
  provider: ModelExplorerProvider,
  logStartupPhase: (phase: string, extra?: Record<string, unknown>) => void,
  logPerf: (event: string, extra?: Record<string, unknown>) => void
): void {
  extensionContext = context;
  modelExplorerProvider = provider;
  lspModelProvider = handles.lspModelProvider;
  logStartupPhaseFn = logStartupPhase;
  logPerfFn = logPerf;

  registerWorkspaceLifecycleSnapshotProvider(collectWorkspaceLifecycleInput);
  onWorkspaceLifecycleChanged(refreshWorkspaceLifecycleSurfaces);

  const semanticIndexReadyNotification = new NotificationType<SemanticIndexReadyParams>(
    "spec42/semanticIndexReady"
  );
  context.subscriptions.push(
    handles.client.onNotification(semanticIndexReadyNotification, (params) => {
      void reloadWorkspaceExplorerModel(
        context,
        params,
        logStartupPhase,
        logPerf
      );
    })
  );

  const scheduleModelExplorerSelectionSync = (
    event: vscode.TextEditorSelectionChangeEvent
  ) => {
    if (!isLanguageClientReady() || !modelExplorerProvider) {
      return;
    }
    const doc = event.textEditor.document;
    if (!isSysmlDoc(doc)) {
      return;
    }
    if (modelExplorerSelectionSyncTimer) {
      clearTimeout(modelExplorerSelectionSyncTimer);
    }
    const editor = event.textEditor;
    const position =
      event.selections[0]?.active ?? event.textEditor.selection.active;
    modelExplorerSelectionSyncTimer = setTimeout(() => {
      modelExplorerSelectionSyncTimer = undefined;
      void syncModelExplorerFromEditor(
        editor,
        position,
        "selectionSync:modelExplorer",
        logPerf,
        context,
        logStartupPhase
      );
    }, 150);
  };

  context.subscriptions.push(
    vscode.window.onDidChangeTextEditorSelection((event) => {
      const doc = event.textEditor.document;
      if (!isSysmlDoc(doc)) {
        return;
      }
      scheduleModelExplorerSelectionSync(event);

      const panel = VisualizationPanel.currentPanel;
      if (!panel || !panel.tracksUri(doc.uri) || panel.isNavigating()) {
        return;
      }
      if (sourceSelectionSyncTimer) {
        clearTimeout(sourceSelectionSyncTimer);
      }
      sourceSelectionSyncTimer = setTimeout(async () => {
        sourceSelectionSyncTimer = undefined;
        const client = getLanguageClient();
        if (!client || client.state !== State.Running) {
          return;
        }
        const diagramSyncStartedAt = Date.now();
        try {
          const result = await handles.lspModelProvider.getModel(
            doc.uri.toString(),
            graphScopesForContext(),
            undefined,
            "selectionSync:diagram"
          );
          const node = bestGraphNodeAtPosition(
            result.graph?.nodes,
            event.selections[0]?.active ?? event.textEditor.selection.active
          );
          if (node) {
            panel.revealSourceSelection(node);
            logPerf("selectionSync:diagram", {
              uri: doc.uri.toString(),
              totalMs: Date.now() - diagramSyncStartedAt,
              nodeId: node.id,
              nodeCount: result.graph?.nodes?.length ?? 0,
            });
          } else {
            logPerf("selectionSync:diagramNoNode", {
              uri: doc.uri.toString(),
              totalMs: Date.now() - diagramSyncStartedAt,
            });
          }
        } catch (error) {
          if (isClientNotRunningError(error)) {
            logPerf("selectionSync:diagramSkipped", {
              uri: doc.uri.toString(),
              totalMs: Date.now() - diagramSyncStartedAt,
            });
            return;
          }
          logError(`Source-to-diagram sync failed for ${doc.uri.toString()}`, error);
          logPerf("selectionSync:diagramFailed", {
            uri: doc.uri.toString(),
            totalMs: Date.now() - diagramSyncStartedAt,
            error: error instanceof Error ? error.message : String(error),
          });
        }
      }, 150);
    })
  );

  const scheduleModelExplorerRefresh = (
    changeKind: "save" | "create" | "delete",
    uri?: vscode.Uri
  ) => {
    if (!modelExplorerProvider) {
      return;
    }
    const docIsSysml =
      !!uri &&
      (uri.fsPath.toLowerCase().endsWith(".sysml") ||
        uri.fsPath.toLowerCase().endsWith(".kerml"));
    if (uri && !docIsSysml) {
      return;
    }
    if (modelExplorerRefreshTimer) {
      clearTimeout(modelExplorerRefreshTimer);
    }
    modelExplorerRefreshTimer = setTimeout(() => {
      modelExplorerRefreshTimer = undefined;
      const explorerProvider = modelExplorerProvider;
      if (!explorerProvider) {
        return;
      }
      if (
        ((vscode.workspace.workspaceFolders?.length ?? 0) > 0 ||
          explorerProvider.isWorkspaceBacked()) &&
        (changeKind === "save" || changeKind === "create" || changeKind === "delete")
      ) {
        void loadWorkspaceSysMLFiles(
          context,
          explorerProvider,
          logStartupPhase,
          logPerf
        );
        return;
      }
      explorerProvider.refresh();
    }, 250);
  };

  const sysmlFileWatcher = vscode.workspace.createFileSystemWatcher("**/*.{sysml,kerml}");
  sysmlFileWatcher.onDidChange((uri) => {
    handles.lspModelProvider.invalidateModelCache(uri);
    VisualizationPanel.currentPanel?.notifyFileChanged(uri);
    scheduleModelExplorerRefresh("save", uri);
  });
  sysmlFileWatcher.onDidCreate((uri) => {
    handles.lspModelProvider.invalidateModelCache(uri);
    VisualizationPanel.currentPanel?.notifyFileChanged(uri);
    scheduleModelExplorerRefresh("create", uri);
  });
  sysmlFileWatcher.onDidDelete((uri) => {
    handles.lspModelProvider.invalidateModelCache(uri);
    VisualizationPanel.currentPanel?.notifyFileChanged(uri);
    scheduleModelExplorerRefresh("delete", uri);
  });
  context.subscriptions.push(sysmlFileWatcher);

  context.subscriptions.push(
    vscode.workspace.onDidOpenTextDocument((doc) => {
      if (doc.languageId === "sysml" || doc.languageId === "kerml") {
        handles.lspModelProvider.invalidateModelCache(doc.uri);
        if (
          vscode.window.activeTextEditor?.document.uri.toString() ===
          doc.uri.toString()
        ) {
          scheduleModelExplorerRefreshForCurrentMode("didOpen", doc);
        }
      }
    })
  );
  context.subscriptions.push(
    vscode.workspace.onDidChangeTextDocument((event) => {
      const doc = event.document;
      if (doc.languageId === "sysml" || doc.languageId === "kerml") {
        handles.lspModelProvider.invalidateModelCache(doc.uri);
      }
    })
  );
  context.subscriptions.push(
    vscode.workspace.onDidCloseTextDocument((doc) => {
      if (doc.languageId === "sysml" || doc.languageId === "kerml") {
        handles.lspModelProvider.invalidateModelCache(doc.uri);
      }
    })
  );

  context.subscriptions.push(
    vscode.workspace.onDidSaveTextDocument((doc) => {
      if (doc.languageId === "sysml" || doc.languageId === "kerml") {
        VisualizationPanel.currentPanel?.notifyFileChanged(doc.uri);
        scheduleModelExplorerRefresh("save", doc.uri);
      }
    })
  );

  const refreshContext = () => {
    const active = vscode.window.activeTextEditor?.document;
    const loaded = shouldShowModelExplorerContext(modelExplorerProvider);
    vscode.commands.executeCommand("setContext", "sysml.modelLoaded", loaded);
    updateStatusBar(context);
    if (active && isSysmlDoc(active)) {
      scheduleModelExplorerRefreshForCurrentMode("refreshContext", active);
    }
  };

  refreshContext();
  context.subscriptions.push(
    vscode.window.onDidChangeActiveTextEditor(() => refreshContext())
  );

  const hasWorkspaceFolders = (vscode.workspace.workspaceFolders?.length ?? 0) > 0;
  const startupWorkspaceIndexingMode = getStartupWorkspaceIndexingMode();
  log("Activation complete. Workspace folders:", hasWorkspaceFolders);
  vscode.commands.executeCommand("setContext", "sysml.hasWorkspace", hasWorkspaceFolders);
  vscode.commands.executeCommand(
    "setContext",
    "sysml.visualizerOpen",
    !!VisualizationPanel.currentPanel
  );
  if (hasWorkspaceFolders && modelExplorerProvider) {
    scheduleWorkspaceExplorerPending(modelExplorerProvider);
    log(
      "Startup workspace model load deferred until semantic index is ready",
      "mode=",
      startupWorkspaceIndexingMode
    );
  }
}

export function getModelExplorerProvider(): ModelExplorerProvider | undefined {
  return modelExplorerProvider;
}

export function deactivateWorkspaceIndexing(): void {
  if (activeDocumentExplorerRefreshTimer) {
    clearTimeout(activeDocumentExplorerRefreshTimer);
    activeDocumentExplorerRefreshTimer = undefined;
  }
  if (modelExplorerSelectionSyncTimer) {
    clearTimeout(modelExplorerSelectionSyncTimer);
    modelExplorerSelectionSyncTimer = undefined;
  }
  cancelWorkspaceLoad(modelExplorerProvider, "deactivate");
}

export function getDebugExtensionState(): DebugExtensionState {
  const lastRender = getLastVisualizerRender();
  return {
    serverHealthState: getServerHealthState(),
    serverHealthDetail: getServerHealthDetail(),
    workspaceIndexSummary: getWorkspaceIndexSummary(),
    lastLoadedSemanticStateVersion,
    lastSemanticIndexReadyWorkspaceFileCount,
    modelExplorer: {
      lastRevealedElementId:
        modelExplorerProvider?.getDebugState().lastRevealedElementId,
      pendingWorkspaceLoadRunId:
        modelExplorerProvider?.getDebugState().pendingWorkspaceLoadRunId,
    },
    visualizerOpen: VisualizationPanel.currentPanel !== undefined,
    lastVisualizerRender: lastRender
      ? {
          view: lastRender.view,
          outcome: lastRender.outcome,
          graphNodes: lastRender.graphNodes,
          hasExportableSvg: lastRender.hasExportableSvg,
          timestampMs: lastRender.timestampMs,
        }
      : undefined,
  };
}

export function scheduleActiveDocumentExplorerRefreshOnRestart(): void {
  scheduleActiveDocumentExplorerRefresh("restartServer");
}

export async function reloadWorkspaceModel(
  context: vscode.ExtensionContext,
  provider: ModelExplorerProvider,
  logStartupPhase: (phase: string, extra?: Record<string, unknown>) => void,
  logPerf: (event: string, extra?: Record<string, unknown>) => void
): Promise<void> {
  await loadWorkspaceSysMLFiles(context, provider, logStartupPhase, logPerf);
}

export async function debugSyncModelExplorerSelection(
  targetEditor: vscode.TextEditor | undefined,
  logPerf: (event: string, extra?: Record<string, unknown>) => void,
  context: vscode.ExtensionContext,
  logStartupPhase: (phase: string, extra?: Record<string, unknown>) => void
): Promise<void> {
  if (modelExplorerSelectionSyncTimer) {
    clearTimeout(modelExplorerSelectionSyncTimer);
    modelExplorerSelectionSyncTimer = undefined;
  }
  const editor =
    targetEditor ??
    vscode.window.activeTextEditor ??
    vscode.window.visibleTextEditors.find((candidate) =>
      isSysmlDoc(candidate.document)
    );
  if (!editor || !isSysmlDoc(editor.document)) {
    return;
  }
  const previousReveal = modelExplorerProvider?.getDebugState().lastRevealedElementId;
  await syncModelExplorerFromEditor(
    editor,
    editor.selection.active,
    "debug:syncModelExplorerSelection",
    logPerf,
    context,
    logStartupPhase
  );
  if (modelExplorerProvider?.getDebugState().lastRevealedElementId !== previousReveal) {
    return;
  }
  await new Promise((resolve) => setTimeout(resolve, 200));
  await syncModelExplorerFromEditor(
    editor,
    editor.selection.active,
    "debug:syncModelExplorerSelection:retry",
    logPerf,
    context,
    logStartupPhase
  );
}

export function scheduleWorkspaceExplorerPendingOnRestart(): void {
  if (modelExplorerProvider && hasWorkspaceFolder()) {
    scheduleWorkspaceExplorerPending(modelExplorerProvider);
  }
}

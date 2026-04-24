import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import {
  CloseAction,
  ErrorAction,
  ErrorHandler,
  LanguageClient,
  LanguageClientOptions,
  RevealOutputChannelOn,
  ServerOptions,
  State,
  TransportKind,
} from "vscode-languageclient/node";
import { LspModelProvider } from "./providers/lspModelProvider";
import {
  ModelExplorerProvider,
  ModelTreeItem,
} from "./explorer/modelExplorerProvider";
import { ExamplesViewProvider } from "./examples/examplesViewProvider";
import { LibraryWebviewViewProvider } from "./library/libraryWebviewViewProvider";
import { AddonsWebviewViewProvider } from "./addons/addonsWebviewViewProvider";
import {
  areExperimentalFeaturesEnabled,
  getAddonStates,
  isAddonEnabled,
  setAddonEnabled,
} from "./addons/registry";
import { SoftwareAnalysisStore } from "./addons/softwareAnalysisStore";
import type { GraphNodeDTO } from "./providers/sysmlModelTypes";
import { getOutputChannel, log, logError, logPerfEvent, logStartupEvent, showChannel } from "./logger";
import { dumpGraphForGeneralView } from "./graphDump";
import {
  RESTORE_STATE_KEY,
  VisualizationPanel,
  VisualizerRestoreState,
} from "./visualization/visualizationPanel";
import {
  SOFTWARE_RESTORE_STATE_KEY,
  SoftwareVisualizationPanel,
  type SoftwareVisualizerRestoreState,
} from "./visualization/softwareVisualizationPanel";
import {
  SOFTWARE_ENABLED_VIEWS,
  SYSML_ENABLED_VIEWS,
} from "./visualization/webview/constants";
import { getWebviewHtml } from "./visualization/htmlBuilder";
const CONFIG_SECTION = "spec42";
const LEGACY_CONFIG_SECTION = "sysml-language-server";
const EXTENSION_ID = "Elan8.spec42";

type ServerHealthState =
  | "starting"
  | "ready"
  | "indexing"
  | "degraded"
  | "restarting"
  | "crashed";

type StartupWorkspaceIndexingMode = "lazy" | "background" | "eager";

let client: LanguageClient | undefined;
let statusItem: vscode.StatusBarItem | undefined;
let modelExplorerProvider: ModelExplorerProvider | undefined;
let examplesViewProvider: ExamplesViewProvider | undefined;
let libraryWebviewProvider: LibraryWebviewViewProvider | undefined;
let addonsWebviewProvider: AddonsWebviewViewProvider | undefined;
let lspModelProviderForStatus: LspModelProvider | undefined;
let serverHealthState: ServerHealthState = "starting";
let serverHealthDetail = "";
let sourceSelectionSyncTimer: ReturnType<typeof setTimeout> | undefined;
let activeDocumentExplorerRefreshTimer: ReturnType<typeof setTimeout> | undefined;
let activeDocumentExplorerRefreshUri: string | undefined;
let activeDocumentExplorerRefreshGuardUntil = 0;
let languageClientReady = false;

function getConfig() {
  return {
    primary: vscode.workspace.getConfiguration(CONFIG_SECTION),
    legacy: vscode.workspace.getConfiguration(LEGACY_CONFIG_SECTION),
  };
}

function getConfigString(key: string): string | undefined {
  const { primary, legacy } = getConfig();
  return primary.get<string>(key) ?? legacy.get<string>(key) ?? undefined;
}

function getConfigStringArray(key: string): string[] | undefined {
  const { primary, legacy } = getConfig();
  return primary.get<string[]>(key) ?? legacy.get<string[]>(key) ?? undefined;
}

function getConfigBoolean(key: string, defaultValue: boolean): boolean {
  const { primary, legacy } = getConfig();
  return primary.get<boolean>(key) ?? legacy.get<boolean>(key) ?? defaultValue;
}

function getConfigNumber(key: string, defaultValue: number): number {
  const { primary, legacy } = getConfig();
  return primary.get<number>(key) ?? legacy.get<number>(key) ?? defaultValue;
}

type StandardLibraryConfig = {
  enabled: boolean;
  version: string;
  repo: string;
  contentPath: string;
};

function getStartupWorkspaceIndexingMode(): StartupWorkspaceIndexingMode {
  const configured = getConfigString("startup.workspaceIndexing");
  if (
    configured === "lazy" ||
    configured === "background" ||
    configured === "eager"
  ) {
    return configured;
  }
  return "lazy";
}

function getStandardLibraryConfig(): StandardLibraryConfig {
  return {
    enabled: getConfigBoolean("standardLibrary.enabled", true),
    version: getConfigString("standardLibrary.version") ?? "2026-02",
    repo: getConfigString("standardLibrary.repo") ?? "Systems-Modeling/SysML-v2-Release",
    contentPath: getConfigString("standardLibrary.contentPath") ?? "sysml.library",
  };
}

function isDefaultServerPath(value: string): boolean {
  return value === "spec42" || value === "sysml-language-server";
}

type WorkspaceIndexSummary = {
  scannedFiles: number;
  loadedFiles: number;
  truncated: boolean;
  cancelled: boolean;
  failures?: number;
};

let lastWorkspaceIndexSummary: WorkspaceIndexSummary | undefined;

type WorkspaceLoadRun = {
  runId: string;
  cts: vscode.CancellationTokenSource;
  targetViewMode: "bySemantic" | "byFile";
  discoveredFiles: vscode.Uri[];
  startedAt: number;
};

let activeWorkspaceLoadRun: WorkspaceLoadRun | undefined;
let nextWorkspaceLoadRunId = 0;

type DebugExtensionState = {
  serverHealthState: ServerHealthState;
  serverHealthDetail: string;
  workspaceIndexSummary?: WorkspaceIndexSummary;
  modelExplorer?: {
    lastRevealedElementId?: string;
  };
};

function setServerHealth(
  context: vscode.ExtensionContext,
  state: ServerHealthState,
  detail = ""
): void {
  serverHealthState = state;
  serverHealthDetail = detail;
  log("Server health:", state, detail);
  updateStatusBar(context);
}

function activeSysmlDocument(
  doc?: vscode.TextDocument
): vscode.TextDocument | undefined {
  if (doc && isSysmlDoc(doc)) {
    return doc;
  }
  const active = vscode.window.activeTextEditor?.document;
  return isSysmlDoc(active) ? active : undefined;
}

async function showServerIssue(
  message: string,
  level: "warning" | "error"
): Promise<void> {
  const actions = ["Show Output", "Restart Server"];
  const selection =
    level === "error"
      ? await vscode.window.showErrorMessage(message, ...actions)
      : await vscode.window.showWarningMessage(message, ...actions);
  if (selection === "Show Output") {
    showChannel();
  } else if (selection === "Restart Server") {
    await vscode.commands.executeCommand("sysml.restartServer");
  }
}

function getBundledServerCommand(extensionPath: string): string {
  const platform = process.platform;
  const arch = process.arch;
  const binaryName =
    platform === "win32" ? "spec42.exe" : "spec42";
  const bundledPath = path.join(
    extensionPath,
    "server",
    `${platform}-${arch}`,
    binaryName
  );
  if (fs.existsSync(bundledPath)) {
    return bundledPath;
  }
  return "spec42";
}

function getDevelopmentServerSource(extensionPath: string, workspaceRoot: string): string | undefined {
  const candidates = [
    // Dev checkout layout: <repo>/vscode as extensionPath.
    path.resolve(
      extensionPath,
      "..",
      "target",
      "debug",
      process.platform === "win32" ? "spec42.exe" : "spec42"
    ),
    // Fallback: workspace-relative for fixture workspaces inside repo.
    path.resolve(
      workspaceRoot || ".",
    "target",
    "debug",
    process.platform === "win32" ? "spec42.exe" : "spec42"
    ),
  ];
  return candidates.find((candidate) => fs.existsSync(candidate));
}

function prepareDevelopmentServerCommand(
  extensionPath: string,
  workspaceRoot: string,
  storagePath: string
): string | undefined {
  const source = getDevelopmentServerSource(extensionPath, workspaceRoot);
  if (!source) {
    return undefined;
  }
  try {
    fs.mkdirSync(storagePath, { recursive: true });
    const ext = process.platform === "win32" ? ".exe" : "";
    const staged = path.join(storagePath, `spec42-dev${ext}`);
    fs.copyFileSync(source, staged);
    return staged;
  } catch (err) {
    logError("Failed to stage development server binary", err);
    return source;
  }
}

function isSysmlDoc(doc: vscode.TextDocument | undefined): boolean {
  if (!doc) return false;
  return doc.languageId === "sysml" || doc.languageId === "kerml";
}

function rangeContainsPosition(
  range: { start: { line: number; character: number }; end: { line: number; character: number } },
  position: vscode.Position
): boolean {
  const afterStart =
    position.line > range.start.line ||
    (position.line === range.start.line && position.character >= range.start.character);
  const beforeEnd =
    position.line < range.end.line ||
    (position.line === range.end.line && position.character <= range.end.character);
  return afterStart && beforeEnd;
}

function rangeSpanScore(
  range: { start: { line: number; character: number }; end: { line: number; character: number } }
): number {
  return (range.end.line - range.start.line) * 10000 + (range.end.character - range.start.character);
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

function shouldShowModelExplorerContext(
  provider: ModelExplorerProvider | undefined
): boolean {
  const hasWorkspace = (vscode.workspace.workspaceFolders?.length ?? 0) > 0;
  const activeIsSysml = isSysmlDoc(vscode.window.activeTextEditor?.document);
  const hasModelData =
    (provider?.getAllElements().length ?? 0) > 0 ||
    (provider?.isWorkspaceBacked() ?? false);
  return hasWorkspace || activeIsSysml || hasModelData;
}

function resolveAdditionalExamplesRoots(
  extensionPath: string
): vscode.Uri[] {
  const roots: vscode.Uri[] = [];
  const seen = new Set<string>();
  const addRoot = (rootPath: string): void => {
    const normalized = path.resolve(rootPath);
    const key = normalized.toLowerCase();
    if (seen.has(key) || !fs.existsSync(normalized)) {
      return;
    }
    seen.add(key);
    roots.push(vscode.Uri.file(normalized));
  };

  // Extension-local examples roots only (no workspace-relative probing).
  addRoot(path.join(extensionPath, "examples"));
  addRoot(path.join(extensionPath, "..", "examples"));

  return roots;
}

function getEnabledVisualizationViewIds(): Set<string> {
  return new Set<string>(SYSML_ENABLED_VIEWS);
}

function getVisualizationViews(): Array<{ id: string; label: string; description: string }> {
  const enabledViews = getEnabledVisualizationViewIds();
  return [
    { id: "general-view", label: "General", description: "General view (SysML v2 general-view)" },
    { id: "interconnection-view", label: "Interconnection", description: "Interconnection view (internal block and connector routing)" },
    { id: "action-flow-view", label: "Action Flow", description: "Behavior and flow rendering" },
    { id: "state-transition-view", label: "State Transition", description: "State-machine rendering" },
  ].filter((v) => enabledViews.has(v.id));
}

function setWorkspaceIndexSummary(summary: WorkspaceIndexSummary | undefined): void {
  lastWorkspaceIndexSummary = summary;
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

function ensureStatusItem(context: vscode.ExtensionContext): vscode.StatusBarItem {
  if (!statusItem) {
    statusItem = vscode.window.createStatusBarItem(
      vscode.StatusBarAlignment.Right,
      100
    );
    statusItem.name = "SysML Diagnostics";
    statusItem.command = "workbench.actions.view.problems";
    context.subscriptions.push(statusItem);
  }
  return statusItem;
}

function updateStatusBar(context: vscode.ExtensionContext): void {
  const enabled = getConfigBoolean("statusBar.enabled", true);
  if (!enabled) {
    statusItem?.hide();
    return;
  }

  const editor = vscode.window.activeTextEditor;
  const doc = editor?.document;
  const showHealthWithoutDoc = serverHealthState !== "ready";
  if ((!doc || !isSysmlDoc(doc)) && !showHealthWithoutDoc) {
    statusItem?.hide();
    return;
  }

  const item = ensureStatusItem(context);
  const diags = doc && isSysmlDoc(doc) ? vscode.languages.getDiagnostics(doc.uri) : [];
  const errors = diags.filter(
    (d) => d.severity === vscode.DiagnosticSeverity.Error
  ).length;
  const warnings = diags.filter(
    (d) => d.severity === vscode.DiagnosticSeverity.Warning
  ).length;
  const healthText =
    serverHealthState === "starting"
      ? "$(sync~spin) SysML: Starting"
      : serverHealthState === "indexing"
        ? "$(sync~spin) SysML: Indexing"
      : serverHealthState === "restarting"
        ? "$(sync~spin) SysML: Restarting"
        : serverHealthState === "degraded"
          ? "$(warning) SysML: Degraded"
          : serverHealthState === "crashed"
            ? "$(error) SysML: Server stopped"
            : undefined;
  const diagnosticsText = (() => {
    const icon = errors > 0 ? "$(error)" : warnings > 0 ? "$(warning)" : "$(check)";
    return `${icon} SysML: ${errors}E ${warnings}W`;
  })();
  item.text = healthText ?? diagnosticsText;
  const baseTooltip = healthText
    ? `Server state: ${serverHealthState}${serverHealthDetail ? `\n${serverHealthDetail}` : ""}`
    : `${errors} error(s), ${warnings} warning(s)\nClick to open Problems panel.`;
  const workspaceTooltip = lastWorkspaceIndexSummary
    ? `\n\nWorkspace indexing:\nScanned ${lastWorkspaceIndexSummary.scannedFiles} file(s)\nLoaded ${lastWorkspaceIndexSummary.loadedFiles} file(s)${(lastWorkspaceIndexSummary.failures ?? 0) > 0 ? `\nFailures: ${lastWorkspaceIndexSummary.failures}` : ""}${lastWorkspaceIndexSummary.truncated ? "\nResults may be incomplete." : ""}${lastWorkspaceIndexSummary.cancelled ? "\nLast scan was cancelled." : ""}`
    : "";
  item.tooltip = `${baseTooltip}${workspaceTooltip}`;
  item.show();

  // Append server health to tooltip (async, best-effort)
  const provider = lspModelProviderForStatus;
  if (provider) {
    provider.getServerStats().then((stats) => {
      if (!stats || !statusItem) return;
      const uptimeStr =
        stats.uptime >= 60
          ? `${Math.floor(stats.uptime / 60)}m ${stats.uptime % 60}s`
          : `${stats.uptime}s`;
      const caches = stats.caches;
      item.tooltip = `${baseTooltip}${workspaceTooltip}\n\n── LSP Server ──\nUptime: ${uptimeStr}\nCaches: ${caches.documents} docs, ${caches.symbolTables} symbols`;
    }).catch(() => {});
  }
}

export function activate(context: vscode.ExtensionContext): void {
  const startupTraceId = `startup-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
  const startupT0 = Date.now();
  const logPerf = (event: string, extra?: Record<string, unknown>) => {
    const elapsedMs = Date.now() - startupT0;
    const payload = {
      traceId: startupTraceId,
      event,
      elapsedMs,
      ...(extra ?? {}),
    };
    log("perf event", payload);
    logPerfEvent(event, {
      traceId: startupTraceId,
      elapsedMs,
      ...(extra ?? {}),
    });
  };
  const logStartupPhase = (phase: string, extra?: Record<string, unknown>) => {
    const elapsedMs = Date.now() - startupT0;
    const payload = {
      traceId: startupTraceId,
      phase,
      elapsedMs,
      ...(extra ?? {}),
    };
    log("startup phase", payload);
    logStartupEvent(phase, {
      traceId: startupTraceId,
      elapsedMs,
      ...(extra ?? {}),
    });
  };
  logStartupPhase("activate:start");
  log("Extension activating");
  setServerHealth(context, "starting", "Preparing SysML language server.");

  // In CI/test environments we may provide an explicit server path via env to
  // avoid workspace/OS-specific settings issues.
  const configStartedAt = Date.now();
  const envServerPath = (process.env.SPEC42_SERVER_PATH || "").trim();
  const serverPath = envServerPath || getConfigString("serverPath");
  const libraryPathsRaw = getConfigStringArray("libraryPaths") ?? [];
  const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath ?? "";
  const customLibraryPaths = libraryPathsRaw.map((p) =>
    path.isAbsolute(p) ? p : path.resolve(workspaceRoot, p)
  );
  const libraryPaths = customLibraryPaths.filter(
    (value, index, all) => all.indexOf(value) === index
  );
  logPerf("activate:configResolved", {
    totalMs: Date.now() - configStartedAt,
    workspaceFolderCount: vscode.workspace.workspaceFolders?.length ?? 0,
    libraryPathCount: libraryPaths.length,
    customLibraryPathCount: customLibraryPaths.length,
  });

  let serverCommand: string;
  const serverCommandResolutionStartedAt = Date.now();
  const devServerCommand = 
    context.extensionMode === vscode.ExtensionMode.Development
      ? prepareDevelopmentServerCommand(
          context.extensionPath,
          workspaceRoot,
          path.join(context.globalStorageUri.fsPath, "dev-server")
        )
      : undefined;
  if (devServerCommand && (!serverPath || isDefaultServerPath(serverPath))) {
    serverCommand = devServerCommand;
  } else if (!serverPath || isDefaultServerPath(serverPath)) {
    serverCommand = getBundledServerCommand(context.extensionPath);
  } else if (path.isAbsolute(serverPath) || path.win32.isAbsolute(serverPath)) {
    serverCommand = serverPath;
  } else {
    serverCommand = path.resolve(workspaceRoot, serverPath);
  }

  // On Windows, if the path doesn't exist but path.exe does, use that (e.g. target/release/spec42)
  if (
    process.platform === "win32" &&
    !fs.existsSync(serverCommand) &&
    !serverCommand.endsWith(".exe")
  ) {
    const withExe = `${serverCommand}.exe`;
    if (fs.existsSync(withExe)) {
      serverCommand = withExe;
    }
  }

  if (serverPath && !isDefaultServerPath(serverPath) && !fs.existsSync(serverCommand)) {
    void showServerIssue(
      `Configured SysML server path does not exist: ${serverCommand}. Startup will likely fail until this is corrected.`,
      "warning"
    );
  }
  const serverArgs = ["lsp"];
  logPerf("activate:serverCommandResolved", {
    totalMs: Date.now() - serverCommandResolutionStartedAt,
    serverCommand,
    serverArgs,
    hasExplicitServerPath: !!serverPath,
    usingDevServer: serverCommand === devServerCommand,
  });
  const missingLibraryPaths = libraryPaths.filter((p) => !fs.existsSync(p));
  if (missingLibraryPaths.length > 0) {
    void showServerIssue(
      `Some SysML library paths do not exist and will be ignored: ${missingLibraryPaths.join(", ")}`,
      "warning"
    );
  }
  log("Server command:", serverCommand, "args:", serverArgs, "libraryPaths:", libraryPaths);
  // Also log to console so CI captures the resolved command.
  try {
    // eslint-disable-next-line no-console
    console.log("[SysML] Server command:", serverCommand, "args:", serverArgs);
  } catch {
    // ignore
  }

  let restartCount = 0;
  let manualStopInProgress = false;
  let crashDialogShown = false;

  const errorHandler: ErrorHandler = {
    error: async (error, _message, count) => {
      restartCount = Math.max(restartCount, count ?? 0);
      const detail = error instanceof Error ? error.message : String(error ?? "unknown error");
      setServerHealth(context, "degraded", `Connection error: ${detail}`);
      logError("Language client transport error", error);
      if ((count ?? 0) >= 3) {
        await showServerIssue(
          `The SysML language server is unstable (${count} transport errors). Check the SysML output channel for details.`,
          "warning"
        );
      }
      return { action: ErrorAction.Continue, handled: true };
    },
    closed: async () => {
      if (manualStopInProgress) {
        setServerHealth(context, "starting", "Restarting SysML language server.");
        return { action: CloseAction.DoNotRestart, handled: true };
      }
      const shouldRestart = restartCount < 4;
      if (shouldRestart) {
        restartCount += 1;
        setServerHealth(
          context,
          "restarting",
          `Server process exited unexpectedly. Restart attempt ${restartCount} of 4.`
        );
        await showServerIssue(
          `The SysML language server stopped unexpectedly and will be restarted (attempt ${restartCount} of 4).`,
          "warning"
        );
        return { action: CloseAction.Restart, handled: true };
      }
      setServerHealth(
        context,
        "crashed",
        "Server process exited repeatedly. Automatic restart has been stopped."
      );
      if (!crashDialogShown) {
        crashDialogShown = true;
        await showServerIssue(
          "The SysML language server keeps crashing. Automatic restart has been stopped.",
          "error"
        );
      }
      return { action: CloseAction.DoNotRestart, handled: true };
    },
  };

  const serverOptions: ServerOptions = {
    command: serverCommand,
    args: serverArgs,
    transport: TransportKind.stdio,
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [
      { language: "sysml" },
      { language: "kerml" },
    ],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher("**/*.{sysml,kerml}"),
    },
    initializationOptions: {
      libraryPaths,
      startupTraceId,
      codeLens: {
        enabled: getConfigBoolean("codeLens.enabled", true),
      },
      performanceLogging: {
        enabled: getConfigBoolean("performanceLogging.enabled", false),
      },
    },
    initializationFailedHandler: (error) => {
      const detail = error instanceof Error ? error.message : String(error ?? "unknown initialization error");
      setServerHealth(context, "crashed", `Initialization failed: ${detail}`);
      logError("Language client initialization failed", error);
      void showServerIssue(
        `Failed to initialize the SysML language server: ${detail}`,
        "error"
      );
      return false;
    },
    errorHandler,
    outputChannel: getOutputChannel(),
    revealOutputChannelOn: RevealOutputChannelOn.Error,
    connectionOptions: {
      maxRestartCount: 4,
    },
    // No semantic-token dump middleware: production/runtime never writes token dumps.
  };

  client = new LanguageClient(
    "sysmlLanguageServer",
    "SysML Language Server",
    serverOptions,
    clientOptions
  );

  context.subscriptions.push(
    client.onDidChangeState(({ newState }) => {
      if (newState === State.Starting) {
        setServerHealth(context, "starting", "Starting SysML language server.");
      } else if (newState === State.Running) {
        restartCount = 0;
        crashDialogShown = false;
        setServerHealth(context, "ready", "SysML language server is ready.");
      } else if (newState === State.Stopped && !manualStopInProgress) {
        setServerHealth(context, "crashed", "SysML language server is stopped.");
      }
    })
  );

  const clientReadyPromise = client.start()
    .then(() => {
      restartCount = 0;
      crashDialogShown = false;
      languageClientReady = true;
      setServerHealth(context, "ready", "SysML language server is ready.");
      log("Language client ready, scheduling Model Explorer refresh");
      logStartupPhase("languageClient:ready");
      scheduleModelExplorerRefreshForCurrentMode("languageClient:ready");
      // If the visualizer panel was restored/open before LSP became ready,
      // force a refresh now so it doesn't remain empty until manual reopen.
      VisualizationPanel.currentPanel?.refresh();
    })
    .catch((error) => {
      const detail = error instanceof Error ? error.message : String(error ?? "unknown startup failure");
      languageClientReady = false;
      setServerHealth(context, "crashed", `Startup failed: ${detail}`);
      logError("Language client failed to start", error);
      void showServerIssue(
        `Failed to start the SysML language server: ${detail}`,
        "error"
      );
    });
  log("Language client started");
  logStartupPhase("languageClient:start");

  // Model Explorer (phase 3). getModel awaits whenReady so the server has received didOpen.
  const lspModelProvider = new LspModelProvider(client, clientReadyPromise);
  const softwareAnalysisStore = new SoftwareAnalysisStore();
  context.subscriptions.push(softwareAnalysisStore);
  lspModelProviderForStatus = lspModelProvider;
  modelExplorerProvider = new ModelExplorerProvider(lspModelProvider);
  examplesViewProvider = new ExamplesViewProvider(
    resolveAdditionalExamplesRoots(context.extensionPath)
  );
  libraryWebviewProvider = new LibraryWebviewViewProvider(
    context.extensionUri,
    lspModelProvider,
    () => ({
      pinnedVersion: getConfigString("standardLibrary.version") ?? "2026-02",
    })
  );
  addonsWebviewProvider = new AddonsWebviewViewProvider(
    context.extensionUri,
    () => {
      const workspaceRootUri = vscode.workspace.workspaceFolders?.[0]?.uri?.toString();
      const analysisEntry = workspaceRootUri
        ? softwareAnalysisStore.get(workspaceRootUri)
        : undefined;
      return getAddonStates(analysisEntry);
    },
    async (addonId, enabled) => {
      await setAddonEnabled(addonId, enabled);
      if (!enabled && addonId === "software-architecture") {
        const workspaceRootUri = vscode.workspace.workspaceFolders?.[0]?.uri?.toString();
        if (workspaceRootUri) {
          softwareAnalysisStore.clear(workspaceRootUri);
        }
      }
      await addonsWebviewProvider?.refresh();
    },
    async (addonId) => {
      if (addonId === "software-architecture") {
        await vscode.commands.executeCommand("spec42.addons.runSoftwareArchitectureAnalysis");
      }
    },
    async (addonId) => {
      if (addonId === "software-architecture") {
        await vscode.commands.executeCommand("spec42.addons.openSoftwareArchitecture");
      }
    }
  );

  function scheduleActiveDocumentExplorerRefresh(
    reason: string,
    doc?: vscode.TextDocument
  ): void {
    const targetDoc = activeSysmlDocument(doc);
    if (
      !languageClientReady ||
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

  function scheduleModelExplorerRefreshForCurrentMode(
    reason: string,
    doc?: vscode.TextDocument
  ): void {
    const provider = modelExplorerProvider;
    if (!languageClientReady || !provider) {
      return;
    }
    if ((vscode.workspace.workspaceFolders?.length ?? 0) > 0) {
      log("scheduleModelExplorerRefreshForCurrentMode: workspace", reason);
      void ensureWorkspaceModelLoaded(provider).catch((error) => {
        logError(`Workspace explorer refresh failed (${reason})`, error);
      });
      return;
    }
    scheduleActiveDocumentExplorerRefresh(reason, doc);
  }

  context.subscriptions.push(
    vscode.window.registerWebviewPanelSerializer("sysmlVisualizer", {
      async deserializeWebviewPanel(
        panel: vscode.WebviewPanel,
        _state: unknown
      ) {
        const saved = context.workspaceState.get<VisualizerRestoreState>(
          RESTORE_STATE_KEY
        );
        const extVersion =
          vscode.extensions.getExtension(EXTENSION_ID)?.packageJSON?.version ??
          "0.0.0";
        if (!saved?.workspaceRootUri) {
          panel.webview.html = getWebviewHtml(
            panel.webview,
            context.extensionUri,
            extVersion,
            SYSML_ENABLED_VIEWS,
          );
          return;
        }
        try {
          await VisualizationPanel.restore(
            panel,
            context,
            lspModelProvider,
            saved
          );
        } catch (err) {
          logError("Failed to restore visualization panel", err);
          panel.webview.html = getWebviewHtml(
            panel.webview,
            context.extensionUri,
            extVersion,
            SYSML_ENABLED_VIEWS,
          );
        }
      },
    })
  );

  context.subscriptions.push(
    vscode.window.registerWebviewPanelSerializer("spec42SoftwareVisualizer", {
      async deserializeWebviewPanel(
        panel: vscode.WebviewPanel,
        _state: unknown
      ) {
        const saved = context.workspaceState.get<SoftwareVisualizerRestoreState>(
          SOFTWARE_RESTORE_STATE_KEY
        );
        const extVersion =
          vscode.extensions.getExtension(EXTENSION_ID)?.packageJSON?.version ??
          "0.0.0";
        if (!saved?.workspaceRootUri) {
          panel.webview.html = getWebviewHtml(
            panel.webview,
            context.extensionUri,
            extVersion,
            SOFTWARE_ENABLED_VIEWS,
          );
          return;
        }
        try {
          await SoftwareVisualizationPanel.restore(
            panel,
            context,
            lspModelProvider,
            softwareAnalysisStore,
            saved
          );
        } catch (err) {
          logError("Failed to restore software visualization panel", err);
          panel.webview.html = getWebviewHtml(
            panel.webview,
            context.extensionUri,
            extVersion,
            SOFTWARE_ENABLED_VIEWS,
          );
        }
      },
    })
  );

  const treeView = vscode.window.createTreeView("sysmlModelExplorer", {
    treeDataProvider: modelExplorerProvider,
  });
  modelExplorerProvider.setTreeView(treeView);
  context.subscriptions.push(treeView);
  const examplesTreeView = vscode.window.createTreeView("spec42Examples", {
    treeDataProvider: examplesViewProvider,
  });
  context.subscriptions.push(examplesTreeView);
  context.subscriptions.push(
    treeView.onDidChangeVisibility((event) => {
      if (!event.visible) {
        return;
      }
      scheduleModelExplorerRefreshForCurrentMode("treeView:visible");
    })
  );

  context.subscriptions.push(
    vscode.window.registerWebviewViewProvider("spec42Library", libraryWebviewProvider)
  );
  context.subscriptions.push(
    vscode.window.registerWebviewViewProvider("spec42Addons", addonsWebviewProvider)
  );
  context.subscriptions.push(
    softwareAnalysisStore.onDidChange((entry) => {
      void addonsWebviewProvider?.refresh();
      if (
        SoftwareVisualizationPanel.currentPanel &&
        SoftwareVisualizationPanel.currentPanel.getWorkspaceRootUri() === entry.workspaceRootUri
      ) {
        SoftwareVisualizationPanel.currentPanel.refresh();
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("spec42.examples.openWorkspace", async (folderUri: vscode.Uri) => {
      if (!folderUri) {
        return;
      }
      await vscode.commands.executeCommand("vscode.openFolder", folderUri, false);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("spec42.examples.refresh", () => {
      examplesViewProvider?.refresh();
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.library.refresh", () => {
      libraryWebviewProvider?.refresh();
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.library.managePaths", async () => {
      await vscode.commands.executeCommand(
        "workbench.action.openSettings",
        "spec42.libraryPaths"
      );
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.library.search", async () => {
      await vscode.commands.executeCommand("workbench.view.extension.spec42");
      await vscode.commands.executeCommand("spec42Library.focus");
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.library.showStdLibStatus", async () => {
      const cfg = getStandardLibraryConfig();
      void vscode.window.showInformationMessage(
        `The SysML standard library is bundled with the Spec42 language server (release ${cfg.version}). Add extra library roots with spec42.libraryPaths if needed.`
      );
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("spec42.addons.toggle", async (addonId?: string, enabled?: boolean) => {
      const targetAddonId = addonId || "software-architecture";
      if (targetAddonId === "software-architecture" && !areExperimentalFeaturesEnabled()) {
        vscode.window.showInformationMessage(
          "The Software Architecture add-on is experimental. Enable Spec42 experimental features in settings first."
        );
        return;
      }
      const nextEnabled = typeof enabled === "boolean"
        ? enabled
        : !isAddonEnabled(targetAddonId);
      await setAddonEnabled(targetAddonId, nextEnabled);
      await addonsWebviewProvider?.refresh();
      if (!nextEnabled && targetAddonId === "software-architecture") {
        SoftwareVisualizationPanel.currentPanel?.dispose();
        const workspaceRootUri = vscode.workspace.workspaceFolders?.[0]?.uri?.toString();
        if (workspaceRootUri) {
          softwareAnalysisStore.clear(workspaceRootUri);
        }
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("spec42.addons.runSoftwareArchitectureAnalysis", async () => {
      if (!areExperimentalFeaturesEnabled()) {
        vscode.window.showInformationMessage(
          "The Software Architecture add-on is experimental. Enable Spec42 experimental features in settings first."
        );
        return;
      }
      if (!isAddonEnabled("software-architecture")) {
        vscode.window.showInformationMessage(
          "The Software Architecture add-on is currently disabled. Enable it in the Spec42 Add-ons view first."
        );
        return;
      }
      const workspaceRootUri = vscode.workspace.workspaceFolders?.[0]?.uri;
      if (!workspaceRootUri) {
        vscode.window.showWarningMessage("Open a workspace folder to analyze a Rust software workspace.");
        return;
      }
      try {
        const result = await softwareAnalysisStore.runAnalysis(
          workspaceRootUri.toString(),
          lspModelProvider,
        );
        if (result.status === "ready" && SoftwareVisualizationPanel.currentPanel) {
          SoftwareVisualizationPanel.currentPanel.refresh();
        }
      } catch (error) {
        vscode.window.showErrorMessage(
          `Software workspace analysis failed: ${error instanceof Error ? error.message : String(error)}`
        );
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("spec42.addons.openSoftwareArchitecture", async () => {
      if (!areExperimentalFeaturesEnabled()) {
        vscode.window.showInformationMessage(
          "The Software Architecture add-on is experimental. Enable Spec42 experimental features in settings first."
        );
        return;
      }
      if (!isAddonEnabled("software-architecture")) {
        vscode.window.showInformationMessage(
          "The Software Architecture add-on is currently disabled. Enable it in the Spec42 Add-ons view first."
        );
        return;
      }
      const workspaceRootUri = vscode.workspace.workspaceFolders?.[0]?.uri;
      if (!workspaceRootUri) {
        vscode.window.showWarningMessage("Open a workspace folder to use the Software Architecture visualizer.");
        return;
      }
      const entry = softwareAnalysisStore.get(workspaceRootUri.toString());
      if (entry.status !== "ready" || !entry.model) {
        vscode.window.showInformationMessage(
          "Run Software Architecture analysis from the Spec42 Add-ons view before opening the visualizer."
        );
        return;
      }
      SoftwareVisualizationPanel.createOrShow(
        context,
        workspaceRootUri,
        lspModelProvider,
        softwareAnalysisStore,
      );
    })
  );

  context.subscriptions.push(
    vscode.window.onDidChangeTextEditorSelection((event) => {
      const panel = VisualizationPanel.currentPanel;
      const doc = event.textEditor.document;
      if (!panel || !isSysmlDoc(doc) || !panel.tracksUri(doc.uri) || panel.isNavigating()) {
        return;
      }
      if (sourceSelectionSyncTimer) {
        clearTimeout(sourceSelectionSyncTimer);
      }
      sourceSelectionSyncTimer = setTimeout(async () => {
        sourceSelectionSyncTimer = undefined;
        const diagramSyncStartedAt = Date.now();
        try {
          const result = await lspModelProvider.getModel(
            doc.uri.toString(),
            ["graph"],
            undefined,
            "selectionSync:diagram"
          );
          const node = bestGraphNodeAtPosition(result.graph?.nodes, event.selections[0]?.active ?? event.textEditor.selection.active);
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

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.refreshModelTree", async () => {
      lspModelProvider.clearModelCache();
      if ((vscode.workspace.workspaceFolders?.length ?? 0) > 0 && modelExplorerProvider) {
        await loadWorkspaceSysMLFiles(modelExplorerProvider);
      } else {
        modelExplorerProvider?.refresh();
      }
    })
  );

  async function loadWorkspaceSysMLFiles(provider: ModelExplorerProvider): Promise<void> {
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
            result.failures > 0 || result.cancelled > 0
              ? "degraded"
              : "ready",
          scannedFiles: result.fileCount,
          loadedFiles,
          truncated: false,
          cancelled: result.cancelled > 0 && !result.committed,
          failures: result.failures,
        });
        log("loadWorkspaceSysMLFiles: loaded backend workspace model for", result.fileCount, "files");
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
    updateStatusBar(context);
  }

  async function ensureWorkspaceModelLoaded(
    provider: ModelExplorerProvider | undefined
  ): Promise<void> {
    if (!provider) {
      return;
    }
    if (provider.hasWorkspaceData()) {
      provider.refresh();
      return;
    }
    const hasWorkspaceFolders =
      (vscode.workspace.workspaceFolders?.length ?? 0) > 0;
    if (!hasWorkspaceFolders) {
      provider.refresh();
      return;
    }
    await loadWorkspaceSysMLFiles(provider);
  }

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.switchToByFile", async () => {
      modelExplorerProvider?.setWorkspaceViewMode("byFile");
      await ensureWorkspaceModelLoaded(modelExplorerProvider);
    })
  );
  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.switchToSemanticModel", async () => {
      modelExplorerProvider?.setWorkspaceViewMode("bySemantic");
      await ensureWorkspaceModelLoaded(modelExplorerProvider);
    })
  );
  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.toggleWorkspaceViewMode", async () => {
      modelExplorerProvider?.toggleWorkspaceViewMode();
      await ensureWorkspaceModelLoaded(modelExplorerProvider);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.openLocation", async (item: ModelTreeItem) => {
      log("openLocation called", "item:", !!item, "elementUri:", !!item?.elementUri, "resourceUri:", !!item?.resourceUri);
      if (!item) return;
      const uri = item.elementUri ?? item.resourceUri;
      if (!uri) {
        logError("openLocation: element has no URI", item);
        vscode.window.showErrorMessage("Cannot open location: element has no URI.");
        return;
      }
      const dtoRange = item.element?.range;
      if (!dtoRange) return;
      const range = new vscode.Range(
        new vscode.Position(dtoRange.start.line, dtoRange.start.character),
        new vscode.Position(dtoRange.end.line, dtoRange.end.character)
      );
      const targetUri = uri.toString();
      const activeEditor = vscode.window.activeTextEditor;
      const existingVisibleEditor =
        activeEditor?.document.uri.toString() === targetUri
          ? activeEditor
          : vscode.window.visibleTextEditors.find(
              (editorCandidate) =>
                editorCandidate.document.uri.toString() === targetUri
            );

      const editor = existingVisibleEditor
        ? await vscode.window.showTextDocument(existingVisibleEditor.document, {
            viewColumn: existingVisibleEditor.viewColumn,
            preserveFocus: false,
            preview: false,
          })
        : await vscode.window.showTextDocument(
            await vscode.workspace.openTextDocument(uri),
            {
              preserveFocus: false,
              preview: true,
            }
          );
      editor.selection = new vscode.Selection(range.start, range.start);
      editor.revealRange(range, vscode.TextEditorRevealType.InCenter);
    })
  );

  // Commands (quick wins)
  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.formatDocument", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!isSysmlDoc(editor?.document)) {
        vscode.window.showWarningMessage("No SysML/KerML document is active.");
        return;
      }
      await vscode.commands.executeCommand("editor.action.formatDocument");
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.validateModel", async () => {
      const editor = vscode.window.activeTextEditor;
      const doc = editor?.document;
      if (!doc || !isSysmlDoc(doc)) {
        vscode.window.showWarningMessage("No SysML/KerML document is active.");
        return;
      }
      const diags = vscode.languages.getDiagnostics(doc.uri);
      const errors = diags.filter(
        (d) => d.severity === vscode.DiagnosticSeverity.Error
      ).length;
      const warnings = diags.filter(
        (d) => d.severity === vscode.DiagnosticSeverity.Warning
      ).length;
      vscode.window.showInformationMessage(
        `Validation: ${errors} error(s), ${warnings} warning(s).`
      );
      await vscode.commands.executeCommand("workbench.actions.view.problems");
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.restartServer", async () => {
      if (!client) {
        vscode.window.showErrorMessage("SysML language server is not running.");
        return;
      }
      try {
        lspModelProvider.clearModelCache();
        languageClientReady = false;
        manualStopInProgress = true;
        setServerHealth(context, "restarting", "Restarting SysML language server.");
        await client.stop();
        manualStopInProgress = false;
        restartCount = 0;
        crashDialogShown = false;
        await client.start();
        lspModelProvider.clearModelCache();
        languageClientReady = true;
        scheduleActiveDocumentExplorerRefresh("restartServer");
        vscode.window.showInformationMessage("SysML language server restarted.");
      } catch (e) {
        manualStopInProgress = false;
        languageClientReady = false;
        setServerHealth(
          context,
          "crashed",
          `Restart failed: ${e instanceof Error ? e.message : String(e)}`
        );
        logError("restartServer failed", e);
        vscode.window.showErrorMessage(`Failed to restart server: ${e}`);
      }
    })
  );

  // Placeholder commands for later phases (so palette entries don't fail).
  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.showModelExplorer", async () => {
      await vscode.commands.executeCommand("workbench.view.extension.spec42");
      await vscode.commands.executeCommand(
        "setContext",
        "sysml.modelLoaded",
        shouldShowModelExplorerContext(modelExplorerProvider)
      );
      await vscode.commands.executeCommand("sysmlModelExplorer.focus");
      if ((vscode.workspace.workspaceFolders?.length ?? 0) > 0) {
        await ensureWorkspaceModelLoaded(modelExplorerProvider);
      } else if (modelExplorerProvider?.isWorkspaceBacked()) {
        modelExplorerProvider.refresh();
      } else {
        scheduleActiveDocumentExplorerRefresh("showModelExplorer");
      }
    })
  );
  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.showVisualizer", async () => {
      if (!client) {
        vscode.window.showErrorMessage("SysML language server is not running.");
        return;
      }
      let editor = vscode.window.activeTextEditor;
      if (!editor || (editor.document.languageId !== "sysml" && editor.document.languageId !== "kerml")) {
        editor = vscode.window.visibleTextEditors.find(
          (e) => (e.document.languageId === "sysml" || e.document.languageId === "kerml") && !e.document.isClosed
        );
      }
      if (!editor) {
        vscode.window.showWarningMessage("No SysML/KerML document is open. Open a .sysml or .kerml file first.");
        return;
      }
      // When in workspace mode, pass all workspace file URIs so the diagram shows merged model from all files
      const isWorkspace = modelExplorerProvider?.isWorkspaceBacked() ?? false;
      const workspaceUris = isWorkspace ? modelExplorerProvider?.getWorkspaceFileUris() : undefined;
      if (isWorkspace && workspaceUris && workspaceUris.length > 1) {
        const openDocs: vscode.TextDocument[] = [];
        let combinedContent = "";
        const fileNames: string[] = [];
        for (const uri of workspaceUris) {
          try {
            const doc = await vscode.workspace.openTextDocument(uri);
            openDocs.push(doc);
            const fileName = uri.fsPath.split(/[/\\]/).pop() ?? "";
            fileNames.push(fileName);
            combinedContent += `// === ${fileName} ===\n`;
            combinedContent += doc.getText();
            combinedContent += "\n\n";
          } catch {
            /* skip */
          }
        }
        if (openDocs.length > 0) {
          const firstDoc = openDocs[0];
          const combinedDocumentProxy = {
            getText: () => combinedContent,
            uri: firstDoc.uri,
            languageId: "sysml" as const,
            version: firstDoc.version,
            lineCount: combinedContent.split("\n").length,
            lineAt: (line: number) =>
              firstDoc.lineAt(Math.min(line, firstDoc.lineCount - 1)),
            offsetAt: (position: vscode.Position) => firstDoc.offsetAt(position),
            positionAt: (offset: number) => firstDoc.positionAt(offset),
            getWordRangeAtPosition: (position: vscode.Position) =>
              firstDoc.getWordRangeAtPosition(position),
            validateRange: (range: vscode.Range) => firstDoc.validateRange(range),
            validatePosition: (position: vscode.Position) =>
              firstDoc.validatePosition(position),
            fileName: firstDoc.fileName,
            isUntitled: false,
            isDirty: false,
            isClosed: false,
            eol: firstDoc.eol,
            save: () => Promise.resolve(false),
          } as unknown as vscode.TextDocument;
          const title = `SysML Visualization - ${fileNames.length} file(s)`;
          VisualizationPanel.createOrShow(
            context,
            combinedDocumentProxy,
            title,
            lspModelProvider
          );
          return;
        }
      }
      VisualizationPanel.createOrShow(context, editor.document, undefined, lspModelProvider);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "sysml.visualizeFolder",
      async (uri: vscode.Uri, selectedUris?: vscode.Uri[]) => {
        if (!client) {
          vscode.window.showErrorMessage("SysML language server is not running.");
          return;
        }
        try {
          let targetUris: vscode.Uri[] = [];
          if (selectedUris && selectedUris.length > 0) {
            targetUris = selectedUris;
          } else if (uri) {
            targetUris = [uri];
          } else {
            const editor = vscode.window.activeTextEditor;
            if (editor) targetUris = [editor.document.uri];
          }
          if (targetUris.length === 0) {
            vscode.window.showErrorMessage("No folder or file selected for SysML visualization");
            return;
          }

          const allSysmlFiles: vscode.Uri[] = [];
          const folderNames: string[] = [];
          for (const targetUri of targetUris) {
            const stat = await vscode.workspace.fs.stat(targetUri);
            if (stat.type === vscode.FileType.Directory) {
              const folderName = targetUri.fsPath.split(/[/\\]/).pop() ?? "";
              folderNames.push(folderName);
              const sysml = await vscode.workspace.findFiles(
                new vscode.RelativePattern(targetUri, "**/*.sysml"),
                "**/node_modules/**"
              );
              const kerml = await vscode.workspace.findFiles(
                new vscode.RelativePattern(targetUri, "**/*.kerml"),
                "**/node_modules/**"
              );
              allSysmlFiles.push(...sysml, ...kerml);
            } else if (targetUri.fsPath.endsWith(".sysml") || targetUri.fsPath.endsWith(".kerml")) {
              allSysmlFiles.push(targetUri);
            }
          }

          const uniqueFiles = [...new Map(allSysmlFiles.map((f) => [f.fsPath, f])).values()];
          if (uniqueFiles.length === 0) {
            vscode.window.showInformationMessage("No SysML/KerML files found in the selected folders/files");
            return;
          }

          const openDocs: vscode.TextDocument[] = [];
          let combinedContent = "";
          const fileNames: string[] = [];
          for (const fileUri of uniqueFiles) {
            try {
              const doc = await vscode.workspace.openTextDocument(fileUri);
              openDocs.push(doc);
              const fileName = fileUri.fsPath.split(/[/\\]/).pop() ?? "";
              fileNames.push(fileName);
              combinedContent += `// === ${fileName} ===\n`;
              combinedContent += doc.getText();
              combinedContent += "\n\n";
            } catch {
              log("Failed to open SysML file", fileUri.fsPath);
            }
          }

          if (openDocs.length === 0) {
            vscode.window.showErrorMessage("Failed to read any SysML files");
            return;
          }

          const firstDoc = openDocs[0];
          const combinedDocumentProxy = {
            getText: () => combinedContent,
            uri: firstDoc.uri,
            languageId: "sysml" as const,
            version: firstDoc.version,
            lineCount: combinedContent.split("\n").length,
            lineAt: (line: number) =>
              firstDoc.lineAt(Math.min(line, firstDoc.lineCount - 1)),
            offsetAt: (position: vscode.Position) => firstDoc.offsetAt(position),
            positionAt: (offset: number) => firstDoc.positionAt(offset),
            getWordRangeAtPosition: (position: vscode.Position) =>
              firstDoc.getWordRangeAtPosition(position),
            validateRange: (range: vscode.Range) => firstDoc.validateRange(range),
            validatePosition: (position: vscode.Position) =>
              firstDoc.validatePosition(position),
            fileName: firstDoc.fileName,
            isUntitled: false,
            isDirty: false,
            isClosed: false,
            eol: firstDoc.eol,
            save: () => Promise.resolve(false),
          } as unknown as vscode.TextDocument;

          let title: string;
          if (folderNames.length > 0) {
            title = `SysML Visualization - ${fileNames.length} files from ${folderNames.length} folder(s)`;
          } else {
            title = `SysML Visualization - ${fileNames.length} file(s)`;
          }

          VisualizationPanel.createOrShow(
            context,
            combinedDocumentProxy,
            title,
            lspModelProvider,
            uri && (await vscode.workspace.fs.stat(uri)).type === vscode.FileType.Directory
              ? uri
              : vscode.workspace.getWorkspaceFolder(firstDoc.uri)?.uri
          );
          VisualizationPanel.currentPanel?.refresh();
        } catch (error) {
          logError("sysml.visualizeFolder failed", error);
          vscode.window.showErrorMessage(`Failed to visualize SysML: ${error}`);
        }
      }
    )
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.showTypeHierarchy", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor || !isSysmlDoc(editor.document)) {
        vscode.window.showWarningMessage(
          "Open a SysML/KerML file to view the type hierarchy."
        );
        return;
      }
      await vscode.commands.executeCommand("editor.showTypeHierarchy");
    }),

    vscode.commands.registerCommand("sysml.showCallHierarchy", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor || !isSysmlDoc(editor.document)) {
        vscode.window.showWarningMessage(
          "Open a SysML/KerML file to view the call hierarchy."
        );
        return;
      }
      await vscode.commands.executeCommand("editor.showCallHierarchy");
    }),

    vscode.commands.registerCommand("sysml.showOutput", () => {
      showChannel();
    }),

    vscode.commands.registerCommand("sysml.debug.getExtensionState", (): DebugExtensionState => ({
      serverHealthState,
      serverHealthDetail,
      workspaceIndexSummary: lastWorkspaceIndexSummary,
      modelExplorer: {
        lastRevealedElementId: modelExplorerProvider?.getDebugState().lastRevealedElementId,
      },
    })),

    vscode.commands.registerCommand(
      "sysml.debug.getModelForTests",
      async (
        uri: string,
        scope?: Array<"graph" | "ibd" | "activityDiagrams" | "sequenceDiagrams" | "stats">
      ) => {
        return await lspModelProvider.getModel(
          uri,
          scope,
          undefined,
          "debug.getModelForTests"
        );
      }
    ),

    vscode.commands.registerCommand("sysml.debugDumpGraphForGeneralView", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor || !isSysmlDoc(editor.document)) {
        vscode.window.showWarningMessage(
          "Open a SysML/KerML file first (e.g. SurveillanceDrone.sysml)."
        );
        return;
      }
      const workspaceFolder = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
      if (!workspaceFolder) {
        vscode.window.showWarningMessage("No workspace folder open.");
        return;
      }
      try {
        const result = await lspModelProvider.getModel(
          editor.document.uri.toString(),
          ["graph"],
          undefined,
          "debugDumpGraphForGeneralView"
        );
        const outPath = path.join(workspaceFolder, "graph_general_view_dump.txt");
        await dumpGraphForGeneralView(result.graph, outPath);
        const jsonPath = outPath.replace(/\.txt$/, ".json");
        vscode.window.showInformationMessage(
          `Graph dump written to ${path.basename(outPath)} and ${path.basename(jsonPath)}`
        );
        log(`Graph dump: ${outPath}`);
      } catch (err) {
        logError("Debug graph dump failed", err);
        vscode.window.showErrorMessage(
          `Debug failed: ${err instanceof Error ? err.message : String(err)}`
        );
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.clearCache", async () => {
      if (!client) {
        vscode.window.showErrorMessage("SysML language server is not running.");
        return;
      }
      const result = await lspModelProvider.clearCache();
      if (result) {
        const total = result.documents + result.symbolTables + result.semanticTokens;
        vscode.window.showInformationMessage(
          `SysML: Cleared ${total} cache entries (${result.documents} docs, ${result.symbolTables} symbols)`
        );
        modelExplorerProvider?.refresh();
        // Refresh visualizer if open
        VisualizationPanel.currentPanel?.refresh();
      } else {
        vscode.window.showWarningMessage(
          "SysML: Could not clear cache (server may not be ready)."
        );
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "sysml.visualizePackage",
      async (item: ModelTreeItem) => {
        if (!item || !client) {
          vscode.window.showErrorMessage("No package selected or server not running.");
          return;
        }
        if (item.element.type !== "package") {
          vscode.window.showInformationMessage(
            "Visualize package is available for package elements."
          );
          return;
        }
        const packageName = item.element.name;
        const fileUri = item.elementUri;

        const isWorkspace = modelExplorerProvider?.isWorkspaceBacked() ?? false;
        const workspaceUris = isWorkspace ? modelExplorerProvider?.getWorkspaceFileUris() : undefined;

        const document = await vscode.workspace.openTextDocument(fileUri);

        if (isWorkspace && workspaceUris && workspaceUris.length > 1) {
          const openDocs: vscode.TextDocument[] = [];
          let combinedContent = "";
          const fileNames: string[] = [];
          for (const uri of workspaceUris) {
            try {
              const doc = await vscode.workspace.openTextDocument(uri);
              openDocs.push(doc);
              const fileName = uri.fsPath.split(/[/\\]/).pop() ?? "";
              fileNames.push(fileName);
              combinedContent += `// === ${fileName} ===\n`;
              combinedContent += doc.getText();
              combinedContent += "\n\n";
            } catch {
              /* skip */
            }
          }
          if (openDocs.length > 0) {
            const firstDoc = openDocs[0];
            const combinedDocumentProxy = {
              getText: () => combinedContent,
              uri: firstDoc.uri,
              languageId: "sysml" as const,
              version: firstDoc.version,
              lineCount: combinedContent.split("\n").length,
              lineAt: (line: number) =>
                firstDoc.lineAt(Math.min(line, firstDoc.lineCount - 1)),
              offsetAt: (position: vscode.Position) => firstDoc.offsetAt(position),
              positionAt: (offset: number) => firstDoc.positionAt(offset),
              getWordRangeAtPosition: (position: vscode.Position) =>
                firstDoc.getWordRangeAtPosition(position),
              validateRange: (range: vscode.Range) => firstDoc.validateRange(range),
              validatePosition: (position: vscode.Position) =>
                firstDoc.validatePosition(position),
              fileName: firstDoc.fileName,
              isUntitled: false,
              isDirty: false,
              isClosed: false,
              eol: firstDoc.eol,
              save: () => Promise.resolve(false),
            } as unknown as vscode.TextDocument;
            const title = `SysML Visualization - ${fileNames.length} file(s)`;
            VisualizationPanel.createOrShow(
              context,
              combinedDocumentProxy,
              title,
              lspModelProvider
            );
            setTimeout(() => {
              VisualizationPanel.currentPanel?.selectPackage(packageName);
            }, 500);
            return;
          }
        }

        VisualizationPanel.createOrShow(
          context,
          document,
          undefined,
          lspModelProvider
        );
        setTimeout(() => {
          VisualizationPanel.currentPanel?.selectPackage(packageName);
        }, 500);
      }
    )
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.changeVisualizerView", async (viewId?: string) => {
      if (!VisualizationPanel.currentPanel) {
        vscode.window.showWarningMessage("No visualization panel is currently open");
        return;
      }
      let selectedViewId = viewId;
      if (!selectedViewId) {
        const selected = await vscode.window.showQuickPick(
          getVisualizationViews().map((v) => ({
            label: v.label,
            description: v.description,
            viewId: v.id,
          })),
          { placeHolder: "Select visualization view" }
        );
        selectedViewId = selected?.viewId;
      }
      if (selectedViewId) {
        const enabledViews = getEnabledVisualizationViewIds();
        const view = enabledViews.has(selectedViewId) ? selectedViewId : 'general-view';
        if (view !== selectedViewId) {
          vscode.window.showWarningMessage(
            "That visualizer view is not currently enabled."
          );
        }
        VisualizationPanel.currentPanel.changeView(view);
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.visualizeFolderWithView", async (uri: vscode.Uri, selectedUris?: vscode.Uri[]) => {
      const selected = await vscode.window.showQuickPick(
        getVisualizationViews().map((v) => ({
          label: v.label,
          description: v.description,
          viewId: v.id,
        })),
        { placeHolder: "Select visualization view" }
      );
      if (selected) {
        await vscode.commands.executeCommand("sysml.visualizeFolder", uri, selectedUris);
        if (VisualizationPanel.currentPanel) {
          await vscode.commands.executeCommand("sysml.changeVisualizerView", selected.viewId);
        }
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.exportVisualization", async () => {
      if (!VisualizationPanel.currentPanel) {
        vscode.window.showWarningMessage("No visualization panel is currently open");
        return;
      }
      const defaultScale = getConfigNumber("visualization.exportScale", 2);
      const selected = await vscode.window.showQuickPick(
        [
          { label: "PNG", format: "png" },
          { label: "SVG", format: "svg" },
        ],
        { placeHolder: "Select export format" }
      );
      if (selected) {
        VisualizationPanel.currentPanel.exportVisualization(selected.format, defaultScale);
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.refreshVisualization", async () => {
      if (!VisualizationPanel.currentPanel) {
        vscode.window.showWarningMessage("No visualization panel is currently open");
        return;
      }
      const doc = VisualizationPanel.currentPanel.getDocument();
      VisualizationPanel.currentPanel.dispose();
      VisualizationPanel.createOrShow(context, doc, undefined, lspModelProvider);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("spec42.showInheritedAttributeInfo", () => {
      // Informational CodeLens only; intentionally no-op when clicked.
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "spec42.showReferencesCount",
      async (uriArg: unknown, positionArg: unknown) => {
        try {
          const uri = (() => {
            if (uriArg instanceof vscode.Uri) {
              return uriArg;
            }
            if (typeof uriArg === "string") {
              return vscode.Uri.parse(uriArg);
            }
            if (
              uriArg &&
              typeof uriArg === "object" &&
              "scheme" in uriArg &&
              "path" in uriArg
            ) {
              return vscode.Uri.from(uriArg as { scheme: string; authority?: string; path: string; query?: string; fragment?: string });
            }
            return vscode.window.activeTextEditor?.document.uri;
          })();
          const position = (() => {
            if (positionArg instanceof vscode.Position) {
              return positionArg;
            }
            if (
              positionArg &&
              typeof positionArg === "object" &&
              "line" in positionArg &&
              "character" in positionArg
            ) {
              return new vscode.Position(
                Number((positionArg as { line: number }).line),
                Number((positionArg as { character: number }).character)
              );
            }
            return vscode.window.activeTextEditor?.selection.active;
          })();
          if (!uri || !position) {
            log("showReferencesCount: missing uri/position", { uriArg, positionArg });
            return;
          }
          const locations = (await vscode.commands.executeCommand(
            "vscode.executeReferenceProvider",
            uri,
            position
          )) as vscode.Location[] | undefined;
          const refs = Array.isArray(locations) ? locations : [];
          if (refs.length === 0) {
            vscode.window.showInformationMessage("No references found.");
            return;
          }
          await vscode.commands.executeCommand(
            "editor.action.showReferences",
            uri,
            position,
            refs
          );
        } catch (err) {
          logError("showReferencesCount command failed", err);
        }
      }
    )
  );

  // Status bar + context for contributed view
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
  // When a SysML document is opened, refresh so we load it (did_open will be processed by then)
  context.subscriptions.push(
    vscode.workspace.onDidOpenTextDocument((doc) => {
      if (doc.languageId === "sysml" || doc.languageId === "kerml") {
        lspModelProvider.invalidateModelCache(doc.uri);
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
        lspModelProvider.invalidateModelCache(doc.uri);
      }
    })
  );
  context.subscriptions.push(
    vscode.workspace.onDidCloseTextDocument((doc) => {
      if (doc.languageId === "sysml" || doc.languageId === "kerml") {
        lspModelProvider.invalidateModelCache(doc.uri);
      }
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

      const verboseLoggingChanged =
        event.affectsConfiguration("spec42.logging.verbose") ||
        event.affectsConfiguration("spec42.debug") ||
        event.affectsConfiguration("sysml-language-server.debug");
      const codeLensConfigChanged =
        event.affectsConfiguration("spec42.codeLens.enabled") ||
        event.affectsConfiguration("sysml-language-server.codeLens.enabled");
      const performanceLoggingConfigChanged =
        event.affectsConfiguration("spec42.performanceLogging.enabled") ||
        event.affectsConfiguration("sysml-language-server.performanceLogging.enabled");

      // Recreate the panel so webview bootstrap flags (enabled views / verbose logging)
      // are regenerated without requiring a full VS Code reload.
      if (verboseLoggingChanged && VisualizationPanel.currentPanel) {
        const panelDoc = VisualizationPanel.currentPanel.getDocument();
        VisualizationPanel.currentPanel.dispose();
        VisualizationPanel.createOrShow(context, panelDoc, undefined, lspModelProvider);
      }
      if (verboseLoggingChanged && SoftwareVisualizationPanel.currentPanel) {
        const workspaceRootUri = vscode.workspace.workspaceFolders?.[0]?.uri;
        SoftwareVisualizationPanel.currentPanel.dispose();
        if (workspaceRootUri) {
          SoftwareVisualizationPanel.createOrShow(
            context,
            workspaceRootUri,
            lspModelProvider,
            softwareAnalysisStore,
          );
        }
      }

      if (event.affectsConfiguration("spec42.addons.softwareArchitecture.enabled")) {
        if (!isAddonEnabled("software-architecture")) {
          SoftwareVisualizationPanel.currentPanel?.dispose();
          const workspaceRootUri = vscode.workspace.workspaceFolders?.[0]?.uri?.toString();
          if (workspaceRootUri) {
            softwareAnalysisStore.clear(workspaceRootUri);
          }
        }
        void addonsWebviewProvider?.refresh();
      }

      if (event.affectsConfiguration("spec42.experimentalFeatures.enabled")) {
        if (!areExperimentalFeaturesEnabled()) {
          SoftwareVisualizationPanel.currentPanel?.dispose();
          const workspaceRootUri = vscode.workspace.workspaceFolders?.[0]?.uri?.toString();
          if (workspaceRootUri) {
            softwareAnalysisStore.clear(workspaceRootUri);
          }
        }
        void addonsWebviewProvider?.refresh();
      }

      if (codeLensConfigChanged || performanceLoggingConfigChanged) {
        void vscode.window
          .showInformationMessage(
            "A SysML server setting changed. Restart the SysML language server to apply it.",
            "Restart Server"
          )
          .then(async (selection) => {
            if (selection === "Restart Server") {
              await vscode.commands.executeCommand("sysml.restartServer");
            }
          });
      }

      void addonsWebviewProvider?.refresh();
    })
  );

  // Keep Model Explorer aligned with source changes similar to visualizer auto-refresh.
  let modelExplorerRefreshTimer: ReturnType<typeof setTimeout> | undefined;
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
      const provider = modelExplorerProvider;
      if (!provider) {
        return;
      }
      // In workspace mode, file additions/removals require rediscovery.
      if (
        ((vscode.workspace.workspaceFolders?.length ?? 0) > 0 || provider.isWorkspaceBacked()) &&
        (changeKind === "save" || changeKind === "create" || changeKind === "delete")
      ) {
        void loadWorkspaceSysMLFiles(provider);
        return;
      }
      provider.refresh();
    }, 250);
  };

  // Notify visualizer when SysML files change so it can refresh
  const sysmlFileWatcher = vscode.workspace.createFileSystemWatcher("**/*.{sysml,kerml}");
  sysmlFileWatcher.onDidChange((uri) => {
    lspModelProvider.invalidateModelCache(uri);
    VisualizationPanel.currentPanel?.notifyFileChanged(uri);
    scheduleModelExplorerRefresh("save", uri);
  });
  sysmlFileWatcher.onDidCreate((uri) => {
    lspModelProvider.invalidateModelCache(uri);
    VisualizationPanel.currentPanel?.notifyFileChanged(uri);
    scheduleModelExplorerRefresh("create", uri);
  });
  sysmlFileWatcher.onDidDelete((uri) => {
    lspModelProvider.invalidateModelCache(uri);
    VisualizationPanel.currentPanel?.notifyFileChanged(uri);
    scheduleModelExplorerRefresh("delete", uri);
  });
  context.subscriptions.push(sysmlFileWatcher);

  const rustFileWatcher = vscode.workspace.createFileSystemWatcher("**/*.{rs,toml}");
  rustFileWatcher.onDidChange(() => {
    void addonsWebviewProvider?.refresh();
  });
  rustFileWatcher.onDidCreate(() => {
    void addonsWebviewProvider?.refresh();
  });
  rustFileWatcher.onDidDelete(() => {
    void addonsWebviewProvider?.refresh();
  });
  context.subscriptions.push(rustFileWatcher);

  context.subscriptions.push(
    vscode.workspace.onDidChangeWorkspaceFolders(() => {
      examplesViewProvider?.refresh();
      void addonsWebviewProvider?.refresh();
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

  // Workspace mode: when workspace has folders (workspace file or opened folder), load all SysML/KerML files after delay
  const hasWorkspaceFolders = (vscode.workspace.workspaceFolders?.length ?? 0) > 0;
  const startupWorkspaceIndexingMode = getStartupWorkspaceIndexingMode();
  log("Activation complete. Workspace folders:", hasWorkspaceFolders);
  vscode.commands.executeCommand(
    "setContext",
    "sysml.hasWorkspace",
    hasWorkspaceFolders
  );
  vscode.commands.executeCommand(
    "setContext",
    "sysml.visualizerOpen",
    !!VisualizationPanel.currentPanel
  );
  if (hasWorkspaceFolders && modelExplorerProvider) {
    const provider = modelExplorerProvider;
    if (startupWorkspaceIndexingMode === "background") {
      setTimeout(() => {
        loadWorkspaceSysMLFiles(provider).catch(() => {});
      }, 3000);
    } else if (startupWorkspaceIndexingMode === "eager") {
      setTimeout(() => {
        loadWorkspaceSysMLFiles(provider).catch(() => {});
      }, 0);
    } else {
      log(
        "Startup workspace indexing deferred",
        "mode=",
        startupWorkspaceIndexingMode
      );
    }
  }
}

export function deactivate(): Thenable<void> | undefined {
  languageClientReady = false;
  if (activeDocumentExplorerRefreshTimer) {
    clearTimeout(activeDocumentExplorerRefreshTimer);
    activeDocumentExplorerRefreshTimer = undefined;
  }
  cancelWorkspaceLoad(modelExplorerProvider, "deactivate");
  return client?.stop();
}

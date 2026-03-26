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
import { LibraryViewProvider } from "./library/libraryViewProvider";
import type { GraphNodeDTO } from "./providers/sysmlModelTypes";
import { getOutputChannel, log, logError, showChannel } from "./logger";
import {
  decodeSemanticTokens,
  dumpSemanticTokens,
  getTokenAtPosition,
  SEMANTIC_TYPE_NAMES,
} from "./semanticTokensDump";
import { dumpGraphForGeneralView } from "./graphDump";
import {
  RESTORE_STATE_KEY,
  VisualizationPanel,
  VisualizerRestoreState,
} from "./visualization/visualizationPanel";
import {
  DEFAULT_ENABLED_VIEWS,
  EXPERIMENTAL_VIEWS,
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

let client: LanguageClient | undefined;
let statusItem: vscode.StatusBarItem | undefined;
let modelExplorerProvider: ModelExplorerProvider | undefined;
let libraryViewProvider: LibraryViewProvider | undefined;
let lspModelProviderForStatus: LspModelProvider | undefined;
let serverHealthState: ServerHealthState = "starting";
let serverHealthDetail = "";
let workspaceIndexingCts: vscode.CancellationTokenSource | undefined;
let sourceSelectionSyncTimer: ReturnType<typeof setTimeout> | undefined;

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

function isDefaultServerPath(value: string): boolean {
  return value === "spec42" || value === "sysml-language-server";
}

type WorkspaceIndexSummary = {
  scannedFiles: number;
  loadedFiles: number;
  perPatternLimit: number;
  truncated: boolean;
  cancelled: boolean;
};

let lastWorkspaceIndexSummary: WorkspaceIndexSummary | undefined;

type DebugExtensionState = {
  serverHealthState: ServerHealthState;
  serverHealthDetail: string;
  workspaceIndexSummary?: WorkspaceIndexSummary;
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

function getWorkspaceFileQueryLimit(): number {
  return getConfigNumber("workspace.maxFilesPerPattern", 500);
}

function getEnabledVisualizationViewIds(): Set<string> {
  const enabled = new Set<string>(DEFAULT_ENABLED_VIEWS);
  const includeExperimental = getConfigBoolean(
    "visualization.enableExperimentalViews",
    false
  );
  if (includeExperimental) {
    for (const viewId of EXPERIMENTAL_VIEWS) {
      enabled.add(viewId);
    }
  }
  return enabled;
}

function getVisualizationViews(): Array<{ id: string; label: string; description: string }> {
  const enabledViews = getEnabledVisualizationViewIds();
  return [
    { id: "general-view", label: "General", description: "General view (SysML v2 general-view)" },
    { id: "interconnection-view", label: "Interconnection", description: "Interconnection view (internal block and connector routing)" },
    { id: "action-flow-view", label: "Action Flow", description: "Experimental: behavior and flow rendering" },
    { id: "state-transition-view", label: "State Transition", description: "Experimental: state-machine rendering" },
    { id: "sequence-view", label: "Sequence", description: "Experimental: interaction rendering" },
  ].filter((v) => enabledViews.has(v.id));
}

function setWorkspaceIndexSummary(summary: WorkspaceIndexSummary | undefined): void {
  lastWorkspaceIndexSummary = summary;
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
    ? `\n\nWorkspace indexing:\nScanned ${lastWorkspaceIndexSummary.scannedFiles} file(s)\nLoaded ${lastWorkspaceIndexSummary.loadedFiles} file(s)\nLimit ${lastWorkspaceIndexSummary.perPatternLimit} per folder and file type${lastWorkspaceIndexSummary.truncated ? "\nResults truncated" : ""}${lastWorkspaceIndexSummary.cancelled ? "\nLast scan was cancelled" : ""}`
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
      item.tooltip = `${baseTooltip}\n\n── LSP Server ──\nUptime: ${uptimeStr}\nCaches: ${caches.documents} docs, ${caches.symbolTables} symbols`;
    }).catch(() => {});
  }
}

export function activate(context: vscode.ExtensionContext): void {
  log("Extension activating");
  setServerHealth(context, "starting", "Preparing SysML language server.");

  // In CI/test environments we may provide an explicit server path via env to
  // avoid workspace/OS-specific settings issues.
  const envServerPath = (process.env.SPEC42_SERVER_PATH || "").trim();
  const serverPath = envServerPath || getConfigString("serverPath");
  const libraryPathsRaw = getConfigStringArray("libraryPaths") ?? [];
  const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath ?? "";
  const libraryPaths = libraryPathsRaw.map((p) =>
    path.isAbsolute(p) ? p : path.resolve(workspaceRoot, p)
  );

  let serverCommand: string;
  if (!serverPath || isDefaultServerPath(serverPath)) {
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
  const missingLibraryPaths = libraryPaths.filter((p) => !fs.existsSync(p));
  if (missingLibraryPaths.length > 0) {
    void showServerIssue(
      `Some SysML library paths do not exist and will be ignored: ${missingLibraryPaths.join(", ")}`,
      "warning"
    );
  }

  log("Server command:", serverCommand, "libraryPaths:", libraryPaths);
  // Also log to console so CI captures the resolved command.
  try {
    // eslint-disable-next-line no-console
    console.log("[SysML] Server command:", serverCommand);
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
    args: [],
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
    middleware: {
      provideDocumentSemanticTokens: (document, token, next) => {
        const result = next(document, token);
        const dumpIfEnabled = (res: vscode.SemanticTokens | null | undefined): vscode.SemanticTokens | null | undefined => {
          if (getConfigBoolean("debug.dumpSemanticTokens", false) && res) {
            const workspaceFolder = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
            if (workspaceFolder) {
              const outPath = path.join(workspaceFolder, "semantic_tokens_frontend_dump.txt");
              dumpSemanticTokens(document, res, outPath);
              log(`Semantic tokens dump written to ${outPath}`);
            }
          }
          return res;
        };
        return Promise.resolve(result).then(dumpIfEnabled);
      },
    },
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
      setServerHealth(context, "ready", "SysML language server is ready.");
      log("Language client ready, refreshing Model Explorer");
      modelExplorerProvider?.refresh();
    })
    .catch((error) => {
      const detail = error instanceof Error ? error.message : String(error ?? "unknown startup failure");
      setServerHealth(context, "crashed", `Startup failed: ${detail}`);
      logError("Language client failed to start", error);
      void showServerIssue(
        `Failed to start the SysML language server: ${detail}`,
        "error"
      );
    });
  log("Language client started");

  // Model Explorer (phase 3). getModel awaits whenReady so the server has received didOpen.
  const lspModelProvider = new LspModelProvider(client, clientReadyPromise);
  lspModelProviderForStatus = lspModelProvider;
  modelExplorerProvider = new ModelExplorerProvider(lspModelProvider);
  libraryViewProvider = new LibraryViewProvider();

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
        if (!saved?.documentUri) {
          panel.webview.html = getWebviewHtml(
            panel.webview,
            context.extensionUri,
            extVersion
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
            extVersion
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

  const libraryTreeView = vscode.window.createTreeView("spec42Library", {
    treeDataProvider: libraryViewProvider,
  });
  context.subscriptions.push(libraryTreeView);

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.library.refresh", () => {
      libraryViewProvider?.refresh();
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.library.installStdLib", async () => {
      vscode.window.showInformationMessage(
        "Spec42 Library: Standard Library install/update will be added in a future release."
      );
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
      const query = await vscode.window.showInputBox({
        prompt: "Search libraries (placeholder)",
        placeHolder: "e.g. Systems Library",
      });
      if (query && query.trim().length > 0) {
        vscode.window.showInformationMessage(
          `Spec42 Library search is not implemented yet. Query: ${query}`
        );
      }
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
        try {
          const result = await lspModelProvider.getModel(doc.uri.toString(), ["graph"]);
          const node = bestGraphNodeAtPosition(result.graph?.nodes, event.selections[0]?.active ?? event.textEditor.selection.active);
          if (node) {
            panel.revealSourceSelection(node);
          }
        } catch (error) {
          logError(`Source-to-diagram sync failed for ${doc.uri.toString()}`, error);
        }
      }, 150);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.refreshModelTree", async () => {
      if (modelExplorerProvider?.isWorkspaceMode()) {
        await loadWorkspaceSysMLFiles(modelExplorerProvider);
      } else {
        modelExplorerProvider?.refresh();
      }
    })
  );

  async function loadWorkspaceSysMLFiles(provider: ModelExplorerProvider): Promise<void> {
    const folders = vscode.workspace.workspaceFolders ?? [];
    if (folders.length === 0) {
      log("loadWorkspaceSysMLFiles: no workspace folders");
      return;
    }
    workspaceIndexingCts?.cancel();
    workspaceIndexingCts?.dispose();
    workspaceIndexingCts = new vscode.CancellationTokenSource();
    const cancellationToken = workspaceIndexingCts.token;
    const perPatternLimit = getWorkspaceFileQueryLimit();
    setServerHealth(
      context,
      "indexing",
      `Scanning workspace for SysML/KerML files (limit ${perPatternLimit} per folder and file type).`
    );
    provider.setWorkspaceLoadStatus({
      state: "indexing",
      scannedFiles: 0,
      loadedFiles: 0,
      perPatternLimit,
      truncated: false,
      cancelled: false,
      failures: 0,
    });
    await vscode.window.withProgress(
      {
        location: vscode.ProgressLocation.Notification,
        title: "SysML workspace indexing",
        cancellable: true,
      },
      async (progress, token) => {
        token.onCancellationRequested(() => workspaceIndexingCts?.cancel());
        const fileUris: vscode.Uri[] = [];
        let limitHit = false;
        const totalSteps = Math.max(folders.length * 2, 1);

        for (const [folderIndex, folder] of folders.entries()) {
          if (cancellationToken.isCancellationRequested) {
            break;
          }
          progress.report({
            message: `Scanning ${path.basename(folder.uri.fsPath) || folder.name} for .sysml files`,
            increment: 100 / totalSteps,
          });
          const sysml = await vscode.workspace.findFiles(
            new vscode.RelativePattern(folder, "**/*.sysml"),
            null,
            perPatternLimit
          );
          if (sysml.length >= perPatternLimit) {
            limitHit = true;
          }
          fileUris.push(...sysml);

          if (cancellationToken.isCancellationRequested) {
            break;
          }

          progress.report({
            message: `Scanning ${path.basename(folder.uri.fsPath) || folder.name} for .kerml files`,
            increment: 100 / totalSteps,
          });
          const kerml = await vscode.workspace.findFiles(
            new vscode.RelativePattern(folder, "**/*.kerml"),
            null,
            perPatternLimit
          );
          if (kerml.length >= perPatternLimit) {
            limitHit = true;
          }
          fileUris.push(...kerml);

          if (folderIndex === folders.length - 1) {
            progress.report({
              message: "Preparing workspace model",
            });
          }
        }

        const uniqueFiles = [...new Map(fileUris.map((f) => [f.toString(), f])).values()];
        log("loadWorkspaceSysMLFiles: found", uniqueFiles.length, "unique files");
        provider.setWorkspaceLoadStatus({
          state: "indexing",
          scannedFiles: uniqueFiles.length,
          loadedFiles: 0,
          perPatternLimit,
          truncated: limitHit,
          cancelled: false,
          failures: 0,
        });

        if (cancellationToken.isCancellationRequested) {
          setWorkspaceIndexSummary({
            scannedFiles: uniqueFiles.length,
            loadedFiles: 0,
            perPatternLimit,
            truncated: limitHit,
            cancelled: true,
          });
          provider.setWorkspaceLoadStatus({
            state: "degraded",
            scannedFiles: uniqueFiles.length,
            loadedFiles: 0,
            perPatternLimit,
            truncated: limitHit,
            cancelled: true,
          });
          setServerHealth(
            context,
            "degraded",
            `Workspace indexing was cancelled after scanning ${uniqueFiles.length} file(s).`
          );
          return;
        }

        if (uniqueFiles.length > 0) {
          progress.report({
            message: `Loading model for ${uniqueFiles.length} file(s)`,
          });
          await provider.loadWorkspaceModel(uniqueFiles, cancellationToken);
          const loadedFiles = provider.getWorkspaceFileUris().length;
          setWorkspaceIndexSummary({
            scannedFiles: uniqueFiles.length,
            loadedFiles,
            perPatternLimit,
            truncated: limitHit,
            cancelled: false,
          });
          provider.setWorkspaceLoadStatus({
            state: limitHit ? "degraded" : "ready",
            scannedFiles: uniqueFiles.length,
            loadedFiles,
            perPatternLimit,
            truncated: limitHit,
            cancelled: false,
          });
          log("loadWorkspaceSysMLFiles: loaded model for", uniqueFiles.length, "files");
          vscode.commands.executeCommand("setContext", "sysml.hasWorkspace", true);
          vscode.commands.executeCommand(
            "setContext",
            "sysml.workspaceViewMode",
            provider.getWorkspaceViewMode()
          );
          vscode.commands.executeCommand("setContext", "sysml.modelLoaded", true);
          if (limitHit) {
            setServerHealth(
              context,
              "degraded",
              `Workspace indexing is truncated at ${perPatternLimit} files per folder and file type. Scanned ${uniqueFiles.length} file(s).`
            );
            void showServerIssue(
              `SysML workspace indexing hit the configured limit of ${perPatternLimit} files per folder and file type. Results may be incomplete.`,
              "warning"
            );
          } else {
            setServerHealth(
              context,
              "ready",
              `Workspace indexed ${uniqueFiles.length} SysML/KerML file(s).`
            );
          }
        } else {
          setWorkspaceIndexSummary({
            scannedFiles: 0,
            loadedFiles: 0,
            perPatternLimit,
            truncated: false,
            cancelled: false,
          });
          provider.setWorkspaceLoadStatus({
            state: "ready",
            scannedFiles: 0,
            loadedFiles: 0,
            perPatternLimit,
            truncated: false,
            cancelled: false,
          });
          setServerHealth(
            context,
            "ready",
            "No SysML/KerML files were found in the current workspace."
          );
        }
      }
    );
    updateStatusBar(context);
  }

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.switchToByFile", () => {
      modelExplorerProvider?.setWorkspaceViewMode("byFile");
    })
  );
  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.switchToSemanticModel", () => {
      modelExplorerProvider?.setWorkspaceViewMode("bySemantic");
    })
  );
  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.toggleWorkspaceViewMode", () => {
      modelExplorerProvider?.toggleWorkspaceViewMode();
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
      const doc = await vscode.workspace.openTextDocument(uri);
      const editor = await vscode.window.showTextDocument(doc, {
        preserveFocus: false,
        preview: true,
      });
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
        manualStopInProgress = true;
        setServerHealth(context, "restarting", "Restarting SysML language server.");
        await client.stop();
        manualStopInProgress = false;
        restartCount = 0;
        crashDialogShown = false;
        await client.start();
        vscode.window.showInformationMessage("SysML language server restarted.");
      } catch (e) {
        manualStopInProgress = false;
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
        isSysmlDoc(vscode.window.activeTextEditor?.document)
      );
      await vscode.commands.executeCommand("sysmlModelExplorer.focus");
      modelExplorerProvider?.refresh();
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
      const isWorkspace = modelExplorerProvider?.isWorkspaceMode() ?? false;
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
            lspModelProvider,
            workspaceUris
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
            uniqueFiles
          );

          if (modelExplorerProvider) {
            await modelExplorerProvider.loadWorkspaceModel(uniqueFiles);
          }
        } catch (error) {
          logError("sysml.visualizeFolder failed", error);
          vscode.window.showErrorMessage(`Failed to visualize SysML: ${error}`);
        }
      }
    )
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.showOutput", () => {
      showChannel();
    }),

    vscode.commands.registerCommand("sysml.debug.getExtensionState", (): DebugExtensionState => ({
      serverHealthState,
      serverHealthDetail,
      workspaceIndexSummary: lastWorkspaceIndexSummary,
    })),

    vscode.commands.registerCommand(
      "sysml.debug.getModelForTests",
      async (
        uri: string,
        scope?: Array<"graph" | "ibd" | "sequenceDiagrams" | "activityDiagrams" | "stats" | "renderedDiagrams">
      ) => {
        return await lspModelProvider.getModel(uri, scope);
      }
    ),

    vscode.commands.registerCommand("sysml.debugSemanticTokenAtCursor", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor || !isSysmlDoc(editor.document)) {
        vscode.window.showWarningMessage(
          "Open a SysML/KerML file and place the cursor on the word to debug."
        );
        return;
      }
      if (!client) {
        vscode.window.showErrorMessage("SysML language server is not running.");
        return;
      }
      try {
        const doc = editor.document;
        const pos = editor.selection.active;
        const result = await client.sendRequest<{ data?: number[] }>(
          "textDocument/semanticTokens/full",
          {
            textDocument: { uri: doc.uri.toString(), version: doc.version },
          }
        );
        if (!result?.data?.length) {
          vscode.window.showInformationMessage(
            "No semantic tokens returned. The server may not have indexed this document yet."
          );
          return;
        }
        const tokens = new vscode.SemanticTokens(new Uint32Array(result.data));
        const decoded = decodeSemanticTokens(doc, tokens);
        const token = getTokenAtPosition(decoded, pos.line, pos.character);
        const typeName = token
          ? SEMANTIC_TYPE_NAMES[token.type] ?? `?${token.type}`
          : "(none)";
        // Display 1-based line/col to match editor gutter
        const line1 = pos.line + 1;
        const col1 = pos.character + 1;
        const msg = token
          ? `Server token at line ${line1}, col ${col1}: "${token.text}" = ${typeName}`
          : `No token at line ${line1}, col ${col1} (cursor may be in whitespace or between tokens)`;
        vscode.window.showInformationMessage(msg);
        log(msg, "→ Use Developer: Inspect Editor Tokens and Scopes to compare with VS Code's rendering");
      } catch (err) {
        logError("Debug semantic token failed", err);
        vscode.window.showErrorMessage(
          `Debug failed: ${err instanceof Error ? err.message : String(err)}`
        );
      }
    }),

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
          ["graph"]
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

        const isWorkspace = modelExplorerProvider?.isWorkspaceMode() ?? false;
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
              lspModelProvider,
              workspaceUris
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
            "That visualizer view is not currently enabled. Turn on 'SysML > Visualization: Enable Experimental Views' to try experimental diagram types."
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
      const workspaceUris =
        modelExplorerProvider?.isWorkspaceMode() && (modelExplorerProvider?.getWorkspaceFileUris()?.length ?? 0) > 1
          ? modelExplorerProvider.getWorkspaceFileUris()
          : undefined;
      VisualizationPanel.currentPanel.dispose();
      VisualizationPanel.createOrShow(context, doc, undefined, lspModelProvider, workspaceUris);
    })
  );

  // Status bar + context for contributed view
  const refreshContext = () => {
    const active = vscode.window.activeTextEditor?.document;
    const loaded = isSysmlDoc(active);
    vscode.commands.executeCommand("setContext", "sysml.modelLoaded", loaded);
    updateStatusBar(context);
    // Refresh Model Explorer when switching to a SysML doc so the tree shows the correct model
    if (loaded) {
      modelExplorerProvider?.refresh();
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
        modelExplorerProvider?.refresh();
      }
    })
  );
  context.subscriptions.push(
    vscode.languages.onDidChangeDiagnostics(() => updateStatusBar(context))
  );

  // Notify visualizer when SysML files change so it can refresh
  const sysmlFileWatcher = vscode.workspace.createFileSystemWatcher("**/*.{sysml,kerml}");
  sysmlFileWatcher.onDidChange((uri) => {
    VisualizationPanel.currentPanel?.notifyFileChanged(uri);
  });
  sysmlFileWatcher.onDidCreate((uri) => {
    VisualizationPanel.currentPanel?.notifyFileChanged(uri);
  });
  sysmlFileWatcher.onDidDelete((uri) => {
    VisualizationPanel.currentPanel?.notifyFileChanged(uri);
  });
  context.subscriptions.push(sysmlFileWatcher);

  context.subscriptions.push(
    vscode.workspace.onDidSaveTextDocument((doc) => {
      if (doc.languageId === "sysml" || doc.languageId === "kerml") {
        VisualizationPanel.currentPanel?.notifyFileChanged(doc.uri);
      }
    })
  );

  // Workspace mode: when workspace has folders (workspace file or opened folder), load all SysML/KerML files after delay
  const hasWorkspaceFolders = (vscode.workspace.workspaceFolders?.length ?? 0) > 0;
  log("Activation complete. Workspace folders:", hasWorkspaceFolders);
  vscode.commands.executeCommand(
    "setContext",
    "sysml.hasWorkspace",
    hasWorkspaceFolders
  );
  if (hasWorkspaceFolders && modelExplorerProvider) {
    const provider = modelExplorerProvider;
    setTimeout(() => {
      loadWorkspaceSysMLFiles(provider).catch(() => {});
    }, 3000);
  }
}

export function deactivate(): Thenable<void> | undefined {
  workspaceIndexingCts?.cancel();
  workspaceIndexingCts?.dispose();
  workspaceIndexingCts = undefined;
  return client?.stop();
}

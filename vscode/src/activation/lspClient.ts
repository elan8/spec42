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
import { getOutputChannel, log, logError, logStartupEvent, showChannel } from "../logger";
import { registerSpec42LmTools } from "../lmTools/spec42LmTools";
import { runSpec42Json } from "../lmTools/spec42Cli";
import type { SysandStatusViewModel } from "../library/libraryStatusViewModel";
import { LspModelProvider } from "../providers/lspModelProvider";
import { VisualizationPanel } from "../visualization/visualizationPanel";
import { notifyWorkspaceLifecycleChanged } from "./workspaceLifecycle";
import { setVisualizationGateState } from "../visualization/visualizationGate";
import {
  getConfigBoolean,
  getConfigString,
  getConfigStringArray,
  isDefaultServerPath,
} from "./configBridge";
import { setServerHealth } from "./statusBar";

type RawSysandStatus = {
  installed?: boolean;
  executablePath?: string;
  version?: string;
  projectRoot?: string;
  manifestPresent?: boolean;
  lockPresent?: boolean;
  dependencyRoots?: string[];
  warnings?: string[];
};

type RawDoctorReport = {
  resolved_domain_libraries_path?: string;
  domain_libraries_source_kind?: string;
};

export type LspClientHandles = {
  client: LanguageClient;
  lspModelProvider: LspModelProvider;
  clientReadyPromise: Promise<void>;
  serverCommand: string;
  libraryPaths: string[];
  missingLibraryPaths: string[];
  workspaceRoot: string;
  readSysandStatus: () => Promise<SysandStatusViewModel>;
  readDomainLibrariesStatus: () => Promise<{
    resolvedPath?: string;
    sourceKind: string;
  }>;
};

let client: LanguageClient | undefined;
let languageClientReady = false;
let restartCount = 0;
let manualStopInProgress = false;
let crashDialogShown = false;
let lspModelProvider: LspModelProvider | undefined;

export function isLanguageClientReady(): boolean {
  return languageClientReady;
}

export function getLanguageClient(): LanguageClient | undefined {
  return client;
}

export function getLspModelProvider(): LspModelProvider | undefined {
  return lspModelProvider;
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
  const binaryName = platform === "win32" ? "spec42.exe" : "spec42";
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

function getDevelopmentServerSource(
  extensionPath: string,
  workspaceRoot: string
): string | undefined {
  const candidates = [
    path.resolve(
      extensionPath,
      "..",
      "target",
      "debug",
      process.platform === "win32" ? "spec42.exe" : "spec42"
    ),
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

function resolveServerCommand(
  context: vscode.ExtensionContext,
  workspaceRoot: string,
  serverPath: string | undefined
): { serverCommand: string; devServerCommand: string | undefined } {
  const devServerCommand =
    context.extensionMode === vscode.ExtensionMode.Development
      ? prepareDevelopmentServerCommand(
          context.extensionPath,
          workspaceRoot,
          path.join(context.globalStorageUri.fsPath, "dev-server")
        )
      : undefined;

  let serverCommand: string;
  if (devServerCommand && (!serverPath || isDefaultServerPath(serverPath))) {
    serverCommand = devServerCommand;
  } else if (!serverPath || isDefaultServerPath(serverPath)) {
    serverCommand = getBundledServerCommand(context.extensionPath);
  } else if (path.isAbsolute(serverPath) || path.win32.isAbsolute(serverPath)) {
    serverCommand = serverPath;
  } else {
    serverCommand = path.resolve(workspaceRoot, serverPath);
  }

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

  return { serverCommand, devServerCommand };
}

function normalizeSysandStatus(
  status: RawSysandStatus | undefined
): SysandStatusViewModel {
  return {
    installed: !!status?.installed,
    executablePath: status?.executablePath,
    version: status?.version,
    projectRoot: status?.projectRoot,
    manifestPresent: !!status?.manifestPresent,
    lockPresent: !!status?.lockPresent,
    dependencyRoots: Array.isArray(status?.dependencyRoots)
      ? status.dependencyRoots
      : [],
    warnings: Array.isArray(status?.warnings) ? status.warnings : [],
  };
}

export function startLanguageClient(
  context: vscode.ExtensionContext,
  startupTraceId: string,
  onClientReady: () => void,
  logStartupPhase: (phase: string, extra?: Record<string, unknown>) => void,
  logPerf?: (event: string, extra?: Record<string, unknown>) => void
): LspClientHandles {
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
  logPerf?.("activate:configResolved", {
    workspaceFolderCount: vscode.workspace.workspaceFolders?.length ?? 0,
    libraryPathCount: libraryPaths.length,
    customLibraryPathCount: customLibraryPaths.length,
  });

  const serverCommandResolutionStartedAt = Date.now();
  const { serverCommand, devServerCommand } = resolveServerCommand(
    context,
    workspaceRoot,
    serverPath
  );

  if (
    serverPath &&
    !isDefaultServerPath(serverPath) &&
    !fs.existsSync(serverCommand)
  ) {
    void showServerIssue(
      `Configured SysML server path does not exist: ${serverCommand}. Startup will likely fail until this is corrected.`,
      "warning"
    );
  }

  const serverArgs = ["lsp"];
  logPerf?.("activate:serverCommandResolved", {
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

  const readSysandStatus = async (): Promise<SysandStatusViewModel> => {
    const status = (await runSpec42Json(
      serverCommand,
      ["sysand", "status", "--format", "json"],
      workspaceRoot
    )) as RawSysandStatus;
    return normalizeSysandStatus(status);
  };

  const readDomainLibrariesStatus = async (): Promise<{
    resolvedPath?: string;
    sourceKind: string;
  }> => {
    const report = (await runSpec42Json(
      serverCommand,
      ["doctor", "--format", "json"],
      workspaceRoot
    )) as RawDoctorReport;
    return {
      resolvedPath: report.resolved_domain_libraries_path,
      sourceKind: report.domain_libraries_source_kind ?? "none",
    };
  };

  log("Server command:", serverCommand, "args:", serverArgs, "libraryPaths:", libraryPaths);
  registerSpec42LmTools(context, {
    serverCommand,
    workspaceRoot,
    libraryPaths,
  });

  if (getConfigBoolean("debug", false) || process.env.SPEC42_LOG_SERVER_COMMAND === "1") {
    try {
      // eslint-disable-next-line no-console
      console.log("[SysML] Server command:", serverCommand, "args:", serverArgs);
    } catch {
      // ignore
    }
  }

  const errorHandler: ErrorHandler = {
    error: async (error, _message, count) => {
      restartCount = Math.max(restartCount, count ?? 0);
      const detail =
        error instanceof Error ? error.message : String(error ?? "unknown error");
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
    documentSelector: [{ language: "sysml" }, { language: "kerml" }],
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
      const detail =
        error instanceof Error
          ? error.message
          : String(error ?? "unknown initialization error");
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
        setVisualizationGateState({ languageClientReady: false });
        setServerHealth(context, "starting", "Starting SysML language server.");
      } else if (newState === State.Running) {
        restartCount = 0;
        crashDialogShown = false;
        setVisualizationGateState({ languageClientReady: true });
        setServerHealth(context, "ready", "SysML language server is ready.");
        notifyWorkspaceLifecycleChanged();
      } else if (newState === State.Stopped && !manualStopInProgress) {
        setServerHealth(context, "crashed", "SysML language server is stopped.");
      }
    })
  );

  const clientReadyPromise = client
    .start()
    .then(() => {
      restartCount = 0;
      crashDialogShown = false;
      languageClientReady = true;
      setVisualizationGateState({ languageClientReady: true });
      setServerHealth(context, "ready", "SysML language server is ready.");
      notifyWorkspaceLifecycleChanged();
      log("Language client ready, waiting for semantic index before workspace model load");
      logStartupPhase("languageClient:ready");
      onClientReady();
      VisualizationPanel.currentPanel?.notifyWorkspaceLifecycleChanged();
    })
    .catch((error) => {
      const detail =
        error instanceof Error ? error.message : String(error ?? "unknown startup failure");
      languageClientReady = false;
      setVisualizationGateState({ languageClientReady: false });
      setServerHealth(context, "crashed", `Startup failed: ${detail}`);
      logError("Language client failed to start", error);
      void showServerIssue(
        `Failed to start the SysML language server: ${detail}`,
        "error"
      );
    });

  log("Language client started");
  logStartupPhase("languageClient:start");

  lspModelProvider = new LspModelProvider(client, clientReadyPromise);

  return {
    client,
    lspModelProvider,
    clientReadyPromise,
    serverCommand,
    libraryPaths,
    missingLibraryPaths,
    workspaceRoot,
    readSysandStatus,
    readDomainLibrariesStatus,
  };
}

export function registerRestartServerCommand(
  context: vscode.ExtensionContext,
  handles: Pick<LspClientHandles, "client" | "lspModelProvider">,
  callbacks: {
    onBeforeRestart?: () => void;
    onRestartComplete: () => void;
  }
): void {
  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.restartServer", async () => {
      if (!handles.client) {
        vscode.window.showErrorMessage("SysML language server is not running.");
        return;
      }
      try {
        handles.lspModelProvider.clearModelCache();
        callbacks.onBeforeRestart?.();
        languageClientReady = false;
        manualStopInProgress = true;
        setServerHealth(context, "restarting", "Restarting SysML language server.");
        await handles.client.stop(process.env.CI ? 20000 : 10000);
        manualStopInProgress = false;
        restartCount = 0;
        crashDialogShown = false;
        await handles.client.start();
        handles.lspModelProvider.clearModelCache();
        languageClientReady = true;
        callbacks.onRestartComplete();
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
}

export function registerServerConfigChangeHandler(
  context: vscode.ExtensionContext,
  lspModelProviderRef: LspModelProvider
): void {
  context.subscriptions.push(
    vscode.workspace.onDidChangeConfiguration((event) => {
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

      if (verboseLoggingChanged && VisualizationPanel.currentPanel) {
        const panelDoc = VisualizationPanel.currentPanel.getDocument();
        VisualizationPanel.currentPanel.dispose();
        VisualizationPanel.createOrShow(
          context,
          panelDoc,
          undefined,
          lspModelProviderRef
        );
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
    })
  );
}

export function deactivateLanguageClient(): Thenable<void> | undefined {
  languageClientReady = false;
  return client?.stop();
}

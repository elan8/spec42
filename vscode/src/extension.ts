import * as vscode from "vscode";
import { ModelExplorerProvider } from "./explorer/modelExplorerProvider";
import { LibraryWebviewViewProvider } from "./library/libraryWebviewViewProvider";
import { DOMAIN_LIBRARIES_DEFAULTS } from "./generated/domainLibrariesDefaults";
import { STANDARD_LIBRARY_DEFAULTS } from "./generated/standardLibraryDefaults";
import { log, logPerfEvent, logStartupEvent } from "./logger";
import {
  createExamplesViewProvider,
  onRestartServerComplete,
  registerExplorerCommands,
} from "./activation/commands/explorer";
import { registerLibraryCommands } from "./activation/commands/library";
import { HelpViewProvider } from "./help/helpViewProvider";
import { SysmlReferencePanel } from "./help/sysmlReferencePanel";
import { registerVisualizerCommands } from "./activation/commands/visualizer";
import { VisualizationPanel } from "./visualization/visualizationPanel";
import {
  deactivateLanguageClient,
  registerLanguageClientDebugCommands,
  registerRestartServerCommand,
  registerServerConfigChangeHandler,
  startLanguageClient,
} from "./activation/lspClient";
import {
  registerStatusBar,
  setLspModelProviderForStatus,
  setServerHealth,
} from "./activation/statusBar";
import {
  deactivateWorkspaceIndexing,
  registerWorkspaceIndexing,
  resetSemanticIndexTracking,
  scheduleModelExplorerRefreshForCurrentMode,
} from "./activation/workspaceIndexing";

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

  const handles = startLanguageClient(
    context,
    startupTraceId,
    () => scheduleModelExplorerRefreshForCurrentMode("languageClient:ready"),
    logStartupPhase,
    logPerf
  );

  setLspModelProviderForStatus(handles.lspModelProvider);
  registerStatusBar(context);

  const modelExplorerProvider = new ModelExplorerProvider(handles.lspModelProvider);
  const examplesViewProvider = createExamplesViewProvider(context.extensionPath);
  const libraryWebviewProvider = new LibraryWebviewViewProvider(
    context.extensionUri,
    handles.lspModelProvider,
    {
      getStdlibHeading: () => ({
        pinnedVersion: STANDARD_LIBRARY_DEFAULTS.version,
        format: STANDARD_LIBRARY_DEFAULTS.format,
      }),
      getDomainLibrariesHeading: () => ({
        pinnedVersion: DOMAIN_LIBRARIES_DEFAULTS.version,
        format: DOMAIN_LIBRARIES_DEFAULTS.format,
      }),
      getDomainLibrariesStatus: handles.readDomainLibrariesStatus,
      getConfiguredLibraryPaths: () => handles.libraryPaths,
      getMissingLibraryPaths: () => handles.missingLibraryPaths,
      getSysandStatus: handles.readSysandStatus,
    }
  );

  registerWorkspaceIndexing(
    context,
    handles,
    modelExplorerProvider,
    logStartupPhase,
    logPerf
  );

  VisualizationPanel.register(context, handles.lspModelProvider);
  registerVisualizerCommands(context, handles);
  registerExplorerCommands(
    context,
    handles,
    modelExplorerProvider,
    examplesViewProvider,
    logStartupPhase,
    logPerf
  );
  registerLibraryCommands(context, libraryWebviewProvider, handles);

  registerRestartServerCommand(context, handles, {
    onBeforeRestart: resetSemanticIndexTracking,
    onRestartComplete: onRestartServerComplete,
  });
  registerLanguageClientDebugCommands(context, handles);
  registerServerConfigChangeHandler(context, handles.lspModelProvider);

  const treeView = vscode.window.createTreeView("sysmlModelExplorer", {
    treeDataProvider: modelExplorerProvider,
  });
  modelExplorerProvider.setTreeView(treeView);
  context.subscriptions.push(treeView);

  const examplesTreeView = vscode.window.createTreeView("spec42Examples", {
    treeDataProvider: examplesViewProvider,
  });
  context.subscriptions.push(examplesTreeView);

  const helpTreeView = vscode.window.createTreeView("spec42Help", {
    treeDataProvider: new HelpViewProvider(),
  });
  context.subscriptions.push(helpTreeView);

  context.subscriptions.push(
    vscode.commands.registerCommand("spec42.help.openReference", () => {
      SysmlReferencePanel.open(context);
    })
  );

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

  logStartupPhase("activate:complete");
}

export function deactivate(): Thenable<void> | undefined {
  deactivateWorkspaceIndexing();
  return deactivateLanguageClient();
}

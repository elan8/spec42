import * as vscode from "vscode";
import { FeatureInspectorViewProvider } from "./inspector/featureInspectorViewProvider";
import { LibraryWebviewViewProvider } from "./library/libraryWebviewViewProvider";
import { KPAR_LIBRARIES_DEFAULTS } from "./generated/kparLibrariesDefaults";
import { STANDARD_LIBRARY_DEFAULTS } from "./generated/standardLibraryDefaults";
import { log, logPerfEvent, logStartupEvent } from "./logger";
import {
  createExamplesViewProvider,
  registerExplorerCommands,
} from "./activation/commands/explorer";
import { registerLibraryCommands } from "./activation/commands/library";
import { HelpViewProvider } from "./help/helpViewProvider";
import { SysmlReferencePanel } from "./help/sysmlReferencePanel";
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
} from "./activation/workspaceIndexing";
import { registerDiagramViewer } from "./diagram/diagramViewer";

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
    () => {},
    logStartupPhase,
    logPerf
  );

  setLspModelProviderForStatus(handles.lspModelProvider);
  registerStatusBar(context);

  const featureInspectorProvider = new FeatureInspectorViewProvider(
    context.extensionUri,
    handles.lspModelProvider
  );
  const examplesViewProvider = createExamplesViewProvider(context.extensionPath);
  const libraryWebviewProvider = new LibraryWebviewViewProvider(
    context.extensionUri,
    handles.lspModelProvider,
    {
      getStdlibHeading: () => ({
        pinnedVersion: STANDARD_LIBRARY_DEFAULTS.version,
        format: STANDARD_LIBRARY_DEFAULTS.format,
      }),
      getKparHeadings: () =>
        KPAR_LIBRARIES_DEFAULTS.map((library) => ({
          id: library.id,
          displayName: library.displayName,
          pinnedVersion: library.version,
          format: library.format,
        })),
      getConfiguredLibraryPaths: () => handles.libraryPaths,
      getMissingLibraryPaths: () => handles.missingLibraryPaths,
      getSysandStatus: handles.readSysandStatus,
    }
  );

  registerWorkspaceIndexing(
    context,
    handles,
    featureInspectorProvider,
    logStartupPhase,
    logPerf
  );

  registerExplorerCommands(
    context,
    handles,
    examplesViewProvider,
  );
  registerLibraryCommands(context, libraryWebviewProvider, handles);
  registerDiagramViewer(context, handles);

  registerRestartServerCommand(context, handles, {
    onBeforeRestart: resetSemanticIndexTracking,
    onRestartComplete: () => {},
  });
  registerLanguageClientDebugCommands(context, handles);
  registerServerConfigChangeHandler(context, handles.lspModelProvider);

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
    vscode.window.registerWebviewViewProvider("spec42Library", libraryWebviewProvider)
  );

  context.subscriptions.push(
    vscode.window.registerWebviewViewProvider(
      "sysmlFeatureInspectorView",
      featureInspectorProvider,
      { webviewOptions: { retainContextWhenHidden: true } }
    )
  );

  logStartupPhase("activate:complete");
}

export function deactivate(): Thenable<void> | undefined {
  deactivateWorkspaceIndexing();
  return deactivateLanguageClient();
}

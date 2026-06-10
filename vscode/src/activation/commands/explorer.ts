import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import {
  ExampleTreeItem,
  ExamplesViewProvider,
  metadataForExample,
} from "../../examples/examplesViewProvider";
import {
  ModelExplorerProvider,
  ModelTreeItem,
} from "../../explorer/modelExplorerProvider";
import { log, logError } from "../../logger";
import { hasWorkspaceFolder } from "../../providers/lspModelProvider";
import type { LspClientHandles } from "../lspClient";
import { isSysmlDoc } from "../configBridge";
import {
  debugSyncModelExplorerSelection,
  ensureWorkspaceModelLoaded,
  getDebugExtensionState,
  getModelExplorerProvider,
  reloadWorkspaceModel,
  scheduleActiveDocumentExplorerRefresh,
  scheduleWorkspaceExplorerPendingOnRestart,
  shouldShowModelExplorerContext,
} from "../workspaceIndexing";
import {
  summarizeActiveFileSysmlDiagnostics,
  summarizeWorkspaceSysmlDiagnostics,
} from "../../diagnostics/workspaceDiagnostics";
import { getConfigStringArray } from "../configBridge";
import { VisualizationPanel } from "../../visualization/visualizationPanel";

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

function resolveAdditionalExamplesRoots(extensionPath: string): vscode.Uri[] {
  const repoExamples = path.resolve(extensionPath, "..", "examples");
  const extensionExamples = path.resolve(extensionPath, "examples");

  if (fs.existsSync(repoExamples)) {
    return [vscode.Uri.file(repoExamples)];
  }
  if (fs.existsSync(extensionExamples)) {
    return [vscode.Uri.file(extensionExamples)];
  }
  return [];
}

function exampleFolderUri(arg: vscode.Uri | ExampleTreeItem | undefined): vscode.Uri | undefined {
  if (!arg) {
    return undefined;
  }
  if (arg instanceof vscode.Uri) {
    return arg;
  }
  return arg.folderUri;
}

function exampleMetadata(arg: vscode.Uri | ExampleTreeItem | undefined) {
  if (arg instanceof ExampleTreeItem) {
    return arg.metadata;
  }
  const folder = arg ? path.basename(arg.fsPath) : "";
  return metadataForExample(folder);
}

async function findRecommendedExampleUri(
  context: vscode.ExtensionContext
): Promise<vscode.Uri | undefined> {
  const roots = resolveAdditionalExamplesRoots(context.extensionPath);
  for (const root of roots) {
    const candidate = vscode.Uri.joinPath(root, "timer");
    try {
      const stat = await vscode.workspace.fs.stat(candidate);
      if (stat.type === vscode.FileType.Directory) {
        return candidate;
      }
    } catch {
      // Try the next packaged/repo examples root.
    }
  }
  return undefined;
}

async function openExamplePrimaryFile(
  arg: vscode.Uri | ExampleTreeItem | undefined
): Promise<vscode.TextDocument | undefined> {
  const folderUri = exampleFolderUri(arg);
  if (!folderUri) {
    return undefined;
  }
  const metadata = exampleMetadata(arg);
  const primaryFile = metadata?.primaryFile;
  const fileUri = primaryFile ? vscode.Uri.joinPath(folderUri, primaryFile) : undefined;
  if (!fileUri) {
    await vscode.commands.executeCommand("vscode.openFolder", folderUri, false);
    return undefined;
  }
  try {
    const document = await vscode.workspace.openTextDocument(fileUri);
    await vscode.window.showTextDocument(document, {
      preview: false,
      preserveFocus: false,
    });
    return document;
  } catch (error) {
    logError(`Failed to open example primary file ${fileUri.fsPath}`, error);
    vscode.window.showWarningMessage(
      `Could not open ${primaryFile}; opening the example workspace instead.`
    );
    await vscode.commands.executeCommand("vscode.openFolder", folderUri, false);
    return undefined;
  }
}

async function visualizeExample(
  arg: vscode.Uri | ExampleTreeItem | undefined
): Promise<void> {
  const folderUri = exampleFolderUri(arg);
  if (!folderUri) {
    vscode.window.showWarningMessage("No Spec42 example selected.");
    return;
  }
  await openExamplePrimaryFile(arg);
  await vscode.commands.executeCommand("sysml.visualizeFolder", folderUri);
  const viewId = exampleMetadata(arg)?.recommendedView;
  if (viewId && VisualizationPanel.currentPanel) {
    await vscode.commands.executeCommand("sysml.changeVisualizerView", viewId);
  }
}

export function createExamplesViewProvider(
  extensionPath: string
): ExamplesViewProvider {
  return new ExamplesViewProvider(resolveAdditionalExamplesRoots(extensionPath));
}

export function registerExplorerCommands(
  context: vscode.ExtensionContext,
  handles: LspClientHandles,
  modelExplorerProvider: ModelExplorerProvider,
  examplesViewProvider: ExamplesViewProvider,
  logStartupPhase: (phase: string, extra?: Record<string, unknown>) => void,
  logPerf: (event: string, extra?: Record<string, unknown>) => void
): void {
  const { lspModelProvider } = handles;

  context.subscriptions.push(
    vscode.commands.registerCommand("spec42.examples.openWorkspace", async (arg: vscode.Uri | ExampleTreeItem) => {
      const folderUri = exampleFolderUri(arg);
      if (!folderUri) {
        return;
      }
      await vscode.commands.executeCommand("vscode.openFolder", folderUri, false);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "spec42.examples.openPrimaryFile",
      async (arg: vscode.Uri | ExampleTreeItem) => {
        await openExamplePrimaryFile(arg);
      }
    )
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "spec42.examples.openAndVisualize",
      async (arg: vscode.Uri | ExampleTreeItem) => {
        await visualizeExample(arg);
      }
    )
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("spec42.examples.openRecommended", async () => {
      const folderUri = await findRecommendedExampleUri(context);
      if (!folderUri) {
        vscode.window.showWarningMessage("Could not find the bundled timer example.");
        return;
      }
      await visualizeExample(folderUri);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("spec42.examples.refresh", () => {
      examplesViewProvider.refresh();
    })
  );

  context.subscriptions.push(
    vscode.workspace.onDidChangeWorkspaceFolders(() => {
      examplesViewProvider.refresh();
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.refreshModelTree", async () => {
      lspModelProvider.clearModelCache();
      if ((vscode.workspace.workspaceFolders?.length ?? 0) > 0 && modelExplorerProvider) {
        await reloadWorkspaceModel(
          context,
          modelExplorerProvider,
          logStartupPhase,
          logPerf
        );
      } else {
        modelExplorerProvider?.refresh();
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.switchToByFile", async () => {
      modelExplorerProvider.setWorkspaceViewMode("byFile");
      await ensureWorkspaceModelLoaded(
        modelExplorerProvider,
        context,
        logStartupPhase,
        logPerf
      );
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.switchToSemanticModel", async () => {
      modelExplorerProvider.setWorkspaceViewMode("bySemantic");
      await ensureWorkspaceModelLoaded(
        modelExplorerProvider,
        context,
        logStartupPhase,
        logPerf
      );
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.toggleWorkspaceViewMode", async () => {
      modelExplorerProvider.toggleWorkspaceViewMode();
      await ensureWorkspaceModelLoaded(
        modelExplorerProvider,
        context,
        logStartupPhase,
        logPerf
      );
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

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.copyQualifiedName", async (item: ModelTreeItem) => {
      const qualifiedName = item?.element?.id || item?.element?.name;
      if (!qualifiedName) {
        vscode.window.showWarningMessage("No SysML element selected.");
        return;
      }
      await vscode.env.clipboard.writeText(qualifiedName);
      vscode.window.setStatusBarMessage(`Copied ${qualifiedName}`, 1800);
    })
  );

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
      if (hasWorkspaceFolder()) {
        const summary = summarizeWorkspaceSysmlDiagnostics({
          libraryRootPaths: getResolvedLibraryPaths(),
        });
        vscode.window.showInformationMessage(
          `Validation (workspace): ${summary.errors} error(s), ${summary.warnings} warning(s) across ${summary.totalFiles} file(s).`
        );
        await vscode.commands.executeCommand("workbench.actions.view.problems");
        return;
      }
      const editor = vscode.window.activeTextEditor;
      const doc = editor?.document;
      if (!doc || !isSysmlDoc(doc)) {
        vscode.window.showWarningMessage("No SysML/KerML document is active.");
        return;
      }
      const summary = summarizeActiveFileSysmlDiagnostics(doc);
      vscode.window.showInformationMessage(
        `Validation: ${summary.errors} error(s), ${summary.warnings} warning(s).`
      );
      await vscode.commands.executeCommand("workbench.actions.view.problems");
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.showModelExplorer", async () => {
      await vscode.commands.executeCommand("workbench.view.extension.spec42");
      await vscode.commands.executeCommand(
        "setContext",
        "sysml.modelLoaded",
        shouldShowModelExplorerContext(getModelExplorerProvider())
      );
      await vscode.commands.executeCommand("sysmlModelExplorer.focus");
      if (hasWorkspaceFolder()) {
        await ensureWorkspaceModelLoaded(
          getModelExplorerProvider(),
          context,
          logStartupPhase,
          logPerf
        );
      } else if (getModelExplorerProvider()?.isWorkspaceBacked()) {
        getModelExplorerProvider()?.refresh();
      } else {
        scheduleActiveDocumentExplorerRefresh("showModelExplorer");
      }
    })
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

    vscode.commands.registerCommand(
      "sysml.debug.syncModelExplorerSelection",
      async (targetEditor?: vscode.TextEditor) => {
        await debugSyncModelExplorerSelection(
          targetEditor,
          logPerf,
          context,
          logStartupPhase
        );
      }
    ),

    vscode.commands.registerCommand("sysml.debug.getExtensionState", () => {
      return getDebugExtensionState();
    }),

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
    )
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
}

export function onRestartServerComplete(): void {
  if (hasWorkspaceFolder()) {
    scheduleWorkspaceExplorerPendingOnRestart();
  } else {
    scheduleActiveDocumentExplorerRefresh("restartServer");
  }
}

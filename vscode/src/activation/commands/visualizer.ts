import * as path from "path";
import * as vscode from "vscode";
import { dumpGraphForGeneralView } from "../../graphDump";
import { log, logError, showChannel } from "../../logger";
import { graphScopesForContext } from "../../providers/lspModelProvider";
import type { LspModelProvider } from "../../providers/lspModelProvider";
import { ModelTreeItem } from "../../explorer/modelExplorerProvider";
import {
  RESTORE_STATE_KEY,
  VisualizationPanel,
  VisualizerRestoreState,
} from "../../visualization/visualizationPanel";
import { SYSML_ENABLED_VIEWS } from "../../visualization/webview/constants";
import { configureVisualizerWebview, getWebviewHtml } from "../../visualization/htmlBuilder";
import { waitForVisualizerRender } from "../../visualization/renderTracker";
import type { RenderOutcome } from "../../visualization/renderContract";
import { getConfigNumber, isSysmlDoc } from "../configBridge";
import { getLanguageClient, isLanguageClientReady, type LspClientHandles } from "../lspClient";
import { getModelExplorerProvider } from "../workspaceIndexing";

const EXTENSION_ID = "Elan8.spec42";

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

export function registerVisualizerPanelSerializer(
  context: vscode.ExtensionContext,
  lspModelProvider: LspModelProvider
): void {
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
          configureVisualizerWebview(panel.webview, context.extensionUri);
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
          configureVisualizerWebview(panel.webview, context.extensionUri);
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
}

export function registerVisualizerCommands(
  context: vscode.ExtensionContext,
  handles: LspClientHandles
): void {
  const { lspModelProvider } = handles;

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.showVisualizer", async () => {
      if (!getLanguageClient() || !isLanguageClientReady()) {
        vscode.window.showErrorMessage("SysML language server is not running.");
        return;
      }
      let editor =
        vscode.window.activeTextEditor ??
        vscode.window.visibleTextEditors.find(
          (e) =>
            (e.document.languageId === "sysml" || e.document.languageId === "kerml") &&
            !e.document.isClosed
        );
      if (
        editor &&
        editor.document.languageId !== "sysml" &&
        editor.document.languageId !== "kerml"
      ) {
        editor = undefined;
      }
      if (!editor) {
        editor = vscode.window.visibleTextEditors.find(
          (e) =>
            (e.document.languageId === "sysml" || e.document.languageId === "kerml") &&
            !e.document.isClosed
        );
      }
      if (!editor) {
        vscode.window.showWarningMessage("No SysML/KerML document is open. Open a .sysml or .kerml file first.");
        return;
      }
      try {
        VisualizationPanel.createOrShow(
          context,
          editor.document,
          undefined,
          lspModelProvider
        );
      } catch (error) {
        logError("Failed to open SysML visualizer", error);
        void vscode.window.showErrorMessage(
          `Failed to open visualizer: ${error instanceof Error ? error.message : String(error)}`
        );
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "sysml.visualizeFolder",
      async (uri: vscode.Uri, selectedUris?: vscode.Uri[]) => {
        if (!getLanguageClient()) {
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
    vscode.commands.registerCommand(
      "sysml.visualizePackage",
      async (item: ModelTreeItem) => {
        if (!item || !getLanguageClient()) {
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
        const modelExplorerProvider = getModelExplorerProvider();
        const isWorkspace = modelExplorerProvider?.isWorkspaceBacked() ?? false;
        const workspaceUris = isWorkspace
          ? modelExplorerProvider?.getWorkspaceFileUris()
          : undefined;

        const document = await vscode.workspace.openTextDocument(fileUri);

        if (isWorkspace && workspaceUris && workspaceUris.length > 1) {
          const openDocs: vscode.TextDocument[] = [];
          let combinedContent = "";
          const fileNames: string[] = [];
          for (const workspaceUri of workspaceUris) {
            try {
              const doc = await vscode.workspace.openTextDocument(workspaceUri);
              openDocs.push(doc);
              const fileName = workspaceUri.fsPath.split(/[/\\]/).pop() ?? "";
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
        const view = enabledViews.has(selectedViewId) ? selectedViewId : "general-view";
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
    vscode.commands.registerCommand("sysml.clearCache", async () => {
      if (!getLanguageClient()) {
        vscode.window.showErrorMessage("SysML language server is not running.");
        return;
      }
      const result = await lspModelProvider.clearCache();
      if (result) {
        const total = result.documents + result.symbolTables + result.semanticTokens;
        vscode.window.showInformationMessage(
          `SysML: Cleared ${total} cache entries (${result.documents} docs, ${result.symbolTables} symbols)`
        );
        getModelExplorerProvider()?.refresh();
        VisualizationPanel.currentPanel?.refresh();
      } else {
        vscode.window.showWarningMessage(
          "SysML: Could not clear cache (server may not be ready)."
        );
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.showOutput", () => {
      showChannel();
    }),

    vscode.commands.registerCommand("sysml.debug.disposeVisualizer", () => {
      VisualizationPanel.currentPanel?.dispose();
    }),

    vscode.commands.registerCommand("sysml.debug.exportVisualizerDiagramForTest", () => {
      VisualizationPanel.currentPanel
        ?.getWebview()
        ?.postMessage({ command: "exportDiagramForTest" });
    }),

    vscode.commands.registerCommand("sysml.debug.clearVisualizerPackageSelection", () => {
      VisualizationPanel.currentPanel?.clearPackageSelection();
    }),

    vscode.commands.registerCommand(
      "sysml.debug.selectVisualizerPackage",
      (packageName: string) => {
        VisualizationPanel.currentPanel?.selectPackage(packageName);
      }
    ),

    vscode.commands.registerCommand(
      "sysml.debug.postVisualizerMessage",
      (message: Record<string, unknown>) => {
        VisualizationPanel.currentPanel?.getWebview()?.postMessage(message);
      }
    ),

    vscode.commands.registerCommand(
      "sysml.debug.getVisualizationForTests",
      async (
        workspaceRootUri: string,
        view: string,
        selectedView?: string
      ) => {
        return await lspModelProvider.getVisualization(
          workspaceRootUri,
          view,
          selectedView
        );
      }
    ),

    vscode.commands.registerCommand(
      "sysml.debug.exportInterconnectionPipeline",
      async (
        workspaceRootUri: string,
        view: string,
        selectedView?: string
      ) => {
        const visualization = await lspModelProvider.getVisualization(
          workspaceRootUri,
          view,
          selectedView
        );
        const { exportInterconnectionPipeline } = await import(
          "@spec42/diagram-renderer/pipeline-export"
        );
        return exportInterconnectionPipeline({
          ...visualization,
          view,
        } as Record<string, unknown>);
      }
    ),

    vscode.commands.registerCommand(
      "sysml.debug.seedVisualizerFromLspForTests",
      async (
        workspaceRootUri: string,
        viewId: string,
        selectedView?: string
      ) => {
        const panel = VisualizationPanel.currentPanel;
        if (!panel) {
          throw new Error("Visualizer panel is not open");
        }
        panel.prepareViewForTests(viewId, selectedView);
        const visualization = await lspModelProvider.getVisualization(
          workspaceRootUri,
          viewId,
          selectedView
        );
        const viewCandidates = visualization.viewCandidates ?? [];
        const preparedView = visualization.preparedView;
        const summary = {
          modelReady: visualization.modelReady !== false,
          preparedViewNodes: preparedView?.nodes?.length ?? 0,
          preparedViewEdges: preparedView?.edges?.length ?? 0,
          graphNodes: visualization.graph?.nodes?.length ?? 0,
          viewCandidateCount: viewCandidates.length,
          viewCandidateIds: viewCandidates.map((candidate) => candidate.id),
          viewCandidateNames: viewCandidates.map((candidate) => candidate.name),
          selectedView: visualization.selectedView,
          selectedViewName: visualization.selectedViewName,
          emptyStateMessage: visualization.emptyStateMessage,
          requestedViewId: viewId,
          requestedSelectedView: selectedView,
        };
        try {
          // eslint-disable-next-line no-console
          console.log(
            `[spec42-test][seedVisualizerFromLsp] ${JSON.stringify(summary)}`
          );
        } catch {
          // ignore serialization issues in test logging
        }
        return summary;
      }
    ),

    vscode.commands.registerCommand(
      "sysml.debug.waitForVisualizerRender",
      async (options?: {
        view?: string;
        outcome?: string | string[];
        minGraphNodes?: number;
        updateId?: string;
        timeoutMs?: number;
      }) => {
        const outcome = options?.outcome;
        return await waitForVisualizerRender({
          view: options?.view,
          outcome: outcome as RenderOutcome | RenderOutcome[] | undefined,
          minGraphNodes: options?.minGraphNodes,
          updateId: options?.updateId,
          timeoutMs: options?.timeoutMs,
        });
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
          graphScopesForContext(),
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
}

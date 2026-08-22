import * as vscode from "vscode";
import type { LspClientHandles } from "./lspClient";
import { isClientNotRunningError, resolvedReference } from "../providers/lspModelProvider";
import type { FeatureInspectorViewProvider } from "../inspector/featureInspectorViewProvider";
import { isLanguageClientReady } from "./lspClient";
import { isSysmlDoc } from "./configBridge";
import { logError } from "../logger";
import { updateStatusBar } from "./statusBar";

let selectionTimer: ReturnType<typeof setTimeout> | undefined;

async function syncInspector(
  handles: LspClientHandles,
  provider: FeatureInspectorViewProvider,
  editor: vscode.TextEditor,
  position: vscode.Position,
  logPerf: (event: string, extra?: Record<string, unknown>) => void
): Promise<void> {
  if (!provider.isVisible() || provider.isPinned() || !isSysmlDoc(editor.document) || !isLanguageClientReady()) return;
  const startedAt = Date.now();
  try {
    const result = await handles.lspModelProvider.getFeatureInspector(editor.document.uri.toString(), position);
    provider.update(result);
    logPerf("selectionSync:featureInspector", {
      uri: editor.document.uri.toString(),
      totalMs: Date.now() - startedAt,
      selectionKind: result.selection.kind,
      elementId: (resolvedReference(result) ?? result.containingElement)?.id,
    });
  } catch (error) {
    if (!isClientNotRunningError(error)) logError("Feature inspector cursor sync failed", error);
  }
}

export function registerWorkspaceIndexing(
  context: vscode.ExtensionContext,
  handles: LspClientHandles,
  inspectorProvider: FeatureInspectorViewProvider,
  _logStartupPhase: (phase: string, extra?: Record<string, unknown>) => void,
  logPerf: (event: string, extra?: Record<string, unknown>) => void
): void {
  const syncActive = () => {
    const editor = vscode.window.activeTextEditor;
    if (editor && isSysmlDoc(editor.document)) void syncInspector(handles, inspectorProvider, editor, editor.selection.active, logPerf);
  };
  context.subscriptions.push(inspectorProvider.onResumeRequested(syncActive));
  context.subscriptions.push(vscode.window.onDidChangeTextEditorSelection((event) => {
    if (!isSysmlDoc(event.textEditor.document)) return;
    if (selectionTimer) clearTimeout(selectionTimer);
    const position = event.selections[0]?.active ?? event.textEditor.selection.active;
    selectionTimer = setTimeout(() => {
      selectionTimer = undefined;
      void syncInspector(handles, inspectorProvider, event.textEditor, position, logPerf);
    }, 150);
  }));
  context.subscriptions.push(vscode.window.onDidChangeActiveTextEditor(() => {
    updateStatusBar(context);
    syncActive();
  }));
  updateStatusBar(context);
  syncActive();
}

export function resetSemanticIndexTracking(): void {}
export function deactivateWorkspaceIndexing(): void {
  if (selectionTimer) clearTimeout(selectionTimer);
  selectionTimer = undefined;
}

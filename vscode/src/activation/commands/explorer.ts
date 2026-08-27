import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { ExampleTreeItem, ExamplesViewProvider, metadataForExample } from "../../examples/examplesViewProvider";
import { log, logError } from "../../logger";
import type { LspClientHandles } from "../lspClient";
import { isSysmlDoc, getConfigStringArray } from "../configBridge";
import { summarizeActiveFileSysmlDiagnostics, summarizeWorkspaceSysmlDiagnostics } from "../../diagnostics/workspaceDiagnostics";
import { hasWorkspaceFolder } from "../../providers/lspModelProvider";

function roots(extensionPath: string): vscode.Uri[] {
  for (const candidate of [path.resolve(extensionPath, "..", "examples"), path.resolve(extensionPath, "examples")]) {
    if (fs.existsSync(candidate)) return [vscode.Uri.file(candidate)];
  }
  return [];
}
function folderUri(arg: vscode.Uri | ExampleTreeItem | undefined): vscode.Uri | undefined { return arg instanceof vscode.Uri ? arg : arg?.folderUri; }
async function openExample(arg: vscode.Uri | ExampleTreeItem | undefined): Promise<void> {
  const folder = folderUri(arg); if (!folder) return;
  const metadata = arg instanceof ExampleTreeItem ? arg.metadata : metadataForExample(path.basename(folder.fsPath));
  if (!metadata?.primaryFile) { await vscode.commands.executeCommand("vscode.openFolder", folder, false); return; }
  try { await vscode.window.showTextDocument(await vscode.workspace.openTextDocument(vscode.Uri.joinPath(folder, metadata.primaryFile)), { preview: false }); }
  catch (error) { logError("Failed to open example", error); await vscode.commands.executeCommand("vscode.openFolder", folder, false); }
}
export function createExamplesViewProvider(extensionPath: string): ExamplesViewProvider { return new ExamplesViewProvider(roots(extensionPath)); }

export function registerExplorerCommands(context: vscode.ExtensionContext, _handles: LspClientHandles, examples: ExamplesViewProvider): void {
  context.subscriptions.push(
    vscode.commands.registerCommand("spec42.examples.openWorkspace", async (arg: vscode.Uri | ExampleTreeItem) => { const folder = folderUri(arg); if (folder) await vscode.commands.executeCommand("vscode.openFolder", folder, false); }),
    vscode.commands.registerCommand("spec42.examples.openPrimaryFile", openExample),
    vscode.commands.registerCommand("spec42.examples.openRecommended", async () => { const base = roots(context.extensionPath)[0]; if (base) await openExample(vscode.Uri.joinPath(base, "timer")); }),
    vscode.commands.registerCommand("spec42.examples.refresh", () => examples.refresh()),
    vscode.workspace.onDidChangeWorkspaceFolders(() => examples.refresh()),
    vscode.commands.registerCommand("sysml.formatDocument", async () => { if (isSysmlDoc(vscode.window.activeTextEditor?.document)) await vscode.commands.executeCommand("editor.action.formatDocument"); }),
    vscode.commands.registerCommand("sysml.validateModel", async () => {
      const doc = vscode.window.activeTextEditor?.document;
      if (hasWorkspaceFolder()) {
        const configured = getConfigStringArray("libraryPaths") ?? [];
        const root = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath ?? "";
        const summary = summarizeWorkspaceSysmlDiagnostics({ libraryRootPaths: configured.map(p => path.isAbsolute(p) ? p : path.resolve(root, p)) });
        vscode.window.showInformationMessage(`Validation (workspace): ${summary.errors} error(s), ${summary.warnings} warning(s) across ${summary.totalFiles} file(s).`);
      } else if (doc && isSysmlDoc(doc)) {
        const summary = summarizeActiveFileSysmlDiagnostics(doc);
        vscode.window.showInformationMessage(`Validation: ${summary.errors} error(s), ${summary.warnings} warning(s).`);
      }
      await vscode.commands.executeCommand("workbench.actions.view.problems");
    }),
    vscode.commands.registerCommand("sysml.showTypeHierarchy", () => vscode.commands.executeCommand("editor.showTypeHierarchy")),
    vscode.commands.registerCommand("sysml.showCallHierarchy", () => vscode.commands.executeCommand("editor.showCallHierarchy")),
    vscode.commands.registerCommand("spec42.showInheritedAttributeInfo", () => {}),
    vscode.commands.registerCommand("spec42.showReferencesCount", async (uri: vscode.Uri, position: vscode.Position) => {
      const targetUri = uri ?? vscode.window.activeTextEditor?.document.uri; const targetPosition = position ?? vscode.window.activeTextEditor?.selection.active;
      if (!targetUri || !targetPosition) return;
      try { const refs = await vscode.commands.executeCommand<vscode.Location[]>("vscode.executeReferenceProvider", targetUri, targetPosition) ?? []; if (refs.length) await vscode.commands.executeCommand("editor.action.showReferences", targetUri, targetPosition, refs); else vscode.window.showInformationMessage("No references found."); }
      catch (error) { logError("showReferencesCount command failed", error); }
    })
  );
  log("Editor and example commands registered");
}

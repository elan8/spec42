import * as fs from "fs/promises";
import * as path from "path";
import * as vscode from "vscode";
import type { LspClientHandles } from "../activation/lspClient";
import {
  isPathInsideWorkspace,
  parseLspGenerationResult,
  parseSourceNavigation,
  readSvgMetadata,
  selectSingleSvg,
  validateStandaloneSvg,
} from "./stateTransitionViewerCore";

type Artifact = {
  svg: string;
  digest: string;
  viewName: string;
  modulePrepareMs: number;
  guestExecutionUs: number;
  preparedReused: boolean;
  compilationCacheHits: number;
  compilationCacheMisses: number;
  compilationCacheError: string | null;
};
export type StateTransitionViewerDependencies = {
  resolvePluginPath: (context: vscode.ExtensionContext) => string;
};

function pluginPath(context: vscode.ExtensionContext): string {
  const configured = vscode.workspace.getConfiguration("spec42.stateTransitionViewer").get<string>("pluginPath", "").trim();
  if (configured) {
    const root = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath ?? process.cwd();
    return path.isAbsolute(configured) ? configured : path.resolve(root, configured);
  }
  return path.join(context.extensionPath, "generators", "state-transition-view.wasm");
}

const defaultDependencies: StateTransitionViewerDependencies = {
  resolvePluginPath: pluginPath,
};

function prepareCacheLabel(artifact: Artifact): string {
  if (artifact.preparedReused) return " (memory cache)";
  if (artifact.compilationCacheHits > 0) return " (native cache)";
  if (artifact.compilationCacheError) return " (cache unavailable)";
  if (artifact.compilationCacheMisses > 0) return " (compiled)";
  return "";
}

async function requireSavedModel(): Promise<vscode.TextDocument | undefined> {
  const document = vscode.window.activeTextEditor?.document;
  if (!document || document.languageId !== "sysml" || document.uri.scheme !== "file") {
    await vscode.window.showErrorMessage("Open a saved SysML file before rendering a state-transition view.");
    return undefined;
  }
  const dirty = vscode.workspace.textDocuments.filter(
    (candidate) => candidate.isDirty && (candidate.languageId === "sysml" || candidate.languageId === "kerml")
  );
  if (dirty.length > 0) {
    const choice = await vscode.window.showWarningMessage(
      "State-transition rendering uses the saved workspace. Save all changed SysML/KerML files?",
      { modal: true },
      "Save All"
    );
    if (choice !== "Save All") return undefined;
    const saved = await Promise.all(dirty.map((candidate) => candidate.save()));
    if (saved.some((ok) => !ok)) {
      await vscode.window.showErrorMessage("Could not save every changed model file; rendering was cancelled.");
      return undefined;
    }
  }
  return document;
}

export class StateTransitionViewer {
  private panel: vscode.WebviewPanel | undefined;
  private lastArtifact: Artifact | undefined;
  private generation = 0;
  private activeAbort: AbortController | undefined;

  constructor(
    private readonly context: vscode.ExtensionContext,
    private readonly handles: LspClientHandles,
    private readonly dependencies: StateTransitionViewerDependencies = defaultDependencies
  ) {}

  async open(): Promise<void> {
    const document = await requireSavedModel();
    if (!document) return;
    this.activeAbort?.abort();
    const abort = new AbortController();
    this.activeAbort = abort;
    const current = ++this.generation;
    try {
      const artifact = await this.generate(document, abort.signal);
      if (current !== this.generation) return;
      this.lastArtifact = artifact;
      this.show(artifact);
    } catch (error) {
      if (current !== this.generation) return;
      const message = error instanceof Error ? error.message : String(error);
      if (this.panel && this.lastArtifact) this.show(this.lastArtifact, message);
      await vscode.window.showErrorMessage(`State-transition rendering failed: ${message}`);
    }
  }

  private async generate(document: vscode.TextDocument, signal: AbortSignal): Promise<Artifact> {
    const plugin = this.dependencies.resolvePluginPath(this.context);
    let module: Buffer;
    try { module = await fs.readFile(plugin); }
    catch { throw new Error(`Compatible diagram plugin not found at ${plugin}. Configure spec42.stateTransitionViewer.pluginPath.`); }
    if (signal.aborted) throw new Error("generation was cancelled");
    const result = parseLspGenerationResult(await this.handles.client.sendRequest("spec42/generate", {
      generatorBase64: module.toString("base64"),
      modelUri: document.uri.toString(),
      args: [],
    }));
    if (signal.aborted) throw new Error("generation was cancelled");
    const svgName = selectSingleSvg(result.artifacts.map((artifact) => artifact.path));
    const selected = result.artifacts.find((artifact) => artifact.path === svgName);
    if (!selected) throw new Error("Spec42 omitted the selected SVG artifact.");
    const svg = validateStandaloneSvg(Buffer.from(selected.content).toString("utf8"));
    const metadata = readSvgMetadata(svg);
    if (metadata.modelDigest !== result.modelDigest) {
      throw new Error("Generated SVG model digest does not match the current LSP publication.");
    }
    return {
      svg,
      digest: result.modelDigest,
      viewName: metadata.viewName,
      modulePrepareMs: result.timings.modulePrepareMs,
      guestExecutionUs: result.timings.guestExecutionUs,
      preparedReused: result.timings.preparedReused,
      compilationCacheHits: result.timings.compilationCacheHits,
      compilationCacheMisses: result.timings.compilationCacheMisses,
      compilationCacheError: result.timings.compilationCacheError,
    };
  }

  private show(artifact: Artifact, error?: string): void {
    if (!this.panel) {
      this.panel = vscode.window.createWebviewPanel(
        "spec42.stateTransitionView",
        "State Transition View",
        vscode.ViewColumn.Beside,
        { enableScripts: true, retainContextWhenHidden: true }
      );
      this.panel.onDidDispose(() => { this.panel = undefined; });
      this.panel.webview.onDidReceiveMessage((message) => this.navigate(message));
    }
    this.panel.title = `State Transition: ${artifact.viewName}`;
    this.panel.webview.html = this.html(this.panel.webview, artifact, error);
    this.panel.reveal(vscode.ViewColumn.Beside, true);
  }

  private async navigate(message: unknown): Promise<void> {
    if (!message || typeof message !== "object" || (message as { type?: unknown }).type !== "openSource") return;
    const target = parseSourceNavigation((message as { target?: unknown }).target);
    if (!target) return;
    let uri: vscode.Uri;
    try { uri = vscode.Uri.parse(target.uri, true); } catch { return; }
    const roots = vscode.workspace.workspaceFolders?.map((folder) => folder.uri.fsPath) ?? [];
    if (uri.scheme !== "file" || !isPathInsideWorkspace(uri.fsPath, roots)) return;
    try {
      const document = await vscode.workspace.openTextDocument(uri);
      const maxLine = Math.max(0, document.lineCount - 1);
      if (target.startLine > maxLine || target.endLine > maxLine) return;
      const range = new vscode.Range(target.startLine, target.startCharacter, target.endLine, target.endCharacter);
      const editor = await vscode.window.showTextDocument(document, { preview: true });
      editor.selection = new vscode.Selection(range.start, range.start);
      editor.revealRange(range, vscode.TextEditorRevealType.InCenterIfOutsideViewport);
    } catch { /* Invalid or unavailable provenance remains inert. */ }
  }

  private html(webview: vscode.Webview, artifact: Artifact, error?: string): string {
    const nonce = `${Date.now()}${Math.random().toString(36).slice(2)}`;
    const escaped = (value: string) => value.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");
    return `<!doctype html><html><head><meta charset="utf-8">
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${webview.cspSource} 'unsafe-inline'; script-src 'nonce-${nonce}'; img-src ${webview.cspSource};">
<style>body{padding:0;margin:0;color:var(--vscode-foreground);background:var(--vscode-editor-background)}header{padding:8px 12px;border-bottom:1px solid var(--vscode-panel-border);font:12px var(--vscode-font-family)}.error{color:var(--vscode-errorForeground);font-weight:600}.canvas{padding:12px;overflow:auto}.canvas svg{display:block;max-width:100%;height:auto;margin:auto}</style></head><body>
<header><strong>${escaped(artifact.viewName)}</strong> · model ${escaped(artifact.digest)} · saved workspace · prepare ${artifact.modulePrepareMs} ms${prepareCacheLabel(artifact)} · execute ${(artifact.guestExecutionUs / 1000).toFixed(2)} ms${error ? ` · <span class="error">stale: ${escaped(error)}</span>` : ""}</header>
<main class="canvas">${artifact.svg}</main>
<script nonce="${nonce}">document.querySelector('.canvas').addEventListener('click',event=>{event.preventDefault();const node=event.target.closest('[data-source-uri]');if(!node)return;const n=k=>Number(node.dataset[k]);acquireVsCodeApi().postMessage({type:'openSource',target:{uri:node.dataset.sourceUri,startLine:n('sourceStartLine'),startCharacter:n('sourceStartCharacter'),endLine:n('sourceEndLine'),endCharacter:n('sourceEndCharacter')}});});</script>
</body></html>`;
  }
}

export function registerStateTransitionViewer(
  context: vscode.ExtensionContext,
  handles: LspClientHandles
): void {
  const viewer = new StateTransitionViewer(context, handles);
  context.subscriptions.push(vscode.commands.registerCommand("spec42.diagram.openStateTransitionView", () => viewer.open()));
}

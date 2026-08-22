import * as fs from "fs/promises";
import * as path from "path";
import * as vscode from "vscode";
import type { LspClientHandles } from "../activation/lspClient";
import {
  type DiagramProduct,
  type DiagramViewCatalog,
  type DiagramViewId,
  diagramViewsForDocument,
  isPathInsideWorkspace,
  parseDiagramProduct,
  parseDiagramViewCatalog,
  parseLspGenerationResult,
  parseSourceNavigation,
  selectSingleDiagramJson,
  visibleSourceColumn,
} from "./diagramViewerCore";

type RenderedArtifact = {
  product: DiagramProduct;
  productJson: string;
  modulePrepareMs: number;
  guestExecutionUs: number;
  preparedReused: boolean;
  compilationCacheHits: number;
  compilationCacheMisses: number;
  compilationCacheError: string | null;
};

export type DiagramViewerDependencies = {
  resolvePluginPath: (context: vscode.ExtensionContext) => string;
};

function pluginPath(context: vscode.ExtensionContext): string {
  const configured = vscode.workspace.getConfiguration("spec42.diagramViewer").get<string>("pluginPath", "").trim();
  if (configured) {
    const root = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath ?? process.cwd();
    return path.isAbsolute(configured) ? configured : path.resolve(root, configured);
  }
  return path.join(context.extensionPath, "generators", "diagram.wasm");
}

const defaultDependencies: DiagramViewerDependencies = { resolvePluginPath: pluginPath };

function prepareCacheLabel(artifact: RenderedArtifact): string {
  if (artifact.preparedReused) return " (memory cache)";
  if (artifact.compilationCacheHits > 0) return " (native cache)";
  if (artifact.compilationCacheError) return " (cache unavailable)";
  if (artifact.compilationCacheMisses > 0) return " (compiled)";
  return "";
}

async function requireSavedModel(): Promise<vscode.TextDocument | undefined> {
  const document = vscode.window.activeTextEditor?.document;
  if (!document || document.languageId !== "sysml" || document.uri.scheme !== "file") {
    await vscode.window.showErrorMessage("Open a saved SysML file before rendering a diagram.");
    return undefined;
  }
  const dirty = vscode.workspace.textDocuments.filter(
    (candidate) => candidate.isDirty && (candidate.languageId === "sysml" || candidate.languageId === "kerml")
  );
  if (dirty.length === 0) return document;
  const choice = await vscode.window.showWarningMessage(
    "Diagram generation uses the saved workspace. Save all changed SysML/KerML files?",
    { modal: true },
    "Save All"
  );
  if (choice !== "Save All") return undefined;
  const saved = await Promise.all(dirty.map((candidate) => candidate.save()));
  if (saved.some((ok) => !ok)) {
    await vscode.window.showErrorMessage("Could not save every changed model file; generation was cancelled.");
    return undefined;
  }
  return document;
}

export class DiagramViewer {
  private panel: vscode.WebviewPanel | undefined;
  private lastArtifact: RenderedArtifact | undefined;
  private generation = 0;
  private activeAbort: AbortController | undefined;

  constructor(
    private readonly context: vscode.ExtensionContext,
    private readonly handles: LspClientHandles,
    private readonly dependencies: DiagramViewerDependencies = defaultDependencies
  ) {}

  async open(): Promise<void> {
    const document = await requireSavedModel();
    if (!document) return;
    const diagramCatalog = parseDiagramViewCatalog(await this.handles.client.sendRequest(
      "spec42/diagramViews",
      { modelUri: document.uri.toString() }
    ));
    const availableViews = diagramViewsForDocument(diagramCatalog, document.uri.toString());
    if (availableViews.length === 0) {
      await vscode.window.showInformationMessage("The active file does not author a supported diagram view.");
      return;
    }
    const selectedView = await vscode.window.showQuickPick(
      availableViews.map((view) => ({
        label: view.label,
        description: view.queryStatus === "implemented" ? "typed query available" : "typed query stub",
        view,
      })),
      { placeHolder: "Select a SysML diagram view" }
    );
    if (!selectedView) return;

    this.activeAbort?.abort();
    const abort = new AbortController();
    this.activeAbort = abort;
    const current = ++this.generation;
    try {
      const selection = await this.resolveSelection(document, selectedView.view.id, diagramCatalog);
      if (!selection || abort.signal.aborted || current !== this.generation) return;
      const artifact = await this.generate(document, selectedView.view.id, selection.modelDigest, selection.handle, abort.signal);
      if (current !== this.generation) return;
      this.lastArtifact = artifact;
      this.show(artifact);
    } catch (error) {
      if (current !== this.generation) return;
      const message = error instanceof Error ? error.message : String(error);
      if (this.panel && this.lastArtifact) this.show(this.lastArtifact, message);
      await vscode.window.showErrorMessage(`Diagram generation failed: ${message}`);
    }
  }

  private async resolveSelection(
    document: vscode.TextDocument,
    view: DiagramViewId,
    catalog: DiagramViewCatalog
  ): Promise<{ modelDigest: string; handle: string } | undefined> {
    const choices = catalog.views.filter((candidate) =>
      candidate.kind === view && candidate.source.uri === document.uri.toString());
    if (choices.length === 0) throw new Error("The active file does not author the selected diagram view kind.");
    if (choices.length === 1) return { modelDigest: catalog.modelDigest, handle: choices[0].handle };
    const picked = await vscode.window.showQuickPick(
      choices.map((candidate) => ({
        label: candidate.name,
        description: candidate.kind,
        detail: candidate.reference.kind === "qualified-name"
          ? `${candidate.reference.document}#${candidate.reference.qualifiedName}`
          : candidate.reference.kind,
        candidate,
      })),
      { placeHolder: "Select an authored diagram view", matchOnDescription: true, matchOnDetail: true }
    );
    return picked ? { modelDigest: catalog.modelDigest, handle: picked.candidate.handle } : undefined;
  }

  private async generate(
    document: vscode.TextDocument,
    view: DiagramViewId,
    expectedModelDigest: string | undefined,
    handle: string | undefined,
    signal: AbortSignal
  ): Promise<RenderedArtifact> {
    const plugin = this.dependencies.resolvePluginPath(this.context);
    let module: Buffer;
    try { module = await fs.readFile(plugin); }
    catch { throw new Error(`Compatible diagram plugin not found at ${plugin}. Configure spec42.diagramViewer.pluginPath.`); }
    if (signal.aborted) throw new Error("generation was cancelled");
    const result = parseLspGenerationResult(await this.handles.client.sendRequest("spec42/generate", {
      generatorBase64: module.toString("base64"),
      modelUri: document.uri.toString(),
      args: handle ? [handle] : [],
      ...(expectedModelDigest ? { expectedModelDigest } : {}),
    }));
    if (signal.aborted) throw new Error("generation was cancelled");
    const artifactName = selectSingleDiagramJson(result.artifacts.map((artifact) => artifact.path));
    const selected = result.artifacts.find((artifact) => artifact.path === artifactName);
    if (!selected) throw new Error("Spec42 omitted the selected diagram artifact.");
    const productJson = Buffer.from(selected.content).toString("utf8");
    const product = parseDiagramProduct(productJson);
    if (product.modelDigest !== result.modelDigest) {
      throw new Error("Generated diagram model digest does not match the current LSP publication.");
    }
    if (product.selectedView.kind !== view) throw new Error("Generated diagram view does not match the requested view.");
    return { product, productJson, ...result.timings };
  }

  async copyJson(): Promise<void> {
    if (!this.lastArtifact) {
      await vscode.window.showInformationMessage("Open a Spec42 diagram before copying its JSON.");
      return;
    }
    await vscode.env.clipboard.writeText(this.lastArtifact.productJson);
    await vscode.window.showInformationMessage("Copied the generated diagram JSON.");
  }

  private show(artifact: RenderedArtifact, error?: string): void {
    if (!this.panel) {
      this.panel = vscode.window.createWebviewPanel(
        "spec42.diagram",
        "SysML Diagram",
        vscode.ViewColumn.Beside,
        { enableScripts: true, retainContextWhenHidden: true }
      );
      this.panel.onDidDispose(() => { this.panel = undefined; });
      this.panel.webview.onDidReceiveMessage((message) => this.navigate(message));
    }
    this.panel.title = artifact.product.selectedView.name;
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
      const existingColumn = visibleSourceColumn(
        uri.toString(),
        vscode.window.visibleTextEditors.map((editor) => ({
          uri: editor.document.uri.toString(),
          viewColumn: editor.viewColumn,
        })),
      );
      const editor = await vscode.window.showTextDocument(document, {
        viewColumn: existingColumn ?? vscode.ViewColumn.Beside,
        preview: false,
      });
      editor.selection = new vscode.Selection(range.start, range.start);
      editor.revealRange(range, vscode.TextEditorRevealType.InCenterIfOutsideViewport);
    } catch { /* Invalid or unavailable provenance remains inert. */ }
  }

  private html(webview: vscode.Webview, artifact: RenderedArtifact, error?: string): string {
    const nonce = `${Date.now()}${Math.random().toString(36).slice(2)}`;
    const escaped = (value: string) => value.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");
    const productJson = artifact.productJson.replace(/</g, "\\u003c");
    const script = webview.asWebviewUri(vscode.Uri.joinPath(this.context.extensionUri, "media", "diagram-viewer.js"));
    const status = artifact.product.completeness.status === "complete"
      ? "complete projection"
      : `incomplete projection (${artifact.product.completeness.reasons.length})`;
    return `<!doctype html><html><head><meta charset="utf-8">
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${webview.cspSource} 'unsafe-inline'; script-src 'nonce-${nonce}' ${webview.cspSource};">
<style>html,body{height:100%}body{padding:0;margin:0;color:var(--vscode-foreground);background:var(--vscode-editor-background);display:flex;flex-direction:column}header{padding:8px 12px;border-bottom:1px solid var(--vscode-panel-border);font:12px var(--vscode-font-family)}.error{color:var(--vscode-errorForeground);font-weight:600}.canvas{flex:1;min-height:0;position:relative}.empty{padding:32px;max-width:720px;color:var(--vscode-descriptionForeground);font:14px var(--vscode-font-family)}.canvas svg{display:block;width:100%;height:100%}</style></head><body>
<header><strong>${escaped(artifact.product.selectedView.name)}</strong> · ${escaped(status)} · model ${escaped(artifact.product.modelDigest)} · prepare ${artifact.modulePrepareMs} ms${prepareCacheLabel(artifact)} · execute ${(artifact.guestExecutionUs / 1000).toFixed(2)} ms${error ? ` · <span class="error">stale: ${escaped(error)}</span>` : ""}</header>
<main id="diagram" class="canvas"></main><script id="diagram-product" type="application/json">${productJson}</script>
<script nonce="${nonce}" src="${script}"></script></body></html>`;
  }
}

export function registerDiagramViewer(context: vscode.ExtensionContext, handles: LspClientHandles): void {
  const viewer = new DiagramViewer(context, handles);
  context.subscriptions.push(
    vscode.commands.registerCommand("spec42.diagram.open", () => viewer.open()),
    vscode.commands.registerCommand("spec42.diagram.copyJson", () => viewer.copyJson()),
  );
}

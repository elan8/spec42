import * as fs from "fs/promises";
import * as path from "path";
import * as vscode from "vscode";
import type { LspClientHandles } from "../activation/lspClient";
import {
  allDiagramViewOptions,
  authoredDiagramViewOptions,
  type DiagramProduct,
  type DiagramViewCatalog,
  type DiagramViewId,
  type DiagramViewOption,
  diagramRenderIsStale,
  diagramViewKindForHandle,
  isPathInsideWorkspace,
  parseDiagramProduct,
  parseDiagramViewCatalog,
  parseLspGenerationResult,
  parseSourceNavigation,
  selectSingleDiagramJson,
  visibleSourceColumn,
} from "./diagramViewerCore";

export const DIAGRAM_VIEW_ID = "spec42DiagramView";

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

/** A payload the webview can draw without any further round trip. */
type RenderMessage = {
  type: "render";
  productJson: string;
  views: Array<{ handle: string; label: string; group: string }>;
  selectedHandle: string;
  header: string;
  incompleteReasons: string[];
  placeholder?: string;
  loading?: boolean;
  error?: string;
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

const PUBLICATION_DEBOUNCE_MS = 250;
const DIGEST_MISMATCH_RETRIES = 2;
const MODEL_GLOB = "**/*.{sysml,kerml}";

function artifactHeader(artifact: RenderedArtifact): string {
  if (artifact.product.completeness.status === "complete") return "Complete projection";
  const count = artifact.product.completeness.reasons.length;
  return `Incomplete projection · ${count} ${count === 1 ? "limitation" : "limitations"}`;
}

function isModelDocument(document: vscode.TextDocument | undefined): document is vscode.TextDocument {
  return (
    !!document &&
    document.uri.scheme === "file" &&
    (document.languageId === "sysml" || document.languageId === "kerml")
  );
}

/**
 * The diagram view in the secondary side bar. Project-scoped: it lists every authored diagram
 * view in the model and regenerates the selected one whenever the publication changes.
 */
export class DiagramViewProvider implements vscode.WebviewViewProvider, vscode.Disposable {
  private view: vscode.WebviewView | undefined;
  private options: DiagramViewOption[] = [];
  private catalogModelDigest: string | undefined;
  private selectedHandle: string | undefined;
  private anchorUri: string | undefined;
  private lastArtifact: RenderedArtifact | undefined;
  private generation = 0;
  private activeAbort: AbortController | undefined;
  private webviewReady = false;
  private pendingRender: RenderMessage | undefined;
  private publicationDebounce: ReturnType<typeof setTimeout> | undefined;
  private firstLoadRetry: ReturnType<typeof setTimeout> | undefined;
  private firstLoadAttempt = 0;
  private emptyCatalogDigest: string | undefined;
  private readonly disposables: vscode.Disposable[] = [];

  constructor(
    private readonly context: vscode.ExtensionContext,
    private readonly handles: LspClientHandles,
    private readonly dependencies: DiagramViewerDependencies = defaultDependencies
  ) {
    this.disposables.push(
      this.handles.client.onNotification("spec42/publicationChanged", (params: unknown) => {
        const digest = (params as { modelDigest?: unknown } | null)?.modelDigest;
        if (typeof digest === "string") this.onPublicationChanged(digest);
      }),
    );
  }

  dispose(): void {
    if (this.publicationDebounce) clearTimeout(this.publicationDebounce);
    if (this.firstLoadRetry) clearTimeout(this.firstLoadRetry);
    this.activeAbort?.abort();
    for (const disposable of this.disposables.splice(0)) disposable.dispose();
  }

  resolveWebviewView(webviewView: vscode.WebviewView): void {
    this.view = webviewView;
    this.webviewReady = false;
    this.pendingRender = undefined;
    webviewView.webview.options = {
      enableScripts: true,
      localResourceRoots: [vscode.Uri.joinPath(this.context.extensionUri, "media")],
    };
    webviewView.webview.html = this.shellHtml(webviewView.webview);
    webviewView.webview.onDidReceiveMessage((message) => this.onWebviewMessage(message), undefined, this.disposables);
    webviewView.onDidChangeVisibility(() => {
      if (webviewView.visible) void this.regenerate("visibility");
    }, undefined, this.disposables);
    webviewView.onDidDispose(() => {
      if (this.view === webviewView) {
        this.view = undefined;
        this.webviewReady = false;
        this.pendingRender = undefined;
        this.activeAbort?.abort();
      }
    }, undefined, this.disposables);
    this.firstLoadAttempt = 0;
    this.postLoading();
    void this.handles.clientReadyPromise
      .then(() => this.regenerate("resolve"))
      .catch(() => this.regenerate("resolve"));
  }

  /** Command entry point — reveal the view and regenerate. */
  async open(): Promise<void> {
    await vscode.commands.executeCommand(`${DIAGRAM_VIEW_ID}.focus`);
    await this.regenerate("manual");
  }

  async copyJson(): Promise<void> {
    if (!this.lastArtifact) {
      await vscode.window.showInformationMessage("Generate a Spec42 diagram before copying its JSON.");
      return;
    }
    await vscode.env.clipboard.writeText(this.lastArtifact.productJson);
    await vscode.window.showInformationMessage("Copied the generated diagram JSON.");
  }

  private onPublicationChanged(publishedModelDigest: string): void {
    if (!this.view) return;
    // A publication signal is worth acting on when it invalidates what we drew, and also when we
    // never got a first render (the initial attempt raced an in-progress index).
    const worthRegenerating =
      !this.lastArtifact ||
      diagramRenderIsStale(this.lastArtifact.product.modelDigest, publishedModelDigest);
    if (!worthRegenerating) return;
    if (this.publicationDebounce) clearTimeout(this.publicationDebounce);
    this.publicationDebounce = setTimeout(() => {
      this.publicationDebounce = undefined;
      void this.regenerate("publication");
    }, PUBLICATION_DEBOUNCE_MS);
  }

  /** The first render can lose a race with workspace indexing; retry with backoff until it
   * lands or the caller triggers a fresh attempt. */
  private scheduleFirstLoadRetry(): void {
    if (this.lastArtifact || this.firstLoadRetry) return;
    if (this.firstLoadAttempt >= 6) return;
    const wait = Math.min(4000, 500 * 2 ** this.firstLoadAttempt);
    this.firstLoadAttempt += 1;
    this.firstLoadRetry = setTimeout(() => {
      this.firstLoadRetry = undefined;
      void this.regenerate("retry");
    }, wait);
  }

  private onWebviewMessage(message: unknown): void {
    if (!message || typeof message !== "object") return;
    const kind = (message as { type?: unknown }).type;
    if (kind === "ready") {
      this.webviewReady = true;
      if (this.pendingRender) {
        void this.view?.webview.postMessage(this.pendingRender);
        this.pendingRender = undefined;
      }
      return;
    }
    if (kind === "copyJson") { void this.copyJson(); return; }
    if (kind === "switchView") {
      const handle = (message as { handle?: unknown }).handle;
      if (typeof handle === "string" && handle !== this.selectedHandle) {
        this.selectedHandle = handle;
        void this.regenerate("switchView");
      }
      return;
    }
    if (kind === "export") {
      const format = (message as { format?: unknown }).format;
      const data = (message as { data?: unknown }).data;
      if ((format === "svg" || format === "png") && typeof data === "string") {
        void this.saveExport(format, data);
      }
      return;
    }
    if (kind === "openSource") {
      void this.navigate(message);
    }
  }

  /** A workspace model file to anchor `spec42/diagramViews` on; the catalog it returns is
   * model-wide regardless of which file is used. */
  private async resolveAnchorUri(): Promise<string | undefined> {
    const active = vscode.window.activeTextEditor?.document;
    if (isModelDocument(active)) {
      this.anchorUri = active.uri.toString();
      return this.anchorUri;
    }
    const open = vscode.workspace.textDocuments.find(isModelDocument);
    if (open) {
      this.anchorUri = open.uri.toString();
      return this.anchorUri;
    }
    if (this.anchorUri) {
      try {
        await vscode.workspace.fs.stat(vscode.Uri.parse(this.anchorUri));
        return this.anchorUri;
      } catch {
        this.anchorUri = undefined;
      }
    }
    const found = await vscode.workspace.findFiles(MODEL_GLOB, "**/node_modules/**", 1);
    this.anchorUri = found[0]?.toString();
    return this.anchorUri;
  }

  private async fetchCatalog(anchorUri: string): Promise<DiagramViewCatalog> {
    return parseDiagramViewCatalog(
      await this.handles.client.sendRequest("spec42/diagramViews", { modelUri: anchorUri }),
    );
  }

  private async regenerate(
    reason: "resolve" | "visibility" | "manual" | "switchView" | "publication" | "retry",
  ): Promise<void> {
    if (!this.view) return;
    if (reason === "visibility" && this.lastArtifact && this.view.visible) {
      // Becoming visible with a current render already drawn: nothing to do.
      if (!diagramRenderIsStale(this.lastArtifact.product.modelDigest, this.catalogModelDigest ?? "")) {
        return;
      }
    }
    if (reason === "manual" || reason === "switchView") {
      // A user-driven attempt gets a fresh first-load retry budget.
      this.firstLoadAttempt = 0;
      if (this.firstLoadRetry) { clearTimeout(this.firstLoadRetry); this.firstLoadRetry = undefined; }
    }

    this.activeAbort?.abort();
    const abort = new AbortController();
    this.activeAbort = abort;
    const current = ++this.generation;

    void this.view.webview.postMessage({ type: "busy", busy: true });
    try {
      const anchorUri = await this.resolveAnchorUri();
      if (current !== this.generation || abort.signal.aborted) return;
      if (!anchorUri) {
        this.postPlaceholder("This workspace has no SysML files.");
        return;
      }

      let attempt = 0;
      // eslint-disable-next-line no-constant-condition
      while (true) {
        const catalog = await this.fetchCatalog(anchorUri);
        if (current !== this.generation || abort.signal.aborted) return;
        this.catalogModelDigest = catalog.modelDigest;
        this.options = allDiagramViewOptions(catalog);
        if (this.options.length === 0) {
          if (!this.lastArtifact && this.emptyCatalogDigest !== catalog.modelDigest) {
            this.emptyCatalogDigest = catalog.modelDigest;
            this.postLoading();
            this.scheduleFirstLoadRetry();
            return;
          }
          this.postPlaceholder("This model authors no diagram views. Add a `view … : GeneralView { expose … }` to see one here.");
          return;
        }
        this.emptyCatalogDigest = undefined;

        const remembered = this.selectedHandle
          ? this.options.find((option) => option.handle === this.selectedHandle)
          : undefined;
        const selected = remembered ?? this.defaultSelection(catalog) ?? this.options[0];
        if (!selected) {
          this.postPlaceholder("This model authors no diagram views.");
          return;
        }
        this.selectedHandle = selected.handle;

        const document = vscode.workspace.textDocuments.find(
          (candidate) => candidate.uri.toString() === selected.documentUri,
        ) ?? await this.openHidden(selected.documentUri);
        if (!document) {
          this.postPlaceholder(`Could not open ${selected.group} to generate its diagram.`);
          return;
        }
        const kind = diagramViewKindForHandle(catalog, selected.handle);
        if (!kind) {
          this.postPlaceholder("The selected diagram view is no longer in the catalog.");
          return;
        }

        try {
          const artifact = await this.generate(document, kind, catalog.modelDigest, selected.handle, abort.signal);
          if (current !== this.generation) return;
          this.lastArtifact = artifact;
          this.firstLoadAttempt = 0;
          if (this.firstLoadRetry) { clearTimeout(this.firstLoadRetry); this.firstLoadRetry = undefined; }
          this.postRender();
          return;
        } catch (error) {
          if (current !== this.generation || abort.signal.aborted) return;
          if (isDigestMismatch(error) && attempt < DIGEST_MISMATCH_RETRIES) {
            attempt += 1;
            await delay(150);
            continue;
          }
          throw error;
        }
      }
    } catch (error) {
      if (current !== this.generation) return;
      const message = describeError(error);
      if (!this.lastArtifact) {
        // No render has ever landed — the LSP is probably still indexing. Say so and keep
        // trying with backoff; `spec42/publicationChanged` also retriggers this path.
        if (this.firstLoadAttempt >= 6) {
          this.postPlaceholder(`Could not generate a diagram: ${message}`);
        } else {
          this.postLoading();
        }
        this.scheduleFirstLoadRetry();
      } else if (isServerNotReady(error)) {
        this.postRender("the language server is catching up");
      } else {
        this.postRender(message);
        if (reason === "manual" || reason === "switchView") {
          await vscode.window.showErrorMessage(`Diagram generation failed: ${message}`);
        }
      }
    } finally {
      if (current === this.generation) void this.view?.webview.postMessage({ type: "busy", busy: false });
    }
  }

  private defaultSelection(catalog: DiagramViewCatalog): DiagramViewOption | undefined {
    const active = vscode.window.activeTextEditor?.document;
    if (isModelDocument(active)) {
      const authored = authoredDiagramViewOptions(catalog, active.uri.toString());
      if (authored.length > 0) return authored[0];
    }
    return this.options[0];
  }

  private async openHidden(uri: string): Promise<vscode.TextDocument | undefined> {
    try {
      return await vscode.workspace.openTextDocument(vscode.Uri.parse(uri));
    } catch {
      return undefined;
    }
  }

  private viewList(): RenderMessage["views"] {
    return this.options.map((option) => ({ handle: option.handle, label: option.label, group: option.group }));
  }

  private postPlaceholder(placeholder: string): void {
    this.send({
      type: "render",
      productJson: JSON.stringify(emptyProduct()),
      views: this.viewList(),
      selectedHandle: this.selectedHandle ?? "",
      header: placeholder,
      incompleteReasons: [],
      placeholder,
    });
  }

  private postLoading(): void {
    this.send({
      type: "render",
      productJson: JSON.stringify(emptyProduct()),
      views: this.viewList(),
      selectedHandle: this.selectedHandle ?? "",
      header: "Loading diagram…",
      incompleteReasons: [],
      placeholder: "Loading diagram…",
      loading: true,
    });
  }

  private postRender(error?: string): void {
    const artifact = this.lastArtifact;
    if (!artifact) {
      this.postPlaceholder(error ?? "No diagram generated yet.");
      return;
    }
    this.send({
      type: "render",
      productJson: artifact.productJson,
      views: this.viewList(),
      selectedHandle: this.selectedHandle ?? "",
      header: artifactHeader(artifact),
      incompleteReasons: artifact.product.completeness.reasons.map((reason) => reason.code),
      error,
    });
  }

  private send(message: RenderMessage): void {
    if (this.webviewReady) {
      void this.view?.webview.postMessage(message);
    } else {
      this.pendingRender = message;
    }
  }

  private async generate(
    document: vscode.TextDocument,
    view: DiagramViewId,
    expectedModelDigest: string | undefined,
    handle: string,
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
      args: [handle],
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

  private async saveExport(format: "svg" | "png", data: string): Promise<void> {
    const base = this.lastArtifact?.product.selectedView.name?.replace(/[^\w.-]+/g, "_") || "diagram";
    const folder = vscode.workspace.workspaceFolders?.[0]?.uri;
    const defaultUri = folder
      ? vscode.Uri.joinPath(folder, `${base}.${format}`)
      : vscode.Uri.file(`${base}.${format}`);
    const target = await vscode.window.showSaveDialog({
      defaultUri,
      filters: format === "svg" ? { "SVG image": ["svg"] } : { "PNG image": ["png"] },
    });
    if (!target) return;
    let bytes: Uint8Array;
    if (format === "svg") {
      bytes = Buffer.from(data, "utf8");
    } else {
      const comma = data.indexOf(",");
      bytes = Buffer.from(comma >= 0 ? data.slice(comma + 1) : data, "base64");
    }
    try {
      await vscode.workspace.fs.writeFile(target, bytes);
      await vscode.window.showInformationMessage(`Saved ${vscode.workspace.asRelativePath(target)}.`);
    } catch (error) {
      await vscode.window.showErrorMessage(`Could not save the diagram: ${describeError(error)}`);
    }
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
        viewColumn: existingColumn ?? vscode.ViewColumn.Active,
        preview: false,
      });
      editor.selection = new vscode.Selection(range.start, range.start);
      editor.revealRange(range, vscode.TextEditorRevealType.InCenterIfOutsideViewport);
    } catch { /* Invalid or unavailable provenance remains inert. */ }
  }

  private shellHtml(webview: vscode.Webview): string {
    const nonce = `${Date.now()}${Math.random().toString(36).slice(2)}`;
    const script = webview.asWebviewUri(vscode.Uri.joinPath(this.context.extensionUri, "media", "diagram-viewer.js"));
    return `<!doctype html><html><head><meta charset="utf-8">
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; img-src ${webview.cspSource} data:; style-src ${webview.cspSource} 'unsafe-inline'; script-src 'nonce-${nonce}' ${webview.cspSource};">
<style>
  html,body{height:100%}
  body{padding:0;margin:0;color:var(--vscode-foreground);background:var(--vscode-editor-background);display:flex;flex-direction:column;font:12px var(--vscode-font-family)}
  body.busy .canvas{opacity:.5}
  header{display:flex;flex-wrap:wrap;align-items:center;gap:4px 6px;padding:6px 8px;border-bottom:1px solid var(--vscode-panel-border)}
  header select,header button{font:inherit;color:var(--vscode-foreground);background:var(--vscode-button-secondaryBackground);border:1px solid var(--vscode-panel-border);border-radius:3px;padding:2px 5px;cursor:pointer}
  header select{flex:1 1 140px;min-width:0;color:var(--vscode-dropdown-foreground);background:var(--vscode-dropdown-background);border-color:var(--vscode-dropdown-border,var(--vscode-panel-border))}
  header select option,header select optgroup{color:var(--vscode-dropdown-foreground);background:var(--vscode-dropdown-background)}
  header button:hover,header select:hover{background:var(--vscode-toolbar-hoverBackground)}
  .status{flex-basis:100%;color:var(--vscode-descriptionForeground);white-space:nowrap;overflow:hidden;text-overflow:ellipsis}
  .status .error{color:var(--vscode-errorForeground);font-weight:600}
  .canvas{flex:1;min-height:0;position:relative}
  .canvas svg{display:block;width:100%;height:100%}
  .empty{padding:24px 16px;color:var(--vscode-descriptionForeground);font:13px var(--vscode-font-family)}
  .loading{display:flex;align-items:center;gap:9px}
  .loading-spinner{width:13px;height:13px;border:2px solid var(--vscode-progressBar-background);border-right-color:transparent;border-radius:50%;animation:diagram-spin .8s linear infinite}
  @keyframes diagram-spin{to{transform:rotate(360deg)}}
</style></head><body>
<header>
  <select id="view-select" title="Authored diagram view"></select>
  <button id="home" title="Fit and center the entire diagram">Home</button>
  <button id="copy-json" title="Copy the generated diagram JSON">JSON</button>
  <button id="export-svg" title="Export as SVG">SVG</button>
  <button id="export-png" title="Export as PNG">PNG</button>
  <span class="status" id="status"></span>
</header>
<main id="diagram" class="canvas"></main>
<script nonce="${nonce}" src="${script}"></script></body></html>`;
  }
}

function emptyProduct(): DiagramProduct {
  return {
    schemaVersion: 5,
    modelDigest: "",
    documents: [],
    sources: [],
    references: [],
    selectedView: { reference: 0, kind: "general-view", name: "", source: 0 },
    completeness: { status: "incomplete", reasons: [] },
    projection: { kind: "general-view", exposedRoots: [], nodes: [], relationships: [], edges: [], metadata: {}, scene: { kind: "general" } },
  } as unknown as DiagramProduct;
}

function describeError(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

function isDigestMismatch(error: unknown): boolean {
  const message = describeError(error).toLowerCase();
  return message.includes("publication changed") || message.includes("model digest");
}

function isServerNotReady(error: unknown): boolean {
  const message = describeError(error).toLowerCase();
  return (
    message.includes("not part of the current workspace publication") ||
    message.includes("client is not running") ||
    message.includes("connection") ||
    message.includes("server is not ready") ||
    message.includes("no workspace")
  );
}

function delay(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export function registerDiagramViewer(context: vscode.ExtensionContext, handles: LspClientHandles): void {
  const provider = new DiagramViewProvider(context, handles);
  context.subscriptions.push(
    provider,
    vscode.window.registerWebviewViewProvider(DIAGRAM_VIEW_ID, provider, {
      webviewOptions: { retainContextWhenHidden: true },
    }),
    vscode.commands.registerCommand("spec42.diagram.open", () => provider.open()),
    vscode.commands.registerCommand("spec42.diagram.copyJson", () => provider.copyJson()),
  );
}

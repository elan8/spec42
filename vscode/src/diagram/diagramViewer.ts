import * as fs from "fs/promises";
import * as path from "path";
import * as vscode from "vscode";
import { State } from "vscode-languageclient/node";
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
  reconcileDelayMs,
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
/** After this many consecutive empty-catalog reads at one digest, treat "no views" as the
 * answer and let the reconcile loop drop to its slow keep-checking cadence. */
const EMPTY_CATALOG_PATIENCE = 4;
/** If the webview never reports `ready`, its bundle probably failed to load — re-inject once. */
const WEBVIEW_READY_TIMEOUT_MS = 4_000;

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
  private selectedHandle: string | undefined;
  private anchorUri: string | undefined;
  private lastArtifact: RenderedArtifact | undefined;
  private generation = 0;
  private activeAbort: AbortController | undefined;
  private webviewReady = false;
  private pendingRender: RenderMessage | undefined;
  private publicationDebounce: ReturnType<typeof setTimeout> | undefined;
  private webviewWatchdog: ReturnType<typeof setTimeout> | undefined;
  // The reconcile loop is the single source of eventual consistency: as long as the view is
  // visible and the render does not match the current publication, it keeps retrying with
  // backoff, so no missed `spec42/publicationChanged`, slow index, or server restart can leave
  // the panel permanently stuck. It is idle (no timer) once converged.
  private reconcileTimer: ReturnType<typeof setTimeout> | undefined;
  private reconcileAttempt = 0;
  private regenerating = false;
  private converged = false;
  private emptyCatalogDigest: string | undefined;
  private emptyCatalogAttempts = 0;
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
      // A server restart rebuilds the publication from scratch; the notification for that build
      // can land before this view resolves, so treat "Running" as a reason to reconcile.
      this.handles.client.onDidChangeState(({ newState }) => {
        if (newState === State.Running) {
          this.converged = false;
          this.kickReconcile();
        }
      }),
    );
  }

  dispose(): void {
    if (this.publicationDebounce) clearTimeout(this.publicationDebounce);
    if (this.reconcileTimer) clearTimeout(this.reconcileTimer);
    if (this.webviewWatchdog) clearTimeout(this.webviewWatchdog);
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
      if (webviewView.visible) this.kickReconcile();
      else this.stopReconcile();
    }, undefined, this.disposables);
    webviewView.onDidDispose(() => {
      if (this.view === webviewView) {
        this.view = undefined;
        this.webviewReady = false;
        this.pendingRender = undefined;
        this.stopReconcile();
        this.activeAbort?.abort();
      }
    }, undefined, this.disposables);

    this.reconcileAttempt = 0;
    this.converged = false;
    this.armWebviewWatchdog();
    this.postLoading();
    // Both the reconcile loop and this one-shot both aim at the first render; whichever the LSP
    // is ready for first wins, and the loop keeps trying if neither does yet.
    void this.handles.clientReadyPromise.then(() => this.kickReconcile(), () => this.kickReconcile());
    this.kickReconcile();
  }

  /** Re-inject the shell if the webview never reported `ready` — its script bundle probably
   * failed to load, and without this the panel would sit on a stale placeholder forever. */
  private armWebviewWatchdog(): void {
    if (this.webviewWatchdog) clearTimeout(this.webviewWatchdog);
    this.webviewWatchdog = setTimeout(() => {
      this.webviewWatchdog = undefined;
      if (this.webviewReady || !this.view) return;
      this.webviewReady = false;
      this.pendingRender = undefined;
      this.view.webview.html = this.shellHtml(this.view.webview);
      // The re-injected script sends a fresh `ready`; queue what should be on screen for it.
      if (this.lastArtifact) this.postRender();
      else this.postLoading();
      this.armWebviewWatchdog();
    }, WEBVIEW_READY_TIMEOUT_MS);
  }

  /** Reconcile now if there is something to do, and make sure the backoff loop is armed. */
  private kickReconcile(): void {
    if (!this.view) return;
    if (this.reconcileTimer) { clearTimeout(this.reconcileTimer); this.reconcileTimer = undefined; }
    if (this.converged || this.regenerating || !this.view.visible) {
      if (!this.converged && this.view.visible) this.scheduleReconcile();
      return;
    }
    void this.regenerate("reconcile");
  }

  private stopReconcile(): void {
    if (this.reconcileTimer) { clearTimeout(this.reconcileTimer); this.reconcileTimer = undefined; }
  }

  private scheduleReconcile(): void {
    if (this.reconcileTimer || this.converged || !this.view?.visible) return;
    const wait = reconcileDelayMs(
      this.reconcileAttempt++,
      this.emptyCatalogAttempts >= EMPTY_CATALOG_PATIENCE,
    );
    this.reconcileTimer = setTimeout(() => {
      this.reconcileTimer = undefined;
      void this.regenerate("reconcile");
    }, wait);
  }

  /** Command entry point — reveal the view and force a fresh generate. */
  async open(): Promise<void> {
    await vscode.commands.executeCommand(`${DIAGRAM_VIEW_ID}.focus`);
    this.converged = false;
    this.reconcileAttempt = 0;
    this.emptyCatalogAttempts = 0;
    this.emptyCatalogDigest = undefined;
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
    const stillCurrent =
      this.lastArtifact !== undefined &&
      !diagramRenderIsStale(this.lastArtifact.product.modelDigest, publishedModelDigest);
    if (stillCurrent) return;
    this.converged = false;
    this.reconcileAttempt = 0;
    // A digest change is a fresh chance for an empty catalog to have gained a view.
    if (this.emptyCatalogDigest !== publishedModelDigest) this.emptyCatalogAttempts = 0;
    if (this.publicationDebounce) clearTimeout(this.publicationDebounce);
    this.publicationDebounce = setTimeout(() => {
      this.publicationDebounce = undefined;
      this.kickReconcile();
    }, PUBLICATION_DEBOUNCE_MS);
  }

  private onWebviewMessage(message: unknown): void {
    if (!message || typeof message !== "object") return;
    const kind = (message as { type?: unknown }).type;
    if (kind === "ready") {
      this.webviewReady = true;
      if (this.webviewWatchdog) { clearTimeout(this.webviewWatchdog); this.webviewWatchdog = undefined; }
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
    reason: "manual" | "switchView" | "reconcile",
  ): Promise<void> {
    if (!this.view) return;
    // A background reconcile never stacks, interrupts an in-flight attempt, or fires once the
    // render already matches the publication; a user action always runs.
    if (reason === "reconcile" && (this.regenerating || this.converged)) return;
    if (reason === "manual" || reason === "switchView") {
      this.reconcileAttempt = 0;
      this.emptyCatalogAttempts = 0;
      this.emptyCatalogDigest = undefined;
    }

    this.stopReconcile();
    this.activeAbort?.abort();
    const abort = new AbortController();
    this.activeAbort = abort;
    const current = ++this.generation;
    this.regenerating = true;

    void this.view.webview.postMessage({ type: "busy", busy: true });
    try {
      const anchorUri = await this.resolveAnchorUri();
      if (current !== this.generation || abort.signal.aborted) return;
      if (!anchorUri) {
        // Recoverable: a model file may be added later; the reconcile loop keeps checking.
        this.emptyCatalogAttempts = EMPTY_CATALOG_PATIENCE;
        this.showWaiting("This workspace has no SysML files.");
        return;
      }

      let attempt = 0;
      // eslint-disable-next-line no-constant-condition
      while (true) {
        const catalog = await this.fetchCatalog(anchorUri);
        if (current !== this.generation || abort.signal.aborted) return;
        this.options = allDiagramViewOptions(catalog);
        if (this.options.length === 0) {
          if (this.emptyCatalogDigest !== catalog.modelDigest) {
            this.emptyCatalogDigest = catalog.modelDigest;
            this.emptyCatalogAttempts = 0;
          }
          this.emptyCatalogAttempts += 1;
          if (!this.lastArtifact && this.emptyCatalogAttempts < EMPTY_CATALOG_PATIENCE) {
            this.postLoading();
          } else {
            this.postPlaceholder("This model authors no diagram views. Add a `view … : GeneralView { expose … }` to see one here.");
          }
          return;
        }
        this.emptyCatalogDigest = undefined;
        this.emptyCatalogAttempts = 0;

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
          this.showWaiting(`Waiting for ${selected.group}…`);
          return;
        }
        const kind = diagramViewKindForHandle(catalog, selected.handle);
        if (!kind) {
          this.showWaiting("The selected diagram view is no longer in the catalog.");
          return;
        }

        try {
          const artifact = await this.generate(document, kind, catalog.modelDigest, selected.handle, abort.signal);
          if (current !== this.generation) return;
          this.lastArtifact = artifact;
          this.converged = true;
          this.reconcileAttempt = 0;
          this.stopReconcile();
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
      if (this.lastArtifact && !isServerNotReady(error)) {
        // Keep the last good render on screen, flagged stale; the loop retries in the background.
        this.postRender(message);
      } else if (isServerNotReady(error) || this.reconcileAttempt < 5) {
        this.showWaiting("Loading diagram…", true);
      } else {
        // Real, non-transient failure (e.g. missing generator plugin). Surface it, but the
        // reconcile loop keeps trying slowly so fixing the cause recovers with no user action.
        this.showWaiting(`Could not generate a diagram: ${message}`);
      }
    } finally {
      // Only the currently-active attempt owns these; a superseded one must not touch them or it
      // would clear `regenerating` / restart the loop underneath its replacement.
      if (current === this.generation) {
        this.regenerating = false;
        void this.view?.webview.postMessage({ type: "busy", busy: false });
        if (!this.converged && this.view?.visible) this.scheduleReconcile();
      }
    }
  }

  /** A non-fatal "not there yet" state: show the message and let the reconcile loop keep going. */
  private showWaiting(message: string, spinner = false): void {
    this.converged = false;
    if (spinner) this.postLoading();
    else this.postPlaceholder(message);
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

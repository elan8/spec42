import { DIAGRAM_PRODUCT_SCHEMA_VERSION } from "../generated/diagram-product/schemaVersion";
import * as vscode from "vscode";
import { diagramLimitations } from "./diagramLimitations";
import { ANALYSIS_STATUS_CHANGED_METHOD } from "../generated/editor-status";
import type { DiagramPresentationParams, DiagramPresentationResult, DiagramSelection, KnownDiagramProduct } from "../generated/diagram-presentation";
import { DIAGRAM_PRESENTATION_METHOD } from "../generated/diagram-presentation";
import { DiagramRefreshQueue } from "./diagramRefreshQueue";
import { State } from "vscode-languageclient/node";
import type { LspClientHandles } from "../activation/lspClient";
import { isPerformanceLoggingEnabled, logPerfEvent } from "../logger";
import { RenderMeasurementTracker, type RenderMeasurementIdentity } from "./diagramPerformance";
import {
  allDiagramViewOptions,
  type DiagramProduct,
  type DiagramViewCatalog,
  type DiagramViewOption,
  type SourceNavigation,
  isPathInsideWorkspace,
  parseDiagramProduct,
  parseDiagramViewCatalog,
  parseSourceNavigation,
  productContainsSourceTarget,
  visibleSourceColumn,
} from "./diagramViewerCore";

export const DIAGRAM_VIEW_ID = "spec42DiagramView";

type RenderedArtifact = {
  product: DiagramProduct;
  productJson: string;
  durationUs: number;
};

/** A payload the webview can draw without any further round trip. */
type RenderMessage = {
  type: "render";
  productJson: string;
  views: Array<{ handle: string; label: string; group: string; kindLabel?: string; definition?: import("./diagramViewerCore").SourceNavigation }>;
  selectedHandle: string;
  header: string;
  placeholder?: string;
  loading?: boolean;
  error?: string;
  measurement?: RenderMeasurementIdentity;
};

const MODEL_GLOB = "**/*.{sysml,kerml}";
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
  /** Browser view rows whose members the user has opened: their keys, sent with every
   * `spec42/diagram` request so the opened branches stay loaded across regenerations. Keys are
   * stable while the model's structure is, so a model edit keeps them; switching views clears. */
  private readonly expandedBranches = new Set<string>();
  private anchorUri: string | undefined;
  private lastArtifact: RenderedArtifact | undefined;
  private readonly refreshes = new DiagramRefreshQueue();
  private activeRequest: vscode.CancellationTokenSource | undefined;
  private selection: DiagramSelection | null = null;
  private catalog: DiagramViewCatalog | undefined;
  private knownProduct: KnownDiagramProduct | null = null;
  private presentationState: "loading" | "updating" | "ready" | "empty" | "failed" = "loading";
  private activeDrawCancellation: vscode.CancellationTokenSource | undefined;
  private webviewReady = false;
  private pendingRender: RenderMessage | undefined;
  private webviewWatchdog: ReturnType<typeof setTimeout> | undefined;
  private readonly disposables: vscode.Disposable[] = [];
  private readonly renderMeasurements = new RenderMeasurementTracker();
  private actionContext: { hasFrame: boolean; canGoToDefinition: boolean } | undefined;

  private logPerformance(event: string, fields: Record<string, unknown>): void {
    logPerfEvent(`view:${event}`, { ...fields, extensionHostMs: performance.now() });
  }

  constructor(
    private readonly context: vscode.ExtensionContext,
    private readonly handles: LspClientHandles,
  ) {
    this.disposables.push(
      // These are wakes to read the backend's current state, including failures and same-digest
      // completions. Register before the initial request so a completion cannot be lost.
      this.handles.client.onNotification(ANALYSIS_STATUS_CHANGED_METHOD, () => this.requestRefresh()),
      // A server restart rebuilds the publication from scratch; the notification for that build
      // can land before this view resolves, so treat "Running" as a reason to request current state.
      vscode.workspace.onDidCreateFiles(() => this.requestRefresh()),
      vscode.workspace.onDidDeleteFiles(() => this.requestRefresh()),
      vscode.workspace.onDidChangeWorkspaceFolders(() => { this.anchorUri = undefined; this.requestRefresh(); }),
      this.handles.client.onDidChangeState(({ newState }) => {
        if (newState === State.Running) this.requestRefresh();
        else {
          this.cancelPresentation();
          this.presentationState = newState === State.Starting ? "loading" : "failed";
          this.showStatus(newState === State.Starting ? "Starting language server..." : "Language server stopped");
          this.syncActionContext();
        }
      }),
    );
  }

  dispose(): void {
    this.renderMeasurements.invalidate();
    if (this.webviewWatchdog) clearTimeout(this.webviewWatchdog);
    this.cancelPresentation();
    this.activeDrawCancellation?.cancel();
    this.activeDrawCancellation?.dispose();
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
      this.logPerformance("visibility", { visible: webviewView.visible });
      if (webviewView.visible) this.requestRefresh();
      else { this.renderMeasurements.invalidate(); this.cancelPresentation(); }
    }, undefined, this.disposables);
    webviewView.onDidDispose(() => {
      if (this.view === webviewView) {
        this.view = undefined;
        this.webviewReady = false;
        this.pendingRender = undefined;
        this.renderMeasurements.invalidate();
        this.cancelPresentation();
      }
    }, undefined, this.disposables);

    this.armWebviewWatchdog();
    if (this.lastArtifact) this.postRender(undefined, "Updating view…");
    else this.postLoading();
    void this.handles.clientReadyPromise.then(() => this.requestRefresh(), () => undefined);
    this.requestRefresh();
  }

  /** Re-inject the shell if the webview never reported `ready` — its script bundle probably
   * failed to load, and without this the panel would sit on a stale placeholder forever. */
  private armWebviewWatchdog(): void {
    if (this.webviewWatchdog) clearTimeout(this.webviewWatchdog);
    this.webviewWatchdog = setTimeout(() => {
      this.webviewWatchdog = undefined;
      if (this.webviewReady || !this.view) return;
      this.webviewReady = false;
      this.renderMeasurements.invalidate();
      this.view.webview.html = this.shellHtml(this.view.webview);
      // The re-injected script sends a fresh `ready`; queue what should be on screen for it.
      if (!this.pendingRender) {
        if (this.lastArtifact) this.postRender();
        else this.postLoading();
      }
      this.armWebviewWatchdog();
    }, WEBVIEW_READY_TIMEOUT_MS);
  }

  private cancelPresentation(): void {
    this.refreshes.cancel();
    this.activeRequest?.cancel();
    this.activeRequest?.dispose();
    this.activeRequest = undefined;
    void this.view?.webview.postMessage({ type: "busy", busy: false });
  }

  private requestRefresh(): void {
    this.refreshes.request();
    this.renderMeasurements.invalidate();
    this.pumpRefresh();
  }

  private pumpRefresh(): void {
    if (!this.view?.visible || this.handles.client.state !== State.Running) return;
    const ticket = this.refreshes.begin();
    if (ticket === undefined) return;
    void this.regenerate(ticket).finally(() => {
      this.refreshes.finish(ticket);
      this.pumpRefresh();
    });
  }

  async open(): Promise<void> {
    await vscode.commands.executeCommand(`${DIAGRAM_VIEW_ID}.focus`);
    this.requestRefresh();
  }

  /** View title action: fit and center the whole drawing. */
  home(): void {
    void this.view?.webview.postMessage({ type: "resetView" });
  }

  /** View title action: open the selected view's declaration. */
  async goToDefinition(): Promise<void> {
    const target = this.selectedDefinition();
    if (!target) {
      await vscode.window.showInformationMessage("The selected view has no source definition to open.");
      return;
    }
    await this.navigate({ type: "openSource", target });
  }

  /** View title action: choose an export format, then save it through the webview renderer. */
  async exportView(): Promise<void> {
    if (!this.lastArtifact) {
      await vscode.window.showInformationMessage("Open a Spec42 view before exporting it.");
      return;
    }
    type ExportChoice = vscode.QuickPickItem & { action: "svg" | "png" | "json"; transparent?: boolean };
    const choices: ExportChoice[] = [
      { label: "$(file-media) SVG", description: "Vector image", action: "svg", transparent: false },
      { label: "$(file-media) SVG, transparent background", description: "Element fills are kept", action: "svg", transparent: true },
      { label: "$(file-media) PNG", description: "Raster image at 2× scale", action: "png", transparent: false },
      { label: "$(file-media) PNG, transparent background", description: "Element fills are kept", action: "png", transparent: true },
      { label: "$(json) Copy view JSON", description: "The generated diagram product", action: "json" },
    ];
    const choice = await vscode.window.showQuickPick(choices, { title: "Export view", placeHolder: "Choose an export format" });
    if (!choice) return;
    if (choice.action === "json") {
      await this.copyJson();
      return;
    }
    void this.view?.webview.postMessage({ type: "requestExport", format: choice.action, transparent: choice.transparent === true });
  }

  private selectedDefinition(): SourceNavigation | undefined {
    return this.options.find((option) => option.handle === this.selectedHandle)?.definition;
  }

  /** Enables the view title actions; navigation needs a current presentation, export a frame. */
  private syncActionContext(): void {
    const next = {
      hasFrame: this.lastArtifact !== undefined,
      canGoToDefinition: this.presentationState === "ready" && this.selectedDefinition() !== undefined,
    };
    if (this.actionContext?.hasFrame !== next.hasFrame) {
      void vscode.commands.executeCommand("setContext", "spec42.diagram.hasFrame", next.hasFrame);
    }
    if (this.actionContext?.canGoToDefinition !== next.canGoToDefinition) {
      void vscode.commands.executeCommand("setContext", "spec42.diagram.canGoToDefinition", next.canGoToDefinition);
    }
    this.actionContext = next;
  }

  async copyJson(): Promise<void> {
    if (!this.lastArtifact) {
      await vscode.window.showInformationMessage("Open a Spec42 view before copying its JSON.");
      return;
    }
    // The server sends compact JSON; indent the copy for the person reading it.
    await vscode.env.clipboard.writeText(`${JSON.stringify(this.lastArtifact.product, null, 2)}\n`);
    await vscode.window.showInformationMessage("Copied the generated view JSON.");
  }

  private onWebviewMessage(message: unknown): void {
    if (!message || typeof message !== "object") return;
    const kind = (message as { type?: unknown }).type;
    if (kind === "renderComplete") {
      if (!isPerformanceLoggingEnabled()) return;
      const measurement = this.renderMeasurements.complete(message, performance.now());
      if (measurement && this.view?.visible && this.presentationState === "ready") {
        this.logPerformance("renderComplete", measurement);
      }
      return;
    }
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
        const view = this.catalog?.views.find((view) => view.handle === handle);
        if (!view) return;
        this.selectedHandle = handle;
        this.selection = { kind: view.kind, reference: view.reference };
        this.expandedBranches.clear();
        this.syncActionContext();
        this.requestRefresh();
      }
      return;
    }
    if (kind === "loadBranch") {
      // A browser row whose members are not in the product yet: ask for them and redraw.
      const key = (message as { key?: unknown }).key;
      if (typeof key === "string" && key && !this.expandedBranches.has(key)) {
        this.expandedBranches.add(key);
        this.requestRefresh();
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
    if (kind === "copyLimitations") {
      const request = message as { modelDigest?: string; viewHandle?: string };
      const artifact = this.lastArtifact;
      if (artifact && request.modelDigest === artifact.product.modelDigest && request.viewHandle === this.selectedHandle) {
        // Only the limitations: each canonical reason beside its explanation and the source sites
        // its product-local indexes name, so the copy stands alone without the whole product.
        const product = artifact.product;
        const details = diagramLimitations(product).map((detail, index) => ({
          ...detail,
          reason: product.completeness.reasons[index],
        }));
        void vscode.env.clipboard.writeText(JSON.stringify({
          view: product.selectedView.name,
          modelDigest: product.modelDigest,
          status: product.completeness.status,
          limitations: details,
        }, null, 2));
      }
      return;
    }
    if (kind === "openSource") {
      void this.navigate(message);
    }
    if (kind === "drawRequest") {
      void this.requestDraw(message);
    }
  }

  /** Select a project context without opening a source editor or creating analysis inputs. */
  private async resolveAnchorUri(): Promise<string | undefined> {
    const active = vscode.window.activeTextEditor?.document;
    if (isModelDocument(active) && vscode.workspace.getWorkspaceFolder(active.uri)) {
      this.anchorUri = active.uri.toString();
      return this.anchorUri;
    }
    const open = vscode.workspace.textDocuments.find((document) => isModelDocument(document) && vscode.workspace.getWorkspaceFolder(document.uri));
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

  /** Forwards a webview "drawRequest" to `spec42/draw` and posts back "drawResponse" or
   * "drawError". Cancellation is advisory: a new request cancels whichever one is in flight,
   * but the webview is what actually decides whether a response is still current -- it tracks
   * its own request id and re-checks the echoed identity/revision. There is no client drawing
   * fallback when this fails. */
  private async requestDraw(message: unknown): Promise<void> {
    const requestId = (message as { requestId?: unknown }).requestId;
    const modelDigest = (message as { modelDigest?: unknown }).modelDigest;
    const viewHandle = (message as { viewHandle?: unknown }).viewHandle;
    const presentationRevision = (message as { presentationRevision?: unknown }).presentationRevision;
    const product = (message as { product?: unknown }).product;
    const width = (message as { width?: unknown }).width;
    const height = (message as { height?: unknown }).height;
    const colorScheme = (message as { colorScheme?: unknown }).colorScheme;
    const disclosure = (message as { disclosure?: unknown }).disclosure;
    if (
      typeof requestId !== "number" ||
      typeof modelDigest !== "string" ||
      typeof viewHandle !== "string" ||
      typeof presentationRevision !== "number" ||
      product === undefined
    ) {
      return;
    }

    this.activeDrawCancellation?.cancel();
    this.activeDrawCancellation?.dispose();
    const cancellation = new vscode.CancellationTokenSource();
    this.activeDrawCancellation = cancellation;
    const drawStarted = performance.now();

    try {
      const result = await this.handles.client.sendRequest(
        "spec42/draw",
        { modelDigest, viewHandle, presentationRevision, product, width, height, colorScheme, disclosure },
        cancellation.token,
      );
      if (cancellation.token.isCancellationRequested) return;
      const drawn = result as {
        modelDigest: string;
        viewHandle: string;
        presentationRevision: number;
        svg: string;
      };
      this.logPerformance("drawComplete", {
        modelDigest: drawn.modelDigest, viewHandle: drawn.viewHandle,
        presentationRevision: drawn.presentationRevision, clientDrawMs: performance.now() - drawStarted,
      });
      void this.view?.webview.postMessage({
        type: "drawResponse",
        requestId,
        modelDigest: drawn.modelDigest,
        viewHandle: drawn.viewHandle,
        presentationRevision: drawn.presentationRevision,
        svg: drawn.svg,
      });
    } catch (error) {
      if (cancellation.token.isCancellationRequested) return;
      this.logPerformance("drawFailed", {
        modelDigest, viewHandle, presentationRevision, clientDrawMs: performance.now() - drawStarted,
      });
      void this.view?.webview.postMessage({
        type: "drawError",
        requestId,
        message: describeError(error),
      });
    }
  }

  private showStatus(header: string, error?: string): void {
    if (this.lastArtifact && this.webviewReady) {
      void this.view?.webview.postMessage({ type: "status", header, error });
    } else if (this.lastArtifact) this.postRender(error, header);
    else this.postPlaceholder(error ?? header);
  }

  private async regenerate(ticket: number): Promise<void> {
    const cancellation = new vscode.CancellationTokenSource();
    this.activeRequest = cancellation;
    const current = () => this.refreshes.isCurrent(ticket) && !cancellation.token.isCancellationRequested;
    const started = performance.now();
    this.presentationState = this.lastArtifact ? "updating" : "loading";
    if (this.lastArtifact) this.showStatus("Updating view…");
    else this.postLoading();
    void this.view?.webview.postMessage({ type: "busy", busy: true });
    try {
      const anchorUri = await this.resolveAnchorUri();
      if (!current()) return;
      if (!anchorUri) {
        this.presentationState = "empty";
        this.lastArtifact = undefined;
        this.knownProduct = null;
        this.options = [];
        this.postPlaceholder("This workspace has no SysML files.");
        return;
      }
      const active = vscode.window.activeTextEditor?.document;
      const params: DiagramPresentationParams = {
        projectUri: anchorUri, selection: this.selection,
        preferredDocumentUri: isModelDocument(active) ? active.uri.toString() : null,
        expanded: [...this.expandedBranches].sort(), knownProduct: this.knownProduct,
      };
      const result = await this.handles.client.sendRequest<DiagramPresentationResult>(DIAGRAM_PRESENTATION_METHOD, params, cancellation.token);
      if (!current()) return;
      switch (result.state) {
        case "updating":
          this.presentationState = "updating";
          this.showStatus("Updating view…");
          return; // Completion is delivered by the backend lifecycle wake, never a timer.
        case "superseded":
          this.refreshes.request();
          return; // The owner rejected this completed work; query the new owner state once.
        case "failed":
        case "unavailable":
          this.presentationState = "failed";
          this.showStatus("View unavailable", result.message);
          return;
        case "cancelled":
          this.presentationState = "failed";
          this.showStatus("Analysis cancelled", "The previous publication is retained.");
          return;
        case "empty":
          this.presentationState = "empty";
          this.catalog = { modelDigest: result.modelDigest, views: [] };
          this.options = [];
          this.expandedBranches.clear();
          this.selection = null;
          this.selectedHandle = undefined;
          this.knownProduct = null;
          this.lastArtifact = undefined;
          this.postPlaceholder("No views are available. Check source diagnostics or add a view declaration.");
          return;
        case "unchanged":
          if (!this.lastArtifact) throw new Error("The server reported an unchanged diagram without a retained product.");
          this.presentationState = "ready";
          this.showStatus(artifactHeader(this.lastArtifact));
          return;
        case "ready": {
          const catalog = parseDiagramViewCatalog(result);
          const selected = catalog.views.find((view) => view.handle === result.selectedHandle);
          const product = parseDiagramProduct(result.productJson);
          if (!selected || selected.kind !== product.selectedView.kind || selected.kind !== result.selection.kind ||
            JSON.stringify(selected.reference) !== JSON.stringify(result.selection.reference) || product.modelDigest !== result.modelDigest) {
            throw new Error("The diagram presentation response is incoherent.");
          }
          this.catalog = catalog;
          this.options = allDiagramViewOptions(catalog);
          this.selectedHandle = result.selectedHandle;
          this.selection = result.selection;
          this.expandedBranches.clear();
          for (const key of result.expanded) this.expandedBranches.add(key);
          this.knownProduct = { modelDigest: result.modelDigest, selection: result.selection, expanded: result.expanded };
          this.lastArtifact = { product, productJson: result.productJson, durationUs: result.durationUs };
          this.presentationState = "ready";
          this.logPerformance("generated", { modelDigest: result.modelDigest, viewHandle: result.selectedHandle, generationMs: performance.now() - started, serverGenerationMs: result.durationUs / 1000, semanticStateVersion: result.semanticStateVersion });
          this.postRender();
          return;
        }
        default: throw new Error("The server returned an unknown diagram presentation state.");
      }
    } catch (error) {
      if (!current()) return;
      this.presentationState = "failed";
      this.showStatus("Could not generate a view", describeError(error));
    } finally {
      if (current()) void this.view?.webview.postMessage({ type: "busy", busy: this.presentationState === "updating" });
      if (this.activeRequest === cancellation) this.activeRequest = undefined;
      cancellation.dispose();
      this.syncActionContext();
    }
  }

  private viewList(): RenderMessage["views"] {
    return this.options.map((option) => ({ handle: option.handle, label: option.label, group: option.group, kindLabel: option.kindLabel, definition: option.definition }));
  }

  private postPlaceholder(placeholder: string): void {
    this.send({
      type: "render",
      productJson: JSON.stringify(emptyProduct()),
      views: this.viewList(),
      selectedHandle: this.selectedHandle ?? "",
      header: placeholder,
      placeholder,
    });
  }

  private postLoading(): void {
    this.send({
      type: "render",
      productJson: JSON.stringify(emptyProduct()),
      views: this.viewList(),
      selectedHandle: this.selectedHandle ?? "",
      header: "Loading view…",
      placeholder: "Loading view…",
      loading: true,
    });
  }

  private postRender(error?: string, header?: string): void {
    const artifact = this.lastArtifact;
    if (!artifact) {
      this.postPlaceholder(error ?? "No view generated yet.");
      return;
    }
    this.send({
      type: "render",
      productJson: artifact.productJson,
      views: this.viewList(),
      selectedHandle: this.selectedHandle ?? "",
      header: header ?? artifactHeader(artifact),
      error,
      ...(isPerformanceLoggingEnabled() && !error && this.presentationState === "ready" ? {
        measurement: this.renderMeasurements.begin(artifact.product.modelDigest, this.selectedHandle ?? "", performance.now()),
      } : {}),
    });
  }

  private send(message: RenderMessage): void {
    if (!message.measurement) this.renderMeasurements.invalidate();
    if (this.webviewReady) {
      void this.view?.webview.postMessage(message);
    } else {
      this.pendingRender = message;
    }
  }

  private async saveExport(format: "svg" | "png", data: string): Promise<void> {
    const base = this.lastArtifact?.product.selectedView.name?.replace(/[^\w.-]+/g, "_") || "view";
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
      await vscode.window.showErrorMessage(`Could not save the view: ${describeError(error)}`);
    }
  }

  private async navigate(message: unknown): Promise<void> {
    // A retained frame is readable while analysis runs; its old source ranges are not current.
    if (this.presentationState !== "ready") return;
    if (!message || typeof message !== "object" || (message as { type?: unknown }).type !== "openSource") return;
    const target = parseSourceNavigation((message as { target?: unknown }).target);
    if (!target) return;
    let uri: vscode.Uri;
    try { uri = vscode.Uri.parse(target.uri, true); } catch { return; }
    const roots = vscode.workspace.workspaceFolders?.map((folder) => folder.uri.fsPath) ?? [];
    if (uri.scheme !== "file" || (!isPathInsideWorkspace(uri.fsPath, roots) &&
      !(this.lastArtifact && productContainsSourceTarget(this.lastArtifact.product, target)))) return;
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
  header select{font:inherit;color:var(--vscode-foreground);background:var(--vscode-button-secondaryBackground);border:1px solid var(--vscode-panel-border);border-radius:3px;padding:2px 5px;cursor:pointer}
  header select{flex:1 1 140px;min-width:0;color:var(--vscode-dropdown-foreground);background:var(--vscode-dropdown-background);border-color:var(--vscode-dropdown-border,var(--vscode-panel-border))}
  header select option,header select optgroup{color:var(--vscode-dropdown-foreground);background:var(--vscode-dropdown-background)}
  header select:hover{background:var(--vscode-toolbar-hoverBackground)}
  .subline{flex-basis:100%;display:flex;gap:6px;min-width:0;color:var(--vscode-descriptionForeground)}
  .subline .view-kind:not(:empty)::after{content:"·";margin-left:6px}
  .status{flex:1 1 auto;min-width:0;white-space:nowrap;overflow:hidden;text-overflow:ellipsis}
  .status .error{color:var(--vscode-errorForeground);font-weight:600}
  .canvas{flex:1;min-height:0;position:relative}
  .limitations{padding:6px 10px;border-bottom:1px solid var(--vscode-panel-border);max-height:35vh;overflow:auto}
  .limitations summary{cursor:pointer;color:var(--vscode-textLink-foreground)}
  .limitations code{color:var(--vscode-descriptionForeground)}
  .limitations button{color:var(--vscode-button-foreground);background:var(--vscode-button-background);border:1px solid var(--vscode-contrastBorder,transparent);padding:3px 6px;margin:3px}
  .limitations button:focus-visible,.limitations summary:focus-visible{outline:1px solid var(--vscode-focusBorder);outline-offset:2px}
  .view-kind{color:var(--vscode-descriptionForeground)}
  .canvas .viz-bg{fill:transparent}
  .canvas svg{display:block;width:100%;height:100%}
  .empty{padding:24px 16px;color:var(--vscode-descriptionForeground);font:13px var(--vscode-font-family)}
  .loading{display:flex;align-items:center;gap:9px}
  .loading-spinner{width:13px;height:13px;border:2px solid var(--vscode-progressBar-background);border-right-color:transparent;border-radius:50%;animation:diagram-spin .8s linear infinite}
  @keyframes diagram-spin{to{transform:rotate(360deg)}}
</style></head><body>
<header>
  <select id="view-select" title="Select a view by namespace" aria-label="View"></select>
  <div class="subline"><span class="view-kind" id="view-kind"></span><span class="status" id="status"></span></div>
</header>
<aside id="limitations" class="limitations" aria-label="Projection limitations" hidden></aside>
<main id="diagram" class="canvas"></main>
<script nonce="${nonce}" src="${script}"></script></body></html>`;
  }
}

function emptyProduct(): DiagramProduct {
  return {
    schemaVersion: DIAGRAM_PRODUCT_SCHEMA_VERSION,
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

export function registerDiagramViewer(context: vscode.ExtensionContext, handles: LspClientHandles): void {
  const provider = new DiagramViewProvider(context, handles);
  context.subscriptions.push(
    provider,
    vscode.window.registerWebviewViewProvider(DIAGRAM_VIEW_ID, provider, {
      webviewOptions: { retainContextWhenHidden: true },
    }),
    vscode.commands.registerCommand("spec42.diagram.open", () => provider.open()),
    vscode.commands.registerCommand("spec42.diagram.copyJson", () => provider.copyJson()),
    vscode.commands.registerCommand("spec42.diagram.home", () => provider.home()),
    vscode.commands.registerCommand("spec42.diagram.goToDefinition", () => provider.goToDefinition()),
    vscode.commands.registerCommand("spec42.diagram.export", () => provider.exportView()),
  );
}

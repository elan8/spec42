import * as vscode from "vscode";
import { log, logError, logPerfEvent, logStartupEvent } from "../logger";
import type { LspModelProvider } from "../providers/lspModelProvider";
import type {
  SysMLElementDTO,
  RelationshipDTO,
  RangeDTO,
} from "../providers/sysmlModelTypes";
import { graphToElementTree } from "../visualization/prepareData";

function logPerf(event: string, extra?: Record<string, unknown>): void {
  logPerfEvent(event, extra);
}

/** Helper to convert RangeDTO to vscode.Range for openLocation. */
export function toVscodeRange(dto: RangeDTO): vscode.Range {
  return new vscode.Range(
    new vscode.Position(dto.start.line, dto.start.character),
    new vscode.Position(dto.end.line, dto.end.character)
  );
}

export class FileTreeItem extends vscode.TreeItem {
  readonly itemType = "file-node" as const;
  childrenItems: ModelTreeItem[] = [];

  constructor(
    public readonly fileUri: vscode.Uri,
    childCount: number
  ) {
    const fileName = fileUri.fsPath.split(/[/\\]/).pop() ?? fileUri.toString();
    super(fileName, vscode.TreeItemCollapsibleState.Collapsed);
    this.tooltip = `${fileUri.fsPath} (${childCount} element(s))`;
    this.description = `${childCount} element(s)`;
    this.iconPath = new vscode.ThemeIcon("file");
    this.contextValue = "sysmlFile";
    this.resourceUri = fileUri;
    this.command = {
      command: "vscode.open",
      title: "Open File",
      arguments: [fileUri],
    };
  }
}

export class ExplorerInfoItem extends vscode.TreeItem {
  readonly itemType = "explorer-info" as const;

  constructor(
    label: string,
    description?: string,
    tooltip?: string,
    iconId: "info" | "warning" | "sync" = "info"
  ) {
    super(label, vscode.TreeItemCollapsibleState.None);
    this.description = description;
    this.tooltip = tooltip ?? label;
    this.iconPath = new vscode.ThemeIcon(iconId);
    this.contextValue = "sysmlExplorerInfo";
  }
}

export type ElementReferenceSummary = {
  id?: string;
  name: string;
  type?: string;
  uri: vscode.Uri;
  range: RangeDTO;
};

type ElementPresentation = {
  description?: string;
  tooltip: string;
};

export type ElementMetadata = {
  reference: ElementReferenceSummary;
  parentId?: string;
};

export type ElementPresentationContext = {
  activeUri?: vscode.Uri;
  metadataById: Map<string, ElementMetadata>;
  incomingRelationshipCounts: Map<string, number>;
};

function tryString(value: unknown): string | undefined {
  return typeof value === "string" && value.trim().length > 0 ? value.trim() : undefined;
}

function basename(uri: vscode.Uri): string {
  return uri.fsPath.split(/[/\\]/).pop() ?? uri.toString();
}

function rangeLabel(range: RangeDTO): string {
  const startLine = range.start.line + 1;
  const startChar = range.start.character + 1;
  const endLine = range.end.line + 1;
  const endChar = range.end.character + 1;
  return startLine === endLine
    ? `L${startLine}:${startChar}-${endChar}`
    : `L${startLine}:${startChar} to L${endLine}:${endChar}`;
}

function flattenAttributeEntries(attributes: Record<string, unknown> | undefined): string[] {
  if (!attributes) {
    return [];
  }
  const preferredKeys = [
    "partType",
    "portType",
    "attributeType",
    "actionType",
    "itemType",
    "flowType",
    "stateType",
    "requirementType",
    "specializes",
    "multiplicity",
    "direction",
  ];
  const entries = preferredKeys
    .map((key) => {
      const value = attributes[key];
      if (typeof value === "string" || typeof value === "number" || typeof value === "boolean") {
        return `${key}: ${String(value)}`;
      }
      if (Array.isArray(value) && value.every((entry) => typeof entry === "string" || typeof entry === "number" || typeof entry === "boolean")) {
        return `${key}: ${value.join(", ")}`;
      }
      return undefined;
    })
    .filter((entry): entry is string => !!entry);
  return Array.from(new Set(entries));
}

function summarizeRelationshipTarget(
  element: SysMLElementDTO,
  relationshipType: "typing" | "specializes",
  context: ElementPresentationContext
): ElementReferenceSummary | undefined {
  const rel = (element.relationships ?? []).find(
    (relationship) => String(relationship.type || "").toLowerCase() === relationshipType
  );
  if (!rel) {
    return undefined;
  }
  return context.metadataById.get(rel.target)?.reference ?? {
    id: rel.target,
    name: rel.name || rel.target,
    uri: context.activeUri ?? vscode.Uri.parse("untitled:unknown"),
    range: element.range,
  };
}

function parentSummary(
  parentItem: ModelTreeItem | FileTreeItem | undefined,
  context: ElementPresentationContext
): ElementReferenceSummary | undefined {
  if (!parentItem || parentItem.itemType !== "sysml-element") {
    return undefined;
  }
  return context.metadataById.get(parentItem.element.id ?? "")?.reference ?? {
    id: parentItem.element.id,
    name: parentItem.element.name || "(anonymous)",
    type: parentItem.element.type,
    uri: parentItem.elementUri,
    range: parentItem.element.range,
  };
}

export function buildElementPresentation(
  element: SysMLElementDTO,
  uri: vscode.Uri,
  parentItem: ModelTreeItem | FileTreeItem | undefined,
  context: ElementPresentationContext
): ElementPresentation {
  const typingTarget = summarizeRelationshipTarget(element, "typing", context);
  const specializationTarget = summarizeRelationshipTarget(element, "specializes", context);
  const multiplicity = tryString(element.attributes?.multiplicity);
  const parent = parentSummary(parentItem, context);
  const activeUri = context.activeUri?.toString();
  const crossFile = !!activeUri && activeUri !== uri.toString();

  const descriptionParts: string[] = [];
  if (typingTarget?.name) {
    descriptionParts.push(`: ${typingTarget.name}`);
  } else if (specializationTarget?.name) {
    descriptionParts.push(`:> ${specializationTarget.name}`);
  }
  if (multiplicity) {
    descriptionParts.push(`[${multiplicity}]`);
  }
  if (descriptionParts.length === 0 && parent?.name) {
    descriptionParts.push(`in ${parent.name}`);
  }
  if (crossFile) {
    descriptionParts.push(`@ ${basename(uri)}`);
  }

  const tooltipParts: string[] = [];
  const qualifiedName = tryString(element.id) ?? element.name ?? "(anonymous)";
  tooltipParts.push(`${element.type}: ${element.name || "(anonymous)"}`);
  tooltipParts.push(`Qualified name: ${qualifiedName}`);
  tooltipParts.push(`Source: ${basename(uri)} ${rangeLabel(element.range)}`);
  if (parent?.name) {
    tooltipParts.push(`Parent: ${parent.name}`);
  }
  if (typingTarget?.name) {
    tooltipParts.push(`Type: ${typingTarget.name}`);
  } else if (tryString(element.attributes?.partType) || tryString(element.attributes?.portType)) {
    tooltipParts.push(`Type: ${tryString(element.attributes?.partType) ?? tryString(element.attributes?.portType)}`);
  }
  if (specializationTarget?.name) {
    tooltipParts.push(`Specializes: ${specializationTarget.name}`);
  } else if (tryString(element.attributes?.specializes)) {
    tooltipParts.push(`Specializes: ${tryString(element.attributes?.specializes)}`);
  }
  if (multiplicity) {
    tooltipParts.push(`Multiplicity: [${multiplicity}]`);
  }
  const outgoingCount = element.relationships?.length ?? 0;
  const incomingCount = context.incomingRelationshipCounts.get(element.id ?? "") ?? 0;
  if (outgoingCount > 0 || incomingCount > 0) {
    tooltipParts.push(`Relationships: ${outgoingCount} outgoing, ${incomingCount} incoming`);
  }
  for (const entry of flattenAttributeEntries(element.attributes).slice(0, 6)) {
    tooltipParts.push(entry);
  }

  return {
    description: descriptionParts.join(" ").trim() || undefined,
    tooltip: tooltipParts.join("\n"),
  };
}

export class ModelTreeItem extends vscode.TreeItem {
  readonly itemType = "sysml-element" as const;
  readonly elementUri: vscode.Uri;
  readonly parentItem?: ModelTreeItem | FileTreeItem;
  childrenItems: ModelTreeItem[] = [];

  constructor(
    public readonly element: SysMLElementDTO,
    uri: vscode.Uri,
    parentItem?: ModelTreeItem | FileTreeItem,
    presentation?: ElementPresentation
  ) {
    const hasChildren =
      (element.children?.length ?? 0) > 0 ||
      (element.relationships?.length ?? 0) > 0 ||
      (element.attributes && Object.keys(element.attributes).length > 0);
    super(
      element.name || "(anonymous)",
      hasChildren
        ? vscode.TreeItemCollapsibleState.Collapsed
        : vscode.TreeItemCollapsibleState.None
    );

    this.elementUri = uri;
    this.parentItem = parentItem;
    this.contextValue =
      element.type === "package" ? "sysmlPackage" : "sysmlElement";
    this.iconPath = iconForElementType(String(element.type || "").toLowerCase());

    // Keep the tree visually simple: primary label only.
    const partType = element.attributes?.partType as string | undefined;
    const portType = element.attributes?.portType as string | undefined;
    const typeName = partType ?? portType;
    const multiplicity = element.attributes?.multiplicity as string | undefined;
    this.label = element.name || "(anonymous)";
    this.description = presentation?.description;

    const tooltipParts: string[] = presentation?.tooltip
      ? [presentation.tooltip]
      : [`${element.type}: ${element.name || "(anonymous)"}`];
    if (element.type === "package") {
      const stats = computePackageStats(element);
      tooltipParts.push(`Parts: ${stats.parts}`);
      tooltipParts.push(`Part defs: ${stats.partDefs}`);
      tooltipParts.push(`Ports: ${stats.ports}`);
    }
    if (!presentation?.tooltip) {
      if (typeName) tooltipParts.push(`Type: ${typeName}`);
      if (multiplicity) tooltipParts.push(`Multiplicity: [${multiplicity}]`);
      if (element.children?.length) tooltipParts.push(`Children: ${element.children.length}`);
      if (element.relationships?.length) tooltipParts.push(`Relationships: ${element.relationships.length}`);
    }
    this.tooltip = tooltipParts.join("\n");

    this.command = {
      command: "sysml.openLocation",
      title: "Open Location",
      arguments: [this],
    };
  }

  private computePackageStats(root: SysMLElementDTO): { parts: number; partDefs: number; ports: number } {
    const stats = { parts: 0, partDefs: 0, ports: 0 };
    const walk = (node: SysMLElementDTO): void => {
      const type = String(node.type || "").toLowerCase();
      if (type === "part") stats.parts += 1;
      if (type === "part def") stats.partDefs += 1;
      if (type.includes("port")) stats.ports += 1;
      for (const child of node.children ?? []) {
        walk(child);
      }
    };
    walk(root);
    return stats;
  }
}

function iconForElementType(elementType: string): vscode.ThemeIcon {
  switch (elementType) {
    case "package":
    case "namespace":
      return new vscode.ThemeIcon("package");
    case "part def":
      return new vscode.ThemeIcon("symbol-class");
    case "part":
      return new vscode.ThemeIcon("symbol-object");
    case "port def":
    case "interface":
      return new vscode.ThemeIcon("symbol-interface");
    case "port":
      return new vscode.ThemeIcon("plug");
    case "attribute def":
    case "attribute":
      return new vscode.ThemeIcon("symbol-property");
    case "action def":
      return new vscode.ThemeIcon("symbol-method");
    case "action":
      return new vscode.ThemeIcon("run");
    case "requirement":
    case "requirement def":
      return new vscode.ThemeIcon("checklist");
    default:
      return new vscode.ThemeIcon("symbol-misc");
  }
}

function computePackageStats(root: SysMLElementDTO): {
  parts: number;
  partDefs: number;
  ports: number;
} {
  const stats = { parts: 0, partDefs: 0, ports: 0 };
  const walk = (node: SysMLElementDTO): void => {
    const type = String(node.type || "").toLowerCase();
    if (type === "part") stats.parts += 1;
    if (type === "part def") stats.partDefs += 1;
    if (type.includes("port")) stats.ports += 1;
    for (const child of node.children ?? []) {
      walk(child);
    }
  };
  walk(root);
  return stats;
}

function percentileMs(values: number[], percentile: number): number {
  if (values.length === 0) return 0;
  const sorted = [...values].sort((a, b) => a - b);
  const index = Math.min(
    sorted.length - 1,
    Math.max(0, Math.ceil((percentile / 100) * sorted.length) - 1)
  );
  return Math.round(sorted[index]);
}

type ExplorerTreeItem = ExplorerInfoItem | FileTreeItem | ModelTreeItem;

type WorkspaceLoadStatus = {
  state: "idle" | "indexing" | "ready" | "degraded";
  scannedFiles: number;
  loadedFiles: number;
  perPatternLimit?: number;
  truncated: boolean;
  cancelled: boolean;
  failures: number;
};

type ModelExplorerDebugState = {
  lastRevealedElementId?: string;
  pendingDocumentLoadUri?: string;
  pendingWorkspaceLoadRunId?: string;
};

type InFlightDocumentLoad = {
  uri: string;
  generation: number;
  cts: vscode.CancellationTokenSource;
  promise: Promise<void>;
};

type WorkspaceLoadOptions = {
  runId: string;
  token?: vscode.CancellationToken;
};

type WorkspaceLoadResult = {
  runId: string;
  fileCount: number;
  loadedFiles: number;
  failures: number;
  cancelled: number;
  committed: boolean;
  stale: boolean;
  totalMs: number;
};

type InFlightWorkspaceLoad = {
  runId: string;
  promise: Promise<WorkspaceLoadResult>;
};

export class ModelExplorerProvider
  implements vscode.TreeDataProvider<ExplorerTreeItem>
{
  private readonly _onDidChangeTreeData = new vscode.EventEmitter<
    ExplorerTreeItem | undefined | void
  >();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  private lastUri: vscode.Uri | undefined;
  private lastElements: SysMLElementDTO[] | undefined;

  private workspaceFileData = new Map<
    string,
    { uri: vscode.Uri; elements: SysMLElementDTO[] }
  >();
  private workspaceSemanticElements: SysMLElementDTO[] = [];
  private workspaceFileUris: vscode.Uri[] = [];
  private _workspaceViewMode: "byFile" | "bySemantic" = "bySemantic";
  private treeView?: vscode.TreeView<ExplorerTreeItem>;
  private uriToRootItems = new Map<string, ExplorerTreeItem[]>();
  private rootItemsCache: ExplorerTreeItem[] | undefined;
  private elementIndex = new Map<string, ModelTreeItem>();
  private metadataById = new Map<string, ElementMetadata>();
  private incomingRelationshipCounts = new Map<string, number>();
  private lastRevealedElementId: string | undefined;
  private workspaceLoadStatus: WorkspaceLoadStatus = {
    state: "idle",
    scannedFiles: 0,
    loadedFiles: 0,
    truncated: false,
    cancelled: false,
    failures: 0,
  };
  private documentLoadState: "idle" | "loading" | "ready" | "error" = "idle";
  private documentLoadGeneration = 0;
  private inFlightDocumentLoad: InFlightDocumentLoad | undefined;
  private inFlightWorkspaceLoad: InFlightWorkspaceLoad | undefined;

  constructor(private readonly modelProvider: LspModelProvider) {}

  setTreeView(treeView: vscode.TreeView<ExplorerTreeItem>): void {
    this.treeView = treeView;
  }

  getDebugState(): ModelExplorerDebugState {
    return {
      lastRevealedElementId: this.lastRevealedElementId,
      pendingDocumentLoadUri: this.inFlightDocumentLoad?.uri,
      pendingWorkspaceLoadRunId: this.inFlightWorkspaceLoad?.runId,
    };
  }

  getParent(element: ExplorerTreeItem): ExplorerTreeItem | undefined {
    if (element.itemType === "sysml-element") {
      return element.parentItem;
    }
    return undefined;
  }

  isWorkspaceBacked(): boolean {
    return (
      this.workspaceFileData.size > 0 ||
      this.workspaceFileUris.length > 0 ||
      this.workspaceLoadStatus.state === "indexing"
    );
  }

  getWorkspaceFileUris(): vscode.Uri[] {
    return this.workspaceFileUris;
  }

  getWorkspaceViewMode(): "byFile" | "bySemantic" {
    return this._workspaceViewMode;
  }

  setWorkspaceViewMode(mode: "byFile" | "bySemantic"): void {
    log("setWorkspaceViewMode:", mode);
    this._workspaceViewMode = mode;
    vscode.commands.executeCommand(
      "setContext",
      "sysml.workspaceViewMode",
      this._workspaceViewMode
    );
    this.invalidateTreeCache();
    this._onDidChangeTreeData.fire();
  }

  toggleWorkspaceViewMode(): void {
    log("toggleWorkspaceViewMode:", this._workspaceViewMode, "->", this._workspaceViewMode === "byFile" ? "bySemantic" : "byFile");
    this._workspaceViewMode =
      this._workspaceViewMode === "byFile" ? "bySemantic" : "byFile";
    vscode.commands.executeCommand(
      "setContext",
      "sysml.workspaceViewMode",
      this._workspaceViewMode
    );
    this.invalidateTreeCache();
    this._onDidChangeTreeData.fire();
  }

  async revealActiveDocument(docUri: vscode.Uri): Promise<void> {
    if (!this.treeView) return;
    this.ensureTreeCache();
    const items = this.uriToRootItems.get(docUri.toString());
    if (!items?.length) return;
    const seen = new Set<ExplorerTreeItem>();
    for (const item of items) {
      if (seen.has(item)) continue;
      seen.add(item);
      try {
        await this.treeView.reveal(item, {
          select: true,
          focus: false,
          expand: true,
        });
      } catch {
        // Ignore
      }
    }
  }

  async revealElement(
    docUri: vscode.Uri,
    elementId?: string,
    range?: RangeDTO
  ): Promise<void> {
    if (!this.treeView) return;
    const startedAt = Date.now();
    this.ensureTreeCache();
    const item = this.findElementTreeItem(docUri, elementId, range);
    if (!item) {
      logPerf("modelExplorer:revealElementMiss", {
        uri: docUri.toString(),
        elementId,
        totalMs: Date.now() - startedAt,
      });
      return;
    }
    try {
      await this.treeView.reveal(item, {
        select: true,
        focus: false,
        expand: true,
      });
      this.lastRevealedElementId = item.element.id;
      logPerf("modelExplorer:revealElement", {
        uri: docUri.toString(),
        elementId: item.element.id,
        totalMs: Date.now() - startedAt,
      });
    } catch {
      // Ignore reveal failures when the view is not visible yet.
      logPerf("modelExplorer:revealElementFailed", {
        uri: docUri.toString(),
        elementId: item.element.id,
        totalMs: Date.now() - startedAt,
      });
    }
  }

  clear(): void {
    this.lastUri = undefined;
    this.lastElements = undefined;
    this.workspaceFileData.clear();
    this.workspaceSemanticElements = [];
    this.workspaceFileUris = [];
    this.uriToRootItems.clear();
    this.rootItemsCache = undefined;
    this.elementIndex.clear();
    this.metadataById.clear();
    this.incomingRelationshipCounts.clear();
    this.lastRevealedElementId = undefined;
    this.inFlightDocumentLoad?.cts.cancel();
    this.inFlightDocumentLoad?.cts.dispose();
    this.inFlightWorkspaceLoad = undefined;
    this.workspaceLoadStatus = {
      state: "idle",
      scannedFiles: 0,
      loadedFiles: 0,
      truncated: false,
      cancelled: false,
      failures: 0,
    };
    this.documentLoadState = "idle";
    this.inFlightDocumentLoad = undefined;
    this._onDidChangeTreeData.fire();
  }

  hasWorkspaceData(): boolean {
    return this.workspaceFileData.size > 0 || this.workspaceSemanticElements.length > 0;
  }

  setWorkspaceLoadStatus(status: Partial<WorkspaceLoadStatus>): void {
    this.workspaceLoadStatus = {
      ...this.workspaceLoadStatus,
      ...status,
    };
    this.invalidateTreeCache();
    this._onDidChangeTreeData.fire();
  }

  async loadWorkspaceModel(
    fileUris: vscode.Uri[],
    options: WorkspaceLoadOptions
  ): Promise<WorkspaceLoadResult> {
    log(
      "loadWorkspaceModel:",
      fileUris.length,
      "files. URIs:",
      fileUris.map((u) => u.toString())
    );
    if (this.inFlightWorkspaceLoad?.runId === options.runId) {
      return await this.inFlightWorkspaceLoad.promise;
    }
    const anchorUri =
      fileUris[0] ??
      this.lastUri ??
      vscode.window.activeTextEditor?.document.uri;
    const loadStartedAt = Date.now();
    const localWorkspaceFileData = new Map<
      string,
      { uri: vscode.Uri; elements: SysMLElementDTO[] }
    >();
    let localWorkspaceSemanticElements: SysMLElementDTO[] = [];
    let localWorkspaceFileUris: vscode.Uri[] = [];
    let failures = 0;
    let cancelled = 0;
    let backendScannedFiles = 0;
    let backendLoadedFiles = 0;

    const runPromise = async (): Promise<WorkspaceLoadResult> => {
      try {
        if (!anchorUri) {
          log("loadWorkspaceModel: no anchor URI available");
        } else {
          const anchorUriStr = anchorUri.toString();
          log("loadWorkspaceModel: requesting backend workspace model for", anchorUriStr);
          const result = await this.modelProvider.getModel(
            anchorUriStr,
            ["graph", "stats", "workspaceVisualization"],
            options.token,
            "modelExplorer.loadWorkspaceModel"
          );
          if (
            options.token?.isCancellationRequested ||
            this.inFlightWorkspaceLoad?.runId !== options.runId
          ) {
            cancelled += 1;
            logPerf("modelExplorer:workspaceLoadDropped", {
              runId: options.runId,
              anchorUri: anchorUriStr,
              reason: options.token?.isCancellationRequested ? "cancelled" : "stale-run",
              totalMs: Date.now() - loadStartedAt,
            });
          } else {
            const workspaceModel = result.workspaceModel;
            backendScannedFiles = workspaceModel?.summary?.scannedFiles ?? 0;
            backendLoadedFiles = workspaceModel?.summary?.loadedFiles ?? 0;
            failures = workspaceModel?.summary?.failures ?? 0;
            localWorkspaceSemanticElements = workspaceModel?.semantic ?? [];
            for (const fileEntry of workspaceModel?.files ?? []) {
              const fileUri = vscode.Uri.parse(fileEntry.uri);
              localWorkspaceFileUris.push(fileUri);
              localWorkspaceFileData.set(fileUri.toString(), {
                uri: fileUri,
                elements: fileEntry.elements ?? [],
              });
            }
            logPerf("modelExplorer:workspaceModelLoaded", {
              runId: options.runId,
              anchorUri: anchorUriStr,
              totalMs: Date.now() - loadStartedAt,
              scannedFiles: backendScannedFiles,
              loadedFiles: backendLoadedFiles,
              semanticRoots: localWorkspaceSemanticElements.length,
              fileRoots: localWorkspaceFileData.size,
              nodeCount: result.graph?.nodes?.length ?? 0,
              edgeCount: result.graph?.edges?.length ?? 0,
              parseTimeMs: result.stats?.parseTimeMs,
              modelBuildTimeMs: result.stats?.modelBuildTimeMs,
            });
          }
        }
      } finally {
        const totalMs = Date.now() - loadStartedAt;
        const stale = this.inFlightWorkspaceLoad?.runId !== options.runId;
        const committed = !stale && !options.token?.isCancellationRequested;
        if (committed) {
          this.workspaceFileData = localWorkspaceFileData;
          this.workspaceSemanticElements = localWorkspaceSemanticElements;
          this.workspaceFileUris = localWorkspaceFileUris;
          this.invalidateTreeCache();
          this._onDidChangeTreeData.fire();
        } else {
          logPerf("modelExplorer:workspaceLoadStaleResultDropped", {
            runId: options.runId,
            workspaceBacked: this.isWorkspaceBacked(),
            workspaceViewMode: this._workspaceViewMode,
            cancelled: options.token?.isCancellationRequested ?? false,
            totalMs,
          });
        }
        log("loadWorkspaceModel: done,", localWorkspaceFileData.size, "files loaded");
        logStartupEvent("explorer:loadWorkspaceModel", {
          runId: options.runId,
          fileCount: backendScannedFiles || fileUris.length,
          loadedFiles: backendLoadedFiles || localWorkspaceFileData.size,
          failures,
          cancelled,
          committed,
          stale,
          totalMs,
          anchorUri: anchorUri?.toString(),
        });
      }

      return {
        runId: options.runId,
        fileCount: backendScannedFiles || fileUris.length,
        loadedFiles: backendLoadedFiles || localWorkspaceFileData.size,
        failures,
        cancelled,
        committed:
          this.inFlightWorkspaceLoad?.runId === options.runId &&
          !options.token?.isCancellationRequested,
        stale: this.inFlightWorkspaceLoad?.runId !== options.runId,
        totalMs: Date.now() - loadStartedAt,
      };
    };

    const promise = runPromise().finally(() => {
      if (this.inFlightWorkspaceLoad?.runId === options.runId) {
        this.inFlightWorkspaceLoad = undefined;
      }
    });

    this.inFlightWorkspaceLoad = {
      runId: options.runId,
      promise,
    };
    return await promise;
  }

  async loadDocument(
    document: vscode.TextDocument,
    token?: vscode.CancellationToken
  ): Promise<void> {
    const uriString = document.uri.toString();
    if (this.inFlightDocumentLoad?.uri === uriString) {
      log("loadDocument: joining in-flight load for", uriString);
      logPerf("modelExplorer:loadDocumentJoin", {
        uri: uriString,
        generation: this.inFlightDocumentLoad.generation,
      });
      return await this.inFlightDocumentLoad.promise;
    }
    this.inFlightDocumentLoad?.cts.cancel();
    this.inFlightDocumentLoad?.cts.dispose();
    const cts = new vscode.CancellationTokenSource();
    const effectiveToken = token
      ? ModelExplorerProvider.mergeCancellationTokens(token, cts.token)
      : cts.token;
    const generation = ++this.documentLoadGeneration;
    const promise = this.performDocumentLoad(document, generation, effectiveToken)
      .finally(() => {
        if (
          this.inFlightDocumentLoad?.uri === uriString &&
          this.inFlightDocumentLoad.generation === generation
        ) {
          this.inFlightDocumentLoad.cts.dispose();
          this.inFlightDocumentLoad = undefined;
        } else {
          cts.dispose();
        }
      });
    this.inFlightDocumentLoad = {
      uri: uriString,
      generation,
      cts,
      promise,
    };
    return await promise;
  }

  private static mergeCancellationTokens(
    first: vscode.CancellationToken,
    second: vscode.CancellationToken
  ): vscode.CancellationToken {
    const cts = new vscode.CancellationTokenSource();
    const cancel = () => {
      if (!cts.token.isCancellationRequested) {
        cts.cancel();
      }
    };
    first.onCancellationRequested(cancel);
    second.onCancellationRequested(cancel);
    return cts.token;
  }

  private async performDocumentLoad(
    document: vscode.TextDocument,
    generation: number,
    token?: vscode.CancellationToken
  ): Promise<void> {
    log("loadDocument:", document.uri.toString().slice(-50));
    const startedAt = Date.now();
    this.lastUri = document.uri;
    this.documentLoadState = "loading";
    this.invalidateTreeCache();
    this._onDidChangeTreeData.fire();

    try {
      const result = await this.modelProvider.getModel(
        document.uri.toString(),
        ["graph", "stats"],
        token,
        "modelExplorer.loadDocument"
      );
      if (generation !== this.documentLoadGeneration) {
        logPerf("modelExplorer:loadDocumentStale", {
          uri: document.uri.toString(),
          generation,
          activeGeneration: this.documentLoadGeneration,
          totalMs: Date.now() - startedAt,
        });
        return;
      }
      const graphTransformStartedAt = Date.now();
      this.lastElements = result.graph
        ? (graphToElementTree(result.graph) as SysMLElementDTO[])
        : [];
      const graphTransformMs = Date.now() - graphTransformStartedAt;
      this.documentLoadState = "ready";
      log("loadDocument: done,", this.lastElements.length, "elements");
      logPerf("modelExplorer:loadDocument", {
        uri: document.uri.toString(),
        totalMs: Date.now() - startedAt,
        graphTransformMs,
        elementCount: this.lastElements.length,
        nodeCount: result.graph?.nodes?.length ?? 0,
        edgeCount: result.graph?.edges?.length ?? 0,
        parseTimeMs: result.stats?.parseTimeMs,
        modelBuildTimeMs: result.stats?.modelBuildTimeMs,
      });
    } catch (error) {
      if (generation !== this.documentLoadGeneration) {
        logPerf("modelExplorer:loadDocumentStale", {
          uri: document.uri.toString(),
          generation,
          activeGeneration: this.documentLoadGeneration,
          totalMs: Date.now() - startedAt,
        });
        return;
      }
      if (error instanceof vscode.CancellationError || token?.isCancellationRequested) {
        log("loadDocument: cancelled for", document.uri.toString());
        logPerf("modelExplorer:loadDocumentCancelled", {
          uri: document.uri.toString(),
          totalMs: Date.now() - startedAt,
        });
      } else {
        logError(`loadDocument failed for ${document.uri.toString()}`, error);
        logPerf("modelExplorer:loadDocumentFailed", {
          uri: document.uri.toString(),
          totalMs: Date.now() - startedAt,
          error: error instanceof Error ? error.message : String(error),
        });
      }
      this.lastElements = [];
      this.documentLoadState = "error";
    } finally {
      this.invalidateTreeCache();
      this._onDidChangeTreeData.fire();
    }
  }

  refresh(): void {
    log("refresh: workspaceBacked=", this.isWorkspaceBacked(), "fileCount=", this.workspaceFileUris.length);
    if (this.isWorkspaceBacked()) {
      this.invalidateTreeCache();
      this._onDidChangeTreeData.fire();
    } else if (this.lastUri) {
      const doc = vscode.workspace.textDocuments.find(
        (d) => d.uri.toString() === this.lastUri!.toString()
      );
      if (doc && (doc.languageId === "sysml" || doc.languageId === "kerml")) {
        this.loadDocument(doc);
      } else {
        this._onDidChangeTreeData.fire();
      }
    } else {
      const active = vscode.window.activeTextEditor?.document;
      if (active && (active.languageId === "sysml" || active.languageId === "kerml")) {
        this.loadDocument(active);
      } else {
        this._onDidChangeTreeData.fire();
      }
    }
  }

  getAllElements(): SysMLElementDTO[] {
    if (this.hasWorkspaceData()) {
      const fileElements = Array.from(this.workspaceFileData.values()).flatMap(
        (d) => d.elements
      );
      return fileElements.length > 0 ? fileElements : this.workspaceSemanticElements;
    }
    return this.lastElements ?? [];
  }

  getLastUri(): vscode.Uri | undefined {
    return this.lastUri;
  }

  getTreeItem(element: ExplorerTreeItem): vscode.TreeItem {
    return element;
  }

  async getChildren(element?: ExplorerTreeItem): Promise<ExplorerTreeItem[]> {
    if (!element) {
      const active = vscode.window.activeTextEditor?.document;
      if (
        !this.lastUri &&
        !this.isWorkspaceBacked() &&
        (!active ||
          (active.languageId !== "sysml" && active.languageId !== "kerml"))
      ) {
        return [];
      }
      return this.ensureTreeCache();
    }

    if (element.itemType === "file-node") {
      return element.childrenItems;
    }

    if (element.itemType === "explorer-info") {
      return [];
    }

    return element.childrenItems;
  }

  private getWorkspaceInfoItems(): ExplorerInfoItem[] {
    const status = this.workspaceLoadStatus;
    if (status.state === "idle") {
      return [];
    }

    const details = `Scanned ${status.scannedFiles} file(s), loaded ${status.loadedFiles} file(s)${status.failures > 0 ? `, ${status.failures} failed` : ""}${status.perPatternLimit ? `, limit ${status.perPatternLimit} per folder/type` : ""}`;
    if (status.state === "indexing") {
      return [
        new ExplorerInfoItem(
          "Workspace indexing in progress",
          `${status.scannedFiles} scanned`,
          `${details}. Results may still be incomplete.`,
          "sync"
        ),
      ];
    }
    if (status.truncated || status.cancelled || status.failures > 0) {
      return [
        new ExplorerInfoItem(
          "Workspace results may be incomplete",
          `${status.loadedFiles}/${status.scannedFiles} loaded`,
          `${details}${status.truncated ? ". Discovery limit reached." : ""}${status.cancelled ? ". Indexing was cancelled." : ""}`,
          "warning"
        ),
      ];
    }
    return [
      new ExplorerInfoItem(
        "Workspace indexed",
        `${status.loadedFiles} loaded`,
        details,
        "info"
      ),
    ];
  }

  private buildSemanticUriMapping(rootItems: ModelTreeItem[]): void {
    this.uriToRootItems.clear();
    for (const [uriStr, data] of this.workspaceFileData) {
      const matching: ExplorerTreeItem[] = [];
      for (const el of data.elements) {
        const key = `${el.type}::${el.name || "(anonymous)"}`;
        const match = rootItems.find(
          (item) =>
            `${item.element.type}::${item.element.name || "(anonymous)"}` === key
        );
        if (match && !matching.includes(match)) {
          matching.push(match);
        }
      }
      if (matching.length > 0) {
        this.uriToRootItems.set(uriStr, matching);
      }
    }
  }

  private mergeElements(elements: SysMLElementDTO[]): SysMLElementDTO[] {
    const mergedMap = new Map<string, SysMLElementDTO>();
    const result: SysMLElementDTO[] = [];

    for (const el of elements) {
      const key = `${el.type}::${el.name || "(anonymous)"}`;
      if (this.namespaceTypes.has(el.type) && mergedMap.has(key)) {
        const existing = mergedMap.get(key)!;
        const merged = this.mergeTwo(existing, el);
        const idx = result.indexOf(existing);
        if (idx !== -1) result[idx] = merged;
        mergedMap.set(key, merged);
      } else if (this.namespaceTypes.has(el.type)) {
        const clone = this.cloneElement(el);
        mergedMap.set(key, clone);
        result.push(clone);
      } else {
        result.push(el);
      }
    }
    return result;
  }

  private readonly namespaceTypes = new Set(["package"]);

  private mergeTwo(a: SysMLElementDTO, b: SysMLElementDTO): SysMLElementDTO {
    const childMap = new Map<string, SysMLElementDTO>();
    for (const c of a.children ?? []) {
      const ck = `${c.type}::${c.name || "(anonymous)"}`;
      childMap.set(ck, c);
    }
    for (const child of b.children ?? []) {
      const ck = `${child.type}::${child.name || "(anonymous)"}`;
      const existing = childMap.get(ck);
      if (existing && this.namespaceTypes.has(child.type)) {
        childMap.set(ck, this.mergeTwo(existing, child));
      } else if (!existing) {
        childMap.set(ck, child);
      }
    }
    const children = Array.from(childMap.values());

    const attrs = { ...(a.attributes ?? {}), ...(b.attributes ?? {}) };
    const relKeys = new Set(
      (a.relationships ?? []).map((r) => `${r.type}::${r.source}::${r.target}`)
    );
    const relationships: RelationshipDTO[] = [...(a.relationships ?? [])];
    for (const rel of b.relationships ?? []) {
      const rk = `${rel.type}::${rel.source}::${rel.target}`;
      if (!relKeys.has(rk)) {
        relationships.push(rel);
        relKeys.add(rk);
      }
    }

    return {
      ...a,
      children,
      attributes: attrs,
      relationships,
    };
  }

  private cloneElement(el: SysMLElementDTO): SysMLElementDTO {
    return {
      id: el.id,
      type: el.type,
      name: el.name,
      range: el.range,
      children: (el.children ?? []).map((c) => this.cloneElement(c)),
      attributes: el.attributes ? { ...el.attributes } : {},
      relationships: [...(el.relationships ?? [])],
      errors: el.errors ? [...el.errors] : undefined,
    };
  }

  private invalidateTreeCache(): void {
    this.rootItemsCache = undefined;
    this.elementIndex.clear();
    this.uriToRootItems.clear();
    this.metadataById.clear();
    this.incomingRelationshipCounts.clear();
  }

  private ensureTreeCache(): ExplorerTreeItem[] {
    if (this.rootItemsCache) {
      return this.rootItemsCache;
    }
    const startedAt = Date.now();

    const infoItems = this.getWorkspaceInfoItems();
    if (this._workspaceViewMode === "byFile" && this.workspaceFileData.size > 0) {
      const metadataStartedAt = Date.now();
      this.buildElementMetadata(
        Array.from(this.workspaceFileData.values()).flatMap((data) =>
          data.elements.map((element) => ({ element, uri: data.uri }))
        )
      );
      const metadataMs = Date.now() - metadataStartedAt;
      const itemBuildStartedAt = Date.now();
      const fileItems = Array.from(this.workspaceFileData.values())
        .sort((a, b) => a.uri.fsPath.localeCompare(b.uri.fsPath))
        .map((data) => this.createFileItem(data.uri, data.elements));
      const itemBuildMs = Date.now() - itemBuildStartedAt;
      this.uriToRootItems.clear();
      for (const fileItem of fileItems) {
        this.uriToRootItems.set(fileItem.fileUri.toString(), [fileItem]);
      }
      this.rootItemsCache = [...infoItems, ...fileItems];
      logPerf("modelExplorer:buildTreeCache", {
        mode: "workspace-byFile",
        metadataMs,
        itemBuildMs,
        totalMs: Date.now() - startedAt,
        rootItemCount: this.rootItemsCache.length,
        fileCount: this.workspaceFileData.size,
      });
      return this.rootItemsCache;
    }
    if (this._workspaceViewMode === "bySemantic" && this.workspaceSemanticElements.length > 0) {
      const metadataStartedAt = Date.now();
      this.buildElementMetadata(
        this.workspaceSemanticElements.map((element) => ({
          element,
          uri: this.elementUriFor(element),
        }))
      );
      const metadataMs = Date.now() - metadataStartedAt;
      const itemBuildStartedAt = Date.now();
      const items = this.workspaceSemanticElements.map((element) =>
        this.createModelTreeItem(element, this.elementUriFor(element))
      );
      const itemBuildMs = Date.now() - itemBuildStartedAt;
      this.buildSemanticUriMapping(items);
      this.rootItemsCache = [...infoItems, ...items];
      logPerf("modelExplorer:buildTreeCache", {
        mode: "workspace-bySemantic",
        metadataMs,
        itemBuildMs,
        totalMs: Date.now() - startedAt,
        rootItemCount: this.rootItemsCache.length,
        fileCount: this.workspaceFileData.size,
        semanticRootCount: this.workspaceSemanticElements.length,
      });
      return this.rootItemsCache;
    }

    if (this._workspaceViewMode === "bySemantic" && infoItems.length > 0) {
      this.rootItemsCache = infoItems;
      return this.rootItemsCache;
    }

    if (this.documentLoadState === "loading") {
      this.rootItemsCache = [
        new ExplorerInfoItem(
          "Loading model...",
          "Parsing and indexing",
          "The language server is building the semantic model.",
          "sync"
        ),
      ];
      logPerf("modelExplorer:buildTreeCache", {
        mode: "document-loading",
        totalMs: Date.now() - startedAt,
      });
      return this.rootItemsCache;
    }

    if (
      !this.isWorkspaceBacked() &&
      !this.lastUri &&
      !this.lastElements &&
      vscode.window.activeTextEditor &&
      (vscode.window.activeTextEditor.document.languageId === "sysml" ||
        vscode.window.activeTextEditor.document.languageId === "kerml")
    ) {
      this.rootItemsCache = [
        new ExplorerInfoItem(
          "Model pending",
          "Waiting for active document load",
          "The Model Explorer is waiting for the coordinated active-document load to finish.",
          "sync"
        ),
      ];
      logPerf("modelExplorer:buildTreeCache", {
        mode: "document-pending",
        totalMs: Date.now() - startedAt,
      });
      return this.rootItemsCache;
    }

    if (this.lastUri && this.lastElements) {
      const mergeStartedAt = Date.now();
      const merged = this.mergeElements(this.lastElements);
      const mergeMs = Date.now() - mergeStartedAt;
      const metadataStartedAt = Date.now();
      this.buildElementMetadata(merged.map((element) => ({ element, uri: this.lastUri! })));
      const metadataMs = Date.now() - metadataStartedAt;
      const itemBuildStartedAt = Date.now();
      const items = merged.map((e) => this.createModelTreeItem(e, this.lastUri!));
      const itemBuildMs = Date.now() - itemBuildStartedAt;
      this.uriToRootItems.set(this.lastUri.toString(), items);
      if (items.length === 0 && this.documentLoadState === "ready") {
        this.rootItemsCache = [
          new ExplorerInfoItem(
            "No model elements found",
            "0 loaded",
            "The active file has no extracted model elements yet.",
            "info"
          ),
        ];
        logPerf("modelExplorer:buildTreeCache", {
          mode: "document-empty",
          mergeMs,
          metadataMs,
          itemBuildMs,
          totalMs: Date.now() - startedAt,
          rootItemCount: this.rootItemsCache.length,
        });
        return this.rootItemsCache;
      }
      this.rootItemsCache = items;
      logPerf("modelExplorer:buildTreeCache", {
        mode: "document",
        mergeMs,
        metadataMs,
        itemBuildMs,
        totalMs: Date.now() - startedAt,
        rootItemCount: items.length,
        elementCount: this.lastElements.length,
      });
      return this.rootItemsCache;
    }

    this.rootItemsCache = [];
    logPerf("modelExplorer:buildTreeCache", {
      mode: "empty",
      totalMs: Date.now() - startedAt,
    });
    return this.rootItemsCache;
  }

  private createFileItem(uri: vscode.Uri, elements: SysMLElementDTO[]): FileTreeItem {
    const item = new FileTreeItem(uri, elements.length);
    this.uriToRootItems.set(uri.toString(), [item]);
    item.childrenItems = elements.map((element) =>
      this.createModelTreeItem(element, uri, item)
    );
    return item;
  }

  private createModelTreeItem(
    element: SysMLElementDTO,
    uri: vscode.Uri,
    parentItem?: ModelTreeItem | FileTreeItem
  ): ModelTreeItem {
    const item = new ModelTreeItem(
      element,
      uri,
      parentItem,
      buildElementPresentation(element, uri, parentItem, {
        activeUri: vscode.window.activeTextEditor?.document.uri,
        metadataById: this.metadataById,
        incomingRelationshipCounts: this.incomingRelationshipCounts,
      })
    );
    item.childrenItems = (element.children ?? []).map((child) =>
      this.createModelTreeItem(child, uri, item)
    );
    this.registerElementItem(item);
    return item;
  }

  private registerElementItem(item: ModelTreeItem): void {
    const byId = this.elementIdKey(item.elementUri, item.element.id);
    if (byId && !this.elementIndex.has(byId)) {
      this.elementIndex.set(byId, item);
    }
    const byRange = this.elementRangeKey(item.elementUri, item.element.range);
    if (byRange && !this.elementIndex.has(byRange)) {
      this.elementIndex.set(byRange, item);
    }
    const byIdOnly = this.elementIdOnlyKey(item.element.id);
    if (byIdOnly && !this.elementIndex.has(byIdOnly)) {
      this.elementIndex.set(byIdOnly, item);
    }
  }

  private findElementTreeItem(
    docUri: vscode.Uri,
    elementId?: string,
    range?: RangeDTO
  ): ModelTreeItem | undefined {
    const byId = this.elementIdKey(docUri, elementId);
    if (byId && this.elementIndex.has(byId)) {
      return this.elementIndex.get(byId);
    }
    const byRange = this.elementRangeKey(docUri, range);
    if (byRange && this.elementIndex.has(byRange)) {
      return this.elementIndex.get(byRange);
    }
    const byIdOnly = this.elementIdOnlyKey(elementId);
    if (byIdOnly && this.elementIndex.has(byIdOnly)) {
      return this.elementIndex.get(byIdOnly);
    }
    return undefined;
  }

  private elementIdKey(uri: vscode.Uri, elementId?: string): string | undefined {
    if (!elementId) {
      return undefined;
    }
    return `${uri.toString().toLowerCase()}::${elementId.toLowerCase()}`;
  }

  private elementIdOnlyKey(elementId?: string): string | undefined {
    if (!elementId) {
      return undefined;
    }
    return `id::${elementId.toLowerCase()}`;
  }

  private elementRangeKey(uri: vscode.Uri, range?: RangeDTO): string | undefined {
    if (!range) {
      return undefined;
    }
    return `${uri.toString().toLowerCase()}::${range.start.line}:${range.start.character}-${range.end.line}:${range.end.character}`;
  }

  private buildElementMetadata(
    entries: { element: SysMLElementDTO; uri: vscode.Uri }[]
  ): void {
    this.metadataById.clear();
    this.incomingRelationshipCounts.clear();
    for (const { element, uri } of entries) {
      this.collectElementMetadata(element, uri);
    }
  }

  private collectElementMetadata(
    element: SysMLElementDTO,
    uri: vscode.Uri,
    parentId?: string
  ): void {
    if (element.id) {
      this.metadataById.set(element.id, {
        reference: {
          id: element.id,
          name: element.name || "(anonymous)",
          type: element.type,
          uri,
          range: element.range,
        },
        parentId,
      });
    }
    for (const relationship of element.relationships ?? []) {
      const targetId = relationship.target;
      if (targetId) {
        this.incomingRelationshipCounts.set(
          targetId,
          (this.incomingRelationshipCounts.get(targetId) ?? 0) + 1
        );
      }
    }
    for (const child of element.children ?? []) {
      this.collectElementMetadata(child, uri, element.id);
    }
  }

  private elementUriFor(element: SysMLElementDTO): vscode.Uri {
    if (element.uri) {
      try {
        return vscode.Uri.parse(element.uri);
      } catch {
        // Fall back below.
      }
    }
    return this.lastUri ?? this.workspaceFileUris[0] ?? vscode.Uri.parse("untitled:unknown");
  }
}

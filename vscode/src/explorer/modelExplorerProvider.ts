import * as vscode from "vscode";
import { log, logError } from "../logger";
import type { LspModelProvider } from "../providers/lspModelProvider";
import type {
  SysMLElementDTO,
  RelationshipDTO,
  RangeDTO,
} from "../providers/sysmlModelTypes";
import { graphToElementTree } from "../visualization/prepareData";

/** Helper to convert RangeDTO to vscode.Range for openLocation. */
export function toVscodeRange(dto: RangeDTO): vscode.Range {
  return new vscode.Range(
    new vscode.Position(dto.start.line, dto.start.character),
    new vscode.Position(dto.end.line, dto.end.character)
  );
}

export class FileTreeItem extends vscode.TreeItem {
  readonly itemType = "file-node" as const;

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

export class ModelTreeItem extends vscode.TreeItem {
  readonly itemType = "sysml-element" as const;
  readonly elementUri: vscode.Uri;

  constructor(
    public readonly element: SysMLElementDTO,
    uri: vscode.Uri
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
    this.resourceUri = uri;
    this.contextValue =
      element.type === "package" ? "sysmlPackage" : "sysmlElement";

    // Build label: name : Type [mult] when attributes available
    const partType = element.attributes?.partType as string | undefined;
    const portType = element.attributes?.portType as string | undefined;
    const typeName = partType ?? portType;
    const multiplicity = element.attributes?.multiplicity as string | undefined;
    let labelText = element.name || "(anonymous)";
    if (typeName) labelText += ` : ${typeName}`;
    if (multiplicity) labelText += ` [${multiplicity}]`;
    this.label = labelText;
    this.description = element.type;

    const tooltipParts: string[] = [`${element.type}: ${element.name || "(anonymous)"}`];
    if (typeName) tooltipParts.push(`Type: ${typeName}`);
    if (multiplicity) tooltipParts.push(`Multiplicity: [${multiplicity}]`);
    if (element.children?.length) tooltipParts.push(`Children: ${element.children.length}`);
    if (element.relationships?.length) tooltipParts.push(`Relationships: ${element.relationships.length}`);
    this.tooltip = tooltipParts.join("\n");

    this.command = {
      command: "sysml.openLocation",
      title: "Open Location",
      arguments: [this],
    };
  }
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

export class ModelExplorerProvider
  implements vscode.TreeDataProvider<ExplorerTreeItem>
{
  private readonly _onDidChangeTreeData = new vscode.EventEmitter<
    ExplorerTreeItem | undefined | void
  >();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  private lastUri: vscode.Uri | undefined;
  private lastElements: SysMLElementDTO[] | undefined;

  private workspaceMode = false;
  private workspaceFileData = new Map<
    string,
    { uri: vscode.Uri; elements: SysMLElementDTO[] }
  >();
  private workspaceFileUris: vscode.Uri[] = [];
  private _workspaceViewMode: "byFile" | "bySemantic" = "bySemantic";
  private treeView?: vscode.TreeView<ExplorerTreeItem>;
  private uriToRootItems = new Map<string, ExplorerTreeItem[]>();
  private workspaceLoadStatus: WorkspaceLoadStatus = {
    state: "idle",
    scannedFiles: 0,
    loadedFiles: 0,
    truncated: false,
    cancelled: false,
    failures: 0,
  };

  constructor(private readonly modelProvider: LspModelProvider) {}

  setTreeView(treeView: vscode.TreeView<ExplorerTreeItem>): void {
    this.treeView = treeView;
  }

  isWorkspaceMode(): boolean {
    return this.workspaceMode;
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
    this._onDidChangeTreeData.fire();
  }

  async revealActiveDocument(docUri: vscode.Uri): Promise<void> {
    if (!this.treeView) return;
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

  clear(): void {
    this.lastUri = undefined;
    this.lastElements = undefined;
    this.workspaceMode = false;
    this.workspaceFileData.clear();
    this.workspaceFileUris = [];
    this.uriToRootItems.clear();
    this.workspaceLoadStatus = {
      state: "idle",
      scannedFiles: 0,
      loadedFiles: 0,
      truncated: false,
      cancelled: false,
      failures: 0,
    };
    this._onDidChangeTreeData.fire();
  }

  setWorkspaceLoadStatus(status: Partial<WorkspaceLoadStatus>): void {
    this.workspaceLoadStatus = {
      ...this.workspaceLoadStatus,
      ...status,
    };
    this._onDidChangeTreeData.fire();
  }

  async loadWorkspaceModel(
    fileUris: vscode.Uri[],
    token?: vscode.CancellationToken
  ): Promise<void> {
    log("loadWorkspaceModel:", fileUris.length, "files. URIs:", fileUris.map((u) => u.toString()));
    this.workspaceMode = true;
    this.workspaceFileUris = fileUris;
    this.lastUri = undefined;
    this.lastElements = undefined;
    this.workspaceFileData.clear();
    this.uriToRootItems.clear();
    let failures = 0;

    try {
      for (const uri of fileUris) {
        if (token?.isCancellationRequested) break;
        const uriStr = uri.toString();
        try {
          log("loadWorkspaceModel: requesting getModel for", uriStr);
          const result = await this.modelProvider.getModel(
            uriStr,
            ["graph", "stats"],
            token
          );
          const elements = result.graph ? (graphToElementTree(result.graph) as SysMLElementDTO[]) : [];
          if (elements.length) {
            log("loadWorkspaceModel: loaded", uriStr, "->", elements.length, "elements");
            this.workspaceFileData.set(uriStr, {
              uri,
              elements,
            });
          } else {
            log("loadWorkspaceModel: 0 elements for", uriStr, "(graph nodes:", result.graph?.nodes?.length ?? 0, ")");
          }
        } catch (e) {
          failures += 1;
          if (e instanceof vscode.CancellationError || token?.isCancellationRequested) {
            log("loadWorkspaceModel: cancelled while loading", uriStr);
            break;
          }
          logError(`loadWorkspaceModel: skip file (failed): ${uriStr}`, e);
        }
      }
    } finally {
      this.workspaceLoadStatus = {
        ...this.workspaceLoadStatus,
        state: token?.isCancellationRequested
          ? "degraded"
          : (this.workspaceLoadStatus.truncated || failures > 0 ? "degraded" : "ready"),
        loadedFiles: this.workspaceFileData.size,
        failures,
      };
      log("loadWorkspaceModel: done,", this.workspaceFileData.size, "files loaded");
      this._onDidChangeTreeData.fire();
    }
  }

  async loadDocument(
    document: vscode.TextDocument,
    token?: vscode.CancellationToken
  ): Promise<void> {
    log("loadDocument:", document.uri.toString().slice(-50));
    this.workspaceMode = false;
    this.workspaceFileData.clear();
    this.workspaceFileUris = [];
    this.lastUri = document.uri;

    try {
      const result = await this.modelProvider.getModel(
        document.uri.toString(),
        ["graph", "stats"],
        token
      );
      this.lastElements = result.graph
        ? (graphToElementTree(result.graph) as SysMLElementDTO[])
        : [];
      log("loadDocument: done,", this.lastElements.length, "elements");
    } catch (error) {
      if (error instanceof vscode.CancellationError || token?.isCancellationRequested) {
        log("loadDocument: cancelled for", document.uri.toString());
      } else {
        logError(`loadDocument failed for ${document.uri.toString()}`, error);
      }
      this.lastElements = [];
    } finally {
      this._onDidChangeTreeData.fire();
    }
  }

  refresh(): void {
    log("refresh: workspaceMode=", this.workspaceMode, "fileCount=", this.workspaceFileUris.length);
    if (this.workspaceMode && this.workspaceFileUris.length > 0) {
      this.loadWorkspaceModel(this.workspaceFileUris);
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
    if (this.workspaceMode) {
      return Array.from(this.workspaceFileData.values()).flatMap(
        (d) => d.elements
      );
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
      const infoItems = this.getWorkspaceInfoItems();
      if (this.workspaceMode && this.workspaceFileData.size > 0) {
        if (this._workspaceViewMode === "byFile") {
          const items = Array.from(this.workspaceFileData.entries()).map(
            ([, data]) => new FileTreeItem(data.uri, data.elements.length)
          );
          this.uriToRootItems.clear();
          for (const item of items) {
            this.uriToRootItems.set(item.fileUri.toString(), [item]);
          }
          return [...infoItems, ...items];
        }
        const entries = Array.from(this.workspaceFileData.entries());
        const items = this.mergeNamespaceElements(entries);
        this.buildSemanticUriMapping(items);
        return [...infoItems, ...items];
      }

      if (this.workspaceMode && infoItems.length > 0) {
        return infoItems;
      }

      if (!this.lastUri && !this.workspaceMode) {
        const active = vscode.window.activeTextEditor?.document;
        if (
          active &&
          (active.languageId === "sysml" || active.languageId === "kerml")
        ) {
          try {
            const result = await this.modelProvider.getModel(
              active.uri.toString(),
              ["graph", "stats"]
            );
            this.lastUri = active.uri;
            this.lastElements = result.graph
              ? (graphToElementTree(result.graph) as SysMLElementDTO[])
              : [];
          } catch (error) {
            logError(`getChildren: failed to load active document model for ${active.uri.toString()}`, error);
            this.lastUri = active.uri;
            this.lastElements = [];
            return [
              new ExplorerInfoItem(
                "Model data unavailable",
                "Open the SysML output for details",
                `The language server did not return model data for ${active.uri.fsPath}.`,
                "warning"
              ),
            ];
          }
        } else {
          return [];
        }
      }

      if (this.lastUri && this.lastElements) {
        const merged = this.mergeElements(this.lastElements);
        const items = merged.map(
          (e) => new ModelTreeItem(e, this.lastUri!)
        );
        this.uriToRootItems.clear();
        this.uriToRootItems.set(this.lastUri.toString(), items);
        return items;
      }
      return [];
    }

    if (element.itemType === "file-node") {
      const data = this.workspaceFileData.get(element.fileUri.toString());
      if (!data) return [];
      return data.elements.map(
        (e) => new ModelTreeItem(e, data.uri)
      );
    }

    if (element.itemType === "explorer-info") {
      return [];
    }

    const children: ExplorerTreeItem[] = [];
    const el = element.element;
    const childElements = el.children ?? [];
    for (const c of childElements) {
      children.push(new ModelTreeItem(c, element.elementUri));
    }
    return children;
  }

  private getWorkspaceInfoItems(): ExplorerInfoItem[] {
    if (!this.workspaceMode) {
      return [];
    }
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

  private mergeNamespaceElements(
    entries: [string, { uri: vscode.Uri; elements: SysMLElementDTO[] }][]
  ): ModelTreeItem[] {
    const pairs: { el: SysMLElementDTO; uri: vscode.Uri }[] = [];
    for (const [, data] of entries) {
      for (const el of data.elements) {
        pairs.push({ el, uri: data.uri });
      }
    }

    const mergedMap = new Map<string, { merged: SysMLElementDTO; uri: vscode.Uri }>();
    const result: ModelTreeItem[] = [];

    for (const { el, uri } of pairs) {
      const key = `${el.type}::${el.name || "(anonymous)"}`;
      if (this.namespaceTypes.has(el.type) && mergedMap.has(key)) {
        const existing = mergedMap.get(key)!;
        existing.merged = this.mergeTwo(existing.merged, el);
      } else if (this.namespaceTypes.has(el.type)) {
        const clone = this.cloneElement(el);
        mergedMap.set(key, { merged: clone, uri });
        result.push(new ModelTreeItem(clone, uri));
      } else {
        result.push(new ModelTreeItem(el, uri));
      }
    }
    return result;
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
      type: el.type,
      name: el.name,
      range: el.range,
      children: (el.children ?? []).map((c) => this.cloneElement(c)),
      attributes: el.attributes ? { ...el.attributes } : {},
      relationships: [...(el.relationships ?? [])],
      errors: el.errors ? [...el.errors] : undefined,
    };
  }
}

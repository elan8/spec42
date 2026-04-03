import * as vscode from "vscode";
import { log, logError } from "../logger";
import type { LspModelProvider } from "./lspModelProvider";
import type {
  RangeDTO,
  SysMLFeatureInspectorElementDTO,
  SysMLFeatureInspectorElementRefDTO,
  SysMLFeatureInspectorRelationshipDTO,
  SysMLFeatureInspectorResult,
} from "./sysmlModelTypes";

type ServerHealthState =
  | "starting"
  | "ready"
  | "indexing"
  | "degraded"
  | "restarting"
  | "crashed";

type InspectorStateKind = "inactive" | "loading" | "ready" | "empty" | "error";

type FeatureInspectorDebugState = {
  state: InspectorStateKind;
  sourceUri?: string;
  selectedElementId?: string;
  selectedElementName?: string;
  emptyMessage?: string;
  errorMessage?: string;
  lastRequestedUri?: string;
  lastRequestedPosition?: { line: number; character: number };
  requestSequence: number;
};

type FeatureInspectorTreeItem =
  | FeatureInspectorInfoItem
  | FeatureInspectorSectionItem
  | FeatureInspectorValueItem
  | FeatureInspectorReferenceItem;

type InspectorSection =
  | "header"
  | "location"
  | "typing"
  | "specialization"
  | "outgoingRelationships"
  | "incomingRelationships"
  | "attributes";

type InspectorNavigationTarget = {
  uri: string;
  range: RangeDTO;
  label?: string;
};

type InspectorAttributeEntry = {
  key: string;
  value: string;
};

type InspectorSectionData = {
  section: InspectorSection;
  label: string;
  description?: string;
  tooltip?: string;
  iconId?: string;
  children: FeatureInspectorTreeItem[];
};

function toVscodeRange(range: RangeDTO): vscode.Range {
  return new vscode.Range(
    new vscode.Position(range.start.line, range.start.character),
    new vscode.Position(range.end.line, range.end.character)
  );
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

function samePosition(
  a: vscode.Position | undefined,
  b: vscode.Position | undefined
): boolean {
  return !!a && !!b && a.line === b.line && a.character === b.character;
}

function elementIdentity(
  result: SysMLFeatureInspectorResult | undefined
): string | undefined {
  return result?.element?.id;
}

function isPrimitiveValue(value: unknown): value is string | number | boolean {
  return (
    typeof value === "string" ||
    typeof value === "number" ||
    typeof value === "boolean"
  );
}

function formatAttributeValue(value: unknown): string | undefined {
  if (value === null || value === undefined) {
    return undefined;
  }
  if (isPrimitiveValue(value)) {
    return String(value);
  }
  if (Array.isArray(value) && value.length > 0 && value.every(isPrimitiveValue)) {
    return value.map((entry) => String(entry)).join(", ");
  }
  return undefined;
}

function collectAttributeEntries(
  attributes: Record<string, unknown> | undefined
): InspectorAttributeEntry[] {
  if (!attributes) {
    return [];
  }
  return Object.entries(attributes)
    .map(([key, value]) => ({ key, value: formatAttributeValue(value) }))
    .filter((entry): entry is { key: string; value: string } => !!entry.value)
    .sort((a, b) => a.key.localeCompare(b.key));
}

export class FeatureInspectorInfoItem extends vscode.TreeItem {
  readonly itemType = "feature-inspector-info" as const;

  constructor(
    label: string,
    description?: string,
    tooltip?: string,
    iconId: "info" | "warning" | "error" | "sync" = "info"
  ) {
    super(label, vscode.TreeItemCollapsibleState.None);
    this.description = description;
    this.tooltip = tooltip ?? label;
    this.iconPath = new vscode.ThemeIcon(iconId);
    this.contextValue = "sysmlFeatureInspectorInfo";
  }
}

export class FeatureInspectorSectionItem extends vscode.TreeItem {
  readonly itemType = "feature-inspector-section" as const;

  constructor(public readonly data: InspectorSectionData) {
    super(data.label, vscode.TreeItemCollapsibleState.Expanded);
    this.description = data.description;
    this.tooltip = data.tooltip ?? data.label;
    this.iconPath = new vscode.ThemeIcon(data.iconId ?? "symbol-misc");
    this.contextValue = "sysmlFeatureInspectorSection";
  }
}

export class FeatureInspectorValueItem extends vscode.TreeItem {
  readonly itemType = "feature-inspector-value" as const;

  constructor(
    label: string,
    description?: string,
    tooltip?: string,
    iconId = "symbol-value"
  ) {
    super(label, vscode.TreeItemCollapsibleState.None);
    this.description = description;
    this.tooltip = tooltip ?? label;
    this.iconPath = new vscode.ThemeIcon(iconId);
    this.contextValue = "sysmlFeatureInspectorValue";
  }
}

export class FeatureInspectorReferenceItem extends vscode.TreeItem {
  readonly itemType = "feature-inspector-reference" as const;

  constructor(
    label: string,
    public readonly target: InspectorNavigationTarget,
    description?: string,
    tooltip?: string,
    iconId = "go-to-file"
  ) {
    super(label, vscode.TreeItemCollapsibleState.None);
    this.description = description;
    this.tooltip = tooltip ?? label;
    this.iconPath = new vscode.ThemeIcon(iconId);
    this.contextValue = "sysmlFeatureInspectorReference";
    this.command = {
      command: "sysml.featureInspector.openReference",
      title: "Open Feature Inspector Reference",
      arguments: [target],
    };
  }
}

export class FeatureInspectorProvider
  implements vscode.TreeDataProvider<FeatureInspectorTreeItem>
{
  private readonly _onDidChangeTreeData = new vscode.EventEmitter<
    FeatureInspectorTreeItem | undefined | void
  >();

  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  private treeView?: vscode.TreeView<FeatureInspectorTreeItem>;
  private refreshTimer: ReturnType<typeof setTimeout> | undefined;
  private requestCts: vscode.CancellationTokenSource | undefined;
  private currentResult: SysMLFeatureInspectorResult | undefined;
  private state: InspectorStateKind = "inactive";
  private emptyMessage = "Open a SysML/KerML file to inspect model elements.";
  private errorMessage = "";
  private lastRequestedUri: string | undefined;
  private lastRequestedPosition: { line: number; character: number } | undefined;
  private lastObservedPosition: vscode.Position | undefined;
  private requestSequence = 0;

  constructor(
    private readonly modelProvider: LspModelProvider,
    private readonly isSysmlDoc: (doc: vscode.TextDocument | undefined) => boolean,
    private readonly getServerHealth: () => {
      state: ServerHealthState;
      detail: string;
    }
  ) {}

  setTreeView(treeView: vscode.TreeView<FeatureInspectorTreeItem>): void {
    this.treeView = treeView;
  }

  getDebugState(): FeatureInspectorDebugState {
    return {
      state: this.state,
      sourceUri: this.currentResult?.sourceUri,
      selectedElementId: this.currentResult?.element?.id,
      selectedElementName: this.currentResult?.element?.name,
      emptyMessage: this.emptyMessage,
      errorMessage: this.errorMessage || undefined,
      lastRequestedUri: this.lastRequestedUri,
      lastRequestedPosition: this.lastRequestedPosition,
      requestSequence: this.requestSequence,
    };
  }

  dispose(): void {
    this.cancelPendingWork();
  }

  cancelPendingWork(): void {
    if (this.refreshTimer) {
      clearTimeout(this.refreshTimer);
      this.refreshTimer = undefined;
    }
    this.requestCts?.cancel();
    this.requestCts?.dispose();
    this.requestCts = undefined;
  }

  clearInactive(message?: string): void {
    this.cancelPendingWork();
    this.currentResult = undefined;
    this.state = "inactive";
    this.errorMessage = "";
    this.emptyMessage =
      message ?? "Open a SysML/KerML file to inspect model elements.";
    this._onDidChangeTreeData.fire();
  }

  refresh(immediate = false): void {
    const editor = vscode.window.activeTextEditor;
    if (!editor || !this.isSysmlDoc(editor.document)) {
      const health = this.getServerHealth();
      if (health.state === "starting" || health.state === "restarting") {
        this.clearInactive("Feature Inspector unavailable while the language server is starting.");
      } else {
        this.clearInactive();
      }
      return;
    }

    if (this.refreshTimer) {
      clearTimeout(this.refreshTimer);
      this.refreshTimer = undefined;
    }

    const delay = immediate ? 0 : 150;
    this.refreshTimer = setTimeout(() => {
      this.refreshTimer = undefined;
      void this.fetchForEditor(editor);
    }, delay);
  }

  handleActiveEditorChanged(editor: vscode.TextEditor | undefined): void {
    if (!editor || !this.isSysmlDoc(editor.document)) {
      const health = this.getServerHealth();
      if (health.state === "starting" || health.state === "restarting") {
        this.clearInactive("Feature Inspector unavailable while the language server is starting.");
      } else {
        this.clearInactive();
      }
      return;
    }
    this.lastObservedPosition = editor.selection.active;
    this.refresh(true);
  }

  handleSelectionChanged(event: vscode.TextEditorSelectionChangeEvent): void {
    if (!this.isSysmlDoc(event.textEditor.document)) {
      return;
    }
    const position = event.selections[0]?.active ?? event.textEditor.selection.active;
    if (samePosition(position, this.lastObservedPosition)) {
      return;
    }
    this.lastObservedPosition = position;
    this.refresh(false);
  }

  async openReference(target: InspectorNavigationTarget): Promise<void> {
    const uri = vscode.Uri.parse(target.uri);
    const range = toVscodeRange(target.range);
    const doc = await vscode.workspace.openTextDocument(uri);
    const editor = await vscode.window.showTextDocument(doc, {
      preserveFocus: false,
      preview: true,
    });
    editor.selection = new vscode.Selection(range.start, range.start);
    editor.revealRange(range, vscode.TextEditorRevealType.InCenter);
  }

  getTreeItem(element: FeatureInspectorTreeItem): vscode.TreeItem {
    return element;
  }

  async getChildren(
    element?: FeatureInspectorTreeItem
  ): Promise<FeatureInspectorTreeItem[]> {
    if (!element) {
      if (this.state === "loading" && this.currentResult?.element) {
        const sections = this.buildSections(this.currentResult.element);
        return [
          new FeatureInspectorInfoItem(
            "Updating inspector...",
            undefined,
            "Refreshing semantic details for the current selection.",
            "sync"
          ),
          ...sections,
        ];
      }
      if (this.state === "inactive" || this.state === "empty") {
        return [new FeatureInspectorInfoItem(this.emptyMessage)];
      }
      if (this.state === "error") {
        return [
          new FeatureInspectorInfoItem(
            "Feature Inspector failed",
            "Open the SysML output for details",
            this.errorMessage || "Feature Inspector request failed.",
            "error"
          ),
        ];
      }
      if (!this.currentResult?.element) {
        return [
          new FeatureInspectorInfoItem(
            "No inspectable element at current cursor position."
          ),
        ];
      }
      return this.buildSections(this.currentResult.element);
    }

    if (element.itemType === "feature-inspector-section") {
      return element.data.children;
    }
    return [];
  }

  private async fetchForEditor(editor: vscode.TextEditor): Promise<void> {
    const doc = editor.document;
    if (!this.isSysmlDoc(doc)) {
      this.clearInactive();
      return;
    }

    const health = this.getServerHealth();
    if (health.state === "starting" || health.state === "restarting") {
      this.clearInactive("Feature Inspector unavailable while the language server is starting.");
      return;
    }

    const position = editor.selection.active;
    const uri = doc.uri.toString();
    this.lastRequestedUri = uri;
    this.lastRequestedPosition = {
      line: position.line,
      character: position.character,
    };
    this.requestSequence += 1;
    const requestId = this.requestSequence;

    this.requestCts?.cancel();
    this.requestCts?.dispose();
    this.requestCts = new vscode.CancellationTokenSource();
    const token = this.requestCts.token;

    if (!this.currentResult?.element) {
      this.state = "loading";
      this._onDidChangeTreeData.fire();
    }

    try {
      const result = await this.modelProvider.getFeatureInspector(
        uri,
        { line: position.line, character: position.character },
        token
      );
      if (token.isCancellationRequested || requestId !== this.requestSequence) {
        return;
      }

      const previousElementId = elementIdentity(this.currentResult);
      const nextElementId = elementIdentity(result);
      this.currentResult = result;
      this.errorMessage = "";
      if (!result.element) {
        this.state = "empty";
        this.emptyMessage = "No inspectable element at current cursor position.";
      } else {
        this.state = "ready";
      }

      if (
        previousElementId !== nextElementId ||
        this.state !== "ready" ||
        result.element?.uri !== this.currentResult?.sourceUri
      ) {
        this._onDidChangeTreeData.fire();
      } else {
        this._onDidChangeTreeData.fire();
      }
    } catch (error) {
      if (token.isCancellationRequested || error instanceof vscode.CancellationError) {
        return;
      }
      logError(`Feature Inspector request failed for ${uri}`, error);
      this.state = "error";
      this.errorMessage =
        error instanceof Error ? error.message : String(error);
      this._onDidChangeTreeData.fire();
    }
  }

  private buildSections(
    element: SysMLFeatureInspectorElementDTO
  ): FeatureInspectorTreeItem[] {
    const sections: InspectorSectionData[] = [
      {
        section: "header",
        label: "Header",
        iconId: "symbol-object",
        children: this.buildHeaderItems(element),
      },
      {
        section: "location",
        label: "Location",
        iconId: "location",
        children: this.buildLocationItems(element),
      },
      {
        section: "typing",
        label: "Typing",
        description: element.typing.status,
        iconId: "type-hierarchy",
        children: this.buildResolutionItems("Type", element.typing.targets),
      },
      {
        section: "specialization",
        label: "Specialization",
        description: element.specialization.status,
        iconId: "references",
        children: this.buildResolutionItems(
          "Base",
          element.specialization.targets
        ),
      },
      {
        section: "outgoingRelationships",
        label: "Outgoing Relationships",
        description: String(element.outgoingRelationships.length),
        iconId: "arrow-right",
        children: this.buildRelationshipItems(element.outgoingRelationships),
      },
      {
        section: "incomingRelationships",
        label: "Incoming Relationships",
        description: String(element.incomingRelationships.length),
        iconId: "arrow-left",
        children: this.buildRelationshipItems(element.incomingRelationships),
      },
      {
        section: "attributes",
        label: "Attributes",
        description: String(collectAttributeEntries(element.attributes).length),
        iconId: "symbol-field",
        children: this.buildAttributeItems(element.attributes),
      },
    ];

    return sections
      .map((section) => {
        if (section.children.length === 0) {
          section.children = [
            new FeatureInspectorValueItem(
              "None",
              undefined,
              `${section.label} is empty.`,
              "circle-slash"
            ),
          ];
        }
        return new FeatureInspectorSectionItem(section);
      });
  }

  private buildHeaderItems(
    element: SysMLFeatureInspectorElementDTO
  ): FeatureInspectorTreeItem[] {
    const items: FeatureInspectorTreeItem[] = [
      new FeatureInspectorValueItem(
        element.name || "(anonymous)",
        element.type,
        `${element.type}: ${element.qualifiedName}`,
        "symbol-misc"
      ),
      new FeatureInspectorValueItem(
        element.qualifiedName,
        undefined,
        element.qualifiedName,
        "symbol-string"
      ),
    ];
    const activeUri = vscode.window.activeTextEditor?.document.uri.toString();
    if (element.uri !== activeUri) {
      const fileName = vscode.Uri.parse(element.uri).fsPath.split(/[/\\]/).pop() ?? element.uri;
      items.push(
        new FeatureInspectorValueItem(
          "Source file",
          fileName,
          element.uri,
          "file"
        )
      );
    }
    return items;
  }

  private buildLocationItems(
    element: SysMLFeatureInspectorElementDTO
  ): FeatureInspectorTreeItem[] {
    const items: FeatureInspectorTreeItem[] = [
      new FeatureInspectorReferenceItem(
        "Current element",
        { uri: element.uri, range: element.range, label: element.name },
        rangeLabel(element.range),
        `${element.qualifiedName}\n${rangeLabel(element.range)}`,
        "location"
      ),
    ];
    if (element.parent) {
      items.push(
        new FeatureInspectorReferenceItem(
          "Parent",
          {
            uri: element.parent.uri,
            range: element.parent.range,
            label: element.parent.name,
          },
          element.parent.name,
          element.parent.qualifiedName,
          "symbol-namespace"
        )
      );
    }
    return items;
  }

  private buildResolutionItems(
    labelPrefix: string,
    refs: SysMLFeatureInspectorElementRefDTO[]
  ): FeatureInspectorTreeItem[] {
    return refs.map(
      (ref) =>
        new FeatureInspectorReferenceItem(
          `${labelPrefix}: ${ref.name}`,
          { uri: ref.uri, range: ref.range, label: ref.name },
          ref.type,
          ref.qualifiedName,
          "go-to-file"
        )
    );
  }

  private buildRelationshipItems(
    relationships: SysMLFeatureInspectorRelationshipDTO[]
  ): FeatureInspectorTreeItem[] {
    return relationships
      .slice()
      .sort((a, b) => {
        const kindCmp = a.type.localeCompare(b.type);
        return kindCmp !== 0
          ? kindCmp
          : a.peer.name.localeCompare(b.peer.name);
      })
      .map(
        (relationship) =>
          new FeatureInspectorReferenceItem(
            `${relationship.type}: ${relationship.peer.name}`,
            {
              uri: relationship.peer.uri,
              range: relationship.peer.range,
              label: relationship.peer.name,
            },
            relationship.peer.type,
            relationship.peer.qualifiedName,
            "references"
          )
      );
  }

  private buildAttributeItems(
    attributes: Record<string, unknown>
  ): FeatureInspectorTreeItem[] {
    return collectAttributeEntries(attributes).map(
      (entry) =>
        new FeatureInspectorValueItem(
          entry.key,
          entry.value,
          `${entry.key}: ${entry.value}`,
          "symbol-key"
        )
    );
  }
}

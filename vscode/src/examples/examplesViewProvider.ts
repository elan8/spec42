import * as path from "path";
import * as vscode from "vscode";
import { log } from "../logger";

export class ExampleTreeItem extends vscode.TreeItem {
  constructor(
    public readonly folderUri: vscode.Uri,
    relativeExamplesPath: string
  ) {
    super(relativeExamplesPath, vscode.TreeItemCollapsibleState.None);
    this.description = "Open as workspace";
    this.tooltip = folderUri.fsPath;
    this.iconPath = new vscode.ThemeIcon("folder");
    this.contextValue = "spec42ExampleFolder";
    this.command = {
      command: "spec42.examples.openWorkspace",
      title: "Open Example Workspace",
      arguments: [folderUri],
    };
  }
}

type ExamplesInfoItemType =
  | "no-workspace"
  | "no-examples-dir"
  | "empty-examples-dir";

export class ExamplesInfoItem extends vscode.TreeItem {
  readonly itemType = "examples-info" as const;

  constructor(type: ExamplesInfoItemType, description?: string) {
    const label =
      type === "no-workspace"
        ? "Open a workspace folder to list examples"
        : type === "no-examples-dir"
          ? "No examples directory found"
          : "No example folders found";
    super(label, vscode.TreeItemCollapsibleState.None);
    this.description = description;
    this.iconPath = new vscode.ThemeIcon("info");
    this.contextValue = "spec42ExamplesInfo";
  }
}

export type ExamplesTreeItem = ExampleTreeItem | ExamplesInfoItem;

export class ExamplesViewProvider
  implements vscode.TreeDataProvider<ExamplesTreeItem>
{
  constructor(private readonly examplesRoots: readonly vscode.Uri[]) {}

  private readonly _onDidChangeTreeData = new vscode.EventEmitter<
    ExamplesTreeItem | undefined | void
  >();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  refresh(): void {
    log("examples.refresh");
    this._onDidChangeTreeData.fire();
  }

  getTreeItem(element: ExamplesTreeItem): vscode.TreeItem {
    return element;
  }

  private candidateExamplesUris(): vscode.Uri[] {
    const candidates: vscode.Uri[] = [];
    const seen = new Set<string>();
    const addCandidate = (uri: vscode.Uri): void => {
      const key = uri.fsPath.toLowerCase();
      if (seen.has(key)) {
        return;
      }
      seen.add(key);
      candidates.push(uri);
    };

    for (const root of this.examplesRoots) {
      addCandidate(root);
    }
    return candidates;
  }

  async getChildren(element?: ExamplesTreeItem): Promise<ExamplesTreeItem[]> {
    if (element) {
      return [];
    }

    log(
      "examples.getChildren:start",
      this.examplesRoots.map((folder) => folder.fsPath)
    );
    const candidateRoots = this.candidateExamplesUris();
    log(
      "examples.getChildren:candidateRoots",
      candidateRoots.map((uri) => uri.fsPath)
    );
    const items: ExampleTreeItem[] = [];
    let foundExamplesRoot = false;
    const addedFolders = new Set<string>();

    for (const examplesUri of candidateRoots) {
      let entries: [string, vscode.FileType][];
      try {
        entries = await vscode.workspace.fs.readDirectory(examplesUri);
        foundExamplesRoot = true;
        log(
          "examples.getChildren:readDirectory:ok",
          examplesUri.fsPath,
          entries.length
        );
      } catch {
        log("examples.getChildren:readDirectory:missing", examplesUri.fsPath);
        continue;
      }

      const folders = entries
        .filter((entry) => (entry[1] & vscode.FileType.Directory) !== 0)
        .map((entry) => entry[0])
        .sort((a, b) => a.localeCompare(b));

      for (const folderName of folders) {
        const folderUri = vscode.Uri.joinPath(examplesUri, folderName);
        const folderKey = folderUri.fsPath.toLowerCase();
        if (addedFolders.has(folderKey)) {
          continue;
        }
        addedFolders.add(folderKey);

        const relativePath = path.basename(folderUri.fsPath);

        items.push(new ExampleTreeItem(folderUri, relativePath));
      }
    }

    if (!foundExamplesRoot) {
      log("examples.getChildren:no-examples-dir");
      return [new ExamplesInfoItem("no-examples-dir")];
    }
    if (items.length === 0) {
      log("examples.getChildren:empty-examples-dir");
      return [new ExamplesInfoItem("empty-examples-dir", "examples/")];
    }
    log("examples.getChildren:done", items.length);
    return items.sort((a, b) => String(a.label).localeCompare(String(b.label)));
  }
}

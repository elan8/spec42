import * as vscode from "vscode";

export class LibraryTreeItem extends vscode.TreeItem {
  constructor(
    label: string,
    description: string,
    commandId?: string
  ) {
    super(label, vscode.TreeItemCollapsibleState.None);
    this.description = description;
    this.contextValue = "spec42LibraryAction";
    this.iconPath = new vscode.ThemeIcon("book");
    if (commandId) {
      this.command = {
        command: commandId,
        title: label,
      };
    }
  }
}

export class LibraryViewProvider
  implements vscode.TreeDataProvider<LibraryTreeItem>
{
  private readonly _onDidChangeTreeData = new vscode.EventEmitter<
    LibraryTreeItem | undefined | null | void
  >();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  refresh(): void {
    this._onDidChangeTreeData.fire();
  }

  getTreeItem(element: LibraryTreeItem): vscode.TreeItem {
    return element;
  }

  getChildren(): LibraryTreeItem[] {
    return [
      new LibraryTreeItem(
        "Standard Library",
        "Install or update standard library",
        "sysml.library.installStdLib"
      ),
      new LibraryTreeItem(
        "Configured Library Paths",
        "Manage spec42.libraryPaths setting",
        "sysml.library.managePaths"
      ),
      new LibraryTreeItem(
        "Search Libraries",
        "Search available libraries (coming soon)",
        "sysml.library.search"
      ),
    ];
  }
}

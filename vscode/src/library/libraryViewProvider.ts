import * as vscode from "vscode";

type StandardLibraryViewStatus = {
  enabled: boolean;
  pinnedVersion: string;
  installedVersion?: string;
  isInstalled: boolean;
};

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
  constructor(
    private readonly getStandardLibraryStatus?: () => StandardLibraryViewStatus
  ) {}

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
    const status = this.getStandardLibraryStatus?.();
    const stdLibDescription = !status
      ? "Install or update standard library"
      : !status.enabled
      ? "Disabled in settings"
      : status.isInstalled
      ? `Installed (${status.installedVersion ?? status.pinnedVersion})`
      : status.installedVersion
      ? `Pinned ${status.pinnedVersion}; installed ${status.installedVersion}`
      : `Not installed (pinned ${status.pinnedVersion})`;

    return [
      new LibraryTreeItem(
        "Standard Library",
        stdLibDescription,
        "sysml.library.installStdLib"
      ),
      new LibraryTreeItem(
        "Custom Libraries",
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

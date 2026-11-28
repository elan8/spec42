import * as vscode from "vscode";

export class ToolGroup extends vscode.TreeItem {
  constructor(
    label: string, icon: string, expanded: boolean,
    readonly actions: (signal: AbortSignal) => Promise<vscode.TreeItem[]>,
  ) {
    super(label, expanded ? vscode.TreeItemCollapsibleState.Expanded : vscode.TreeItemCollapsibleState.Collapsed);
    this.iconPath = new vscode.ThemeIcon(icon);
  }
}

export function toolAction(label: string, command: string, icon: string, ...args: unknown[]): vscode.TreeItem {
  const item = new vscode.TreeItem(label);
  item.command = { command, title: label, arguments: args };
  item.iconPath = new vscode.ThemeIcon(icon);
  return item;
}

export class Elan8ToolsView implements vscode.TreeDataProvider<vscode.TreeItem>, vscode.Disposable {
  private readonly changed = new vscode.EventEmitter<void>();
  private scan = new AbortController();
  readonly onDidChangeTreeData = this.changed.event;

  constructor(private readonly tools: vscode.TreeItem[]) {}

  getTreeItem(item: vscode.TreeItem): vscode.TreeItem {
    return item;
  }

  async getChildren(parent?: vscode.TreeItem): Promise<vscode.TreeItem[]> {
    if (!parent) return this.tools;
    if (!(parent instanceof ToolGroup)) return [];
    const signal = this.scan.signal;
    try {
      const items = await parent.actions(signal);
      return signal.aborted ? [] : items;
    } catch (error) {
      if (signal.aborted) return [];
      throw error;
    }
  }

  refresh(): void {
    this.scan.abort();
    this.scan = new AbortController();
    this.changed.fire();
  }

  dispose(): void {
    this.scan.abort();
    this.changed.dispose();
  }
}

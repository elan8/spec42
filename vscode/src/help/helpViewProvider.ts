import * as vscode from "vscode";

type HelpItemKind = "action" | "link";

class HelpItem extends vscode.TreeItem {
  constructor(
    label: string,
    icon: string,
    command: vscode.Command,
    kind: HelpItemKind = "action"
  ) {
    super(label, vscode.TreeItemCollapsibleState.None);
    this.iconPath = new vscode.ThemeIcon(icon);
    this.command = command;
    this.contextValue = kind === "link" ? "helpLink" : "helpAction";
  }
}

export class HelpViewProvider implements vscode.TreeDataProvider<HelpItem> {
  getTreeItem(element: HelpItem): vscode.TreeItem {
    return element;
  }

  getChildren(): HelpItem[] {
    return [
      new HelpItem(
        "Open diagram visualizer",
        "layout-sidebar-right",
        { command: "sysml.showVisualizer", title: "Open Visualizer" }
      ),
      new HelpItem(
        "Open recommended example",
        "star-full",
        { command: "spec42.examples.openRecommended", title: "Open Recommended Example" }
      ),
      new HelpItem(
        "SysML v2 quick reference",
        "book",
        { command: "spec42.help.openReference", title: "SysML v2 Quick Reference" }
      ),
      new HelpItem(
        "Spec42 documentation",
        "link-external",
        { command: "vscode.open", title: "Open Docs", arguments: [vscode.Uri.parse("https://elan8.github.io/spec42/")] },
        "link"
      ),
      new HelpItem(
        "SysML v2 language specification",
        "link-external",
        { command: "vscode.open", title: "Open SysML v2 Spec", arguments: [vscode.Uri.parse("https://www.omg.org/spec/SysML/2.0/Language/")] },
        "link"
      ),
    ];
  }
}

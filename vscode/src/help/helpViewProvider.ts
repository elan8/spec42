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
        "Open recommended example",
        "star-full",
        { command: "spec42.examples.openRecommended", title: "Open Recommended Example" }
      ),
      new HelpItem(
        "Open Diagram view",
        "type-hierarchy-sub",
        { command: "spec42.diagram.open", title: "Open Diagram" }
      ),
      new HelpItem(
        "Browse or add a library",
        "folder-library",
        { command: "spec42Library.focus", title: "Show Library View" }
      ),
      new HelpItem(
        "SysML v2 quick reference",
        "book",
        { command: "spec42.help.openReference", title: "SysML v2 Quick Reference" }
      ),
      new HelpItem(
        "Spec42 documentation",
        "link-external",
        { command: "vscode.open", title: "Open Docs", arguments: [vscode.Uri.parse("https://github.com/elan8/spec42/blob/HEAD/docs/README.md")] },
        "link"
      ),
      new HelpItem(
        "What's included",
        "link-external",
        { command: "vscode.open", title: "Open What's Included", arguments: [vscode.Uri.parse("https://github.com/elan8/spec42/blob/HEAD/docs/reference/WHATS-INCLUDED.md")] },
        "link"
      ),
      new HelpItem(
        "Domain & method libraries",
        "link-external",
        { command: "vscode.open", title: "Open Domain & Method Libraries", arguments: [vscode.Uri.parse("https://github.com/elan8/spec42/blob/HEAD/docs/user/LIBRARIES.md")] },
        "link"
      ),
      new HelpItem(
        "SysML v2 language specification",
        "link-external",
        { command: "vscode.open", title: "Open SysML v2 Spec", arguments: [vscode.Uri.parse("https://www.omg.org/spec/SysML/2.0/")] },
        "link"
      ),
    ];
  }
}

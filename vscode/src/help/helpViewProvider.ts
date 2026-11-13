import * as vscode from "vscode";
import documentation from "./documentation.json";
import { documentationUrl, DocumentationPage } from "./documentation";

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
  constructor(private readonly version: string, private readonly development: boolean) {}

  private documentationItem(page: DocumentationPage, label = documentation.pages[page].title): HelpItem {
    return new HelpItem(label, "link-external", {
      command: "vscode.open",
      title: `Open ${label}`,
      arguments: [vscode.Uri.parse(documentationUrl(this.version, page, this.development))],
    }, "link");
  }

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
        "Open Views panel",
        "type-hierarchy-sub",
        { command: "spec42.diagram.open", title: "Open Views" }
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
      this.documentationItem("overview", "Spec42 documentation"),
      this.documentationItem("gettingStarted"),
      this.documentationItem("troubleshooting"),
      this.documentationItem("included"),
      this.documentationItem("libraries"),
      this.documentationItem("domainLibraries"),
      new HelpItem(
        "SysML v2 language specification",
        "link-external",
        { command: "vscode.open", title: "Open SysML v2 Spec", arguments: [vscode.Uri.parse("https://www.omg.org/spec/SysML/2.0/")] },
        "link"
      ),
    ];
  }
}

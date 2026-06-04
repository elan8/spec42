import * as assert from "assert";
import * as vscode from "vscode";
import {
  compareExampleItems,
  ExampleTreeItem,
  isVisibleExampleFolder,
  metadataForExample,
} from "../../examples/examplesViewProvider";

describe("examplesViewProvider", () => {
  it("hides dot-prefixed directories such as .github", () => {
    assert.strictEqual(isVisibleExampleFolder(".github"), false);
    assert.strictEqual(isVisibleExampleFolder(".git"), false);
  });

  it("shows normal example folder names", () => {
    assert.strictEqual(isVisibleExampleFolder("timer"), true);
    assert.strictEqual(isVisibleExampleFolder("webshop"), true);
  });

  it("marks timer as the recommended start example", () => {
    const metadata = metadataForExample("timer");
    assert.strictEqual(metadata?.recommended, true);
    assert.strictEqual(metadata?.primaryFile, "KitchenTimer.sysml");
  });

  it("sorts the recommended example before alphabetical examples", () => {
    const root = vscode.Uri.file("C:/examples");
    const drone = new ExampleTreeItem(
      vscode.Uri.joinPath(root, "drone"),
      "drone",
      metadataForExample("drone")
    );
    const timer = new ExampleTreeItem(
      vscode.Uri.joinPath(root, "timer"),
      "timer",
      metadataForExample("timer")
    );
    const sorted = [drone, timer].sort(compareExampleItems);
    assert.strictEqual(sorted[0].label, "timer");
    assert.strictEqual(timer.contextValue, "spec42ExampleFolderRecommended");
  });
});

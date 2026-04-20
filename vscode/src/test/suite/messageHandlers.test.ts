import * as assert from "assert";
import * as vscode from "vscode";
import { createMessageDispatcher } from "../../visualization/messageHandlers";

function createContext() {
  const calls: Array<{ force: boolean; triggerSource?: string }> = [];
  let currentView = "general-view";
  let lastContentHash = "seed";
  let selectedPackage: string | undefined;

  const dispatcher = createMessageDispatcher({
    panel: {} as vscode.WebviewPanel,
    document: {
      uri: vscode.Uri.parse("file:///drone.sysml"),
    } as vscode.TextDocument,
    workspaceRootUri: "file:///workspace",
    lspModelProvider: {} as any,
    updateVisualization: (force: boolean, triggerSource?: string) => {
      calls.push({ force, triggerSource });
    },
    setNavigating: () => {},
    setCurrentView: (view: string) => {
      currentView = view;
    },
    setSelectedPackage: (value?: string) => {
      selectedPackage = value;
    },
    setLastContentHash: (hash: string) => {
      lastContentHash = hash;
    },
  });

  return {
    dispatcher,
    calls,
    getCurrentView: () => currentView,
    getLastContentHash: () => lastContentHash,
    getSelectedPackage: () => selectedPackage,
  };
}

describe("createMessageDispatcher", () => {
  it("refreshes visualization when the webview changes view", () => {
    const ctx = createContext();

    ctx.dispatcher({ command: "viewChanged", view: "interconnection-view" });

    assert.strictEqual(ctx.getCurrentView(), "interconnection-view");
    assert.strictEqual(ctx.getLastContentHash(), "");
    assert.deepStrictEqual(ctx.calls, [{ force: true, triggerSource: "viewChanged" }]);
  });

  it("does not trigger a refresh for currentViewResponse", () => {
    const ctx = createContext();

    ctx.dispatcher({ command: "currentViewResponse", view: "action-flow-view" });

    assert.strictEqual(ctx.getCurrentView(), "action-flow-view");
    assert.strictEqual(ctx.calls.length, 0);
  });

  it("refreshes visualization when the package filter changes", () => {
    const ctx = createContext();

    ctx.dispatcher({ command: "packageFilterChanged", packageRef: "AnalysisPackage" });

    assert.strictEqual(ctx.getSelectedPackage(), "AnalysisPackage");
    assert.strictEqual(ctx.getLastContentHash(), "");
    assert.deepStrictEqual(ctx.calls, [{ force: true, triggerSource: "packageFilterChanged" }]);
  });
});

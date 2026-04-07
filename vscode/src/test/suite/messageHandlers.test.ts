import * as assert from "assert";
import * as vscode from "vscode";
import { createMessageDispatcher } from "../../visualization/messageHandlers";

function createContext() {
  const calls: Array<{ force: boolean; triggerSource?: string }> = [];
  let currentView = "general-view";
  let lastContentHash = "seed";

  const dispatcher = createMessageDispatcher({
    panel: {} as vscode.WebviewPanel,
    document: {
      uri: vscode.Uri.parse("file:///drone.sysml"),
    } as vscode.TextDocument,
    lspModelProvider: {} as any,
    fileUris: [],
    updateVisualization: (force: boolean, triggerSource?: string) => {
      calls.push({ force, triggerSource });
    },
    setNavigating: () => {},
    setCurrentView: (view: string) => {
      currentView = view;
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
});

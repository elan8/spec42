import * as assert from "assert";
import * as vscode from "vscode";
import { createMessageDispatcher } from "../../visualization/messageHandlers";

function createContext() {
  const calls: Array<{ force: boolean; triggerSource?: string }> = [];
  const inspectCalls: Array<{ uri: string; range: unknown }> = [];
  let currentView = "general-view";
  let lastContentHash = "seed";
  let selectedView: string | undefined;

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
    setSelectedView: (value?: string) => {
      selectedView = value;
    },
    setLastContentHash: (hash: string) => {
      lastContentHash = hash;
    },
    inspectElement: (uri, range) => {
      inspectCalls.push({ uri, range });
    },
  });

  return {
    dispatcher,
    calls,
    inspectCalls,
    getCurrentView: () => currentView,
    getLastContentHash: () => lastContentHash,
    getSelectedView: () => selectedView,
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

  it("refreshes visualization when the selected SysML view changes", () => {
    const ctx = createContext();

    ctx.dispatcher({ command: "viewSelectionChanged", viewId: "AnalysisView" });

    assert.strictEqual(ctx.getSelectedView(), "AnalysisView");
    assert.strictEqual(ctx.getLastContentHash(), "");
    assert.deepStrictEqual(ctx.calls, [{ force: true, triggerSource: "viewSelectionChanged" }]);
  });

  it("forwards inspectElement to the Feature Inspector when a location is present", () => {
    const ctx = createContext();
    const elementRange = {
      start: { line: 3, character: 1 },
      end: { line: 3, character: 10 },
    };

    ctx.dispatcher({
      command: "inspectElement",
      elementUri: "file:///drone.sysml",
      elementRange,
    });

    assert.deepStrictEqual(ctx.inspectCalls, [
      { uri: "file:///drone.sysml", range: elementRange },
    ]);
  });

  it("ignores inspectElement without a resolvable location", () => {
    const ctx = createContext();

    ctx.dispatcher({ command: "inspectElement" });

    assert.deepStrictEqual(ctx.inspectCalls, []);
  });

  it("ignores invalid webview messages", () => {
    const ctx = createContext();

    ctx.dispatcher(null);
    ctx.dispatcher({ view: "interconnection-view" });

    assert.strictEqual(ctx.getCurrentView(), "general-view");
    assert.strictEqual(ctx.getLastContentHash(), "seed");
    assert.deepStrictEqual(ctx.calls, []);
  });
});

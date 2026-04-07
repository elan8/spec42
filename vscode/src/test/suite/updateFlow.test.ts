import * as assert from "assert";
import * as vscode from "vscode";
import { createUpdateVisualizationFlow } from "../../visualization/updateFlow";

function createMockDocument(uri: string): vscode.TextDocument {
  return {
    uri: vscode.Uri.parse(uri),
    getText: () => "package Drone {}",
  } as vscode.TextDocument;
}

function createMockPanel() {
  const messages: unknown[] = [];
  return {
    panel: {
      visible: true,
      webview: {
        postMessage: (message: unknown) => {
          messages.push(message);
          return true;
        },
      },
    } as unknown as vscode.WebviewPanel,
    messages,
  };
}

describe("createUpdateVisualizationFlow", () => {
  it("deduplicates repeated webviewReady startup updates", async () => {
    let getModelCount = 0;
    const { panel } = createMockPanel();
    const document = createMockDocument("file:///drone.sysml");
    const provider = {
      getModel: async () => {
        getModelCount += 1;
        await new Promise((resolve) => setTimeout(resolve, 20));
        return {
          version: 1,
          graph: { nodes: [], edges: [] },
          stats: {
            totalElements: 0,
            resolvedElements: 0,
            unresolvedElements: 0,
            parseTimeMs: 1,
            modelBuildTimeMs: 1,
            parseCached: true,
          },
        };
      },
      getDiagram: async () => ({
        version: 1,
        kind: "general-view",
        sourceUri: document.uri.toString(),
        scene: {},
      }),
    } as any;

    const flow = createUpdateVisualizationFlow({
      panel,
      getDocument: () => document,
      getFileUris: () => [],
      lspModelProvider: provider,
      getCurrentView: () => "general-view",
      getPendingPackageName: () => undefined,
      getIsNavigating: () => false,
      getNeedsUpdateWhenVisible: () => false,
      getLastContentHash: () => "",
      setLastContentHash: () => {},
      setNeedsUpdateWhenVisible: () => {},
      clearPendingPackageName: () => {},
    });

    await Promise.all([
      flow.update(true, "webviewReady"),
      flow.update(true, "webviewReady"),
    ]);

    assert.strictEqual(getModelCount, 1);
  });

  it("skips pre-bootstrap forced updates until webviewReady", async () => {
    let getModelCount = 0;
    const { panel } = createMockPanel();
    const document = createMockDocument("file:///drone.sysml");
    const provider = {
      getModel: async () => {
        getModelCount += 1;
        return {
          version: 1,
          graph: { nodes: [], edges: [] },
          stats: {
            totalElements: 0,
            resolvedElements: 0,
            unresolvedElements: 0,
            parseTimeMs: 1,
            modelBuildTimeMs: 1,
            parseCached: true,
          },
        };
      },
      getDiagram: async () => ({
        version: 1,
        kind: "general-view",
        sourceUri: document.uri.toString(),
        scene: {},
      }),
    } as any;

    const flow = createUpdateVisualizationFlow({
      panel,
      getDocument: () => document,
      getFileUris: () => [],
      lspModelProvider: provider,
      getCurrentView: () => "general-view",
      getPendingPackageName: () => undefined,
      getIsNavigating: () => false,
      getNeedsUpdateWhenVisible: () => false,
      getLastContentHash: () => "",
      setLastContentHash: () => {},
      setNeedsUpdateWhenVisible: () => {},
      clearPendingPackageName: () => {},
    });

    await flow.update(true, "panelReveal");
    assert.strictEqual(getModelCount, 0);

    await flow.update(true, "webviewReady");
    assert.strictEqual(getModelCount, 1);
  });

  it("allows a later view change to trigger a new fetch after bootstrap", async () => {
    let getModelCount = 0;
    let currentView = "general-view";
    const requestedScopes: string[][] = [];
    const { panel } = createMockPanel();
    const document = createMockDocument("file:///drone.sysml");
    const provider = {
      getModel: async (_uri: string, scopes?: string[]) => {
        getModelCount += 1;
        requestedScopes.push(scopes ?? []);
        return {
          version: 1,
          graph: { nodes: [], edges: [] },
          stats: {
            totalElements: 0,
            resolvedElements: 0,
            unresolvedElements: 0,
            parseTimeMs: 1,
            modelBuildTimeMs: 1,
            parseCached: true,
          },
        };
      },
      getDiagram: async () => ({
        version: 1,
        kind: "general-view",
        sourceUri: document.uri.toString(),
        scene: {},
      }),
    } as any;

    const flow = createUpdateVisualizationFlow({
      panel,
      getDocument: () => document,
      getFileUris: () => [],
      lspModelProvider: provider,
      getCurrentView: () => currentView,
      getPendingPackageName: () => undefined,
      getIsNavigating: () => false,
      getNeedsUpdateWhenVisible: () => false,
      getLastContentHash: () => "",
      setLastContentHash: () => {},
      setNeedsUpdateWhenVisible: () => {},
      clearPendingPackageName: () => {},
    });

    await flow.update(true, "webviewReady");
    currentView = "action-flow-view";
    await flow.update(true, "viewChanged");

    assert.strictEqual(getModelCount, 2);
    assert.deepStrictEqual(requestedScopes[0], ["graph", "stats"]);
    assert.deepStrictEqual(requestedScopes[1], [
      "graph",
      "activityDiagrams",
      "stats",
    ]);
  });
});

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
    let getVisualizationCount = 0;
    const { panel } = createMockPanel();
    const document = createMockDocument("file:///drone.sysml");
    const provider = {
      getVisualization: async () => {
        getVisualizationCount += 1;
        await new Promise((resolve) => setTimeout(resolve, 20));
        return {
          version: 1,
          view: "general-view",
          workspaceRootUri: "file:///workspace",
          viewCandidates: [],
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
      }
    } as any;

    const flow = createUpdateVisualizationFlow({
      panel,
      getDocument: () => document,
      getWorkspaceRootUri: () => "file:///workspace",
      lspModelProvider: provider,
      getCurrentView: () => "general-view",
      getSelectedView: () => undefined,
      setCurrentView: () => {},
      getIsNavigating: () => false,
      getNeedsUpdateWhenVisible: () => false,
      getLastContentHash: () => "",
      setLastContentHash: () => {},
      setNeedsUpdateWhenVisible: () => {},
    });

    await Promise.all([
      flow.update(true, "webviewReady"),
      flow.update(true, "webviewReady"),
    ]);

    assert.strictEqual(getVisualizationCount, 1);
  });

  it("allows non-webviewReady forced updates before bootstrap", async () => {
    let getVisualizationCount = 0;
    const { panel } = createMockPanel();
    const document = createMockDocument("file:///drone.sysml");
    const provider = {
      getVisualization: async () => {
        getVisualizationCount += 1;
        return {
          version: 1,
          view: "general-view",
          workspaceRootUri: "file:///workspace",
          viewCandidates: [],
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
      }
    } as any;

    const flow = createUpdateVisualizationFlow({
      panel,
      getDocument: () => document,
      getWorkspaceRootUri: () => "file:///workspace",
      lspModelProvider: provider,
      getCurrentView: () => "general-view",
      getSelectedView: () => undefined,
      setCurrentView: () => {},
      getIsNavigating: () => false,
      getNeedsUpdateWhenVisible: () => false,
      getLastContentHash: () => "",
      setLastContentHash: () => {},
      setNeedsUpdateWhenVisible: () => {},
    });

    await flow.update(true, "panelReveal");
    assert.strictEqual(getVisualizationCount, 1);

    await flow.update(true, "webviewReady");
    assert.strictEqual(getVisualizationCount, 2);
  });

  it("allows a later view change to trigger a new fetch after bootstrap", async () => {
    let getVisualizationCount = 0;
    let currentView = "general-view";
    const requests: Array<{ workspaceRootUri: string; view: string; selectedView?: string }> = [];
    const { panel } = createMockPanel();
    const document = createMockDocument("file:///drone.sysml");
    const provider = {
      getVisualization: async (workspaceRootUri: string, view: string, selectedView?: string) => {
        getVisualizationCount += 1;
        requests.push({ workspaceRootUri, view, selectedView });
        return {
          version: 1,
          view,
          workspaceRootUri,
          viewCandidates: [],
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
      }
    } as any;

    const flow = createUpdateVisualizationFlow({
      panel,
      getDocument: () => document,
      getWorkspaceRootUri: () => "file:///workspace",
      lspModelProvider: provider,
      getCurrentView: () => currentView,
      getSelectedView: () => undefined,
      setCurrentView: (view: string) => { currentView = view; },
      getIsNavigating: () => false,
      getNeedsUpdateWhenVisible: () => false,
      getLastContentHash: () => "",
      setLastContentHash: () => {},
      setNeedsUpdateWhenVisible: () => {},
    });

    await flow.update(true, "webviewReady");
    currentView = "action-flow-view";
    await flow.update(true, "viewChanged");

    assert.strictEqual(getVisualizationCount, 2);
    assert.deepStrictEqual(requests, [
      {
        workspaceRootUri: "file:///workspace",
        view: "general-view",
        selectedView: undefined,
      },
      {
        workspaceRootUri: "file:///workspace",
        view: "action-flow-view",
        selectedView: undefined,
      },
    ]);
  });
});

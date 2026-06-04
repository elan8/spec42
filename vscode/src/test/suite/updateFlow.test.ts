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
      getCurrentView: () => "general-view",
      getSelectedView: () => undefined,
      setCurrentView: () => {},
      getIsNavigating: () => false,
      getNeedsUpdateWhenVisible: () => false,
      getLastContentHash: () => "",
      setLastContentHash: () => {},
      setNeedsUpdateWhenVisible: () => {},
      fetchUpdateMessage: async () => {
        getVisualizationCount += 1;
        await new Promise((resolve) => setTimeout(resolve, 20));
        return {
          command: "update",
          modelReady: true,
          graph: { nodes: [], edges: [] },
          generalViewGraph: { nodes: [], edges: [] },
          activityDiagrams: [],
          sequenceDiagrams: [],
          currentView: "general-view",
          viewCandidates: [],
        };
      },
    });

    await Promise.all([
      flow.update(true, "webviewReady"),
      flow.update(true, "webviewReady"),
    ]);

    assert.strictEqual(getVisualizationCount, 1);
  });

  it("posts modelNotReady without fetching when client gate is closed", async () => {
    const { panel, messages } = createMockPanel();
    let fetchCount = 0;
    const flow = createUpdateVisualizationFlow({
      panel,
      getDocument: () => createMockDocument("file:///drone.sysml"),
      getWorkspaceRootUri: () => "file:///workspace",
      getCurrentView: () => "general-view",
      getSelectedView: () => undefined,
      setCurrentView: () => {},
      getIsNavigating: () => false,
      getNeedsUpdateWhenVisible: () => false,
      getLastContentHash: () => "",
      setLastContentHash: () => {},
      setNeedsUpdateWhenVisible: () => {},
      fetchUpdateMessage: async () => {
        fetchCount += 1;
        return {
          command: "update",
          modelReady: true,
          graph: { nodes: [], edges: [] },
          generalViewGraph: { nodes: [], edges: [] },
          activityDiagrams: [],
          sequenceDiagrams: [],
          currentView: "general-view",
          viewCandidates: [],
        };
      },
    });

    const { setVisualizationGateState } = await import("../../visualization/visualizationGate");
    setVisualizationGateState({ languageClientReady: false, serverHealthState: "starting" });

    await flow.update(true, "webviewReady");

    assert.strictEqual(fetchCount, 0);
    assert.ok(messages.some((message) => (message as { command?: string }).command === "modelNotReady"));
  });

  it("allows non-webviewReady forced updates before bootstrap", async () => {
    let getVisualizationCount = 0;
    const { panel } = createMockPanel();
    const document = createMockDocument("file:///drone.sysml");

    const flow = createUpdateVisualizationFlow({
      panel,
      getDocument: () => document,
      getWorkspaceRootUri: () => "file:///workspace",
      getCurrentView: () => "general-view",
      getSelectedView: () => undefined,
      setCurrentView: () => {},
      getIsNavigating: () => false,
      getNeedsUpdateWhenVisible: () => false,
      getLastContentHash: () => "",
      setLastContentHash: () => {},
      setNeedsUpdateWhenVisible: () => {},
      fetchUpdateMessage: async () => {
        getVisualizationCount += 1;
        return {
          command: "update",
          modelReady: true,
          graph: { nodes: [], edges: [] },
          generalViewGraph: { nodes: [], edges: [] },
          activityDiagrams: [],
          sequenceDiagrams: [],
          currentView: "general-view",
          viewCandidates: [],
        };
      },
    });

    await flow.update(true, "panelReveal");
    assert.strictEqual(getVisualizationCount, 1);

    await flow.update(true, "webviewReady");
    assert.strictEqual(getVisualizationCount, 2);
  });

  it("allows a later view change to trigger a new fetch after bootstrap", async () => {
    let getVisualizationCount = 0;
    let currentView = "general-view";
    const requests: Array<{ view: string }> = [];
    const { panel } = createMockPanel();
    const document = createMockDocument("file:///drone.sysml");

    const flow = createUpdateVisualizationFlow({
      panel,
      getDocument: () => document,
      getWorkspaceRootUri: () => "file:///workspace",
      getCurrentView: () => currentView,
      getSelectedView: () => undefined,
      setCurrentView: (view: string) => { currentView = view; },
      getIsNavigating: () => false,
      getNeedsUpdateWhenVisible: () => false,
      getLastContentHash: () => "",
      setLastContentHash: () => {},
      setNeedsUpdateWhenVisible: () => {},
      fetchUpdateMessage: async () => {
        getVisualizationCount += 1;
        requests.push({ view: currentView });
        return {
          command: "update",
          modelReady: true,
          graph: { nodes: [], edges: [] },
          generalViewGraph: { nodes: [], edges: [] },
          activityDiagrams: [],
          sequenceDiagrams: [],
          currentView,
          viewCandidates: [],
        };
      },
    });

    await flow.update(true, "webviewReady");
    currentView = "action-flow-view";
    await flow.update(true, "viewChanged");

    assert.strictEqual(getVisualizationCount, 2);
    assert.deepStrictEqual(requests, [{ view: "general-view" }, { view: "action-flow-view" }]);
  });

  it("skips unchanged startup retries after the first successful render", async () => {
    let getVisualizationCount = 0;
    let lastContentHash = "";
    const { panel, messages } = createMockPanel();
    const document = createMockDocument("file:///drone.sysml");

    const flow = createUpdateVisualizationFlow({
      panel,
      getDocument: () => document,
      getWorkspaceRootUri: () => "file:///workspace",
      getCurrentView: () => "general-view",
      getSelectedView: () => undefined,
      setCurrentView: () => {},
      getIsNavigating: () => false,
      getNeedsUpdateWhenVisible: () => false,
      getLastContentHash: () => lastContentHash,
      setLastContentHash: (hash) => { lastContentHash = hash; },
      setNeedsUpdateWhenVisible: () => {},
      fetchUpdateMessage: async () => {
        getVisualizationCount += 1;
        return {
          command: "update",
          modelReady: true,
          graph: { nodes: [], edges: [] },
          generalViewGraph: { nodes: [], edges: [] },
          activityDiagrams: [],
          sequenceDiagrams: [],
          currentView: "general-view",
          viewCandidates: [],
        };
      },
    });

    await flow.update(true, "webviewReady");
    await flow.update(true, "startupRetry");

    assert.strictEqual(getVisualizationCount, 1);
    assert.strictEqual(messages.filter((message) => (message as { command?: string }).command === "showLoading").length, 1);
  });
});

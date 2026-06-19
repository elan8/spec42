import * as assert from "assert";
import * as path from "path";
import * as vscode from "vscode";
import { resetWorkspaceLifecycleSnapshotProvider } from "../../activation/workspaceLifecycle";
import {
  BaseVisualizationPanelController,
  type BaseVisualizerRestoreState,
  type VisualizationPanelRuntimeState,
  type VisualizationPanelVariantConfig,
} from "../../visualization/baseVisualizationPanelController";
import { resetVisualizerRenderTracker } from "../../visualization/renderTracker";
import {
  setVisualizerBootstrapCompleted,
  setVisualizerUpdateInFlight,
} from "../../visualization/visualizerReadiness";
import { setVisualizationGateState } from "../../visualization/visualizationGate";
import { waitFor } from "./testUtils";

function createMockPanel() {
  const messages: unknown[] = [];
  const disposeHandlers: Array<() => void> = [];
  const viewStateHandlers: Array<() => void> = [];
  const panel = {
    title: "SysML Visualizer",
    visible: true,
    viewColumn: vscode.ViewColumn.One,
    webview: {
      html: "",
      cspSource: "https://webview.vscode-cdn.net",
      asWebviewUri: (uri: vscode.Uri) => uri,
      postMessage: (message: unknown) => {
        messages.push(message);
        return true;
      },
      onDidReceiveMessage: () => ({ dispose() {} }),
    },
    onDidDispose: (handler: () => void) => {
      disposeHandlers.push(handler);
      return { dispose() {} };
    },
    onDidChangeViewState: (handler: () => void) => {
      viewStateHandlers.push(handler);
      return { dispose() {} };
    },
    reveal: () => {},
    dispose: () => {
      disposeHandlers.forEach((handler) => handler());
    },
  } as unknown as vscode.WebviewPanel;

  return { panel, messages, viewStateHandlers };
}

describe("BaseVisualizationPanelController", () => {
  beforeEach(() => {
    resetWorkspaceLifecycleSnapshotProvider();
    setVisualizationGateState({
      languageClientReady: true,
      serverHealthState: "ready",
    });
    setVisualizerBootstrapCompleted(false);
    setVisualizerUpdateInFlight(false);
    resetVisualizerRenderTracker();
  });

  it("supports documentless variants and debounces tracked refreshes", async () => {
    let fetchCount = 0;
    const { panel, messages } = createMockPanel();
    const workspaceStateUpdates: Array<{ key: string; value: unknown }> = [];
    const runtimeState: VisualizationPanelRuntimeState = {
      workspaceRootUri: "file:///workspace",
      currentView: "general-view",
      selectedView: "general-view",
      document: undefined,
      lspModelProvider: {} as never,
    };
    const config: VisualizationPanelVariantConfig<BaseVisualizerRestoreState> = {
      panelTypeId: "sysmlVisualizer",
      restoreStateKey: "visualizerRestoreState",
      defaultTitle: "SysML Visualizer",
      enabledViews: ["general-view", "interconnection-view"],
      defaultView: "general-view",
      loadingMessage: "Loading SysML visualization...",
      waitForTrackedUriDiagnostics: async () => {},
      getRuntimeState: () => runtimeState,
      updateCurrentView: (view) => {
        runtimeState.currentView = view;
      },
      updateSelectedView: (selectedView) => {
        runtimeState.selectedView = selectedView;
      },
      serializeRestoreState: (state, title) => ({
        workspaceRootUri: state.workspaceRootUri,
        currentView: state.currentView,
        title,
      }),
      fetchUpdateMessage: async () => {
        fetchCount += 1;
        return {
          command: "update",
          graph: { nodes: [], edges: [] },
          elements: [],
          generalViewGraph: { nodes: [], edges: [] },
          activityDiagrams: [],
          sequenceDiagrams: [],
          currentView: runtimeState.currentView,
          viewCandidates: [],
        };
      },
      getContentHashSource: (state) =>
        JSON.stringify({
          workspaceRootUri: state.workspaceRootUri,
          currentView: state.currentView,
        }),
      shouldTrackUri: (uri) => uri.fsPath.toLowerCase().endsWith(".sysml"),
    };

    const extension = vscode.extensions.all.find(
      (entry) => entry.packageJSON?.name === "spec42"
    );
    assert.ok(extension, "SysML Language Server extension should be installed");

    const controller = new BaseVisualizationPanelController(
      panel,
      vscode.Uri.file(extension.extensionPath),
      {
        workspaceState: {
          update: async (key: string, value: unknown) => {
            workspaceStateUpdates.push({ key, value });
          },
        },
      } as unknown as vscode.ExtensionContext,
      config
    );

    await controller.updateVisualization(true, "webviewReady");
    assert.ok(fetchCount >= 1, "initial webviewReady update should fetch once");
    // Let the built-in 2000ms startup retry finish so only debounced file changes add fetches.
    await waitFor(
      "startup retries settled",
      async () => fetchCount,
      (count) => (count ?? 0) >= 1,
      4000,
      100
    );
    const settledFetchCount = fetchCount;

    await controller.notifyTrackedUriChanged(
      vscode.Uri.file(path.join("/workspace", "model.sysml"))
    );
    await controller.notifyTrackedUriChanged(
      vscode.Uri.file(path.join("/workspace", "parts.sysml"))
    );
    await waitFor(
      "debounced tracked-uri fetch",
      async () => fetchCount,
      (count) => count === settledFetchCount + 1,
      8000,
      50
    );

    controller.dispose();

    assert.ok(messages.some((message) => (message as { command?: string }).command === "requestCurrentView"));
    assert.ok(workspaceStateUpdates.some((entry) => entry.key === "visualizerRestoreState" && entry.value !== undefined));
    assert.ok(workspaceStateUpdates.some((entry) => entry.key === "visualizerRestoreState" && entry.value === undefined));
  });
});

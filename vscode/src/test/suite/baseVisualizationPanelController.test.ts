import * as assert from "assert";
import * as vscode from "vscode";
import {
  BaseVisualizationPanelController,
  type BaseVisualizerRestoreState,
  type VisualizationPanelRuntimeState,
  type VisualizationPanelVariantConfig,
} from "../../visualization/baseVisualizationPanelController";

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

    const controller = new BaseVisualizationPanelController(
      panel,
      vscode.Uri.file("C:\\Git\\spec42\\vscode"),
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
    assert.strictEqual(fetchCount, 1);

    await controller.notifyTrackedUriChanged(vscode.Uri.file("C:\\workspace\\model.sysml"));
    await controller.notifyTrackedUriChanged(vscode.Uri.file("C:\\workspace\\parts.sysml"));
    await new Promise((resolve) => setTimeout(resolve, 500));
    assert.strictEqual(fetchCount, 2);

    controller.dispose();

    assert.ok(messages.some((message) => (message as { command?: string }).command === "requestCurrentView"));
    assert.ok(workspaceStateUpdates.some((entry) => entry.key === "visualizerRestoreState" && entry.value !== undefined));
    assert.ok(workspaceStateUpdates.some((entry) => entry.key === "visualizerRestoreState" && entry.value === undefined));
  });
});

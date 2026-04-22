import * as assert from "assert";
import { fetchModelData } from "../../visualization/modelFetcher";
import type { SysMLVisualizationResult } from "../../providers/sysmlModelTypes";

function createVisualizationResult(): SysMLVisualizationResult {
  return {
    version: 1,
    view: "general-view",
    workspaceRootUri: "file:///workspace",
    viewCandidates: [
      { id: "AnalysisView", name: "Analysis View", rendererView: "general-view", supported: true },
      { id: "FunctionsView", name: "Functions View", rendererView: "action-flow-view", supported: true },
    ],
    graph: {
      nodes: [
        {
          id: "AnalysisPackage",
          type: "package",
          name: "AnalysisPackage",
          uri: "file:///workspace/Analysis/AnalysisPackage.sysml",
          range: {
            start: { line: 0, character: 0 },
            end: { line: 3, character: 0 },
          },
          attributes: {},
        },
      ],
      edges: [],
    },
    generalViewGraph: {
      nodes: [],
      edges: [],
    },
    workspaceModel: {
      semantic: [
        {
          id: "AnalysisPackage",
          type: "package",
          name: "AnalysisPackage",
          uri: "file:///workspace/Analysis/AnalysisPackage.sysml",
          range: {
            start: { line: 0, character: 0 },
            end: { line: 1, character: 0 },
          },
          children: [],
          attributes: {},
          relationships: [],
        },
      ],
      files: [],
      summary: {
        scannedFiles: 2,
        loadedFiles: 2,
        failures: 0,
        truncated: false,
      },
    },
    stats: {
      totalElements: 1,
      resolvedElements: 0,
      unresolvedElements: 0,
      parseTimeMs: 0,
      modelBuildTimeMs: 1,
      parseCached: false,
    },
  };
}

describe("fetchModelData", () => {
  it("uses the workspace-only visualization endpoint", async () => {
    const requests: Array<{ workspaceRootUri: string; view: string; selectedView?: string }> = [];
    const provider = {
      getVisualization: async (workspaceRootUri: string, view: string, selectedView?: string) => {
        requests.push({ workspaceRootUri, view, selectedView });
        return createVisualizationResult();
      },
    } as any;

    await fetchModelData({
      workspaceRootUri: "file:///workspace",
      lspModelProvider: provider,
      currentView: "general-view",
    });

    assert.deepStrictEqual(requests, [
      {
        workspaceRootUri: "file:///workspace",
        view: "general-view",
        selectedView: undefined,
      },
    ]);
  });

  it("passes the selected SysML view when one is selected", async () => {
    const requests: Array<{ workspaceRootUri: string; view: string; selectedView?: string }> = [];
    const provider = {
      getVisualization: async (workspaceRootUri: string, view: string, selectedView?: string) => {
        requests.push({ workspaceRootUri, view, selectedView });
        return {
          ...createVisualizationResult(),
          selectedView: "AnalysisView",
          selectedViewName: "Analysis View",
        };
      },
    } as any;

    const result = await fetchModelData({
      workspaceRootUri: "file:///workspace",
      lspModelProvider: provider,
      currentView: "interconnection-view",
      selectedView: "AnalysisView",
    });

    assert.deepStrictEqual(requests, [
      {
        workspaceRootUri: "file:///workspace",
        view: "interconnection-view",
        selectedView: "AnalysisView",
      },
    ]);
    assert.strictEqual(result?.selectedViewName, "Analysis View");
  });

  it("includes backend view candidates and semantic roots in the update message", async () => {
    const provider = {
      getVisualization: async () => createVisualizationResult(),
    } as any;

    const result = await fetchModelData({
      workspaceRootUri: "file:///workspace",
      lspModelProvider: provider,
      currentView: "general-view",
    });

    assert.deepStrictEqual(
      result?.viewCandidates?.map((candidate) => candidate.name),
      ["Analysis View", "Functions View"]
    );
    assert.deepStrictEqual(
      result?.elements?.map((element) => element.name),
      ["AnalysisPackage"]
    );
  });
});

import * as assert from "assert";
import { fetchModelData } from "../../visualization/modelFetcher";
import type { SysMLVisualizationResult } from "../../providers/sysmlModelTypes";

function createVisualizationResult(): SysMLVisualizationResult {
  return {
    version: 1,
    view: "general-view",
    workspaceRootUri: "file:///workspace",
    packageCandidates: [
      { id: "AnalysisPackage", name: "AnalysisPackage" },
      { id: "FunctionsPackage", name: "FunctionsPackage" },
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
    const requests: Array<{ workspaceRootUri: string; view: string; packageFilter?: { kind: string; package?: string } }> = [];
    const provider = {
      getVisualization: async (workspaceRootUri: string, view: string, packageFilter?: { kind: string; package?: string }) => {
        requests.push({ workspaceRootUri, view, packageFilter });
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
        packageFilter: { kind: "all" },
      },
    ]);
  });

  it("passes a package filter when one package is selected", async () => {
    const requests: Array<{ workspaceRootUri: string; view: string; packageFilter?: { kind: string; package?: string } }> = [];
    const provider = {
      getVisualization: async (workspaceRootUri: string, view: string, packageFilter?: { kind: string; package?: string }) => {
        requests.push({ workspaceRootUri, view, packageFilter });
        return {
          ...createVisualizationResult(),
          selectedPackage: "AnalysisPackage",
          selectedPackageName: "AnalysisPackage",
        };
      },
    } as any;

    const result = await fetchModelData({
      workspaceRootUri: "file:///workspace",
      lspModelProvider: provider,
      currentView: "interconnection-view",
      selectedPackage: "AnalysisPackage",
    });

    assert.deepStrictEqual(requests, [
      {
        workspaceRootUri: "file:///workspace",
        view: "interconnection-view",
        packageFilter: { kind: "package", package: "AnalysisPackage" },
      },
    ]);
    assert.strictEqual(result?.selectedPackageName, "AnalysisPackage");
  });

  it("includes backend package candidates and semantic roots in the update message", async () => {
    const provider = {
      getVisualization: async () => createVisualizationResult(),
    } as any;

    const result = await fetchModelData({
      workspaceRootUri: "file:///workspace",
      lspModelProvider: provider,
      currentView: "general-view",
    });

    assert.deepStrictEqual(
      result?.packageCandidates?.map((candidate) => candidate.name),
      ["AnalysisPackage", "FunctionsPackage"]
    );
    assert.deepStrictEqual(
      result?.elements?.map((element) => element.name),
      ["AnalysisPackage"]
    );
  });
});

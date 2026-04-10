import * as assert from "assert";
import * as vscode from "vscode";
import { fetchModelData } from "../../visualization/modelFetcher";
import type { SysMLDiagramResult, SysMLModelResult } from "../../providers/sysmlModelTypes";

function createModelResult(): SysMLModelResult {
  return {
    version: 1,
    graph: {
      nodes: [
        {
          id: "Drone",
          type: "package",
          name: "Drone",
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
    activityDiagrams: [
      {
        name: "MissionFlow",
        actions: [],
        flows: [],
        decisions: [],
        states: [],
        range: {
          start: { line: 0, character: 0 },
          end: { line: 0, character: 0 },
        },
      },
    ],
    stats: {
      totalElements: 1,
      resolvedElements: 1,
      unresolvedElements: 0,
      parseTimeMs: 1,
      modelBuildTimeMs: 1,
      parseCached: true,
    },
  };
}

function createDiagramResult(): SysMLDiagramResult {
  return {
    version: 1,
    kind: "general-view",
    sourceUri: "file:///drone.sysml",
    scene: {
      generalView: {
        nodes: [],
        edges: [],
        bounds: { x: 0, y: 0, width: 1, height: 1 },
      },
    },
  } as unknown as SysMLDiagramResult;
}

describe("fetchModelData", () => {
  it("omits activityDiagrams scope for general view requests", async () => {
    const requestedScopes: string[][] = [];
    const requestedDiagrams: string[] = [];
    const provider = {
      getModel: async (_uri: string, scopes?: string[]) => {
        requestedScopes.push(scopes ?? []);
        return createModelResult();
      },
      getDiagram: async (_uri: string, kind: string) => {
        requestedDiagrams.push(kind);
        return createDiagramResult();
      },
    } as any;

    await fetchModelData({
      documentUri: "file:///drone.sysml",
      fileUris: [],
      lspModelProvider: provider,
      currentView: "general-view",
    });

    assert.deepStrictEqual(requestedScopes, [["graph", "stats"]]);
    assert.deepStrictEqual(requestedDiagrams, ["general-view"]);
  });

  it("includes activityDiagrams scope for action flow view requests", async () => {
    const requestedScopes: string[][] = [];
    const requestedDiagrams: string[] = [];
    const provider = {
      getModel: async (_uri: string, scopes?: string[]) => {
        requestedScopes.push(scopes ?? []);
        return createModelResult();
      },
      getDiagram: async (_uri: string, kind: string) => {
        requestedDiagrams.push(kind);
        return createDiagramResult();
      },
    } as any;

    await fetchModelData({
      documentUri: "file:///drone.sysml",
      fileUris: [],
      lspModelProvider: provider,
      currentView: "action-flow-view",
    });

    assert.deepStrictEqual(requestedScopes, [["graph", "activityDiagrams", "stats"]]);
    assert.deepStrictEqual(requestedDiagrams, []);
  });

  it("includes ibd scope for interconnection view requests", async () => {
    const requestedScopes: string[][] = [];
    const requestedDiagrams: string[] = [];
    const provider = {
      getModel: async (_uri: string, scopes?: string[]) => {
        requestedScopes.push(scopes ?? []);
        return createModelResult();
      },
      getDiagram: async (_uri: string, kind: string) => {
        requestedDiagrams.push(kind);
        return createDiagramResult();
      },
    } as any;

    await fetchModelData({
      documentUri: "file:///drone.sysml",
      fileUris: [],
      lspModelProvider: provider,
      currentView: "interconnection-view",
    });

    assert.deepStrictEqual(requestedScopes, [["graph", "ibd", "stats"]]);
    assert.deepStrictEqual(requestedDiagrams, ["interconnection-view"]);
  });

  it("uses a workspace anchor URI for workspace visualization model requests", async () => {
    const requestedUris: string[] = [];
    const provider = {
      getModel: async (uri: string, _scopes?: string[]) => {
        requestedUris.push(uri);
        return createModelResult();
      },
      getDiagram: async (_uri: string, _kind: string) => createDiagramResult(),
    } as any;

    await fetchModelData({
      documentUri: "file:///c%3A/Git/apollo-11-sysml-v2/Analysis/AnalysisPackage.sysml",
      fileUris: [
        vscode.Uri.file("C:/Git/apollo-11-sysml-v2/Analysis/AnalysisPackage.sysml"),
        vscode.Uri.file("C:/Git/apollo-11-sysml-v2/Function/FunctionsPackage.sysml"),
      ],
      lspModelProvider: provider,
      currentView: "general-view",
    });

    assert.deepStrictEqual(requestedUris, ["file:///c%3A/Git/apollo-11-sysml-v2"]);
  });

  it("includes workspace semantic roots in the update message when available", async () => {
    const provider = {
      getModel: async (_uri: string, _scopes?: string[]) => ({
        ...createModelResult(),
        workspaceModel: {
          semantic: [
            {
              id: "AnalysisPackage",
              type: "package",
              name: "AnalysisPackage",
              range: {
                start: { line: 0, character: 0 },
                end: { line: 1, character: 0 },
              },
              children: [],
              attributes: {},
              relationships: [],
            },
            {
              id: "FunctionsPackage",
              type: "package",
              name: "FunctionsPackage",
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
      }),
      getDiagram: async (_uri: string, _kind: string) => createDiagramResult(),
    } as any;

    const result = await fetchModelData({
      documentUri: "file:///c%3A/Git/apollo-11-sysml-v2/Analysis/AnalysisPackage.sysml",
      fileUris: [
        vscode.Uri.file("C:/Git/apollo-11-sysml-v2/Analysis/AnalysisPackage.sysml"),
        vscode.Uri.file("C:/Git/apollo-11-sysml-v2/Function/FunctionsPackage.sysml"),
      ],
      lspModelProvider: provider,
      currentView: "general-view",
    });

    assert.deepStrictEqual(
      result?.elements?.map((element) => element.name),
      ["AnalysisPackage", "FunctionsPackage"]
    );
  });
});

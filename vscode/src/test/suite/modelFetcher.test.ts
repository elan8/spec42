import * as assert from "assert";
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
});

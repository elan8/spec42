import * as assert from "assert";
import { buildSoftwareUpdateMessage } from "../../visualization/modelFetcher";
import type { LspModelProvider } from "../../providers/lspModelProvider";
import type { SoftwareWorkspaceModelDTO } from "../../providers/sysmlModelTypes";

function createWorkspaceModel(): SoftwareWorkspaceModelDTO {
  return {
    workspaceRoot: "file:///c:/workspace",
    summary: {
      crateCount: 1,
      moduleCount: 1,
      dependencyCount: 1,
    },
    architecture: {
      components: [
        {
          id: "rust:crate:demo",
          name: "demo",
          kind: "crate",
          crateName: "demo",
          modulePath: "demo",
          anchors: [],
          isExternal: false,
        },
      ],
      dependencies: [],
    },
  };
}

describe("buildSoftwareUpdateMessage", () => {
  it("returns an empty-state update when no analysis model is cached", async () => {
    const provider = {} as LspModelProvider;
    const result = await buildSoftwareUpdateMessage(
      "file:///c:/workspace",
      "software-module-view",
      provider,
      undefined
    );

    assert.strictEqual(result.emptyStateMessage?.includes("Run analysis"), true);
    assert.deepStrictEqual(result.graph, { nodes: [], edges: [] });
  });

  it("uses the backend projection request when a cached model is available", async () => {
    let capturedView: string | undefined;
    let capturedWorkspaceRootUri: string | undefined;
    let capturedModel: SoftwareWorkspaceModelDTO | undefined;
    const provider = {
      projectSoftwareView: async (
        workspaceRootUri: string,
        view: string,
        workspaceModel: SoftwareWorkspaceModelDTO
      ) => {
        capturedWorkspaceRootUri = workspaceRootUri;
        capturedView = view;
        capturedModel = workspaceModel;
        return {
          version: 0,
          view,
          workspaceRootUri,
          views: [
            { id: "software-module-view", name: "Rust Module View", supported: true },
            { id: "software-dependency-view", name: "Rust Dependency View", supported: true },
          ],
          graph: {
            nodes: [
              {
                id: "rust:crate:demo",
                type: "crate",
                name: "demo",
                range: {
                  start: { line: 0, character: 0 },
                  end: { line: 0, character: 0 },
                },
                attributes: {},
              },
            ],
            edges: [],
          },
          softwareArchitecture: workspaceModel.architecture,
          workspaceModel: {
            files: [],
            semantic: [],
            summary: {
              scannedFiles: 1,
              loadedFiles: 1,
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
      },
    } as unknown as LspModelProvider;

    const model = createWorkspaceModel();
    const result = await buildSoftwareUpdateMessage(
      "file:///c:/workspace",
      "software-dependency-view",
      provider,
      model
    );

    assert.strictEqual(capturedWorkspaceRootUri, "file:///c:/workspace");
    assert.strictEqual(capturedView, "software-dependency-view");
    assert.strictEqual(capturedModel, model);
    assert.strictEqual(result.currentView, "software-dependency-view");
    assert.strictEqual(result.graph?.nodes.length, 1);
  });
});

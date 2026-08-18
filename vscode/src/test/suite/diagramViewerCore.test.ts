import assert from "node:assert/strict";
import { describe, it } from "node:test";
import {
  buildGenerateArgv,
  diagramViewsForDocument,
  DIAGRAM_VIEWS,
  isPathInsideWorkspace,
  parseDiagramProduct,
  parseDiagramViewCatalog,
  parseGenerationReport,
  parseLspGenerationResult,
  parseStateTransitionViewCatalog,
  parseSourceNavigation,
  selectSingleDiagramJson,
} from "../../diagram/diagramViewerCore";

describe("diagram viewer core", () => {
  it("builds a bounded saved-file generation invocation", () => {
    assert.deepEqual(buildGenerateArgv("/plugin.wasm", "/w/model.sysml", "/tmp/out", "/w", ["/lib"]), [
      "generate", "/plugin.wasm", "/w/model.sysml", "--output", "/tmp/out", "--format", "json",
      "--timeout-seconds", "30", "--max-files", "16", "--max-total-bytes", "16777216",
      "--workspace-root", "/w", "--library-path", "/lib",
    ]);
  });

  it("requires the authoritative digest in the report", () => {
    assert.deepEqual(parseGenerationReport({ status: "generated", model_digest: "sha256:abc" }), {
      status: "generated", model_digest: "sha256:abc",
    });
    assert.throws(() => parseGenerationReport({ status: "generated" }));
  });

  it("declares all renderer views at the plugin boundary", () => {
    assert.deepEqual(DIAGRAM_VIEWS.map((view) => view.id), [
      "general-view", "interconnection-view", "action-flow-view", "state-transition-view",
      "sequence-view", "browser-view", "grid-view", "geometry-view",
    ]);
  });

  it("selects exactly one diagram JSON artifact", () => {
    assert.equal(selectSingleDiagramJson(["diagram.json", ".spec42-generator-manifest.json"]), "diagram.json");
    assert.throws(() => selectSingleDiagramJson([]));
    assert.throws(() => selectSingleDiagramJson(["a.json", "b.json"]));
  });

  it("validates persistent LSP artifacts and timing identity", () => {
    const value = {
      modelDigest: "blake3:model",
      generatorDigest: "sha256:guest",
      artifacts: [{ path: "diagram.json", content: [123, 125] }],
      timings: {
        modulePrepareMs: 0,
        guestExecutionUs: 1000,
        preparedReused: true,
        compilationCacheEnabled: true,
        compilationCacheHits: 1,
        compilationCacheMisses: 0,
        compilationCacheError: null,
      },
    };
    assert.deepEqual(parseLspGenerationResult(value), value);
    assert.throws(() => parseLspGenerationResult({ ...value, artifacts: [{ path: "diagram.json", content: [256] }] }));
  });

  it("validates typed state-transition view choices", () => {
    const value = {
      modelDigest: "blake3:model",
      views: [{
        handle: "view:one",
        semanticId: "semantic:one",
        name: "operations",
        exposedMachine: { semanticId: "machine:one", label: "Operations" },
        source: { uri: "file:///workspace/model.sysml", range: {} },
      }],
    };
    assert.deepEqual(parseStateTransitionViewCatalog(value), {
      modelDigest: value.modelDigest,
      views: [{ ...value.views[0], source: { uri: value.views[0].source.uri } }],
    });
    assert.equal(parseStateTransitionViewCatalog({
      modelDigest: value.modelDigest,
      views: [{
        handle: "view:legacy",
        semantic_id: "semantic:legacy",
        name: "legacy",
        exposed_machine: { semantic_id: "machine:legacy", label: "Legacy" },
        source: { uri: "file:///workspace/model.sysml" },
      }],
    }).views[0].exposedMachine.label, "Legacy");
    assert.throws(() => parseStateTransitionViewCatalog({ ...value, views: [{ name: "missing identity" }] }));
  });

  it("offers only diagram capabilities authored by the active document", () => {
    const catalog = parseDiagramViewCatalog({
      modelDigest: "blake3:model",
      views: [{
        kind: "state-transition-view",
        semanticId: "P::lifecycle",
        name: "lifecycle",
        source: { uri: "file:///workspace/views.sysml" },
      }, {
        kind: "general-view",
        semanticId: "P::structure",
        name: "structure",
        source: { uri: "file:///workspace/views.sysml" },
      }, {
        kind: "grid-view",
        semanticId: "Q::matrix",
        name: "matrix",
        source: { uri: "file:///workspace/other.sysml" },
      }],
    });
    assert.deepEqual(
      diagramViewsForDocument(catalog, "file:///workspace/views.sysml").map((view) => view.id),
      ["general-view", "state-transition-view"]
    );
    assert.deepEqual(diagramViewsForDocument(catalog, "file:///workspace/structure.sysml"), []);
  });

  it("validates the versioned render product and explicit incompleteness", () => {
    const value = {
      schemaVersion: 1,
      modelDigest: "blake3:model",
      view: { id: "general-view", name: "General View" },
      completeness: {
        status: "incomplete",
        reasons: [{ code: "diagram.query.unsupported", message: "not implemented", requiredQuery: "general_view" }],
      },
      preparedView: { title: "General View", view: "general-view", nodes: [], edges: [] },
    };
    assert.deepEqual(parseDiagramProduct(JSON.stringify(value)), value);
    assert.throws(() => parseDiagramProduct(JSON.stringify({ ...value, schemaVersion: 2 })));
    assert.throws(() => parseDiagramProduct(JSON.stringify({ ...value, preparedView: { ...value.preparedView, view: "grid-view" } })));
  });

  it("validates bounded source navigation", () => {
    const valid = { uri: "file:///w/model.sysml", startLine: 1, startCharacter: 2, endLine: 1, endCharacter: 4 };
    assert.deepEqual(parseSourceNavigation(valid), valid);
    assert.equal(parseSourceNavigation({ ...valid, startLine: -1 }), undefined);
    assert.equal(parseSourceNavigation({ ...valid, endCharacter: 1 }), undefined);
  });

  it("confines source paths to workspace roots", () => {
    assert.equal(isPathInsideWorkspace("/workspace/model.sysml", ["/workspace"]), true);
    assert.equal(isPathInsideWorkspace("/workspace-other/model.sysml", ["/workspace"]), false);
    assert.equal(isPathInsideWorkspace("/outside/model.sysml", ["/workspace"]), false);
  });
});

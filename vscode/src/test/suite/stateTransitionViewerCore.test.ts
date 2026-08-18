import assert from "node:assert/strict";
import { describe, it } from "node:test";
import {
  buildGenerateArgv,
  isPathInsideWorkspace,
  parseGenerationReport,
  parseLspGenerationResult,
  parseSourceNavigation,
  readSvgMetadata,
  selectSingleSvg,
  validateStandaloneSvg,
} from "../../diagram/stateTransitionViewerCore";

describe("state transition viewer core", () => {
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

  it("selects exactly one SVG", () => {
    assert.equal(selectSingleSvg(["view.svg", ".spec42-generator-manifest.json"]), "view.svg");
    assert.throws(() => selectSingleSvg([]));
    assert.throws(() => selectSingleSvg(["a.svg", "b.svg"]));
  });

  it("validates persistent LSP artifacts and timing identity", () => {
    const value = {
      modelDigest: "blake3:model",
      generatorDigest: "sha256:guest",
      artifacts: [{ path: "view.svg", content: [60, 115, 118, 103, 62] }],
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
    assert.throws(() => parseLspGenerationResult({ ...value, artifacts: [{ path: "view.svg", content: [256] }] }));
  });

  it("rejects active and external SVG content", () => {
    assert.equal(validateStandaloneSvg('<svg xmlns="http://www.w3.org/2000/svg"><path d="M0 0"/></svg>').startsWith("<svg"), true);
    for (const unsafe of [
      "<svg><script>alert(1)</script></svg>",
      '<svg><image href="https://example.com/a.png"/></svg>',
      '<svg><a href="command:evil">bad</a></svg>',
      '<svg><style>@import "https://example.com/a.css"</style></svg>',
      '<!DOCTYPE svg><svg></svg>',
      '<svg><g onclick="alert(1)"/></svg>',
      '<svg><foreignObject>bad</foreignObject></svg>',
    ]) assert.throws(() => validateStandaloneSvg(unsafe));
  });

  it("reads authoritative SVG identity metadata", () => {
    assert.deepEqual(readSvgMetadata('<svg data-view-name="Door lifecycle" data-model-digest="sha256:abc"></svg>'), {
      modelDigest: "sha256:abc", viewName: "Door lifecycle",
    });
    assert.throws(() => readSvgMetadata("<svg></svg>"));
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

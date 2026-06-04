import assert from "node:assert/strict";
import { describe, it } from "node:test";
import {
  appendLibraryPathArgs,
  buildCheckArgv,
  buildDoctorArgv,
  buildExplainDiagnosticArgv,
  buildModelSummaryArgv,
} from "../../lmTools/spec42CliArgs";

describe("spec42Cli argv builder", () => {
  const ctx = {
    serverCommand: "/bin/spec42",
    workspaceRoot: "/workspace",
    libraryPaths: ["/libs/a", "/libs/b"],
  };

  it("appendLibraryPathArgs adds repeated flags", () => {
    assert.deepEqual(appendLibraryPathArgs(["doctor"], ctx.libraryPaths), [
      "doctor",
      "--library-path",
      "/libs/a",
      "--library-path",
      "/libs/b",
    ]);
  });

  it("buildCheckArgv includes workspace root and format json", () => {
    assert.deepEqual(buildCheckArgv(ctx, "/workspace/Model.sysml"), [
      "check",
      "/workspace/Model.sysml",
      "--format",
      "json",
      "--library-path",
      "/libs/a",
      "--library-path",
      "/libs/b",
      "--workspace-root",
      "/workspace",
    ]);
  });

  it("buildDoctorArgv", () => {
    assert.deepEqual(buildDoctorArgv(ctx), [
      "doctor",
      "--format",
      "json",
      "--library-path",
      "/libs/a",
      "--library-path",
      "/libs/b",
    ]);
  });

  it("buildModelSummaryArgv honors max nodes", () => {
    assert.deepEqual(buildModelSummaryArgv(ctx, "models", 1), [
      "model-summary",
      "models",
      "--format",
      "json",
      "--max-nodes",
      "1",
      "--library-path",
      "/libs/a",
      "--library-path",
      "/libs/b",
      "--workspace-root",
      "/workspace",
    ]);
  });

  it("buildExplainDiagnosticArgv", () => {
    assert.deepEqual(
      buildExplainDiagnosticArgv(ctx, "unresolved_type_reference", {
        path: "bad.sysml",
        line: 3,
      }),
      [
        "explain-diagnostic",
        "--code",
        "unresolved_type_reference",
        "--format",
        "json",
        "--library-path",
        "/libs/a",
        "--library-path",
        "/libs/b",
        "--path",
        "bad.sysml",
        "--line",
        "3",
        "--workspace-root",
        "/workspace",
      ]
    );
  });
});

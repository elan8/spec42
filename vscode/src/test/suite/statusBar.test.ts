import * as assert from "assert";
import { analysisStatusText, formatSpec42StatusBar } from "../../statusBar/statusBarViewModel";

describe("statusBar", () => {
  it("distinguishes pending, failed and cancelled retained analysis even with no diagnostics", () => {
    const summary = { activeJobs: 0, pendingJobs: 0, failure: null };
    assert.strictEqual(analysisStatusText({ ...summary, state: "current" }, 0, 0), undefined);
    assert.strictEqual(analysisStatusText({ ...summary, state: "updating" }, 0, 0), "$(sync~spin) SysML: Updating · 0E 0W");
    assert.strictEqual(analysisStatusText({ ...summary, state: "failedRetained" }, 0, 0), "$(error) SysML: Analysis failed · 0E 0W");
    assert.strictEqual(analysisStatusText({ ...summary, state: "cancelledRetained" }, 0, 0), "$(circle-slash) SysML: Analysis cancelled · 0E 0W");
    assert.strictEqual(analysisStatusText(undefined, 0, 0), "$(question) SysML: Analysis status unavailable");
  });
  it("formats degraded server state ahead of diagnostics", () => {
    const vm = formatSpec42StatusBar(
      "degraded",
      "Workspace indexed with failures.",
      0,
      0
    );

    assert.strictEqual(vm.text, "$(warning) SysML: Degraded");
    assert.ok(vm.baseTooltip.includes("Server state: degraded"));
    assert.ok(vm.baseTooltip.includes("Workspace indexed with failures."));
  });

  it("formats ready diagnostics with action-oriented tooltip", () => {
    const vm = formatSpec42StatusBar(
      "ready",
      "",
      1,
      2,
      { errors: 1, warnings: 2, filesWithIssues: 2, totalFiles: 3 },
      { errors: 0, warnings: 1 }
    );

    assert.strictEqual(vm.text, "$(error) SysML: 1E 2W");
    assert.ok(vm.baseTooltip.includes("3 workspace file"));
    assert.ok(vm.baseTooltip.includes("Click for Spec42 actions."));
  });

  it("shows compound status while validating files with diagnostics", () => {
    const vm = formatSpec42StatusBar(
      "ready",
      "",
      3,
      1,
      { errors: 3, warnings: 1, filesWithIssues: 2, totalFiles: 6 },
      undefined,
      "validatingFiles"
    );

    assert.strictEqual(vm.text, "$(sync~spin) Validating files · 3E 1W");
    assert.ok(vm.baseTooltip.includes("Problems reflect open files"));
    assert.ok(vm.baseTooltip.includes("Cross-file editor results update"));
  });

  it("shows compound status while building workspace model", () => {
    const vm = formatSpec42StatusBar(
      "indexing",
      "Building workspace model",
      0,
      0,
      undefined,
      undefined,
      "buildingWorkspaceModel"
    );

    assert.strictEqual(vm.text, "$(sync~spin) Building workspace model");
    assert.ok(vm.baseTooltip.includes("cross-file workspace model"));
  });
});

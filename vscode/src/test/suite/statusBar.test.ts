import * as assert from "assert";
import { formatSpec42StatusBar } from "../../statusBar/statusBarViewModel";

describe("statusBar", () => {
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
});

import * as assert from "assert";
import {
  deriveWorkspaceLifecycle,
  getLifecycleMessage,
  getWorkspaceLifecycle,
  registerWorkspaceLifecycleSnapshotProvider,
  resetWorkspaceLifecycleSnapshotProvider,
  type WorkspaceLifecycleInput,
} from "../../activation/workspaceLifecycle";

function input(
  overrides: Partial<WorkspaceLifecycleInput> = {}
): WorkspaceLifecycleInput {
  return {
    languageClientReady: true,
    serverHealthState: "ready",
    hasWorkspaceFolder: true,
    semanticIndexReady: false,
    workspaceLoadState: "idle",
    hasWorkspaceData: false,
    ...overrides,
  };
}

describe("workspaceLifecycle", () => {
  afterEach(() => {
    resetWorkspaceLifecycleSnapshotProvider();
  });

  it("derives serverStarting when language client is not ready", () => {
    const lifecycle = deriveWorkspaceLifecycle(
      input({ languageClientReady: false })
    );
    assert.strictEqual(lifecycle.phase, "serverStarting");
  });

  it("derives validatingFiles before semantic index is ready", () => {
    const lifecycle = deriveWorkspaceLifecycle(input());
    assert.strictEqual(lifecycle.phase, "validatingFiles");
  });

  it("derives buildingWorkspaceModel during workspace load", () => {
    const lifecycle = deriveWorkspaceLifecycle(
      input({
        semanticIndexReady: true,
        workspaceLoadState: "indexing",
        serverHealthState: "indexing",
      })
    );
    assert.strictEqual(lifecycle.phase, "buildingWorkspaceModel");
  });

  it("derives workspaceReady when model data is loaded", () => {
    const lifecycle = deriveWorkspaceLifecycle(
      input({
        semanticIndexReady: true,
        workspaceLoadState: "ready",
        hasWorkspaceData: true,
      })
    );
    assert.strictEqual(lifecycle.phase, "workspaceReady");
  });

  it("derives degraded when workspace load failed", () => {
    const lifecycle = deriveWorkspaceLifecycle(
      input({
        semanticIndexReady: true,
        workspaceLoadState: "degraded",
        workspaceLoadFailures: 2,
      })
    );
    assert.strictEqual(lifecycle.phase, "degraded");
  });

  it("uses snapshot provider for getWorkspaceLifecycle", () => {
    registerWorkspaceLifecycleSnapshotProvider(() =>
      input({ semanticIndexReady: true, workspaceLoadState: "pending" })
    );
    assert.strictEqual(
      getWorkspaceLifecycle().phase,
      "buildingWorkspaceModel"
    );
  });

  it("returns surface-specific visualizer messages", () => {
    assert.strictEqual(
      getLifecycleMessage("visualizer", "validatingFiles"),
      "Validating SysML files..."
    );
    assert.strictEqual(
      getLifecycleMessage("visualizer", "buildingWorkspaceModel"),
      "Building workspace model..."
    );
    assert.strictEqual(
      getLifecycleMessage("visualizer", "workspaceReady"),
      "Preparing diagram..."
    );
  });
});

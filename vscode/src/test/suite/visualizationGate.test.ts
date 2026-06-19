import * as assert from "assert";
import { registerWorkspaceLifecycleSnapshotProvider, resetWorkspaceLifecycleSnapshotProvider } from "../../activation/workspaceLifecycle";
import {
  evaluateClientVisualizationReadiness,
  setVisualizationGateState,
} from "../../visualization/visualizationGate";

describe("visualizationGate", () => {
  beforeEach(() => {
    setVisualizationGateState({
      languageClientReady: true,
      serverHealthState: "ready",
    });
    registerWorkspaceLifecycleSnapshotProvider(() => ({
      languageClientReady: true,
      serverHealthState: "ready",
      hasWorkspaceFolder: true,
      semanticIndexReady: true,
      workspaceLoadState: "ready",
      hasWorkspaceData: true,
    }));
  });

  afterEach(() => {
    resetWorkspaceLifecycleSnapshotProvider();
  });

  it("blocks when language client is not ready", () => {
    setVisualizationGateState({ languageClientReady: false });
    const readiness = evaluateClientVisualizationReadiness();
    assert.strictEqual(readiness.ready, false);
    assert.ok(readiness.message);
  });

  it("blocks while workspace indexing is in progress", () => {
    setVisualizationGateState({ serverHealthState: "indexing" });
    const readiness = evaluateClientVisualizationReadiness();
    assert.strictEqual(readiness.ready, false);
    assert.match(readiness.message ?? "", /building workspace model/i);
  });

  it("blocks while files are validating before workspace model is ready", () => {
    registerWorkspaceLifecycleSnapshotProvider(() => ({
      languageClientReady: true,
      serverHealthState: "ready",
      hasWorkspaceFolder: true,
      semanticIndexReady: false,
      workspaceLoadState: "idle",
      hasWorkspaceData: false,
    }));
    const readiness = evaluateClientVisualizationReadiness();
    assert.strictEqual(readiness.ready, false);
    assert.match(readiness.message ?? "", /validating sysml files/i);
  });
});

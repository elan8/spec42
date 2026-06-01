import * as assert from "assert";
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
    assert.match(readiness.message ?? "", /indexing/i);
  });
});

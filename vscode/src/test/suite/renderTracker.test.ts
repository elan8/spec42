import * as assert from "assert";
import {
  getLastVisualizerRender,
  onRenderComplete,
  resetVisualizerRenderTracker,
  waitForVisualizerRender,
} from "../../visualization/renderTracker";

describe("renderTracker", () => {
  beforeEach(() => {
    resetVisualizerRenderTracker();
  });

  it("resolves waiters when renderComplete matches", async () => {
    const promise = waitForVisualizerRender({
      view: "general-view",
      outcome: "diagram",
      timeoutMs: 1000,
    });
    onRenderComplete({
      updateId: "upd-1",
      view: "general-view",
      dataHash: "abc",
      outcome: "diagram",
      graphNodes: 3,
      hasExportableSvg: true,
    });
    const event = await promise;
    assert.strictEqual(event.view, "general-view");
    assert.strictEqual(event.outcome, "diagram");
    assert.strictEqual(getLastVisualizerRender()?.graphNodes, 3);
  });

  it("rejects cancelled renders for diagram waiters", async () => {
    const promise = waitForVisualizerRender({
      view: "general-view",
      outcome: "diagram",
      timeoutMs: 200,
    });
    onRenderComplete({
      view: "general-view",
      dataHash: "abc",
      outcome: "cancelled",
      graphNodes: 0,
      hasExportableSvg: false,
    });
    await assert.rejects(promise, /did not settle/);
  });

  it("times out when no matching render arrives", async () => {
    await assert.rejects(
      waitForVisualizerRender({ view: "interconnection-view", timeoutMs: 100 }),
      /did not settle/
    );
  });
});

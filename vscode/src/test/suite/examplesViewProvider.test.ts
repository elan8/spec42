import * as assert from "assert";
import { isVisibleExampleFolder } from "../../examples/examplesViewProvider";

describe("examplesViewProvider", () => {
  it("hides dot-prefixed directories such as .github", () => {
    assert.strictEqual(isVisibleExampleFolder(".github"), false);
    assert.strictEqual(isVisibleExampleFolder(".git"), false);
  });

  it("shows normal example folder names", () => {
    assert.strictEqual(isVisibleExampleFolder("timer"), true);
    assert.strictEqual(isVisibleExampleFolder("webshop"), true);
  });
});

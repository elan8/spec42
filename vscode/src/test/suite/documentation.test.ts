import * as assert from "assert";
import { documentationUrl } from "../../help/documentation";

describe("versioned documentation links", () => {
  it("keeps installed releases on their exact version, including older release lines", () => {
    assert.strictEqual(documentationUrl("0.60.2", "gettingStarted", false), "https://elan8.github.io/spec42/v0.60.2/getting-started.html");
    assert.strictEqual(documentationUrl("1.0.3", "troubleshooting", false), "https://elan8.github.io/spec42/v1.0.3/troubleshooting.html");
    assert.strictEqual(documentationUrl("1.0.3+build.1", "included", false), "https://elan8.github.io/spec42/v1.0.3/included.html");
  });

  it("uses explicitly unreleased documentation for development and prerelease builds", () => {
    assert.strictEqual(documentationUrl("0.60.2", "overview", true), "https://elan8.github.io/spec42/dev/index.html");
    assert.strictEqual(documentationUrl("1.1.0-rc.1", "domainLibraries", false), "https://elan8.github.io/spec42/dev/domain-libraries.html");
  });

  it("rejects invalid version input rather than silently opening a different version", () => {
    assert.throws(() => documentationUrl("latest", "overview", false), /Invalid extension version/);
  });
});

import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";

describe("snippets", () => {
  it("contains workflow snippets with stable prefixes", () => {
    const extension = vscode.extensions.all.find(
      (candidate) => candidate.packageJSON?.name === "spec42"
    );
    assert.ok(extension, "Expected spec42 extension metadata");
    const snippetsPath = path.join(extension.extensionPath, "snippets", "sysml.json");
    const snippets = JSON.parse(fs.readFileSync(snippetsPath, "utf8")) as Record<
      string,
      { prefix?: string }
    >;
    const prefixes = new Set(
      Object.values(snippets).map((snippet) => snippet.prefix)
    );

    for (const prefix of [
      "reqflow",
      "systemports",
      "stateflow",
      "actionflow",
      "viewexpose",
      "satisfyreq",
    ]) {
      assert.ok(prefixes.has(prefix), `missing ${prefix}`);
    }
  });
});

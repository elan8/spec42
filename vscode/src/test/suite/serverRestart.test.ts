import * as assert from "assert";
import * as vscode from "vscode";
import {
  configureServerForTests,
  getFixturePath,
  getTestWorkspaceFolder,
  integrationHookTimeoutMs,
  isCi,
  waitFor,
  waitForExtensionServerReady,
} from "./testUtils";

const FIXTURE_FILE = "SurveillanceDrone.sysml";

function findPosition(doc: vscode.TextDocument, needle: string): vscode.Position {
  const index = doc.getText().indexOf(needle);
  assert.ok(index >= 0, `Expected to find ${needle} in fixture`);
  return doc.positionAt(index);
}

describe("Server restart smoke", () => {
  before(async function () {
    this.timeout(integrationHookTimeoutMs);
    await configureServerForTests();
    getTestWorkspaceFolder();
  });

  it("Server recovers after manual restart", async function () {
    this.timeout(isCi ? 60000 : 45000);
    const filePath = getFixturePath(FIXTURE_FILE);
    const doc = await vscode.workspace.openTextDocument(filePath);
    await vscode.window.showTextDocument(doc);

    await vscode.commands.executeCommand("sysml.restartServer");
    await waitForExtensionServerReady(isCi ? 45000 : 45000);

    const hovers = await waitFor(
      "hover after manual restart",
      () =>
        vscode.commands.executeCommand<vscode.Hover[]>(
          "vscode.executeHoverProvider",
          doc.uri,
          findPosition(doc, "part def Airframe")
        ),
      (value) => Array.isArray(value) && value.length > 0,
      isCi ? 45000 : 45000
    );
    assert.ok(hovers.length > 0, "Server should recover after manual restart");
  });
});

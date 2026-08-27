import * as assert from "assert";
import * as vscode from "vscode";
import {
  closeAllEditorsForTests,
  configureServerForTests,
  getFixturePath,
  getTestWorkspaceFolder,
  integrationHookTimeoutMs,
  waitFor,
  waitForLanguageServerReady,
} from "./testUtils";

function findPosition(doc: vscode.TextDocument, needle: string, occurrence = 0): vscode.Position {
  const text = doc.getText();
  let from = 0;
  let index = -1;
  for (let i = 0; i <= occurrence; i += 1) {
    index = text.indexOf(needle, from);
    assert.ok(index >= 0, `Could not find "${needle}" in ${doc.fileName}`);
    from = index + needle.length;
  }
  return doc.positionAt(index);
}

describe("Multi-file VS Code Flows", () => {
  before(async function () {
    this.timeout(integrationHookTimeoutMs);
    await configureServerForTests();
    getTestWorkspaceFolder();
    const doc = await vscode.workspace.openTextDocument(getFixturePath("def.sysml"));
    await waitForLanguageServerReady(doc);
  });

  after(async () => {
    await closeAllEditorsForTests();
  });

  it("goes from a usage to its cross-file definition", async () => {
    const useDoc = await vscode.workspace.openTextDocument(getFixturePath("use.sysml"));
    await vscode.window.showTextDocument(useDoc);

    const locations = await waitFor(
      "cross-file definition",
      () =>
        vscode.commands.executeCommand<vscode.Location[]>(
          "vscode.executeDefinitionProvider",
          useDoc.uri,
          findPosition(useDoc, "Spec42SmokeWidget")
        ),
      (value) => Array.isArray(value) && value.length > 0,
    );

    assert.ok(
      locations.some((location) => location.uri.fsPath.endsWith("def.sysml")),
      `Expected definition in def.sysml, got ${locations.map((location) => location.uri.fsPath).join(", ")}`
    );
  });

  it("finds references across files", async function () {
    this.timeout(20000);
    const useDoc = await vscode.workspace.openTextDocument(getFixturePath("use.sysml"));
    await vscode.window.showTextDocument(useDoc);

    const locations = await waitFor(
      "cross-file references",
      () =>
        vscode.commands.executeCommand<vscode.Location[]>(
          "vscode.executeReferenceProvider",
          useDoc.uri,
          findPosition(useDoc, "Spec42SmokeWidget")
        ),
      (value) => Array.isArray(value) && value.length >= 2,
    );

    const fsPaths = locations.map((location) => location.uri.fsPath);
    assert.ok(
      fsPaths.some((path) => path.endsWith("def.sysml")),
      `Expected references to include def.sysml, got ${fsPaths.join(", ")}`
    );
    assert.ok(
      fsPaths.some((path) => path.endsWith("use.sysml")),
      `Expected references to include use.sysml, got ${fsPaths.join(", ")}`
    );
  });

});

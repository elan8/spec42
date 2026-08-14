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
  waitForWorkspaceIndexReady,
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
    await waitForWorkspaceIndexReady(2);
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

  it("renames symbols across files", async function () {
    this.timeout(20000);
    const defDoc = await vscode.workspace.openTextDocument(getFixturePath("def.sysml"));
    await vscode.window.showTextDocument(defDoc);

    const workspaceEdit = await waitFor(
      "cross-file rename",
      async () => {
        try {
          return await vscode.commands.executeCommand<vscode.WorkspaceEdit>(
            "vscode.executeDocumentRenameProvider",
            defDoc.uri,
            findPosition(defDoc, "Spec42SmokeWidget"),
            "RenamedSmokeWidget"
          );
        } catch (error) {
          if (error instanceof Error && error.message === "The element can't be renamed.") {
            return undefined;
          }
          throw error;
        }
      },
      (value) => Boolean(value),
    );

    assert.ok(workspaceEdit, "Expected a workspace edit for rename");
    const entries = workspaceEdit.entries();
    assert.ok(entries.length >= 2, `Expected edits for at least two files, got ${entries.length}`);
    const fileNames = entries.map(([uri]) => uri.fsPath);
    assert.ok(
      fileNames.some((path) => path.endsWith("def.sysml")),
      `Expected rename to include def.sysml, got ${fileNames.join(", ")}`
    );
    assert.ok(
      fileNames.some((path) => path.endsWith("use.sysml")),
      `Expected rename to include use.sysml, got ${fileNames.join(", ")}`
    );
    for (const [, edits] of entries) {
      assert.ok(
        edits.some((edit) => edit.newText === "RenamedSmokeWidget"),
        "Expected rename edits to use the requested new symbol name"
      );
    }
  });
});

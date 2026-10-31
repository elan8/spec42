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

  it("registers type hierarchy and follows the referenced type across files", async () => {
    const useDoc = await vscode.workspace.openTextDocument(getFixturePath("use.sysml"));
    const items = await waitFor(
      "type hierarchy provider",
      () => vscode.commands.executeCommand<vscode.TypeHierarchyItem[]>(
        "vscode.prepareTypeHierarchy", useDoc.uri, findPosition(useDoc, "Spec42SmokeWidget")
      ),
      value => Array.isArray(value) && value.length > 0,
    );
    assert.strictEqual(items[0].name, "Spec42SmokeWidget");
    assert.ok(items[0].uri.fsPath.endsWith("def.sysml"));
    const subtypes = await vscode.commands.executeCommand<vscode.TypeHierarchyItem[]>(
      "vscode.provideSubtypes", items[0]
    );
    assert.ok(subtypes?.some(item => item.name === "widget"));
  });

  it("accepts an import completion and undoes the reference and import together", async function () {
    this.timeout(60000);
    const uri = vscode.Uri.joinPath(getTestWorkspaceFolder().uri, "import-completion.sysml");
    const original = "package CompletionClient {\n    part widget : Spec42SmokeWid;\n}\n";
    await vscode.workspace.fs.writeFile(uri, Buffer.from(original));
    const doc = await vscode.workspace.openTextDocument(uri);
    const editor = await vscode.window.showTextDocument(doc);
    try {
      assert.strictEqual(doc.languageId, "sysml");
      await waitForLanguageServerReady(doc);
      const at = doc.positionAt(original.indexOf("Spec42SmokeWid") + "Spec42SmokeWid".length);
      editor.selection = new vscode.Selection(at, at);
      await waitFor("resolved import completion", () =>
        vscode.commands.executeCommand<vscode.CompletionList>(
          "vscode.executeCompletionItemProvider", uri, at, undefined, 100
        ), list => Boolean(list?.items.some(item =>
          (typeof item.label === "string" ? item.label : item.label.label) === "Spec42SmokeWidget"
            && item.additionalTextEdits?.length
        )));
      await vscode.commands.executeCommand("editor.action.triggerSuggest");
      // The prefix has exactly one matching definition. Accept through the editor so its
      // completion transaction, including resolved additionalTextEdits, owns the undo stop.
      await waitFor("accepted completion and import", async () => {
        // triggerSuggest can return before the suggestion widget has populated. Once the
        // acceptance changes the text, never issue another accept command.
        if (doc.getText() === original) {
          await vscode.commands.executeCommand("acceptSelectedSuggestion");
        }
        return doc.getText();
      }, text =>
        text === "package CompletionClient {\n    private import Spec42MultiFileDefinitions::Spec42SmokeWidget;\n    part widget : Spec42SmokeWidget;\n}\n");
      await vscode.commands.executeCommand("undo");
      await waitFor("completion undone", async () => doc.getText(), text => text === original);
      assert.strictEqual(doc.getText(), original);
    } finally {
      await vscode.commands.executeCommand("workbench.action.revertAndCloseActiveEditor");
      await vscode.workspace.fs.delete(uri);
    }
  });

  it("applies all missing imports as one undoable workspace edit", async function () {
    this.timeout(60000);
    const uri = vscode.Uri.joinPath(getTestWorkspaceFolder().uri, "import-fix-all.sysml");
    const original = "package ImportClient {\n    part a : Spec42SmokeWidget;\n    part b : Spec42SmokeWidget;\n}\n";
    await vscode.workspace.fs.writeFile(uri, Buffer.from(original));
    const doc = await vscode.workspace.openTextDocument(uri);
    await vscode.window.showTextDocument(doc);
    try {
      assert.strictEqual(doc.languageId, "sysml");
      await waitForLanguageServerReady(doc);
      const actions = await waitFor("fix all missing imports", () =>
        vscode.commands.executeCommand<vscode.CodeAction[]>(
          "vscode.executeCodeActionProvider", uri, new vscode.Range(0, 0, 0, 0),
          "source.fixAll.spec42.imports", 100
        ), value => Boolean(value?.some(action => action.edit)));
      const action = actions.find(item => item.kind?.value === "source.fixAll.spec42.imports");
      assert.ok(action?.edit);
      assert.strictEqual(await vscode.workspace.applyEdit(action.edit), true);
      assert.strictEqual(doc.getText(),
        "package ImportClient {\n    private import Spec42MultiFileDefinitions::Spec42SmokeWidget;\n    part a : Spec42SmokeWidget;\n    part b : Spec42SmokeWidget;\n}\n");
      await vscode.commands.executeCommand("undo");
      await waitFor("import action undone", async () => doc.getText(), text => text === original);
      assert.strictEqual(doc.getText(), original);
    } finally {
      await vscode.commands.executeCommand("workbench.action.revertAndCloseActiveEditor");
      await vscode.workspace.fs.delete(uri);
    }
  });

});

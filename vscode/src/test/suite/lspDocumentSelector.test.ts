import * as assert from "assert";
import * as vscode from "vscode";
import { MODEL_DOCUMENT_SELECTOR } from "../../activation/lspDocumentSelector";

describe("workspace language client document selection", () => {
  const score = (scheme: string, languageId: string) => vscode.languages.match(
    MODEL_DOCUMENT_SELECTOR,
    { uri: vscode.Uri.from({ scheme, path: "/c:/workspace/Model.sysml" }), languageId } as vscode.TextDocument
  );

  for (const language of ["sysml", "kerml"]) {
    it(`selects ${language} files`, () => assert.ok(score("file", language) > 0));
    for (const scheme of ["chat-editing-text-model", "chat-editing-snapshot-text-model", "vscode-chat-code-block", "git", "untitled"]) {
      it(`excludes ${language} ${scheme} documents`, () => assert.strictEqual(score(scheme, language), 0));
    }
  }
  it("excludes other languages", () => assert.strictEqual(score("file", "typescript"), 0));
  it("excludes a Windows Copilot editing URI with revision query data", () => {
    const document = {
      uri: vscode.Uri.parse("chat-editing-text-model:/C%3A/Git/project/Model.sysml?session=test&revision=12"),
      languageId: "sysml",
    } as vscode.TextDocument;
    assert.ok(vscode.languages.match([{ language: "sysml" }], document) > 0);
    assert.strictEqual(vscode.languages.match(MODEL_DOCUMENT_SELECTOR, document), 0);
  });
});

import * as assert from "assert";
import * as vscode from "vscode";
import { LSPErrorCodes, ResponseError } from "vscode-languageclient/node";
import { analysisRequestMiddleware } from "../../activation/pendingAnalysis";

const pending = () => new ResponseError(LSPErrorCodes.ContentModified, "Analysis pending");

describe("pending analysis requests", () => {
  it("retries pending analysis before the SDK can convert it to an empty result", async () => {
    let calls = 0;
    const result = await analysisRequestMiddleware.sendRequest!("textDocument/hover", {}, undefined, async () => {
      if (++calls === 1) throw pending();
      return "current";
    });
    assert.strictEqual(result, "current");
    assert.strictEqual(calls, 2);
  });

  it("rejects a successful positional response after its document version changed", async () => {
    const doc = await vscode.workspace.openTextDocument({ language: "sysml", content: "part def A;" });
    let calls = 0;
    await assert.rejects(() => analysisRequestMiddleware.sendRequest!("textDocument/hover", { textDocument: { uri: doc.uri.toString() } }, undefined, async () => {
      ++calls;
      const edit = new vscode.WorkspaceEdit();
      edit.insert(doc.uri, new vscode.Position(0, 0), "// changed\n");
      assert.ok(await vscode.workspace.applyEdit(edit));
      return "obsolete";
    }), (error: unknown) => error instanceof ResponseError && error.code === LSPErrorCodes.ContentModified);
    assert.strictEqual(calls, 1);
  });

  it("stops a pending retry when its cancellation token is cancelled", async () => {
    const cancellation = new vscode.CancellationTokenSource();
    let calls = 0;
    try {
      await assert.rejects(() => analysisRequestMiddleware.sendRequest!("textDocument/hover", {}, cancellation.token, async () => {
        ++calls;
        cancellation.cancel();
        throw pending();
      }), (error: unknown) => error instanceof ResponseError && error.code === LSPErrorCodes.ContentModified);
      assert.strictEqual(calls, 1);
    } finally { cancellation.dispose(); }
  });

  it("does not blindly retry a custom view request carrying an older publication identity", async () => {
    let calls = 0;
    await assert.rejects(() => analysisRequestMiddleware.sendRequest!("sysml/diagram", { modelDigest: "previous" }, undefined, async () => {
      ++calls;
      throw pending();
    }), (error: unknown) => error instanceof ResponseError && error.code === LSPErrorCodes.ContentModified);
    assert.strictEqual(calls, 1);
  });
});

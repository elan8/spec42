import * as vscode from "vscode";
import { LSPErrorCodes, ResponseError, type Middleware } from "vscode-languageclient/node";

/** Retry pending analysis before the language client converts ContentModified to empty results.
 * Positional requests belong to one editor document version; never retry them onto later text.
 */
export const analysisRequestMiddleware: Middleware = {
  async sendRequest(type, param, token, next) {
    const method = typeof type === "string" ? type : type.method;
    if (!method.startsWith("textDocument/") && method !== "workspace/symbol") {
      return next(type, param, token);
    }
    const uri = (param as { textDocument?: { uri?: string } } | undefined)?.textDocument?.uri;
    const document = uri ? vscode.workspace.textDocuments.find(doc => doc.uri.toString() === uri) : undefined;
    const version = document?.version;
    const changed = () => token?.isCancellationRequested || (document !== undefined && (document.isClosed || document.version !== version));
    const deadline = Date.now() + 30_000;
    for (;;) {
      try {
        const result = await next(type, param, token);
        if (changed()) throw new ResponseError(LSPErrorCodes.ContentModified, "Document changed during analysis request");
        return result;
      } catch (error) {
        if (!(error instanceof ResponseError) || error.code !== LSPErrorCodes.ContentModified || changed() || Date.now() >= deadline) throw error;
        await new Promise<void>(resolve => setTimeout(resolve, 100));
        if (changed()) throw error;
      }
    }
  },
};

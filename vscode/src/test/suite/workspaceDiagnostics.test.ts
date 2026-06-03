import * as assert from "assert";
import * as vscode from "vscode";
import { summarizeSysmlDiagnosticEntries } from "../../diagnostics/workspaceDiagnostics";

function diag(
  severity: vscode.DiagnosticSeverity,
  message: string
): vscode.Diagnostic {
  return new vscode.Diagnostic(
    new vscode.Range(0, 0, 0, 1),
    message,
    severity
  );
}

describe("workspaceDiagnostics", () => {
  it("summarizes diagnostics for workspace sysml files only", () => {
    const workspaceRoot = "C:/project";
    const summary = summarizeSysmlDiagnosticEntries(
      [
        [vscode.Uri.file(`${workspaceRoot}/models/a.sysml`), [diag(vscode.DiagnosticSeverity.Error, "e1"), diag(vscode.DiagnosticSeverity.Warning, "w1")]],
        [vscode.Uri.file(`${workspaceRoot}/models/b.kerml`), [diag(vscode.DiagnosticSeverity.Warning, "w2")]],
        [vscode.Uri.file(`${workspaceRoot}/readme.md`), [diag(vscode.DiagnosticSeverity.Error, "ignored")]],
        [vscode.Uri.file("C:/stdlib/Types.sysml"), [diag(vscode.DiagnosticSeverity.Error, "lib")]],
      ],
      {
        workspaceRootPaths: [workspaceRoot],
        libraryRootPaths: ["C:/stdlib"],
      }
    );

    assert.strictEqual(summary.errors, 1);
    assert.strictEqual(summary.warnings, 2);
    assert.strictEqual(summary.totalFiles, 2);
    assert.strictEqual(summary.filesWithIssues, 2);
  });

  it("excludes files outside workspace roots", () => {
    const summary = summarizeSysmlDiagnosticEntries(
      [[vscode.Uri.file("C:/other/out.sysml"), [diag(vscode.DiagnosticSeverity.Error, "outside")]]],
      { workspaceRootPaths: ["C:/project"] }
    );

    assert.strictEqual(summary.totalFiles, 0);
    assert.strictEqual(summary.errors, 0);
  });
});

import { basename, dirname, join } from "node:path";
import { randomBytes } from "node:crypto";
import * as vscode from "vscode";

export interface Doc42Result {
  file: string;
  title: string;
  project: string;
}

export class Doc42Preview implements vscode.Disposable {
  private panel?: vscode.WebviewPanel;
  private file?: string;

  constructor(private readonly extensionUri: vscode.Uri) {}

  open(result: Doc42Result): void {
    if (this.panel && this.file === result.file) {
      this.panel.reveal();
      return;
    }
    this.dispose();
    const file = vscode.Uri.file(result.file);
    const assets = vscode.Uri.joinPath(this.extensionUri, "media", "pdfjs");
    const panel = vscode.window.createWebviewPanel("spec42.doc42Preview", result.title, vscode.ViewColumn.Active, {
      enableScripts: true,
      localResourceRoots: [assets, vscode.Uri.file(dirname(result.file))],
    });
    this.panel = panel;
    this.file = result.file;
    const messages = panel.webview.onDidReceiveMessage(async (message: unknown) => {
      try {
        if (message === "save") {
          const destination = await vscode.window.showSaveDialog({
            defaultUri: vscode.Uri.file(join(result.project, basename(result.file))),
            filters: { PDF: ["pdf"] },
          });
          if (destination) await vscode.workspace.fs.copy(file, destination, { overwrite: true });
        } else if (message === "open" && !await vscode.env.openExternal(file)) {
          throw new Error("No external PDF viewer could be opened. Use Save As to copy the file.");
        }
      } catch (error) {
        void vscode.window.showErrorMessage(`Doc42: ${error instanceof Error ? error.message : String(error)}`);
      }
    });
    panel.onDidDispose(() => {
      messages.dispose();
      this.panel = undefined;
      this.file = undefined;
    });

    const nonce = randomBytes(16).toString("hex");
    const assetUri = panel.webview.asWebviewUri(assets);
    const pdfUri = panel.webview.asWebviewUri(file);
    const source = panel.webview.cspSource;
    panel.webview.html = `<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'nonce-${nonce}' ${source}; style-src 'nonce-${nonce}'; connect-src ${source}; worker-src blob:; img-src blob: data:; font-src blob: data:;">
  <title>Doc42 PDF preview</title>
  <style nonce="${nonce}">
    body { margin: 0; }
    header { position: sticky; top: 0; z-index: 1; display: flex; gap: 8px; align-items: center; flex-wrap: wrap; padding: 8px 12px; background: var(--vscode-editor-background); border-bottom: 1px solid var(--vscode-panel-border); }
    button { padding: 4px 9px; color: var(--vscode-button-secondaryForeground); background: var(--vscode-button-secondaryBackground); border: 1px solid transparent; border-radius: 2px; cursor: pointer; font: inherit; }
    button:hover:enabled { background: var(--vscode-button-secondaryHoverBackground); }
    button:focus-visible { outline: 1px solid var(--vscode-focusBorder); }
    button:disabled { opacity: .5; cursor: default; }
    #status { flex: 1; }
    main { padding: 12px; }
    canvas:not([hidden]) { display: block; margin: 0 auto; max-width: 100%; height: auto; background: white; }
  </style>
</head>
<body data-pdf="${pdfUri}">
  <header aria-label="PDF controls">
    <button id="previous" disabled>Previous</button>
    <button id="next" disabled>Next</button>
    <span id="status" role="status">Loading PDF…</span>
    <button id="save">Save As…</button>
    <button id="open">Open Externally</button>
  </header>
  <main><canvas id="page" aria-label="Rendered PDF page" hidden></canvas></main>
  <script type="module" nonce="${nonce}" src="${assetUri}/preview.mjs"></script>
</body>
</html>`;
  }

  dispose(): void {
    this.panel?.dispose();
  }

  invalidate(files: string[]): void {
    if (this.file && files.includes(this.file)) this.dispose();
  }
}

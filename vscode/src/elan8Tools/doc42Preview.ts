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
    body { margin: 0; height: 100vh; display: flex; flex-direction: column; overflow: hidden; }
    header { display: flex; gap: 8px; align-items: center; flex-wrap: wrap; padding: 8px 12px; background: var(--vscode-editor-background); border-bottom: 1px solid var(--vscode-panel-border); }
    .zoom-controls { display: flex; gap: 4px; align-items: center; }
    button { padding: 4px 9px; color: var(--vscode-button-secondaryForeground); background: var(--vscode-button-secondaryBackground); border: 1px solid transparent; border-radius: 2px; cursor: pointer; font: inherit; }
    button:hover:enabled { background: var(--vscode-button-secondaryHoverBackground); }
    button:focus-visible { outline: 1px solid var(--vscode-focusBorder); }
    button:disabled { opacity: .5; cursor: default; }
    button[aria-pressed="true"] { border-color: var(--vscode-focusBorder); }
    #status { flex: 1; }
    #zoom-level { min-width: 4em; text-align: center; }
    #viewport { flex: 1; min-height: 0; position: relative; overflow: auto; overflow-anchor: none; scrollbar-gutter: stable; background: var(--vscode-editorWidget-background); }
    #viewport:focus-visible { outline: 1px solid var(--vscode-focusBorder); outline-offset: -1px; }
    #pages { box-sizing: border-box; min-width: 100%; width: max-content; padding: 12px; display: flex; flex-direction: column; gap: 16px; align-items: center; }
    .pdf-page { position: relative; display: grid; place-items: center; overflow: hidden; background: white; color: #333; box-shadow: 0 1px 6px #0005; }
    .pdf-page canvas { position: absolute; inset: 0; display: block; width: 100%; height: 100%; }
    .pdf-page p { margin: 12px; text-align: center; }
  </style>
</head>
<body data-pdf="${pdfUri}">
  <header aria-label="PDF controls">
    <div class="zoom-controls" role="group" aria-label="Zoom controls">
      <button id="fit-page" aria-pressed="true" disabled>Fit Page</button>
      <button id="zoom-out" aria-label="Zoom out" disabled>−</button>
      <span id="zoom-level" aria-label="Zoom level">—</span>
      <button id="zoom-in" aria-label="Zoom in" disabled>+</button>
    </div>
    <span id="status" role="status">Loading PDF…</span>
    <button id="save">Save As…</button>
    <button id="open">Open Externally</button>
  </header>
  <main id="viewport" aria-label="PDF pages" tabindex="0"><div id="pages"></div></main>
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

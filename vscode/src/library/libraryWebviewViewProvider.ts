import * as vscode from "vscode";
import type {
  LibrarySearchItem,
  LspModelProvider,
  SysMLLibrarySearchResult,
} from "../providers/lspModelProvider";

type StandardLibraryStatus = {
  enabled: boolean;
  pinnedVersion: string;
  installedVersion?: string;
  isInstalled: boolean;
};

type OpenRangeMessage = {
  uri: string;
  range: {
    start: { line: number; character: number };
    end: { line: number; character: number };
  };
};

export class LibraryWebviewViewProvider implements vscode.WebviewViewProvider {
  private view: vscode.WebviewView | undefined;

  constructor(
    private readonly extensionUri: vscode.Uri,
    private readonly lspModelProvider: LspModelProvider,
    private readonly getStandardLibraryStatus: () => StandardLibraryStatus
  ) {}

  resolveWebviewView(webviewView: vscode.WebviewView): void | Thenable<void> {
    this.view = webviewView;
    const webview = webviewView.webview;
    webview.options = {
      enableScripts: true,
    };
    webview.html = this.getHtml(webview);

    webview.onDidReceiveMessage(async (message) => {
      if (message?.type === "initLoad") {
        this.post({ type: "searching" });
        try {
          const result = await this.lspModelProvider.searchLibraries("", 500);
          this.post({ type: "allItems", payload: result });
        } catch (error) {
          this.post({
            type: "error",
            payload: error instanceof Error ? error.message : String(error),
          });
        }
        return;
      }

      if (message?.type === "search") {
        const query = String(message.query ?? "").trim();
        if (!query) {
          const result = await this.lspModelProvider.searchLibraries("", 500);
          this.post({ type: "allItems", payload: result });
          return;
        }
        this.post({ type: "searching" });
        try {
          const result = await this.lspModelProvider.searchLibraries(query, 100);
          this.post({ type: "results", payload: result });
        } catch (error) {
          this.post({
            type: "error",
            payload: error instanceof Error ? error.message : String(error),
          });
        }
        return;
      }

      if (message?.type === "openResult") {
        await this.openResult(message.payload as OpenRangeMessage);
        return;
      }

      if (message?.type === "installStdLib") {
        await vscode.commands.executeCommand("sysml.library.installStdLib");
        this.refresh();
        return;
      }

      if (message?.type === "manageCustomLibraries") {
        await vscode.commands.executeCommand("sysml.library.managePaths");
        return;
      }

      if (message?.type === "showStdLibStatus") {
        await vscode.commands.executeCommand("sysml.library.showStdLibStatus");
      }
    });

    this.postStatus();
  }

  refresh(): void {
    this.postStatus();
  }

  private async openResult(message: OpenRangeMessage): Promise<void> {
    if (!message?.uri || !message.range) {
      return;
    }
    const uri = vscode.Uri.parse(message.uri);
    const doc = await vscode.workspace.openTextDocument(uri);
    const editor = await vscode.window.showTextDocument(doc, {
      preserveFocus: false,
      preview: true,
    });
    const range = new vscode.Range(
      new vscode.Position(message.range.start.line, message.range.start.character),
      new vscode.Position(message.range.end.line, message.range.end.character)
    );
    editor.selection = new vscode.Selection(range.start, range.start);
    editor.revealRange(range, vscode.TextEditorRevealType.InCenter);
  }

  private postStatus(): void {
    const status = this.getStandardLibraryStatus();
    this.post({ type: "status", payload: status });
  }

  private post(message: unknown): void {
    this.view?.webview.postMessage(message);
  }

  private getHtml(webview: vscode.Webview): string {
    const nonce = getNonce();
    const codiconsCss = webview
      .asWebviewUri(
        vscode.Uri.joinPath(this.extensionUri, "media", "codicons", "codicon.css")
      )
      .toString();
    return `<!doctype html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${webview.cspSource} 'unsafe-inline'; script-src 'nonce-${nonce}';">
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <link nonce="${nonce}" rel="stylesheet" href="${codiconsCss}">
  <style>
    body { font-family: var(--vscode-font-family); color: var(--vscode-foreground); padding: 8px; }
    .row { display: flex; gap: 6px; margin-bottom: 8px; }
    input { width: 100%; padding: 6px; border: 1px solid var(--vscode-input-border); background: var(--vscode-input-background); color: var(--vscode-input-foreground); }
    button { border: 1px solid var(--vscode-button-border); background: var(--vscode-button-background); color: var(--vscode-button-foreground); padding: 4px 8px; cursor: pointer; }
    button.secondary { background: var(--vscode-button-secondaryBackground); color: var(--vscode-button-secondaryForeground); }
    .status { font-size: 12px; opacity: 0.9; margin-bottom: 8px; }
    .result { border-top: 1px solid var(--vscode-panel-border); padding: 8px 2px; cursor: pointer; }
    .title { font-weight: 600; }
    .meta { font-size: 12px; opacity: 0.9; margin-top: 4px; display: flex; gap: 8px; flex-wrap: wrap; }
    .muted { opacity: 0.8; font-size: 12px; }
  </style>
</head>
<body>
  <div class="row">
    <input id="query" type="text" placeholder="Search standard + custom libraries..." />
  </div>
  <div class="row">
    <button id="install">Install/Update Standard</button>
    <button id="custom" class="secondary">Custom Libraries</button>
    <button id="status" class="secondary">Status</button>
  </div>
  <div id="stdlibStatus" class="status">Standard library status: loading...</div>
  <div id="state" class="muted">Loading libraries...</div>
  <div id="results"></div>

  <script nonce="${nonce}">
    const vscode = acquireVsCodeApi();
    const query = document.getElementById('query');
    const state = document.getElementById('state');
    const results = document.getElementById('results');
    const stdlibStatus = document.getElementById('stdlibStatus');
    let timer = null;
    let allItems = [];
    let renderedItems = [];

    function groupAndRender(items, total, queryText) {
      if (!items || items.length === 0) {
        state.textContent = queryText
          ? 'No results for "' + queryText + '".'
          : 'No library symbols indexed yet. Install/Update Standard Library and restart the SysML server.';
        results.innerHTML = '';
        return;
      }
      state.textContent = queryText
        ? ('Filtered to ' + items.length + ' of ' + total + ' symbol(s).')
        : ('Showing ' + items.length + ' library symbol(s).');

      const bySource = new Map();
      for (const item of items) {
        const source = item.source || 'custom';
        if (!bySource.has(source)) bySource.set(source, new Map());
        const byContainer = bySource.get(source);
        const container = item.container || '(top level)';
        if (!byContainer.has(container)) byContainer.set(container, []);
        byContainer.get(container).push(item);
      }

      let idx = 0;
      renderedItems = [];
      const blocks = [];
      for (const [source, containers] of bySource.entries()) {
        const sourceLabel = source === 'standard' ? 'Standard Library' : 'Custom Libraries';
        let sourceHtml = '<details><summary class="title">' + escapeHtml(sourceLabel) + '</summary>';
        for (const [container, entries] of containers.entries()) {
          sourceHtml += '<details style="margin-left:8px"><summary class="muted">' + escapeHtml(container) + ' (' + entries.length + ')</summary>';
          for (const item of entries) {
            renderedItems.push(item);
            sourceHtml += '<div class="result" data-index="' + idx + '">' +
              '<div class="title">' + escapeHtml(item.name) + '</div>' +
              '<div class="meta"><span>' + escapeHtml(item.kind) + '</span></div>' +
            '</div>';
            idx += 1;
          }
          sourceHtml += '</details>';
        }
        sourceHtml += '</details>';
        blocks.push(sourceHtml);
      }
      results.innerHTML = blocks.join('');

      results.querySelectorAll('.result').forEach((el) => {
        el.addEventListener('click', () => {
          const selectedIdx = Number(el.getAttribute('data-index'));
          const item = renderedItems[selectedIdx];
          if (!item) return;
          vscode.postMessage({ type: 'openResult', payload: { uri: item.uri, range: item.range } });
        });
      });
    }

    function escapeHtml(str) {
      return String(str || '')
        .replaceAll('&', '&amp;')
        .replaceAll('<', '&lt;')
        .replaceAll('>', '&gt;');
    }

    query.addEventListener('input', () => {
      const value = query.value.trim();
      clearTimeout(timer);
      timer = setTimeout(() => {
        if (!value) {
          groupAndRender(allItems, allItems.length, '');
        } else {
          const filtered = allItems.filter((item) => {
            const hay = (item.name + ' ' + (item.container || '') + ' ' + (item.path || '')).toLowerCase();
            return hay.includes(value.toLowerCase());
          });
          groupAndRender(filtered, allItems.length, value);
        }
      }, 250);
    });

    document.getElementById('install').addEventListener('click', () => vscode.postMessage({ type: 'installStdLib' }));
    document.getElementById('custom').addEventListener('click', () => vscode.postMessage({ type: 'manageCustomLibraries' }));
    document.getElementById('status').addEventListener('click', () => vscode.postMessage({ type: 'showStdLibStatus' }));

    window.addEventListener('message', (event) => {
      const message = event.data;
      if (message.type === 'searching') {
        state.textContent = 'Searching...';
        return;
      }
      if (message.type === 'results') {
        allItems = message.payload.items || [];
        groupAndRender(allItems, message.payload.total || allItems.length, query.value.trim());
        return;
      }
      if (message.type === 'allItems') {
        allItems = message.payload.items || [];
        groupAndRender(allItems, message.payload.total || allItems.length, query.value.trim());
        return;
      }
      if (message.type === 'error') {
        state.textContent = 'Search failed: ' + message.payload;
        return;
      }
      if (message.type === 'status') {
        const s = message.payload;
        const text = s.enabled
          ? (s.isInstalled
            ? 'Standard library installed (' + (s.installedVersion || s.pinnedVersion) + ')'
            : 'Standard library not installed (pinned ' + s.pinnedVersion + ')')
          : 'Managed standard library is disabled';
        stdlibStatus.textContent = text;
      }
    });

    vscode.postMessage({ type: 'initLoad' });
  </script>
</body>
</html>`;
  }
}

function getNonce(): string {
  const chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  let out = "";
  for (let i = 0; i < 32; i += 1) {
    out += chars.charAt(Math.floor(Math.random() * chars.length));
  }
  return out;
}

export type LibrarySearchResultMessage = {
  items: LibrarySearchItem[];
  total: number;
};

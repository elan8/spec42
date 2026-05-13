import * as vscode from "vscode";
import type { LspModelProvider } from "../providers/lspModelProvider";

type StdlibHeading = {
  pinnedVersion: string;
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
    private readonly getStdlibHeading: () => StdlibHeading
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

      if (message?.type === "manageCustomLibraries") {
        await vscode.commands.executeCommand("sysml.library.managePaths");
        return;
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
    const heading = this.getStdlibHeading();
    this.post({ type: "status", payload: heading });
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
  <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${webview.cspSource} 'unsafe-inline'; font-src ${webview.cspSource}; script-src 'nonce-${nonce}';">
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <link nonce="${nonce}" rel="stylesheet" href="${codiconsCss}">
  <style>
    body { font-family: var(--vscode-font-family); color: var(--vscode-foreground); padding: 8px; }
    .row { display: flex; gap: 6px; margin-bottom: 8px; }
    .stdlib-row { display: flex; align-items: center; justify-content: space-between; margin-bottom: 6px; padding: 4px 0; }
    .stdlib-title { font-weight: 600; font-size: 12px; letter-spacing: 0.2px; }
    .stdlib-version { font-weight: 400; opacity: 0.85; margin-left: 4px; }
    .actions { display: flex; gap: 6px; }
    .icon-btn { border: 1px solid var(--vscode-button-border, var(--vscode-panel-border)); background: transparent; color: var(--vscode-foreground); border-radius: 4px; width: 24px; height: 24px; cursor: pointer; display: inline-flex; align-items: center; justify-content: center; }
    .icon-btn:hover { background: var(--vscode-toolbar-hoverBackground); }
    .icon-btn:disabled { opacity: 0.5; cursor: default; }
    .custom-packages { margin: 0 0 8px 0; padding-left: 2px; }
    .custom-packages details { margin: 0; }
    .custom-packages summary { cursor: pointer; font-size: 12px; opacity: 0.9; }
    .custom-packages-list { margin: 6px 0 0 14px; padding: 0; }
    .custom-packages-list li { list-style: disc; margin-bottom: 2px; font-size: 12px; opacity: 0.9; }
    .custom-package-btn { border: none; background: transparent; color: inherit; cursor: pointer; padding: 0; font: inherit; text-align: left; }
    .custom-package-btn:hover { text-decoration: underline; }
    .custom-filter-row { margin: 4px 0 0 14px; font-size: 12px; }
    .custom-filter-clear { border: none; background: transparent; color: var(--vscode-textLink-foreground); cursor: pointer; padding: 0; font: inherit; }
    .custom-filter-clear:hover { text-decoration: underline; }
    input { width: 100%; padding: 6px; border: 1px solid var(--vscode-input-border); background: var(--vscode-input-background); color: var(--vscode-input-foreground); }
    .result { border-top: 1px solid var(--vscode-panel-border); padding: 8px 2px; cursor: pointer; }
    .title { font-weight: 600; }
    .meta { font-size: 12px; opacity: 0.9; margin-top: 4px; display: flex; gap: 8px; flex-wrap: wrap; }
    .muted { opacity: 0.8; font-size: 12px; }
  </style>
</head>
<body>
  <div class="stdlib-row">
    <div id="stdlibTitle" class="stdlib-title" title="Bundled with the Spec42 language server">Standard library<span id="stdlibVersion" class="stdlib-version"></span></div>
  </div>
  <div class="stdlib-row">
    <div class="stdlib-title" title="Custom libraries">Custom libraries</div>
    <div class="actions">
      <button id="btnManageCustomLibraries" class="icon-btn" title="Manage custom library paths">
        <span class="codicon codicon-settings-gear"></span>
      </button>
    </div>
  </div>
  <div id="customPackages" class="custom-packages"></div>
  <div class="row">
    <input id="query" type="text" placeholder="Search standard + custom libraries..." />
  </div>
  <div id="state" class="muted">Loading libraries...</div>
  <div id="results"></div>

  <script nonce="${nonce}">
    const vscode = acquireVsCodeApi();
    const query = document.getElementById('query');
    const state = document.getElementById('state');
    const results = document.getElementById('results');
    const stdlibTitle = document.getElementById('stdlibTitle');
    const stdlibVersion = document.getElementById('stdlibVersion');
    const btnManageCustomLibraries = document.getElementById('btnManageCustomLibraries');
    const customPackages = document.getElementById('customPackages');
    let timer = null;
    let latestTree = { sources: [], symbolTotal: 0, total: 0 };
    let renderedItems = [];
    let selectedCustomPackage = '';

    function renderTree(tree, queryText) {
      const sources = Array.isArray(tree?.sources) ? tree.sources : [];
      const filteredSources = sources.map((src) => {
        const sourceName = src?.source || 'custom';
        const packages = Array.isArray(src?.packages) ? src.packages : [];
        const nextPackages = sourceName === 'custom' && selectedCustomPackage
          ? packages.filter((pkg) => String(pkg?.name || '') === selectedCustomPackage)
          : packages;
        return { ...src, packages: nextPackages };
      });
      const visibleCount = filteredSources
        .reduce((acc, src) => acc + (Array.isArray(src.packages)
          ? src.packages.reduce((a, p) => a + (Array.isArray(p.symbols) ? p.symbols.length : 0), 0)
          : 0), 0);
      if (visibleCount === 0) {
        state.textContent = queryText
          ? 'No results for "' + queryText + '".'
          : 'No library symbols indexed yet. Restart the SysML server or add library paths under spec42.libraryPaths.';
        results.replaceChildren();
        return;
      }
      state.textContent = queryText
        ? ('Filtered to ' + visibleCount + ' of ' + (tree.total || visibleCount) + ' symbol(s).')
        : ('Showing ' + visibleCount + ' library symbol(s).');

      let idx = 0;
      renderedItems = [];
      const resultNodes = [];
      for (const sourceNode of filteredSources) {
        const source = sourceNode.source || 'custom';
        const sourceLabel = source === 'standard' ? 'Standard Library' : 'Custom Libraries';
        const sourceDetails = document.createElement('details');
        const sourceSummary = document.createElement('summary');
        sourceSummary.className = 'title';
        sourceSummary.textContent = sourceLabel;
        sourceDetails.appendChild(sourceSummary);
        const packages = Array.isArray(sourceNode.packages) ? sourceNode.packages : [];
        for (const pkg of packages) {
          const symbols = Array.isArray(pkg.symbols) ? pkg.symbols : [];
          const packageDetails = document.createElement('details');
          packageDetails.style.marginLeft = '8px';
          const packageSummary = document.createElement('summary');
          packageSummary.className = 'muted';
          packageSummary.textContent = String(pkg.name || '(unknown package)') + ' (' + symbols.length + ')';
          packageDetails.appendChild(packageSummary);
          for (const item of symbols) {
            renderedItems.push(item);
            const result = document.createElement('div');
            result.className = 'result';
            result.dataset.index = String(idx);
            const title = document.createElement('div');
            title.className = 'title';
            title.textContent = String(item.name || '');
            const meta = document.createElement('div');
            meta.className = 'meta';
            const kind = document.createElement('span');
            kind.textContent = String(item.kind || '');
            meta.appendChild(kind);
            result.appendChild(title);
            result.appendChild(meta);
            packageDetails.appendChild(result);
            idx += 1;
          }
          sourceDetails.appendChild(packageDetails);
        }
        resultNodes.push(sourceDetails);
      }
      results.replaceChildren(...resultNodes);

      results.querySelectorAll('.result').forEach((el) => {
        el.addEventListener('click', () => {
          const selectedIdx = Number(el.getAttribute('data-index'));
          const item = renderedItems[selectedIdx];
          if (!item) return;
          vscode.postMessage({ type: 'openResult', payload: { uri: item.uri, range: item.range } });
        });
      });
    }

    function renderStdlibHeading(heading) {
      const v = heading?.pinnedVersion || 'unknown';
      if (stdlibVersion) {
        stdlibVersion.textContent = '(' + v + ')';
      }
      if (stdlibTitle) {
        stdlibTitle.title = 'Bundled SysML standard library (release ' + v + ')';
      }
    }

    function renderCustomPackages(tree) {
      const sources = Array.isArray(tree?.sources) ? tree.sources : [];
      const customSource = sources.find((src) => (src?.source || 'custom') === 'custom');
      const packages = Array.isArray(customSource?.packages) ? customSource.packages : [];

      if (!customPackages) {
        return;
      }
      if (packages.length === 0) {
        const empty = document.createElement('div');
        empty.className = 'muted';
        empty.textContent = 'No custom library packages loaded.';
        customPackages.replaceChildren(empty);
        return;
      }

      const details = document.createElement('details');
      const summary = document.createElement('summary');
      summary.textContent = 'Loaded packages (' + packages.length + ')';
      details.appendChild(summary);

      const list = document.createElement('ul');
      list.className = 'custom-packages-list';
      packages.forEach((pkg, index) => {
        const item = document.createElement('li');
        const button = document.createElement('button');
        button.className = 'custom-package-btn';
        button.textContent = String(pkg?.name || '(unknown package)') + ' (' + (Array.isArray(pkg?.symbols) ? pkg.symbols.length : 0) + ')';
        button.addEventListener('click', () => {
          const selectedPkg = packages[index];
          selectedCustomPackage = String(selectedPkg?.name || '');
          renderCustomPackages(latestTree);
          renderTree(latestTree, query.value.trim());
        });
        item.appendChild(button);
        list.appendChild(item);
      });
      details.appendChild(list);

      if (selectedCustomPackage) {
        const filterRow = document.createElement('div');
        filterRow.className = 'custom-filter-row';
        filterRow.appendChild(document.createTextNode('Filtered package: '));
        const selected = document.createElement('strong');
        selected.textContent = selectedCustomPackage;
        filterRow.appendChild(selected);
        filterRow.appendChild(document.createTextNode(' '));

        const clearBtn = document.createElement('button');
        clearBtn.className = 'custom-filter-clear';
        clearBtn.textContent = 'Clear';
        clearBtn.addEventListener('click', () => {
          selectedCustomPackage = '';
          renderCustomPackages(latestTree);
          renderTree(latestTree, query.value.trim());
        });
        filterRow.appendChild(clearBtn);
        details.appendChild(filterRow);
      }

      customPackages.replaceChildren(details);
    }

    window.addEventListener('message', (event) => {
      const msg = event.data;
      if (msg?.type === 'status') {
        renderStdlibHeading(msg.payload || {});
        return;
      }
      if (msg?.type === 'searching') {
        state.textContent = 'Searching...';
        return;
      }
      if (msg?.type === 'error') {
        state.textContent = 'Error: ' + (msg.payload || 'unknown');
        results.replaceChildren();
        return;
      }
      if (msg?.type === 'allItems' || msg?.type === 'results') {
        latestTree = msg.payload || latestTree;
        renderCustomPackages(latestTree);
        renderTree(latestTree, query.value.trim());
      }
    });

    query.addEventListener('input', () => {
      if (timer) clearTimeout(timer);
      timer = setTimeout(() => {
        const q = query.value.trim();
        vscode.postMessage({ type: 'search', query: q });
      }, 200);
    });

    btnManageCustomLibraries.addEventListener('click', () => {
      vscode.postMessage({ type: 'manageCustomLibraries' });
    });

    vscode.postMessage({ type: 'initLoad' });
  </script>
</body>
</html>`;
  }
}

function getNonce(): string {
  let text = "";
  const possible = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  for (let i = 0; i < 32; i++) {
    text += possible.charAt(Math.floor(Math.random() * possible.length));
  }
  return text;
}

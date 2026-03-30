import * as vscode from "vscode";
import type {
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

      if (message?.type === "removeStdLib") {
        await vscode.commands.executeCommand("sysml.library.removeStdLib");
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
  <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${webview.cspSource} 'unsafe-inline'; font-src ${webview.cspSource}; script-src 'nonce-${nonce}';">
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <link nonce="${nonce}" rel="stylesheet" href="${codiconsCss}">
  <style>
    body { font-family: var(--vscode-font-family); color: var(--vscode-foreground); padding: 8px; }
    .row { display: flex; gap: 6px; margin-bottom: 8px; }
    .stdlib-row { display: flex; align-items: center; justify-content: space-between; margin-bottom: 6px; padding: 4px 0; }
    .stdlib-title { font-weight: 600; font-size: 12px; letter-spacing: 0.2px; cursor: help; }
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
    <div id="stdlibTitle" class="stdlib-title" title="Standard library">Standard library</div>
    <div id="stdlibActions" class="actions"></div>
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
    const stdlibActions = document.getElementById('stdlibActions');
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
          : 'No library symbols indexed yet. Install/Update Standard Library and restart the SysML server.';
        results.innerHTML = '';
        return;
      }
      state.textContent = queryText
        ? ('Filtered to ' + visibleCount + ' of ' + (tree.total || visibleCount) + ' symbol(s).')
        : ('Showing ' + visibleCount + ' library symbol(s).');

      let idx = 0;
      renderedItems = [];
      const blocks = [];
      for (const sourceNode of filteredSources) {
        const source = sourceNode.source || 'custom';
        const sourceLabel = source === 'standard' ? 'Standard Library' : 'Custom Libraries';
        let sourceHtml = '<details><summary class="title">' + escapeHtml(sourceLabel) + '</summary>';
        const packages = Array.isArray(sourceNode.packages) ? sourceNode.packages : [];
        for (const pkg of packages) {
          const symbols = Array.isArray(pkg.symbols) ? pkg.symbols : [];
          sourceHtml += '<details style="margin-left:8px"><summary class="muted">' + escapeHtml(pkg.name || '(unknown package)') + ' (' + symbols.length + ')</summary>';
          for (const item of symbols) {
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

    function renderStdlibActions(status) {
      const enabled = !!status?.enabled;
      const isInstalled = !!status?.isInstalled;
      const installedVersion = status?.installedVersion || status?.pinnedVersion || 'unknown';
      stdlibTitle.title = isInstalled
        ? ('Installed version: ' + installedVersion)
        : ('Pinned version: ' + (status?.pinnedVersion || 'unknown'));

      const buttons = [];
      if (isInstalled) {
        buttons.push(
          '<button id="btnUpdateStdlib" class="icon-btn" title="Update standard library"' + (enabled ? '' : ' disabled') + '>' +
          '<span class="codicon codicon-cloud-download"></span></button>'
        );
        buttons.push(
          '<button id="btnRemoveStdlib" class="icon-btn" title="Remove standard library"' + (enabled ? '' : ' disabled') + '>' +
          '<span class="codicon codicon-trash"></span></button>'
        );
      } else {
        buttons.push(
          '<button id="btnAddStdlib" class="icon-btn" title="Add standard library"' + (enabled ? '' : ' disabled') + '>' +
          '<span class="codicon codicon-add"></span></button>'
        );
      }
      stdlibActions.innerHTML = buttons.join('');

      const btnAdd = document.getElementById('btnAddStdlib');
      if (btnAdd) {
        btnAdd.addEventListener('click', () => vscode.postMessage({ type: 'installStdLib' }));
      }
      const btnUpdate = document.getElementById('btnUpdateStdlib');
      if (btnUpdate) {
        btnUpdate.addEventListener('click', () => vscode.postMessage({ type: 'installStdLib' }));
      }
      const btnRemove = document.getElementById('btnRemoveStdlib');
      if (btnRemove) {
        btnRemove.addEventListener('click', () => vscode.postMessage({ type: 'removeStdLib' }));
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
        customPackages.innerHTML = '<div class="muted">No custom library packages loaded.</div>';
        return;
      }

      const packageItems = packages
        .map((pkg, index) => {
          const packageName = escapeHtml(pkg?.name || '(unknown package)');
          const symbolCount = Array.isArray(pkg?.symbols) ? pkg.symbols.length : 0;
          return '<li><button class="custom-package-btn" data-package-index="' + index + '">' + packageName + ' (' + symbolCount + ')</button></li>';
        })
        .join('');

      const selectedFilterHtml = selectedCustomPackage
        ? '<div class="custom-filter-row">Filtered package: <strong>' + escapeHtml(selectedCustomPackage) + '</strong> <button id="btnClearCustomFilter" class="custom-filter-clear">Clear</button></div>'
        : '';

      customPackages.innerHTML =
        '<details>' +
        '<summary>Loaded packages (' + packages.length + ')</summary>' +
        '<ul class="custom-packages-list">' + packageItems + '</ul>' +
        selectedFilterHtml +
        '</details>';

      const packageButtons = customPackages.querySelectorAll('.custom-package-btn');
      packageButtons.forEach((buttonEl) => {
        buttonEl.addEventListener('click', () => {
          const rawIndex = Number(buttonEl.getAttribute('data-package-index'));
          const selectedPkg = packages[rawIndex];
          selectedCustomPackage = String(selectedPkg?.name || '');
          renderCustomPackages(latestTree);
          renderTree(latestTree, query.value.trim());
        });
      });

      const clearFilterButton = document.getElementById('btnClearCustomFilter');
      if (clearFilterButton) {
        clearFilterButton.addEventListener('click', () => {
          selectedCustomPackage = '';
          renderCustomPackages(latestTree);
          renderTree(latestTree, query.value.trim());
        });
      }
    }

    query.addEventListener('input', () => {
      const value = query.value.trim();
      clearTimeout(timer);
      timer = setTimeout(() => {
        vscode.postMessage({ type: 'search', query: value });
      }, 250);
    });

    if (btnManageCustomLibraries) {
      btnManageCustomLibraries.addEventListener('click', () => {
        vscode.postMessage({ type: 'manageCustomLibraries' });
      });
    }

    window.addEventListener('message', (event) => {
      const message = event.data;
      if (message.type === 'searching') {
        state.textContent = 'Searching...';
        return;
      }
      if (message.type === 'results') {
        latestTree = message.payload || { sources: [], symbolTotal: 0, total: 0 };
        renderCustomPackages(latestTree);
        renderTree(latestTree, query.value.trim());
        return;
      }
      if (message.type === 'allItems') {
        latestTree = message.payload || { sources: [], symbolTotal: 0, total: 0 };
        renderCustomPackages(latestTree);
        renderTree(latestTree, query.value.trim());
        return;
      }
      if (message.type === 'error') {
        state.textContent = 'Search failed: ' + message.payload;
        return;
      }
      if (message.type === 'status') {
        const s = message.payload;
        renderStdlibActions(s);
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
  sources: SysMLLibrarySearchResult["sources"];
  symbolTotal: number;
  total: number;
};

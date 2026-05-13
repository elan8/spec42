import * as vscode from 'vscode';
import type { AddonState } from './registry';

type AddonsMessage =
    | { type: 'initLoad' }
    | { type: 'toggleAddon'; addonId: string; enabled: boolean }
    | { type: 'runAnalysis'; addonId: string }
    | { type: 'openAddon'; addonId: string };

export class AddonsWebviewViewProvider implements vscode.WebviewViewProvider {
    private view: vscode.WebviewView | undefined;

    constructor(
        private readonly extensionUri: vscode.Uri,
        private readonly getAddons: () => Promise<AddonState[]>,
        private readonly onToggleAddon: (addonId: string, enabled: boolean) => Promise<void>,
        private readonly onRunAnalysis: (addonId: string) => Promise<void>,
        private readonly onOpenAddon: (addonId: string) => Promise<void>,
    ) {}

    resolveWebviewView(webviewView: vscode.WebviewView): void | Thenable<void> {
        this.view = webviewView;
        const webview = webviewView.webview;
        webview.options = { enableScripts: true };
        webview.html = this.getHtml(webview);
        webview.onDidReceiveMessage(async (message: AddonsMessage) => {
            if (message.type === 'initLoad') {
                await this.refresh();
                return;
            }
            if (message.type === 'toggleAddon') {
                await this.onToggleAddon(message.addonId, !!message.enabled);
                await this.refresh();
                return;
            }
            if (message.type === 'runAnalysis') {
                await this.onRunAnalysis(message.addonId);
                await this.refresh();
                return;
            }
            if (message.type === 'openAddon') {
                await this.onOpenAddon(message.addonId);
                return;
            }
        });
    }

    async refresh(): Promise<void> {
        const addons = await this.getAddons();
        this.view?.webview.postMessage({ type: 'addons', payload: addons });
    }

    private getHtml(webview: vscode.Webview): string {
        const nonce = getNonce();
        return `<!doctype html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${webview.cspSource} 'unsafe-inline'; script-src 'nonce-${nonce}';">
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <style>
    body { font-family: var(--vscode-font-family); color: var(--vscode-foreground); padding: 8px; }
    .intro { font-size: 12px; opacity: 0.86; margin-bottom: 10px; line-height: 1.45; }
    .addon-card { border: 1px solid var(--vscode-panel-border); border-radius: 8px; padding: 10px; margin-bottom: 10px; background: var(--vscode-editor-background); }
    .addon-header { display: flex; justify-content: space-between; gap: 8px; align-items: center; margin-bottom: 6px; }
    .addon-title { font-weight: 600; }
    .badge { font-size: 11px; padding: 2px 6px; border-radius: 999px; background: var(--vscode-badge-background); color: var(--vscode-badge-foreground); }
    .addon-description { font-size: 12px; opacity: 0.92; line-height: 1.45; margin-bottom: 8px; }
    .addon-status { font-size: 12px; opacity: 0.8; margin-bottom: 8px; }
    .addon-actions { display: flex; flex-direction: column; gap: 8px; align-items: stretch; }
    button { border: 1px solid var(--vscode-button-border, transparent); border-radius: 6px; padding: 6px 10px; cursor: pointer; }
    .primary { background: var(--vscode-button-background); color: var(--vscode-button-foreground); }
    .secondary { background: var(--vscode-button-secondaryBackground); color: var(--vscode-button-secondaryForeground); }
    button:disabled { opacity: 0.5; cursor: default; }
    .toggle { display: inline-flex; gap: 6px; align-items: center; font-size: 12px; }
  </style>
</head>
<body>
  <div class="intro">Enable optional add-ons, open dedicated tools, and discover future expansion points for Spec42.</div>
  <div id="addons"></div>

  <script nonce="${nonce}">
    const vscode = acquireVsCodeApi();
    const addonsRoot = document.getElementById('addons');

    function appendText(parent, className, value) {
      const el = document.createElement('div');
      el.className = className;
      el.textContent = String(value || '');
      parent.appendChild(el);
      return el;
    }

    function render(addons) {
      addonsRoot.replaceChildren();
      (addons || []).forEach((addon) => {
        const card = document.createElement('div');
        card.className = 'addon-card';

        const header = document.createElement('div');
        header.className = 'addon-header';
        appendText(header, 'addon-title', addon.name);
        if (addon.badge) {
          const badge = document.createElement('span');
          badge.className = 'badge';
          badge.textContent = String(addon.badge);
          header.appendChild(badge);
        }
        card.appendChild(header);
        appendText(card, 'addon-description', addon.description);
        appendText(card, 'addon-status', addon.statusText);

        const actions = document.createElement('div');
        actions.className = 'addon-actions';
        if (addon.canToggle) {
          const label = document.createElement('label');
          label.className = 'toggle';
          const input = document.createElement('input');
          input.type = 'checkbox';
          input.checked = !!addon.enabled;
          input.addEventListener('change', () => {
            vscode.postMessage({ type: 'toggleAddon', addonId: addon.id, enabled: !!input.checked });
          });
          label.appendChild(input);
          label.appendChild(document.createTextNode(' Enabled'));
          actions.appendChild(label);
        }
        if (addon.canRunAnalysis) {
          const button = document.createElement('button');
          button.className = 'secondary';
          button.disabled = !addon.runAnalysisEnabled;
          button.textContent = addon.statusText === 'Analyzing...' ? 'Analyzing...' : 'Run Analysis';
          button.addEventListener('click', () => {
            vscode.postMessage({ type: 'runAnalysis', addonId: addon.id });
          });
          actions.appendChild(button);
        }
        if (addon.canOpen) {
          const button = document.createElement('button');
          button.className = 'primary';
          button.disabled = !addon.openEnabled;
          button.textContent = addon.id === 'software-architecture' ? 'Open Visualizer' : 'Open';
          button.addEventListener('click', () => {
            vscode.postMessage({ type: 'openAddon', addonId: addon.id });
          });
          actions.appendChild(button);
        }
        card.appendChild(actions);
        addonsRoot.appendChild(card);
      });
    }

    window.addEventListener('message', (event) => {
      if (event.data?.type === 'addons') {
        render(event.data.payload || []);
      }
    });

    vscode.postMessage({ type: 'initLoad' });
  </script>
</body>
</html>`;
    }
}

function getNonce(): string {
    let text = '';
    const possible = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    for (let i = 0; i < 32; i++) {
        text += possible.charAt(Math.floor(Math.random() * possible.length));
    }
    return text;
}

import * as vscode from 'vscode';
import { LspModelProvider } from '../providers/lspModelProvider';
import type { GraphNodeDTO } from '../providers/sysmlModelTypes';
import { createMessageDispatcher } from './messageHandlers';
import { createUpdateVisualizationFlow } from './updateFlow';
import { getWebviewHtml } from './htmlBuilder';
import { DEFAULT_ENABLED_VIEWS } from './webview/constants';
import { logError } from '../logger';

export const RESTORE_STATE_KEY = 'sysmlVisualizerRestoreState';

function getEnabledVisualizationViewIds(): Set<string> {
    return new Set<string>(DEFAULT_ENABLED_VIEWS);
}

function parseFileUri(value: string, label: string): vscode.Uri | undefined {
    try {
        const uri = vscode.Uri.parse(value);
        if (uri.scheme !== 'file') {
            logError(`Visualization restore ignored non-file URI for ${label}: ${value}`);
            return undefined;
        }
        return uri;
    } catch (error) {
        logError(`Visualization restore failed to parse URI for ${label}: ${value}`, error);
        return undefined;
    }
}

async function findRepresentativeWorkspaceDocument(workspaceRootUri: vscode.Uri): Promise<vscode.TextDocument> {
    const sysml = await vscode.workspace.findFiles(
        new vscode.RelativePattern(workspaceRootUri, '**/*.sysml'),
        '**/node_modules/**',
        1
    );
    const kerml = sysml.length === 0
        ? await vscode.workspace.findFiles(
            new vscode.RelativePattern(workspaceRootUri, '**/*.kerml'),
            '**/node_modules/**',
            1
        )
        : [];
    const target = sysml[0] ?? kerml[0];
    if (!target) {
        throw new Error(`No SysML/KerML documents found under ${workspaceRootUri.toString()}`);
    }
    return await vscode.workspace.openTextDocument(target);
}

export interface VisualizerRestoreState {
    workspaceRootUri: string;
    currentView: string;
    selectedView?: string;
    title?: string;
}

export class VisualizationPanel {
    public static currentPanel: VisualizationPanel | undefined;
    private readonly _panel: vscode.WebviewPanel;
    private _disposables: vscode.Disposable[] = [];
    private _currentView = 'general-view';
    private _isNavigating = false;
    private _fileChangeDebounceTimer: ReturnType<typeof setTimeout> | undefined;
    private _requestCurrentViewTimer: ReturnType<typeof setTimeout> | undefined;
    private _lastContentHash = '';
    private _needsUpdateWhenVisible = false;
    private _lastViewColumn: vscode.ViewColumn | undefined;
    private _extensionVersion = '';
    private _selectedView: string | undefined;
    private _updateFlow: ReturnType<typeof createUpdateVisualizationFlow>;

    private constructor(
        panel: vscode.WebviewPanel,
        extensionUri: vscode.Uri,
        private _document: vscode.TextDocument,
        private _lspModelProvider: LspModelProvider,
        private _workspaceRootUri: string,
        private _context?: vscode.ExtensionContext,
        initialCurrentView?: string,
        initialSelectedView?: string,
    ) {
        if (initialCurrentView && getEnabledVisualizationViewIds().has(initialCurrentView)) {
            this._currentView = initialCurrentView;
        }
        this._selectedView = initialSelectedView;
        this._extensionVersion = vscode.extensions.getExtension('Elan8.spec42')?.packageJSON?.version ?? '0.0.0';
        this._panel = panel;
        this._panel.onDidDispose(() => this.dispose(), null, this._disposables);
        this._lastViewColumn = panel.viewColumn;

        this._panel.onDidChangeViewState(() => {
            const columnChanged = this._panel.viewColumn !== this._lastViewColumn;
            this._lastViewColumn = this._panel.viewColumn;
            if (this._panel.visible && (this._needsUpdateWhenVisible || columnChanged)) {
                this._needsUpdateWhenVisible = false;
                this._lastContentHash = '';
                this.updateVisualization(true, 'panelReveal');
            }
        }, null, this._disposables);

        this._panel.webview.html = getWebviewHtml(this._panel.webview, extensionUri, this._extensionVersion);

        this._updateFlow = createUpdateVisualizationFlow({
            panel: this._panel,
            getDocument: () => this._document,
            getWorkspaceRootUri: () => this._workspaceRootUri,
            lspModelProvider: this._lspModelProvider,
            getCurrentView: () => this._currentView,
            getSelectedView: () => this._selectedView,
            setCurrentView: (value) => {
                this._currentView = value;
                this.persistRestoreState();
            },
            getIsNavigating: () => this._isNavigating,
            getNeedsUpdateWhenVisible: () => this._needsUpdateWhenVisible,
            getLastContentHash: () => this._lastContentHash,
            setLastContentHash: (hash) => { this._lastContentHash = hash; },
            setNeedsUpdateWhenVisible: (value) => { this._needsUpdateWhenVisible = value; },
        });

        this._requestCurrentViewTimer = setTimeout(() => {
            this._requestCurrentViewTimer = undefined;
            try {
                this._panel.webview.postMessage({ command: 'requestCurrentView' });
            } catch {
                // ignore teardown races
            }
        }, 100);

        const dispatch = createMessageDispatcher({
            panel: this._panel,
            document: this._document,
            workspaceRootUri: this._workspaceRootUri,
            lspModelProvider: this._lspModelProvider,
            updateVisualization: (force, triggerSource) => this.updateVisualization(force, triggerSource),
            setNavigating: (value) => { this._isNavigating = value; },
            setCurrentView: (value) => {
                this._currentView = value;
                this.persistRestoreState();
            },
            setSelectedView: (value) => {
                this._selectedView = value || undefined;
                this.persistRestoreState();
            },
            setLastContentHash: (hash) => { this._lastContentHash = hash; },
        });
        this._panel.webview.onDidReceiveMessage(dispatch, null, this._disposables);

        this.persistRestoreState();
    }

    private persistRestoreState(): void {
        if (!this._context) return;
        const state: VisualizerRestoreState = {
            workspaceRootUri: this._workspaceRootUri,
            currentView: this._currentView,
            selectedView: this._selectedView,
            title: this._panel.title !== 'SysML Model Visualizer' ? this._panel.title : undefined,
        };
        this._context.workspaceState.update(RESTORE_STATE_KEY, state);
    }

    public static createOrShow(
        context: vscode.ExtensionContext,
        document: vscode.TextDocument,
        customTitle?: string,
        lspModelProvider?: LspModelProvider,
        workspaceRootUri?: vscode.Uri,
    ): void {
        const extensionUri = context.extensionUri;
        const activeColumn = vscode.window.activeTextEditor?.viewColumn;
        const visualizerColumn = activeColumn === vscode.ViewColumn.One
            ? vscode.ViewColumn.Two
            : activeColumn === vscode.ViewColumn.Two
                ? vscode.ViewColumn.Three
                : vscode.ViewColumn.Beside;
        const title = customTitle || 'SysML Model Visualizer';
        const resolvedWorkspaceRootUri = workspaceRootUri
            ?? vscode.workspace.getWorkspaceFolder(document.uri)?.uri
            ?? vscode.workspace.workspaceFolders?.[0]?.uri;
        if (!resolvedWorkspaceRootUri) {
            throw new Error('Cannot open the visualizer without a workspace root URI.');
        }

        if (VisualizationPanel.currentPanel) {
            VisualizationPanel.currentPanel._panel.title = title;
            VisualizationPanel.currentPanel._panel.reveal(visualizerColumn);
            if (lspModelProvider) {
                VisualizationPanel.currentPanel._lspModelProvider = lspModelProvider;
            }
            const workspaceChanged =
                VisualizationPanel.currentPanel._workspaceRootUri !== resolvedWorkspaceRootUri.toString();
            if (VisualizationPanel.currentPanel._document !== document || workspaceChanged) {
                VisualizationPanel.currentPanel._document = document;
                VisualizationPanel.currentPanel._workspaceRootUri = resolvedWorkspaceRootUri.toString();
                VisualizationPanel.currentPanel._lastContentHash = '';
                VisualizationPanel.currentPanel.updateVisualization(true, 'createOrShow');
            }
            VisualizationPanel.currentPanel.persistRestoreState();
            return;
        }

        if (!lspModelProvider) {
            return;
        }

        const panel = vscode.window.createWebviewPanel(
            'sysmlVisualizer',
            title,
            visualizerColumn,
            {
                enableScripts: true,
                retainContextWhenHidden: true,
                localResourceRoots: [vscode.Uri.joinPath(extensionUri, 'media')],
            }
        );

        VisualizationPanel.currentPanel = new VisualizationPanel(
            panel,
            extensionUri,
            document,
            lspModelProvider,
            resolvedWorkspaceRootUri.toString(),
            context,
        );
    }

    public static async restore(
        panel: vscode.WebviewPanel,
        context: vscode.ExtensionContext,
        lspModelProvider: LspModelProvider,
        savedState: VisualizerRestoreState,
    ): Promise<void> {
        const extensionUri = context.extensionUri;
        const workspaceRootUri = parseFileUri(savedState.workspaceRootUri, 'workspaceRootUri');
        if (!workspaceRootUri) {
            throw new Error('Saved visualization state does not contain a valid workspace root URI.');
        }
        const document = await findRepresentativeWorkspaceDocument(workspaceRootUri);
        if (savedState.title) {
            panel.title = savedState.title;
        }
        const view = getEnabledVisualizationViewIds().has(savedState.currentView)
            ? savedState.currentView
            : 'general-view';
        VisualizationPanel.currentPanel = new VisualizationPanel(
            panel,
            extensionUri,
            document,
            lspModelProvider,
            workspaceRootUri.toString(),
            context,
            view,
            savedState.selectedView,
        );
    }

    public exportVisualization(format: string, scale = 2): void {
        this._panel.webview.postMessage({ command: 'export', format: format.toLowerCase(), scale });
    }

    private updateVisualization(forceUpdate = false, triggerSource = 'unknown'): Promise<void> {
        return this._updateFlow.update(forceUpdate, triggerSource);
    }

    public getDocument(): vscode.TextDocument {
        return this._document;
    }

    public isNavigating(): boolean {
        return this._isNavigating;
    }

    public tracksUri(uri: vscode.Uri): boolean {
        const workspaceRootUri = vscode.Uri.parse(this._workspaceRootUri);
        const rootPath = workspaceRootUri.fsPath.toLowerCase();
        return uri.fsPath.toLowerCase().startsWith(rootPath);
    }

    public getWebview(): vscode.Webview {
        return this._panel.webview;
    }

    public setLspModelProvider(provider: LspModelProvider): void {
        this._lspModelProvider = provider;
    }

    public changeView(viewId: string): void {
        this._panel.webview.postMessage({ command: 'changeView', view: viewId });
        this._currentView = viewId;
        this.persistRestoreState();
    }

    public selectPackage(_packageName: string): void {
        this._currentView = 'general-view';
        this._lastContentHash = '';
        this.persistRestoreState();
        this.updateVisualization(true, 'selectPackage');
    }

    public clearPackageSelection(): void {
        this._selectedView = undefined;
        this._lastContentHash = '';
        this.persistRestoreState();
        this.updateVisualization(true, 'clearPackageSelection');
    }

    public highlightElementByName(elementName: string, skipCentering = true): void {
        this._panel.webview.postMessage({
            command: 'highlightElement',
            elementName,
            skipCentering,
        });
    }

    public revealSourceSelection(node: GraphNodeDTO): void {
        if (node.type === 'package') {
            this.selectPackage(node.id || node.name);
            return;
        }
        this.highlightElementByName(node.name, false);
    }

    public notifyFileChanged(uri: vscode.Uri): void {
        if (!this.tracksUri(uri)) {
            return;
        }
        if (this._fileChangeDebounceTimer) {
            clearTimeout(this._fileChangeDebounceTimer);
        }
        this._fileChangeDebounceTimer = setTimeout(() => {
            this._fileChangeDebounceTimer = undefined;
            this.updateVisualization(true, 'fileChanged');
        }, 400);
    }

    public refresh(): void {
        this.updateVisualization(true, 'manualRefresh');
    }

    public dispose(): void {
        VisualizationPanel.currentPanel = undefined;
        this._context?.workspaceState.update(RESTORE_STATE_KEY, undefined);
        if (this._requestCurrentViewTimer) {
            clearTimeout(this._requestCurrentViewTimer);
            this._requestCurrentViewTimer = undefined;
        }
        if (this._fileChangeDebounceTimer) {
            clearTimeout(this._fileChangeDebounceTimer);
            this._fileChangeDebounceTimer = undefined;
        }
        this._panel.dispose();
        while (this._disposables.length) {
            const disposable = this._disposables.pop();
            if (disposable) {
                disposable.dispose();
            }
        }
    }
}

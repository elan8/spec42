import * as vscode from 'vscode';
import { LspModelProvider } from '../providers/lspModelProvider';
import { getWebviewHtml } from './htmlBuilder';
import { createMessageDispatcher } from './messageHandlers';
import { type UpdateMessage, type FetchModelParams } from './modelFetcher';
import { createUpdateVisualizationFlow } from './updateFlow';

export interface BaseVisualizerRestoreState {
    workspaceRootUri: string;
    currentView: string;
    title?: string;
}

export interface VisualizationPanelRuntimeState {
    workspaceRootUri: string;
    currentView: string;
    selectedView?: string;
    document?: vscode.TextDocument;
    lspModelProvider: LspModelProvider;
}

export interface VisualizationPanelVariantConfig<TRestoreState extends BaseVisualizerRestoreState> {
    panelTypeId: string;
    restoreStateKey: string;
    defaultTitle: string;
    enabledViews: readonly string[];
    defaultView: string;
    loadingMessage: string;
    getRuntimeState: () => VisualizationPanelRuntimeState;
    updateCurrentView: (view: string) => void;
    updateSelectedView?: (selectedView?: string) => void;
    serializeRestoreState: (runtimeState: VisualizationPanelRuntimeState, panelTitle: string) => TRestoreState;
    fetchUpdateMessage: (params: FetchModelParams) => Promise<UpdateMessage | null>;
    getContentHashSource: (runtimeState: VisualizationPanelRuntimeState) => string;
    normalizeView?: (viewId: string) => string;
    shouldTrackUri?: (uri: vscode.Uri, runtimeState: VisualizationPanelRuntimeState) => boolean;
}

export function parseFileUri(value: string, label: string, logError: (message: string, error?: unknown) => void): vscode.Uri | undefined {
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

export function getVisualizerColumn(): vscode.ViewColumn {
    const activeColumn = vscode.window.activeTextEditor?.viewColumn;
    return activeColumn === vscode.ViewColumn.One
        ? vscode.ViewColumn.Two
        : activeColumn === vscode.ViewColumn.Two
            ? vscode.ViewColumn.Three
            : vscode.ViewColumn.Beside;
}

export class BaseVisualizationPanelController<TRestoreState extends BaseVisualizerRestoreState> {
    private readonly _panel: vscode.WebviewPanel;
    private readonly _config: VisualizationPanelVariantConfig<TRestoreState>;
    private readonly _context?: vscode.ExtensionContext;
    private readonly _disposables: vscode.Disposable[] = [];
    private _isNavigating = false;
    private _fileChangeDebounceTimer: ReturnType<typeof setTimeout> | undefined;
    private _requestCurrentViewTimer: ReturnType<typeof setTimeout> | undefined;
    private _lastContentHash = '';
    private _needsUpdateWhenVisible = false;
    private _lastViewColumn: vscode.ViewColumn | undefined;
    private _updateFlow: ReturnType<typeof createUpdateVisualizationFlow>;
    private _disposed = false;

    constructor(
        panel: vscode.WebviewPanel,
        extensionUri: vscode.Uri,
        context: vscode.ExtensionContext | undefined,
        config: VisualizationPanelVariantConfig<TRestoreState>,
    ) {
        this._panel = panel;
        this._context = context;
        this._config = config;
        const extensionVersion = vscode.extensions.getExtension('Elan8.spec42')?.packageJSON?.version ?? '0.0.0';
        this._lastViewColumn = panel.viewColumn;

        this._panel.onDidDispose(() => this.dispose(), null, this._disposables);
        this._panel.onDidChangeViewState(() => {
            const columnChanged = this._panel.viewColumn !== this._lastViewColumn;
            this._lastViewColumn = this._panel.viewColumn;
            if (this._panel.visible && (this._needsUpdateWhenVisible || columnChanged)) {
                this._needsUpdateWhenVisible = false;
                this._lastContentHash = '';
                void this.updateVisualization(true, 'panelReveal');
            }
        }, null, this._disposables);

        this._panel.webview.html = getWebviewHtml(
            this._panel.webview,
            extensionUri,
            extensionVersion,
            config.enabledViews,
        );

        this._updateFlow = createUpdateVisualizationFlow({
            panel: this._panel,
            getDocument: () => this._config.getRuntimeState().document,
            getWorkspaceRootUri: () => this._config.getRuntimeState().workspaceRootUri,
            lspModelProvider: this._config.getRuntimeState().lspModelProvider,
            getCurrentView: () => this._config.getRuntimeState().currentView,
            getSelectedView: () => this._config.getRuntimeState().selectedView,
            setCurrentView: (value) => {
                this._config.updateCurrentView(this.normalizeView(value));
                this.persistRestoreState();
            },
            getIsNavigating: () => this._isNavigating,
            getNeedsUpdateWhenVisible: () => this._needsUpdateWhenVisible,
            getLastContentHash: () => this._lastContentHash,
            getContentHashSource: () => this._config.getContentHashSource(this._config.getRuntimeState()),
            setLastContentHash: (hash) => { this._lastContentHash = hash; },
            setNeedsUpdateWhenVisible: (value) => { this._needsUpdateWhenVisible = value; },
            fetchUpdateMessage: () => this._config.fetchUpdateMessage({
                workspaceRootUri: this._config.getRuntimeState().workspaceRootUri,
                lspModelProvider: this._config.getRuntimeState().lspModelProvider,
                currentView: this._config.getRuntimeState().currentView,
                selectedView: this._config.getRuntimeState().selectedView,
            }),
            loadingMessage: this._config.loadingMessage,
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
            document: this._config.getRuntimeState().document,
            workspaceRootUri: this._config.getRuntimeState().workspaceRootUri,
            lspModelProvider: this._config.getRuntimeState().lspModelProvider,
            updateVisualization: (force, triggerSource) => void this.updateVisualization(force, triggerSource),
            setNavigating: (value) => { this._isNavigating = value; },
            setCurrentView: (value) => {
                this._config.updateCurrentView(this.normalizeView(value));
                this.persistRestoreState();
            },
            setSelectedView: (value) => {
                this._config.updateSelectedView?.(value);
                this.persistRestoreState();
            },
            setLastContentHash: (hash) => { this._lastContentHash = hash; },
        });
        this._panel.webview.onDidReceiveMessage(dispatch, null, this._disposables);

        this.persistRestoreState();
    }

    get panel(): vscode.WebviewPanel {
        return this._panel;
    }

    getWebview(): vscode.Webview {
        return this._panel.webview;
    }

    getDocument(): vscode.TextDocument | undefined {
        return this._config.getRuntimeState().document;
    }

    getWorkspaceRootUri(): string {
        return this._config.getRuntimeState().workspaceRootUri;
    }

    isNavigating(): boolean {
        return this._isNavigating;
    }

    setLspModelProvider(provider: LspModelProvider): void {
        this._config.getRuntimeState().lspModelProvider = provider;
    }

    updatePanelTitle(title: string): void {
        this._panel.title = title;
        this.persistRestoreState();
    }

    reveal(column = getVisualizerColumn()): void {
        this._panel.reveal(column);
    }

    updateRuntimeState(mutator: (runtimeState: VisualizationPanelRuntimeState) => void): void {
        const runtimeState = this._config.getRuntimeState();
        mutator(runtimeState);
    }

    normalizeView(viewId: string): string {
        if (this._config.normalizeView) {
            return this._config.normalizeView(viewId);
        }
        return this.getEnabledVisualizationViewIds().has(viewId) ? viewId : this._config.defaultView;
    }

    async updateVisualization(forceUpdate = false, triggerSource = 'unknown'): Promise<void> {
        await this._updateFlow.update(forceUpdate, triggerSource);
    }

    changeView(viewId: string): void {
        const normalizedView = this.normalizeView(viewId);
        this._panel.webview.postMessage({ command: 'changeView', view: normalizedView });
        this._config.updateCurrentView(normalizedView);
        this.persistRestoreState();
    }

    async notifyTrackedUriChanged(uri: vscode.Uri, triggerSource = 'fileChanged'): Promise<void> {
        const runtimeState = this._config.getRuntimeState();
        if (this._config.shouldTrackUri && !this._config.shouldTrackUri(uri, runtimeState)) {
            return;
        }
        if (this._fileChangeDebounceTimer) {
            clearTimeout(this._fileChangeDebounceTimer);
        }
        this._fileChangeDebounceTimer = setTimeout(() => {
            this._fileChangeDebounceTimer = undefined;
            void this.updateVisualization(true, triggerSource);
        }, 400);
    }

    refresh(): void {
        this._lastContentHash = '';
        void this.updateVisualization(true, 'manualRefresh');
    }

    persistRestoreState(): void {
        if (!this._context) {
            return;
        }
        const state = this._config.serializeRestoreState(this._config.getRuntimeState(), this._panel.title);
        this._context.workspaceState.update(this._config.restoreStateKey, state);
    }

    clearRestoreState(): void {
        this._context?.workspaceState.update(this._config.restoreStateKey, undefined);
    }

    dispose(): void {
        if (this._disposed) {
            return;
        }
        this._disposed = true;
        this.clearRestoreState();
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
            disposable?.dispose();
        }
    }

    private getEnabledVisualizationViewIds(): Set<string> {
        return new Set<string>(this._config.enabledViews);
    }
}

import * as vscode from 'vscode';
import { LspModelProvider } from '../providers/lspModelProvider';
import { fetchModelData, type FetchModelParams } from './modelFetcher';
import type { GraphNodeDTO } from '../providers/sysmlModelTypes';
import { SYSML_ENABLED_VIEWS } from './webview/constants';
import { logError } from '../logger';
import {
    BaseVisualizationPanelController,
    getVisualizerColumn,
    parseFileUri,
    type BaseVisualizerRestoreState,
    type VisualizationPanelRuntimeState,
    type VisualizationPanelVariantConfig,
} from './baseVisualizationPanelController';

export const RESTORE_STATE_KEY = 'sysmlVisualizerRestoreState';

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

export interface VisualizerRestoreState extends BaseVisualizerRestoreState {
    selectedView?: string;
}

function createVariantConfig(runtimeState: VisualizationPanelRuntimeState): VisualizationPanelVariantConfig<VisualizerRestoreState> {
    return {
        panelTypeId: 'sysmlVisualizer',
        restoreStateKey: RESTORE_STATE_KEY,
        defaultTitle: 'SysML Model Visualizer',
        enabledViews: SYSML_ENABLED_VIEWS,
        defaultView: 'general-view',
        loadingMessage: 'Parsing SysML model...',
        getRuntimeState: () => runtimeState,
        updateCurrentView: (view) => {
            runtimeState.currentView = view;
        },
        updateSelectedView: (selectedView) => {
            runtimeState.selectedView = selectedView || undefined;
        },
        serializeRestoreState: (state, panelTitle) => ({
            workspaceRootUri: state.workspaceRootUri,
            currentView: state.currentView,
            selectedView: state.selectedView,
            title: panelTitle !== 'SysML Model Visualizer' ? panelTitle : undefined,
        }),
        fetchUpdateMessage: (params: FetchModelParams) => fetchModelData(params),
        getContentHashSource: (state) => JSON.stringify({
            uri: state.document?.uri.toString() ?? null,
            version: state.document?.version ?? 0,
            workspaceRootUri: state.workspaceRootUri,
            currentView: state.currentView,
            selectedView: state.selectedView ?? null,
        }),
        normalizeView: (viewId) => new Set<string>(SYSML_ENABLED_VIEWS).has(viewId) ? viewId : 'general-view',
        shouldTrackUri: (uri, state) => {
            const workspaceRootUri = vscode.Uri.parse(state.workspaceRootUri);
            const rootPath = workspaceRootUri.fsPath.toLowerCase();
            return uri.fsPath.toLowerCase().startsWith(rootPath);
        },
    };
}

export class VisualizationPanel {
    public static currentPanel: VisualizationPanel | undefined;

    private readonly _runtimeState: VisualizationPanelRuntimeState;
    private readonly _controller: BaseVisualizationPanelController<VisualizerRestoreState>;

    private constructor(
        panel: vscode.WebviewPanel,
        extensionUri: vscode.Uri,
        document: vscode.TextDocument,
        lspModelProvider: LspModelProvider,
        workspaceRootUri: string,
        context?: vscode.ExtensionContext,
        initialCurrentView?: string,
        initialSelectedView?: string,
    ) {
        this._runtimeState = {
            workspaceRootUri,
            currentView: new Set<string>(SYSML_ENABLED_VIEWS).has(initialCurrentView || '')
                ? initialCurrentView || 'general-view'
                : 'general-view',
            selectedView: initialSelectedView,
            document,
            lspModelProvider,
        };
        this._controller = new BaseVisualizationPanelController(
            panel,
            extensionUri,
            context,
            createVariantConfig(this._runtimeState),
        );
        panel.onDidDispose(() => {
            if (VisualizationPanel.currentPanel === this) {
                VisualizationPanel.currentPanel = undefined;
            }
        });
    }

    public static createOrShow(
        context: vscode.ExtensionContext,
        document: vscode.TextDocument,
        customTitle?: string,
        lspModelProvider?: LspModelProvider,
        workspaceRootUri?: vscode.Uri,
    ): void {
        const extensionUri = context.extensionUri;
        const visualizerColumn = getVisualizerColumn();
        const title = customTitle || 'SysML Model Visualizer';
        const resolvedWorkspaceRootUri = workspaceRootUri
            ?? vscode.workspace.getWorkspaceFolder(document.uri)?.uri
            ?? vscode.workspace.workspaceFolders?.[0]?.uri;
        if (!resolvedWorkspaceRootUri || !lspModelProvider) {
            throw new Error('Cannot open the visualizer without a workspace root URI and model provider.');
        }

        if (VisualizationPanel.currentPanel) {
            VisualizationPanel.currentPanel._controller.updatePanelTitle(title);
            VisualizationPanel.currentPanel._controller.reveal(visualizerColumn);
            VisualizationPanel.currentPanel._controller.setLspModelProvider(lspModelProvider);
            const runtimeState = VisualizationPanel.currentPanel._runtimeState;
            const workspaceChanged = runtimeState.workspaceRootUri !== resolvedWorkspaceRootUri.toString();
            if (runtimeState.document !== document || workspaceChanged) {
                runtimeState.document = document;
                runtimeState.workspaceRootUri = resolvedWorkspaceRootUri.toString();
                VisualizationPanel.currentPanel._controller.refresh();
            }
            VisualizationPanel.currentPanel._controller.persistRestoreState();
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
        const workspaceRootUri = parseFileUri(savedState.workspaceRootUri, 'workspaceRootUri', logError);
        if (!workspaceRootUri) {
            throw new Error('Saved visualization state does not contain a valid workspace root URI.');
        }
        const document = await findRepresentativeWorkspaceDocument(workspaceRootUri);
        if (savedState.title) {
            panel.title = savedState.title;
        }
        VisualizationPanel.currentPanel = new VisualizationPanel(
            panel,
            extensionUri,
            document,
            lspModelProvider,
            workspaceRootUri.toString(),
            context,
            savedState.currentView,
            savedState.selectedView,
        );
    }

    public exportVisualization(format: string, scale = 2): void {
        this._controller.getWebview().postMessage({ command: 'export', format: format.toLowerCase(), scale });
    }

    public getDocument(): vscode.TextDocument {
        return this._runtimeState.document!;
    }

    public isNavigating(): boolean {
        return this._controller.isNavigating();
    }

    public tracksUri(uri: vscode.Uri): boolean {
        const workspaceRootUri = vscode.Uri.parse(this._runtimeState.workspaceRootUri);
        const rootPath = workspaceRootUri.fsPath.toLowerCase();
        return uri.fsPath.toLowerCase().startsWith(rootPath);
    }

    public getWebview(): vscode.Webview {
        return this._controller.getWebview();
    }

    public setLspModelProvider(provider: LspModelProvider): void {
        this._controller.setLspModelProvider(provider);
    }

    public changeView(viewId: string): void {
        this._controller.changeView(viewId);
    }

    public selectPackage(_packageName: string): void {
        this._runtimeState.currentView = 'general-view';
        this._controller.refresh();
        this._controller.persistRestoreState();
    }

    public clearPackageSelection(): void {
        this._runtimeState.selectedView = undefined;
        this._controller.refresh();
        this._controller.persistRestoreState();
    }

    public highlightElementByName(elementName: string, skipCentering = true): void {
        this.getWebview().postMessage({
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
        void this._controller.notifyTrackedUriChanged(uri, 'fileChanged');
    }

    public refresh(): void {
        this._controller.refresh();
    }

    public dispose(): void {
        VisualizationPanel.currentPanel = undefined;
        this._controller.dispose();
    }
}

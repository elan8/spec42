import * as vscode from 'vscode';
import { logError } from '../logger';
import { LspModelProvider } from '../providers/lspModelProvider';
import { buildSoftwareUpdateMessage } from './modelFetcher';
import type { SoftwareAnalysisStore } from '../addons/softwareAnalysisStore';
import {
    BaseVisualizationPanelController,
    getVisualizerColumn,
    parseFileUri,
    type BaseVisualizerRestoreState,
    type VisualizationPanelRuntimeState,
    type VisualizationPanelVariantConfig,
} from './baseVisualizationPanelController';
import { SOFTWARE_ENABLED_VIEWS } from './webview/constants';

export const SOFTWARE_RESTORE_STATE_KEY = 'softwareVisualizerRestoreState';

export interface SoftwareVisualizerRestoreState extends BaseVisualizerRestoreState {}

function createVariantConfig(
    runtimeState: VisualizationPanelRuntimeState,
    analysisStore: SoftwareAnalysisStore,
): VisualizationPanelVariantConfig<SoftwareVisualizerRestoreState> {
    return {
        panelTypeId: 'spec42SoftwareVisualizer',
        restoreStateKey: SOFTWARE_RESTORE_STATE_KEY,
        defaultTitle: 'Software Architecture Visualizer',
        enabledViews: SOFTWARE_ENABLED_VIEWS,
        defaultView: 'software-module-view',
        loadingMessage: 'Extracting Rust software architecture...',
        getRuntimeState: () => runtimeState,
        updateCurrentView: (view) => {
            runtimeState.currentView = view;
            runtimeState.selectedView = view;
        },
        updateSelectedView: (selectedView) => {
            runtimeState.currentView = selectedView || 'software-module-view';
            runtimeState.selectedView = runtimeState.currentView;
        },
        serializeRestoreState: (state, panelTitle) => ({
            workspaceRootUri: state.workspaceRootUri,
            currentView: state.currentView,
            title: panelTitle !== 'Software Architecture Visualizer' ? panelTitle : undefined,
        }),
        fetchUpdateMessage: async () => {
            const entry = analysisStore.get(runtimeState.workspaceRootUri);
            return buildSoftwareUpdateMessage(
                runtimeState.workspaceRootUri,
                runtimeState.currentView,
                runtimeState.lspModelProvider,
                entry.status === 'ready' ? entry.model : undefined,
            );
        },
        getContentHashSource: (state) => JSON.stringify({
            workspaceRootUri: state.workspaceRootUri,
            currentView: state.currentView,
            analysisStatus: analysisStore.get(state.workspaceRootUri).status,
            modelSummary: analysisStore.get(state.workspaceRootUri).model?.summary ?? null,
            analysisFinishedAt: analysisStore.get(state.workspaceRootUri).finishedAt ?? null,
            analysisError: analysisStore.get(state.workspaceRootUri).errorMessage ?? null,
        }),
        normalizeView: (viewId) => new Set<string>(SOFTWARE_ENABLED_VIEWS).has(viewId) ? viewId : 'software-module-view',
        shouldTrackUri: (uri, state) => {
            const workspaceRootUri = vscode.Uri.parse(state.workspaceRootUri);
            const rootPath = workspaceRootUri.fsPath.toLowerCase();
            return uri.fsPath.toLowerCase().startsWith(rootPath);
        },
    };
}

export class SoftwareVisualizationPanel {
    public static currentPanel: SoftwareVisualizationPanel | undefined;

    private readonly _runtimeState: VisualizationPanelRuntimeState;
    private readonly _controller: BaseVisualizationPanelController<SoftwareVisualizerRestoreState>;

    private constructor(
        panel: vscode.WebviewPanel,
        extensionUri: vscode.Uri,
        workspaceRootUri: string,
        lspModelProvider: LspModelProvider,
        private readonly _analysisStore: SoftwareAnalysisStore,
        context?: vscode.ExtensionContext,
        initialCurrentView?: string,
    ) {
        this._runtimeState = {
            workspaceRootUri,
            currentView: new Set<string>(SOFTWARE_ENABLED_VIEWS).has(initialCurrentView || '')
                ? initialCurrentView || 'software-module-view'
                : 'software-module-view',
            selectedView: initialCurrentView,
            document: undefined,
            lspModelProvider,
        };
        this._controller = new BaseVisualizationPanelController(
            panel,
            extensionUri,
            context,
            createVariantConfig(this._runtimeState, this._analysisStore),
        );
        panel.onDidDispose(() => {
            if (SoftwareVisualizationPanel.currentPanel === this) {
                SoftwareVisualizationPanel.currentPanel = undefined;
            }
        });
    }

    public static createOrShow(
        context: vscode.ExtensionContext,
        workspaceRootUri: vscode.Uri,
        lspModelProvider: LspModelProvider,
        analysisStore: SoftwareAnalysisStore,
        initialView = 'software-module-view',
    ): void {
        const extensionUri = context.extensionUri;
        const visualizerColumn = getVisualizerColumn();

        if (SoftwareVisualizationPanel.currentPanel) {
            SoftwareVisualizationPanel.currentPanel._controller.updatePanelTitle('Software Architecture Visualizer');
            SoftwareVisualizationPanel.currentPanel._controller.reveal(visualizerColumn);
            SoftwareVisualizationPanel.currentPanel._controller.setLspModelProvider(lspModelProvider);
            const runtimeState = SoftwareVisualizationPanel.currentPanel._runtimeState;
            if (
                runtimeState.workspaceRootUri !== workspaceRootUri.toString() ||
                runtimeState.currentView !== initialView
            ) {
                runtimeState.workspaceRootUri = workspaceRootUri.toString();
                runtimeState.currentView = new Set<string>(SOFTWARE_ENABLED_VIEWS).has(initialView)
                    ? initialView
                    : 'software-module-view';
                runtimeState.selectedView = runtimeState.currentView;
                SoftwareVisualizationPanel.currentPanel._controller.refresh();
            }
            SoftwareVisualizationPanel.currentPanel._controller.persistRestoreState();
            return;
        }

        const panel = vscode.window.createWebviewPanel(
            'spec42SoftwareVisualizer',
            'Software Architecture Visualizer',
            visualizerColumn,
            {
                enableScripts: true,
                retainContextWhenHidden: true,
                localResourceRoots: [vscode.Uri.joinPath(extensionUri, 'media')],
            }
        );

        SoftwareVisualizationPanel.currentPanel = new SoftwareVisualizationPanel(
            panel,
            extensionUri,
            workspaceRootUri.toString(),
            lspModelProvider,
            analysisStore,
            context,
            initialView,
        );
    }

    public static async restore(
        panel: vscode.WebviewPanel,
        context: vscode.ExtensionContext,
        lspModelProvider: LspModelProvider,
        analysisStore: SoftwareAnalysisStore,
        savedState: SoftwareVisualizerRestoreState,
    ): Promise<void> {
        const extensionUri = context.extensionUri;
        const workspaceRootUri = parseFileUri(savedState.workspaceRootUri, 'workspaceRootUri', logError);
        if (!workspaceRootUri) {
            throw new Error('Saved software visualization state does not contain a valid workspace root URI.');
        }
        if (savedState.title) {
            panel.title = savedState.title;
        }
        SoftwareVisualizationPanel.currentPanel = new SoftwareVisualizationPanel(
            panel,
            extensionUri,
            workspaceRootUri.toString(),
            lspModelProvider,
            analysisStore,
            context,
            savedState.currentView,
        );
    }

    public changeView(viewId: string): void {
        this._controller.changeView(viewId);
    }

    public refresh(): void {
        this._controller.refresh();
    }

    public getWorkspaceRootUri(): string {
        return this._runtimeState.workspaceRootUri;
    }

    public notifyWorkspaceFileChanged(uri: vscode.Uri): void {
        void this._controller.notifyTrackedUriChanged(uri, 'fileChanged');
    }

    public dispose(): void {
        SoftwareVisualizationPanel.currentPanel = undefined;
        this._controller.dispose();
    }
}

import * as vscode from 'vscode';
import { hashContent, type UpdateMessage } from './modelFetcher';
import { createUpdateId } from './renderContract';
import { isVerboseLoggingEnabled, log, logError, logPerfEvent } from '../logger';
import {
    getVisualizerReadinessSnapshot,
    setVisualizerBootstrapCompleted,
    setVisualizerUpdateInFlight,
} from './visualizerReadiness';

export interface UpdateFlowDeps {
    panel: vscode.WebviewPanel;
    getDocument?: () => vscode.TextDocument | undefined;
    getWorkspaceRootUri: () => string;
    lspModelProvider?: unknown;
    getCurrentView: () => string;
    getSelectedView: () => string | undefined;
    setCurrentView: (view: string) => void;
    getIsNavigating: () => boolean;
    getNeedsUpdateWhenVisible: () => boolean;
    getLastContentHash: () => string;
    getContentHashSource?: () => string;
    setLastContentHash: (hash: string) => void;
    setNeedsUpdateWhenVisible: (value: boolean) => void;
    fetchUpdateMessage?: () => Promise<UpdateMessage | null>;
    loadingMessage?: string;
    getLoadingMessage?: () => string;
    isDisposed?: () => boolean;
}

function logPerf(event: string, extra?: Record<string, unknown>): void {
    logPerfEvent(event, extra);
}

function isWebviewDisposedError(error: unknown): boolean {
    return error instanceof Error && error.message.includes('Webview is disposed');
}

async function safePostMessage(
    panel: vscode.WebviewPanel,
    message: unknown,
    isDisposed?: () => boolean,
): Promise<boolean> {
    if (isDisposed?.()) {
        return false;
    }
    try {
        return await panel.webview.postMessage(message);
    } catch (error) {
        if (!isWebviewDisposedError(error)) {
            throw error;
        }
        return false;
    }
}

/** Workspace semantic trees may contain parent/child cycles; omit from webview payloads. */
export function toWebviewUpdateMessage(msg: UpdateMessage): UpdateMessage {
    const { elements: _elements, ...safe } = msg;
    try {
        return JSON.parse(JSON.stringify(safe)) as UpdateMessage;
    } catch (error) {
        logError('updateFlow:webviewSerializeFailed', error);
        return {
            command: 'update',
            modelReady: false,
            modelStatusMessage: 'Diagram data could not be sent to the visualizer.',
            graph: { nodes: [], edges: [] },
            generalViewGraph: { nodes: [], edges: [] },
            activityDiagrams: [],
            sequenceDiagrams: [],
            currentView: safe.currentView,
            viewCandidates: [],
            emptyStateMessage: 'Diagram data could not be sent to the visualizer.',
        };
    }
}

export function createUpdateVisualizationFlow(deps: UpdateFlowDeps): { update: (force: boolean, triggerSource?: string) => Promise<void> } {
    const {
        panel,
        getDocument,
        getWorkspaceRootUri,
        getCurrentView,
        getSelectedView,
        setCurrentView,
        getIsNavigating,
        getNeedsUpdateWhenVisible,
        getLastContentHash,
        getContentHashSource,
        setLastContentHash,
        setNeedsUpdateWhenVisible,
        fetchUpdateMessage,
        loadingMessage,
        getLoadingMessage,
        isDisposed,
    } = deps;
    let bootstrapCompleted = false;
    let inFlightUpdate:
        | {
            key: string;
            triggerSource: string;
            isBootstrap: boolean;
            promise: Promise<void>;
        }
        | undefined;

    function currentUpdateKey(): string {
        return JSON.stringify({
            documentUri: getDocument?.()?.uri.toString() ?? null,
            workspaceRootUri: getWorkspaceRootUri(),
            currentView: getCurrentView(),
            selectedView: getSelectedView() ?? null,
        });
    }

    async function doUpdateVisualization(triggerSource: string): Promise<void> {
        const document = getDocument?.();
        const workspaceRootUri = getWorkspaceRootUri();
        const updateStartedAt = Date.now();
        const readiness = getVisualizerReadinessSnapshot(triggerSource);
        if (!readiness.fetchAllowed) {
            log('updateFlow:modelNotReady', readiness.loadingMessage ?? 'waiting');
            if (!readiness.suppressNotReadyFlash) {
                await safePostMessage(panel, {
                    command: 'modelNotReady',
                    message: readiness.loadingMessage ?? 'Waiting for SysML model...',
                }, isDisposed);
            }
            return;
        }
        try {
            log(
                'updateFlow:fetch:start',
                `doc=${document?.uri.toString() ?? '(none)'}`,
                `workspaceRootUri=${workspaceRootUri}`,
                `currentView=${getCurrentView()}`,
                `selectedView=${getSelectedView() ?? '(auto)'}`,
            );
            if (isVerboseLoggingEnabled()) {
                try {
                    // eslint-disable-next-line no-console
                    console.log(
                        '[viz][updateFlow:fetch:start]',
                        JSON.stringify({
                            doc: document?.uri.toString() ?? null,
                            workspaceRootUri,
                            currentView: getCurrentView(),
                            selectedView: getSelectedView() ?? null,
                        })
                    );
                } catch {
                    // ignore
                }
            }
            const msg = fetchUpdateMessage ? await fetchUpdateMessage() : null;
            logPerf('visualizer:fetchModelDataCompleted', {
                currentView: getCurrentView(),
                totalMs: Date.now() - updateStartedAt,
                hasMessage: !!msg,
                workspaceRootUri,
            });
            if (msg) {
                if (msg.modelReady === false) {
                    log('updateFlow:serverModelNotReady', msg.modelStatusMessage ?? '');
                    if (!readiness.suppressNotReadyFlash) {
                        await safePostMessage(panel, {
                            command: 'modelNotReady',
                            message: msg.modelStatusMessage ?? 'Waiting for SysML model...',
                        }, isDisposed);
                    }
                    return;
                }
                if (msg.currentView && msg.currentView !== getCurrentView()) {
                    setCurrentView(msg.currentView);
                }
                log(
                    'updateFlow:post:update',
                    `graphNodes=${msg.graph?.nodes?.length || 0}`,
                    `graphEdges=${msg.graph?.edges?.length || 0}`,
                    `viewCandidates=${msg.viewCandidates?.length || 0}`,
                    `currentView=${msg.currentView}`,
                    `selectedView=${msg.selectedViewName ?? '(auto)'}`,
                );
                if (isVerboseLoggingEnabled()) {
                    try {
                        // eslint-disable-next-line no-console
                        console.log(
                            '[viz][updateFlow:post:update]',
                            JSON.stringify({
                                graphNodes: msg.graph?.nodes?.length || 0,
                                graphEdges: msg.graph?.edges?.length || 0,
                                viewCandidates: msg.viewCandidates?.length || 0,
                                currentView: msg.currentView,
                                selectedView: msg.selectedViewName ?? null,
                            })
                        );
                    } catch {
                        // ignore
                    }
                }
                const postStartedAt = Date.now();
                const updateId = createUpdateId();
                logPerf('visualizer:webviewPostMessageStarted', {
                    command: msg.command,
                    currentView: msg.currentView,
                    graphNodes: msg.graph?.nodes?.length || 0,
                    graphEdges: msg.graph?.edges?.length || 0,
                    updateId,
                });
                const webviewMsg = toWebviewUpdateMessage({ ...msg, updateId });
                const delivered = await safePostMessage(panel, webviewMsg, isDisposed);
                logPerf('visualizer:webviewPostMessageCompleted', {
                    command: msg.command,
                    currentView: msg.currentView,
                    delivered,
                    updateId,
                    totalMs: Date.now() - postStartedAt,
                });
                if (!delivered) {
                    logError('updateFlow:webviewPostMessageRejected', new Error('webview postMessage returned false'));
                    await safePostMessage(panel, {
                        command: 'modelNotReady',
                        message: 'Visualizer could not receive diagram data. Try closing and reopening the panel.',
                    }, isDisposed);
                }
            } else {
                log('updateVisualization: no model data available, hiding loading state');
                await safePostMessage(panel, { command: 'hideLoading' }, isDisposed);
            }
        } catch (error) {
            if (!isWebviewDisposedError(error)) {
                logError('updateVisualization failed', error);
            }
            await safePostMessage(panel, { command: 'hideLoading' }, isDisposed);
        }
    }

    async function update(forceUpdate: boolean = false, triggerSource = 'unknown'): Promise<void> {
        if (isDisposed?.() || getIsNavigating()) {
            return;
        }

        const isBootstrapTrigger = triggerSource === 'webviewReady';

        const key = currentUpdateKey();
        if (inFlightUpdate && inFlightUpdate.key === key) {
            logPerf('visualizer:updateJoined', {
                triggerSource,
                joinedTriggerSource: inFlightUpdate.triggerSource,
                isBootstrap: inFlightUpdate.isBootstrap,
            });
            return await inFlightUpdate.promise;
        }

        if (!panel.visible && !forceUpdate) {
            setNeedsUpdateWhenVisible(true);
            return;
        }

        const contentHashSource = getContentHashSource
            ? getContentHashSource()
            : JSON.stringify({
                documentUri: getDocument?.()?.uri.toString() ?? null,
                documentVersion: getDocument?.()?.version ?? 0,
                workspaceRootUri: getWorkspaceRootUri(),
                currentView: getCurrentView(),
                selectedView: getSelectedView() ?? null,
            });
        const contentHash = hashContent(contentHashSource);

        const contentUnchanged = contentHash === getLastContentHash();
        const readiness = getVisualizerReadinessSnapshot(triggerSource);
        const isRedundantStartupRetry = triggerSource === 'startupRetry'
            && bootstrapCompleted
            && contentUnchanged;
        if ((!forceUpdate || isRedundantStartupRetry) && contentUnchanged) {
            return;
        }
        logPerf('visualizer:updateRequested', {
            triggerSource,
            forceUpdate,
            isBootstrap: !bootstrapCompleted,
            currentView: getCurrentView(),
        });
        setLastContentHash(contentHash);
        const promise = (async () => {
            setVisualizerUpdateInFlight(true);
            logPerf('visualizer:updateStarted', {
                triggerSource,
                isBootstrap: !bootstrapCompleted,
                currentView: getCurrentView(),
                suppressLoadingFlash: readiness.suppressLoadingFlash,
            });
            if (!readiness.suppressLoadingFlash) {
                await safePostMessage(panel, {
                    command: 'showLoading',
                    message: readiness.loadingMessage
                        ?? getLoadingMessage?.()
                        ?? loadingMessage
                        ?? 'Loading visualization...',
                }, isDisposed);
            }
            await new Promise(resolve => setTimeout(resolve, 0));
            const startedAt = Date.now();
            try {
                await doUpdateVisualization(triggerSource);
                bootstrapCompleted = true;
                setVisualizerBootstrapCompleted(true);
                logPerf('visualizer:updateCompleted', {
                    triggerSource,
                    isBootstrap: isBootstrapTrigger,
                    currentView: getCurrentView(),
                    totalMs: Date.now() - startedAt,
                });
            } finally {
                setVisualizerUpdateInFlight(false);
                if (inFlightUpdate?.key === key) {
                    inFlightUpdate = undefined;
                }
            }
        })();
        inFlightUpdate = {
            key,
            triggerSource,
            isBootstrap: !bootstrapCompleted,
            promise,
        };
        await promise;
    }

    return { update };
}

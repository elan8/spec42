/* eslint-disable @typescript-eslint/no-explicit-any */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
import { buildSharedRendererInput } from '../dtoAdapter';
import { SYSML_ENABLED_VIEWS, VIEW_OPTIONS } from './constants';
import { emptyStateTitleForData, resolveEmptyStateMessage } from './emptyStateHelpers';
import { postJumpToElement, type JumpToElementPayload } from './jumpToElement';
import { renderVisualizationEmptyState } from './renderers/placeholder';
import { prepareSharedViewData, renderSharedView, jumpPayloadFromNode } from './sharedRendererAdapter';
import { clearVisualHighlights } from './selectionSync';
import type { VisualizerContext } from './visualizerContext';
import {
    attachCanvasClickHandler,
    bindSharedCanvasRefs,
    destroySharedRenderController,
    ensureSharedCanvasZoom,
    ensureVisualizationCanvas,
} from './zoomController';

export async function renderVisualization(
    ctx: VisualizerContext,
    view: string,
    preserveZoomOverride: unknown = null,
    allowDuringResize = false,
): Promise<void> {
    void preserveZoomOverride;
    void allowDuringResize;

    if (!ctx.currentData) {
        return;
    }

    const {
        requestId: renderRequestId,
        abortController: renderAbortController,
        isStale: isStaleRender,
    } = ctx.renderScheduler.beginRender();

    const renderStartedAt = Date.now();

    if (ctx.renderScheduler.isRendering) {
        if (isStaleRender()) {
            return;
        }
        ctx.renderScheduler.queueRenderRequest({ view, preserveZoomOverride, allowDuringResize });
        ctx.webviewPerf('visualizer:webviewRenderQueued', {
            view,
            allowDuringResize,
        });
        return;
    }

    const viewChanged = view !== ctx.lastView;
    if (viewChanged) {
        window.userHasManuallyZoomed = false;
    }

    const baseData = ctx.filteredData || ctx.currentData;

    const hasSpecificPackageSelection =
        !!ctx.selectedDiagramName && ctx.selectedDiagramName !== 'All Packages';
    if (view === 'general-view') {
        ctx.webviewLog('info', '[GENERAL][render-start]', {
            selectedDiagramName: ctx.selectedDiagramName,
            selectedDiagramIndex: ctx.selectedDiagramIndex,
            hasSpecificPackageSelection,
            graphNodes: baseData?.graph?.nodes?.length || 0,
            graphEdges: baseData?.graph?.edges?.length || 0,
        });
    }

    const dataForPrepare = baseData;
    const prepareStartedAt = Date.now();
    const sharedInput = buildSharedRendererInput(dataForPrepare, view);
    const sharedRendererViewSet = new Set<string>(SYSML_ENABLED_VIEWS);
    const sharedPrepared = sharedRendererViewSet.has(view) && sharedInput
        ? prepareSharedViewData(sharedInput)
        : null;
    const prepareMs = Date.now() - prepareStartedAt;

    ctx.renderScheduler.markRendering();

    ctx.showLoading('Rendering ' + (VIEW_OPTIONS[view]?.label || view) + '...');

    let didFinishRender = false;
    const finishRender = () => {
        if (didFinishRender) return;
        didFinishRender = true;
        clearTimeout(renderSafetyTimeout);
        const { supersededByNewerRequest, nextRequest } = ctx.renderScheduler.finishRender(
            renderAbortController,
            renderRequestId,
        );
        ctx.hideLoading();
        ctx.webviewPerf(
            supersededByNewerRequest
                ? 'visualizer:webviewRenderSuperseded'
                : 'visualizer:webviewRenderCompleted',
            {
                view,
                prepareMs,
                totalMs: Date.now() - renderStartedAt,
            },
        );
        if (nextRequest) {
            setTimeout(() => {
                void renderVisualization(
                    ctx,
                    nextRequest.view,
                    nextRequest.preserveZoomOverride,
                    nextRequest.allowDuringResize,
                );
            }, 0);
        }
    };

    const renderSafetyTimeout = setTimeout(() => {
        ctx.webviewPerf('visualizer:webviewRenderSafetyTimeout', {
            view,
            prepareMs,
            elapsedMs: Date.now() - renderStartedAt,
        });
        finishRender();
    }, 10000);

    ctx.vizElement = document.getElementById('visualization');

    try {
        if (isStaleRender()) {
            finishRender();
            return;
        }
        ctx.webviewPerf('visualizer:webviewRenderStarted', {
            view,
            prepareMs,
            graphNodes: dataForPrepare?.graph?.nodes?.length || 0,
            graphEdges: dataForPrepare?.graph?.edges?.length || 0,
        });

        const width = document.getElementById('visualization')!.clientWidth;
        const height = document.getElementById('visualization')!.clientHeight;
        const sysmlSharedViews = new Set<string>(SYSML_ENABLED_VIEWS);
        const isSysmlSharedView = sysmlSharedViews.has(view);
        if (isSysmlSharedView && ctx.vizElement) {
            destroySharedRenderController(ctx, true);
            ctx.vizElement.innerHTML = '';
        }
        if (!isSysmlSharedView) {
            ensureVisualizationCanvas(ctx, width, height);
            ctx.g!.selectAll('*').remove();
        }

        if (!isSysmlSharedView) {
            attachCanvasClickHandler(ctx);
        }

        if (isSysmlSharedView) {
            const emptyStateMessage =
                resolveEmptyStateMessage(dataForPrepare) ?? resolveEmptyStateMessage(ctx.currentData);
            if (!ctx.vizElement) {
                finishRender();
                return;
            }
            if (emptyStateMessage) {
                renderVisualizationEmptyState(emptyStateMessage, {
                    viewLabel: emptyStateTitleForData(ctx.currentData, view),
                    data: ctx.currentData,
                });
                setTimeout(() => {
                    ctx.updateDimensionsDisplay();
                    finishRender();
                }, 100);
                ctx.lastView = view;
                return;
            }
            if (!sharedPrepared) {
                renderVisualizationEmptyState('Unable to prepare shared view data.', {
                    viewLabel: VIEW_OPTIONS[view]?.label || 'Shared Renderer',
                    data: dataForPrepare,
                });
                setTimeout(() => {
                    ctx.updateDimensionsDisplay();
                    finishRender();
                }, 100);
                return;
            }
            ctx.sharedRenderController = await renderSharedView(ctx.vizElement, sharedPrepared, {
                onNodeNavigate: (node: any) => {
                    clearVisualHighlights();
                    const parentContext = String(
                        sharedPrepared.meta?.parentContext ?? sharedPrepared.meta?.selectedDiagramName ?? '',
                    );
                    const payload = jumpPayloadFromNode(node, parentContext || undefined);
                    postJumpToElement(
                        (msg) => ctx.vscode.postMessage(msg),
                        {
                            name: payload.name,
                            id: payload.id,
                            uri: payload.uri,
                            range: payload.range as JumpToElementPayload['range'],
                        },
                        {
                            parentContext: payload.parentContext,
                            skipCentering: true,
                        },
                    );
                },
            });
            bindSharedCanvasRefs(ctx, ctx.vizElement);
            ensureSharedCanvasZoom(ctx, ctx.sharedRenderController.getFitTransform());
            attachCanvasClickHandler(ctx);
            setTimeout(() => {
                if (isStaleRender()) {
                    finishRender();
                    return;
                }
                ctx.updateDimensionsDisplay();
                finishRender();
            }, 100);
            ctx.lastView = view;
            return;
        } else {
            renderVisualizationEmptyState('The selected view is not yet implemented.', {
                viewLabel: 'Unknown View',
                data: dataForPrepare,
            });
            setTimeout(() => {
                ctx.updateDimensionsDisplay();
                finishRender();
            }, 100);
        }

        ctx.lastView = view;
    } catch (error) {
        if (isStaleRender()) {
            finishRender();
            return;
        }
        ctx.webviewPerf('visualizer:webviewRenderFailed', {
            view,
            prepareMs,
            totalMs: Date.now() - renderStartedAt,
            error: error instanceof Error ? error.message : String(error),
        });
        console.error('Error during rendering:', error);
        finishRender();

        const statusText = document.getElementById('status-text');
        if (statusText) {
            statusText.textContent =
                'Error rendering visualization: ' +
                (error instanceof Error ? error.message : String(error));
        }
    }
}

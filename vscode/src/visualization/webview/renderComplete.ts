import type { RenderOutcome } from '../renderContract';
import type { VisualizerContext } from './visualizerContext';

function hasExportableSvg(ctx: VisualizerContext): boolean {
    const exportPreview = ctx.exportHandler?.getSvgStringForExport?.();
    if (typeof exportPreview === 'string' && exportPreview.includes('<svg')) {
        return true;
    }
    const svgElement = document.querySelector('#visualization svg');
    const groupElement = svgElement?.querySelector('g');
    return !!(svgElement && groupElement && groupElement.childElementCount > 0);
}

export function postRenderComplete(
    ctx: VisualizerContext,
    view: string,
    outcome: RenderOutcome,
    supersededByNewerRequest: boolean,
): void {
    const resolvedOutcome: RenderOutcome = supersededByNewerRequest ? 'cancelled' : outcome;
    const exportable = !supersededByNewerRequest && outcome === 'diagram' && hasExportableSvg(ctx);
    ctx.lastRenderOutcome = resolvedOutcome;
    ctx.lastRenderHasExportableSvg = exportable;
    ctx.vscode.postMessage({
        command: 'renderComplete',
        updateId: ctx.lastUpdateId ?? undefined,
        view,
        dataHash: ctx.renderScheduler.dataHash,
        outcome: resolvedOutcome,
        graphNodes: ctx.currentData?.graph?.nodes?.length || 0,
        hasExportableSvg: exportable,
    });
}

export function replayLastRenderComplete(ctx: VisualizerContext, view: string): void {
    if (!ctx.lastRenderOutcome || ctx.lastRenderOutcome === 'cancelled') {
        return;
    }
    ctx.vscode.postMessage({
        command: 'renderComplete',
        updateId: ctx.lastUpdateId ?? undefined,
        view,
        dataHash: ctx.renderScheduler.dataHash,
        outcome: ctx.lastRenderOutcome,
        graphNodes: ctx.currentData?.graph?.nodes?.length || 0,
        hasExportableSvg: ctx.lastRenderHasExportableSvg,
    });
}

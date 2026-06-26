import type { VisualizerContext } from './visualizerContext';

export interface GlobalCommandBindings {
    changeView: (view: string) => void;
    resetZoom: () => void;
    zoomToFit: () => void;
}

export function installGlobalCommandBindings(
    ctx: VisualizerContext,
    bindings: GlobalCommandBindings,
): void {
    window.changeView = bindings.changeView;
    window.exportPNG = (scale) => ctx.exportHandler.exportPNG(scale);
    window.exportSVG = () => ctx.exportHandler.exportSVG();
    window.exportJSON = () => ctx.exportHandler.exportJSON();
    window.resetZoom = bindings.resetZoom;
    window.zoomToFit = bindings.zoomToFit;
}

import type * as D3 from 'd3';

export {};

declare global {
    interface Window {
        d3: typeof D3;
        __VIZ_INIT?: {
            elkWorkerUrl?: string;
            experimentalViews?: string[];
            verboseLogging?: boolean;
        };
        userHasManuallyZoomed?: boolean;
        changeView?: (view: string) => void;
        exportPNG?: (scale: number) => void;
        exportSVG?: () => void;
        exportJSON?: () => void;
        resetZoom?: () => void;
        zoomToFit?: () => void;
    }

    // Loaded via script tag before the webview bundle.
    const d3: typeof D3;
}

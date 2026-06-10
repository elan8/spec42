/* eslint-disable @typescript-eslint/no-explicit-any */
import type { Selection, ZoomTransform } from 'd3';
import { MIN_CANVAS_ZOOM, MAX_CANVAS_ZOOM, SYSML_ENABLED_VIEWS } from './constants';
import { clearVisualHighlights } from './selectionSync';
import type { VisualizerContext } from './visualizerContext';

export function setupSharedRendererWheelCapture(): void {
    const wrapper = document.getElementById('visualization-wrapper');
    if (!wrapper || wrapper.dataset.wheelCapture === '1') {
        return;
    }
    wrapper.dataset.wheelCapture = '1';
    wrapper.addEventListener(
        'wheel',
        (event) => {
            const viz = document.getElementById('visualization');
            if (!viz || !viz.contains(event.target as Node)) {
                return;
            }
            event.preventDefault();
        },
        { passive: false, capture: true },
    );
}

export function attachCanvasZoomHandlers(
    ctx: VisualizerContext,
    targetSvg: Selection<SVGSVGElement, unknown, null, undefined>,
): void {
    if (!ctx.zoom) {
        ctx.zoom = d3
            .zoom<SVGSVGElement, unknown>()
            .scaleExtent([MIN_CANVAS_ZOOM, MAX_CANVAS_ZOOM])
            .on('zoom', (event) => {
                if (!ctx.g || ctx.g.empty()) {
                    return;
                }
                ctx.g.attr('transform', event.transform);
                if (event.sourceEvent) {
                    window.userHasManuallyZoomed = true;
                }
            });
    }

    targetSvg
        .style('touch-action', 'none')
        .style('cursor', 'grab')
        .call(ctx.zoom)
        .on('dblclick.zoom', null)
        .on('wheel.zoom', function (event) {
            event.preventDefault();
            event.stopPropagation();

            window.userHasManuallyZoomed = true;

            const mouse = d3.pointer(event, this);
            const currentTransform = d3.zoomTransform(this);
            const factor = event.deltaY > 0 ? 0.7 : 1.45;
            const newScale = Math.min(
                Math.max(currentTransform.k * factor, MIN_CANVAS_ZOOM),
                MAX_CANVAS_ZOOM,
            );
            const translateX = mouse[0] - (mouse[0] - currentTransform.x) * (newScale / currentTransform.k);
            const translateY = mouse[1] - (mouse[1] - currentTransform.y) * (newScale / currentTransform.k);

            d3.select(this)
                .transition()
                .duration(50)
                .call(ctx.zoom!.transform, d3.zoomIdentity.translate(translateX, translateY).scale(newScale));
        });
}

export function ensureSharedCanvasZoom(
    ctx: VisualizerContext,
    fitTransform?: ZoomTransform | { toString: () => string },
): void {
    if (!ctx.svg || ctx.svg.empty() || !ctx.g || ctx.g.empty()) {
        return;
    }
    const initialTransform = (fitTransform ?? d3.zoomIdentity) as ZoomTransform;
    attachCanvasZoomHandlers(ctx, ctx.svg);
    ctx.svg.call(ctx.zoom!.transform, initialTransform);
}

export function ensureVisualizationCanvas(ctx: VisualizerContext, width: number, height: number): void {
    const root = d3.select('#visualization');

    if (!ctx.svg || ctx.svg.empty()) {
        ctx.svg = root.append('svg').attr('width', width).attr('height', height) as typeof ctx.svg;
    } else {
        ctx.svg.attr('width', width).attr('height', height);
    }

    ctx.g = ctx.svg.select('g.codex-render-root');
    if (ctx.g.empty()) {
        ctx.g = ctx.svg.append('g').attr('class', 'codex-render-root');
    }
    attachCanvasZoomHandlers(ctx, ctx.svg);
}

export function destroySharedRenderController(ctx: VisualizerContext, resetCanvasRefs = false): void {
    if (ctx.sharedRenderController) {
        ctx.sharedRenderController.destroy();
        ctx.sharedRenderController = null;
    }
    if (resetCanvasRefs) {
        ctx.svg = null;
        ctx.g = null;
        ctx.zoom = null;
    }
}

export function bindSharedCanvasRefs(ctx: VisualizerContext, container: HTMLElement): boolean {
    const svgNode = container.querySelector('svg.sysml-viz-svg');
    if (!svgNode) {
        return false;
    }
    const rootNode = svgNode.querySelector('g.viz-root');
    if (!rootNode) {
        return false;
    }
    ctx.svg = d3.select(svgNode) as typeof ctx.svg;
    ctx.g = d3.select(rootNode) as typeof ctx.g;
    return true;
}

export function attachCanvasClickHandler(ctx: VisualizerContext): void {
    if (!ctx.svg || ctx.svg.empty() || !ctx.g || ctx.g.empty()) {
        return;
    }
    ctx.svg.on('click', (event) => {
        if (event.target === ctx.svg!.node() || event.target === ctx.g!.node()) {
            clearVisualHighlights();
            ctx.g!.selectAll('.expanded-details').remove();
            ctx.g!.selectAll('.graph-node-background').each(function () {
                const el = d3.select(this);
                el.style('stroke', el.attr('data-original-stroke') || 'var(--vscode-panel-border)');
                el.style('stroke-width', el.attr('data-original-width') || '2px');
            });
            ctx.g!.selectAll('.node-group').classed('selected', false);
            ctx.g!.selectAll('.graph-node-group').classed('selected', false);
            ctx.g!.selectAll('.hierarchy-cell').classed('selected', false);
            ctx.g!.selectAll('.elk-node').classed('selected', false);
            ctx.g!.selectAll('.ibd-connector').each(function () {
                const el = d3.select(this);
                const origStroke = el.attr('data-original-stroke');
                const origWidth = el.attr('data-original-width');
                if (origStroke) {
                    el.style('stroke', origStroke)
                        .style('stroke-width', origWidth)
                        .classed('connector-highlighted', false);
                    el.attr('data-original-stroke', null).attr('data-original-width', null);
                }
            });
            ctx.g!.selectAll('.general-connector').each(function () {
                const el = d3.select(this);
                const origStroke = el.attr('data-original-stroke');
                const origWidth = el.attr('data-original-width');
                if (origStroke) {
                    el.style('stroke', origStroke)
                        .style('stroke-width', origWidth)
                        .classed('connector-highlighted', false);
                    el.attr('data-original-stroke', null).attr('data-original-width', null);
                }
            });
        }
    });
}

export function getHighlightedSvgBounds(ctx: VisualizerContext) {
    if (!ctx.g) {
        return null;
    }

    const highlighted = Array.from(
        ctx.g.node()!.querySelectorAll('.highlighted-element, .selected'),
    );
    if (highlighted.length === 0) {
        return null;
    }

    let minX = Infinity;
    let minY = Infinity;
    let maxX = -Infinity;
    let maxY = -Infinity;

    highlighted.forEach((element) => {
        const graphics = element as SVGGraphicsElement;
        if (!graphics || typeof graphics.getBBox !== 'function') {
            return;
        }
        try {
            const bbox = graphics.getBBox();
            if (!bbox || (bbox.width === 0 && bbox.height === 0)) {
                return;
            }
            minX = Math.min(minX, bbox.x);
            minY = Math.min(minY, bbox.y);
            maxX = Math.max(maxX, bbox.x + bbox.width);
            maxY = Math.max(maxY, bbox.y + bbox.height);
        } catch {
            return;
        }
    });

    if (!isFinite(minX) || !isFinite(minY) || !isFinite(maxX) || !isFinite(maxY)) {
        return null;
    }

    return {
        x: minX,
        y: minY,
        width: maxX - minX,
        height: maxY - minY,
    };
}

export function zoomToFit(ctx: VisualizerContext, trigger = 'user'): void {
    const isAuto = trigger === 'auto';
    if (!ctx.g || !ctx.svg) return;

    try {
        if (!isAuto) {
            window.userHasManuallyZoomed = true;
        }

        const selectionBounds = getHighlightedSvgBounds(ctx);
        const bounds = selectionBounds || ctx.g.node()!.getBBox();
        if (!bounds || bounds.width === 0 || bounds.height === 0) return;

        const svgNode = ctx.svg.node() as SVGSVGElement | null;
        const svgWidth = svgNode?.clientWidth || Number(ctx.svg.attr('width')) || 0;
        const svgHeight = svgNode?.clientHeight || Number(ctx.svg.attr('height')) || 0;
        if (!svgWidth || !svgHeight) {
            return;
        }

        const basePadding = selectionBounds ? 0.06 : 0.08;
        const padding = Math.min(svgWidth, svgHeight) * basePadding;

        const scaleX = (svgWidth - 2 * padding) / bounds.width;
        const scaleY = (svgHeight - 2 * padding) / bounds.height;
        const scale = Math.min(scaleX, scaleY);

        const maxScale = selectionBounds ? 3 : 1;
        const finalScale = Math.max(Math.min(scale, maxScale), MIN_CANVAS_ZOOM);

        const centerX = svgWidth / 2;
        const centerY = svgHeight / 2;
        const boundsX = bounds.x + bounds.width / 2;
        const boundsY = bounds.y + bounds.height / 2;

        const translateX = centerX - boundsX * finalScale;
        const translateY = centerY - boundsY * finalScale;

        ctx.svg
            .transition()
            .duration(750)
            .call(
                ctx.zoom!.transform,
                d3.zoomIdentity.translate(translateX, translateY).scale(finalScale),
            );
    } catch (error) {
        console.warn('Error in zoomToFit:', error);
        resetZoom(ctx);
    }
}

export function resetZoom(ctx: VisualizerContext): void {
    if (new Set<string>(SYSML_ENABLED_VIEWS).has(ctx.currentView) && ctx.sharedRenderController) {
        ctx.sharedRenderController.reset();
        ensureSharedCanvasZoom(ctx, ctx.sharedRenderController.getFitTransform());
        return;
    }
    zoomToFit(ctx, 'user');
}

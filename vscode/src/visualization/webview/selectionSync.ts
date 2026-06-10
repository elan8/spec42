/* eslint-disable @typescript-eslint/no-explicit-any */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
import type { BaseType, Selection } from 'd3';
import { DIAGRAM_STYLE } from './styleTokens';
import type { VisualizerContext } from './visualizerContext';

export function clearVisualHighlights(): void {
    d3.selectAll('.outline-highlighted').each(function () {
        const t = d3.select(this);
        t.classed('outline-highlighted', false);
        const origStroke = t.attr('data-original-stroke');
        const origWidth = t.attr('data-original-width');
        if (origStroke) {
            t.style('stroke', origStroke);
        } else {
            t.style('stroke', null);
        }
        if (origWidth) {
            t.style('stroke-width', origWidth);
        } else {
            t.style('stroke-width', null);
        }
    });

    d3.selectAll('.highlighted-element').classed('highlighted-element', false);
    d3.selectAll('.selected').classed('selected', false);

    d3.selectAll('.node-group').style('opacity', null);
    d3.selectAll('.node-group .node-background').each(function () {
        const el = d3.select(this);
        el.style('stroke', el.attr('data-original-stroke') || 'var(--vscode-panel-border)');
        el.style('stroke-width', el.attr('data-original-width') || '1px');
    });
    d3.selectAll('.general-node .node-background').each(function () {
        const el = d3.select(this);
        el.style('stroke', el.attr('data-original-stroke') || 'var(--vscode-panel-border)');
        el.style('stroke-width', el.attr('data-original-width') || '2px');
    });
    d3.selectAll('.state-node .node-background').each(function () {
        const el = d3.select(this);
        el.style('stroke', el.attr('data-original-stroke') || 'var(--vscode-panel-border)');
        el.style('stroke-width', el.attr('data-original-width') || '2px');
    });
    d3.selectAll('.activity-action .node-background').each(function () {
        const el = d3.select(this);
        el.style('stroke', el.attr('data-original-stroke') || 'var(--vscode-panel-border)');
        el.style('stroke-width', el.attr('data-original-width') || '2px');
    });
    d3.selectAll('.ibd-part rect:first-child').each(function () {
        const el = d3.select(this);
        const orig = el.attr('data-original-stroke');
        if (orig) {
            el.style('stroke', orig);
            el.style('stroke-width', el.attr('data-original-width') || '2px');
        }
    });
    d3.selectAll('.graph-node-group').style('opacity', null);
    d3.selectAll('.hierarchy-cell').style('opacity', null);
}

export function highlightElementInVisualization(
    ctx: VisualizerContext,
    elementName: string,
    skipCentering = false,
): void {
    clearVisualHighlights();

    let targetElement: Selection<BaseType, unknown, null, undefined> | null = null;
    let elementData: { name: string; type: string } | null = null;

    if (ctx.currentView === 'general-view') {
        d3.selectAll('.general-node').each(function () {
            const node = d3.select(this);
            const nodeName = node.attr('data-element-name');
            if (nodeName === elementName) {
                targetElement = node;
                elementData = { name: elementName, type: 'element' };
            }
        });
    } else if (ctx.currentView === 'interconnection-view') {
        d3.selectAll('.ibd-part').each(function () {
            const partG = d3.select(this);
            const partName = partG.attr('data-element-name');
            if (partName === elementName) {
                targetElement = partG;
                elementData = { name: elementName, type: 'part' };
            }
        });
    } else if (ctx.currentView === 'state-transition-view') {
        d3.selectAll('.state-node').each(function () {
            const stateNode = d3.select(this);
            const stateName = stateNode.attr('data-element-name');
            if (stateName === elementName) {
                targetElement = stateNode;
                elementData = { name: elementName, type: 'state' };
            }
        });
    }

    if (targetElement && elementData) {
        targetElement.classed('highlighted-element', true);

        targetElement
            .select('.node-background')
            .style('stroke', DIAGRAM_STYLE.highlight)
            .style('stroke-width', '3px');
        targetElement
            .select('rect')
            .style('stroke', DIAGRAM_STYLE.highlight)
            .style('stroke-width', '3px');

        const statusBar = document.getElementById('status-bar');
        const statusText = document.getElementById('status-text');
        if (statusText)
            statusText.textContent = 'Selected: ' + elementData.name + ' [' + elementData.type + ']';
        if (statusBar) statusBar.style.display = 'flex';

        if (!skipCentering) {
            const targetNode = targetElement.node() as Element | null;
            const svgNode = ctx.svg?.node();
            if (!targetNode || !svgNode || !ctx.zoom) {
                return;
            }
            const targetRect = targetNode.getBoundingClientRect();
            const svgRect = svgNode.getBoundingClientRect();
            const targetCenterViewportX = targetRect.left - svgRect.left + targetRect.width / 2;
            const targetCenterViewportY = targetRect.top - svgRect.top + targetRect.height / 2;

            const transform = d3.zoomTransform(svgNode);
            const [centerX, centerY] = transform.invert([targetCenterViewportX, targetCenterViewportY]);
            const scale = Math.min(1.5, transform.k);
            const translateX = svgNode.clientWidth / 2 - centerX * scale;
            const translateY = svgNode.clientHeight / 2 - centerY * scale;

            ctx.svg
                ?.transition()
                .duration(750)
                .call(
                    ctx.zoom.transform,
                    d3.zoomIdentity.translate(translateX, translateY).scale(scale),
                );
        }
    }
}

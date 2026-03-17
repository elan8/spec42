/**
 * General View renderer - D3 + elkjs with SysML v2 compartment nodes.
 * Uses shared sysmlNodeBuilder for Header, Attributes, Parts, Ports compartments.
 */
/* eslint-disable @typescript-eslint/no-explicit-any */

import type { RenderContext } from '../types';
import { GENERAL_VIEW_PALETTE } from '../constants';
import { postJumpToElement } from '../jumpToElement';
import { formatSysMLStereotype } from '../shared';
import { getTypeColor } from '../shared';
import {
    collectCompartmentsFromElement,
    computeNodeHeightFromCompartments,
    renderSysMLNode,
    type SysMLNodeCompartments,
    type SysMLNodeConfig
} from './sysmlNodeBuilder';

declare const d3: any;
declare const ELK: any;

const NODE_WIDTH = 200;
const NODE_HEIGHT_BASE = 70;

/** General view uses full SysML v2 compartments: Header, Attributes, Parts, Ports, Other */
const GENERAL_VIEW_NODE_CONFIG: SysMLNodeConfig = {
    showHeader: true,
    showAttributes: true,
    showParts: true,
    showPorts: true,
    showOther: true,
    maxLinesPerCompartment: 8
};

export interface GeneralViewContext extends RenderContext {
    buildGeneralViewGraph: (data: any) => { elements: any[]; typeStats: Record<string, number>; packageGroups: Array<{ id: string; label: string; nodeIds: string[]; depth: number }> };
    renderGeneralChips: (typeStats: Record<string, number>) => void;
    elkWorkerUrl: string;
}

function computeOrthogonalPath(
    x1: number, y1: number, x2: number, y2: number,
    options: { offset?: number; srcRect?: any; tgtRect?: any } = {}
): { pathD: string; labelX: number; labelY: number } {
    const offset = options.offset ?? 0;
    const srcRect = options.srcRect;
    const tgtRect = options.tgtRect;

    const srcCx = srcRect ? srcRect.x + srcRect.width / 2 : x1;
    const srcCy = srcRect ? srcRect.y + srcRect.height / 2 : y1;
    const tgtCx = tgtRect ? tgtRect.x + tgtRect.width / 2 : x2;
    const tgtCy = tgtRect ? tgtRect.y + tgtRect.height / 2 : y2;

    const dx = tgtCx - srcCx;
    const dy = tgtCy - srcCy;

    let ox1 = x1, oy1 = y1, ox2 = x2, oy2 = y2;
    if (srcRect) {
        if (Math.abs(dx) > Math.abs(dy)) {
            ox1 = dx > 0 ? srcRect.x + srcRect.width : srcRect.x;
            oy1 = srcCy + offset;
        } else {
            ox1 = srcCx + offset;
            oy1 = dy > 0 ? srcRect.y + srcRect.height : srcRect.y;
        }
    }
    if (tgtRect) {
        if (Math.abs(dx) > Math.abs(dy)) {
            ox2 = dx > 0 ? tgtRect.x : tgtRect.x + tgtRect.width;
            oy2 = tgtCy + offset;
        } else {
            ox2 = tgtCx + offset;
            oy2 = dy > 0 ? tgtRect.y : tgtRect.y + tgtRect.height;
        }
    }

    const distX = Math.abs(ox2 - ox1);
    const distY = Math.abs(oy2 - oy1);
    const wpSpread = offset * 0.4;

    let pathD: string;
    let labelX: number, labelY: number;

    if (distX > distY) {
        const midX = (ox1 + ox2) / 2 + wpSpread;
        pathD = 'M' + ox1 + ',' + oy1 + ' L' + midX + ',' + oy1 + ' L' + midX + ',' + oy2 + ' L' + ox2 + ',' + oy2;
        labelX = midX;
        labelY = (oy1 + oy2) / 2 - 8 + offset * 0.5;
    } else {
        const midY = (oy1 + oy2) / 2 + wpSpread;
        pathD = 'M' + ox1 + ',' + oy1 + ' L' + ox1 + ',' + midY + ' L' + ox2 + ',' + midY + ' L' + ox2 + ',' + oy2;
        labelX = (ox1 + ox2) / 2 + offset * 0.5;
        labelY = midY - 8;
    }
    return { pathD, labelX, labelY };
}

/**
 * Build SVG path from ELK edge sections (startPoint, endPoint, bendPoints).
 * Returns null if sections are missing or invalid.
 */
function pathFromElkSections(sections: Array<{ startPoint?: { x: number; y: number }; endPoint?: { x: number; y: number }; bendPoints?: Array<{ x: number; y: number }> }> | undefined): string | null {
    if (!sections || sections.length === 0) return null;
    const parts: string[] = [];
    for (const sec of sections) {
        const sp = sec.startPoint;
        const ep = sec.endPoint;
        const bp = sec.bendPoints || [];
        if (!sp || !ep) return null;
        parts.push('M' + sp.x + ',' + sp.y);
        for (const p of bp) {
            parts.push('L' + p.x + ',' + p.y);
        }
        parts.push('L' + ep.x + ',' + ep.y);
    }
    return parts.join(' ');
}

export async function renderGeneralViewD3(ctx: GeneralViewContext, data: any): Promise<void> {
    const { width, height, svg, g, postMessage, renderPlaceholder, clearVisualHighlights } = ctx;

    const result = ctx.buildGeneralViewGraph(data);
    const { elements, typeStats, packageGroups } = result;

    ctx.renderGeneralChips(typeStats);

    const cyNodes = elements.filter((el: any) => el.group === 'nodes');
    const cyEdges = elements.filter((el: any) => el.group === 'edges');

    if (cyNodes.length === 0) {
        renderPlaceholder(width, height, 'General View',
            'No matching elements to display.\\n\\nTry another General View preset above.',
            data);
        return;
    }

    if (typeof ELK === 'undefined') {
        renderPlaceholder(width, height, 'General View',
            'ELK layout library not loaded. Please refresh the view.',
            data);
        return;
    }

    let elk: any;
    try {
        elk = new ELK({ workerUrl: ctx.elkWorkerUrl || undefined });
    } catch (e) {
        console.warn('[General View] ELK worker init failed, layout may be unavailable:', e);
    }

    const nodeWidth = NODE_WIDTH;

    const nodeDataMap = new Map<string, { compartments: SysMLNodeCompartments; height: number }>();
    cyNodes.forEach((el: any) => {
        const d = el.data;
        const element = d.element;
        const compartments = element
            ? collectCompartmentsFromElement(element)
            : {
                header: {
                    stereotype: (d.sysmlType || 'element').toLowerCase(),
                    name: (d.elementName || d.label || d.baseLabel || 'Unnamed').toString()
                },
                typedByName: null,
                attributes: [],
                parts: [],
                ports: [],
                other: []
            };
        const nodeHeight = computeNodeHeightFromCompartments(compartments, GENERAL_VIEW_NODE_CONFIG, NODE_WIDTH);
        nodeDataMap.set(d.id, { compartments, height: Math.max(NODE_HEIGHT_BASE, nodeHeight) });
    });

    // Build per-node port indices: each edge gets its own connection point to avoid overlap.
    const outgoingByNode = new Map<string, { edge: any; idx: number }[]>();
    const incomingByNode = new Map<string, { edge: any; idx: number }[]>();
    cyEdges.forEach((edge: any, idx: number) => {
        const src = edge.data.source;
        const tgt = edge.data.target;
        if (!outgoingByNode.has(src)) outgoingByNode.set(src, []);
        outgoingByNode.get(src)!.push({ edge, idx });
        if (!incomingByNode.has(tgt)) incomingByNode.set(tgt, []);
        incomingByNode.get(tgt)!.push({ edge, idx });
    });
    const getOutgoingPortIndex = (nodeId: string, edge: any) => {
        const list = outgoingByNode.get(nodeId) || [];
        const i = list.findIndex((x) => x.edge === edge);
        return i >= 0 ? i : 0;
    };
    const getIncomingPortIndex = (nodeId: string, edge: any) => {
        const list = incomingByNode.get(nodeId) || [];
        const i = list.findIndex((x) => x.edge === edge);
        return i >= 0 ? i : 0;
    };

    // Layout uses ALL edges (hierarchy + typing) so ELK positions the full tree correctly.
    // Multiple ports per side (north_0..north_k, south_0..south_m) distribute edges and reduce overlap.
    const elkGraph = {
        id: 'root',
        layoutOptions: {
            'elk.algorithm': 'layered',
            'elk.direction': 'DOWN',
            'elk.spacing.nodeNode': '220',
            'elk.layered.spacing.nodeNodeBetweenLayers': '280',
            'elk.spacing.edgeNode': '120',
            'elk.spacing.edgeEdge': '120',
            'elk.edgeRouting': 'ORTHOGONAL',
            'elk.layered.nodePlacement.strategy': 'NETWORK_SIMPLEX',
            'elk.separateConnectedComponents': 'true',
            'elk.aspectRatio': '1.4',
            'elk.padding': '[top=100,left=100,bottom=100,right=100]',
            'org.eclipse.elk.portConstraints': 'FIXED_SIDE',
            'org.eclipse.elk.spacing.portPort': '15',
            'org.eclipse.elk.json.edgeCoords': 'ROOT'
        },
        children: cyNodes.map((el: any) => {
            const nodeId = el.data.id;
            const nd = nodeDataMap.get(nodeId);
            const nodeHeight = nd?.height ?? NODE_HEIGHT_BASE;
            const outCount = outgoingByNode.get(nodeId)?.length ?? 0;
            const inCount = incomingByNode.get(nodeId)?.length ?? 0;
            const ports: { id: string; layoutOptions: Record<string, string> }[] = [];
            for (let i = 0; i < Math.max(outCount, 1); i++) {
                ports.push({
                    id: nodeId + '_south_' + i,
                    layoutOptions: { 'org.eclipse.elk.port.side': 'SOUTH' }
                });
            }
            for (let i = 0; i < Math.max(inCount, 1); i++) {
                ports.push({
                    id: nodeId + '_north_' + i,
                    layoutOptions: { 'org.eclipse.elk.port.side': 'NORTH' }
                });
            }
            return {
                id: nodeId,
                width: nodeWidth,
                height: nodeHeight,
                ports
            };
        }),
        edges: cyEdges.map((el: any, idx: number) => {
            const src = el.data.source;
            const tgt = el.data.target;
            const srcPort = src + '_south_' + getOutgoingPortIndex(src, el);
            const tgtPort = tgt + '_north_' + getIncomingPortIndex(tgt, el);
            return {
                id: el.data.id || ('edge-' + idx),
                sources: [srcPort],
                targets: [tgtPort]
            };
        })
    };

    let laidOut: any;
    try {
        laidOut = elk ? await elk.layout(elkGraph) : null;
    } catch (e) {
        console.error('[General View] ELK layout failed:', e);
    }

    const nodePositions = new Map<string, { x: number; y: number; width: number; height: number }>();
    if (laidOut && laidOut.children) {
        laidOut.children.forEach((child: any) => {
            const nd = nodeDataMap.get(child.id);
            nodePositions.set(child.id, {
                x: child.x ?? 0,
                y: child.y ?? 0,
                width: child.width ?? nodeWidth,
                height: child.height ?? nd?.height ?? NODE_HEIGHT_BASE
            });
        });
    } else {
        let x = 80, y = 80;
        cyNodes.forEach((el: any, i: number) => {
            const nd = nodeDataMap.get(el.data.id);
            const h = nd?.height ?? NODE_HEIGHT_BASE;
            nodePositions.set(el.data.id, { x, y, width: nodeWidth, height: h });
            x += nodeWidth + 60;
            if (x > width - nodeWidth - 80) {
                x = 80;
                y += h + 80;
            }
        });
    }

    g.selectAll('*').remove();

    const defs = svg.select('defs').empty() ? svg.append('defs') : svg.select('defs');
    defs.selectAll('#general-d3-arrow').remove();
    defs.append('marker')
        .attr('id', 'general-d3-arrow')
        .attr('viewBox', '0 -5 10 10')
        .attr('refX', 8)
        .attr('refY', 0)
        .attr('markerWidth', 5)
        .attr('markerHeight', 5)
        .attr('orient', 'auto')
        .append('path')
        .attr('d', 'M0,-4L10,0L0,4')
        .style('fill', 'var(--vscode-charts-blue)');

    defs.selectAll('#general-d3-specializes').remove();
    defs.append('marker')
        .attr('id', 'general-d3-specializes')
        .attr('viewBox', '0 -6 12 12')
        .attr('refX', 11)
        .attr('refY', 0)
        .attr('markerWidth', 8)
        .attr('markerHeight', 8)
        .attr('orient', 'auto')
        .append('path')
        .attr('d', 'M0,0L10,-4L10,4Z')
        .style('fill', GENERAL_VIEW_PALETTE.structural.port);

    const edgeGroup = g.append('g').attr('class', 'general-edges');
    const nodeGroup = g.append('g').attr('class', 'general-nodes');
    const packageGroup = g.append('g').attr('class', 'general-packages');

    packageGroups
        .sort((a, b) => a.depth - b.depth)
        .forEach((group) => {
            const memberPositions = group.nodeIds
                .map((nodeId) => nodePositions.get(nodeId))
                .filter(Boolean) as Array<{ x: number; y: number; width: number; height: number }>;
            if (memberPositions.length === 0) return;
            const paddingX = 28;
            const paddingTop = 36;
            const paddingBottom = 22;
            const minX = Math.min(...memberPositions.map((pos) => pos.x)) - paddingX;
            const minY = Math.min(...memberPositions.map((pos) => pos.y)) - paddingTop;
            const maxX = Math.max(...memberPositions.map((pos) => pos.x + pos.width)) + paddingX;
            const maxY = Math.max(...memberPositions.map((pos) => pos.y + pos.height)) + paddingBottom;
            const tone = group.depth === 1 ? 0.08 : 0.05;
            packageGroup.append('rect')
                .attr('x', minX)
                .attr('y', minY)
                .attr('width', maxX - minX)
                .attr('height', maxY - minY)
                .attr('rx', 18)
                .style('fill', 'color-mix(in srgb, var(--vscode-editor-inactiveSelectionBackground) ' + Math.round(tone * 100) + '%, transparent)')
                .style('stroke', 'var(--vscode-panel-border)')
                .style('stroke-width', group.depth === 1 ? '1.5px' : '1px')
                .style('stroke-dasharray', group.depth === 1 ? 'none' : '6,4')
                .style('opacity', 0.9);
            packageGroup.append('text')
                .attr('x', minX + 14)
                .attr('y', minY + 21)
                .text(group.label)
                .style('font-size', '11px')
                .style('font-weight', '700')
                .style('fill', 'var(--vscode-descriptionForeground)');
        });

    const laidOutEdges = laidOut?.edges ?? [];
    // For typing edges to same target, assign offsets to reduce overlap
    const typingByTarget = new Map<string, Array<{ el: any; idx: number }>>();
    cyEdges.forEach((el: any, idx: number) => {
        const t = (el.data.type || el.data.relType || '').toLowerCase();
        if (t === 'typing') {
            const tgt = el.data.target;
            if (!typingByTarget.has(tgt)) typingByTarget.set(tgt, []);
            typingByTarget.get(tgt)!.push({ el, idx });
        }
    });
    cyEdges.forEach((el: any, edgeIdx: number) => {
        const srcPos = nodePositions.get(el.data.source);
        const tgtPos = nodePositions.get(el.data.target);
        if (!srcPos || !tgtPos) return;

        const relType = (el.data.relType || el.data.type || 'relationship').toLowerCase();
        const elkEdge = laidOutEdges[edgeIdx];

        let pathD: string;
        if (elkEdge?.sections) {
            const elkPath = pathFromElkSections(elkEdge.sections);
            pathD = elkPath ?? computeOrthogonalPath(0, 0, 0, 0, {
                srcRect: { x: srcPos.x, y: srcPos.y, width: srcPos.width, height: srcPos.height },
                tgtRect: { x: tgtPos.x, y: tgtPos.y, width: tgtPos.width, height: tgtPos.height }
            }).pathD;
        } else {
            let offset = 0;
            if (relType === 'typing') {
                const group = typingByTarget.get(el.data.target) || [];
                const rank = group.findIndex((x) => x.el === el);
                if (rank >= 0 && group.length > 1) {
                    offset = (rank - (group.length - 1) / 2) * 18;
                }
            }
            const { pathD: fallbackPath } = computeOrthogonalPath(0, 0, 0, 0, {
                srcRect: { x: srcPos.x, y: srcPos.y, width: srcPos.width, height: srcPos.height },
                tgtRect: { x: tgtPos.x, y: tgtPos.y, width: tgtPos.width, height: tgtPos.height },
                offset
            });
            pathD = fallbackPath;
        }

        let strokeColor = GENERAL_VIEW_PALETTE.other.default;
        let strokeDash = 'none';
        let markerEnd = 'url(#general-d3-arrow)';
        let strokeWidth = '2px';

        if (relType === 'specializes') {
            strokeColor = GENERAL_VIEW_PALETTE.structural.port;
            markerEnd = 'url(#general-d3-specializes)';
        } else if (relType === 'typing') {
            strokeColor = GENERAL_VIEW_PALETTE.requirements.requirement;
            strokeDash = '5,3';
        } else if (relType === 'hierarchy' || relType === 'contains') {
            strokeColor = GENERAL_VIEW_PALETTE.structural.part;
        } else if (relType === 'connection' || relType === 'connect') {
            strokeColor = GENERAL_VIEW_PALETTE.structural.interface;
        } else if (relType === 'bind' || relType === 'binding') {
            strokeColor = '#808080';
            strokeDash = '2,2';
            markerEnd = 'none';
        } else if (relType === 'allocate' || relType === 'allocation') {
            strokeColor = GENERAL_VIEW_PALETTE.other.allocation;
            strokeDash = '8,4';
        }

        edgeGroup.append('path')
            .attr('d', pathD)
            .attr('class', 'general-connector')
            .attr('data-source', el.data.source)
            .attr('data-target', el.data.target)
            .attr('data-type', relType)
            .style('fill', 'none')
            .style('stroke', strokeColor)
            .style('stroke-width', strokeWidth)
            .style('stroke-dasharray', strokeDash)
            .style('opacity', 0.85)
            .style('marker-end', markerEnd)
            .style('cursor', 'pointer');
    });

    const statusEl = document.getElementById('status-text');
    if (statusEl) statusEl.textContent = 'General View • Tap element to highlight, double-tap to jump';

    let lastTappedId: string | null = null;
    let tapTimeout: ReturnType<typeof setTimeout> | null = null;

    cyNodes.forEach((el: any) => {
        const pos = nodePositions.get(el.data.id);
        if (!pos) return;

        const d = el.data;
        const nd = nodeDataMap.get(d.id);
        const compartments = nd?.compartments ?? {
            header: { stereotype: (d.sysmlType || 'element').toLowerCase(), name: (d.elementName || d.label || 'Unnamed').toString() },
            typedByName: null,
            attributes: [],
            parts: [],
            ports: [],
            other: []
        };

        const isDefinition = d.isDefinition === true;
        const typeColor = d.color || getTypeColor(d.sysmlType);

        const nodeG = renderSysMLNode(nodeGroup, compartments, {
            x: pos.x,
            y: pos.y,
            width: pos.width,
            height: pos.height,
            config: GENERAL_VIEW_NODE_CONFIG,
            isDefinition,
            typeColor,
            formatStereotype: (t) => formatSysMLStereotype(t) || ('«' + t + '»'),
            nodeClass: 'general-node elk-node',
            dataElementName: d.elementName || d.label
        });

        nodeG.on('click', function (event: any) {
            event.stopPropagation();
            clearVisualHighlights();
            g.selectAll('.general-node').select('.graph-node-background').each(function (this: any) {
                const r = d3.select(this);
                r.style('stroke', r.attr('data-original-stroke'))
                    .style('stroke-width', r.attr('data-original-width'));
            });
            nodeG.select('.graph-node-background')
                .style('stroke', '#FFD700')
                .style('stroke-width', '4px');

            const statusEl = document.getElementById('status-text');
            if (statusEl) statusEl.textContent = (d.label || d.elementName) + ' [' + (d.sysmlType || 'element') + ']';

            const elementName = d.elementName;
            const elementQualifiedName = d.elementQualifiedName || elementName;
            const nodeId = d.id;
            if (elementName) {
                if (lastTappedId === nodeId && tapTimeout) {
                    clearTimeout(tapTimeout);
                    tapTimeout = null;
                    lastTappedId = null;
                    postJumpToElement(postMessage, { name: elementName, id: elementQualifiedName || undefined });
                } else {
                    lastTappedId = nodeId;
                    tapTimeout = setTimeout(() => {
                        tapTimeout = null;
                        lastTappedId = null;
                    }, 250);
                }
            }
        });
    });
}

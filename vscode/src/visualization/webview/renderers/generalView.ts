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
import { DIAGRAM_STYLE } from '../styleTokens';
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
const GENERAL_NEUTRAL_EDGE = 'var(--vscode-editor-foreground)';
const GENERAL_NEUTRAL_BORDER = DIAGRAM_STYLE.nodeBorder;

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

type Side = 'NORTH' | 'SOUTH' | 'EAST' | 'WEST';

function segmentIntersectsRect(
    a: { x: number; y: number },
    b: { x: number; y: number },
    rect: { x: number; y: number; width: number; height: number }
): boolean {
    const rx1 = rect.x;
    const ry1 = rect.y;
    const rx2 = rect.x + rect.width;
    const ry2 = rect.y + rect.height;
    if (a.x === b.x) {
        const x = a.x;
        if (x <= rx1 || x >= rx2) return false;
        const sy1 = Math.min(a.y, b.y);
        const sy2 = Math.max(a.y, b.y);
        return sy2 > ry1 && sy1 < ry2;
    }
    if (a.y === b.y) {
        const y = a.y;
        if (y <= ry1 || y >= ry2) return false;
        const sx1 = Math.min(a.x, b.x);
        const sx2 = Math.max(a.x, b.x);
        return sx2 > rx1 && sx1 < rx2;
    }
    return false;
}

function scorePathAgainstObstacles(
    points: Array<{ x: number; y: number }>,
    obstacles: Array<{ x: number; y: number; width: number; height: number }>
): number {
    let score = 0;
    for (let i = 0; i < points.length - 1; i++) {
        for (const rect of obstacles) {
            if (segmentIntersectsRect(points[i], points[i + 1], rect)) score += 1;
        }
    }
    return score;
}

function pointsToPath(points: Array<{ x: number; y: number }>): string {
    if (points.length === 0) return '';
    const parts = ['M' + points[0].x + ',' + points[0].y];
    for (let i = 1; i < points.length; i++) {
        parts.push('L' + points[i].x + ',' + points[i].y);
    }
    return parts.join(' ');
}

function buildPortPoint(
    rect: { x: number; y: number; width: number; height: number },
    side: Side
): { x: number; y: number } {
    if (side === 'NORTH') return { x: rect.x + rect.width / 2, y: rect.y };
    if (side === 'SOUTH') return { x: rect.x + rect.width / 2, y: rect.y + rect.height };
    if (side === 'EAST') return { x: rect.x + rect.width, y: rect.y + rect.height / 2 };
    return { x: rect.x, y: rect.y + rect.height / 2 };
}

function extendFromPort(p: { x: number; y: number }, side: Side, amount: number): { x: number; y: number } {
    if (side === 'NORTH') return { x: p.x, y: p.y - amount };
    if (side === 'SOUTH') return { x: p.x, y: p.y + amount };
    if (side === 'EAST') return { x: p.x + amount, y: p.y };
    return { x: p.x - amount, y: p.y };
}

function chooseSide(from: { x: number; y: number }, to: { x: number; y: number }): Side {
    const dx = to.x - from.x;
    const dy = to.y - from.y;
    if (Math.abs(dx) >= Math.abs(dy)) {
        return dx >= 0 ? 'EAST' : 'WEST';
    }
    return dy >= 0 ? 'SOUTH' : 'NORTH';
}

function routeOrthogonalAvoiding(
    srcRect: { x: number; y: number; width: number; height: number },
    tgtRect: { x: number; y: number; width: number; height: number },
    obstacles: Array<{ x: number; y: number; width: number; height: number }>
): string {
    const srcCenter = { x: srcRect.x + srcRect.width / 2, y: srcRect.y + srcRect.height / 2 };
    const tgtCenter = { x: tgtRect.x + tgtRect.width / 2, y: tgtRect.y + tgtRect.height / 2 };
    const srcSide = chooseSide(srcCenter, tgtCenter);
    const tgtSide = chooseSide(tgtCenter, srcCenter);
    const stub = 24;
    const srcPort = buildPortPoint(srcRect, srcSide);
    const tgtPort = buildPortPoint(tgtRect, tgtSide);
    const srcOut = extendFromPort(srcPort, srcSide, stub);
    const tgtOut = extendFromPort(tgtPort, tgtSide, stub);

    const candidates: Array<Array<{ x: number; y: number }>> = [];
    const midpoint = { x: (srcOut.x + tgtOut.x) / 2, y: (srcOut.y + tgtOut.y) / 2 };
    candidates.push([srcPort, srcOut, { x: tgtOut.x, y: srcOut.y }, tgtOut, tgtPort]);
    candidates.push([srcPort, srcOut, { x: srcOut.x, y: tgtOut.y }, tgtOut, tgtPort]);
    candidates.push([srcPort, srcOut, { x: midpoint.x, y: srcOut.y }, { x: midpoint.x, y: tgtOut.y }, tgtOut, tgtPort]);
    candidates.push([srcPort, srcOut, { x: srcOut.x, y: midpoint.y }, { x: tgtOut.x, y: midpoint.y }, tgtOut, tgtPort]);

    let best = candidates[0];
    let bestScore = Number.MAX_SAFE_INTEGER;
    for (const c of candidates) {
        const score = scorePathAgainstObstacles(c, obstacles);
        if (score < bestScore) {
            best = c;
            bestScore = score;
            if (score === 0) break;
        }
    }
    return pointsToPath(best);
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
            'No matching elements to display.',
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

    const topPackageOf = (node: any): string =>
        (Array.isArray(node?.data?.packagePath) && node.data.packagePath.length > 0
            ? String(node.data.packagePath[0])
            : 'Unscoped');

    const packageNodeMap = new Map<string, any[]>();
    cyNodes.forEach((node: any) => {
        const pkg = topPackageOf(node);
        if (!packageNodeMap.has(pkg)) packageNodeMap.set(pkg, []);
        packageNodeMap.get(pkg)!.push(node);
    });
    const packageOrder = [...packageNodeMap.keys()].sort((a, b) => a.localeCompare(b));

    const nodePositions = new Map<string, { x: number; y: number; width: number; height: number }>();
    const topPackageBounds = new Map<string, { x: number; y: number; width: number; height: number }>();
    const internalEdgesToRender: any[] = [];

    async function layoutPackage(nodesInPkg: any[], edgesInPkg: any[]) {
        const outgoingByNode = new Map<string, { edge: any; idx: number }[]>();
        const incomingByNode = new Map<string, { edge: any; idx: number }[]>();
        edgesInPkg.forEach((edge: any, idx: number) => {
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
            children: nodesInPkg.map((el: any) => {
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
                return { id: nodeId, width: nodeWidth, height: nodeHeight, ports };
            }),
            edges: edgesInPkg.map((el: any, idx: number) => {
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

        try {
            return elk ? await elk.layout(elkGraph) : null;
        } catch (e) {
            console.error('[General View] ELK package layout failed:', e);
            return null;
        }
    }

    // Place each top-level package as an independent container, with its own layout.
    const packageGapY = 220;
    let cursorX = 80;
    let cursorY = 80;
    const fixedPackageLaneX = 80;
    for (const pkg of packageOrder) {
        const nodesInPkg = packageNodeMap.get(pkg) || [];
        const nodeIdSet = new Set(nodesInPkg.map((n: any) => n.data.id));
        const edgesInPkg = cyEdges.filter((e: any) =>
            nodeIdSet.has(e.data.source) && nodeIdSet.has(e.data.target)
        );
        internalEdgesToRender.push(...edgesInPkg);

        const laidOut = await layoutPackage(nodesInPkg, edgesInPkg);
        const localPositions = new Map<string, { x: number; y: number; width: number; height: number }>();
        if (laidOut && laidOut.children) {
            laidOut.children.forEach((child: any) => {
                const nd = nodeDataMap.get(child.id);
                localPositions.set(child.id, {
                    x: child.x ?? 0,
                    y: child.y ?? 0,
                    width: child.width ?? nodeWidth,
                    height: child.height ?? nd?.height ?? NODE_HEIGHT_BASE
                });
            });
        } else {
            let lx = 40;
            let ly = 40;
            nodesInPkg.forEach((el: any) => {
                const nd = nodeDataMap.get(el.data.id);
                const h = nd?.height ?? NODE_HEIGHT_BASE;
                localPositions.set(el.data.id, { x: lx, y: ly, width: nodeWidth, height: h });
                lx += nodeWidth + 60;
                if (lx > 1000) {
                    lx = 40;
                    ly += h + 80;
                }
            });
        }

        const allLocal = [...localPositions.values()];
        const minX = Math.min(...allLocal.map((p) => p.x));
        const minY = Math.min(...allLocal.map((p) => p.y));
        const maxX = Math.max(...allLocal.map((p) => p.x + p.width));
        const maxY = Math.max(...allLocal.map((p) => p.y + p.height));
        const contentWidth = Math.max(maxX - minX, laidOut?.width ?? 0);
        const contentHeight = Math.max(maxY - minY, laidOut?.height ?? 0);
        const pkgWidth = Math.max(900, contentWidth + 220);
        const pkgHeight = Math.max(360, contentHeight + 240);
        const offsetX = cursorX - minX + 40;
        const offsetY = cursorY - minY + 48;
        localPositions.forEach((pos, nodeId) => {
            nodePositions.set(nodeId, {
                x: pos.x + offsetX,
                y: pos.y + offsetY,
                width: pos.width,
                height: pos.height
            });
        });
        topPackageBounds.set(pkg, {
            x: cursorX,
            y: cursorY,
            width: pkgWidth,
            height: pkgHeight
        });
        cursorX = fixedPackageLaneX;
        cursorY += pkgHeight + packageGapY;
    }

    g.selectAll('*').remove();

    const defs = svg.select('defs').empty() ? svg.append('defs') : svg.select('defs');
    defs.selectAll('#general-d3-arrow').remove();
    defs.selectAll('#general-d3-arrow-open').remove();
    defs.selectAll('#general-d3-diamond').remove();
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
        .style('fill', GENERAL_NEUTRAL_EDGE);

    defs.append('marker')
        .attr('id', 'general-d3-arrow-open')
        .attr('viewBox', '0 -5 10 10')
        .attr('refX', 9)
        .attr('refY', 0)
        .attr('markerWidth', 8)
        .attr('markerHeight', 8)
        .attr('orient', 'auto')
        .append('path')
        .attr('d', 'M0,-4L10,0L0,4')
        .style('fill', 'none')
        .style('stroke', GENERAL_NEUTRAL_EDGE)
        .style('stroke-width', '1.3');

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
        .style('fill', 'var(--vscode-editor-background)')
        .style('stroke', GENERAL_NEUTRAL_EDGE)
        .style('stroke-width', '1.2');

    defs.append('marker')
        .attr('id', 'general-d3-diamond')
        .attr('viewBox', '0 -6 12 12')
        .attr('refX', 2)
        .attr('refY', 0)
        .attr('markerWidth', 7)
        .attr('markerHeight', 7)
        .attr('orient', 'auto')
        .append('path')
        .attr('d', 'M0,0L5,-4L10,0L5,4Z')
        .style('fill', GENERAL_NEUTRAL_EDGE);

    const edgeGroup = g.append('g').attr('class', 'general-edges');
    const nodeGroup = g.append('g').attr('class', 'general-nodes');
    const packageGroup = g.append('g').attr('class', 'general-packages');

    topPackageBounds.forEach((bounds, pkgName) => {
        packageGroup.append('rect')
            .attr('x', bounds.x)
            .attr('y', bounds.y)
            .attr('width', bounds.width)
            .attr('height', bounds.height)
            .attr('rx', 18)
            .style('fill', 'transparent')
            .style('stroke', GENERAL_NEUTRAL_BORDER)
            .style('stroke-width', '1.5px')
            .style('stroke-dasharray', 'none')
            .style('opacity', 0.9);
        packageGroup.append('text')
            .attr('x', bounds.x + 14)
            .attr('y', bounds.y + 21)
            .text(pkgName)
            .style('font-size', '11px')
            .style('font-weight', '700')
            .style('fill', GENERAL_NEUTRAL_BORDER);
    });

    const laidOutEdges: any[] = [];
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
    internalEdgesToRender.forEach((el: any, edgeIdx: number) => {
        const srcPos = nodePositions.get(el.data.source);
        const tgtPos = nodePositions.get(el.data.target);
        if (!srcPos || !tgtPos) return;
        const obstacles = [...nodePositions.entries()]
            .filter(([nodeId]) => nodeId !== el.data.source && nodeId !== el.data.target)
            .map(([, pos]) => ({ x: pos.x - 8, y: pos.y - 8, width: pos.width + 16, height: pos.height + 16 }));

        const relType = (el.data.relType || el.data.type || 'relationship').toLowerCase();
        const elkEdge = laidOutEdges[edgeIdx];

        let pathD: string;
        if (elkEdge?.sections) {
            const elkPath = pathFromElkSections(elkEdge.sections);
            pathD = elkPath ?? routeOrthogonalAvoiding(
                { x: srcPos.x, y: srcPos.y, width: srcPos.width, height: srcPos.height },
                { x: tgtPos.x, y: tgtPos.y, width: tgtPos.width, height: tgtPos.height },
                obstacles
            );
        } else {
            let offset = 0;
            if (relType === 'typing') {
                const group = typingByTarget.get(el.data.target) || [];
                const rank = group.findIndex((x) => x.el === el);
                if (rank >= 0 && group.length > 1) {
                    offset = (rank - (group.length - 1) / 2) * 18;
                }
            }
            pathD = routeOrthogonalAvoiding(
                { x: srcPos.x, y: srcPos.y, width: srcPos.width, height: srcPos.height },
                { x: tgtPos.x, y: tgtPos.y, width: tgtPos.width, height: tgtPos.height },
                obstacles
            );
        }

        let strokeColor = GENERAL_NEUTRAL_EDGE;
        let strokeDash = 'none';
        let markerStart = 'none';
        let markerEnd = 'url(#general-d3-arrow)';
        let strokeWidth = '2px';

        if (relType === 'specializes') {
            strokeColor = GENERAL_NEUTRAL_EDGE;
            markerEnd = 'url(#general-d3-specializes)';
            strokeWidth = '1.7px';
        } else if (relType === 'typing') {
            strokeColor = GENERAL_NEUTRAL_EDGE;
            strokeDash = '5,3';
            markerEnd = 'url(#general-d3-arrow-open)';
        } else if (relType === 'hierarchy' || relType === 'contains') {
            strokeColor = GENERAL_NEUTRAL_EDGE;
            markerStart = 'url(#general-d3-diamond)';
            markerEnd = 'none';
        } else if (relType === 'connection' || relType === 'connect') {
            strokeColor = GENERAL_NEUTRAL_EDGE;
        } else if (relType === 'bind' || relType === 'binding') {
            strokeColor = GENERAL_NEUTRAL_EDGE;
            strokeDash = '2,2';
            markerEnd = 'none';
        } else if (relType === 'allocate' || relType === 'allocation') {
            strokeColor = GENERAL_NEUTRAL_EDGE;
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
            .style('marker-start', markerStart)
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
        nodeG.select('.graph-node-background')
            .style('fill', 'var(--vscode-editor-background)')
            .style('stroke', GENERAL_NEUTRAL_BORDER);
        nodeG.selectAll('.sysml-header-compartment')
            .style('fill', 'var(--vscode-button-secondaryBackground)');

        nodeG.on('click', function (event: any) {
            event.stopPropagation();
            clearVisualHighlights();
            g.selectAll('.general-node').select('.graph-node-background').each(function (this: any) {
                const r = d3.select(this);
                r.style('stroke', r.attr('data-original-stroke'))
                    .style('stroke-width', r.attr('data-original-width'));
            });
            nodeG.select('.graph-node-background')
                .style('stroke', DIAGRAM_STYLE.highlight)
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

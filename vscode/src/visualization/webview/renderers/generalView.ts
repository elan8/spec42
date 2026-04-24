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
const collapsedGeneralViewSections = new Set<string>();
const expandedGeneralViewSectionRows = new Set<string>();
const initializedGeneralViewSections = new Set<string>();

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

function pointsToPath(points: Array<{ x: number; y: number }>): string {
    if (!Array.isArray(points) || points.length === 0) return '';
    const [first, ...rest] = points;
    return ['M' + first.x + ',' + first.y, ...rest.map((p) => 'L' + p.x + ',' + p.y)].join(' ');
}

type Side = 'NORTH' | 'SOUTH' | 'EAST' | 'WEST';
type NodeRect = { x: number; y: number; width: number; height: number };
type ElkSection = { startPoint?: { x: number; y: number }; endPoint?: { x: number; y: number }; bendPoints?: Array<{ x: number; y: number }> };
type EdgeSectionsMap = Map<string, ElkSection[]>;
type PackageLayoutResult = {
    localPositions: Map<string, NodeRect>;
    edgeSectionsById: EdgeSectionsMap;
    width: number;
    height: number;
};

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

function buildNodeDataMap(cyNodes: any[]): Map<string, { compartments: SysMLNodeCompartments; height: number }> {
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
                other: [],
                collapsibleSections: []
            };
        const sectionKeyPrefix = d.id + '::';
        const maxLines = GENERAL_VIEW_NODE_CONFIG.maxLinesPerCompartment ?? Infinity;
        const directSections: Array<[string, Array<unknown>]> = [
            ['attributes', compartments.attributes],
            ['parts', compartments.parts],
            ['ports', compartments.ports]
        ];
        directSections.forEach(([name, items]) => {
            if (items.length <= maxLines) {
                expandedGeneralViewSectionRows.delete(sectionKeyPrefix + name);
            }
        });
        compartments.collapsibleSections = (compartments.collapsibleSections || []).map((section) => {
            const key = sectionKeyPrefix + section.key;
            if (!initializedGeneralViewSections.has(key)) {
                collapsedGeneralViewSections.add(key);
                initializedGeneralViewSections.add(key);
            }
            if (section.items.length <= maxLines) {
                expandedGeneralViewSectionRows.delete(key);
            }
            return {
                ...section,
                key,
                collapsed: collapsedGeneralViewSections.has(key),
                showAll: expandedGeneralViewSectionRows.has(key)
            };
        });
        const nodeHeight = computeNodeHeightFromCompartments(compartments, GENERAL_VIEW_NODE_CONFIG, NODE_WIDTH);
        nodeDataMap.set(d.id, { compartments, height: Math.max(NODE_HEIGHT_BASE, nodeHeight) });
    });
    return nodeDataMap;
}

async function layoutPackageWithElk(
    elk: any,
    nodesInPkg: any[],
    edgesInPkg: any[],
    nodeDataMap: Map<string, { compartments: SysMLNodeCompartments; height: number }>
): Promise<PackageLayoutResult> {
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

    const edgeIdFor = (el: any, idx: number): string => String(el?.data?.id || ('edge-' + idx));
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
                ports.push({ id: nodeId + '_south_' + i, layoutOptions: { 'org.eclipse.elk.port.side': 'SOUTH' } });
            }
            for (let i = 0; i < Math.max(inCount, 1); i++) {
                ports.push({ id: nodeId + '_north_' + i, layoutOptions: { 'org.eclipse.elk.port.side': 'NORTH' } });
            }
            return { id: nodeId, width: NODE_WIDTH, height: nodeHeight, ports };
        }),
        edges: edgesInPkg.map((el: any, idx: number) => {
            const src = el.data.source;
            const tgt = el.data.target;
            const srcPort = src + '_south_' + getOutgoingPortIndex(src, el);
            const tgtPort = tgt + '_north_' + getIncomingPortIndex(tgt, el);
            return { id: edgeIdFor(el, idx), sources: [srcPort], targets: [tgtPort] };
        })
    };

    if (!elk) {
        const localPositions = new Map<string, NodeRect>();
        let lx = 40;
        let ly = 40;
        nodesInPkg.forEach((el: any) => {
            const nd = nodeDataMap.get(el.data.id);
            const h = nd?.height ?? NODE_HEIGHT_BASE;
            localPositions.set(el.data.id, { x: lx, y: ly, width: NODE_WIDTH, height: h });
            lx += NODE_WIDTH + 60;
            if (lx > 1000) {
                lx = 40;
                ly += h + 80;
            }
        });
        const allLocal = [...localPositions.values()];
        const minX = Math.min(...allLocal.map((p) => p.x));
        const minY = Math.min(...allLocal.map((p) => p.y));
        const maxX = Math.max(...allLocal.map((p) => p.x + p.width));
        const maxY = Math.max(...allLocal.map((p) => p.y + p.height));
        return {
            localPositions,
            edgeSectionsById: new Map<string, ElkSection[]>(),
            width: Math.max(maxX - minX, 0),
            height: Math.max(maxY - minY, 0)
        };
    }

    try {
        const laidOut = await elk.layout(elkGraph);
        const localPositions = new Map<string, NodeRect>();
        (laidOut?.children || []).forEach((child: any) => {
            const nd = nodeDataMap.get(child.id);
            localPositions.set(child.id, {
                x: child.x ?? 0,
                y: child.y ?? 0,
                width: child.width ?? NODE_WIDTH,
                height: child.height ?? nd?.height ?? NODE_HEIGHT_BASE
            });
        });
        const edgeSectionsById = new Map<string, ElkSection[]>();
        (laidOut?.edges || []).forEach((edge: any) => {
            if (edge?.id && Array.isArray(edge.sections) && edge.sections.length > 0) {
                edgeSectionsById.set(String(edge.id), edge.sections as ElkSection[]);
            }
        });
        return {
            localPositions,
            edgeSectionsById,
            width: laidOut?.width ?? 0,
            height: laidOut?.height ?? 0
        };
    } catch (e) {
        console.error('[General View] ELK package layout failed:', e);
        return { localPositions: new Map<string, NodeRect>(), edgeSectionsById: new Map<string, ElkSection[]>(), width: 0, height: 0 };
    }
}

function offsetSections(sections: ElkSection[], dx: number, dy: number): ElkSection[] {
    return sections.map((sec) => ({
        startPoint: sec.startPoint ? { x: sec.startPoint.x + dx, y: sec.startPoint.y + dy } : undefined,
        endPoint: sec.endPoint ? { x: sec.endPoint.x + dx, y: sec.endPoint.y + dy } : undefined,
        bendPoints: Array.isArray(sec.bendPoints)
            ? sec.bendPoints.map((p) => ({ x: p.x + dx, y: p.y + dy }))
            : undefined
    }));
}

function renderPackageContainers(
    packageGroup: any,
    topPackageBounds: Map<string, NodeRect>
): void {
    packageGroup.style('pointer-events', 'none');
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
}

function renderGeneralEdges(
    edgeGroup: any,
    internalEdgesToRender: any[],
    nodePositions: Map<string, NodeRect>,
    edgeSectionsById: EdgeSectionsMap
): void {
    internalEdgesToRender.forEach((el: any, edgeIdx: number) => {
        const srcPos = nodePositions.get(el.data.source);
        const tgtPos = nodePositions.get(el.data.target);
        if (!srcPos || !tgtPos) return;
        const obstacles = [...nodePositions.entries()]
            .filter(([nodeId]) => nodeId !== el.data.source && nodeId !== el.data.target)
            .map(([, pos]) => ({ x: pos.x - 8, y: pos.y - 8, width: pos.width + 16, height: pos.height + 16 }));

        const relType = (el.data.relType || el.data.type || 'relationship').toLowerCase();
        const elkEdgeSections = edgeSectionsById.get(String(el?.data?.id || ('edge-' + edgeIdx)));

        let pathD: string;
        if (elkEdgeSections && elkEdgeSections.length > 0) {
            const elkPath = pathFromElkSections(elkEdgeSections);
            pathD = elkPath ?? routeOrthogonalAvoiding(srcPos, tgtPos, obstacles);
        } else {
            pathD = routeOrthogonalAvoiding(srcPos, tgtPos, obstacles);
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
}

function renderGeneralNodes(
    nodeGroup: any,
    g: any,
    cyNodes: any[],
    nodePositions: Map<string, NodeRect>,
    nodeDataMap: Map<string, { compartments: SysMLNodeCompartments; height: number }>,
    clearVisualHighlights: () => void,
    postMessage: (msg: unknown) => void,
    rerenderGeneralView: () => void
): void {
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
            other: [],
            collapsibleSections: []
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
            dataElementName: d.elementName || d.label,
            sectionKeyPrefix: d.id + '::',
            onSectionToggle: (sectionKey, action) => {
                if (action === 'collapse') {
                    if (collapsedGeneralViewSections.has(sectionKey)) {
                        collapsedGeneralViewSections.delete(sectionKey);
                    } else {
                        collapsedGeneralViewSections.add(sectionKey);
                    }
                } else if (expandedGeneralViewSectionRows.has(sectionKey)) {
                    expandedGeneralViewSectionRows.delete(sectionKey);
                } else {
                    expandedGeneralViewSectionRows.add(sectionKey);
                }
                rerenderGeneralView();
            }
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
                r.style('stroke', r.attr('data-original-stroke')).style('stroke-width', r.attr('data-original-width'));
            });
            nodeG.select('.graph-node-background').style('stroke', DIAGRAM_STYLE.highlight).style('stroke-width', '4px');
            const statusEl = document.getElementById('status-text');
            if (statusEl) statusEl.textContent = (d.label || d.elementName) + ' [' + (d.sysmlType || 'element') + ']';
            const elementName = d.elementName;
            const elementQualifiedName = d.elementQualifiedName || elementName;
            if (elementName) {
                postJumpToElement(
                    postMessage,
                    { name: elementName, id: elementQualifiedName || undefined, uri: d.elementUri || undefined, range: d.element?.range || undefined },
                    { skipCentering: true }
                );
            }
        });
    });
}

export async function renderGeneralViewD3(ctx: GeneralViewContext, data: any): Promise<void> {
    const { width, height, svg, g, postMessage, renderPlaceholder, clearVisualHighlights } = ctx;
    if (ctx.abortSignal?.aborted) return;

    const result = ctx.buildGeneralViewGraph(data);
    const { elements, typeStats, packageGroups } = result;

    ctx.renderGeneralChips(typeStats);

    const cyNodes = elements.filter((el: any) => el.group === 'nodes');
    const cyEdges = elements.filter((el: any) => el.group === 'edges');

    if (cyNodes.length === 0) {
        renderPlaceholder(width, height, data?.selectedViewName || 'SysML Visualizer',
            data?.emptyStateMessage || 'No matching elements to display.',
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

    const nodeDataMap = buildNodeDataMap(cyNodes);

    // Do not split the structure view by package. Render one unified semantic graph.
    const packageNodeMap = new Map<string, any[]>();
    packageNodeMap.set('__all__', cyNodes);
    const packageOrder = ['__all__'];

    const nodePositions = new Map<string, { x: number; y: number; width: number; height: number }>();
    const topPackageBounds = new Map<string, { x: number; y: number; width: number; height: number }>();
    const internalEdgesToRender: any[] = [];
    const edgeSectionsById = new Map<string, ElkSection[]>();

    // Place all nodes in one layout scope (single semantic hierarchy/graph).
    const packageGapY = 220;
    let cursorX = 80;
    let cursorY = 80;
    const fixedPackageLaneX = 80;
    for (const pkg of packageOrder) {
        if (ctx.abortSignal?.aborted) return;
        const nodesInPkg = packageNodeMap.get(pkg) || [];
        const nodeIdSet = new Set(nodesInPkg.map((n: any) => n.data.id));
        const edgesInPkg = cyEdges.filter((e: any) =>
            nodeIdSet.has(e.data.source) && nodeIdSet.has(e.data.target)
        );
        internalEdgesToRender.push(...edgesInPkg);

        const laidOut = await layoutPackageWithElk(elk, nodesInPkg, edgesInPkg, nodeDataMap);
        if (ctx.abortSignal?.aborted) return;
        const localPositions = laidOut.localPositions;

        const allLocal = [...localPositions.values()];
        if (allLocal.length === 0) continue;
        const minX = Math.min(...allLocal.map((p) => p.x));
        const minY = Math.min(...allLocal.map((p) => p.y));
        const maxX = Math.max(...allLocal.map((p) => p.x + p.width));
        const maxY = Math.max(...allLocal.map((p) => p.y + p.height));
        const contentWidth = Math.max(maxX - minX, laidOut.width ?? 0);
        const contentHeight = Math.max(maxY - minY, laidOut.height ?? 0);
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
        laidOut.edgeSectionsById.forEach((sections, edgeId) => {
            edgeSectionsById.set(edgeId, offsetSections(sections, offsetX, offsetY));
        });
        if (pkg !== '__all__') {
            topPackageBounds.set(pkg, {
                x: cursorX,
                y: cursorY,
                width: pkgWidth,
                height: pkgHeight
            });
        }
        cursorX = fixedPackageLaneX;
        cursorY += pkgHeight + packageGapY;
    }

    g.selectAll('*').remove();
    if (ctx.abortSignal?.aborted) return;

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

    const packageGroup = g.append('g').attr('class', 'general-packages');
    const edgeGroup = g.append('g').attr('class', 'general-edges');
    const nodeGroup = g.append('g').attr('class', 'general-nodes');
    renderPackageContainers(packageGroup, topPackageBounds);
    renderGeneralEdges(edgeGroup, internalEdgesToRender, nodePositions, edgeSectionsById);

    const statusEl = document.getElementById('status-text');
    if (statusEl) statusEl.textContent = 'General View • Tap element to highlight, double-tap to jump';

    const rerenderGeneralView = () => {
        g.selectAll('*').remove();
        void renderGeneralViewD3(ctx, data);
    };
    renderGeneralNodes(nodeGroup, g, cyNodes, nodePositions, nodeDataMap, clearVisualHighlights, postMessage, rerenderGeneralView);
}

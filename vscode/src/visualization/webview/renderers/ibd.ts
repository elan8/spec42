/**
 * IBD/Interconnection View renderer - parts, ports, connectors.
 * Uses ELK for connection-aware layout and orthogonal edge routing when available.
 */
/* eslint-disable @typescript-eslint/no-explicit-any */

import type { RenderContext } from '../types';
import { GENERAL_VIEW_PALETTE } from '../constants';
import { postJumpToElement } from '../jumpToElement';
import { getTypeColor, isLibraryValidated } from '../shared';
import { DIAGRAM_STYLE } from '../styleTokens';

declare const d3: any;
declare const ELK: any;
const NEUTRAL_EDGE_BLUE = DIAGRAM_STYLE.edgePrimary;
const NEUTRAL_PORT_BLUE = DIAGRAM_STYLE.edgePrimary;
const NEUTRAL_NODE_BORDER = DIAGRAM_STYLE.nodeBorder;
const NEUTRAL_NODE_FILL = DIAGRAM_STYLE.nodeFill;
const NEUTRAL_TEXT = DIAGRAM_STYLE.textPrimary;
const PORT_OUTLINE_GREEN = 'var(--vscode-button-background)';

type NodeRect = { x: number; y: number; width: number; height: number };
type ElkSection = { startPoint?: { x: number; y: number }; endPoint?: { x: number; y: number }; bendPoints?: Array<{ x: number; y: number }> };
type EdgeSectionsMap = Map<string, ElkSection[]>;
type IbdLayoutResult = {
    partPositions: Map<string, { x: number; y: number; part: any; height: number; width?: number; isContainer?: boolean; depth?: number }>;
    portPositions: Map<string, { x: number; y: number; side?: string; partId: string }>;
    connectorSectionsById: EdgeSectionsMap;
    bounds: NodeRect;
};
type RouteFrameDiagnostics = {
    rootErr: number;
    parentErr: number;
    rootFinalScore: number;
    parentFinalScore: number;
    rootPoints: { x: number; y: number }[];
    parentPoints: { x: number; y: number }[];
    translatedRootPoints?: { x: number; y: number }[];
    translatedRootErr?: number;
    translatedRootFinalScore?: number;
};

function selectIbdViewData(data: any): { parts: any[]; ports: any[]; connectors: any[] } {
    return {
        parts: data?.parts || [],
        ports: data?.ports || [],
        connectors: data?.connectors || [],
    };
}

function getConnectorId(connector: any, idx: number): string {
    const explicit = String(connector?.id || '').trim();
    if (explicit.length > 0) {
        return explicit.replace(/[^A-Za-z0-9_.-]/g, '_');
    }
    return `edge-${idx}`;
}

function collectElkEdgesWithOffsets(
    node: any,
    containerOffset: { x: number; y: number },
    acc: Map<string, { edge: any; offset: { x: number; y: number } }>
): void {
    if (node?.edges) {
        for (const edge of node.edges) {
            const edgeId = String(edge?.id ?? '');
            if (!edgeId) continue;
            acc.set(edgeId, { edge, offset: { x: containerOffset.x, y: containerOffset.y } });
        }
    }
    (node?.children ?? []).forEach((c: any) =>
        collectElkEdgesWithOffsets(
            c,
            {
                x: containerOffset.x + (c?.x ?? 0),
                y: containerOffset.y + (c?.y ?? 0),
            },
            acc,
        )
    );
}

export async function renderIbdView(ctx: RenderContext & { elkWorkerUrl?: string }, data: any): Promise<void> {
    const { width, height, svg, g, layoutDirection, postMessage, onStartInlineEdit, renderPlaceholder, clearVisualHighlights } = ctx;
    const LOG_ENDPOINT_DRIFT = false;
    const LOG_ROUTE_FRAME_SELECTION = false;
    const ENDPOINT_DRIFT_WARN_PX = 1.25;

    if (false && Array.isArray(data?.parts) && data.parts.length > 0 && typeof data.parts[0]?.x === 'number') {
        const parts = data.parts;
        const ports = Array.isArray(data?.ports) ? data.ports : [];
        const connectors = Array.isArray(data?.connectors) ? data.connectors : [];
        const partGroup = g.append('g').attr('class', 'ibd-parts');
        const connectorGroup = g.append('g').attr('class', 'ibd-connectors');
        const labelGroup = g.append('g').attr('class', 'ibd-connector-labels');

        connectors.forEach((connector: any) => {
            const path = Array.isArray(connector.points) && connector.points.length > 0
                ? connector.points.map((p: any, idx: number) => `${idx === 0 ? 'M' : 'L'}${p.x},${p.y}`).join(' ')
                : '';
            connectorGroup.append('path')
                .attr('d', path)
                .attr('class', 'ibd-connector')
                .attr('data-source', connector.source)
                .attr('data-target', connector.target)
                .style('fill', 'none')
                .style('stroke', DIAGRAM_STYLE.edgePrimary)
                .style('stroke-width', '2px');
            const midpoint = Array.isArray(connector.points) && connector.points.length > 1
                ? connector.points[Math.floor(connector.points.length / 2)]
                : null;
            if (midpoint) {
                labelGroup.append('text')
                    .attr('x', midpoint.x)
                    .attr('y', midpoint.y - 4)
                    .attr('text-anchor', 'middle')
                    .text(connector.type || '')
                    .style('font-size', '9px')
                    .style('fill', 'var(--vscode-descriptionForeground)');
            }
        });

        parts
            .slice()
            .sort((a: any, b: any) => (a.depth || 0) - (b.depth || 0))
            .forEach((part: any) => {
                const partG = partGroup.append('g')
                    .attr('transform', `translate(${part.x},${part.y})`)
                    .attr('class', 'ibd-part' + (part.isContainer ? ' ibd-container' : ''))
                    .attr('data-element-name', part.name)
                    .style('cursor', 'pointer');
                partG.append('rect')
                    .attr('width', part.width)
                    .attr('height', part.height)
                    .attr('rx', part.isContainer ? 8 : 6)
                    .attr('class', 'graph-node-background')
                    .attr('data-original-stroke', DIAGRAM_STYLE.nodeBorder)
                    .attr('data-original-width', part.isContainer ? '2px' : '2.5px')
                    .style('fill', 'var(--vscode-editor-background)')
                    .style('stroke', DIAGRAM_STYLE.nodeBorder)
                    .style('stroke-width', part.isContainer ? '2px' : '2.5px')
                    .style('stroke-dasharray', part.isContainer ? '4,4' : 'none');
                partG.append('rect')
                    .attr('width', part.width)
                    .attr('height', 34)
                    .style('fill', 'var(--vscode-button-secondaryBackground)');
                partG.append('text')
                    .attr('x', part.width / 2)
                    .attr('y', 16)
                    .attr('text-anchor', 'middle')
                    .text('«' + (part.type || 'part') + '»')
                    .style('font-size', '9px')
                    .style('fill', DIAGRAM_STYLE.textPrimary);
                partG.append('text')
                    .attr('x', part.width / 2)
                    .attr('y', 29)
                    .attr('text-anchor', 'middle')
                    .text(part.name)
                    .style('font-size', '11px')
                    .style('font-weight', 'bold')
                    .style('fill', DIAGRAM_STYLE.textPrimary);
                partG.on('click', function(event: any) {
                    event.stopPropagation();
                    clearVisualHighlights();
                    const clickedPart = d3.select(this);
                    clickedPart.classed('highlighted-element', true);
                    clickedPart.select('rect')
                        .style('stroke', DIAGRAM_STYLE.highlight)
                        .style('stroke-width', '3px');
                    postJumpToElement(postMessage, { name: part.name, id: part.qualifiedName || part.id, uri: part.uri || undefined }, { skipCentering: true });
                }).on('dblclick', function(event: any) {
                    event.stopPropagation();
                    onStartInlineEdit(d3.select(this), part.name, part.x, part.y, part.width);
                });
            });

        ports.forEach((port: any) => {
            g.append('rect')
                .attr('class', 'port-icon')
                .attr('x', port.x - 5)
                .attr('y', port.y - 5)
                .attr('width', 10)
                .attr('height', 10)
                .style('fill', 'none')
                .style('stroke', 'var(--vscode-button-background)')
                .style('stroke-width', '1.8px');
        });

        const statusEl = document.getElementById('status-text');
        if (statusEl) statusEl.textContent = 'Interconnection View • Backend scene';
        return;
    }

    if (!data || !data.parts || data.parts.length === 0) {
        renderPlaceholder(width, height, 'Interconnection View',
            'No parts or internal structure found to display.\\n\\nThis view shows internal block diagrams with parts, ports, and connectors.',
            data);
        return;
    }

    const selected = selectIbdViewData(data);
    const parts = selected.parts;
    const ports = selected.ports;
    const connectors = selected.connectors;
    const normalizedConnectorUsage = new Map<string, { sourceCount: number; targetCount: number }>();
    const toDot = (qn: string) => (qn || '').replace(/::/g, '.');
    const normalizeEndpointId = (value: string | null | undefined) => toDot(value || '').trim();

    const getPortsForPartRef = (part: any) => ports.filter((p: any) =>
        p && (
            p.parentId === part.name ||
            p.parentId === part.id ||
            p.parentId === part.qualifiedName ||
            normalizeEndpointId(p.parentId) === normalizeEndpointId(part.qualifiedName) ||
            normalizeEndpointId(p.parentId) === normalizeEndpointId(part.name)
        )
    );

    // Layout configuration - adapt spacing to port density instead of a single fixed layout.
    const isHorizontal = layoutDirection === 'horizontal';
    const basePartWidth = 280;
    const maxPortsPerPart = Math.max(0, ...parts.map((part: any) => getPortsForPartRef(part).length));
    const layoutDensityBoost = Math.min(80, Math.max(0, maxPortsPerPart - 2) * 10);
    const partWidth = basePartWidth;
    const padding = 170 + Math.min(60, Math.max(0, maxPortsPerPart - 1) * 8);
    const horizontalSpacing = 220 + layoutDensityBoost;
    const verticalSpacing = 140 + Math.round(layoutDensityBoost * 0.8);

    const bumpConnectorUsage = (endpointId: string | null | undefined, role: 'sourceCount' | 'targetCount') => {
        const key = normalizeEndpointId(endpointId);
        if (!key) return;
        const current = normalizedConnectorUsage.get(key) || { sourceCount: 0, targetCount: 0 };
        current[role] += 1;
        normalizedConnectorUsage.set(key, current);
    };
    connectors.forEach((connector: any) => {
        bumpConnectorUsage(connector.sourceId || connector.source, 'sourceCount');
        bumpConnectorUsage(connector.targetId || connector.target, 'targetCount');
    });

    const partToElkId = (p: any) => toDot(p.qualifiedName) || p.id || p.name;
    const sanitizeElkSegment = (value: string) => value.replace(/[^A-Za-z0-9_.-]/g, "_");
    const getPortDirection = (port: any): 'in' | 'out' | 'inout' => {
        const direction = String(port?.direction || '').toLowerCase();
        const name = String(port?.name || '').toLowerCase();
        if (direction === 'in' || (!direction && name.startsWith('in'))) return 'in';
        if (direction === 'out' || (!direction && name.startsWith('out'))) return 'out';
        return 'inout';
    };
    const getExplicitPortSide = (port: any): 'left' | 'right' | null => {
        const side = String(port?.portSide || port?.attributes?.portSide || '').toLowerCase();
        if (side === 'left' || side === 'west') return 'left';
        if (side === 'right' || side === 'east') return 'right';
        return null;
    };
    const getPortTypeName = (port: any): string | null => {
        const attrs = port?.attributes;
        const raw = port?.portType || (attrs?.get ? (attrs.get('portType') || attrs.get('type')) : (attrs?.portType || attrs?.type));
        if (!raw) return null;
        const text = String(raw).replace(/^~/, '');
        const segments = text.split(/::|\./);
        return segments[segments.length - 1] || text;
    };
    const isConjugatedPort = (port: any): boolean => {
        const raw = String(port?.portType || port?.attributes?.portType || '');
        return raw.trim().startsWith('~');
    };
    const truncateLabel = (value: string, maxLength: number): string =>
        value.length > maxLength ? value.substring(0, maxLength - 2) + '..' : value;
    const getPortVisualColor = (direction: 'in' | 'out' | 'inout'): string => {
        if (direction === 'in') return NEUTRAL_PORT_BLUE;
        if (direction === 'out') return NEUTRAL_PORT_BLUE;
        return NEUTRAL_PORT_BLUE;
    };
    const getConnectorVisualStyle = (connector: any) => {
        const connTypeLower = String(connector?.type || '').toLowerCase();
        const connNameLower = String(connector?.name || '').toLowerCase();
        const isFlow = connTypeLower === 'flow' || connNameLower.includes('flow');
        const isInterface = connTypeLower === 'interface' || connNameLower.includes('interface');
        const isBinding = connTypeLower === 'binding' || connNameLower.includes('bind');
        const isConnection = connTypeLower === 'connection' || connNameLower.includes('connect');

        if (isFlow) {
            return {
                strokeColor: 'var(--vscode-charts-green)',
                strokeStyle: 'none',
                strokeWidth: '2.5px',
                markerStart: 'none',
                markerEnd: 'url(#ibd-flow-arrow)',
                typeIndicator: '-> ',
                isFlow: true,
            };
        }
        if (isInterface) {
            return {
                strokeColor: 'var(--vscode-charts-purple)',
                strokeStyle: '8,4',
                strokeWidth: '2px',
                markerStart: 'none',
                markerEnd: 'url(#ibd-interface-arrow)',
                typeIndicator: '<> ',
                isFlow: false,
            };
        }
        if (isBinding) {
            return {
                strokeColor: NEUTRAL_EDGE_BLUE,
                strokeStyle: '6,4',
                strokeWidth: '1.5px',
                markerStart: 'url(#ibd-connection-dot)',
                markerEnd: 'url(#ibd-connection-dot)',
                typeIndicator: '= ',
                isFlow: false,
            };
        }
        return {
            strokeColor: NEUTRAL_EDGE_BLUE,
            strokeStyle: isConnection ? 'none' : '2,2',
            strokeWidth: isConnection ? '2px' : '1.5px',
            markerStart: 'url(#ibd-connection-dot)',
            markerEnd: 'url(#ibd-connection-dot)',
            typeIndicator: isConnection ? 'o ' : '',
            isFlow: false,
        };
    };
    const getPortLayoutKey = (part: any, port: any): string => {
        const parent = normalizeEndpointId(port?.parentId || part?.qualifiedName || part?.id || part?.name);
        const explicit = normalizeEndpointId(port?.id);
        if (explicit) return explicit;
        return parent ? `${parent}.${String(port?.name || '').trim()}` : normalizeEndpointId(port?.name);
    };
    const getPortUsage = (part: any, port: any) => {
        const explicit = normalizedConnectorUsage.get(getPortLayoutKey(part, port));
        if (explicit) return explicit;
        const parent = normalizeEndpointId(port?.parentId || part?.qualifiedName || part?.id || part?.name);
        const fallback = normalizedConnectorUsage.get(`${parent}.${normalizeEndpointId(port?.name)}`);
        return fallback || { sourceCount: 0, targetCount: 0 };
    };
    const inferDefaultPortSide = (part: any, port: any): 'left' | 'right' => {
        const explicit = getExplicitPortSide(port);
        if (explicit) return explicit;
        const direction = getPortDirection(port);
        if (direction === 'in') return 'left';
        if (direction === 'out') return 'right';
        const usage = getPortUsage(part, port);
        if (usage.targetCount > usage.sourceCount) return 'left';
        if (usage.sourceCount > usage.targetCount) return 'right';

        const partText = `${String(part?.name || '')} ${String(part?.qualifiedName || '')}`.toLowerCase();
        const portText = String(port?.name || '').toLowerCase();
        const rawPortType = String(port?.portType || '').toLowerCase();
        const conjugated = isConjugatedPort(port);
        const endsWithIn = /in$/.test(portText);
        const endsWithOut = /out$/.test(portText);
        if (endsWithIn) return 'left';
        if (endsWithOut) return 'right';
        if (conjugated) {
            if (/(powerport|telemetryport|sensordataport|gimbalcommandport|cameracontrolport)/.test(rawPortType)) {
                return 'left';
            }
        } else if (/(powerport|telemetryport|sensordataport)/.test(rawPortType)) {
            return 'right';
        }
        const prefersLeft = /(sensor|imu|barometer|gnss|receiver|battery|input|telemetryin|videoin|c2in|rcin|sensorin)/.test(partText)
            || /(in$|in[A-Z]?|cmd$|control$|input|telemetryin|videoin|sensorin|mainpower)/.test(portText);
        const prefersRight = /(camera|gimbal|propulsion|motor|radio|communication|distribution|controller|payload|actuator)/.test(partText)
            || /(out$|out[A-Z]?|videoout|telemetryout|regulated|pwr|cmd|ctrl)/.test(portText);
        if (prefersLeft && !prefersRight) return 'left';
        if (prefersRight && !prefersLeft) return 'right';
        return 'right';
    };
    const portLayoutCache = new Map<string, { leftPorts: any[]; rightPorts: any[] }>();
    const getPortLayoutForPart = (part: any) => {
        const cacheKey = String(part?.qualifiedName || part?.id || part?.name || '');
        const cached = portLayoutCache.get(cacheKey);
        if (cached) return cached;
        const leftPorts: any[] = [];
        const rightPorts: any[] = [];
        getPortsForPartRef(part).forEach((port: any) => {
            if (!port?.name) return;
            if (inferDefaultPortSide(part, port) === 'left') leftPorts.push(port);
            else rightPorts.push(port);
        });
        const comparePorts = (a: any, b: any) => {
            const usageA = getPortUsage(part, a);
            const usageB = getPortUsage(part, b);
            const degreeA = usageA.sourceCount + usageA.targetCount;
            const degreeB = usageB.sourceCount + usageB.targetCount;
            if (degreeB !== degreeA) return degreeB - degreeA;
            return String(a.name || '').localeCompare(String(b.name || ''));
        };
        leftPorts.sort(comparePorts);
        rightPorts.sort(comparePorts);
        const layout = { leftPorts, rightPorts };
        portLayoutCache.set(cacheKey, layout);
        return layout;
    };
    const elkPortIdFor = (part: any, port: any) => `${partToElkId(part)}__port__${sanitizeElkSegment(getPortLayoutKey(part, port))}`;

    // Assign IDs to parts
    parts.forEach((part: any, index: number) => {
        if (!part.id) part.id = part.name || ('part-' + index);
    });

    // Helper function to calculate part height based on content
    const calculatePartHeight = (part: any) => {
        const partPorts = getPortsForPartRef(part);
        const { leftPorts, rightPorts } = getPortLayoutForPart(part);
        const partChildren = part.children || [];

        let contentLineCount = 0;

        partChildren.forEach((c: any) => {
            if (!c || !c.name || !c.type) return;
            if (c.type === 'part') {
                contentLineCount++;
                if (c.properties) contentLineCount += Object.keys(c.properties).length;
                if (c.attributes) {
                    if (typeof c.attributes.forEach === 'function') {
                        c.attributes.forEach(() => contentLineCount++);
                    } else if (typeof c.attributes === 'object') {
                        contentLineCount += Object.keys(c.attributes).filter((k: string) => k !== 'isRedefinition').length;
                    }
                }
                if (c.children) {
                    contentLineCount += c.children.filter((gc: any) => gc.type === 'redefinition' && gc.name).length;
                }
            } else if (c.type === 'redefinition' || c.type === 'attribute' || c.type === 'property' || c.type === 'state') {
                contentLineCount++;
            }
        });

        let hasTypedBy = false;
        if (part.attributes && part.attributes.get) {
            hasTypedBy = !!(part.attributes.get('partType') || part.attributes.get('type') || part.attributes.get('typedBy'));
        }
        if (!hasTypedBy && part.partType) hasTypedBy = true;

        const lineHeight = 12;
        const headerHeight = hasTypedBy ? 50 : 38;
        const contentHeight = contentLineCount * lineHeight + 10;
        const portRows = Math.max(leftPorts.length, rightPorts.length, partPorts.length > 0 ? 1 : 0);
        const portSpacing = 26;
        const portsHeight = partPorts.length > 0 ? (portRows * portSpacing + 22) : 0;

        return Math.max(96, headerHeight + contentHeight + portsHeight);
    };

    const calculatePartWidth = (part: any) => {
        const partPorts = getPortsForPartRef(part);
        let typedByName: string | null = null;
        if (part.attributes && part.attributes.get) {
            typedByName = part.attributes.get('partType') || part.attributes.get('type') || part.attributes.get('typedBy');
        }
        if (!typedByName && part.partType) typedByName = part.partType;

        const longestPortLabel = partPorts.reduce((max: number, port: any) => {
            const portType = getPortTypeName(port);
            const label = portType ? `${port.name} : ${portType}` : String(port.name || '');
            return Math.max(max, label.length);
        }, 0);
        const longestHeader = Math.max(
            String(part.name || '').length,
            String(typedByName || '').length
        );
        const connectednessBonus = Math.min(56, partPorts.length * 10);
        const desiredWidth = Math.max(
            basePartWidth + Math.min(120, partPorts.length * 8),
            170 + longestHeader * 7 + connectednessBonus,
            150 + Math.min(longestPortLabel, 22) * 5.8 + connectednessBonus
        );
        return Math.min(460, desiredWidth);
    };

    const partHeights = new Map<string, number>();
    const partWidths = new Map<string, number>();
    parts.forEach((part: any) => {
        partHeights.set(part.name, calculatePartHeight(part));
        partWidths.set(part.name, calculatePartWidth(part));
        if (part.id) partHeights.set(part.id, calculatePartHeight(part));
        if (part.id) partWidths.set(part.id, calculatePartWidth(part));
    });

    // Build part tree from containment: roots have containerId null/absent; children have containerId === parent
    // Backend sends containerId as parent's qualifiedName (dot form); match by name, id, or qualifiedName
    type PartTreeNode = { part: any; children: PartTreeNode[] };
    const getPartChildren = (p: any) => parts.filter((c: any) =>
        c.containerId === p.name || c.containerId === p.id || c.containerId === p.qualifiedName ||
        toDot(c.containerId) === toDot(p.qualifiedName)
    );
    const sortPartsForLayout = (items: any[]) => [...items].sort((a: any, b: any) => {
        const aPorts = getPortsForPartRef(a).length;
        const bPorts = getPortsForPartRef(b).length;
        if (bPorts !== aPorts) return bPorts - aPorts;
        const aWidth = partWidths.get(a.name) || partWidth;
        const bWidth = partWidths.get(b.name) || partWidth;
        if (bWidth !== aWidth) return bWidth - aWidth;
        return String(a.name || '').localeCompare(String(b.name || ''));
    });
    const buildTree = (part: any): PartTreeNode => ({
        part,
        children: sortPartsForLayout(getPartChildren(part)).map((c: any) => buildTree(c))
    });
    const roots = sortPartsForLayout(parts.filter((p: any) => p.containerId == null || p.containerId === undefined || p.containerId === ''));
    const partForest = roots.map((root: any) => buildTree(root));

    const leafParts = parts.filter((p: any) => getPartChildren(p).length === 0);
    const leafPartIds = new Set<string>(leafParts.map((p: any) => partToElkId(p)));

    const findPartForEndpoint = (endpointPath: string): any => {
        if (!endpointPath) return null;
        const pathDot = normalizeEndpointId(endpointPath);
        let best: { part: any; len: number } | null = null;
        for (const part of parts) {
            const qn = normalizeEndpointId(part.qualifiedName || part.name);
            if (!qn) continue;
            if (pathDot === qn || pathDot.startsWith(qn + '.')) {
                if (!best || qn.length > best.len) best = { part, len: qn.length };
            }
        }
        return best?.part ?? null;
    };
    const resolvePortForEndpointInData = (part: any, endpointId: string | null | undefined): any => {
        if (!part || !endpointId) return null;
        const endpoint = normalizeEndpointId(endpointId);
        const endpointLeaf = endpoint.split('.').pop() || endpoint;
        return getPortsForPartRef(part).find((p: any) => {
            const portName = normalizeEndpointId(p?.name);
            const explicitPortId = normalizeEndpointId(getPortLayoutKey(part, p));
            return endpoint === explicitPortId
                || endpoint.endsWith('.' + portName)
                || endpointLeaf === portName;
        }) ?? null;
    };

    const rootHeaderHeight = 28;
    const containerTopInset = rootHeaderHeight + 20;
    const elkUnavailableReason = typeof ELK === 'undefined'
        ? 'ELK layout library not loaded.'
        : partForest.length === 0
            ? 'No valid interconnection roots were found for ELK layout.'
            : null;
    if (elkUnavailableReason) {
        renderPlaceholder(
            width,
            height,
            'Interconnection View',
            `${elkUnavailableReason}\n\nThis view requires ELK-based part and edge routing.`,
            data
        );
        return;
    }

    let elkLaidOut: any = null;
    try {
        // Keep Interconnection View on ELK, but avoid depending on a web worker here.
        // The diagrams are small enough that in-process ELK is more reliable in tests and webviews.
        const elk = new ELK();

        const treeToElkNode = (node: PartTreeNode): any => {
            const part = node.part;
            const id = partToElkId(part);
            const h = partHeights.get(part.name) || 80;
            const w = partWidths.get(part.name) || partWidth;
            const { leftPorts, rightPorts } = getPortLayoutForPart(part);
            const buildElkPort = (port: any, side: 'WEST' | 'EAST', order: number) => ({
                id: elkPortIdFor(part, port),
                width: 10,
                height: 10,
                layoutOptions: {
                    'org.eclipse.elk.port.side': side,
                    'org.eclipse.elk.port.index': String(order),
                }
            });
            if (node.children.length === 0) {
                return {
                    id,
                    width: w,
                    height: h,
                    ports: [
                        ...leftPorts.map((port: any, index: number) => buildElkPort(port, 'WEST', index)),
                        ...rightPorts.map((port: any, index: number) => buildElkPort(port, 'EAST', index)),
                    ],
                    layoutOptions: {
                        'org.eclipse.elk.portConstraints': 'FIXED_ORDER',
                        'org.eclipse.elk.portAlignment.default': 'CENTER',
                    }
                };
            }
            const childNodes = node.children.map((c) => treeToElkNode(c));
            const childWidthSum = node.children.reduce((sum: number, child: PartTreeNode) => {
                return sum + (partWidths.get(child.part.name) || partWidth);
            }, 0);
            const minW = Math.max(w, Math.min(1040, childWidthSum + node.children.length * 72));
            const minH = rootHeaderHeight + 160;
            return {
                id,
                width: minW,
                height: minH,
                ports: [
                    ...leftPorts.map((port: any, index: number) => buildElkPort(port, 'WEST', index)),
                    ...rightPorts.map((port: any, index: number) => buildElkPort(port, 'EAST', index)),
                ],
                children: childNodes,
                layoutOptions: {
                    // Reserve room for the rendered container header strip.
                    'elk.padding': `[top=${containerTopInset},left=24,bottom=24,right=24]`,
                    'org.eclipse.elk.portConstraints': 'FIXED_ORDER',
                    'org.eclipse.elk.portAlignment.default': 'CENTER',
                }
            };
        };

        const elkEdges: Array<{ id: string; sources: string[]; targets: string[] }> = [];
        connectors.forEach((conn: any, idx: number) => {
            const srcPart = findPartForEndpoint(conn.sourceId || conn.source);
            const tgtPart = findPartForEndpoint(conn.targetId || conn.target);
            if (!srcPart || !tgtPart) return;
            const srcPort = resolvePortForEndpointInData(srcPart, conn.sourceId || conn.source);
            const tgtPort = resolvePortForEndpointInData(tgtPart, conn.targetId || conn.target);
            if (!srcPort || !tgtPort) return;
            elkEdges.push({
                id: getConnectorId(conn, idx),
                sources: [elkPortIdFor(srcPart, srcPort)],
                targets: [elkPortIdFor(tgtPart, tgtPort)]
            });
        });

        const elkGraph = {
            id: 'root',
            layoutOptions: {
                'elk.algorithm': 'layered',
                'elk.hierarchyHandling': 'INCLUDE_CHILDREN',
                'elk.direction': 'RIGHT',
                'elk.spacing.nodeNode': String(verticalSpacing),
                'elk.layered.spacing.nodeNodeBetweenLayers': String(horizontalSpacing),
                'elk.spacing.edgeNode': '110',
                'elk.spacing.edgeEdge': '90',
                'elk.edgeRouting': 'ORTHOGONAL',
                'elk.layered.nodePlacement.strategy': 'NETWORK_SIMPLEX',
                'elk.layered.crossingMinimization.strategy': 'LAYER_SWEEP',
                'elk.separateConnectedComponents': 'true',
                'elk.padding': '[top=84,left=84,bottom=84,right=84]',
                'org.eclipse.elk.portConstraints': 'FIXED_ORDER',
                'org.eclipse.elk.portAlignment.default': 'CENTER',
                'org.eclipse.elk.json.edgeCoords': 'ROOT'
            },
            children: partForest.map((rootNode) => treeToElkNode(rootNode)),
            edges: elkEdges
        };

        elkLaidOut = await elk.layout(elkGraph);
    } catch (e) {
        console.error('[IBD] ELK layout failed:', e);
        renderPlaceholder(
            width,
            height,
            'Interconnection View',
            'ELK layout failed for this diagram.\n\nThe Interconnection View no longer falls back to heuristic routing because that produced misleading connector paths.',
            data
        );
        return;
    }

    const partPositions = new Map<string, { x: number; y: number; part: any; height: number; width?: number; isContainer?: boolean }>();
    const innerMargin = 24;

    const relativePath = (qualifiedName: string) => qualifiedName;

    const setPos = (part: any, posData: { x: number; y: number; part: any; height: number; width?: number; isContainer?: boolean; depth?: number }) => {
        partPositions.set(part.name, posData);
        partPositions.set(part.id, posData);
        if (part.qualifiedName && part.qualifiedName !== part.name) {
            partPositions.set(part.qualifiedName, posData);
        }
        partPositions.set(partToElkId(part), posData);
        const rel = relativePath(part.qualifiedName || part.name);
        if (rel && rel !== part.name) partPositions.set(rel, posData);
    };

    const idToPart = new Map<string, any>();
    const elkPortPositions = new Map<string, { x: number; y: number; side?: string; partId: string }>();
    parts.forEach((p: any) => idToPart.set(partToElkId(p), p));

    const extractElkPositions = (elkNode: any, parentAbsX: number, parentAbsY: number, depth: number) => {
        const absX = parentAbsX + (elkNode.x ?? 0);
        const absY = parentAbsY + (elkNode.y ?? 0);
        const part = idToPart.get(elkNode.id);
        if (!part) {
            if (elkNode.children && elkNode.children.length > 0) {
                elkNode.children.forEach((ch: any) => extractElkPositions(ch, absX, absY, depth));
            }
            return;
        }
        const w = elkNode.width ?? (partWidths.get(part.name) || partWidth);
        const h = elkNode.height ?? (partHeights.get(part.name) || 80);
        const isContainer = elkNode.children && elkNode.children.length > 0;
        const posData = { x: absX + padding, y: absY + padding, part, height: h, width: w, isContainer, depth };
        setPos(part, posData);
        (elkNode.ports ?? []).forEach((portNode: any) => {
            const side = portNode?.layoutOptions?.["org.eclipse.elk.port.side"];
            const portWidth = portNode.width ?? 10;
            const portHeight = portNode.height ?? 10;
            // ELK routes to the port border on the declared side.
            const anchorX = side === 'WEST'
                ? absX + (portNode.x ?? 0) + padding
                : side === 'EAST'
                    ? absX + (portNode.x ?? 0) + portWidth + padding
                    : absX + (portNode.x ?? 0) + portWidth / 2 + padding;
            const anchorY = absY + (portNode.y ?? 0) + portHeight / 2 + padding;
            elkPortPositions.set(portNode.id, {
                x: anchorX,
                y: anchorY,
                side,
                partId: elkNode.id,
            });
        });
        if (elkNode.children && elkNode.children.length > 0) {
            elkNode.children.forEach((ch: any) => extractElkPositions(ch, absX, absY, depth + 1));
        }
    };

    if (!elkLaidOut?.children?.length) {
        renderPlaceholder(
            width,
            height,
            'Interconnection View',
            'ELK layout did not return positioned nodes for this diagram.',
            data
        );
        return;
    }
    elkLaidOut.children.forEach((rootElk: any) => extractElkPositions(rootElk, 0, 0, 0));
    const connectorSectionsById: EdgeSectionsMap = new Map<string, ElkSection[]>();
    const collectSections = (
        node: any,
        acc: EdgeSectionsMap
    ): void => {
        (node?.edges || []).forEach((edge: any) => {
            const edgeId = String(edge?.id || '');
            if (edgeId && Array.isArray(edge?.sections) && edge.sections.length > 0) {
                acc.set(edgeId, edge.sections as ElkSection[]);
            }
        });
        (node?.children || []).forEach((c: any) => collectSections(c, acc));
    };
    collectSections(elkLaidOut, connectorSectionsById);
    const ibdLayoutResult: IbdLayoutResult = {
        partPositions,
        portPositions: elkPortPositions,
        connectorSectionsById,
        bounds: {
            x: 0,
            y: 0,
            width: elkLaidOut?.width ?? 0,
            height: elkLaidOut?.height ?? 0,
        },
    };
    if (ibdLayoutResult.partPositions.size === 0) {
        renderPlaceholder(
            width,
            height,
            'Interconnection View',
            'ELK layout returned no part positions for this diagram.',
            data
        );
        return;
    }

    type Rect = { x: number; y: number; width: number; height: number };

    /** Parse SVG path d string (M/L only) into points. */
    const parsePathToPoints = (d: string): { x: number; y: number }[] => {
        const pts: { x: number; y: number }[] = [];
        const tokens = d.replace(/[ML]/gi, ' ').replace(/,/g, ' ').trim().split(/\s+/);
        let i = 0;
        while (i + 1 < tokens.length) {
            const x = parseFloat(tokens[i]);
            const y = parseFloat(tokens[i + 1]);
            if (!Number.isNaN(x) && !Number.isNaN(y)) pts.push({ x, y });
            i += 2;
        }
        return pts;
    };

    const pointsToPathD = (pts: { x: number; y: number }[]): string => {
        if (pts.length === 0) return '';
        let s = 'M' + pts[0].x + ',' + pts[0].y;
        for (let i = 1; i < pts.length; i++) s += ' L' + pts[i].x + ',' + pts[i].y;
        return s;
    };
    const minimalFallbackRoute = (srcX: number, srcY: number, tgtX: number, tgtY: number): { x: number; y: number }[] => {
        const midX = (srcX + tgtX) / 2;
        return pruneRoutePoints([
            { x: srcX, y: srcY },
            { x: midX, y: srcY },
            { x: midX, y: tgtY },
            { x: tgtX, y: tgtY },
        ]);
    };

    const pruneRoutePoints = (points: { x: number; y: number }[]): { x: number; y: number }[] => {
        const pruned: { x: number; y: number }[] = [];
        for (const point of points) {
            const last = pruned[pruned.length - 1];
            if (last && Math.abs(last.x - point.x) < 1e-6 && Math.abs(last.y - point.y) < 1e-6) {
                continue;
            }
            pruned.push({ x: point.x, y: point.y });
            while (pruned.length >= 3) {
                const a = pruned[pruned.length - 3];
                const b = pruned[pruned.length - 2];
                const c = pruned[pruned.length - 1];
                const sameX = Math.abs(a.x - b.x) < 1e-6 && Math.abs(b.x - c.x) < 1e-6;
                const sameY = Math.abs(a.y - b.y) < 1e-6 && Math.abs(b.y - c.y) < 1e-6;
                if (!sameX && !sameY) break;
                pruned.splice(pruned.length - 2, 1);
            }
        }
        return pruned;
    };

    const routeLength = (points: { x: number; y: number }[]): number => points.reduce((sum, point, index) => {
        if (index === 0) return 0;
        const prev = points[index - 1];
        return sum + Math.abs(point.x - prev.x) + Math.abs(point.y - prev.y);
    }, 0);

    type RouteMeta = {
        points: { x: number; y: number }[];
        sourceEndpoint: string;
        targetEndpoint: string;
        sourcePart: string;
        targetPart: string;
    };

    const normalizeRange = (a: number, b: number) => ({
        start: Math.min(a, b),
        end: Math.max(a, b),
    });

    const rangesOverlap = (a1: number, a2: number, b1: number, b2: number, margin: number = 6): boolean => {
        const first = normalizeRange(a1, a2);
        const second = normalizeRange(b1, b2);
        return Math.min(first.end, second.end) - Math.max(first.start, second.start) > margin;
    };

    const routeSharesEndpoint = (a: RouteMeta, b: RouteMeta): boolean =>
        a.sourceEndpoint === b.sourceEndpoint
        || a.sourceEndpoint === b.targetEndpoint
        || a.targetEndpoint === b.sourceEndpoint
        || a.targetEndpoint === b.targetEndpoint;

    const segmentOverlapPenalty = (
        a1: { x: number; y: number },
        a2: { x: number; y: number },
        b1: { x: number; y: number },
        b2: { x: number; y: number },
    ): number => {
        const aHoriz = Math.abs(a1.y - a2.y) < 1e-6;
        const aVert = Math.abs(a1.x - a2.x) < 1e-6;
        const bHoriz = Math.abs(b1.y - b2.y) < 1e-6;
        const bVert = Math.abs(b1.x - b2.x) < 1e-6;
        if (aHoriz && bHoriz && Math.abs(a1.y - b1.y) < 6 && rangesOverlap(a1.x, a2.x, b1.x, b2.x, 10)) {
            return 5000;
        }
        if (aVert && bVert && Math.abs(a1.x - b1.x) < 6 && rangesOverlap(a1.y, a2.y, b1.y, b2.y, 10)) {
            return 5000;
        }
        return 0;
    };

    const routeOverlapPenalty = (
        points: { x: number; y: number }[],
        existingRoutes: RouteMeta[],
        currentMeta: Omit<RouteMeta, 'points'>,
    ): number => {
        let penalty = 0;
        for (let routeIndex = 0; routeIndex < existingRoutes.length; routeIndex++) {
            const existing = existingRoutes[routeIndex];
            if (routeSharesEndpoint({ ...currentMeta, points }, existing)) {
                continue;
            }
            for (let i = 0; i < points.length - 1; i++) {
                for (let j = 0; j < existing.points.length - 1; j++) {
                    penalty += segmentOverlapPenalty(
                        points[i],
                        points[i + 1],
                        existing.points[j],
                        existing.points[j + 1],
                    );
                }
            }
        }
        return penalty;
    };

    const pathIntersectsObstacles = (points: { x: number; y: number }[], obstacles: Rect[], margin: number): boolean => {
        for (let i = 0; i < points.length - 1; i++) {
            const a = points[i];
            const b = points[i + 1];
            const isHoriz = Math.abs(a.y - b.y) < 1e-6;
            const isVert = Math.abs(a.x - b.x) < 1e-6;
            if (!isHoriz && !isVert) continue;
            for (const obstacle of obstacles) {
                const hits = isHoriz
                    ? rectIntersectsHSeg(a.y, a.x, b.x, obstacle, margin)
                    : rectIntersectsVSeg(a.x, a.y, b.y, obstacle, margin);
                if (hits) return true;
            }
        }
        return false;
    };

    const pathIntersectsObstaclesIgnoringEndpointStubs = (
        points: { x: number; y: number }[],
        obstacles: Rect[],
        margin: number,
    ): boolean => {
        if (points.length < 2) return false;
        for (let i = 0; i < points.length - 1; i++) {
            if (i === 0 || i === points.length - 2) {
                continue;
            }
            const a = points[i];
            const b = points[i + 1];
            const isHoriz = Math.abs(a.y - b.y) < 1e-6;
            const isVert = Math.abs(a.x - b.x) < 1e-6;
            if (!isHoriz && !isVert) continue;
            for (const obstacle of obstacles) {
                const hits = isHoriz
                    ? rectIntersectsHSeg(a.y, a.x, b.x, obstacle, margin)
                    : rectIntersectsVSeg(a.x, a.y, b.y, obstacle, margin);
                if (hits) return true;
            }
        }
        return false;
    };

    const getBestLabelAnchor = (points: { x: number; y: number }[], fallbackX: number, fallbackY: number) => {
        if (points.length < 2) return { x: fallbackX, y: fallbackY };
        let bestIndex = 0;
        let bestLength = -1;
        for (let i = 0; i < points.length - 1; i++) {
            const dx = Math.abs(points[i + 1].x - points[i].x);
            const dy = Math.abs(points[i + 1].y - points[i].y);
            const length = dx + dy;
            if (length > bestLength) {
                bestLength = length;
                bestIndex = i;
            }
        }
        const a = points[bestIndex];
        const b = points[bestIndex + 1];
        const mostlyVertical = Math.abs(a.y - b.y) > Math.abs(a.x - b.x);
        return {
            x: (a.x + b.x) / 2,
            y: (a.y + b.y) / 2 + (mostlyVertical ? 0 : -12),
        };
    };

    const rectIntersectsHSeg = (y: number, x1: number, x2: number, r: Rect, margin: number): boolean => {
        const yMin = r.y - margin, yMax = r.y + r.height + margin;
        if (y < yMin || y > yMax) return false;
        const xMin = Math.min(x1, x2) - margin, xMax = Math.max(x1, x2) + margin;
        return !(xMax < r.x || xMin > r.x + r.width);
    };

    const rectIntersectsVSeg = (x: number, y1: number, y2: number, r: Rect, margin: number): boolean => {
        const xMin = r.x - margin, xMax = r.x + r.width + margin;
        if (x < xMin || x > xMax) return false;
        const yMin = Math.min(y1, y2) - margin, yMax = Math.max(y1, y2) + margin;
        return !(yMax < r.y || yMin > r.y + r.height);
    };

    const OBSTACLE_MARGIN = 16;
    const MAX_DETOUR_DEPTH = 4;

    /** Insert detours so no segment crosses a leaf-node obstacle. Containers are not treated as obstacles. */
    const routeAroundLeafObstacles = (
        points: { x: number; y: number }[],
        obstacles: Rect[],
        depth: number = 0
    ): { x: number; y: number }[] => {
        if (points.length < 2 || obstacles.length === 0 || depth >= MAX_DETOUR_DEPTH) return points;

        for (let i = 0; i < points.length - 1; i++) {
            const a = points[i], b = points[i + 1];
            const isHoriz = Math.abs(a.y - b.y) < 1e-6;
            const isVert = Math.abs(a.x - b.x) < 1e-6;
            if (!isHoriz && !isVert) continue;

            for (const r of obstacles) {
                const hits = isHoriz ? rectIntersectsHSeg(a.y, a.x, b.x, r, OBSTACLE_MARGIN)
                    : rectIntersectsVSeg(a.x, a.y, b.y, r, OBSTACLE_MARGIN);
                if (!hits) continue;

                if (isHoriz) {
                    const aboveY = r.y - OBSTACLE_MARGIN;
                    const belowY = r.y + r.height + OBSTACLE_MARGIN;
                    const above: { x: number; y: number }[] = [a, { x: a.x, y: aboveY }, { x: b.x, y: aboveY }, b];
                    const below: { x: number; y: number }[] = [a, { x: a.x, y: belowY }, { x: b.x, y: belowY }, b];
                    const newObs = obstacles.filter((o) => o !== r);
                    const mergedAbove = [...points.slice(0, i), ...above, ...points.slice(i + 2)];
                    const mergedBelow = [...points.slice(0, i), ...below, ...points.slice(i + 2)];
                    const resultAbove = routeAroundLeafObstacles(mergedAbove, newObs, depth + 1);
                    const resultBelow = routeAroundLeafObstacles(mergedBelow, newObs, depth + 1);
                    const lenAbove = resultAbove.reduce((sum, p, j) => {
                        if (j === 0) return 0;
                        return sum + Math.abs(resultAbove[j].x - resultAbove[j - 1].x) + Math.abs(resultAbove[j].y - resultAbove[j - 1].y);
                    }, 0);
                    const lenBelow = resultBelow.reduce((sum, p, j) => {
                        if (j === 0) return 0;
                        return sum + Math.abs(resultBelow[j].x - resultBelow[j - 1].x) + Math.abs(resultBelow[j].y - resultBelow[j - 1].y);
                    }, 0);
                    return lenAbove <= lenBelow ? resultAbove : resultBelow;
                } else {
                    const leftX = r.x - OBSTACLE_MARGIN;
                    const rightX = r.x + r.width + OBSTACLE_MARGIN;
                    const left: { x: number; y: number }[] = [a, { x: leftX, y: a.y }, { x: leftX, y: b.y }, b];
                    const right: { x: number; y: number }[] = [a, { x: rightX, y: a.y }, { x: rightX, y: b.y }, b];
                    const newObs = obstacles.filter((o) => o !== r);
                    const mergedLeft = [...points.slice(0, i), ...left, ...points.slice(i + 2)];
                    const mergedRight = [...points.slice(0, i), ...right, ...points.slice(i + 2)];
                    const resultLeft = routeAroundLeafObstacles(mergedLeft, newObs, depth + 1);
                    const resultRight = routeAroundLeafObstacles(mergedRight, newObs, depth + 1);
                    const lenLeft = resultLeft.reduce((sum, p, j) => {
                        if (j === 0) return 0;
                        return sum + Math.abs(resultLeft[j].x - resultLeft[j - 1].x) + Math.abs(resultLeft[j].y - resultLeft[j - 1].y);
                    }, 0);
                    const lenRight = resultRight.reduce((sum, p, j) => {
                        if (j === 0) return 0;
                        return sum + Math.abs(resultRight[j].x - resultRight[j - 1].x) + Math.abs(resultRight[j].y - resultRight[j - 1].y);
                    }, 0);
                    return lenLeft <= lenRight ? resultLeft : resultRight;
                }
            }
        }
        return points;
    };

    const pickBestOrthogonalRoute = (
        sourcePoint: { x: number; y: number; isLeft?: boolean; order?: number } | null,
        targetPoint: { x: number; y: number; isLeft?: boolean; order?: number } | null,
        sourcePartPos: { x: number; y: number; width?: number } | null,
        targetPartPos: { x: number; y: number; width?: number } | null,
        obstacles: Rect[],
        baseOffset: number,
        routeSpread: number,
        existingRoutes: RouteMeta[],
        routeMeta: Omit<RouteMeta, 'points'>,
    ): { x: number; y: number }[] | null => {
        if (!sourcePoint || !targetPoint) return null;

        const sourceDirection = sourcePoint.isLeft ? -1 : 1;
        const targetDirection = targetPoint.isLeft ? -1 : 1;
        const approach = 28 + Math.max(routeSpread, Math.abs(baseOffset) * 0.35);
        const sourceLaneSpread = (sourcePoint.order ?? 0) * 32;
        const targetLaneSpread = (targetPoint.order ?? 0) * 32;
        const sourceStubX = sourcePoint.x + sourceDirection * (approach + sourceLaneSpread);
        const targetStubX = targetPoint.x + targetDirection * (approach + targetLaneSpread);
        const minX = Math.min(sourcePoint.x, targetPoint.x, sourceStubX, targetStubX);
        const maxX = Math.max(sourcePoint.x, targetPoint.x, sourceStubX, targetStubX);
        const minY = Math.min(sourcePoint.y, targetPoint.y);
        const maxY = Math.max(sourcePoint.y, targetPoint.y);
        const sourceMidX = sourcePartPos ? sourcePartPos.x + ((sourcePartPos.width ?? partWidth) / 2) : sourcePoint.x;
        const targetMidX = targetPartPos ? targetPartPos.x + ((targetPartPos.width ?? partWidth) / 2) : targetPoint.x;
        const outerLeftX = Math.min(
            minX,
            ...(obstacles.map((obstacle) => obstacle.x))
        ) - 34 - routeSpread;
        const outerRightX = Math.max(
            maxX,
            ...(obstacles.map((obstacle) => obstacle.x + obstacle.width))
        ) + 34 + routeSpread;
        const outerTopY = Math.min(
            minY,
            ...(obstacles.map((obstacle) => obstacle.y))
        ) - 26 - routeSpread;
        const outerBottomY = Math.max(
            maxY,
            ...(obstacles.map((obstacle) => obstacle.y + obstacle.height))
        ) + 26 + routeSpread;
        const endpointObstacles: Rect[] = [];
        if (sourcePartPos) {
            endpointObstacles.push({
                x: sourcePartPos.x,
                y: sourcePartPos.y,
                width: sourcePartPos.width ?? partWidth,
                height: (sourcePartPos as { height?: number }).height ?? 80,
            });
        }
        if (targetPartPos) {
            endpointObstacles.push({
                x: targetPartPos.x,
                y: targetPartPos.y,
                width: targetPartPos.width ?? partWidth,
                height: (targetPartPos as { height?: number }).height ?? 80,
            });
        }
        const allObstacles = [...obstacles, ...endpointObstacles];

        const candidateRoutes: { x: number; y: number }[][] = [];
        const pushRoute = (points: { x: number; y: number }[]) => {
            const pruned = pruneRoutePoints(points);
            if (pruned.length >= 2) {
                candidateRoutes.push(pruned);
            }
        };

        const candidateVerticalXs = Array.from(new Set([
            (sourceStubX + targetStubX) / 2 + baseOffset,
            sourceStubX + baseOffset,
            targetStubX + baseOffset,
            outerLeftX,
            outerRightX,
            sourceMidX + baseOffset,
            targetMidX + baseOffset,
        ]));

        const candidateHorizontalYs = Array.from(new Set([
            (sourcePoint.y + targetPoint.y) / 2 + baseOffset * 0.25,
            sourcePoint.y + baseOffset,
            targetPoint.y + baseOffset,
            outerTopY,
            outerBottomY,
        ]));

        candidateVerticalXs.forEach((laneX) => {
            pushRoute([
                { x: sourcePoint.x, y: sourcePoint.y },
                { x: sourceStubX, y: sourcePoint.y },
                { x: laneX, y: sourcePoint.y },
                { x: laneX, y: targetPoint.y },
                { x: targetStubX, y: targetPoint.y },
                { x: targetPoint.x, y: targetPoint.y },
            ]);
        });

        candidateHorizontalYs.forEach((laneY) => {
            pushRoute([
                { x: sourcePoint.x, y: sourcePoint.y },
                { x: sourceStubX, y: sourcePoint.y },
                { x: sourceStubX, y: laneY },
                { x: targetStubX, y: laneY },
                { x: targetStubX, y: targetPoint.y },
                { x: targetPoint.x, y: targetPoint.y },
            ]);
        });

        const scoreRoute = (route: { x: number; y: number }[]) => {
            const obstaclePenalty = pathIntersectsObstacles(route, obstacles, OBSTACLE_MARGIN)
                || pathIntersectsObstaclesIgnoringEndpointStubs(route, allObstacles, OBSTACLE_MARGIN)
                ? 1_000_000
                : 0;
            const overlapPenalty = routeOverlapPenalty(route, existingRoutes, routeMeta);
            const bendPenalty = Math.max(0, route.length - 2) * 20;
            return obstaclePenalty + overlapPenalty + bendPenalty + routeLength(route);
        };

        const cleanRoutes = candidateRoutes.filter((route) =>
            !pathIntersectsObstacles(route, obstacles, OBSTACLE_MARGIN)
            && !pathIntersectsObstaclesIgnoringEndpointStubs(route, allObstacles, OBSTACLE_MARGIN)
            && routeOverlapPenalty(route, existingRoutes, routeMeta) === 0
        );
        if (cleanRoutes.length > 0) {
            return cleanRoutes.sort((a, b) => scoreRoute(a) - scoreRoute(b))[0];
        }

        const detouredRoutes = candidateRoutes
            .map((route) => pruneRoutePoints(routeAroundLeafObstacles(route, allObstacles)))
            .filter((route) =>
                route.length >= 2
                && !pathIntersectsObstacles(route, obstacles, OBSTACLE_MARGIN)
                && !pathIntersectsObstaclesIgnoringEndpointStubs(route, allObstacles, OBSTACLE_MARGIN)
                && routeOverlapPenalty(route, existingRoutes, routeMeta) === 0
            );
        if (detouredRoutes.length > 0) {
            return detouredRoutes.sort((a, b) => scoreRoute(a) - scoreRoute(b))[0];
        }

        return candidateRoutes.length > 0
            ? candidateRoutes.sort((a, b) => scoreRoute(a) - scoreRoute(b))[0]
            : null;
    };

    /** Build route points from ELK edge sections, handling either ROOT or parent-relative coordinates. */
    const pointsFromElkSectionsWithPorts = (
        sections: Array<{ startPoint?: { x: number; y: number }; endPoint?: { x: number; y: number }; bendPoints?: Array<{ x: number; y: number }> }> | undefined,
        edgeParentOffset: { x: number; y: number },
        srcX: number, srcY: number, tgtX: number, tgtY: number,
        sourceIsLeft: boolean,
        targetIsLeft: boolean,
        routingObstacles: Rect[],
        endpointObstacles: Rect[],
    ): { points: { x: number; y: number }[]; frame: 'parent'; diagnostics: RouteFrameDiagnostics } | null => {
        if (!sections || sections.length === 0) return null;
        const parentFrame = (() => {
            const routePoints: { x: number; y: number }[] = [];
            for (const sec of sections) {
                if (sec?.startPoint) {
                    routePoints.push({
                        x: sec.startPoint.x + edgeParentOffset.x + padding,
                        y: sec.startPoint.y + edgeParentOffset.y + padding,
                    });
                }
                for (const bend of sec?.bendPoints || []) {
                    routePoints.push({
                        x: bend.x + edgeParentOffset.x + padding,
                        y: bend.y + edgeParentOffset.y + padding,
                    });
                }
                if (sec?.endPoint) {
                    routePoints.push({
                        x: sec.endPoint.x + edgeParentOffset.x + padding,
                        y: sec.endPoint.y + edgeParentOffset.y + padding,
                    });
                }
            }
            return pruneRoutePoints(routePoints);
        })();
        if (parentFrame.length === 0) return null;

        const endpointError = (pts: { x: number; y: number }[]) => {
            if (pts.length < 2) return Number.POSITIVE_INFINITY;
            const start = pts[0];
            const end = pts[pts.length - 1];
            const srcDist = Math.hypot(start.x - srcX, start.y - srcY);
            const tgtDist = Math.hypot(end.x - tgtX, end.y - tgtY);
            return srcDist + tgtDist;
        };
        const parentErr = endpointError(parentFrame);

        const outwardPenalty = (pts: { x: number; y: number }[]) => {
            if (pts.length < 2) return Number.POSITIVE_INFINITY;
            const first = pts[0];
            const second = pts[1];
            const penultimate = pts[pts.length - 2];
            const last = pts[pts.length - 1];
            const sourceOutward = sourceIsLeft ? (second.x <= first.x) : (second.x >= first.x);
            const targetOutward = targetIsLeft ? (penultimate.x <= last.x) : (penultimate.x >= last.x);
            let penalty = 0;
            if (!sourceOutward) penalty += 5_000;
            if (!targetOutward) penalty += 5_000;
            return penalty;
        };

        const parentScore = parentErr + outwardPenalty(parentFrame);
        const scoreWithObstacles = (baseScore: number, pts: { x: number; y: number }[]) => {
            const allObstacles = [...routingObstacles, ...endpointObstacles];
            const crossesNonEndpoints = pathIntersectsObstacles(pts, routingObstacles, OBSTACLE_MARGIN);
            const crossesBodies = pathIntersectsObstaclesIgnoringEndpointStubs(pts, allObstacles, OBSTACLE_MARGIN);
            const obstaclePenalty = crossesNonEndpoints || crossesBodies ? 1_000_000 : 0;
            return baseScore + obstaclePenalty;
        };
        const parentFinalScore = scoreWithObstacles(parentScore, parentFrame);
        const diagnostics: RouteFrameDiagnostics = {
            rootErr: parentErr,
            parentErr,
            rootFinalScore: parentFinalScore,
            parentFinalScore,
            rootPoints: parentFrame,
            parentPoints: parentFrame,
            translatedRootPoints: parentFrame,
            translatedRootErr: parentErr,
            translatedRootFinalScore: parentFinalScore,
        };
        return { points: parentFrame, frame: 'parent', diagnostics };
    };

    const preserveElkRoute = (
        rawRoute: { x: number; y: number }[],
        sourcePoint: { x: number; y: number },
        targetPoint: { x: number; y: number },
        sourcePartPos?: { x: number; y: number; width?: number },
        targetPartPos?: { x: number; y: number; width?: number },
        sourceIsLeft?: boolean,
        targetIsLeft?: boolean,
    ): { x: number; y: number }[] => {
        if (rawRoute.length === 0) return [];
        const route = rawRoute.map((point) => ({ x: point.x, y: point.y }));
        route[0] = { x: sourcePoint.x, y: sourcePoint.y };
        route[route.length - 1] = { x: targetPoint.x, y: targetPoint.y };

        const outsideOffset = 12;
        const sourceExit = sourcePartPos
            ? {
                x: sourceIsLeft
                    ? sourcePartPos.x - outsideOffset
                    : sourcePartPos.x + (sourcePartPos.width ?? partWidth) + outsideOffset,
                y: sourcePoint.y,
            }
            : route.length >= 2
                ? { x: route[1].x, y: route[0].y }
                : { x: route[0].x, y: route[0].y };
        const targetEntry = targetPartPos
            ? {
                x: targetIsLeft
                    ? targetPartPos.x - outsideOffset
                    : targetPartPos.x + (targetPartPos.width ?? partWidth) + outsideOffset,
                y: targetPoint.y,
            }
            : route.length >= 2
                ? { x: route[route.length - 2].x, y: route[route.length - 1].y }
                : { x: route[route.length - 1].x, y: route[route.length - 1].y };

        const orthogonalized: { x: number; y: number }[] = [route[0], sourceExit];
        const middleRoute = route.slice(1, -1);
        if (middleRoute.length > 0) {
            const firstMiddle = middleRoute[0];
            if (Math.abs(firstMiddle.x - sourceExit.x) > 1e-6 && Math.abs(firstMiddle.y - sourceExit.y) > 1e-6) {
                orthogonalized.push({ x: sourceExit.x, y: firstMiddle.y });
            }
            orthogonalized.push(...middleRoute);
            const lastMiddle = middleRoute[middleRoute.length - 1];
            if (Math.abs(lastMiddle.x - targetEntry.x) > 1e-6 && Math.abs(lastMiddle.y - targetEntry.y) > 1e-6) {
                orthogonalized.push({ x: lastMiddle.x, y: targetEntry.y });
            }
        } else if (Math.abs(sourceExit.x - targetEntry.x) > 1e-6 && Math.abs(sourceExit.y - targetEntry.y) > 1e-6) {
            orthogonalized.push({ x: sourceExit.x, y: targetEntry.y });
        }
        orthogonalized.push(targetEntry, route[route.length - 1]);
        return pruneRoutePoints(orthogonalized);
    };

    const findPartPos = (qualifiedName: string) => {
        if (!qualifiedName) return null;
        const normalized = qualifiedName.lastIndexOf('::') >= 0
            ? qualifiedName.substring(qualifiedName.lastIndexOf('::') + 2)
            : qualifiedName;

        if (partPositions.has(normalized)) {
            return partPositions.get(normalized)!;
        }
        if (partPositions.has(qualifiedName)) {
            return partPositions.get(qualifiedName)!;
        }

        const segments = normalized.split('.');

        for (let i = segments.length - 1; i >= 1; i--) {
            const partialPath = segments.slice(0, i).join('.');
            const pos = partPositions.get(partialPath);
            if (pos) return pos;
        }

        for (let i = segments.length - 1; i >= 0; i--) {
            const pos = partPositions.get(segments[i]);
            if (pos) return pos;
        }

        return null;
    };

    const getRenderedPortAnchor = (partPos: { x: number; y: number; part: any; width?: number } | null, port: any) => {
        if (!partPos || !port) return null;
        const part = partPos.part;
        const elkPort = elkPortPositions.get(elkPortIdFor(part, port));
        if (elkPort) {
            return {
                x: elkPort.x - partPos.x,
                y: elkPort.y - partPos.y,
                isLeft: (elkPort.side || 'WEST') === 'WEST',
            };
        }

        const { leftPorts, rightPorts } = getPortLayoutForPart(part);
        const isLeftPort = leftPorts.includes(port);
        const partNodeWidth = partPos.width ?? partWidth;
        const portSpacing = 24;
        const contentStartY = part.attributes && (part.attributes.get && (part.attributes.get('partType') || part.attributes.get('type'))) ? 50 : 38;
        const portStartY = contentStartY + 20;
        const index = isLeftPort ? leftPorts.findIndex((p: any) => p.name === port.name) : rightPorts.findIndex((p: any) => p.name === port.name);
        return {
            x: isLeftPort ? 0 : partNodeWidth,
            y: portStartY + index * portSpacing,
            isLeft: isLeftPort,
        };
    };

    const defs = svg.select('defs').empty() ? svg.append('defs') : svg.select('defs');

    defs.append('marker')
        .attr('id', 'ibd-flow-arrow')
        .attr('viewBox', '0 -5 10 10')
        .attr('refX', 10)
        .attr('refY', 0)
        .attr('markerWidth', 8)
        .attr('markerHeight', 8)
        .attr('orient', 'auto')
        .append('path')
        .attr('d', 'M0,-4L10,0L0,4Z')
        .style('fill', NEUTRAL_EDGE_BLUE);

    defs.append('marker')
        .attr('id', 'ibd-interface-arrow')
        .attr('viewBox', '0 -5 10 10')
        .attr('refX', 10)
        .attr('refY', 0)
        .attr('markerWidth', 8)
        .attr('markerHeight', 8)
        .attr('orient', 'auto')
        .append('path')
        .attr('d', 'M0,-4L10,0L0,4Z')
        .style('fill', 'none')
        .style('stroke', NEUTRAL_EDGE_BLUE)
        .style('stroke-width', '1.5px');

    defs.append('marker')
        .attr('id', 'ibd-connection-dot')
        .attr('viewBox', '0 0 10 10')
        .attr('refX', 5)
        .attr('refY', 5)
        .attr('markerWidth', 5)
        .attr('markerHeight', 5)
        .append('circle')
        .attr('cx', 5)
        .attr('cy', 5)
        .attr('r', 4)
        .style('fill', NEUTRAL_EDGE_BLUE);

    defs.append('marker')
        .attr('id', 'ibd-port-connector')
        .attr('viewBox', '0 0 8 8')
        .attr('refX', 4)
        .attr('refY', 4)
        .attr('markerWidth', 4)
        .attr('markerHeight', 4)
        .append('rect')
        .attr('x', 1)
        .attr('y', 1)
        .attr('width', 6)
        .attr('height', 6)
        .style('fill', 'var(--vscode-charts-purple)');

    let connectorGroup = g.append('g').attr('class', 'ibd-connectors');
    let usedLabelPositions: { x: number; y: number; width: number; height: number }[] = [];
    let pendingLabels: { x: number; y: number; width: number; height: number; text: string; strokeColor: string; isItemType?: boolean }[] = [];
    const findAvailableLabelPosition = (
        anchorX: number,
        anchorY: number,
        width: number,
        height: number
    ) => {
        const candidates = [
            { x: anchorX, y: anchorY },
            { x: anchorX, y: anchorY - 24 },
            { x: anchorX, y: anchorY + 24 },
            { x: anchorX - width * 0.55, y: anchorY },
            { x: anchorX + width * 0.55, y: anchorY },
            { x: anchorX - width * 0.45, y: anchorY - 22 },
            { x: anchorX + width * 0.45, y: anchorY - 22 },
            { x: anchorX - width * 0.45, y: anchorY + 22 },
            { x: anchorX + width * 0.45, y: anchorY + 22 },
        ];
        for (const candidate of candidates) {
            const hasOverlap = usedLabelPositions.some((pos) => {
                return Math.abs(pos.x - candidate.x) < (pos.width + width) / 2 + 10 &&
                    Math.abs(pos.y - candidate.y) < (pos.height + height) / 2 + 6;
            });
            if (!hasOverlap) return candidate;
        }
        return candidates[candidates.length - 1];
    };

    function drawIbdConnectors() {
        g.selectAll('.ibd-connectors').remove();
        g.selectAll('.ibd-connector-labels').remove();

        connectorGroup = g.append('g').attr('class', 'ibd-connectors');
        usedLabelPositions = [];
        pendingLabels = [];

        const nodePairConnectors = new Map<string, { connector: any; idx: number }[]>();
        const portConnections = new Map<string, { connector: any; idx: number }[]>();

        connectors.forEach((connector: any, idx: number) => {
            const srcPos = findPartPos(connector.sourceId);
            const tgtPos = findPartPos(connector.targetId);
            if (!srcPos || !tgtPos) return;

            const srcKey = srcPos.part.name;
            const tgtKey = tgtPos.part.name;
            const pairKey = srcKey < tgtKey ? srcKey + '|' + tgtKey : tgtKey + '|' + srcKey;

            if (!nodePairConnectors.has(pairKey)) {
                nodePairConnectors.set(pairKey, []);
            }
            nodePairConnectors.get(pairKey)!.push({ connector, idx });

            const srcPortName = connector.sourceId ? connector.sourceId.split('.').pop() : null;
            const tgtPortName = connector.targetId ? connector.targetId.split('.').pop() : null;
            const portKey = srcKey + '.' + (srcPortName || 'edge') + '->' + tgtKey + '.' + (tgtPortName || 'edge');

            if (!portConnections.has(portKey)) {
                portConnections.set(portKey, []);
            }
            portConnections.get(portKey)!.push({ connector, idx });
        });

        const allElkEdges = new Map<string, { edge: any; offset: { x: number; y: number } }>();
        if (elkLaidOut) collectElkEdgesWithOffsets(elkLaidOut, { x: 0, y: 0 }, allElkEdges);
        /** Obstacles used for route refinement; include leaf nodes and container headers. */
        const getRoutingObstaclesExcluding = (srcPartName: string, tgtPartName: string): Rect[] => {
            const rects: Rect[] = [];
            partPositions.forEach((pos, key) => {
                if (key !== pos.part.name) return;
                if (pos.part.name === srcPartName || pos.part.name === tgtPartName) return;
                const width = pos.width ?? partWidth;
                const height = pos.height;
                if (pos.isContainer) {
                    rects.push({
                        x: pos.x,
                        y: pos.y,
                        width,
                        height: rootHeaderHeight + 8,
                    });
                } else {
                    rects.push({
                        x: pos.x,
                        y: pos.y,
                        width,
                        height,
                    });
                }
            });
            return rects;
        };
        const parentKeyForPart = (part: any): string => normalizeEndpointId(part?.containerId || '');
        const partKeysForLookup = (part: any): string[] => {
            const keys = new Set<string>();
            const push = (v: string | null | undefined) => {
                const n = normalizeEndpointId(v || '');
                if (n) keys.add(n);
            };
            push(part?.qualifiedName);
            push(part?.name);
            push(part?.id);
            return Array.from(keys);
        };
        const partByKey = new Map<string, any>();
        parts.forEach((part: any) => {
            partKeysForLookup(part).forEach((k) => partByKey.set(k, part));
        });
        const getPartParent = (part: any): any | null => {
            const parentKey = parentKeyForPart(part);
            if (!parentKey) return null;
            return partByKey.get(parentKey) ?? null;
        };
        const getAncestorChain = (part: any): any[] => {
            const chain: any[] = [];
            let current: any | null = part;
            let guard = 0;
            while (current && guard < 128) {
                chain.push(current);
                current = getPartParent(current);
                guard += 1;
            }
            return chain;
        };
        const getLcaPart = (a: any, b: any): any | null => {
            const aChain = getAncestorChain(a);
            const bSet = new Set<string>(
                getAncestorChain(b).map((part) => normalizeEndpointId(part?.qualifiedName || part?.name || part?.id || ''))
            );
            for (const candidate of aChain) {
                const key = normalizeEndpointId(candidate?.qualifiedName || candidate?.name || candidate?.id || '');
                if (key && bSet.has(key)) return candidate;
            }
            return null;
        };

        const connectorOffsets = new Map<number, { offset: number; groupIndex: number; groupCount: number }>();
        nodePairConnectors.forEach((group) => {
            const count = group.length;
            const step = 36;
            group.forEach((item, i) => {
                const offset = (i - (count - 1) / 2) * step;
                connectorOffsets.set(item.idx, { offset, groupIndex: i, groupCount: count });
            });
        });
        portConnections.forEach((group) => {
            const count = group.length;
            if (count <= 1) return;
            const step = 14;
            group.forEach((item, i) => {
                const current = connectorOffsets.get(item.idx) || { offset: 0, groupIndex: i, groupCount: count };
                const localOffset = (i - (count - 1) / 2) * step;
                connectorOffsets.set(item.idx, {
                    offset: current.offset + localOffset,
                    groupIndex: i,
                    groupCount: count,
                });
            });
        });

        partPositions.forEach((pos, partName) => {
            if (partName !== pos.part.name) return;
            const part = pos.part;
            const partPorts = getPortsForPartRef(part);
            const portStartY = (part.attributes && (part.attributes.get && (part.attributes.get('partType') || part.attributes.get('type')))) ? 70 : 58;

            partPorts.forEach((p: any, i: number) => {
                const portY = pos.y + portStartY + i * 28;
                usedLabelPositions.push({ x: pos.x - 50, y: portY, width: 80, height: 20 });
                usedLabelPositions.push({ x: pos.x + (pos.width ?? partWidth) + 50, y: portY, width: 80, height: 20 });
            });
        });

        const getPortsForPart = (part: any) => ports.filter((p: any) =>
            p && (
                p.parentId === part.name ||
                p.parentId === part.id ||
                p.parentId === part.qualifiedName ||
                normalizeEndpointId(p.parentId) === normalizeEndpointId(part.qualifiedName) ||
                normalizeEndpointId(p.parentId) === normalizeEndpointId(part.name)
            )
        );

        const resolvePortForEndpoint = (part: any, endpointId: string | null): any => {
            if (!part || !endpointId) return null;
            const endpoint = normalizeEndpointId(endpointId);
            const endpointLeaf = endpoint.split('.').pop() || endpoint;
            const partPorts = getPortsForPart(part);
            return partPorts.find((p: any) => {
                const portName = normalizeEndpointId(p.name);
                const portQualifiedName = normalizeEndpointId(p.qualifiedName || p.id || '');
                return endpoint === portQualifiedName
                    || endpoint.endsWith('.' + portName)
                    || endpointLeaf === portName;
            }) ?? null;
        };

        const findPortPosition = (partPos: { x: number; y: number; part: any } | null, endpointId: string | null) => {
            if (!partPos || !endpointId) return null;

            const part = partPos.part;
            const partNodeWidth = (partPos as { width?: number }).width ?? partWidth;
            const partPorts = getPortsForPart(part);
            const port = resolvePortForEndpoint(part, endpointId);

            if (!port) return null;

             const elkPort = elkPortPositions.get(elkPortIdFor(part, port));
             const { leftPorts, rightPorts } = getPortLayoutForPart(part);
             const isLeftPort = leftPorts.includes(port);
             const order = isLeftPort
                 ? leftPorts.findIndex((p: any) => p.name === port.name)
                 : rightPorts.findIndex((p: any) => p.name === port.name);
             if (elkPort) {
                 return {
                     x: elkPort.x,
                     y: elkPort.y,
                     direction: getPortDirection(port),
                     isLeft: (elkPort.side || 'WEST') === 'WEST',
                     order,
                 };
             }

            const portDirection = getPortDirection(port);

            const portSize = 14;
            const portSpacing = 24;
            const contentStartY = part.attributes && (part.attributes.get && (part.attributes.get('partType') || part.attributes.get('type'))) ? 50 : 38;
            const portStartY = contentStartY + 20;

            let portY: number, portX: number;

            if (isLeftPort) {
                portY = partPos.y + portStartY + order * portSpacing;
                portX = partPos.x;
            } else {
                portY = partPos.y + portStartY + order * portSpacing;
                portX = partPos.x + partNodeWidth;
            }

            return { x: portX, y: portY, direction: portDirection, isLeft: isLeftPort, order };
        };

        const connectorRoutes: RouteMeta[] = [];
        const degradedReasons = new Set<string>();

        connectors.forEach((connector: any, connIdx: number) => {
            const srcPos = findPartPos(connector.sourceId);
            const tgtPos = findPartPos(connector.targetId);

            if (!srcPos || !tgtPos) return;

            const srcEndpointId = connector.sourceId || connector.source || null;
            const tgtEndpointId = connector.targetId || connector.target || null;

            const srcPortPos = findPortPosition(srcPos, srcEndpointId);
            const tgtPortPos = findPortPosition(tgtPos, tgtEndpointId);

            const srcHeight = srcPos.height || 80;
            const tgtHeight = tgtPos.height || 80;

            const offsetInfo = connectorOffsets.get(connIdx) || { offset: 0, groupIndex: 0, groupCount: 1 };
            const baseOffset = offsetInfo.offset;
            const routeSpread = offsetInfo.groupCount > 1 ? 18 + offsetInfo.groupCount * 4 : 0;

            let srcX: number, srcY: number, tgtX: number, tgtY: number;

            if (srcPortPos) {
                srcX = srcPortPos.x;
                srcY = srcPortPos.y;
            } else {
                const srcWidth = srcPos.width ?? partWidth;
                const tgtWidth = tgtPos.width ?? partWidth;
                const srcCx = srcPos.x + srcWidth / 2;
                const tgtCx = tgtPos.x + tgtWidth / 2;
                srcX = tgtCx > srcCx ? srcPos.x + srcWidth : srcPos.x;
                srcY = srcPos.y + srcHeight / 2;
            }

            if (tgtPortPos) {
                tgtX = tgtPortPos.x;
                tgtY = tgtPortPos.y;
            } else {
                const srcWidth = srcPos.width ?? partWidth;
                const tgtWidth = tgtPos.width ?? partWidth;
                const srcCx = srcPos.x + srcWidth / 2;
                const tgtCx = tgtPos.x + tgtWidth / 2;
                tgtX = tgtCx > srcCx ? tgtPos.x : tgtPos.x + tgtWidth;
                tgtY = tgtPos.y + tgtHeight / 2;
            }

            let pathPoints: { x: number; y: number }[] | null = null;
            let pathD: string;
            let labelX: number, labelY: number;
            const connectorId = getConnectorId(connector, connIdx);
            const routeObstacles = getRoutingObstaclesExcluding(srcPos.part.name, tgtPos.part.name);
            const endpointObstacles: Rect[] = [
                {
                    x: srcPos.x,
                    y: srcPos.y,
                    width: srcPos.width ?? partWidth,
                    height: srcPos.height || 80,
                },
                {
                    x: tgtPos.x,
                    y: tgtPos.y,
                    width: tgtPos.width ?? partWidth,
                    height: tgtPos.height || 80,
                },
            ];
            const elkEdgeRecord = allElkEdges.get(connectorId);
            const elkEdge = elkEdgeRecord?.edge;
            const edgeOwnerOffset = elkEdgeRecord?.offset ?? { x: 0, y: 0 };
            const lcaPart = getLcaPart(srcPos.part, tgtPos.part);
            const lcaPos = lcaPart ? partPositions.get(partToElkId(lcaPart)) : null;
            const lcaOffset = lcaPos
                ? { x: (lcaPos.x - padding), y: (lcaPos.y - padding) }
                : { x: 0, y: 0 };
            const effectiveEdgeOffset =
                (Math.abs(edgeOwnerOffset.x) > 1e-6 || Math.abs(edgeOwnerOffset.y) > 1e-6)
                    ? edgeOwnerOffset
                    : lcaOffset;
            const elkSections = elkEdge?.sections || ibdLayoutResult.connectorSectionsById.get(connectorId);
            const authoritativeRouteResult = elkSections && srcPortPos && tgtPortPos
                ? pointsFromElkSectionsWithPorts(
                    elkSections,
                    effectiveEdgeOffset,
                    srcX,
                    srcY,
                    tgtX,
                    tgtY,
                    !!srcPortPos.isLeft,
                    !!tgtPortPos.isLeft,
                    routeObstacles,
                    endpointObstacles,
                )
                : null;
            const authoritativeRoute = authoritativeRouteResult?.points ?? null;
            if (LOG_ROUTE_FRAME_SELECTION && authoritativeRouteResult && srcPortPos && tgtPortPos) {
                postMessage({
                    command: 'webviewLog',
                    level: 'info',
                    args: [
                        '[IBD route frame selection]',
                        {
                            connectorId,
                            source: srcEndpointId,
                            target: tgtEndpointId,
                            selectedFrame: authoritativeRouteResult.frame,
                            edgeContainerOffset: effectiveEdgeOffset,
                            edgeOwnerOffset,
                            lcaOffset,
                            sourceAnchor: { x: srcPortPos.x, y: srcPortPos.y, isLeft: srcPortPos.isLeft },
                            targetAnchor: { x: tgtPortPos.x, y: tgtPortPos.y, isLeft: tgtPortPos.isLeft },
                            scores: {
                                rootErr: authoritativeRouteResult.diagnostics.rootErr,
                                parentErr: authoritativeRouteResult.diagnostics.parentErr,
                                rootFinalScore: authoritativeRouteResult.diagnostics.rootFinalScore,
                                parentFinalScore: authoritativeRouteResult.diagnostics.parentFinalScore,
                                translatedRootErr: authoritativeRouteResult.diagnostics.translatedRootErr,
                                translatedRootFinalScore: authoritativeRouteResult.diagnostics.translatedRootFinalScore,
                            },
                            selectedPoints: authoritativeRouteResult.points.slice(0, 6),
                            rootPoints: authoritativeRouteResult.diagnostics.rootPoints.slice(0, 6),
                            parentPoints: authoritativeRouteResult.diagnostics.parentPoints.slice(0, 6),
                            translatedRootPoints: authoritativeRouteResult.diagnostics.translatedRootPoints?.slice(0, 6),
                        },
                    ],
                });
            }

            const routeEndpointDrift = (points: { x: number; y: number }[] | null) => {
                if (!points || points.length < 2) {
                    return Number.POSITIVE_INFINITY;
                }
                const start = points[0];
                const end = points[points.length - 1];
                return Math.hypot(start.x - srcX, start.y - srcY) + Math.hypot(end.x - tgtX, end.y - tgtY);
            };
            const authoritativeDrift = routeEndpointDrift(authoritativeRoute);
            const AUTHORITATIVE_ROUTE_DRIFT_MAX = 64;
            if (!authoritativeRoute || authoritativeRoute.length < 2 || authoritativeDrift > AUTHORITATIVE_ROUTE_DRIFT_MAX) {
                degradedReasons.add(`ELK did not return a usable route for connector ${connectorId}.`);
                pathPoints = minimalFallbackRoute(srcX, srcY, tgtX, tgtY);
            } else {
                pathPoints = authoritativeRoute;
            }

            const routeMeta = {
                sourceEndpoint: normalizeEndpointId(srcEndpointId || ''),
                targetEndpoint: normalizeEndpointId(tgtEndpointId || ''),
                sourcePart: srcPos.part.name,
                targetPart: tgtPos.part.name,
            };
            // Keep ELK route authoritative; no post-normalization.
            if (LOG_ENDPOINT_DRIFT && pathPoints.length >= 2 && srcPortPos && tgtPortPos) {
                const start = pathPoints[0];
                const end = pathPoints[pathPoints.length - 1];
                const srcDx = start.x - srcPortPos.x;
                const srcDy = start.y - srcPortPos.y;
                const tgtDx = end.x - tgtPortPos.x;
                const tgtDy = end.y - tgtPortPos.y;
                const srcDist = Math.hypot(srcDx, srcDy);
                const tgtDist = Math.hypot(tgtDx, tgtDy);
                if (srcDist > ENDPOINT_DRIFT_WARN_PX || tgtDist > ENDPOINT_DRIFT_WARN_PX) {
                    postMessage({
                        command: 'webviewLog',
                        level: 'warn',
                        args: [
                            '[IBD endpoint drift]',
                            {
                                connectorId,
                                source: srcEndpointId,
                                target: tgtEndpointId,
                                selectedFrame: authoritativeRouteResult?.frame ?? 'unknown',
                                edgeContainerOffset: effectiveEdgeOffset,
                                sourceAnchor: { x: srcPortPos.x, y: srcPortPos.y },
                                targetAnchor: { x: tgtPortPos.x, y: tgtPortPos.y },
                                routeStart: { x: start.x, y: start.y },
                                routeEnd: { x: end.x, y: end.y },
                                sourceDelta: { dx: srcDx, dy: srcDy, dist: srcDist },
                                targetDelta: { dx: tgtDx, dy: tgtDy, dist: tgtDist },
                            },
                        ],
                    });
                }
            }
            pathD = pointsToPathD(pathPoints);
            const anchor = getBestLabelAnchor(pathPoints, (srcX + tgtX) / 2, (srcY + tgtY) / 2);
            labelX = anchor.x;
            labelY = anchor.y;

            const visualStyle = getConnectorVisualStyle(connector);
            const { strokeStyle, strokeWidth, markerStart, markerEnd, strokeColor } = visualStyle;

            const originalStroke = strokeColor;
            const originalStrokeWidth = strokeWidth;

            const connectorPath = connectorGroup.append('path')
                .attr('d', pathD)
                .attr('class', 'ibd-connector')
                .attr('data-connector-id', connectorId)
                .attr('data-source', connector.sourceId || '')
                .attr('data-target', connector.targetId || '')
                .attr('data-route-points', (pathPoints ?? parsePathToPoints(pathD)).map((point) => `${point.x},${point.y}`).join(' '))
                .style('fill', 'none')
                .style('stroke', strokeColor)
                .style('stroke-width', strokeWidth)
                .style('stroke-dasharray', strokeStyle)
                .style('marker-start', markerStart)
                .style('marker-end', markerEnd)
                .style('cursor', 'pointer');

            connectorPath.on('click', function(event: any) {
                event.stopPropagation();
                d3.selectAll('.ibd-connector').each(function(this: any) {
                    const el = d3.select(this);
                    const origStroke = el.attr('data-original-stroke');
                    const origWidth = el.attr('data-original-width');
                    if (origStroke) {
                        el.style('stroke', origStroke)
                          .style('stroke-width', origWidth)
                          .classed('connector-highlighted', false);
                        el.attr('data-original-stroke', null)
                          .attr('data-original-width', null);
                    }
                });

                const self = d3.select(this);
                self.attr('data-original-stroke', originalStroke)
                    .attr('data-original-width', originalStrokeWidth)
                    .style('stroke', '#FFD700')
                    .style('stroke-width', '4px')
                    .classed('connector-highlighted', true);
                (this as any).parentNode.appendChild(this);

                postMessage({
                    command: 'connectorSelected',
                    source: connector.sourceId,
                    target: connector.targetId,
                    type: connector.type,
                    name: connector.name
                });
            });

            connectorPath.on('mouseenter', function(this: any) {
                const self = d3.select(this);
                if (!self.classed('connector-highlighted')) {
                    self.style('stroke-width', '3px');
                }
            });

            connectorPath.on('mouseleave', function(this: any) {
                const self = d3.select(this);
                if (!self.classed('connector-highlighted')) {
                    self.style('stroke-width', originalStrokeWidth);
                }
            });

            const normalizedRoute = pruneRoutePoints(pathPoints ?? parsePathToPoints(pathD));
            if (normalizedRoute.length >= 2) {
                connectorRoutes.push({
                    points: normalizedRoute,
                    ...routeMeta,
                });
            }

            const label = connector.name || '';
            if (label && label !== 'connection' && label !== 'connector') {
                const displayLabel = truncateLabel(label, 20);
                const labelWidth = displayLabel.length * 7 + 20;
                const labelHeight = 20;

                const positioned = findAvailableLabelPosition(labelX, labelY, labelWidth, labelHeight);
                const finalLabelX = positioned.x;
                const finalLabelY = positioned.y;

                usedLabelPositions.push({
                    x: finalLabelX,
                    y: finalLabelY,
                    width: labelWidth,
                    height: labelHeight
                });

                const typeIndicator = visualStyle.typeIndicator;

                pendingLabels.push({
                    x: finalLabelX,
                    y: finalLabelY,
                    width: labelWidth,
                    height: labelHeight,
                    text: typeIndicator + displayLabel,
                    strokeColor: strokeColor
                });
            }

            if (visualStyle.isFlow && connector.itemType) {
                const itemWidth = connector.itemType.length * 7 + 10;
                const itemHeight = 16;
                const positioned = findAvailableLabelPosition(labelX, labelY - 28, itemWidth, itemHeight);
                usedLabelPositions.push({
                    x: positioned.x,
                    y: positioned.y,
                    width: itemWidth,
                    height: itemHeight
                });
                pendingLabels.push({
                    x: positioned.x,
                    y: positioned.y,
                    width: itemWidth,
                    height: itemHeight,
                    text: '«' + connector.itemType + '»',
                    strokeColor: 'var(--vscode-charts-green)',
                    isItemType: true
                });
            }
        });

        const labelGroup = g.append('g').attr('class', 'ibd-connector-labels');
        pendingLabels.forEach(labelData => {
            if (labelData.isItemType) {
                labelGroup.append('text')
                    .attr('x', labelData.x)
                    .attr('y', labelData.y)
                    .attr('text-anchor', 'middle')
                    .text(labelData.text)
                    .style('font-size', '9px')
                    .style('font-style', 'italic')
                    .style('fill', labelData.strokeColor);
            } else {
                labelGroup.append('rect')
                    .attr('x', labelData.x - labelData.width / 2)
                    .attr('y', labelData.y - labelData.height / 2)
                    .attr('width', labelData.width)
                    .attr('height', labelData.height)
                    .attr('rx', 4)
                    .style('fill', 'var(--vscode-editor-background)')
                    .style('stroke', labelData.strokeColor)
                    .style('stroke-width', '1px');

                labelGroup.append('text')
                    .attr('x', labelData.x)
                    .attr('y', labelData.y + 4)
                    .attr('text-anchor', 'middle')
                    .text(labelData.text)
                    .style('font-size', '10px')
                    .style('font-weight', '600')
                    .style('fill', labelData.strokeColor);
            }
        });

        const leafBounds = Array.from(partPositions.values()).filter((pos) => !pos.isContainer);
        for (const route of connectorRoutes) {
            if (route.points.length < 2) continue;
            if (route.points.length >= 2) {
                const first = route.points[0];
                const second = route.points[1];
                const penultimate = route.points[route.points.length - 2];
                const last = route.points[route.points.length - 1];
                if (Math.abs(first.y - second.y) >= 1e-6 || Math.abs(penultimate.y - last.y) >= 1e-6) {
                    degradedReasons.add('Some connector routes do not approach side ports horizontally.');
                }
            }
            for (let index = 1; index < route.points.length - 2; index++) {
                const current = route.points[index];
                const next = route.points[index + 1];
                for (const bound of leafBounds) {
                    if (bound.part.name === route.sourcePart || bound.part.name === route.targetPart) continue;
                    const rect = {
                        x: bound.x,
                        y: bound.y,
                        width: bound.width ?? partWidth,
                        height: bound.height,
                    };
                    const hits = Math.abs(current.y - next.y) < 1e-6
                        ? rectIntersectsHSeg(current.y, current.x, next.x, rect, 1)
                        : rectIntersectsVSeg(current.x, current.y, next.y, rect, 1);
                    if (hits) {
                        degradedReasons.add(`Connector route intersects leaf node ${bound.part.name}.`);
                    }
                }
            }
        }

        if (degradedReasons.size > 0) {
            console.warn('[IBD] Degraded interconnection routing:', Array.from(degradedReasons));
            const statusEl = document.getElementById('status-text');
            if (statusEl) {
                statusEl.textContent = `Interconnection View degraded: ${Array.from(degradedReasons)[0]}`;
            }
        }
    }

    const partGroup = g.append('g').attr('class', 'ibd-parts');

    const drawnPartIds = new Set<string>();
    const partEntries = Array.from(partPositions.entries());
    // Draw by depth ascending so parents (root, intermediate containers) are behind children
    const byDepth = partEntries.sort((a, b) => {
        const da = (a[1] as any).depth ?? 999;
        const db = (b[1] as any).depth ?? 999;
        return da - db;
    });
    byDepth.forEach(([partName, pos]) => {
        try {
            if (partName !== pos.part.name) return;
            const partId = pos.part.id || pos.part.name;
            if (drawnPartIds.has(partId)) return;
            drawnPartIds.add(partId);

            const part = pos.part;

            if (!part || !part.name) {
                console.error('[IBD Render] Invalid part in partPositions:', part);
                return;
            }

            const typeLower = (part.type || '').toLowerCase();
            const typeColor = getTypeColor(part.type);
            const isLibValidated = isLibraryValidated(part);
            const isDefinition = typeLower.includes('def');
            const isUsage = !isDefinition;

            let typedByName: string | null = null;
            if (part.attributes && part.attributes.get) {
                typedByName = part.attributes.get('partType') || part.attributes.get('type') || part.attributes.get('typedBy');
            }
            if (!typedByName && part.partType) typedByName = part.partType;

            const partPorts = getPortsForPartRef(part);
            const partChildren = part.children || [];

            const contentLines: string[] = [];

            const formatProperties = (obj: any) => {
                const props: string[] = [];
                if (obj.properties) {
                    if (typeof obj.properties === 'object') {
                        Object.entries(obj.properties).forEach(([key, value]) => {
                            if (value !== null && value !== undefined) {
                                props.push('  :>> ' + key + ' = ' + value);
                            }
                        });
                    }
                }
                if (obj.attributes) {
                    if (typeof obj.attributes.forEach === 'function') {
                        obj.attributes.forEach((value: any, key: string) => {
                            if (value !== null && value !== undefined && key !== 'isRedefinition') {
                                props.push('  ' + key + ' = ' + value);
                            }
                        });
                    } else if (typeof obj.attributes === 'object') {
                        Object.entries(obj.attributes).forEach(([key, value]) => {
                            if (value !== null && value !== undefined && key !== 'isRedefinition') {
                                props.push('  ' + key + ' = ' + value);
                            }
                        });
                    }
                }
                return props;
            };

            partChildren.forEach((c: any) => {
                try {
                    if (!c || !c.name || !c.type) return;

                    if (c.type === 'part') {
                        contentLines.push('[part] ' + c.name);
                        contentLines.push(...formatProperties(c));
                        if (c.children && c.children.length > 0) {
                            c.children.forEach((grandchild: any) => {
                                if (grandchild.type === 'redefinition' && grandchild.name) {
                                    const value = grandchild.attributes && grandchild.attributes.get ?
                                        grandchild.attributes.get('value') :
                                        (grandchild.attributes && grandchild.attributes.value);
                                    if (value) {
                                        contentLines.push('  :>> ' + grandchild.name + ' = ' + value);
                                    }
                                }
                            });
                        }
                    } else if (c.type === 'redefinition') {
                        const value = c.attributes && c.attributes.get ?
                            c.attributes.get('value') :
                            (c.attributes && c.attributes.value);
                        if (value) {
                            contentLines.push(':>> ' + c.name + ' = ' + value);
                        }
                    } else if (c.type === 'attribute' || c.type === 'property') {
                        const valueStr = c.value !== undefined ? ' = ' + c.value : '';
                        contentLines.push('[attr] ' + c.name + valueStr);
                    } else if (c.type === 'state') {
                        contentLines.push('[state] ' + c.name);
                    }
                } catch {
                    // Skip problem children silently
                }
            });

        const lineHeight = 12;
        const headerHeight = typedByName ? 50 : 38;
        const contentHeight = contentLines.length * lineHeight + 10;
        const { leftPorts, rightPorts } = getPortLayoutForPart(part);
        const portRows = Math.max(leftPorts.length, rightPorts.length, partPorts.length > 0 ? 1 : 0);
        const portSpacing = 26;
        const portsHeight = partPorts.length > 0 ? (portRows * portSpacing + 22) : 0;
        const totalHeight = Math.max(96, headerHeight + contentHeight + portsHeight);
        const w = pos.width ?? partWidth;
        const h = pos.height ?? totalHeight;

        const partG = partGroup.append('g')
            .attr('transform', 'translate(' + pos.x + ',' + pos.y + ')')
            .attr('class', 'ibd-part' + (isDefinition ? ' definition-node' : ' usage-node') + (pos.isContainer ? ' ibd-container' : ''))
            .attr('data-element-name', part.name)
            .attr('data-bounds', [pos.x, pos.y, w, h].join(','))
            .style('cursor', 'pointer');

        if (pos.isContainer) {
            const depth = (pos as { depth?: number }).depth ?? 0;
            const isIntermediate = depth === 1;
            const containerFill = 'var(--vscode-editor-background)';
            const containerStrokeWidth = isIntermediate ? '2.4px' : '2px';
            partG.append('rect')
                .attr('width', w)
                .attr('height', h)
                .attr('rx', 8)
                .attr('class', 'graph-node-background')
                .attr('data-original-stroke', NEUTRAL_NODE_BORDER)
                .attr('data-original-width', containerStrokeWidth)
                .style('fill', containerFill)
                .style('stroke', NEUTRAL_NODE_BORDER)
                .style('stroke-width', containerStrokeWidth)
                .style('stroke-dasharray', isIntermediate ? '4,4' : 'none');
            partG.append('rect')
                .attr('width', w)
                .attr('height', rootHeaderHeight)
                .attr('rx', 6)
                .attr('y', 0)
                .style('fill', 'var(--vscode-button-secondaryBackground)');
            partG.append('text')
                .attr('x', w / 2)
                .attr('y', rootHeaderHeight / 2 + 4)
                .attr('text-anchor', 'middle')
                .text(part.name)
                .style('font-size', '11px')
                .style('font-weight', 'bold')
                .style('fill', NEUTRAL_TEXT);
            partG.on('click', function (event: any) {
                event.stopPropagation();
                clearVisualHighlights();
                partGroup.selectAll('.ibd-part').select('.graph-node-background, rect').each(function (this: any) {
                    const r = d3.select(this);
                    if (r.attr('data-original-stroke')) {
                        r.style('stroke', r.attr('data-original-stroke'))
                            .style('stroke-width', r.attr('data-original-width'));
                    }
                });
                partG.select('rect.graph-node-background').style('stroke', '#FFD700').style('stroke-width', '4px');
                const statusEl = document.getElementById('status-text');
                if (statusEl) statusEl.textContent = part.name + ' [' + (part.type || 'part') + ']';
            });

            const partPorts = getPortsForPartRef(part);
            const portSize = 10;
            partPorts.forEach((p: any) => {
                const anchor = getRenderedPortAnchor(pos, p);
                if (!anchor) return;
                const portColor = PORT_OUTLINE_GREEN;
                partG.append('rect')
                    .attr('class', 'port-icon')
                    .attr('x', anchor.x - portSize / 2)
                    .attr('y', anchor.y - portSize / 2)
                    .attr('width', portSize)
                    .attr('height', portSize)
                    .style('fill', 'none')
                    .style('stroke', portColor)
                    .style('stroke-width', '1.8px');
            });
            return;
        }

            const _ibdStroke = isLibValidated ? NEUTRAL_NODE_BORDER : NEUTRAL_NODE_BORDER;
        const _ibdStrokeW = isUsage ? '3px' : '2px';
        partG.append('rect')
            .attr('width', w)
            .attr('height', h)
            .attr('rx', isUsage ? 8 : 4)
            .attr('data-original-stroke', _ibdStroke)
            .attr('data-original-width', _ibdStrokeW)
            .style('fill', NEUTRAL_NODE_FILL)
            .style('stroke', _ibdStroke)
            .style('stroke-width', _ibdStrokeW)
            .style('stroke-dasharray', isDefinition ? '6,3' : 'none');

        partG.append('rect')
            .attr('y', 0)
            .attr('width', w)
            .attr('height', typedByName ? 41 : 33)
            .style('fill', 'var(--vscode-button-secondaryBackground)');

        let stereoDisplay = part.type || 'part';
        if (typeLower.includes('part def')) stereoDisplay = 'part def';
        else if (typeLower.includes('part')) stereoDisplay = 'part';
        else if (typeLower.includes('port def')) stereoDisplay = 'port def';
        else if (typeLower.includes('action def')) stereoDisplay = 'action def';
        else if (typeLower.includes('action')) stereoDisplay = 'action';

        partG.append('text')
            .attr('x', w / 2)
            .attr('y', 17)
            .attr('text-anchor', 'middle')
            .text('«' + stereoDisplay + '»')
            .style('font-size', '9px')
            .style('fill', NEUTRAL_TEXT);

        const displayName = part.name.length > 18 ? part.name.substring(0, 16) + '..' : part.name;
        partG.append('text')
            .attr('class', 'node-name-text')
            .attr('x', w / 2)
            .attr('y', 31)
            .attr('text-anchor', 'middle')
            .text(displayName)
            .style('font-size', '11px')
            .style('font-weight', 'bold')
            .style('fill', NEUTRAL_TEXT);

        if (typedByName) {
            partG.append('text')
                .attr('x', w / 2)
                .attr('y', 43)
                .attr('text-anchor', 'middle')
                .text(': ' + (typedByName.length > 18 ? typedByName.substring(0, 16) + '..' : typedByName))
                .style('font-size', '10px')
                .style('font-style', 'italic')
                .style('fill', NEUTRAL_TEXT);
        }

        const contentStartY = typedByName ? 50 : 38;

        contentLines.forEach((line, i) => {
            partG.append('text')
                .attr('x', 6)
                .attr('y', contentStartY + 8 + i * lineHeight)
                .text(line.length > 28 ? line.substring(0, 26) + '..' : line)
                .style('font-size', '9px')
                .style('fill', 'var(--vscode-descriptionForeground)');
        });

        const portSize = 10;
        const portStartY = contentStartY + 20;

        const drawPortLabel = (
            labelX: number,
            labelY: number,
            label: string,
            anchor: 'start' | 'end',
            fillColor: string,
        ) => {
            partG.append('text')
                .attr('x', labelX)
                .attr('y', labelY + 3)
                .attr('text-anchor', anchor)
                .text(label)
                .style('font-size', '8px')
                .style('font-weight', '500')
                .style('fill', fillColor);
        };

        leftPorts.forEach((p: any) => {
            const anchor = getRenderedPortAnchor(pos, p);
            if (!anchor) return;
            const portY = anchor.y;
            const portX = anchor.x;
            const portColor = PORT_OUTLINE_GREEN;
            partG.append('rect')
                .attr('class', 'port-icon')
                .attr('x', portX - portSize/2)
                .attr('y', portY - portSize/2)
                .attr('width', portSize)
                .attr('height', portSize)
                .style('fill', 'none')
                .style('stroke', portColor)
                .style('stroke-width', '1.8px');
            drawPortLabel(Math.min(w - 10, portX + 16), portY, p.name, 'start', portColor);
        });

        rightPorts.forEach((p: any) => {
            const anchor = getRenderedPortAnchor(pos, p);
            if (!anchor) return;
            const portY = anchor.y;
            const portX = anchor.x;
            const portColor = PORT_OUTLINE_GREEN;
            partG.append('rect')
                .attr('class', 'port-icon')
                .attr('x', portX - portSize/2)
                .attr('y', portY - portSize/2)
                .attr('width', portSize)
                .attr('height', portSize)
                .style('fill', 'none')
                .style('stroke', portColor)
                .style('stroke-width', '1.8px');
            drawPortLabel(Math.max(10, portX - 16), portY, p.name, 'end', portColor);
        });

            partG.on('click', function(event: any) {
                event.stopPropagation();
                clearVisualHighlights();
                const clickedPart = d3.select(this);
                clickedPart.classed('highlighted-element', true);
                clickedPart.select('rect')
                .style('stroke', DIAGRAM_STYLE.highlight)
                    .style('stroke-width', '3px');
                postJumpToElement(postMessage, { name: part.name, id: part.qualifiedName || part.id, uri: part.uri || undefined }, { skipCentering: true });
            })
            .on('dblclick', function(event: any) {
                event.stopPropagation();
                onStartInlineEdit(d3.select(this), part.name, pos.x, pos.y, w);
            });
            partG.style('cursor', 'pointer');
        } catch (error) {
            postMessage({
                command: 'webviewLog',
                level: 'error',
                args: ['[IBD Render] Failed to render part', pos?.part?.name, String(error)],
            });
            console.error('[IBD Render] Failed to render part', pos?.part?.name, error);
        }
    });

    drawIbdConnectors();
}

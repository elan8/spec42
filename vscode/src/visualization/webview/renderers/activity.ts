/**
 * Activity/Action Flow View renderer - ELK-backed action-flow layout with D3 rendering.
 */
/* eslint-disable @typescript-eslint/no-explicit-any */

import type { RenderContext } from '../types';
import { postJumpToElement } from '../jumpToElement';
import { DIAGRAM_STYLE } from '../styleTokens';

declare const d3: any;
declare const ELK: any;

type ActivityActionNode = {
    id: string;
    name: string;
    type?: string;
    kind?: string;
    parent?: string;
    inputs?: string[];
    outputs?: string[];
};

type ActivityFlow = {
    id?: string;
    from: string;
    to: string;
    guard?: string;
    condition?: string;
};

type ElkSection = {
    startPoint?: { x: number; y: number };
    endPoint?: { x: number; y: number };
    bendPoints?: Array<{ x: number; y: number }>;
};

type ActivityRenderContext = RenderContext & { elkWorkerUrl?: string };

const ACTION_WIDTH = 220;
const ACTION_HEIGHT = 68;
const DECISION_SIZE = 76;
const FORK_WIDTH = 220;
const FORK_HEIGHT = 14;
const TERMINAL_SIZE = 40;

function truncateToFit(text: string | null | undefined, maxChars: number): string {
    if (!text) return '';
    return text.length > maxChars ? text.substring(0, maxChars - 2) + '..' : text;
}

function normalizeEdgeLabel(text: string | null | undefined): string {
    const trimmed = String(text || '').trim();
    if (!trimmed) return '';
    const namespaceStripped = trimmed.replace(/.*::/, '');
    return namespaceStripped.length > 24 ? `${namespaceStripped.substring(0, 21)}...` : namespaceStripped;
}

function nodeKind(action: ActivityActionNode): string {
    return String(action.kind || action.type || 'action').toLowerCase();
}

function isInitial(action: ActivityActionNode): boolean {
    const kind = nodeKind(action);
    return kind.includes('initial') || kind.includes('start') || action.name === 'start';
}

function isFinal(action: ActivityActionNode): boolean {
    const kind = nodeKind(action);
    return kind.includes('final') || kind.includes('done') || kind.includes('end') || action.name === 'done';
}

function isDecision(action: ActivityActionNode): boolean {
    const kind = nodeKind(action);
    return kind.includes('decision') || kind.includes('merge');
}

function isFork(action: ActivityActionNode): boolean {
    const kind = nodeKind(action);
    return kind.includes('fork') || kind.includes('join');
}

function visualKind(action: ActivityActionNode): 'perform' | 'regular' {
    const kind = nodeKind(action);
    if (kind.includes('perform')) return 'perform';
    return 'regular';
}

function actionPalette(action: ActivityActionNode): { fill: string; border: string; accent: string; chip: string } {
    const visual = visualKind(action);
    if (visual === 'perform') {
        return {
            fill: 'rgba(255, 184, 77, 0.12)',
            border: '#FFB84D',
            accent: '#FFD08B',
            chip: 'PERFORM',
        };
    }
    return {
        fill: 'var(--vscode-editor-background)',
        border: DIAGRAM_STYLE.nodeBorder,
        accent: 'var(--vscode-descriptionForeground)',
        chip: '',
    };
}

function nodeSize(action: ActivityActionNode): { width: number; height: number } {
    if (isInitial(action) || isFinal(action)) return { width: TERMINAL_SIZE, height: TERMINAL_SIZE };
    if (isDecision(action)) return { width: DECISION_SIZE, height: DECISION_SIZE };
    if (isFork(action)) return { width: FORK_WIDTH, height: FORK_HEIGHT };
    return { width: ACTION_WIDTH, height: ACTION_HEIGHT };
}

function pathFromSections(sections: ElkSection[] | undefined): string | null {
    if (!sections?.length) return null;
    const parts: string[] = [];
    sections.forEach((section) => {
        if (!section.startPoint || !section.endPoint) return;
        parts.push(`M${section.startPoint.x},${section.startPoint.y}`);
        (section.bendPoints || []).forEach((point) => parts.push(`L${point.x},${point.y}`));
        parts.push(`L${section.endPoint.x},${section.endPoint.y}`);
    });
    return parts.length ? parts.join(' ') : null;
}

function edgeLabelPosition(sections: ElkSection[] | undefined): { x: number; y: number } | null {
    if (!sections?.length) return null;
    const points: Array<{ x: number; y: number }> = [];
    sections.forEach((section) => {
        if (section.startPoint) points.push(section.startPoint);
        (section.bendPoints || []).forEach((point) => points.push(point));
        if (section.endPoint) points.push(section.endPoint);
    });
    return points.length ? points[Math.floor(points.length / 2)] : null;
}

function fallbackEdgePath(
    source: { x: number; y: number; width: number; height: number },
    target: { x: number; y: number; width: number; height: number },
    isHorizontal: boolean,
): { path: string; x: number; y: number } {
    if (isHorizontal) {
        const startX = source.x + source.width;
        const startY = source.y + source.height / 2;
        const endX = target.x;
        const endY = target.y + target.height / 2;
        const midX = (startX + endX) / 2;
        return {
            path: `M${startX},${startY} L${midX},${startY} L${midX},${endY} L${endX},${endY}`,
            x: midX,
            y: (startY + endY) / 2 - 6,
        };
    }
    const startX = source.x + source.width / 2;
    const startY = source.y + source.height;
    const endX = target.x + target.width / 2;
    const endY = target.y;
    const midY = (startY + endY) / 2;
    return {
        path: `M${startX},${startY} L${startX},${midY} L${endX},${midY} L${endX},${endY}`,
        x: (startX + endX) / 2,
        y: midY - 6,
    };
}

function renderActionNode(
    group: any,
    action: ActivityActionNode,
    layout: { x: number; y: number; width: number; height: number },
    postMessage: (msg: unknown) => void,
    onStartInlineEdit: (nodeG: any, elementName: string, x: number, y: number, width: number) => void,
    clearVisualHighlights: () => void,
    parentContext: string,
): void {
    const kind = nodeKind(action);
    const palette = actionPalette(action);
    const nodeGroup = group.append('g')
        .attr('class', `activity-action elk-node ${kind.replace(/\s+/g, '-')}`)
        .attr('data-element-name', action.name)
        .attr('transform', `translate(${layout.x},${layout.y})`)
        .style('cursor', 'pointer');

    const handleClick = function(event: any) {
        event.stopPropagation();
        clearVisualHighlights();
        const selected = d3.select(this);
        selected.classed('highlighted-element', true);
        selected.select('.node-background')
            .style('stroke', DIAGRAM_STYLE.highlight)
            .style('stroke-width', '3px');
        postJumpToElement(postMessage, { name: action.name, id: action.id }, { parentContext, skipCentering: true });
    };

    if (isInitial(action) || isFinal(action)) {
        nodeGroup.append('circle')
            .attr('class', 'node-background')
            .attr('data-original-stroke', DIAGRAM_STYLE.nodeBorder)
            .attr('data-original-width', '2px')
            .attr('cx', layout.width / 2)
            .attr('cy', layout.height / 2)
            .attr('r', TERMINAL_SIZE / 2 - 2)
            .style('fill', isInitial(action) ? DIAGRAM_STYLE.edgePrimary : 'var(--vscode-editor-background)')
            .style('stroke', DIAGRAM_STYLE.nodeBorder)
            .style('stroke-width', '2px');
        if (isFinal(action)) {
            nodeGroup.append('circle')
                .attr('cx', layout.width / 2)
                .attr('cy', layout.height / 2)
                .attr('r', 10)
                .style('fill', DIAGRAM_STYLE.edgePrimary)
                .style('stroke', 'none');
        }
    } else if (isDecision(action)) {
        const cx = layout.width / 2;
        const cy = layout.height / 2;
        nodeGroup.append('path')
            .attr('class', 'node-background')
            .attr('data-original-stroke', DIAGRAM_STYLE.edgePrimary)
            .attr('data-original-width', '2px')
            .attr('d', `M${cx},0 L${layout.width},${cy} L${cx},${layout.height} L0,${cy} Z`)
            .style('fill', 'var(--vscode-editor-background)')
            .style('stroke', DIAGRAM_STYLE.edgePrimary)
            .style('stroke-width', '2px');
    } else if (isFork(action)) {
        nodeGroup.append('rect')
            .attr('class', 'node-background')
            .attr('data-original-stroke', 'none')
            .attr('data-original-width', '0px')
            .attr('x', 0)
            .attr('y', 0)
            .attr('width', layout.width)
            .attr('height', layout.height)
            .attr('rx', 3)
            .style('fill', 'var(--vscode-panel-border)')
            .style('stroke', 'none');
    } else {
        nodeGroup.append('rect')
            .attr('class', 'node-background')
            .attr('data-original-stroke', palette.border)
            .attr('data-original-width', '2px')
            .attr('width', layout.width)
            .attr('height', layout.height)
            .attr('rx', 8)
            .attr('ry', 8)
            .style('fill', palette.fill)
            .style('stroke', palette.border)
            .style('stroke-width', '2px');

        nodeGroup.append('rect')
            .attr('x', 0)
            .attr('y', 0)
            .attr('width', layout.width)
            .attr('height', 6)
            .attr('rx', 8)
            .attr('ry', 8)
            .style('fill', palette.border)
            .style('stroke', 'none');
    }

    const label = truncateToFit(action.name, isDecision(action) ? 18 : 24);
    const labelY = isFork(action)
        ? layout.height + 16
        : isInitial(action) || isFinal(action)
            ? layout.height + 18
            : layout.height / 2 + 2;
    nodeGroup.append('text')
        .attr('class', 'node-name-text')
        .attr('x', layout.width / 2)
        .attr('y', labelY)
        .attr('text-anchor', 'middle')
        .text(label)
        .style('font-size', isDecision(action) ? '11px' : '12px')
        .style('font-weight', isDecision(action) || isFork(action) ? '600' : '700')
        .style('fill', 'var(--vscode-editor-foreground)')
        .style('pointer-events', 'none');

    if (!isInitial(action) && !isFinal(action) && !isDecision(action) && !isFork(action) && kind !== 'action') {
        nodeGroup.append('text')
            .attr('x', layout.width / 2)
            .attr('y', layout.height / 2 + 18)
            .attr('text-anchor', 'middle')
            .text(`«${truncateToFit(kind, 16)}»`)
            .style('font-size', '9px')
            .style('fill', 'var(--vscode-descriptionForeground)')
            .style('pointer-events', 'none');
    }

    if (!isInitial(action) && !isFinal(action) && !isDecision(action) && !isFork(action) && palette.chip) {
        const chipWidth = palette.chip.length * 6 + 14;
        nodeGroup.append('rect')
            .attr('x', layout.width / 2 - chipWidth / 2)
            .attr('y', layout.height - 22)
            .attr('width', chipWidth)
            .attr('height', 14)
            .attr('rx', 7)
            .style('fill', 'var(--vscode-editor-background)')
            .style('stroke', palette.border)
            .style('stroke-width', '1px');
        nodeGroup.append('text')
            .attr('x', layout.width / 2)
            .attr('y', layout.height - 12)
            .attr('text-anchor', 'middle')
            .text(palette.chip)
            .style('font-size', '8px')
            .style('font-weight', '700')
            .style('letter-spacing', '0.5px')
            .style('fill', palette.accent)
            .style('pointer-events', 'none');
    }

    if (!isInitial(action) && !isFinal(action) && !isDecision(action) && !isFork(action)) {
        const ioBadges = [
            ...(Array.isArray(action.inputs) && action.inputs.length ? [`in: ${truncateToFit(action.inputs.join(', '), 18)}`] : []),
            ...(Array.isArray(action.outputs) && action.outputs.length ? [`out: ${truncateToFit(action.outputs.join(', '), 18)}`] : []),
        ];
        ioBadges.forEach((badge, idx) => {
            const badgeWidth = badge.length * 5.8 + 14;
            const badgeY = layout.height - 22 - (palette.chip ? 18 : 0) - (idx * 17);
            nodeGroup.append('rect')
                .attr('x', layout.width / 2 - badgeWidth / 2)
                .attr('y', badgeY)
                .attr('width', badgeWidth)
                .attr('height', 14)
                .attr('rx', 7)
                .style('fill', 'var(--vscode-editor-background)')
                .style('stroke', DIAGRAM_STYLE.nodeBorder)
                .style('stroke-width', '1px');
            nodeGroup.append('text')
                .attr('x', layout.width / 2)
                .attr('y', badgeY + 10)
                .attr('text-anchor', 'middle')
                .text(badge)
                .style('font-size', '8px')
                .style('font-weight', '600')
                .style('fill', 'var(--vscode-descriptionForeground)')
                .style('pointer-events', 'none');
        });
    }

    nodeGroup.on('click', handleClick)
        .on('dblclick', function(event: any) {
            event.stopPropagation();
            onStartInlineEdit(d3.select(this), action.name, layout.x, layout.y, layout.width);
        });
}

async function layoutActivityDiagram(
    ctx: ActivityRenderContext,
    diagram: any,
): Promise<{
    positions: Map<string, { x: number; y: number; width: number; height: number }>;
    edgeSectionsById: Map<string, ElkSection[]>;
}> {
    if (typeof ELK === 'undefined') throw new Error('ELK layout library not loaded');
    const elk = new ELK({ workerUrl: ctx.elkWorkerUrl || undefined });
    const isHorizontal = ctx.activityLayoutDirection === 'horizontal';
    const actions: ActivityActionNode[] = (diagram.actions || []).map((action: any, idx: number) => ({
        ...action,
        id: action.id || action.name || `action_${idx + 1}`,
        name: action.name || action.id || `Action ${idx + 1}`,
    }));
    const flows: ActivityFlow[] = (diagram.flows || []).map((flow: any, idx: number) => ({
        ...flow,
        id: flow.id || `${diagram.name}::flow::${idx + 1}`,
    }));

    const elkGraph = {
        id: diagram.name || 'activity-diagram',
        layoutOptions: {
            'elk.algorithm': 'layered',
            'elk.direction': isHorizontal ? 'RIGHT' : 'DOWN',
            'elk.edgeRouting': 'ORTHOGONAL',
            'elk.layered.nodePlacement.strategy': 'NETWORK_SIMPLEX',
            'elk.layered.crossingMinimization.strategy': 'LAYER_SWEEP',
            'elk.spacing.nodeNode': isHorizontal ? '90' : '120',
            'elk.layered.spacing.nodeNodeBetweenLayers': isHorizontal ? '190' : '170',
            'elk.spacing.edgeNode': '80',
            'elk.spacing.edgeEdge': '60',
            'elk.padding': '[top=80,left=80,bottom=80,right=80]',
            'elk.separateConnectedComponents': 'true',
            'elk.json.edgeCoords': 'ROOT',
        },
        children: actions.map((action) => {
            const size = nodeSize(action);
            return { id: action.id, width: size.width, height: size.height };
        }),
        edges: flows.map((flow) => ({
            id: flow.id,
            sources: [flow.from],
            targets: [flow.to],
        })),
    };

    const laidOut = await elk.layout(elkGraph);
    const positions = new Map<string, { x: number; y: number; width: number; height: number }>();
    (laidOut?.children || []).forEach((child: any) => {
        const action = actions.find((candidate) => candidate.id === child.id);
        const fallbackSize = action ? nodeSize(action) : { width: ACTION_WIDTH, height: ACTION_HEIGHT };
        positions.set(String(child.id), {
            x: child.x ?? 0,
            y: child.y ?? 0,
            width: child.width ?? fallbackSize.width,
            height: child.height ?? fallbackSize.height,
        });
    });

    const edgeSectionsById = new Map<string, ElkSection[]>();
    (laidOut?.edges || []).forEach((edge: any) => {
        if (edge?.id && Array.isArray(edge.sections)) {
            edgeSectionsById.set(String(edge.id), edge.sections as ElkSection[]);
        }
    });

    return { positions, edgeSectionsById };
}

export async function renderActivityView(ctx: ActivityRenderContext, data: any): Promise<void> {
    const {
        width,
        height,
        svg,
        g,
        selectedDiagramIndex,
        postMessage,
        onStartInlineEdit,
        renderPlaceholder,
        clearVisualHighlights,
    } = ctx;

    if (!data?.diagrams?.length) {
        renderPlaceholder(
            width,
            height,
            'Action Flow View',
            'No activity diagrams found to display.\n\nThis view shows action flows with decisions and control flows.',
            data,
        );
        return;
    }

    const diagram = data.diagrams[Math.min(selectedDiagramIndex, data.diagrams.length - 1)];
    if (!diagram?.nodes?.length) {
        renderPlaceholder(width, height, 'Action Flow View', 'No behavioral action nodes found in the selected activity diagram.', data);
        return;
    }
    if (!diagram?.flows?.length) {
        const interfaceSummary = [
            Array.isArray(diagram.interface?.inputs) && diagram.interface.inputs.length
                ? `Inputs: ${diagram.interface.inputs.join(', ')}`
                : '',
            Array.isArray(diagram.interface?.outputs) && diagram.interface.outputs.length
                ? `Outputs: ${diagram.interface.outputs.join(', ')}`
                : '',
        ].filter(Boolean).join('\n');
        renderPlaceholder(
            width,
            height,
            'Action Flow View',
            `No explicit behavioral flows found for "${diagram.name}".\n\nThis view no longer invents pseudo-flows from parameter order.${interfaceSummary ? `\n\n${interfaceSummary}` : ''}`,
            data,
        );
        return;
    }

    const actions: ActivityActionNode[] = diagram.nodes.map((action: any, idx: number) => ({
        ...action,
        id: action.id || action.name || `action_${idx + 1}`,
        name: action.name || action.id || `Action ${idx + 1}`,
    }));
    const flows: ActivityFlow[] = (diagram.flows || []).map((flow: any, idx: number) => ({
        ...flow,
        id: flow.id || `${diagram.name}::flow::${idx + 1}`,
    }));

    let layout;
    try {
        layout = await layoutActivityDiagram(ctx, { ...diagram, actions, flows });
    } catch (error) {
        console.error('[Action Flow View] ELK layout failed:', error);
        renderPlaceholder(width, height, 'Action Flow View', 'ELK layout failed for this activity diagram.', data);
        return;
    }

    const defs = svg.select('defs').empty() ? svg.append('defs') : svg.select('defs');
    defs.selectAll('#activity-arrowhead').remove();
    defs.append('marker')
        .attr('id', 'activity-arrowhead')
        .attr('viewBox', '0 -5 10 10')
        .attr('refX', 8)
        .attr('refY', 0)
        .attr('markerWidth', 6)
        .attr('markerHeight', 6)
        .attr('orient', 'auto')
        .append('path')
        .attr('d', 'M0,-5L10,0L0,5')
        .style('fill', DIAGRAM_STYLE.edgePrimary);

    g.append('text')
        .attr('x', 36)
        .attr('y', 32)
        .text(`Action Flow: ${diagram.name}`)
        .style('font-size', '16px')
        .style('font-weight', '700')
        .style('fill', 'var(--vscode-editor-foreground)');

    const interfaceParts = [
        Array.isArray(diagram.interface?.inputs) && diagram.interface.inputs.length
            ? `Inputs: ${diagram.interface.inputs.join(', ')}`
            : '',
        Array.isArray(diagram.interface?.outputs) && diagram.interface.outputs.length
            ? `Outputs: ${diagram.interface.outputs.join(', ')}`
            : '',
    ].filter(Boolean);
    if (interfaceParts.length > 0) {
        g.append('text')
            .attr('x', 36)
            .attr('y', 52)
            .text(interfaceParts.join('  |  '))
            .style('font-size', '11px')
            .style('fill', 'var(--vscode-descriptionForeground)');
    }

    const flowGroup = g.append('g').attr('class', 'activity-flows');
    const actionGroup = g.append('g').attr('class', 'activity-actions');
    const isHorizontal = ctx.activityLayoutDirection === 'horizontal';

    flows.forEach((flow) => {
        const source = layout.positions.get(flow.from);
        const target = layout.positions.get(flow.to);
        if (!source || !target) return;

        const sections = layout.edgeSectionsById.get(flow.id || '');
        const fallback = fallbackEdgePath(source, target, isHorizontal);
        flowGroup.append('path')
            .attr('class', 'activity-flow')
            .attr('d', pathFromSections(sections) || fallback.path)
            .style('fill', 'none')
            .style('stroke', DIAGRAM_STYLE.edgePrimary)
            .style('stroke-width', '2px')
            .style('marker-end', 'url(#activity-arrowhead)');

        const guardLabel = String(flow.guard || flow.condition || '').trim();
        if (guardLabel) {
            const position = edgeLabelPosition(sections) || { x: fallback.x, y: fallback.y };
            const displayLabel = `[${normalizeEdgeLabel(guardLabel)}]`;
            const labelWidth = Math.max(38, displayLabel.length * 6 + 8);
            flowGroup.append('rect')
                .attr('x', position.x - labelWidth / 2)
                .attr('y', position.y - 10)
                .attr('width', labelWidth)
                .attr('height', 16)
                .attr('rx', 3)
                .style('fill', 'var(--vscode-editor-background)')
                .style('stroke', DIAGRAM_STYLE.edgePrimary)
                .style('stroke-width', '1px');
            flowGroup.append('text')
                .attr('x', position.x)
                .attr('y', position.y + 2)
                .attr('text-anchor', 'middle')
                .text(displayLabel)
                .style('font-size', '10px')
                .style('font-weight', '700')
                .style('fill', DIAGRAM_STYLE.edgePrimary);
        }
    });

    actions.forEach((action) => {
        const position = layout.positions.get(action.id);
        if (!position) return;
        renderActionNode(
            actionGroup,
            action,
            position,
            postMessage,
            onStartInlineEdit,
            clearVisualHighlights,
            diagram.name,
        );
    });
}

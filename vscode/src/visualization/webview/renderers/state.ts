/**
 * State Transition View renderer - ELK-backed state-machine layout with D3 rendering.
 */
/* eslint-disable @typescript-eslint/no-explicit-any */

import type { RenderContext } from '../types';
import { postJumpToElement } from '../jumpToElement';
import { DIAGRAM_STYLE } from '../styleTokens';
import { estimateElkLabelBox, toAbsoluteElkLabelBox, type ElkLabelBox } from './elkLabelUtils';

declare const d3: any;
declare const ELK: any;

type StateNode = {
    id: string;
    name: string;
    kind: 'initial' | 'final' | 'state' | 'composite';
    parentId: string | null;
    childIds: string[];
    depth?: number;
};

type StateTransition = {
    id: string;
    name?: string;
    source: string;
    target: string;
    label?: string;
    selfLoop?: boolean;
};

type StateMachineScene = {
    id: string;
    name: string;
    states: StateNode[];
    transitions: StateTransition[];
};

type ElkSection = {
    startPoint?: { x: number; y: number };
    endPoint?: { x: number; y: number };
    bendPoints?: Array<{ x: number; y: number }>;
};

type StateRenderContext = RenderContext & { elkWorkerUrl?: string };

const STATE_WIDTH = 240;
const STATE_HEIGHT = 180;
const PSEUDO_SIZE = 34;
const COMPOSITE_MIN_WIDTH = 340;
const COMPOSITE_MIN_HEIGHT = 320;
const COMPOSITE_HEADER_HEIGHT = 34;

function sanitizeId(value: string): string {
    return String(value || '').replace(/[^A-Za-z0-9_.-]/g, '_');
}

function truncateLabel(value: string | null | undefined, maxLength: number): string {
    const text = String(value || '');
    return text.length > maxLength ? text.substring(0, maxLength - 2) + '..' : text;
}

function pathFromSections(sections: ElkSection[] | undefined): string | null {
    if (!sections || sections.length === 0) return null;
    const parts: string[] = [];
    sections.forEach((section) => {
        if (!section.startPoint || !section.endPoint) return;
        parts.push(`M${section.startPoint.x},${section.startPoint.y}`);
        (section.bendPoints || []).forEach((point) => {
            parts.push(`L${point.x},${point.y}`);
        });
        parts.push(`L${section.endPoint.x},${section.endPoint.y}`);
    });
    return parts.length > 0 ? parts.join(' ') : null;
}

function edgeLabelPositionFromSections(sections: ElkSection[] | undefined): { x: number; y: number } | null {
    if (!sections || sections.length === 0) return null;
    const points: Array<{ x: number; y: number }> = [];
    sections.forEach((section) => {
        if (section.startPoint) points.push(section.startPoint);
        (section.bendPoints || []).forEach((point) => points.push(point));
        if (section.endPoint) points.push(section.endPoint);
    });
    if (points.length === 0) return null;
    return points[Math.floor(points.length / 2)];
}

function transitionDisplayLabel(transition: StateTransition): string {
    return truncateLabel(String(transition.label || transition.name || '').trim(), 28);
}

function buildSelfLoopPath(node: { x: number; y: number; width: number; height: number }): { path: string; labelX: number; labelY: number } {
    const startX = node.x + node.width;
    const startY = node.y + node.height / 2 - 8;
    const loopRadius = 28;
    return {
        path:
            `M${startX},${startY}` +
            ` C${startX + loopRadius},${startY - loopRadius}` +
            ` ${startX + loopRadius},${startY + loopRadius}` +
            ` ${startX},${startY + 18}`,
        labelX: startX + loopRadius + 8,
        labelY: startY,
    };
}

function buildFallbackEdgePath(
    source: { x: number; y: number; width: number; height: number },
    target: { x: number; y: number; width: number; height: number },
): { path: string; labelX: number; labelY: number } {
    const sourceX = source.x + source.width;
    const sourceY = source.y + source.height / 2;
    const targetX = target.x;
    const targetY = target.y + target.height / 2;
    const midX = (sourceX + targetX) / 2;
    return {
        path: `M${sourceX},${sourceY} L${midX},${sourceY} L${midX},${targetY} L${targetX},${targetY}`,
        labelX: midX,
        labelY: (sourceY + targetY) / 2 - 8,
    };
}

function nodeSizeFor(
    state: StateNode,
    metrics?: { degreeByStateId?: Map<string, number>; maxLabelLength?: number; maxDegree?: number; transitionCount?: number },
): { width: number; height: number } {
    if (state.kind === 'initial' || state.kind === 'final') {
        return { width: PSEUDO_SIZE, height: PSEUDO_SIZE };
    }
    if (state.kind === 'composite') {
        const degree = metrics?.degreeByStateId?.get(state.id) || 0;
        const widthBoost = Math.min(80, degree * 8);
        const heightBoost = Math.min(180, degree * 18);
        return { width: COMPOSITE_MIN_WIDTH + widthBoost, height: COMPOSITE_MIN_HEIGHT + heightBoost };
    }
    const degree = metrics?.degreeByStateId?.get(state.id) || 0;
    const denseMachineHeightBoost = Math.min(120, Math.max(0, (metrics?.transitionCount || 0) - 10) * 2.4);
    const labelBoost = Math.min(45, Math.max(0, (metrics?.maxLabelLength || 0) - 14) * 1.5);
    const degreeWidthBoost = Math.min(70, Math.max(0, degree - 2) * 7);
    const degreeHeightBoost = Math.min(190, Math.max(0, degree - 2) * 18);
    const maxDegreeHeightBoost = Math.min(90, Math.max(0, (metrics?.maxDegree || 0) - 6) * 8);
    return {
        width: STATE_WIDTH + labelBoost + degreeWidthBoost,
        height: STATE_HEIGHT + denseMachineHeightBoost + degreeHeightBoost + maxDegreeHeightBoost,
    };
}

function stateKindClass(kind: string): string {
    if (kind === 'initial') return 'state-initial';
    if (kind === 'final') return 'state-final';
    if (kind === 'composite') return 'state-composite';
    return 'state-regular';
}

function machineDirection(
    layoutOrientation: string,
    machine?: StateMachineScene,
): string {
    if (layoutOrientation === 'vertical') return 'DOWN';
    if (layoutOrientation === 'horizontal') return 'RIGHT';
    const stateCount = machine?.states.filter((state) => state.kind !== 'initial' && state.kind !== 'final').length || 0;
    const transitionCount = machine?.transitions.length || 0;
    const density = stateCount > 0 ? transitionCount / stateCount : 0;
    return density >= 2 ? 'DOWN' : 'RIGHT';
}

function renderStateNode(
    group: any,
    state: StateNode,
    layout: { x: number; y: number; width: number; height: number },
    defs: any,
    postMessage: (msg: unknown) => void,
    onStartInlineEdit: (nodeG: any, elementName: string, x: number, y: number, width: number) => void,
    clearVisualHighlights: () => void,
): void {
    const nodeGroup = group.append('g')
        .attr('class', `state-node elk-node ${stateKindClass(state.kind)}`)
        .attr('data-element-name', state.name)
        .attr('data-state-id', state.id)
        .attr('transform', `translate(${layout.x}, ${layout.y})`)
        .style('cursor', 'pointer');

    const gradientId = `state-gradient-${sanitizeId(state.id)}`;

    if (state.kind === 'initial') {
        nodeGroup.append('circle')
            .attr('class', 'graph-node-background node-background')
            .attr('cx', layout.width / 2)
            .attr('cy', layout.height / 2)
            .attr('r', 14)
            .attr('data-original-stroke', DIAGRAM_STYLE.nodeBorder)
            .attr('data-original-width', '2px')
            .style('fill', DIAGRAM_STYLE.edgePrimary)
            .style('stroke', DIAGRAM_STYLE.nodeBorder)
            .style('stroke-width', '2px');
    } else if (state.kind === 'final') {
        nodeGroup.append('circle')
            .attr('class', 'graph-node-background node-background')
            .attr('cx', layout.width / 2)
            .attr('cy', layout.height / 2)
            .attr('r', 15)
            .attr('data-original-stroke', DIAGRAM_STYLE.nodeBorder)
            .attr('data-original-width', '2px')
            .style('fill', 'var(--vscode-editor-background)')
            .style('stroke', DIAGRAM_STYLE.nodeBorder)
            .style('stroke-width', '2px');
        nodeGroup.append('circle')
            .attr('cx', layout.width / 2)
            .attr('cy', layout.height / 2)
            .attr('r', 9)
            .style('fill', DIAGRAM_STYLE.edgePrimary)
            .style('stroke', 'none');
    } else {
        const gradient = defs.append('linearGradient')
            .attr('id', gradientId)
            .attr('x1', '0%')
            .attr('y1', '0%')
            .attr('x2', '0%')
            .attr('y2', '100%');
        gradient.append('stop')
            .attr('offset', '0%')
            .style('stop-color', 'var(--vscode-editor-background)');
        gradient.append('stop')
            .attr('offset', '100%')
            .style('stop-color', state.kind === 'composite'
                ? 'var(--vscode-editorWidget-background)'
                : 'var(--vscode-sideBar-background)');

        nodeGroup.append('rect')
            .attr('class', 'graph-node-background node-background')
            .attr('width', layout.width)
            .attr('height', layout.height)
            .attr('rx', state.kind === 'composite' ? 10 : 8)
            .attr('ry', state.kind === 'composite' ? 10 : 8)
            .attr('data-original-stroke', DIAGRAM_STYLE.nodeBorder)
            .attr('data-original-width', state.kind === 'composite' ? '2.5px' : '2px')
            .style('fill', `url(#${gradientId})`)
            .style('stroke', DIAGRAM_STYLE.nodeBorder)
            .style('stroke-width', state.kind === 'composite' ? '2.5px' : '2px');

        if (state.kind === 'composite') {
            nodeGroup.append('rect')
                .attr('width', layout.width)
                .attr('height', COMPOSITE_HEADER_HEIGHT)
                .attr('rx', 10)
                .style('fill', 'var(--vscode-button-secondaryBackground)')
                .style('stroke', 'none');
        }
    }

    const nameY = state.kind === 'composite'
        ? 22
        : (state.kind === 'state' ? layout.height / 2 + 4 : layout.height + 18);

    nodeGroup.append('text')
        .attr('class', 'node-name-text')
        .attr('x', layout.width / 2)
        .attr('y', nameY)
        .attr('text-anchor', 'middle')
        .text(truncateLabel(state.name, state.kind === 'composite' ? 26 : 20))
        .style('font-size', state.kind === 'composite' ? '12px' : '11px')
        .style('font-weight', '600')
        .style('fill', 'var(--vscode-editor-foreground)')
        .style('pointer-events', 'none');

    if (state.kind === 'state') {
        nodeGroup.append('text')
            .attr('x', layout.width / 2)
            .attr('y', 18)
            .attr('text-anchor', 'middle')
            .text('«state»')
            .style('font-size', '9px')
            .style('fill', 'var(--vscode-descriptionForeground)')
            .style('pointer-events', 'none');
    }

    nodeGroup.on('click', function(event: any) {
        event.stopPropagation();
        clearVisualHighlights();
        const selected = d3.select(this);
        selected.classed('highlighted-element', true);
        selected.select('.graph-node-background')
            .style('stroke', DIAGRAM_STYLE.highlight)
            .style('stroke-width', '3px');
        postJumpToElement(postMessage, { name: state.name, id: state.id }, { skipCentering: true });
    }).on('dblclick', function(event: any) {
        event.stopPropagation();
        onStartInlineEdit(d3.select(this), state.name, layout.x, layout.y, layout.width);
    });
}

function extractStateLayouts(
    elkNode: any,
    machine: StateMachineScene,
    offset: { x: number; y: number },
    layouts: Map<string, { x: number; y: number; width: number; height: number; depth: number }>,
    depth: number,
): void {
    const absoluteX = offset.x + (elkNode?.x ?? 0);
    const absoluteY = offset.y + (elkNode?.y ?? 0);
    const state = machine.states.find((candidate) => candidate.id === elkNode?.id);
    if (state) {
        layouts.set(state.id, {
            x: absoluteX,
            y: absoluteY,
            width: elkNode?.width ?? nodeSizeFor(state).width,
            height: elkNode?.height ?? nodeSizeFor(state).height,
            depth,
        });
    }
    (elkNode?.children || []).forEach((child: any) => {
        extractStateLayouts(child, machine, { x: absoluteX, y: absoluteY }, layouts, depth + 1);
    });
}

function collectEdgeSections(
    elkNode: any,
    offset: { x: number; y: number },
    acc: Map<string, ElkSection[]>,
): void {
    (elkNode?.edges || []).forEach((edge: any) => {
        if (!edge?.id || !Array.isArray(edge.sections)) return;
        acc.set(String(edge.id), edge.sections.map((section: ElkSection) => ({
            startPoint: section.startPoint
                ? { x: section.startPoint.x + offset.x, y: section.startPoint.y + offset.y }
                : undefined,
            endPoint: section.endPoint
                ? { x: section.endPoint.x + offset.x, y: section.endPoint.y + offset.y }
                : undefined,
            bendPoints: Array.isArray(section.bendPoints)
                ? section.bendPoints.map((point) => ({ x: point.x + offset.x, y: point.y + offset.y }))
                : undefined,
        })));
    });
    (elkNode?.children || []).forEach((child: any) => {
        collectEdgeSections(child, { x: offset.x + (child?.x ?? 0), y: offset.y + (child?.y ?? 0) }, acc);
    });
}

function collectEdgeLabels(
    elkNode: any,
    offset: { x: number; y: number },
    acc: Map<string, ElkLabelBox[]>,
): void {
    (elkNode?.edges || []).forEach((edge: any) => {
        if (!edge?.id || !Array.isArray(edge.labels) || edge.labels.length === 0) return;
        const labels = edge.labels
            .map((label: any) => toAbsoluteElkLabelBox(label, offset))
            .filter((label: ElkLabelBox | null): label is ElkLabelBox => Boolean(label));
        if (labels.length > 0) {
            acc.set(String(edge.id), labels);
        }
    });
    (elkNode?.children || []).forEach((child: any) => {
        collectEdgeLabels(child, { x: offset.x + (child?.x ?? 0), y: offset.y + (child?.y ?? 0) }, acc);
    });
}

function buildElkChild(
    state: StateNode,
    childrenByParent: Map<string | null, StateNode[]>,
    direction: string,
    metrics: { degreeByStateId: Map<string, number>; maxLabelLength: number },
): any {
    const size = nodeSizeFor(state, metrics);
    const childStates = childrenByParent.get(state.id) || [];
    const elkNode: any = {
        id: state.id,
        width: size.width,
        height: size.height,
    };

    if (state.kind === 'composite') {
        elkNode.layoutOptions = {
            'elk.algorithm': 'layered',
            'elk.direction': direction,
            'elk.padding': `[top=${COMPOSITE_HEADER_HEIGHT + 16},left=20,bottom=20,right=20]`,
            'elk.edgeRouting': 'ORTHOGONAL',
            'elk.hierarchyHandling': 'INCLUDE_CHILDREN',
            'elk.layered.spacing.nodeNodeBetweenLayers': '110',
            'elk.spacing.nodeNode': '90',
        };
        elkNode.children = childStates.map((child) => buildElkChild(child, childrenByParent, direction, metrics));
    }

    return elkNode;
}

async function layoutStateMachine(
    ctx: StateRenderContext,
    machine: StateMachineScene,
): Promise<{
    layouts: Map<string, { x: number; y: number; width: number; height: number; depth: number }>;
    edgeSectionsById: Map<string, ElkSection[]>;
    edgeLabelsById: Map<string, ElkLabelBox[]>;
}> {
    const direction = machineDirection(ctx.stateLayoutOrientation, machine);
    const childrenByParent = new Map<string | null, StateNode[]>();
    machine.states.forEach((state) => {
        const key = state.parentId || null;
        if (!childrenByParent.has(key)) {
            childrenByParent.set(key, []);
        }
        childrenByParent.get(key)!.push(state);
    });

    const topLevelStates = childrenByParent.get(null) || [];
    const degreeByStateId = new Map<string, number>();
    machine.states.forEach((state) => degreeByStateId.set(state.id, 0));
    machine.transitions.forEach((transition) => {
        degreeByStateId.set(transition.source, (degreeByStateId.get(transition.source) || 0) + 1);
        degreeByStateId.set(transition.target, (degreeByStateId.get(transition.target) || 0) + 1);
    });
    const maxLabelLength = Math.max(
        0,
        ...machine.transitions.map((transition) => String(transition.label || transition.name || '').length),
    );
    const maxDegree = Math.max(0, ...Array.from(degreeByStateId.values()));
    const metrics = { degreeByStateId, maxLabelLength, maxDegree, transitionCount: machine.transitions.length };
    if (typeof ELK === 'undefined') {
        throw new Error('ELK layout library not loaded');
    }

    const elk = new ELK({ workerUrl: ctx.elkWorkerUrl || undefined });
    const elkGraph = {
        id: machine.id,
        layoutOptions: {
            'elk.algorithm': 'layered',
            'elk.direction': direction,
            'elk.hierarchyHandling': 'INCLUDE_CHILDREN',
            'elk.edgeRouting': 'ORTHOGONAL',
            'elk.layered.nodePlacement.strategy': 'NETWORK_SIMPLEX',
            'elk.layered.crossingMinimization.strategy': 'LAYER_SWEEP',
            'elk.layered.spacing.nodeNodeBetweenLayers': '230',
            'elk.spacing.nodeNode': '190',
            'elk.spacing.edgeNode': '130',
            'elk.spacing.edgeEdge': '110',
            'elk.spacing.edgeLabel': '12',
            'elk.padding': '[top=100,left=90,bottom=90,right=90]',
            'elk.separateConnectedComponents': 'true',
            'elk.json.edgeCoords': 'ROOT',
        },
        children: topLevelStates.map((state) => buildElkChild(state, childrenByParent, direction, metrics)),
        edges: machine.transitions.map((transition) => {
            const displayLabel = transitionDisplayLabel(transition);
            const labelBox = displayLabel
                ? estimateElkLabelBox(`${transition.id}::label`, displayLabel, {
                    minWidth: 42,
                    minHeight: 18,
                    paddingX: 10,
                    paddingY: 8,
                    charWidth: 6,
                })
                : null;
            return {
                id: transition.id,
                sources: [transition.source],
                targets: [transition.target],
                labels: labelBox ? [{
                    id: labelBox.id,
                    text: labelBox.text,
                    width: labelBox.width,
                    height: labelBox.height,
                    layoutOptions: {
                        'org.eclipse.elk.edgeLabels.placement': 'CENTER',
                        'org.eclipse.elk.edgeLabels.inline': 'false',
                    },
                }] : [],
            };
        }),
    };

    const laidOut = await elk.layout(elkGraph);
    const layouts = new Map<string, { x: number; y: number; width: number; height: number; depth: number }>();
    (laidOut?.children || []).forEach((child: any) => {
        extractStateLayouts(child, machine, { x: 0, y: 0 }, layouts, 0);
    });
    const edgeSectionsById = new Map<string, ElkSection[]>();
    const edgeLabelsById = new Map<string, ElkLabelBox[]>();
    collectEdgeSections(laidOut, { x: 0, y: 0 }, edgeSectionsById);
    collectEdgeLabels(laidOut, { x: 0, y: 0 }, edgeLabelsById);
    return { layouts, edgeSectionsById, edgeLabelsById };
}

export async function renderStateView(ctx: StateRenderContext, data: any): Promise<void> {
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

    const stateMachines: StateMachineScene[] = Array.isArray(data?.stateMachines) ? data.stateMachines : [];
    if (stateMachines.length === 0) {
        renderPlaceholder(
            width,
            height,
            'State Transition View',
            'No state machines found to display.\n\nThis view shows states, transitions, and composite states.',
            data,
        );
        return;
    }

    const machineIndex = Math.min(selectedDiagramIndex, stateMachines.length - 1);
    const machine = stateMachines[machineIndex];
    if (!machine || machine.states.length === 0) {
        renderPlaceholder(
            width,
            height,
            'State Transition View',
            'No states found in the selected state machine.',
            data,
        );
        return;
    }

    let layoutResult;
    try {
        layoutResult = await layoutStateMachine(ctx, machine);
    } catch (error) {
        console.error('[State View] ELK layout failed:', error);
        renderPlaceholder(
            width,
            height,
            'State Transition View',
            'ELK layout failed for this state machine. Check the output panel for details.',
            data,
        );
        return;
    }

    const defs = svg.select('defs').empty() ? svg.append('defs') : svg.select('defs');
    defs.selectAll('#state-arrowhead').remove();
    defs.append('marker')
        .attr('id', 'state-arrowhead')
        .attr('viewBox', '0 -5 10 10')
        .attr('refX', 9)
        .attr('refY', 0)
        .attr('markerWidth', 7)
        .attr('markerHeight', 7)
        .attr('orient', 'auto')
        .append('path')
        .attr('d', 'M0,-4L10,0L0,4')
        .style('fill', DIAGRAM_STYLE.edgePrimary);

    g.append('text')
        .attr('x', 36)
        .attr('y', 32)
        .attr('class', 'state-machine-title')
        .text(`State Machine: ${machine.name}`)
        .style('font-size', '16px')
        .style('font-weight', '700')
        .style('fill', 'var(--vscode-editor-foreground)');

    const edgeGroup = g.append('g').attr('class', 'state-transitions');
    const nodeGroup = g.append('g').attr('class', 'state-nodes');

    machine.transitions.forEach((transition) => {
        const sourceLayout = layoutResult.layouts.get(transition.source);
        const targetLayout = layoutResult.layouts.get(transition.target);
        if (!sourceLayout || !targetLayout) return;

        const sections = layoutResult.edgeSectionsById.get(transition.id);
        const elkLabel = layoutResult.edgeLabelsById.get(transition.id)?.[0];
        const labelFromSections = edgeLabelPositionFromSections(sections);
        const path = pathFromSections(sections);
        const fallback = transition.selfLoop
            ? buildSelfLoopPath(sourceLayout)
            : buildFallbackEdgePath(sourceLayout, targetLayout);

        edgeGroup.append('path')
            .attr('class', 'state-transition')
            .attr('data-source', transition.source)
            .attr('data-target', transition.target)
            .attr('d', path || fallback.path)
            .style('fill', 'none')
            .style('stroke', DIAGRAM_STYLE.edgePrimary)
            .style('stroke-width', '2px')
            .style('marker-end', 'url(#state-arrowhead)');

        const label = transitionDisplayLabel(transition);
        if (label) {
            const labelPosition = elkLabel
                ? { x: elkLabel.x + elkLabel.width / 2, y: elkLabel.y + elkLabel.height / 2 }
                : (labelFromSections || { x: fallback.labelX, y: fallback.labelY });
            const labelWidth = elkLabel?.width ?? Math.max(42, label.length * 6 + 10);
            const labelHeight = elkLabel?.height ?? 18;

            edgeGroup.append('rect')
                .attr('x', elkLabel ? elkLabel.x : labelPosition.x - labelWidth / 2)
                .attr('y', elkLabel ? elkLabel.y : labelPosition.y - 10)
                .attr('width', labelWidth)
                .attr('height', labelHeight)
                .attr('rx', 4)
                .style('fill', 'var(--vscode-editor-background)')
                .style('stroke', DIAGRAM_STYLE.edgePrimary)
                .style('stroke-width', '1px');

            edgeGroup.append('text')
                .attr('x', labelPosition.x)
                .attr('y', labelPosition.y + 3)
                .attr('text-anchor', 'middle')
                .text(label)
                .style('font-size', '10px')
                .style('font-weight', '500')
                .style('fill', DIAGRAM_STYLE.edgePrimary);
        }
    });

    machine.states
        .slice()
        .sort((a, b) => (layoutResult.layouts.get(a.id)?.depth || 0) - (layoutResult.layouts.get(b.id)?.depth || 0))
        .forEach((state) => {
            const layout = layoutResult.layouts.get(state.id);
            if (!layout) return;
            renderStateNode(nodeGroup, state, layout, defs, postMessage, onStartInlineEdit, clearVisualHighlights);
        });

    const statusEl = document.getElementById('status-text');
    if (statusEl) {
        statusEl.textContent = `State Transition View • ${machine.name}`;
    }
}

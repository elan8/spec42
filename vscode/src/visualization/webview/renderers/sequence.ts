/**
 * Sequence/interaction renderer for the experimental Spec42 sequence-view.
 */
/* eslint-disable @typescript-eslint/no-explicit-any */

import type { RenderContext } from '../types';
import { postJumpToElement } from '../jumpToElement';

declare const d3: any;

const HEADER_Y = 64;
const LIFELINE_TOP = 118;
const LIFELINE_GAP = 220;
const MESSAGE_GAP = 78;
const LIFELINE_BOX_WIDTH = 132;
const LIFELINE_BOX_HEIGHT = 38;
const ACTIVATION_WIDTH = 16;
const FRAGMENT_PADDING = 18;
const FRAGMENT_HEADER_HEIGHT = 24;
const FRAGMENT_GUARD_PADDING = 20;
const FRAGMENT_TOP_PADDING = 54;
const FRAGMENT_BOTTOM_PADDING = 24;

function markerId(kind: string): string {
    return `sequence-arrow-${kind}`;
}

function ensureMarkers(svg: any): void {
    const defs = svg.select('defs').empty() ? svg.append('defs') : svg.select('defs');
    ['sync', 'async', 'return', 'create'].forEach((kind) => defs.select(`#${markerId(kind)}`).remove());

    ['sync', 'async', 'return', 'create'].forEach((kind) => {
        const marker = defs.append('marker')
            .attr('id', markerId(kind))
            .attr('viewBox', '0 -5 10 10')
            .attr('refX', 9)
            .attr('refY', 0)
            .attr('markerWidth', 8)
            .attr('markerHeight', 8)
            .attr('orient', 'auto');

        if (kind === 'async') {
            marker.append('path')
                .attr('d', 'M0,-5L10,0L0,5')
                .style('fill', 'var(--vscode-editor-background)')
                .style('stroke', 'var(--vscode-editor-foreground)')
                .style('stroke-width', '1.2px');
            return;
        }

        marker.append('path')
            .attr('d', 'M0,-5L10,0L0,5')
            .style('fill', 'var(--vscode-editor-foreground)');
    });
}

function resolveMessageRow(messageOrder: number): number {
    return LIFELINE_TOP + 58 + ((Math.max(1, messageOrder) - 1) * MESSAGE_GAP);
}

function fragmentColor(kind: string): string {
    switch (kind) {
        case 'alt':
            return '#C17C00';
        case 'loop':
            return '#0E7C7B';
        case 'ref':
            return '#5B8FC4';
        default:
            return '#2D8A6E';
    }
}

export async function renderSequenceView(ctx: RenderContext, data: any): Promise<void> {
    const {
        width,
        height,
        svg,
        g,
        selectedDiagramId,
        selectedDiagramIndex,
        postMessage,
        renderPlaceholder,
        clearVisualHighlights,
    } = ctx;

    if (!data?.diagrams?.length) {
        renderPlaceholder(
            width,
            height,
            'Sequence View',
            'No sequence interactions found to display.\n\nCreate a SequenceView that exposes an InteractionScenario.',
            data,
        );
        return;
    }

    const resolvedIndex = selectedDiagramId
        ? Math.max(0, data.diagrams.findIndex((candidate: any) => candidate?.id === selectedDiagramId))
        : Math.min(selectedDiagramIndex, data.diagrams.length - 1);
    const diagram = data.diagrams[Math.min(resolvedIndex, data.diagrams.length - 1)];
    if (!diagram?.lifelines?.length || !diagram?.messages?.length) {
        renderPlaceholder(
            width,
            height,
            'Sequence View',
            `No renderable sequence content found for "${diagram?.name || 'the selected interaction'}".`,
            data,
        );
        return;
    }

    ensureMarkers(svg);

    const lifelines = diagram.lifelines;
    const messages = [...diagram.messages].sort((a: any, b: any) => a.order - b.order || String(a.id).localeCompare(String(b.id)));
    const lastMessageY = messages.length ? resolveMessageRow(messages[messages.length - 1].order) : LIFELINE_TOP + 100;
    const lifelineBottom = lastMessageY + 140;
    const xOffset = Math.max(80, (width - ((Math.max(0, lifelines.length - 1) * LIFELINE_GAP) + LIFELINE_BOX_WIDTH)) / 2);

    const lifelineX = new Map<string, number>();
    lifelines.forEach((lifeline: any, index: number) => {
        lifelineX.set(lifeline.id, xOffset + (index * LIFELINE_GAP) + (LIFELINE_BOX_WIDTH / 2));
    });

    const messageYById = new Map<string, number>();
    messages.forEach((message: any) => {
        messageYById.set(message.id, resolveMessageRow(message.order));
    });

    g.append('text')
        .attr('x', 36)
        .attr('y', 32)
        .text(`Sequence: ${diagram.name}`)
        .style('font-size', '16px')
        .style('font-weight', '700')
        .style('fill', 'var(--vscode-editor-foreground)');

    if (diagram.packagePath) {
        g.append('text')
            .attr('x', 36)
            .attr('y', 52)
            .text(diagram.packagePath)
            .style('font-size', '11px')
            .style('fill', 'var(--vscode-descriptionForeground)');
    }

    const fragmentGroup = g.append('g').attr('class', 'sequence-fragments');
    const lifelineGroup = g.append('g').attr('class', 'sequence-lifelines');
    const activationGroup = g.append('g').attr('class', 'sequence-activations');
    const messageGroup = g.append('g').attr('class', 'sequence-messages');

    const allMessageIds = messages.map((message: any) => message.id);
    const messageIndexById = new Map<string, number>(allMessageIds.map((id: string, index: number) => [id, index]));
    const lifelineIndexById = new Map<string, number>(lifelines.map((lifeline: any, index: number) => [lifeline.id, index]));

    const collectFragmentMessageIds = (fragment: any): string[] => {
        const direct = Array.isArray(fragment?.messageIds) ? fragment.messageIds : [];
        const operandIds = Array.isArray(fragment?.operands)
            ? fragment.operands.flatMap((operand: any) => [
                ...(Array.isArray(operand?.messageIds) ? operand.messageIds : []),
                ...(Array.isArray(operand?.fragments) ? operand.fragments.flatMap((nested: any) => collectFragmentMessageIds(nested)) : []),
            ])
            : [];
        const nestedIds = Array.isArray(fragment?.fragments)
            ? fragment.fragments.flatMap((nested: any) => collectFragmentMessageIds(nested))
            : [];
        return Array.from(new Set([...direct, ...operandIds, ...nestedIds]));
    };

    const fragmentBounds = (fragment: any) => {
        const allIds = collectFragmentMessageIds(fragment).filter((id: string) => messageYById.has(id));
        if (allIds.length === 0) return null;
        const messageIndexes = allIds
            .map((id: string) => messageIndexById.get(id))
            .filter((value: any): value is number => typeof value === 'number');
        if (!messageIndexes.length) return null;
        const minMessageId = allMessageIds[Math.min(...messageIndexes)];
        const maxMessageId = allMessageIds[Math.max(...messageIndexes)];
        const yStart = (messageYById.get(minMessageId) || LIFELINE_TOP) - FRAGMENT_TOP_PADDING;
        const yEnd = (messageYById.get(maxMessageId) || yStart) + FRAGMENT_BOTTOM_PADDING;

        const relatedLifelines = new Set<string>();
        allIds.forEach((messageId: string) => {
            const message = messages.find((candidate: any) => candidate.id === messageId);
            if (!message) return;
            relatedLifelines.add(message.from);
            relatedLifelines.add(message.to);
        });
        if (fragment.kind === 'ref' && fragment.target) {
            lifelines.forEach((lifeline: any) => relatedLifelines.add(lifeline.id));
        }
        const lifelineIndexes = Array.from(relatedLifelines)
            .map((lifelineId) => lifelineIndexById.get(lifelineId))
            .filter((value: any): value is number => typeof value === 'number');
        if (!lifelineIndexes.length) return null;
        const minX = xOffset + (Math.min(...lifelineIndexes) * LIFELINE_GAP) - (LIFELINE_BOX_WIDTH / 2) - FRAGMENT_PADDING;
        const maxX = xOffset + (Math.max(...lifelineIndexes) * LIFELINE_GAP) + (LIFELINE_BOX_WIDTH / 2) + FRAGMENT_PADDING;
        return { x: minX, y: yStart, width: maxX - minX, height: yEnd - yStart };
    };

    const renderFragment = (fragment: any, depth = 0) => {
            const bounds = fragmentBounds(fragment);
            if (!bounds) return;
            const color = fragmentColor(fragment.kind);
            const inset = depth * 12;
            const fragmentNode = fragmentGroup.append('g')
                .attr('class', `sequence-fragment ${fragment.kind}`)
                .style('cursor', fragment.range ? 'pointer' : 'default');

            fragmentNode.append('rect')
                .attr('x', bounds.x + inset)
                .attr('y', bounds.y + inset)
                .attr('width', Math.max(60, bounds.width - (inset * 2)))
                .attr('height', Math.max(40, bounds.height - (inset * 2)))
                .style('fill', 'rgba(0, 0, 0, 0)')
                .style('stroke', color)
                .style('stroke-width', '1.5px')
                .style('stroke-dasharray', fragment.kind === 'ref' ? '7 5' : 'none');

            fragmentNode.append('rect')
                .attr('x', bounds.x + inset)
                .attr('y', bounds.y + inset)
                .attr('width', Math.max(72, ((fragment.kind || '').length + String(fragment.label || fragment.target || '').length) * 6 + 38))
                .attr('height', FRAGMENT_HEADER_HEIGHT)
                .style('fill', 'var(--vscode-editor-background)')
                .style('stroke', color)
                .style('stroke-width', '1.5px');

            fragmentNode.append('text')
                .attr('x', bounds.x + inset + 10)
                .attr('y', bounds.y + inset + 16)
                .text(`${String(fragment.kind || '').toUpperCase()}${fragment.label ? ` ${fragment.label}` : fragment.target ? ` ${fragment.target}` : ''}`)
                .style('font-size', '10px')
                .style('font-weight', '700')
                .style('fill', color);

            (fragment.operands || []).forEach((operand: any, operandIndex: number) => {
                const operandIds = (operand.messageIds || []).filter((id: string) => messageYById.has(id));
                if (operandIndex > 0 && operandIds.length > 0) {
                    const separatorY = (messageYById.get(operandIds[0]) || bounds.y) - 30;
                    fragmentNode.append('line')
                        .attr('x1', bounds.x + inset)
                        .attr('y1', separatorY)
                        .attr('x2', bounds.x + bounds.width - inset)
                        .attr('y2', separatorY)
                        .style('stroke', color)
                        .style('stroke-width', '1px')
                        .style('stroke-dasharray', '5 4');
                }
                if (operand.guard) {
                    const guardY = operandIndex === 0
                        ? (bounds.y + inset + FRAGMENT_HEADER_HEIGHT + FRAGMENT_GUARD_PADDING)
                        : (operandIds.length > 0
                            ? ((messageYById.get(operandIds[0]) || bounds.y) - 14)
                            : (bounds.y + inset + FRAGMENT_HEADER_HEIGHT + FRAGMENT_GUARD_PADDING + (operandIndex * 22)));
                    fragmentNode.append('text')
                        .attr('x', bounds.x + inset + 12)
                        .attr('y', guardY)
                        .text(`[${operand.guard}]`)
                        .style('font-size', '9px')
                        .style('font-style', 'italic')
                        .style('fill', color);
                }

                (operand.fragments || []).forEach((nested: any) => renderFragment(nested, depth + 1));
            });

            if (fragment.range) {
                fragmentNode.on('click', (event: any) => {
                    event.stopPropagation();
                    clearVisualHighlights();
                    postJumpToElement(postMessage, { name: fragment.label || fragment.target || fragment.id, uri: fragment.uri, range: fragment.range }, { skipCentering: true });
                });
            }
            (fragment.fragments || []).forEach((nested: any) => renderFragment(nested, depth + 1));
        };

    [...(diagram.fragments || [])]
        .sort((a: any, b: any) => a.order - b.order || String(a.id).localeCompare(String(b.id)))
        .forEach((fragment: any) => renderFragment(fragment, 0));

    lifelines.forEach((lifeline: any) => {
        const centerX = lifelineX.get(lifeline.id) || 0;
        const group = lifelineGroup.append('g')
            .attr('class', 'sequence-lifeline')
            .style('cursor', lifeline.range ? 'pointer' : 'default');

        group.append('rect')
            .attr('x', centerX - (LIFELINE_BOX_WIDTH / 2))
            .attr('y', LIFELINE_TOP)
            .attr('width', LIFELINE_BOX_WIDTH)
            .attr('height', LIFELINE_BOX_HEIGHT)
            .attr('rx', 8)
            .style('fill', 'var(--vscode-editor-background)')
            .style('stroke', 'var(--vscode-panel-border)')
            .style('stroke-width', '1.5px');

        group.append('text')
            .attr('x', centerX)
            .attr('y', LIFELINE_TOP + 16)
            .attr('text-anchor', 'middle')
            .text(lifeline.name)
            .style('font-size', '12px')
            .style('font-weight', '700')
            .style('fill', 'var(--vscode-editor-foreground)');

        if (lifeline.type) {
            group.append('text')
                .attr('x', centerX)
                .attr('y', LIFELINE_TOP + 29)
                .attr('text-anchor', 'middle')
                .text(`:${lifeline.type}`)
                .style('font-size', '10px')
                .style('fill', 'var(--vscode-descriptionForeground)');
        }

        group.append('line')
            .attr('x1', centerX)
            .attr('y1', LIFELINE_TOP + LIFELINE_BOX_HEIGHT)
            .attr('x2', centerX)
            .attr('y2', lifelineBottom)
            .style('stroke', 'var(--vscode-panel-border)')
            .style('stroke-width', '1.2px')
            .style('stroke-dasharray', '6 6');

        if (lifeline.range) {
            group.on('click', (event: any) => {
                event.stopPropagation();
                clearVisualHighlights();
                postJumpToElement(postMessage, { name: lifeline.name, id: lifeline.id, uri: lifeline.uri, range: lifeline.range }, { skipCentering: true });
            });
        }
    });

    (diagram.activations || []).forEach((activation: any) => {
        const centerX = lifelineX.get(activation.lifeline);
        if (typeof centerX !== 'number') return;
        const startY = messageYById.get(activation.startMessage || '') || resolveMessageRow(activation.order);
        const finishY = messageYById.get(activation.finishMessage || '') || (startY + (MESSAGE_GAP * 0.8));
        const topY = Math.min(startY, finishY) - 6;
        const heightPx = Math.max(26, Math.abs(finishY - startY) + 12);

        activationGroup.append('rect')
            .attr('x', centerX - (ACTIVATION_WIDTH / 2))
            .attr('y', topY)
            .attr('width', ACTIVATION_WIDTH)
            .attr('height', heightPx)
            .style('fill', 'rgba(91, 143, 196, 0.18)')
            .style('stroke', '#5B8FC4')
            .style('stroke-width', '1.2px');
    });

    messages.forEach((message: any) => {
        const fromX = lifelineX.get(message.from);
        const toX = lifelineX.get(message.to);
        if (typeof fromX !== 'number' || typeof toX !== 'number') return;
        const y = messageYById.get(message.id) || resolveMessageRow(message.order);
        const pathGroup = messageGroup.append('g')
            .attr('class', `sequence-message ${message.kind}`)
            .style('cursor', message.range ? 'pointer' : 'default');

        pathGroup.append('line')
            .attr('x1', fromX)
            .attr('y1', y)
            .attr('x2', toX)
            .attr('y2', y)
            .style('stroke', 'var(--vscode-editor-foreground)')
            .style('stroke-width', '1.8px')
            .style('stroke-dasharray', message.kind === 'return' ? '7 5' : 'none')
            .style('marker-end', `url(#${markerId(message.kind)})`);

        if (message.kind === 'create') {
            pathGroup.append('circle')
                .attr('cx', toX)
                .attr('cy', y)
                .attr('r', 7)
                .style('fill', 'var(--vscode-editor-background)')
                .style('stroke', 'var(--vscode-editor-foreground)')
                .style('stroke-width', '1.5px');
        }

        pathGroup.append('text')
            .attr('x', (fromX + toX) / 2)
            .attr('y', y - 10)
            .attr('text-anchor', 'middle')
            .text(message.label || message.name)
            .style('font-size', '11px')
            .style('font-weight', '600')
            .style('fill', 'var(--vscode-editor-foreground)');

        if (message.range) {
            pathGroup.on('click', (event: any) => {
                event.stopPropagation();
                clearVisualHighlights();
                postJumpToElement(postMessage, { name: message.name, id: message.id, uri: message.uri, range: message.range }, { skipCentering: true });
            });
        }
    });
}

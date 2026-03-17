/**
 * Sequence diagram view renderer - participants and messages.
 */
/* eslint-disable @typescript-eslint/no-explicit-any */

import type { RenderContext } from '../types';
import { isLibraryValidated, isActorElement, renderActorGlyph } from '../shared';
import { postJumpToElement } from '../jumpToElement';

declare const d3: any;

export function renderSequenceView(ctx: RenderContext, data: any): void {
    const { width, height, g, postMessage, onStartInlineEdit } = ctx;

    if (!data || !data.sequenceDiagrams || data.sequenceDiagrams.length === 0) {
        const messageGroup = g.append('g')
            .attr('class', 'sequence-message');

        messageGroup.append('text')
            .attr('x', width / 2)
            .attr('y', height / 2)
            .attr('text-anchor', 'middle')
            .text('No sequence diagrams found in this SysML model')
            .style('font-size', '18px')
            .style('fill', 'var(--vscode-descriptionForeground)')
            .style('font-weight', 'bold');

        messageGroup.append('text')
            .attr('x', width / 2)
            .attr('y', height / 2 + 30)
            .attr('text-anchor', 'middle')
            .text('Add "interaction def" elements to see sequence diagrams')
            .style('font-size', '14px')
            .style('fill', 'var(--vscode-descriptionForeground)');

        return;
    }

    const diagrams = data.sequenceDiagrams;
    let currentY = 50;

    diagrams.forEach((diagram: any) => {
        const diagramGroup = g.append('g')
            .attr('class', 'sequence-diagram')
            .attr('transform', 'translate(0, ' + currentY + ')');

        diagramGroup.append('text')
            .attr('x', width / 2)
            .attr('y', 0)
            .attr('text-anchor', 'middle')
            .text(diagram.name)
            .style('font-size', '20px')
            .style('font-weight', 'bold')
            .style('fill', 'var(--vscode-editor-foreground)')
            .on('click', () => {
                postJumpToElement(postMessage, { name: diagram.name, id: diagram.id });
            })
            .style('cursor', 'pointer');

        const participants = diagram.participants;
        const messages = diagram.messages;
        const participantWidth = Math.min(150, (width - 100) / participants.length);
        const participantSpacing = (width - 100) / Math.max(1, participants.length - 1);
        const messageHeight = 80;
        const diagramHeight = Math.max(400, messages.length * messageHeight + 200);

        participants.forEach((participant: any, i: number) => {
            const participantX = 50 + (i * participantSpacing);
            const isLibValidated = participant.element ? isLibraryValidated(participant.element) : false;
            const borderColor = isLibValidated ? 'var(--vscode-charts-green)' : 'var(--vscode-panel-border)';
            const borderWidth = isLibValidated ? '3px' : '2px';

            const participantGroup = diagramGroup.append('g')
                .attr('class', 'sequence-participant')
                .attr('transform', 'translate(' + participantX + ', 40)')
                .style('cursor', 'pointer');

            if (isActorElement(participant)) {
                const actorContainer = participantGroup.append('g')
                    .attr('transform', 'translate(0, 0)');
                renderActorGlyph(actorContainer);

                participantGroup.append('text')
                    .attr('class', 'node-name-text')
                    .attr('data-element-name', participant.name)
                    .attr('x', 0)
                    .attr('y', 45)
                    .attr('text-anchor', 'middle')
                    .text(participant.name)
                    .style('font-size', '14px')
                    .style('font-weight', 'bold')
                    .style('fill', 'var(--vscode-editor-foreground)');

                participantGroup.append('text')
                    .attr('x', 0)
                    .attr('y', 62)
                    .attr('text-anchor', 'middle')
                    .text('[' + participant.type + ']')
                    .style('font-size', '11px')
                    .style('fill', 'var(--vscode-descriptionForeground)');
            } else {
                participantGroup.append('rect')
                    .attr('x', -participantWidth / 2)
                    .attr('y', 0)
                    .attr('width', participantWidth)
                    .attr('height', 60)
                    .attr('rx', 8)
                    .style('fill', 'var(--vscode-editor-background)')
                    .style('stroke', borderColor)
                    .style('stroke-width', borderWidth);

                participantGroup.append('text')
                    .attr('class', 'node-name-text')
                    .attr('data-element-name', participant.name)
                    .attr('x', 0)
                    .attr('y', 25)
                    .attr('text-anchor', 'middle')
                    .text(participant.name)
                    .style('font-size', '14px')
                    .style('font-weight', 'bold')
                    .style('fill', 'var(--vscode-editor-foreground)');

                participantGroup.append('text')
                    .attr('x', 0)
                    .attr('y', 42)
                    .attr('text-anchor', 'middle')
                    .text('[' + participant.type + ']')
                    .style('font-size', '11px')
                    .style('fill', 'var(--vscode-descriptionForeground)');
            }

            participantGroup.on('click', function (event: any) {
                event.stopPropagation();
                postJumpToElement(postMessage, { name: participant.name, id: participant.id || participant.element?.id });
            })
                .on('dblclick', function (event: any) {
                    event.stopPropagation();
                    onStartInlineEdit(d3.select(this), participant.name,
                        participantX - participantWidth / 2, 40, participantWidth);
                });

            const lifelineY = isActorElement(participant) ? 70 : 60;
            participantGroup.append('line')
                .attr('x1', 0)
                .attr('y1', lifelineY)
                .attr('x2', 0)
                .attr('y2', diagramHeight - 60)
                .style('stroke', 'var(--vscode-panel-border)')
                .style('stroke-width', '2px')
                .style('stroke-dasharray', '5,5');
        });

        messages.forEach((message: any, messageIndex: number) => {
            const fromParticipant = participants.find((p: any) => p.name === message.from);
            const toParticipant = participants.find((p: any) => p.name === message.to);

            if (!fromParticipant || !toParticipant) {
                return;
            }

            const fromIndex = participants.indexOf(fromParticipant);
            const toIndex = participants.indexOf(toParticipant);
            const fromX = 50 + (fromIndex * participantSpacing);
            const toX = 50 + (toIndex * participantSpacing);
            const messageY = 120 + (messageIndex * messageHeight);

            const messageGroup = diagramGroup.append('g')
                .attr('class', 'sequence-message')
                .on('click', () => {
                    postJumpToElement(postMessage, { name: message.name, id: message.id });
                })
                .style('cursor', 'pointer');

            const arrowPath = fromX < toX
                ? 'M ' + fromX + ' ' + messageY + ' L ' + (toX - 10) + ' ' + messageY + ' L ' + (toX - 20) + ' ' + (messageY - 5) + ' M ' + (toX - 10) + ' ' + messageY + ' L ' + (toX - 20) + ' ' + (messageY + 5)
                : 'M ' + fromX + ' ' + messageY + ' L ' + (toX + 10) + ' ' + messageY + ' L ' + (toX + 20) + ' ' + (messageY - 5) + ' M ' + (toX + 10) + ' ' + messageY + ' L ' + (toX + 20) + ' ' + (messageY + 5);

            messageGroup.append('path')
                .attr('d', arrowPath)
                .style('stroke', 'var(--vscode-charts-blue)')
                .style('stroke-width', '2px')
                .style('fill', 'none');

            const labelX = (fromX + toX) / 2;
            const labelText = message.payload || message.name;
            const labelWidth = Math.max(100, labelText.length * 8);

            messageGroup.append('rect')
                .attr('x', labelX - labelWidth / 2)
                .attr('y', messageY - 25)
                .attr('width', labelWidth)
                .attr('height', 20)
                .attr('rx', 4)
                .style('fill', 'var(--vscode-editor-background)')
                .style('stroke', 'var(--vscode-charts-blue)')
                .style('stroke-width', '1px');

            messageGroup.append('text')
                .attr('x', labelX)
                .attr('y', messageY - 10)
                .attr('text-anchor', 'middle')
                .text(labelText)
                .style('font-size', '12px')
                .style('fill', 'var(--vscode-editor-foreground)')
                .style('pointer-events', 'none');

            if (message.occurrence > 0) {
                messageGroup.append('text')
                    .attr('x', Math.min(fromX, toX) - 30)
                    .attr('y', messageY + 5)
                    .text(message.occurrence + 's')
                    .style('font-size', '10px')
                    .style('fill', 'var(--vscode-descriptionForeground)')
                    .style('font-style', 'italic');
            }
        });

        currentY += diagramHeight + 100;
    });
}

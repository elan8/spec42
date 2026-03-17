/**
 * Activity/Action Flow View renderer - decisions, merge nodes, swim lanes.
 */
/* eslint-disable @typescript-eslint/no-explicit-any */

import type { RenderContext } from '../types';
import { postJumpToElement } from '../jumpToElement';

declare const d3: any;

export function renderActivityView(ctx: RenderContext, data: any): void {
    const { width, height, svg, g, activityLayoutDirection, activityDebugLabels, selectedDiagramIndex, postMessage, onStartInlineEdit, renderPlaceholder } = ctx;

    if (!data || !data.diagrams || data.diagrams.length === 0) {
        renderPlaceholder(width, height, 'Action Flow View',
            'No activity diagrams found to display.\\n\\nThis view shows action flows with decisions, merge nodes, and swim lanes.',
            data);
        return;
    }

    const diagramIndex = Math.min(selectedDiagramIndex, data.diagrams.length - 1);
    const diagram = data.diagrams[diagramIndex];

    const allActions = (diagram.actions || []).map((action: any, idx: number) => ({
        ...action,
        id: action.id || action.name || 'action_' + idx,
        name: action.name || action.id || 'Action ' + (idx + 1)
    }));

    const actions = allActions.filter((action: any) => !action.parent);
    const nestedActions = allActions.filter((action: any) => action.parent);

    const containerChildren = new Map<string, any[]>();
    nestedActions.forEach((action: any) => {
        if (!containerChildren.has(action.parent)) {
            containerChildren.set(action.parent, []);
        }
        containerChildren.get(action.parent)!.push(action);
    });

    let flows = diagram.flows || [];

    if (flows.length === 0 && actions.length > 1) {
        flows = [];
        for (let i = 0; i < actions.length - 1; i++) {
            flows.push({
                from: actions[i].id || actions[i].name,
                to: actions[i + 1].id || actions[i + 1].name,
                type: 'control'
            });
        }
    }

    const isHorizontal = activityLayoutDirection === 'horizontal';
    const actionWidth = 220;
    const actionHeight = 60;
    const verticalSpacing = 100;
    const horizontalSpacing = 60;
    const startX = 80;
    const startY = 80;
    const swimLaneWidth = 280;

    const swimLanes = new Map<string, any[]>();
    const noLaneActions: any[] = [];

    actions.forEach((action: any) => {
        if (action.lane) {
            if (!swimLanes.has(action.lane)) {
                swimLanes.set(action.lane, []);
            }
            swimLanes.get(action.lane)!.push(action);
        } else {
            noLaneActions.push(action);
        }
    });

    const actionPositions = new Map<string, { x: number; y: number; action: any }>();
    const levels = new Map<string, number>();
    const visited = new Set<string>();

    function calculateLevel(actionId: string): number {
        if (visited.has(actionId)) {
            return levels.get(actionId) || 0;
        }
        visited.add(actionId);

        const incomingFlows = flows.filter((f: any) => f.to === actionId);
        let maxSourceLevel = -1;

        incomingFlows.forEach((flow: any) => {
            const sourceLevel = calculateLevel(flow.from);
            maxSourceLevel = Math.max(maxSourceLevel, sourceLevel);
        });

        const level = maxSourceLevel + 1;
        levels.set(actionId, level);
        return level;
    }

    actions.forEach((action: any) => {
        if (!visited.has(action.id)) {
            calculateLevel(action.id);
        }
    });

    const actionsByLevel = new Map<number, any[]>();
    actions.forEach((action: any) => {
        const level = levels.get(action.id) || 0;
        if (!actionsByLevel.has(level)) {
            actionsByLevel.set(level, []);
        }
        actionsByLevel.get(level)!.push(action);
    });

    const childPadding = 10;
    const childActionHeight = 35;
    const childSpacing = 8;
    function getActionHeight(action: any): number {
        const children = containerChildren.get(action.name || action.id);
        if (children && children.length > 0) {
            return 30 + children.length * (childActionHeight + childSpacing) + childPadding;
        }
        return actionHeight;
    }

    const levelYPositions = new Map<number, number>();
    const sortedLevels = Array.from(actionsByLevel.keys()).sort((a, b) => a - b);
    let cumulativeY = startY;
    sortedLevels.forEach(level => {
        levelYPositions.set(level, cumulativeY);
        const actionsAtLevel = actionsByLevel.get(level) || [];
        const maxHeightAtLevel = Math.max(...actionsAtLevel.map((a: any) => getActionHeight(a)), actionHeight);
        cumulativeY += maxHeightAtLevel + verticalSpacing - actionHeight;
    });

    let laneIndex = 0;
    const lanePositions = new Map<string, { x: number; index: number }>();

    if (swimLanes.size > 0) {
        swimLanes.forEach((laneActions: any[], laneName: string) => {
            const laneX = 60 + laneIndex * (swimLaneWidth + 40);
            lanePositions.set(laneName, { x: laneX, index: laneIndex });

            laneActions.forEach((action: any) => {
                const level = levels.get(action.id) || 0;
                actionPositions.set(action.id, {
                    x: laneX + (swimLaneWidth - actionWidth) / 2,
                    y: levelYPositions.get(level) || startY + level * verticalSpacing,
                    action: action
                });
            });

            laneIndex++;
        });

        if (noLaneActions.length > 0) {
            const noLaneX = 60 + laneIndex * (swimLaneWidth + 40);

            const noLaneActionsByLevel = new Map<number, any[]>();
            noLaneActions.forEach((action: any) => {
                const level = levels.get(action.id) || 0;
                if (!noLaneActionsByLevel.has(level)) {
                    noLaneActionsByLevel.set(level, []);
                }
                noLaneActionsByLevel.get(level)!.push(action);
            });

            noLaneActions.forEach((action: any) => {
                const level = levels.get(action.id) || 0;
                const actionsAtLevel = noLaneActionsByLevel.get(level) || [action];
                const positionInLevel = actionsAtLevel.indexOf(action);
                const totalAtLevel = actionsAtLevel.length;
                const centerOffset = (totalAtLevel - 1) * (actionWidth + horizontalSpacing) / 2;

                actionPositions.set(action.id, {
                    x: noLaneX + (swimLaneWidth / 2) - centerOffset + positionInLevel * (actionWidth + horizontalSpacing),
                    y: levelYPositions.get(level) || startY + level * verticalSpacing,
                    action: action
                });
            });
        }
    } else {
        actions.forEach((action: any) => {
            const level = levels.get(action.id) || 0;
            const actionsAtLevel = actionsByLevel.get(level) || [action];
            const positionInLevel = actionsAtLevel.indexOf(action);
            const totalAtLevel = actionsAtLevel.length;

            if (isHorizontal) {
                const centerOffset = (totalAtLevel - 1) * (actionHeight + verticalSpacing) / 2;
                actionPositions.set(action.id, {
                    x: startX + level * (actionWidth + horizontalSpacing),
                    y: height / 2 - centerOffset + positionInLevel * (actionHeight + verticalSpacing),
                    action: action
                });
            } else {
                const centerOffset = (totalAtLevel - 1) * (actionWidth + horizontalSpacing) / 2;
                const yPos = levelYPositions.get(level) || startY + level * verticalSpacing;
                actionPositions.set(action.id, {
                    x: width / 2 - centerOffset + positionInLevel * (actionWidth + horizontalSpacing),
                    y: yPos,
                    action: action
                });
            }
        });
    }

    if (swimLanes.size > 0) {
        const maxLevel = Math.max(...Array.from(levels.values()), 0);
        const lastLevelY = levelYPositions.get(maxLevel) || startY + maxLevel * verticalSpacing;
        const laneHeight = lastLevelY + 100;

        lanePositions.forEach((pos, laneName) => {
            g.append('rect')
                .attr('x', pos.x - 10)
                .attr('y', 20)
                .attr('width', swimLaneWidth)
                .attr('height', laneHeight)
                .attr('rx', 4)
                .style('fill', 'none')
                .style('stroke', 'var(--vscode-panel-border)')
                .style('stroke-width', '2px')
                .style('stroke-dasharray', '5,5')
                .style('opacity', 0.5);

            g.append('text')
                .attr('x', pos.x + swimLaneWidth / 2)
                .attr('y', 40)
                .attr('text-anchor', 'middle')
                .text(laneName)
                .style('font-size', '12px')
                .style('font-weight', 'bold')
                .style('fill', 'var(--vscode-descriptionForeground)');
        });
    }

    const flowGroup = g.append('g').attr('class', 'activity-flows');

    const flowsFromSource = new Map<string, any[]>();
    flows.forEach((flow: any) => {
        if (!flowsFromSource.has(flow.from)) {
            flowsFromSource.set(flow.from, []);
        }
        flowsFromSource.get(flow.from)!.push(flow);
    });

    const flowsToTarget = new Map<string, any[]>();
    flows.forEach((flow: any) => {
        if (!flowsToTarget.has(flow.to)) {
            flowsToTarget.set(flow.to, []);
        }
        flowsToTarget.get(flow.to)!.push(flow);
    });

    flows.forEach((flow: any) => {
        const sourcePos = actionPositions.get(flow.from);
        const targetPos = actionPositions.get(flow.to);

        if (!sourcePos || !targetPos) return;

        let pathData: string;
        let labelX: number, labelY: number;

        const siblingsFromSource = flowsFromSource.get(flow.from) || [flow];
        const siblingIndexFromSource = siblingsFromSource.indexOf(flow);
        const totalSiblingsFromSource = siblingsFromSource.length;

        const siblingsToTarget = flowsToTarget.get(flow.to) || [flow];
        const siblingIndexToTarget = siblingsToTarget.indexOf(flow);
        const totalSiblingsToTarget = siblingsToTarget.length;

        if (isHorizontal) {
            const flowStartX = sourcePos.x + actionWidth;
            const flowStartY = sourcePos.y + actionHeight / 2;
            const endX = targetPos.x;
            const endY = targetPos.y + actionHeight / 2;

            const midX = (flowStartX + endX) / 2;
            pathData = 'M ' + flowStartX + ',' + flowStartY +
                       ' L ' + midX + ',' + flowStartY +
                       ' L ' + midX + ',' + endY +
                       ' L ' + endX + ',' + endY;
            labelX = midX;
            labelY = (flowStartY + endY) / 2 - 5;
        } else {
            const flowStartX = sourcePos.x + actionWidth / 2;
            const flowStartY = sourcePos.y + actionHeight;
            const endX = targetPos.x + actionWidth / 2;
            const endY = targetPos.y;

            let startXOffset = 0;
            let endXOffset = 0;
            const isForkSource = (sourcePos.action?.kind === 'fork' || sourcePos.action?.type === 'fork');
            const isJoinTarget = (targetPos.action?.kind === 'join' || targetPos.action?.type === 'join');

            if (isForkSource && totalSiblingsFromSource > 1) {
                const offsetRange = Math.min(actionWidth * 0.8, 100);
                startXOffset = (siblingIndexFromSource - (totalSiblingsFromSource - 1) / 2) * (offsetRange / (totalSiblingsFromSource - 1 || 1));
            }

            if (isJoinTarget && totalSiblingsToTarget > 1) {
                const offsetRange = Math.min(actionWidth * 0.8, 100);
                endXOffset = (siblingIndexToTarget - (totalSiblingsToTarget - 1) / 2) * (offsetRange / (totalSiblingsToTarget - 1 || 1));
            }

            const adjustedStartX = flowStartX + startXOffset;
            const adjustedEndX = endX + endXOffset;

            let midY: number;
            if (isJoinTarget) {
                midY = endY - 15;
            } else {
                midY = (flowStartY + endY) / 2;
            }

            pathData = 'M ' + adjustedStartX + ',' + flowStartY +
                       ' L ' + adjustedStartX + ',' + midY +
                       ' L ' + adjustedEndX + ',' + midY +
                       ' L ' + adjustedEndX + ',' + endY;

            labelX = (adjustedStartX + adjustedEndX) / 2;
            labelY = midY - 5;
        }

        flowGroup.append('path')
            .attr('d', pathData)
            .style('fill', 'none')
            .style('stroke', 'var(--vscode-charts-blue)')
            .style('stroke-width', '2px')
            .style('marker-end', 'url(#activity-arrowhead)');

        const guardLabel = flow.guard || flow.condition;

        if (guardLabel) {
            let displayLabel: string;
            const trimmedGuard = String(guardLabel).trim();

            const enumMatch = trimmedGuard.match(/::(\w+)/);
            if (enumMatch) {
                displayLabel = enumMatch[1];
            } else {
                displayLabel = trimmedGuard.length > 25 ? trimmedGuard.substring(0, 22) + '...' : trimmedGuard;
            }

            const labelText = '[' + displayLabel + ']';
            const labelWidth = labelText.length * 6 + 8;

            flowGroup.append('rect')
                .attr('x', labelX - labelWidth / 2)
                .attr('y', labelY - 10)
                .attr('width', labelWidth)
                .attr('height', 14)
                .attr('rx', 3)
                .style('fill', 'var(--vscode-editor-background)')
                .style('stroke', 'var(--vscode-charts-orange)')
                .style('stroke-width', '1px')
                .style('opacity', 0.9);

            flowGroup.append('text')
                .attr('x', labelX)
                .attr('y', labelY)
                .attr('text-anchor', 'middle')
                .text(labelText)
                .style('font-size', '10px')
                .style('fill', 'var(--vscode-charts-orange)')
                .style('font-weight', 'bold');
        }
    });

    const defs = svg.select('defs').empty() ? svg.append('defs') : svg.select('defs');

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
        .style('fill', 'var(--vscode-charts-blue)');

    const actionGroup = g.append('g').attr('class', 'activity-actions');

    function truncateToFit(text: string | null | undefined, maxChars: number): string {
        if (!text) return '';
        if (text.length <= maxChars) return text;
        return text.substring(0, maxChars - 2) + '..';
    }

    function handleActionClick(action: any) {
        if (action && action.name) {
            postJumpToElement(postMessage, { name: action.name, id: action.id }, { parentContext: diagram.name });
        }
    }

    actionPositions.forEach((pos, actionId) => {
        const action = pos.action;
        const actionKind = (action.kind || action.type || 'action').toLowerCase();
        const actionName = action.name || actionId || 'unnamed';

        const isDecision = actionKind.includes('decision') || actionKind.includes('merge');
        const isFork = actionKind.includes('fork') || actionKind.includes('join');
        const isStart = actionKind.includes('initial') || actionKind.includes('start') || actionName === 'start';
        const isEnd = actionKind.includes('final') || actionKind.includes('end') || actionKind.includes('done') || actionName === 'done';

        const actionElement = actionGroup.append('g')
            .attr('class', 'activity-action')
            .attr('transform', 'translate(' + pos.x + ',' + pos.y + ')')
            .style('cursor', 'pointer')
            .on('click', function(event: any) {
                event.stopPropagation();
                handleActionClick(action);
            });

        if (isStart || isEnd) {
            actionElement.append('circle')
                .attr('cx', actionWidth / 2)
                .attr('cy', actionHeight / 2)
                .attr('r', 20)
                .style('fill', isStart ? 'var(--vscode-charts-green)' : 'var(--vscode-charts-red)')
                .style('stroke', 'var(--vscode-panel-border)')
                .style('stroke-width', '3px');

            if (isEnd) {
                actionElement.append('circle')
                    .attr('cx', actionWidth / 2)
                    .attr('cy', actionHeight / 2)
                    .attr('r', 12)
                    .style('fill', 'var(--vscode-charts-red)')
                    .style('stroke', 'none');
            }
        } else if (isDecision) {
            const diamond = 'M ' + (actionWidth / 2) + ',0 ' +
                          'L ' + actionWidth + ',' + (actionHeight / 2) + ' ' +
                          'L ' + (actionWidth / 2) + ',' + actionHeight + ' ' +
                          'L 0,' + (actionHeight / 2) + ' Z';

            actionElement.append('path')
                .attr('d', diamond)
                .style('fill', 'var(--vscode-editor-background)')
                .style('stroke', 'var(--vscode-charts-orange)')
                .style('stroke-width', '2px');

            let decisionLabel = '?';
            if (actionKind.includes('merge')) {
                decisionLabel = truncateToFit(actionName, 18);
            } else if (action.condition && action.condition !== 'decide') {
                decisionLabel = truncateToFit(action.condition, 18);
            }

            actionElement.append('text')
                .attr('x', actionWidth / 2)
                .attr('y', actionHeight / 2 + 5)
                .attr('text-anchor', 'middle')
                .text(decisionLabel)
                .style('font-size', actionKind.includes('merge') ? '11px' : '16px')
                .style('font-weight', 'bold')
                .style('fill', 'var(--vscode-editor-foreground)')
                .style('user-select', 'none');
        } else if (isFork) {
            actionElement.append('rect')
                .attr('x', 0)
                .attr('y', actionHeight / 2 - 5)
                .attr('width', actionWidth)
                .attr('height', 10)
                .attr('rx', 2)
                .style('fill', 'var(--vscode-panel-border)')
                .style('stroke', 'none');

            if (activityDebugLabels) {
                actionElement.append('text')
                    .attr('class', 'fork-join-debug-label')
                    .attr('x', actionWidth / 2)
                    .attr('y', actionHeight / 2 + 25)
                    .attr('text-anchor', 'middle')
                    .text(actionName)
                    .style('font-size', '10px')
                    .style('fill', 'var(--vscode-descriptionForeground)')
                    .style('font-style', 'italic')
                    .style('user-select', 'none');
            }
        } else {
            const children = containerChildren.get(actionName);
            const isContainer = children && children.length > 0;

            let containerWidth = actionWidth;
            let containerHeight = actionHeight;

            if (isContainer) {
                containerWidth = actionWidth + 20;
                containerHeight = 30 + children!.length * (childActionHeight + childSpacing) + childPadding;
            }

            actionElement.append('rect')
                .attr('width', containerWidth)
                .attr('height', containerHeight)
                .attr('rx', 8)
                .style('fill', 'var(--vscode-editor-background)')
                .style('stroke', isContainer ? 'var(--vscode-charts-purple)' : 'var(--vscode-charts-blue)')
                .style('stroke-width', isContainer ? '3px' : '2px');

            const maxChars = isContainer ? 28 : 24;
            const displayName = truncateToFit(actionName, maxChars);

            const fontSize = actionName.length > 20 ? '11px' : '13px';

            actionElement.append('text')
                .attr('class', 'node-name-text')
                .attr('data-element-name', actionName)
                .attr('x', containerWidth / 2)
                .attr('y', isContainer ? 18 : containerHeight / 2 - 5)
                .attr('text-anchor', 'middle')
                .text(displayName)
                .style('font-size', fontSize)
                .style('font-weight', 'bold')
                .style('fill', 'var(--vscode-editor-foreground)')
                .style('user-select', 'none');

            if (isContainer) {
                let childY = 30;
                children!.forEach((child: any) => {
                    const childName = child.name || child;
                    const childDisplayName = truncateToFit(childName, 22);

                    actionElement.append('rect')
                        .attr('x', childPadding)
                        .attr('y', childY)
                        .attr('width', containerWidth - 2 * childPadding)
                        .attr('height', childActionHeight)
                        .attr('rx', 4)
                        .style('fill', 'var(--vscode-editor-inactiveSelectionBackground)')
                        .style('stroke', 'var(--vscode-charts-blue)')
                        .style('stroke-width', '1px')
                        .style('cursor', 'pointer')
                        .on('click', function(event: any) {
                            event.stopPropagation();
                            handleActionClick(child);
                        });

                    actionElement.append('text')
                        .attr('class', 'node-name-text')
                        .attr('data-element-name', childName)
                        .attr('x', containerWidth / 2)
                        .attr('y', childY + childActionHeight / 2 + 4)
                        .attr('text-anchor', 'middle')
                        .text(childDisplayName)
                        .style('font-size', '11px')
                        .style('fill', 'var(--vscode-editor-foreground)')
                        .style('pointer-events', 'none')
                        .style('user-select', 'none');

                    childY += childActionHeight + childSpacing;
                });
            }

            if (!isContainer && actionKind !== 'action' && actionKind !== displayName.toLowerCase()) {
                actionElement.append('text')
                    .attr('x', containerWidth / 2)
                    .attr('y', containerHeight / 2 + 12)
                    .attr('text-anchor', 'middle')
                    .text('«' + truncateToFit(actionKind, 14) + '»')
                    .style('font-size', '9px')
                    .style('fill', 'var(--vscode-descriptionForeground)')
                    .style('user-select', 'none');
            }

            actionElement.on('dblclick', function(event: any) {
                event.stopPropagation();
                onStartInlineEdit(d3.select(this), actionName, pos.x, pos.y, containerWidth);
            });
        }
    });
}

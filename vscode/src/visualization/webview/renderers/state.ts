/**
 * State Transition View renderer - states, transitions, guards.
 */
/* eslint-disable @typescript-eslint/no-explicit-any */

import type { RenderContext } from '../types';
import { postJumpToElement } from '../jumpToElement';

declare const d3: any;

export function renderStateView(ctx: RenderContext, data: any): void {
    const { width, height, svg, g, stateLayoutOrientation, selectedDiagramIndex, postMessage, onStartInlineEdit, renderPlaceholder } = ctx;

    if (!data || !data.states || data.states.length === 0) {
        renderPlaceholder(width, height, 'State Transition View',
            'No states found to display.\\n\\nThis view shows state machines with states, transitions, and guards.',
            data);
        return;
    }

    const allStates = data.states || [];
    const transitions = data.transitions || [];

    const stateMachineMap = new Map<string, { container: any; states: any[]; transitions: any[]; depth: number }>();
    const orphanStates: any[] = [];

    function collectChildStates(container: any, collected: any[] = []): any[] {
        if (container.children && container.children.length > 0) {
            container.children.forEach((child: any) => {
                const childType = (child.type || '').toLowerCase();
                const childName = (child.name || '').toLowerCase();

                const isNestedMachine = childName.endsWith('states') || childType.includes('exhibit');

                if (childType.includes('state') && !childType.includes('def')) {
                    if (!isNestedMachine) {
                        collected.push(child);
                    }
                }
                if (!isNestedMachine && child.children) {
                    collectChildStates(child, collected);
                }
            });
        }
        return collected;
    }

    function findStateMachines(stateList: any[], depth = 0) {
        stateList.forEach((s: any) => {
            const typeLower = (s.type || '').toLowerCase();
            const nameLower = (s.name || '').toLowerCase();

            const isContainer = typeLower.includes('exhibit') ||
                               nameLower.endsWith('states') ||
                               (typeLower.includes('state') && s.children && s.children.length > 0 &&
                                s.children.some((c: any) => (c.type || '').toLowerCase().includes('state')));

            const childStates = collectChildStates(s);
            const isStateMachine = isContainer && (childStates.length > 0 || !typeLower.includes('def'));
            if (isStateMachine) {
                stateMachineMap.set(s.name, {
                    container: s,
                    states: childStates,
                    transitions: [],
                    depth: depth
                });
            }

            if (s.children && s.children.length > 0) {
                findStateMachines(s.children, depth + 1);
            }
        });
    }

    findStateMachines(allStates);

    allStates.forEach((s: any) => {
        const typeLower = (s.type || '').toLowerCase();

        if (typeLower.includes('def') || typeLower.includes('definition')) {
            return;
        }
        if (stateMachineMap.has(s.name)) {
            return;
        }

        let alreadyAssigned = false;
        for (const [, machineData] of stateMachineMap) {
            if (machineData.states.some((existing: any) => existing.name === s.name)) {
                alreadyAssigned = true;
                break;
            }
        }
        if (alreadyAssigned) return;

        if (s.parent) {
            for (const [machineName, machineData] of stateMachineMap) {
                if (s.parent === machineName || (typeof s.parent === 'string' && s.parent.includes(machineName))) {
                    if (!machineData.states.some((existing: any) => existing.name === s.name)) {
                        machineData.states.push(s);
                    }
                    return;
                }
            }
        }

        orphanStates.push(s);
    });

    transitions.forEach((t: any) => {
        for (const [, machineData] of stateMachineMap) {
            const stateIds = new Set(machineData.states.map((s: any) => s.id || s.name));
            if (stateIds.has(t.source) || stateIds.has(t.target)) {
                machineData.transitions.push(t);
                break;
            }
        }
    });

    const stateMachines = Array.from(stateMachineMap.entries()).map(([name, data]) => ({
        name,
        container: data.container,
        states: data.states,
        transitions: data.transitions
    }));

    if (stateMachines.length === 0 && (allStates.length > 0 || orphanStates.length > 0)) {
        stateMachines.push({
            name: 'State Machine',
            container: null,
            states: allStates.filter((s: any) => {
                const typeLower = (s.type || '').toLowerCase();
                return !typeLower.includes('def') && !typeLower.includes('definition');
            }),
            transitions: transitions
        });
    }

    if (orphanStates.length > 0 && stateMachines.length > 0) {
        const firstMachine = stateMachines[0];
        orphanStates.forEach((s: any) => {
            if (!firstMachine.states.find((existing: any) => existing.name === s.name)) {
                firstMachine.states.push(s);
            }
        });
    }

    const machineIndex = Math.min(selectedDiagramIndex, stateMachines.length - 1);
    const selectedMachine = stateMachines[machineIndex];

    if (!selectedMachine || selectedMachine.states.length === 0) {
        renderPlaceholder(width, height, 'State Transition View',
            'No states found in selected state machine.\\n\\nTry selecting a different state machine from the dropdown.',
            data);
        return;
    }

    const states = selectedMachine.states;
    const stateMachineNames = [selectedMachine.name];

    const stateWidth = 160;
    const stateHeight = 60;
    const horizontalSpacing = 80;
    const verticalSpacing = 100;
    const marginLeft = 80;
    const marginTop = stateMachineNames.length > 0 ? 110 : 80;

    const getStateKey = (state: any) => state.id || state.name || ('state-' + Math.random().toString(36).substr(2, 9));

    const stateUsages = states.filter((s: any) => {
        const typeLower = (s.type || '').toLowerCase();
        const nameLower = (s.name || '').toLowerCase();

        if (typeLower.includes('def') || typeLower.includes('definition')) {
            return false;
        }
        if (nameLower.endsWith('states') || nameLower.includes('machine')) {
            return false;
        }
        return true;
    });

    const stateKeys = new Set(stateUsages.map((s: any) => getStateKey(s)));
    const outgoing = new Map<string, string[]>();
    const incoming = new Map<string, string[]>();

    stateUsages.forEach((s: any) => {
        const key = getStateKey(s);
        outgoing.set(key, []);
        incoming.set(key, []);
    });

    const machineTransitions = selectedMachine.transitions || transitions;
    machineTransitions.forEach((t: any) => {
        if (stateKeys.has(t.source) && stateKeys.has(t.target)) {
            if (outgoing.has(t.source)) {
                outgoing.get(t.source)!.push(t.target);
            }
            if (incoming.has(t.target)) {
                incoming.get(t.target)!.push(t.source);
            }
        }
    });

    const initialStates = stateUsages.filter((s: any) => {
        const typeLower = (s.type || '').toLowerCase();
        return typeLower.includes('initial') && !typeLower.includes('state');
    });
    const finalStates = stateUsages.filter((s: any) => {
        const typeLower = (s.type || '').toLowerCase();
        return typeLower.includes('final') && !typeLower.includes('state');
    });

    const levels = new Map<string, number>();
    const visited = new Set<string>();

    const roots = stateUsages.filter((s: any) => {
        const key = getStateKey(s);
        const inc = incoming.get(key) || [];
        return inc.length === 0 || initialStates.includes(s);
    });

    let queue: { state: any; level: number }[] = roots.map((s: any) => ({ state: s, level: 0 }));
    if (queue.length === 0 && stateUsages.length > 0) {
        queue = [{ state: stateUsages[0], level: 0 }];
    }

    while (queue.length > 0) {
        const item = queue.shift()!;
        const { state, level } = item;
        const key = getStateKey(state);

        if (visited.has(key)) continue;
        visited.add(key);
        levels.set(key, level);

        const targets = outgoing.get(key) || [];
        targets.forEach((targetKey: string) => {
            const targetState = stateUsages.find((s: any) => getStateKey(s) === targetKey);
            if (targetState && !visited.has(targetKey)) {
                queue.push({ state: targetState, level: level + 1 });
            }
        });
    }

    stateUsages.forEach((s: any) => {
        const key = getStateKey(s);
        if (!visited.has(key)) {
            levels.set(key, Math.max(...Array.from(levels.values()), 0) + 1);
        }
    });

    const statesByLevel = new Map<number, any[]>();
    stateUsages.forEach((s: any) => {
        const key = getStateKey(s);
        const level = levels.get(key) || 0;
        if (!statesByLevel.has(level)) {
            statesByLevel.set(level, []);
        }
        statesByLevel.get(level)!.push(s);
    });

    const statePositions = new Map<string, { x: number; y: number; state: any }>();

    if (stateLayoutOrientation === 'force') {
        const nodes = stateUsages.map((s: any) => ({
            id: getStateKey(s),
            state: s,
            x: marginLeft + Math.random() * (width - marginLeft * 2 - stateWidth),
            y: marginTop + Math.random() * (height - marginTop * 2 - stateHeight)
        }));

        const nodeMap = new Map<string, any>();
        nodes.forEach((n: any) => nodeMap.set(n.id, n));

        const links: { source: any; target: any }[] = [];
        machineTransitions.forEach((t: any) => {
            const sourceKey = t.sourceName || t.source;
            const targetKey = t.targetName || t.target;
            if (nodeMap.has(sourceKey) && nodeMap.has(targetKey) && sourceKey !== targetKey) {
                links.push({
                    source: nodeMap.get(sourceKey),
                    target: nodeMap.get(targetKey)
                });
            }
        });

        const simulation = d3.forceSimulation(nodes)
            .force('center', d3.forceCenter(width / 2 - stateWidth / 2, height / 2 - stateHeight / 2))
            .force('charge', d3.forceManyBody().strength(-800))
            .force('link', d3.forceLink(links).distance(stateWidth + horizontalSpacing).strength(0.5))
            .force('collide', d3.forceCollide().radius(stateWidth * 0.8))
            .force('x', d3.forceX(width / 2 - stateWidth / 2).strength(0.05))
            .force('y', d3.forceY(height / 2 - stateHeight / 2).strength(0.05));

        simulation.stop();
        for (let i = 0; i < 300; ++i) simulation.tick();

        nodes.forEach((n: any) => {
            statePositions.set(n.id, {
                x: Math.max(marginLeft, Math.min(width - stateWidth - marginLeft, n.x)),
                y: Math.max(marginTop, Math.min(height - stateHeight - marginTop, n.y)),
                state: n.state
            });
        });
    } else if (stateLayoutOrientation === 'horizontal') {
        statesByLevel.forEach((statesInLevel, level) => {
            const levelX = marginLeft + level * (stateWidth + horizontalSpacing);
            const compactSpacing = 20;
            const totalHeight = statesInLevel.length * stateHeight + (statesInLevel.length - 1) * compactSpacing;
            const startY = marginTop + Math.max(0, (height - totalHeight - marginTop * 2) / 3);

            statesInLevel.forEach((state: any, index: number) => {
                const key = getStateKey(state);
                statePositions.set(key, {
                    x: levelX,
                    y: startY + index * (stateHeight + compactSpacing),
                    state: state
                });
            });
        });
    } else {
        statesByLevel.forEach((statesInLevel, level) => {
            const levelY = marginTop + level * (stateHeight + verticalSpacing);
            const compactSpacing = 30;
            const totalWidth = statesInLevel.length * stateWidth + (statesInLevel.length - 1) * compactSpacing;
            const startX = marginLeft + Math.max(0, (width - totalWidth - marginLeft * 2) / 3);

            statesInLevel.forEach((state: any, index: number) => {
                const key = getStateKey(state);
                statePositions.set(key, {
                    x: startX + index * (stateWidth + compactSpacing),
                    y: levelY,
                    state: state
                });
            });
        });
    }

    const defs = svg.select('defs').empty() ? svg.append('defs') : svg.select('defs');
    defs.selectAll('#state-arrowhead').remove();

    defs.append('marker')
        .attr('id', 'state-arrowhead')
        .attr('viewBox', '0 -5 10 10')
        .attr('refX', 10)
        .attr('refY', 0)
        .attr('markerWidth', 8)
        .attr('markerHeight', 8)
        .attr('orient', 'auto')
        .append('path')
        .attr('d', 'M0,-4L10,0L0,4')
        .style('fill', 'var(--vscode-charts-purple)');

    const transitionGroup = g.append('g').attr('class', 'state-transitions');
    const stateGroup = g.append('g').attr('class', 'state-nodes');

    if (stateMachineNames.length > 0) {
        const titleText = stateMachineNames.length === 1
            ? 'State Machine: ' + stateMachineNames[0]
            : 'State Machines: ' + stateMachineNames.join(', ');

        g.append('text')
            .attr('x', marginLeft)
            .attr('y', 30)
            .attr('class', 'state-machine-title')
            .style('font-size', '16px')
            .style('font-weight', 'bold')
            .style('fill', 'var(--vscode-editor-foreground)')
            .style('opacity', '0.9')
            .text(titleText);
    }

    function calculateEdgePath(sourceKey: string, targetKey: string, transitionIndex = 0, totalTransitions = 1): { path: string; labelX: number; labelY: number } | null {
        const sourcePos = statePositions.get(sourceKey);
        const targetPos = statePositions.get(targetKey);

        if (!sourcePos || !targetPos) return null;

        const sx = sourcePos.x;
        const sy = sourcePos.y;
        const tx = targetPos.x;
        const ty = targetPos.y;

        if (sourceKey === targetKey) {
            const loopSize = 30;
            return {
                path: 'M ' + (sx + stateWidth) + ' ' + (sy + stateHeight/2) +
                       ' C ' + (sx + stateWidth + loopSize) + ' ' + (sy + stateHeight/2 - loopSize) + ',' +
                         ' ' + (sx + stateWidth + loopSize) + ' ' + (sy + stateHeight/2 + loopSize) + ',' +
                         ' ' + (sx + stateWidth) + ' ' + (sy + stateHeight/2 + 5),
                labelX: sx + stateWidth + loopSize + 5,
                labelY: sy + stateHeight/2
            };
        }

        let startX: number, startY: number, endX: number, endY: number;
        const dx = tx - sx;
        const dy = ty - sy;

        const offset = (transitionIndex - (totalTransitions - 1) / 2) * 15;

        if (Math.abs(dx) > Math.abs(dy)) {
            if (dx > 0) {
                startX = sx + stateWidth;
                startY = sy + stateHeight / 2 + offset;
                endX = tx;
                endY = ty + stateHeight / 2 + offset;
            } else {
                startX = sx;
                startY = sy + stateHeight / 2 + offset;
                endX = tx + stateWidth;
                endY = ty + stateHeight / 2 + offset;
            }
        } else {
            if (dy > 0) {
                startX = sx + stateWidth / 2 + offset;
                startY = sy + stateHeight;
                endX = tx + stateWidth / 2 + offset;
                endY = ty;
            } else {
                startX = sx + stateWidth / 2 + offset;
                startY = sy;
                endX = tx + stateWidth / 2 + offset;
                endY = ty + stateHeight;
            }
        }

        const midX = (startX + endX) / 2;
        const midY = (startY + endY) / 2;

        const curveOffset = offset * 0.5;
        const controlX = midX + curveOffset;
        const controlY = midY + curveOffset;

        return {
            path: 'M ' + startX + ' ' + startY + ' Q ' + controlX + ' ' + controlY + ' ' + endX + ' ' + endY,
            labelX: controlX,
            labelY: controlY - 8
        };
    }

    function drawTransitions() {
        transitionGroup.selectAll('*').remove();

        const transitionPairs = new Map<string, any[]>();
        machineTransitions.forEach((t: any) => {
            const pairKey = t.source + '->' + t.target;
            if (!transitionPairs.has(pairKey)) {
                transitionPairs.set(pairKey, []);
            }
            transitionPairs.get(pairKey)!.push(t);
        });

        transitionPairs.forEach((transitionsForPair) => {
            transitionsForPair.forEach((transition: any, index: number) => {
                const edgeData = calculateEdgePath(
                    transition.source,
                    transition.target,
                    index,
                    transitionsForPair.length
                );

                if (!edgeData) return;

                transitionGroup.append('path')
                    .attr('d', edgeData.path)
                    .attr('class', 'transition-path')
                    .style('fill', 'none')
                    .style('stroke', 'var(--vscode-charts-purple)')
                    .style('stroke-width', '2px')
                    .style('marker-end', 'url(#state-arrowhead)');

                if (transition.label) {
                    const labelText = transition.label.length > 15
                        ? transition.label.substring(0, 12) + '...'
                        : transition.label;

                    transitionGroup.append('rect')
                        .attr('x', edgeData.labelX - 25)
                        .attr('y', edgeData.labelY - 10)
                        .attr('width', 50)
                        .attr('height', 14)
                        .attr('rx', 3)
                        .style('fill', 'var(--vscode-editor-background)')
                        .style('opacity', 0.9);

                    transitionGroup.append('text')
                        .attr('x', edgeData.labelX)
                        .attr('y', edgeData.labelY)
                        .attr('text-anchor', 'middle')
                        .attr('dominant-baseline', 'middle')
                        .text(labelText)
                        .style('font-size', '10px')
                        .style('fill', 'var(--vscode-charts-purple)')
                        .style('font-weight', '500');
                }
            });
        });
    }

    drawTransitions();

    statePositions.forEach((pos, stateKey) => {
        const state = pos.state;
        const isInitial = initialStates.includes(state);
        const isFinal = finalStates.includes(state);

        const stateElement = stateGroup.append('g')
            .attr('class', 'state-node')
            .attr('data-state-key', stateKey)
            .attr('transform', 'translate(' + pos.x + ', ' + pos.y + ')')
            .style('cursor', 'grab');

        const drag = d3.drag()
            .on('start', function() {
                d3.select(this).raise().style('cursor', 'grabbing');
            })
            .on('drag', function(event: any) {
                const newX = pos.x + event.dx;
                const newY = pos.y + event.dy;
                pos.x = newX;
                pos.y = newY;

                d3.select(this).attr('transform', 'translate(' + newX + ', ' + newY + ')');

                drawTransitions();
            })
            .on('end', function() {
                d3.select(this).style('cursor', 'grab');
            });

        stateElement.call(drag);

        if (isInitial) {
            stateElement.append('circle')
                .attr('cx', stateWidth / 2)
                .attr('cy', stateHeight / 2)
                .attr('r', 15)
                .style('fill', 'var(--vscode-charts-green)')
                .style('stroke', 'var(--vscode-panel-border)')
                .style('stroke-width', '2px');

            stateElement.append('text')
                .attr('x', stateWidth / 2)
                .attr('y', stateHeight / 2 + 30)
                .attr('text-anchor', 'middle')
                .text(state.name)
                .style('font-size', '11px')
                .style('fill', 'var(--vscode-editor-foreground)');

        } else if (isFinal) {
            stateElement.append('circle')
                .attr('cx', stateWidth / 2)
                .attr('cy', stateHeight / 2)
                .attr('r', 18)
                .style('fill', 'none')
                .style('stroke', 'var(--vscode-charts-red)')
                .style('stroke-width', '2px');
            stateElement.append('circle')
                .attr('cx', stateWidth / 2)
                .attr('cy', stateHeight / 2)
                .attr('r', 12)
                .style('fill', 'var(--vscode-charts-red)');

            stateElement.append('text')
                .attr('x', stateWidth / 2)
                .attr('y', stateHeight / 2 + 30)
                .attr('text-anchor', 'middle')
                .text(state.name)
                .style('font-size', '11px')
                .style('fill', 'var(--vscode-editor-foreground)');

        } else {
            const gradient = defs.append('linearGradient')
                .attr('id', 'state-gradient-' + stateKey.replace(/[^a-zA-Z0-9]/g, '_'))
                .attr('x1', '0%').attr('y1', '0%')
                .attr('x2', '0%').attr('y2', '100%');
            gradient.append('stop')
                .attr('offset', '0%')
                .style('stop-color', 'var(--vscode-editor-background)');
            gradient.append('stop')
                .attr('offset', '100%')
                .style('stop-color', 'var(--vscode-editorWidget-background)');

            stateElement.append('rect')
                .attr('width', stateWidth)
                .attr('height', stateHeight)
                .attr('rx', 8)
                .attr('ry', 8)
                .style('fill', 'url(#state-gradient-' + stateKey.replace(/[^a-zA-Z0-9]/g, '_') + ')')
                .style('stroke', 'var(--vscode-charts-blue)')
                .style('stroke-width', '2px')
                .style('filter', 'drop-shadow(2px 2px 3px rgba(0,0,0,0.2))');

            const displayName = state.name.length > 18
                ? state.name.substring(0, 15) + '...'
                : state.name;

            stateElement.append('text')
                .attr('class', 'node-name-text')
                .attr('data-element-name', state.name)
                .attr('x', stateWidth / 2)
                .attr('y', stateHeight / 2 + 4)
                .attr('text-anchor', 'middle')
                .text(displayName)
                .style('font-size', '12px')
                .style('font-weight', '600')
                .style('fill', 'var(--vscode-editor-foreground)')
                .style('pointer-events', 'none');

            stateElement.style('cursor', 'pointer');
            stateElement.on('click', function(event: any) {
                event.stopPropagation();
                postJumpToElement(postMessage, { name: state.name, id: state.id });
            })
            .on('dblclick', function(event: any) {
                event.stopPropagation();
                onStartInlineEdit(d3.select(this), state.name, pos.x, pos.y, stateWidth);
            });
        }
    });
}

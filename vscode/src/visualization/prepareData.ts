/**
 * prepareDataForView - Transforms generic model data into view-specific structures.
 * Helper functions (collectAllElements, removeCircularRefs, extractNestedParts, etc.)
 * are used internally. For browser/webview context.
 */

/* eslint-disable @typescript-eslint/no-explicit-any */

/**
 * Build a tree of elements from graph (nodes + edges).
 * Used when data has graph instead of elements for views that need tree structure.
 */
export function graphToElementTree(graph: any): any[] {
    if (!graph?.nodes?.length) return [];
    const nodes = graph.nodes;
    const edges = graph.edges || [];
    const nodeMap = new Map<string, any>();
    nodes.forEach((n: any) => {
        nodeMap.set(n.id, {
            id: n.id,
            name: n.name,
            type: n.type || n.element_type,
            range: n.range,
            attributes: n.attributes || {},
            relationships: [] as any[],
            children: [] as any[]
        });
    });
    const getEdgeType = (e: any) => (e.type || e.rel_type || '').toLowerCase();
    edges.forEach((e: any) => {
        if (getEdgeType(e) === 'contains' && e.source && e.target) {
            const parent = nodeMap.get(e.source);
            const child = nodeMap.get(e.target);
            if (parent && child) {
                parent.children.push(child);
            }
        }
        const relTypes = ['typing', 'specializes', 'connection', 'bind', 'allocate', 'transition', 'satisfy', 'verify', 'subject'];
        if (relTypes.includes(getEdgeType(e))) {
            const src = nodeMap.get(e.source);
            if (src) {
                src.relationships.push({ source: e.source, target: e.target, type: e.type, name: e.name });
            }
        }
    });
    const targetsOfContains = new Set(edges.filter((e: any) => getEdgeType(e) === 'contains').map((e: any) => e.target));
    const roots = nodes
        .filter((n: any) => !targetsOfContains.has(n.id))
        .map((n: any) => nodeMap.get(n.id))
        .filter(Boolean);
    return roots;
}

export function prepareDataForView(data: any, view: string): any {
    if (!data) {
        return data;
    }

    const hasGraph = data.graph?.nodes;
    const elements = hasGraph ? graphToElementTree(data.graph) : (data.elements || []);
    const edgeType = (e: any) => (e.type || e.rel_type || '');
    const relationships = hasGraph
        ? (data.graph.edges || []).filter((e: any) => edgeType(e) !== 'contains').map((e: any) => ({
            source: e.source,
            target: e.target,
            type: edgeType(e),
            name: e.name
        }))
        : (data.relationships || []);

    function collectAllElements(elementList: any[], collected: any[] = [], parentElement: any = null): any[] {
        elementList.forEach((el: any) => {
            if (parentElement && !el.parent) {
                el.parent = parentElement.name;
            }
            collected.push(el);
            if (el.children && el.children.length > 0) {
                collectAllElements(el.children, collected, el);
            }
        });
        return collected;
    }

    function removeCircularRefs(obj: any): any {
        if (!obj || typeof obj !== 'object') return obj;
        if (obj.parentElement) {
            delete obj.parentElement;
        }
        if (obj.children && Array.isArray(obj.children)) {
            obj.children.forEach((child: any) => removeCircularRefs(child));
        }
        return obj;
    }

    const allElements = collectAllElements(elements);

    switch (view) {
        case 'general-view':
            return data;
        case 'interconnection-view': {
            if (data.ibd && Array.isArray(data.ibd.parts)) {
                const isLikelyInstanceRoot = (name: string): boolean => {
                    const n = String(name || '');
                    return /instance$/i.test(n) || /inst$/i.test(n);
                };
                const ibd = data.ibd as {
                    parts: any[];
                    ports?: any[];
                    connectors?: any[];
                    rootCandidates?: string[];
                    defaultRoot?: string;
                    rootViews?: Record<string, { parts?: any[]; ports?: any[]; connectors?: any[] }>;
                };
                const ibdParts = Array.isArray(ibd.parts) ? ibd.parts : [];
                const ibdPorts = Array.isArray(ibd.ports) ? ibd.ports : [];
                const ibdConnectors = Array.isArray(ibd.connectors) ? ibd.connectors : [];
                const ibdRootCandidates = Array.isArray(ibd.rootCandidates) ? ibd.rootCandidates : [];
                const rootViews = (ibd.rootViews && typeof ibd.rootViews === 'object') ? ibd.rootViews : {};
                const availableRootsRaw = ibdRootCandidates.filter((name) => rootViews[name]);
                const hasAnyLikelyInstance = availableRootsRaw.some((name) => isLikelyInstanceRoot(name));
                const availableRoots = hasAnyLikelyInstance
                    ? availableRootsRaw
                        .filter((name) => isLikelyInstanceRoot(name))
                        .slice()
                        .sort((a, b) => a.localeCompare(b))
                    : availableRootsRaw;
                const explicitSelection = (typeof data.selectedIbdRoot === 'string' && data.selectedIbdRoot.trim().length > 0)
                    ? data.selectedIbdRoot
                    : null;
                const selectedRoot = explicitSelection && rootViews[explicitSelection]
                    ? explicitSelection
                    : (ibd.defaultRoot && availableRoots.includes(ibd.defaultRoot) && rootViews[ibd.defaultRoot]
                        ? ibd.defaultRoot
                        : (availableRoots[0] || null));
                const selectedRootView = selectedRoot ? rootViews[selectedRoot] : null;
                const selectedParts = Array.isArray(selectedRootView?.parts) ? selectedRootView.parts : ibdParts;
                const selectedPorts = Array.isArray(selectedRootView?.ports) ? selectedRootView.ports : ibdPorts;
                const selectedConnectors = Array.isArray(selectedRootView?.connectors) ? selectedRootView.connectors : ibdConnectors;
                const ibdRootSummaries = availableRoots.map((name) => {
                    const rootView = rootViews[name] || {};
                    return {
                        name,
                        partCount: Array.isArray(rootView.parts) ? rootView.parts.length : 0,
                        portCount: Array.isArray(rootView.ports) ? rootView.ports.length : 0,
                        connectorCount: Array.isArray(rootView.connectors) ? rootView.connectors.length : 0,
                    };
                });
                return {
                    ...data,
                    elements: selectedParts,
                    parts: selectedParts,
                    ports: selectedPorts,
                    connectors: selectedConnectors,
                    ibdRootCandidates: availableRoots,
                    ibdRootSummaries,
                    selectedIbdRoot: selectedRoot,
                };
            }

            const serverScene = data.diagramInterconnection?.scene?.interconnectionView;
            if (serverScene && typeof serverScene === 'object') {
                const rootCandidates = Array.isArray(serverScene.rootCandidates) ? serverScene.rootCandidates : [];
                const explicitSelection = (typeof data.selectedIbdRoot === 'string' && data.selectedIbdRoot.trim().length > 0)
                    ? data.selectedIbdRoot
                    : null;
                const selectedRoot = explicitSelection && serverScene.roots?.[explicitSelection]
                    ? explicitSelection
                    : (serverScene.selectedRoot && serverScene.roots?.[serverScene.selectedRoot]
                        ? serverScene.selectedRoot
                        : (serverScene.defaultRoot && serverScene.roots?.[serverScene.defaultRoot]
                            ? serverScene.defaultRoot
                            : (rootCandidates.find((name: string) => serverScene.roots?.[name]) || null)));
                const selectedScene = selectedRoot ? serverScene.roots?.[selectedRoot] : null;
                return {
                    ...data,
                    parts: (selectedScene?.parts || []).map((part: any) => ({
                        id: part.id,
                        name: part.name,
                        qualifiedName: part.qualifiedName,
                        containerId: part.containerId,
                        type: part.type,
                        attributes: part.attributes || {},
                    })),
                    ports: (selectedScene?.ports || []).map((port: any) => ({
                        id: port.id,
                        name: port.name,
                        parentId: port.parentId,
                        direction: port.direction,
                        portType: port.portType,
                        portSide: port.portSide,
                    })),
                    connectors: (selectedScene?.connectors || []).map((connector: any) => ({
                        id: connector.id,
                        source: connector.source,
                        target: connector.target,
                        sourceId: connector.sourceId,
                        targetId: connector.targetId,
                        type: connector.type,
                    })),
                    ibdRootCandidates: rootCandidates,
                    ibdRootSummaries: rootCandidates.map((name: string) => ({
                        name,
                        partCount: serverScene.roots?.[name]?.parts?.length || 0,
                        portCount: serverScene.roots?.[name]?.ports?.length || 0,
                        connectorCount: serverScene.roots?.[name]?.connectors?.length || 0,
                    })),
                    selectedIbdRoot: selectedRoot,
                };
            }

            // No interconnection scene payload available from the server.
            return {
                ...data,
                elements: [],
                parts: [],
                ports: [],
                connectors: [],
                ibdRootCandidates: [],
                selectedIbdRoot: null,
            };
        }

        case 'action-flow-view': {
            if (data.activityDiagrams && data.activityDiagrams.length > 0) {
                return {
                    ...data,
                    diagrams: data.activityDiagrams.map((diagram: any) => {
                        const decisionsAsNodes = (diagram.decisions || []).map((d: any) => ({
                            ...d,
                            id: d.id || d.name,
                            type: 'decision',
                            kind: 'decision'
                        }));

                        const stateNodes = (diagram.states || [])
                            .map((state: any, idx: number) => ({
                                ...state,
                                id: state.id || state.name || `state_${idx + 1}`,
                                name: state.name || state.id || `State ${idx + 1}`,
                                type: state.type || state.stateType || 'state',
                                kind: state.type || state.stateType || 'state'
                            }))
                            .filter((state: any) => {
                                const kind = String(state.kind || state.type || '').toLowerCase();
                                return ['initial', 'final', 'decision', 'merge', 'fork', 'join'].some((allowed) => kind.includes(allowed));
                            });

                        const allNodes = [
                            ...(diagram.actions || []).map((a: any) => ({
                                ...a,
                                id: a.id || a.name,
                                inputs: Array.isArray(a.inputs) ? a.inputs : [],
                                outputs: Array.isArray(a.outputs) ? a.outputs : [],
                                parent: (a.parent === diagram.name) ? undefined : a.parent
                            })),
                            ...decisionsAsNodes,
                            ...stateNodes
                        ];

                        const allowedKinds = new Set(['action', 'perform', 'decision', 'merge', 'fork', 'join', 'initial', 'final']);
                        const nodes = allNodes.filter((node: any) => {
                            const kind = String(node.kind || node.type || 'action').toLowerCase();
                            return allowedKinds.has(kind);
                        });

                        const nodeIds = new Set(nodes.map((node: any) => node.id || node.name));
                        const flows = (diagram.flows || []).map((flow: any, idx: number) => ({
                            ...flow,
                            id: flow.id || `${diagram.name}::flow::${idx + 1}`,
                            flowKind: flow.flowKind || flow.type || 'control'
                        }));
                        const incomingFlowCount = new Map<string, number>();
                        const outgoingFlowCount = new Map<string, number>();

                        flows.forEach((f: any) => {
                            if (f.from && nodeIds.has(f.from)) {
                                outgoingFlowCount.set(f.from, (outgoingFlowCount.get(f.from) || 0) + 1);
                            }
                            if (f.to && nodeIds.has(f.to)) {
                                incomingFlowCount.set(f.to, (incomingFlowCount.get(f.to) || 0) + 1);
                            }
                        });

                        const cleanFlows = flows.filter((f: any) =>
                            f.from !== f.to &&
                            nodeIds.has(f.from) &&
                            nodeIds.has(f.to)
                        );

                        return {
                            name: diagram.name,
                            nodes: nodes.map((node: any) => {
                                const nodeId = node.id || node.name;
                                const incoming = incomingFlowCount.get(nodeId) || 0;
                                const outgoing = outgoingFlowCount.get(nodeId) || 0;
                                const hasGuards = cleanFlows.some((flow: any) => flow.from === nodeId && (flow.guard || flow.condition));
                                let normalizedKind = String(node.kind || node.type || 'action').toLowerCase();
                                if (normalizedKind === 'action' && outgoing > 1 && hasGuards) normalizedKind = 'decision';
                                else if (normalizedKind === 'action' && outgoing > 1) normalizedKind = 'fork';
                                else if (normalizedKind === 'action' && incoming > 1) normalizedKind = 'merge';

                                return {
                                    ...node,
                                    id: nodeId,
                                    name: node.name || nodeId || 'Action',
                                    kind: normalizedKind,
                                };
                            }),
                            flows: cleanFlows,
                            interface: {
                                inputs: Array.isArray(diagram.interface?.inputs) ? diagram.interface.inputs : [],
                                outputs: Array.isArray(diagram.interface?.outputs) ? diagram.interface.outputs : [],
                            },
                            hasBehavioralFlow: cleanFlows.length > 0,
                        };
                    })
                };
            }

            const actionDefs = allElements.filter((el: any) => {
                if (!el.type) return false;
                const typeLower = el.type.toLowerCase();
                return typeLower === 'action' || typeLower === 'action def' || typeLower === 'action definition';
            });
            const activityActionDefs = actionDefs.filter((a: any) => a.children && a.children.length > 0);

            return {
                ...data,
                diagrams: activityActionDefs.map((actionDef: any) => {
                    const nodes = actionDef.children
                        .filter((c: any) => {
                            const type = String(c.type || '').toLowerCase();
                            return type.includes('action') || type.includes('perform') || type.includes('decision') || type.includes('merge') || type.includes('fork') || type.includes('join');
                        })
                        .map((c: any) => ({
                            name: c.name,
                            type: c.type || 'action',
                            kind: String(c.type || 'action').toLowerCase().includes('perform') ? 'perform' : 'action',
                            id: c.id || c.name,
                            inputs: [],
                            outputs: [],
                        }));

                    const interfaceInputs = actionDef.children
                        .filter((c: any) => String(c.type || '').toLowerCase() === 'in out parameter' && String(c.attributes?.direction || '').toLowerCase() === 'in')
                        .map((c: any) => c.name)
                        .filter(Boolean);
                    const interfaceOutputs = actionDef.children
                        .filter((c: any) => String(c.type || '').toLowerCase() === 'in out parameter' && String(c.attributes?.direction || '').toLowerCase() === 'out')
                        .map((c: any) => c.name)
                        .filter(Boolean);

                    return {
                        name: actionDef.name,
                        nodes,
                        flows: [],
                        interface: {
                            inputs: interfaceInputs,
                            outputs: interfaceOutputs,
                        },
                        hasBehavioralFlow: false,
                    };
                })
            };
        }

        case 'state-transition-view': {
            const typeLower = (value: any) => String(value || '').toLowerCase();
            const normalizeKey = (value: any) => String(value || '').replace(/::/g, '.').trim();
            const lastSegment = (value: any) => {
                const normalized = normalizeKey(value);
                if (!normalized) return '';
                const parts = normalized.split('.');
                return parts[parts.length - 1] || '';
            };
            const asStringList = (value: any): string[] => {
                if (Array.isArray(value)) {
                    return value.flatMap((item) => asStringList(item));
                }
                if (value == null) return [];
                if (typeof value === 'object') {
                    return Object.values(value).flatMap((item) => asStringList(item));
                }
                const normalized = normalizeKey(value);
                return normalized ? [normalized] : [];
            };
            const extractTransitionEndpointAliases = (el: any, direction: 'source' | 'target'): string[] => {
                const keys = direction === 'source'
                    ? ['source', 'src', 'from', 'first', 'state', 'start']
                    : ['target', 'tgt', 'to', 'then', 'next', 'destination'];
                const directCandidates = keys.flatMap((key) => asStringList(el?.[key]));
                const attributeCandidates = keys.flatMap((key) => asStringList(el?.attributes?.[key]));
                const nestedCandidates = keys.flatMap((key) => asStringList(el?.value?.[key]));
                return Array.from(new Set([...directCandidates, ...attributeCandidates, ...nestedCandidates]));
            };
            const likelySyntheticTransitionName = (value: any) => /^transition_\d+$/i.test(String(value || '').trim());
            const isTransitionElement = (el: any) => typeLower(el?.type).includes('transition');
            const isStateElement = (el: any) => {
                const t = typeLower(el?.type);
                return (t.includes('state') || t.includes('exhibit')) && !isTransitionElement(el);
            };
            const isStateDefinition = (el: any) => {
                const t = typeLower(el?.type);
                return t.includes('state') && (t.includes('def') || t.includes('definition'));
            };
            const hasStateChildren = (el: any) => Array.isArray(el?.children) && el.children.some((child: any) => isStateElement(child));
            const hasTransitionChildren = (el: any) => Array.isArray(el?.children) && el.children.some((child: any) => isTransitionElement(child));
            const isLikelyStateMachine = (el: any) => {
                if (!isStateElement(el)) return false;
                const t = typeLower(el?.type);
                const n = typeLower(el?.name);
                return t.includes('exhibit')
                    || n.endsWith('states')
                    || n.includes('statemachine')
                    || hasTransitionChildren(el)
                    || hasStateChildren(el);
            };

            const machineRoots: any[] = [];
            function findMachineRoots(elementList: any[], insideMachine = false): void {
                elementList.forEach((el: any) => {
                    const startsMachine = isLikelyStateMachine(el) && !insideMachine;
                    if (startsMachine) {
                        machineRoots.push(el);
                    }
                    if (Array.isArray(el?.children) && el.children.length > 0) {
                        findMachineRoots(el.children, insideMachine || startsMachine);
                    }
                });
            }
            findMachineRoots(elements);

            function makeStableId(prefix: string, el: any, path: string[]): string {
                const explicitId = String(el?.id || '').trim();
                if (explicitId) return explicitId;
                const explicitQName = String(el?.qualifiedName || '').trim();
                if (explicitQName) return explicitQName;
                const fallbackName = String(el?.name || prefix).trim() || prefix;
                return [prefix, ...path, fallbackName].join('::');
            }

            function buildMachine(root: any, machineIndex: number) {
                const machineId = makeStableId(`state-machine-${machineIndex + 1}`, root, []);
                const normalizedStates: any[] = [];
                const stateIdByAlias = new Map<string, string>();
                const transitionElements: any[] = [];

                function registerStateAlias(state: any, normalizedId: string): void {
                    [
                        state?.id,
                        state?.name,
                        state?.qualifiedName,
                        state?.parent,
                    ].forEach((candidate) => {
                        const key = normalizeKey(candidate);
                        if (key) stateIdByAlias.set(key, normalizedId);
                    });
                }

                function collectTransitionElements(elementList: any[]): void {
                    elementList.forEach((el: any) => {
                        if (isTransitionElement(el)) {
                            transitionElements.push(el);
                        }
                        if (Array.isArray(el?.children) && el.children.length > 0) {
                            collectTransitionElements(el.children);
                        }
                    });
                }

                function collectStates(elementList: any[], parentStateId: string | null, path: string[], depth: number): void {
                    elementList.forEach((el: any) => {
                        if (!isStateElement(el) || isTransitionElement(el)) {
                            return;
                        }

                        const stateId = makeStableId(`state-${normalizedStates.length + 1}`, el, path);
                        const nestedChildren = Array.isArray(el?.children) ? el.children.filter((child: any) => isStateElement(child)) : [];
                        const kind = (() => {
                            const t = typeLower(el?.type);
                            if (t.includes('initial')) return 'initial';
                            if (t.includes('final')) return 'final';
                            if (nestedChildren.length > 0 || hasTransitionChildren(el)) return 'composite';
                            return 'state';
                        })();

                        const normalizedState = {
                            id: stateId,
                            name: String(el?.name || `State ${normalizedStates.length + 1}`),
                            qualifiedName: el?.qualifiedName || el?.id || el?.name || stateId,
                            type: el?.type || 'state',
                            kind,
                            parentId: parentStateId,
                            childIds: [] as string[],
                            isDefinition: isStateDefinition(el),
                            depth,
                            element: removeCircularRefs(el),
                        };

                        normalizedStates.push(normalizedState);
                        registerStateAlias(el, stateId);

                        if (nestedChildren.length > 0) {
                            collectStates(nestedChildren, stateId, [...path, normalizedState.name], depth + 1);
                        }
                    });
                }

                const rootStateChildren = Array.isArray(root?.children)
                    ? root.children.filter((child: any) => isStateElement(child))
                    : [];
                if (Array.isArray(root?.children) && root.children.length > 0) {
                    collectTransitionElements(root.children);
                }
                collectStates(rootStateChildren, null, [String(root?.name || `StateMachine${machineIndex + 1}`)], 0);

                const statesById = new Map<string, any>();
                normalizedStates.forEach((state: any) => statesById.set(state.id, state));
                normalizedStates.forEach((state: any) => {
                    if (state.parentId && statesById.has(state.parentId)) {
                        statesById.get(state.parentId).childIds.push(state.id);
                    }
                });

                const transitionRecords = transitionElements.map((transitionElement: any, transitionIndex: number) => {
                    const sourceAliases = extractTransitionEndpointAliases(transitionElement, 'source');
                    const targetAliases = extractTransitionEndpointAliases(transitionElement, 'target');
                    return {
                        element: transitionElement,
                        index: transitionIndex,
                        matched: false,
                        name: String(transitionElement?.name || '').trim(),
                        label: String(
                            transitionElement?.label
                            || transitionElement?.guard
                            || transitionElement?.attributes?.label
                            || transitionElement?.attributes?.guard
                            || '',
                        ).trim(),
                        sourceAliases,
                        targetAliases,
                    };
                });

                function resolveStateIdFromRelationshipEndpoint(value: any): string | null {
                    const aliasCandidates = Array.from(new Set([
                        normalizeKey(value),
                        lastSegment(value),
                    ].filter(Boolean)));
                    for (const alias of aliasCandidates) {
                        const match = stateIdByAlias.get(alias);
                        if (match) return match;
                    }
                    return null;
                }

                function matchTransitionRecord(rel: any, sourceId: string, targetId: string) {
                    const relName = String(rel?.name || '').trim();
                    const relSourceName = lastSegment(rel?.source) || statesById.get(sourceId)?.name || '';
                    const relTargetName = lastSegment(rel?.target) || statesById.get(targetId)?.name || '';
                    const availableRecords = transitionRecords.filter((record: any) => !record.matched);

                    const scoredRecords = availableRecords.map((record: any) => {
                        let score = 0;
                        const sourceMatches = record.sourceAliases.some((alias: string) => {
                            const resolved = resolveStateIdFromRelationshipEndpoint(alias);
                            return resolved === sourceId || lastSegment(alias) === normalizeKey(relSourceName);
                        });
                        const targetMatches = record.targetAliases.some((alias: string) => {
                            const resolved = resolveStateIdFromRelationshipEndpoint(alias);
                            return resolved === targetId || lastSegment(alias) === normalizeKey(relTargetName);
                        });
                        if (sourceMatches) score += 5;
                        if (targetMatches) score += 5;
                        if (record.name && relName && record.name === relName) score += 8;
                        if (record.name && likelySyntheticTransitionName(relName)) score += 3;
                        if (!record.sourceAliases.length && !record.targetAliases.length) {
                            score += 1;
                        }
                        return { record, score };
                    }).filter(({ score }) => score > 0);

                    scoredRecords.sort((a: any, b: any) => {
                        if (b.score !== a.score) return b.score - a.score;
                        return a.record.index - b.record.index;
                    });

                    const best = scoredRecords[0]?.record || null;
                    if (best) {
                        best.matched = true;
                    }
                    return best;
                }

                const transitions = relationships
                    .filter((rel: any) => typeLower(rel?.type).includes('transition'))
                    .map((rel: any, relIndex: number) => {
                        const sourceId = resolveStateIdFromRelationshipEndpoint(rel?.source);
                        const targetId = resolveStateIdFromRelationshipEndpoint(rel?.target);
                        if (!sourceId || !targetId) {
                            return null;
                        }
                        const transitionRecord = matchTransitionRecord(rel, sourceId, targetId);
                        const resolvedName = String(
                            (!likelySyntheticTransitionName(rel?.name) && rel?.name)
                            || transitionRecord?.name
                            || rel?.name
                            || `transition_${relIndex + 1}`,
                        ).trim();
                        const resolvedLabel = String(
                            rel?.label
                            || rel?.guard
                            || ((!likelySyntheticTransitionName(rel?.name) && rel?.name) || '')
                            || transitionRecord?.label
                            || transitionRecord?.name
                            || '',
                        ).trim();
                        return {
                            id: String(rel?.id || `${machineId}::transition::${relIndex + 1}`),
                            name: resolvedName,
                            label: resolvedLabel,
                            source: sourceId,
                            target: targetId,
                            sourceName: statesById.get(sourceId)?.name || String(rel?.source || ''),
                            targetName: statesById.get(targetId)?.name || String(rel?.target || ''),
                            selfLoop: sourceId === targetId,
                            relationship: rel,
                        };
                    })
                    .filter(Boolean);

                const hasInitialState = normalizedStates.some((state: any) => state.kind === 'initial');
                if (!hasInitialState && normalizedStates.length > 0) {
                    const incomingCount = new Map<string, number>();
                    normalizedStates.forEach((state: any) => incomingCount.set(state.id, 0));
                    transitions.forEach((transition: any) => {
                        incomingCount.set(transition.target, (incomingCount.get(transition.target) || 0) + 1);
                    });
                    const preferredInitial =
                        normalizedStates.find((state: any) => /^idle$/i.test(state.name)) ||
                        normalizedStates.find((state: any) => /^manual$/i.test(state.name)) ||
                        normalizedStates
                            .filter((state: any) => state.kind !== 'final')
                            .sort((a: any, b: any) => {
                                const incomingDelta = (incomingCount.get(a.id) || 0) - (incomingCount.get(b.id) || 0);
                                if (incomingDelta !== 0) return incomingDelta;
                                return a.name.localeCompare(b.name);
                            })[0];

                    if (preferredInitial) {
                        const entryId = `${machineId}::entry`;
                        normalizedStates.unshift({
                            id: entryId,
                            name: 'entry',
                            qualifiedName: entryId,
                            type: 'initial state',
                            kind: 'initial',
                            parentId: null,
                            childIds: [],
                            isDefinition: false,
                            depth: 0,
                            element: { name: 'entry', type: 'initial state', id: entryId },
                        });
                        transitions.unshift({
                            id: `${machineId}::entry-transition`,
                            name: 'entry',
                            label: 'entry',
                            source: entryId,
                            target: preferredInitial.id,
                            sourceName: 'entry',
                            targetName: preferredInitial.name,
                            selfLoop: false,
                            relationship: null,
                        });
                    }
                }

                return {
                    id: machineId,
                    name: String(root?.name || `State Machine ${machineIndex + 1}`),
                    container: removeCircularRefs(root),
                    states: normalizedStates,
                    transitions,
                };
            }

            let stateMachines = machineRoots.map((root: any, index: number) => buildMachine(root, index));

            if (stateMachines.length === 0) {
                const fallbackStates = allElements.filter((el: any) => isStateElement(el) && !isStateDefinition(el));
                if (fallbackStates.length > 0) {
                    const fallbackRoot = {
                        id: 'fallback-state-machine',
                        name: 'State Machine',
                        type: 'state machine',
                        children: fallbackStates,
                    };
                    stateMachines = [buildMachine(fallbackRoot, 0)];
                }
            }

            const flatStates = stateMachines.flatMap((machine: any) => machine.states);
            const flatTransitions = stateMachines.flatMap((machine: any) => machine.transitions);

            return {
                ...data,
                stateMachines,
                states: flatStates,
                transitions: flatTransitions,
            };
        }

        default:
            return data;
    }
}

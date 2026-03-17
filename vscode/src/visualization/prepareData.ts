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
        const relTypes = ['typing', 'specializes', 'connection', 'bind', 'allocate', 'transition', 'satisfy', 'verify'];
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

    const normalizeQualifiedPath = (value: string | null | undefined): string =>
        (value || '').replace(/::/g, '.').trim();

    switch (view) {
        case 'general-view':
            return data;
        case 'interconnection-view': {
            if (data.ibd && Array.isArray(data.ibd.parts)) {
                const ibd = data.ibd as { parts: any[]; ports?: any[]; connectors?: any[]; rootCandidates?: string[]; defaultRoot?: string };
                const ibdParts = Array.isArray(ibd.parts) ? ibd.parts : [];
                const ibdPorts = Array.isArray(ibd.ports) ? ibd.ports : [];
                const ibdConnectors = Array.isArray(ibd.connectors) ? ibd.connectors : [];
                const ibdRootCandidates = Array.isArray(ibd.rootCandidates) ? ibd.rootCandidates : [];
                const endpointBelongsToFocusedParts = (focusedParts: any[], endpointId: string | null | undefined): boolean => {
                    const normalized = normalizeQualifiedPath(endpointId);
                    if (!normalized) return false;
                    return focusedParts.some((p: any) => {
                        const q = normalizeQualifiedPath(p.qualifiedName || p.name);
                        const simpleName = normalizeQualifiedPath(p.name);
                        return normalized === q
                            || normalized.startsWith(q + '.')
                            || normalized === simpleName
                            || normalized.startsWith(simpleName + '.');
                    });
                };
                const resolveRootPart = (requestedRoot: string | null | undefined) => {
                    const requestedRootNormalized = normalizeQualifiedPath(requestedRoot);
                    if (!requestedRootNormalized) return null;
                    return ibdParts.find((p: any) => {
                        const candidates = [
                            p.name,
                            p.id,
                            p.qualifiedName,
                            normalizeQualifiedPath(p.qualifiedName),
                        ].filter(Boolean);
                        return candidates.some((candidate: any) => normalizeQualifiedPath(String(candidate)) === requestedRootNormalized);
                    }) ?? null;
                };
                const summarizeRoot = (rootName: string) => {
                    const rootPart = resolveRootPart(rootName);
                    const selectedRoot = rootPart?.name ?? rootName;
                    const rootPrefix = normalizeQualifiedPath(rootPart ? (rootPart.qualifiedName || rootPart.name) : selectedRoot);
                    const focusedParts = rootPrefix ? ibdParts.filter((p: any) => {
                        const q = normalizeQualifiedPath(p.qualifiedName || p.name);
                        return q === rootPrefix || q.startsWith(rootPrefix + '.');
                    }) : [];
                    const partIds = new Set(focusedParts.map((p: any) => normalizeQualifiedPath(p.qualifiedName || p.name)));
                    const partNames = new Set(focusedParts.map((p: any) => normalizeQualifiedPath(p.name)));
                    const focusedPorts = ibdPorts.filter((p: any) =>
                        partIds.has(normalizeQualifiedPath(p.parentId)) || partNames.has(normalizeQualifiedPath(p.parentId))
                    );
                    const focusedConnectors = ibdConnectors.filter((c: any) => {
                        return endpointBelongsToFocusedParts(focusedParts, c.sourceId || c.source)
                            && endpointBelongsToFocusedParts(focusedParts, c.targetId || c.target);
                    });
                    return {
                        rootName: selectedRoot,
                        rootPart,
                        rootPrefix,
                        focusedParts,
                        focusedPorts,
                        focusedConnectors,
                        score: focusedConnectors.length * 100 + focusedPorts.length * 10 + focusedParts.length,
                    };
                };
                const rootSummaries = ibdRootCandidates.map((candidate) => summarizeRoot(candidate));
                const explicitlyRequestedRoot = (data.selectedIbdRoot && typeof data.selectedIbdRoot === 'string')
                    ? data.selectedIbdRoot
                    : null;
                const preferredByName = explicitlyRequestedRoot
                    ? rootSummaries.find((summary) => summary.rootName === explicitlyRequestedRoot) ?? null
                    : null;
                const defaultByName = ibd.defaultRoot
                    ? rootSummaries.find((summary) => summary.rootName === ibd.defaultRoot) ?? null
                    : null;
                const richestRoot = [...rootSummaries].sort((a, b) =>
                    b.score - a.score || a.rootName.localeCompare(b.rootName)
                )[0] ?? null;
                const chosenSummary = preferredByName ?? defaultByName ?? richestRoot ?? null;
                const focusedParts = chosenSummary?.focusedParts ?? [];
                const focusedPorts = chosenSummary?.focusedPorts ?? [];
                const focusedConnectors = chosenSummary?.focusedConnectors ?? [];
                const selectedRoot = chosenSummary?.rootName ?? null;
                return {
                    ...data,
                    elements: focusedParts,
                    parts: focusedParts,
                    ports: focusedPorts,
                    connectors: focusedConnectors,
                    ibdRootCandidates,
                    ibdRootSummaries: rootSummaries.map((summary) => ({
                        name: summary.rootName,
                        partCount: summary.focusedParts.length,
                        portCount: summary.focusedPorts.length,
                        connectorCount: summary.focusedConnectors.length,
                    })),
                    selectedIbdRoot: selectedRoot,
                };
            }

            // No backend IBD: interconnection view uses server data only (no client fallback).
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
                        const decisionsAsActions = (diagram.decisions || []).map((d: any) => ({
                            ...d,
                            id: d.id || d.name,
                            type: 'decision',
                            kind: 'decision'
                        }));

                        const allActions = [
                            ...(diagram.actions || []).map((a: any) => ({
                                ...a,
                                id: a.id || a.name,
                                parent: (a.parent === diagram.name) ? undefined : a.parent
                            })),
                            ...decisionsAsActions
                        ];

                        const actionIds = new Set(allActions.map((a: any) => a.id || a.name));
                        const flows = diagram.flows || [];
                        const flowNodeNames = new Set<string>();
                        const incomingFlowCount = new Map<string, number>();
                        const outgoingFlowCount = new Map<string, number>();

                        flows.forEach((f: any) => {
                            if (f.from) {
                                flowNodeNames.add(f.from);
                                outgoingFlowCount.set(f.from, (outgoingFlowCount.get(f.from) || 0) + 1);
                            }
                            if (f.to) {
                                flowNodeNames.add(f.to);
                                incomingFlowCount.set(f.to, (incomingFlowCount.get(f.to) || 0) + 1);
                            }
                        });

                        flowNodeNames.forEach((nodeName: string) => {
                            if (!actionIds.has(nodeName)) {
                                const incoming = incomingFlowCount.get(nodeName) || 0;
                                const outgoing = outgoingFlowCount.get(nodeName) || 0;
                                const nameLower = nodeName.toLowerCase();
                                let nodeType = 'action';
                                let nodeKind = 'action';

                                if (nameLower.includes('merge') || nameLower.includes('join') || nameLower.endsWith('check')) {
                                    nodeType = 'merge';
                                    nodeKind = 'merge';
                                } else if (nameLower.includes('fork')) {
                                    nodeType = 'fork';
                                    nodeKind = 'fork';
                                } else if (nameLower.includes('decision') || nameLower.includes('decide')) {
                                    nodeType = 'decision';
                                    nodeKind = 'decision';
                                } else if (incoming > 1) {
                                    nodeType = 'merge';
                                    nodeKind = 'merge';
                                } else if (outgoing > 1) {
                                    const hasGuards = flows.some((f: any) => f.from === nodeName && (f.guard || f.condition));
                                    if (hasGuards) {
                                        nodeType = 'decision';
                                        nodeKind = 'decision';
                                    } else {
                                        nodeType = 'fork';
                                        nodeKind = 'fork';
                                    }
                                }

                                allActions.push({
                                    name: nodeName,
                                    id: nodeName,
                                    type: nodeType,
                                    kind: nodeKind
                                });
                                actionIds.add(nodeName);
                            }
                        });

                        const cleanFlows = flows.filter((f: any) =>
                            f.from !== f.to &&
                            actionIds.has(f.from) &&
                            actionIds.has(f.to)
                        );

                        return {
                            name: diagram.name,
                            actions: allActions,
                            flows: cleanFlows,
                            decisions: diagram.decisions || [],
                            states: diagram.states || []
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
                    const childActions = actionDef.children
                        .filter((c: any) => c.type && c.type.toLowerCase().includes('action'))
                        .map((c: any) => ({
                            name: c.name,
                            type: 'action',
                            kind: 'action',
                            id: c.name
                        }));

                    const flows: any[] = [];
                    for (let i = 0; i < childActions.length - 1; i++) {
                        flows.push({ from: childActions[i].name, to: childActions[i + 1].name });
                    }

                    if (childActions.length > 0) {
                        flows.unshift({ from: 'start', to: childActions[0].name });
                        flows.push({ from: childActions[childActions.length - 1].name, to: 'done' });
                        childActions.unshift({ name: 'start', type: 'initial', kind: 'initial', id: 'start' });
                        childActions.push({ name: 'done', type: 'final', kind: 'final', id: 'done' });
                    }

                    return {
                        name: actionDef.name,
                        actions: childActions,
                        flows,
                        decisions: [],
                        states: []
                    };
                })
            };
        }

        case 'state-transition-view': {
            const stateElements = allElements.filter((el: any) => el.type && (
                el.type.includes('state') || el.type.includes('State')
            ));
            return {
                ...data,
                states: stateElements,
                transitions: relationships.filter((rel: any) =>
                    rel.type && rel.type.includes('transition')
                )
            };
        }

        case 'sequence-view': {
            if (data.sequenceDiagrams && data.sequenceDiagrams.length > 0) {
                return { ...data, sequenceDiagrams: data.sequenceDiagrams };
            }

            function collectParticipants(el: any): any[] {
                const parts: any[] = [];
                function walk(children: any[]) {
                    for (const c of children) {
                        if (!c.type) continue;
                        const t = c.type.toLowerCase();
                        if (t === 'actor' || t === 'actor usage' || t === 'actor def') {
                            if (!parts.find((p: any) => p.name === c.name)) {
                                parts.push({ name: c.name, type: 'actor' });
                            }
                        } else if (t === 'part' || t === 'part usage' || t === 'part def' ||
                            t === 'item' || t === 'item usage' || t === 'item def') {
                            if (!parts.find((p: any) => p.name === c.name)) {
                                parts.push({ name: c.name, type: c.typing || 'component' });
                            }
                        } else if (t === 'port' || t === 'port usage') {
                            if (!parts.find((p: any) => p.name === c.name)) {
                                parts.push({ name: c.name, type: 'port' });
                            }
                        }
                        if (c.children && c.children.length > 0) walk(c.children);
                    }
                }
                walk(el.children || []);
                if (parts.length === 0) parts.push({ name: 'system', type: 'system' });
                return parts;
            }

            function buildMessages(el: any, participants: any[]): any[] {
                const msgs: any[] = [];
                let occ = 1;
                function walk(children: any[]) {
                    for (const c of children) {
                        if (!c.type) continue;
                        const t = c.type.toLowerCase();
                        if (t === 'action' || t === 'action usage' || t === 'action def') {
                            const cName = (c.name || '').toLowerCase();
                            let from = participants[0]?.name || 'system';
                            let to = participants.length > 1 ? participants[1].name : (participants[0]?.name || 'system');
                            for (const p of participants) {
                                const pLower = p.name.toLowerCase();
                                if (cName.includes(pLower) || pLower.includes(cName)) {
                                    to = p.name;
                                    break;
                                }
                            }
                            const actorP = participants.find((p: any) => p.type === 'actor');
                            if (actorP) from = actorP.name;
                            msgs.push({ name: c.name, from, to, payload: c.name, occurrence: occ++ });
                            if (c.children && c.children.length > 0) walk(c.children);
                        }
                    }
                }
                walk(el.children || []);
                return msgs;
            }

            const seqCandidates = allElements.filter((el: any) => {
                if (!el.type || !el.children || el.children.length === 0) return false;
                const nameLower = (el.name || '').toLowerCase();
                const typeLower = el.type.toLowerCase();
                const hasSequenceName = /sequence|interaction|workflow|scenario|process/.test(nameLower);
                const isInteraction = typeLower.includes('interaction');
                if (!hasSequenceName && !isInteraction) return false;
                const hasParts = el.children.some((c: any) => c.type && c.type.toLowerCase().includes('part'));
                return hasParts;
            });

            const actionSeqCandidates = allElements.filter((el: any) => {
                if (!el.type || !el.children || el.children.length === 0) return false;
                const typeLower = el.type.toLowerCase();
                const isAction = typeLower === 'action def' || typeLower === 'action definition' ||
                    typeLower === 'action' || typeLower === 'action usage';
                if (!isAction) return false;
                const hasChildActions = el.children.some((c: any) => {
                    if (!c.type) return false;
                    const ct = c.type.toLowerCase();
                    return ct === 'action' || ct === 'action usage' || ct === 'action def';
                });
                return hasChildActions;
            });

            const allCandidatesMap = new Map<string, any>();
            for (const c of seqCandidates) allCandidatesMap.set(c.name, c);
            for (const c of actionSeqCandidates) {
                if (!allCandidatesMap.has(c.name)) allCandidatesMap.set(c.name, c);
            }
            const allSeqCandidates = Array.from(allCandidatesMap.values());

            if (allSeqCandidates.length > 0) {
                const synthesisedDiagrams = allSeqCandidates.map((candidate: any) => {
                    const participants = collectParticipants(candidate);
                    const messages = buildMessages(candidate, participants);
                    return { name: candidate.name, participants, messages };
                });
                return { ...data, sequenceDiagrams: synthesisedDiagrams };
            }

            return { ...data, sequenceDiagrams: [] };
        }

        default:
            return data;
    }
}

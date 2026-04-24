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

    const normalizePath = (value: any): string => String(value || '').replace(/::/g, '.').trim();
    const toPackagePath = (value: string): string => {
        const normalized = normalizePath(value);
        if (!normalized) return '';
        const segments = normalized.split('.').filter(Boolean);
        if (segments.length <= 1) return '';
        return segments.slice(0, -1).join('::');
    };
    const countWithFallback = (values: any, fallback = 0): number =>
        Array.isArray(values) ? values.length : fallback;
    const elementIdentityMeta = new Map<string, { stableId: string; qualifiedPath: string; packagePath: string }>();
    const registerElementMeta = (el: any, ancestry: string[]) => {
        const name = String(el?.name || '').trim();
        const explicitId = String(el?.id || '').trim();
        const explicitQualified = normalizePath(el?.qualifiedName || el?.attributes?.qualifiedName || '');
        const fallbackQualified = [...ancestry, name].filter(Boolean).join('.');
        const qualifiedPath = explicitQualified || fallbackQualified || explicitId || name;
        const stableId = explicitId || qualifiedPath || name;
        const packagePath = toPackagePath(qualifiedPath) || ancestry.join('::');
        [
            explicitId,
            name,
            explicitQualified,
            qualifiedPath,
        ].filter(Boolean).forEach((key) => {
            elementIdentityMeta.set(String(key), { stableId, qualifiedPath, packagePath });
        });
        if (Array.isArray(el?.children) && el.children.length > 0) {
            const nextAncestry = name ? [...ancestry, name] : ancestry;
            el.children.forEach((child: any) => registerElementMeta(child, nextAncestry));
        }
    };
    elements.forEach((el: any) => registerElementMeta(el, []));
    const lookupElementMeta = (candidate: any, fallbackName?: string) => {
        const keys = [
            candidate?.id,
            candidate?.qualifiedName,
            candidate?.attributes?.qualifiedName,
            candidate?.name,
            fallbackName,
        ].filter(Boolean);
        for (const key of keys) {
            const direct = elementIdentityMeta.get(String(key));
            if (direct) return direct;
            const normalized = normalizePath(key);
            if (normalized) {
                const normalizedMatch = elementIdentityMeta.get(normalized);
                if (normalizedMatch) return normalizedMatch;
            }
        }
        const fallbackQualified = normalizePath(candidate?.qualifiedName || candidate?.id || fallbackName || candidate?.name || '');
        return {
            stableId: String(candidate?.id || fallbackQualified || fallbackName || candidate?.name || '').trim(),
            qualifiedPath: fallbackQualified,
            packagePath: toPackagePath(fallbackQualified),
        };
    };
    const buildSelectorLabel = (name: string, packagePath: string): string =>
        packagePath ? `${name} - ${packagePath}` : name;

    switch (view) {
        case 'general-view':
            return data;
        case 'interconnection-view': {
            if (data.ibd && Array.isArray(data.ibd.parts)) {
                const isLikelyInstanceRoot = (name: string): boolean => {
                    const n = String(name || '');
                    return /instance$/i.test(n) || /inst$/i.test(n);
                };
                const typeNameFromPart = (part: any): string | null => {
                    const attrs = part?.attributes;
                    const raw = attrs?.partType || attrs?.type || attrs?.typedBy;
                    if (!raw) return null;
                    const text = String(raw).trim().replace(/^~/, '');
                    const segments = text.split(/::|\./);
                    return segments[segments.length - 1] || text;
                };
                const rootDisplayScore = (name: string): number => {
                    const rootView = rootViews[name] || {};
                    const rootPart = ibdParts.find((part: any) => part?.name === name && !part?.containerId) || null;
                    const partCount = Array.isArray(rootView.parts) ? rootView.parts.length : 0;
                    const portCount = Array.isArray(rootView.ports) ? rootView.ports.length : 0;
                    const connectorCount = Array.isArray(rootView.connectors) ? rootView.connectors.length : 0;
                    const isPartDef = String(rootPart?.type || '').toLowerCase().includes('part def');
                    const isInstance = isLikelyInstanceRoot(name);
                    return (
                        connectorCount * 1000
                        + portCount * 100
                        + partCount * 10
                        + (isPartDef ? 5 : 0)
                        - (isInstance ? 5 : 0)
                    );
                };
                const ibd = data.ibd as {
                    parts: any[];
                    ports?: any[];
                    connectors?: any[];
                    containerGroups?: any[];
                    packageContainerGroups?: any[];
                    rootCandidates?: string[];
                    defaultRoot?: string;
                    rootViews?: Record<string, {
                        parts?: any[];
                        ports?: any[];
                        connectors?: any[];
                        containerGroups?: any[];
                        packageContainerGroups?: any[];
                    }>;
                };
                const ibdParts = Array.isArray(ibd.parts) ? ibd.parts : [];
                const ibdPorts = Array.isArray(ibd.ports) ? ibd.ports : [];
                const ibdConnectors = Array.isArray(ibd.connectors) ? ibd.connectors : [];
                const ibdContainerGroups = Array.isArray(ibd.containerGroups) ? ibd.containerGroups : [];
                const ibdPackageContainerGroups = Array.isArray(ibd.packageContainerGroups) ? ibd.packageContainerGroups : [];
                const ibdRootCandidates = Array.isArray(ibd.rootCandidates) ? ibd.rootCandidates : [];
                const rootViews = (ibd.rootViews && typeof ibd.rootViews === 'object') ? ibd.rootViews : {};
                if (ibdRootCandidates.length === 0 || Object.keys(rootViews).length === 0) {
                    return {
                        ...data,
                        elements: ibdParts,
                        parts: ibdParts,
                        ports: ibdPorts,
                        connectors: ibdConnectors,
                        containerGroups: ibdContainerGroups,
                        packageContainerGroups: ibdPackageContainerGroups,
                        ibdRootCandidates: [],
                        ibdRootSummaries: [],
                        selectedIbdRoot: null,
                    };
                }
                const availableRootsRaw = ibdRootCandidates.filter((name) => rootViews[name]);
                const hasAnyLikelyInstance = availableRootsRaw.some((name) => isLikelyInstanceRoot(name));
                const topLevelRootPartByName = new Map<string, any>(
                    ibdParts
                        .filter((part: any) => !part?.containerId)
                        .map((part: any) => [String(part?.name || ''), part])
                );
                const representedNestedTypes = new Set(
                    ibdParts
                        .filter((part: any) => !!part?.containerId)
                        .map((part: any) => typeNameFromPart(part))
                        .filter(Boolean)
                );
                const availableRoots = availableRootsRaw
                    .filter((name) => {
                        const part = topLevelRootPartByName.get(name);
                        if (!part) return true;
                        const isPartDef = String(part?.type || '').toLowerCase().includes('part def');
                        if (!isPartDef) return true;
                        if (representedNestedTypes.has(name) && !hasAnyLikelyInstance) {
                            return false;
                        }
                        return true;
                    })
                    .slice()
                    .sort((a, b) => {
                        const scoreDelta = rootDisplayScore(b) - rootDisplayScore(a);
                        if (scoreDelta !== 0) return scoreDelta;
                        return a.localeCompare(b);
                    });
                const ibdRootSummaries = availableRoots.map((name) => {
                    const rootView = rootViews[name] || {};
                    const rootPart =
                        topLevelRootPartByName.get(name)
                        || (Array.isArray(rootView.parts) ? rootView.parts.find((part: any) => !part?.containerId) : null)
                        || null;
                    const meta = lookupElementMeta(rootPart, name);
                    return {
                        id: meta.stableId || name,
                        name,
                        label: buildSelectorLabel(name, meta.packagePath),
                        packagePath: meta.packagePath,
                        partCount: countWithFallback(rootView.parts),
                        portCount: countWithFallback(rootView.ports),
                        connectorCount: countWithFallback(rootView.connectors),
                    };
                });
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
                const selectedContainerGroups = Array.isArray(selectedRootView?.containerGroups) && selectedRootView.containerGroups.length > 0
                    ? selectedRootView.containerGroups
                    : ibdContainerGroups;
                const selectedPackageContainerGroups = Array.isArray(selectedRootView?.packageContainerGroups) && selectedRootView.packageContainerGroups.length > 0
                    ? selectedRootView.packageContainerGroups
                    : ibdPackageContainerGroups;
                return {
                    ...data,
                    elements: selectedParts,
                    parts: selectedParts,
                    ports: selectedPorts,
                    connectors: selectedConnectors,
                    containerGroups: selectedContainerGroups,
                    packageContainerGroups: selectedPackageContainerGroups,
                    ibdRootCandidates: ibdRootSummaries,
                    ibdRootSummaries,
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
                containerGroups: [],
                packageContainerGroups: [],
                ibdRootCandidates: [],
                selectedIbdRoot: null,
            };
        }

        case 'action-flow-view': {
            const actionElementsByName = new Map<string, any[]>();
            allElements.forEach((el: any) => {
                const typeLower = String(el?.type || '').toLowerCase();
                if (typeLower === 'action' || typeLower === 'action def' || typeLower === 'action definition') {
                    const key = String(el?.name || '').trim();
                    if (!actionElementsByName.has(key)) actionElementsByName.set(key, []);
                    actionElementsByName.get(key)!.push(el);
                }
            });
            const takeActionMeta = (diagram: any, fallbackIndex: number) => {
                const matches = actionElementsByName.get(String(diagram?.name || '').trim()) || [];
                const matchedElement = matches.shift() || null;
                const meta = lookupElementMeta(matchedElement || diagram, `activity-diagram-${fallbackIndex + 1}`);
                return {
                    id: String(diagram?.id || meta.stableId || `activity-diagram-${fallbackIndex + 1}`),
                    packagePath: String(diagram?.packagePath || meta.packagePath || ''),
                };
            };
            const rankActionDiagram = (diagram: any): number => {
                const flowCount = countWithFallback(diagram?.flows);
                const nodeCount = countWithFallback(diagram?.nodes) || countWithFallback(diagram?.actions);
                const sourceKind = String(diagram?.sourceKind || '');
                const sourceBonus = sourceKind === 'actionDef' ? 10000 : (sourceKind === 'performer' ? 5000 : 0);
                return sourceBonus + flowCount * 100 + nodeCount * 10;
            };
            const normalizeActivityDiagram = (diagram: any, index: number) => {
                const meta = takeActionMeta(diagram, index);
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
                        uri: a.uri || diagram.uri,
                        range: a.range || diagram.range,
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

                const nodeIds = new Set(nodes.map((node: any) => String(node.id || node.name)));
                const nodeIdByAlias = new Map<string, string>();
                const registerAlias = (alias: any, nodeId: string) => {
                    const key = String(alias || '').trim();
                    if (!key) return;
                    if (!nodeIdByAlias.has(key)) nodeIdByAlias.set(key, nodeId);
                    const normalized = key.replace(/::/g, '.');
                    if (!nodeIdByAlias.has(normalized)) nodeIdByAlias.set(normalized, nodeId);
                    const lastSegment = normalized.split('.').filter(Boolean).pop();
                    if (lastSegment && !nodeIdByAlias.has(lastSegment)) nodeIdByAlias.set(lastSegment, nodeId);
                };
                nodes.forEach((node: any) => {
                    const nodeId = String(node.id || node.name);
                    registerAlias(node.id, nodeId);
                    registerAlias(node.name, nodeId);
                    registerAlias(node.qualifiedName, nodeId);
                });
                const resolveNodeId = (value: any): string => {
                    const key = String(value || '').trim();
                    if (!key) return '';
                    const normalized = key.replace(/::/g, '.');
                    const segments = normalized.split('.').filter(Boolean);
                    const first = segments[0] || '';
                    const last = segments[segments.length - 1] || '';
                    return nodeIdByAlias.get(key)
                        || nodeIdByAlias.get(normalized)
                        || (last ? nodeIdByAlias.get(last) : undefined)
                        || (first ? nodeIdByAlias.get(first) : undefined)
                        || key;
                };
                const flows = (diagram.flows || []).map((flow: any, idx: number) => ({
                    ...flow,
                    from: resolveNodeId(flow.from),
                    to: resolveNodeId(flow.to),
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

                const normalizedNodes = nodes.map((node: any) => {
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
                });

                const sourceKind = String(diagram?.sourceKind || 'actionDef');
                const hasRenderableContent = normalizedNodes.length > 0 && cleanFlows.length > 0;

                return {
                    id: meta.id,
                    name: diagram.name,
                    label: buildSelectorLabel(String(diagram.name || `Action ${index + 1}`), meta.packagePath),
                    packagePath: meta.packagePath,
                    sourceKind,
                    nodes: normalizedNodes,
                    flows: cleanFlows,
                    interface: {
                        inputs: Array.isArray(diagram.interface?.inputs) ? diagram.interface.inputs : [],
                        outputs: Array.isArray(diagram.interface?.outputs) ? diagram.interface.outputs : [],
                    },
                    hasBehavioralFlow: cleanFlows.length > 0,
                    hasRenderableContent,
                };
            };
            if (data.activityDiagrams && data.activityDiagrams.length > 0) {
                const diagrams = data.activityDiagrams
                    .map((diagram: any, index: number) => normalizeActivityDiagram(diagram, index))
                    .filter((diagram: any) => diagram.hasRenderableContent)
                    .sort((a: any, b: any) => {
                    const scoreDelta = rankActionDiagram(b) - rankActionDiagram(a);
                    if (scoreDelta !== 0) return scoreDelta;
                    return String(a.label || a.name).localeCompare(String(b.label || b.name));
                });
                return {
                    ...data,
                    diagrams,
                    activityDiagramCandidates: diagrams.map((diagram: any) => ({
                        id: diagram.id,
                        name: diagram.name,
                        label: diagram.label,
                        packagePath: diagram.packagePath,
                        sourceKind: diagram.sourceKind,
                        nodeCount: countWithFallback(diagram.nodes),
                        flowCount: countWithFallback(diagram.flows),
                    })),
                };
            }

            const actionDefs = allElements.filter((el: any) => {
                if (!el.type) return false;
                const typeLower = el.type.toLowerCase();
                return typeLower === 'action' || typeLower === 'action def' || typeLower === 'action definition';
            });
            const activityActionDefs = actionDefs.filter((a: any) => a.children && a.children.length > 0);

            const diagrams = activityActionDefs.map((actionDef: any, index: number) => {
                const meta = lookupElementMeta(actionDef, `activity-diagram-${index + 1}`);
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
                    id: meta.stableId,
                    name: actionDef.name,
                    label: buildSelectorLabel(String(actionDef.name || `Action ${index + 1}`), meta.packagePath),
                    packagePath: meta.packagePath,
                    sourceKind: 'actionDef',
                    nodes,
                    flows: [],
                    interface: {
                        inputs: interfaceInputs,
                        outputs: interfaceOutputs,
                    },
                    hasBehavioralFlow: false,
                    hasRenderableContent: nodes.length > 0,
                };
            }).filter((diagram: any) => diagram.hasRenderableContent).sort((a: any, b: any) => {
                const scoreDelta = rankActionDiagram(b) - rankActionDiagram(a);
                if (scoreDelta !== 0) return scoreDelta;
                return String(a.label || a.name).localeCompare(String(b.label || b.name));
            });

            return {
                ...data,
                diagrams,
                activityDiagramCandidates: diagrams.map((diagram: any) => ({
                    id: diagram.id,
                    name: diagram.name,
                    label: diagram.label,
                    packagePath: diagram.packagePath,
                    sourceKind: diagram.sourceKind,
                    nodeCount: countWithFallback(diagram.nodes),
                    flowCount: countWithFallback(diagram.flows),
                })),
            };
        }

        case 'sequence-view': {
            const sequenceElementsByName = new Map<string, any[]>();
            allElements.forEach((el: any) => {
                const typeLower = String(el?.type || '').toLowerCase();
                if (typeLower.includes('interaction') || typeLower.includes('sequence')) {
                    const key = String(el?.name || '').trim();
                    if (!sequenceElementsByName.has(key)) sequenceElementsByName.set(key, []);
                    sequenceElementsByName.get(key)!.push(el);
                }
            });
            const normalizeMessageKind = (value: any): string => {
                const normalized = String(value || '').toLowerCase();
                if (normalized.includes('create')) return 'create';
                if (normalized.includes('return')) return 'return';
                if (normalized.includes('async')) return 'async';
                return 'sync';
            };
            const normalizeFragmentKind = (value: any): string => {
                const normalized = String(value || '').toLowerCase();
                if (normalized.includes('loop')) return 'loop';
                if (normalized.includes('alt')) return 'alt';
                if (normalized.includes('ref')) return 'ref';
                return 'opt';
            };
                const normalizeSequenceFragment = (fragment: any, fragmentIndex: number, messageIds: Set<string>) => ({
                    id: String(fragment?.id || `fragment_${fragmentIndex + 1}`),
                    kind: normalizeFragmentKind(fragment?.kind),
                    label: String(fragment?.label || fragment?.guard || ''),
                    target: String(fragment?.target || fragment?.targetRef || fragment?.target_ref || ''),
                messageIds: (fragment?.messageIds || fragment?.message_ids || []).filter((id: any) => messageIds.has(String(id))).map((id: any) => String(id)),
                operands: (fragment?.operands || []).map((operand: any, operandIndex: number) => ({
                    id: String(operand?.id || `${fragment?.id || `fragment_${fragmentIndex + 1}`}::operand_${operandIndex + 1}`),
                        guard: String(operand?.guard || ''),
                        messageIds: (operand?.messageIds || operand?.message_ids || []).filter((id: any) => messageIds.has(String(id))).map((id: any) => String(id)),
                        fragments: (operand?.fragments || []).map((nested: any, nestedIndex: number) => normalizeSequenceFragment(nested, nestedIndex, messageIds)),
                        uri: operand?.uri,
                        range: operand?.range,
                    })),
                    fragments: (fragment?.fragments || []).map((nested: any, nestedIndex: number) => normalizeSequenceFragment(nested, nestedIndex, messageIds)),
                    order: Number(fragment?.order ?? (fragmentIndex + 1)),
                    uri: fragment?.uri,
                    range: fragment?.range,
                });
            const normalizeSequenceDiagram = (diagram: any, index: number) => {
                const matches = sequenceElementsByName.get(String(diagram?.name || '').trim()) || [];
                const matchedElement = matches.shift() || null;
                const meta = lookupElementMeta(matchedElement || diagram, `sequence-diagram-${index + 1}`);
                const lifelines = (diagram?.lifelines || []).map((lifeline: any, lifelineIndex: number) => ({
                    id: String(lifeline?.id || lifeline?.name || `lifeline_${lifelineIndex + 1}`),
                    name: String(lifeline?.name || lifeline?.id || `Lifeline ${lifelineIndex + 1}`),
                    type: String(lifeline?.type || ''),
                    uri: lifeline?.uri,
                    range: lifeline?.range,
                }));
                const lifelineIds = new Set(lifelines.map((lifeline: any) => lifeline.id));
                const messages = (diagram?.messages || []).map((message: any, messageIndex: number) => ({
                    id: String(message?.id || `message_${messageIndex + 1}`),
                    name: String(message?.name || message?.label || `Message ${messageIndex + 1}`),
                    from: String(message?.from || ''),
                    to: String(message?.to || ''),
                    kind: normalizeMessageKind(message?.kind),
                    order: Number(message?.order ?? (messageIndex + 1)),
                    label: String(message?.label || message?.name || ''),
                    uri: message?.uri,
                    range: message?.range,
                })).filter((message: any) => lifelineIds.has(message.from) && lifelineIds.has(message.to))
                    .sort((a: any, b: any) => a.order - b.order || String(a.id).localeCompare(String(b.id)));
                const messageIds = new Set<string>(messages.map((message: any) => String(message.id)));
                const activations = (diagram?.activations || []).map((activation: any, activationIndex: number) => ({
                    id: String(activation?.id || `activation_${activationIndex + 1}`),
                    lifeline: String(activation?.lifeline || activation?.on || ''),
                    startMessage: String(activation?.startMessage || activation?.start_message || ''),
                    finishMessage: String(activation?.finishMessage || activation?.finish_message || ''),
                    order: Number(activation?.order ?? (activationIndex + 1)),
                    uri: activation?.uri,
                    range: activation?.range,
                })).filter((activation: any) => lifelineIds.has(activation.lifeline));
                const fragments = (diagram?.fragments || [])
                    .map((fragment: any, fragmentIndex: number) => normalizeSequenceFragment(fragment, fragmentIndex, messageIds))
                    .sort((a: any, b: any) => a.order - b.order || String(a.id).localeCompare(String(b.id)));

                return {
                    id: String(diagram?.id || meta.stableId || `sequence-diagram-${index + 1}`),
                    name: String(diagram?.name || `Sequence ${index + 1}`),
                    label: buildSelectorLabel(String(diagram?.name || `Sequence ${index + 1}`), String(diagram?.packagePath || meta.packagePath || '')),
                    packagePath: String(diagram?.packagePath || meta.packagePath || ''),
                    uri: diagram?.uri,
                    lifelines,
                    messages,
                    activations,
                    fragments,
                    range: diagram?.range,
                    hasRenderableContent: lifelines.length > 0 && messages.length > 0,
                };
            };
            const diagrams = (data.sequenceDiagrams || [])
                .map((diagram: any, index: number) => normalizeSequenceDiagram(diagram, index))
                .filter((diagram: any) => diagram.hasRenderableContent)
                .sort((a: any, b: any) => {
                    const scoreDelta = (countWithFallback(b.messages) * 100 + countWithFallback(b.fragments) * 10 + countWithFallback(b.lifelines))
                        - (countWithFallback(a.messages) * 100 + countWithFallback(a.fragments) * 10 + countWithFallback(a.lifelines));
                    if (scoreDelta !== 0) return scoreDelta;
                    return String(a.label || a.name).localeCompare(String(b.label || b.name));
                });
            return {
                ...data,
                diagrams,
                sequenceDiagramCandidates: diagrams.map((diagram: any) => ({
                    id: diagram.id,
                    name: diagram.name,
                    label: diagram.label,
                    packagePath: diagram.packagePath,
                    lifelineCount: countWithFallback(diagram.lifelines),
                    messageCount: countWithFallback(diagram.messages),
                    fragmentCount: countWithFallback(diagram.fragments),
                })),
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
                const hasOwnedBehavior = hasTransitionChildren(el) || hasStateChildren(el);
                return n.endsWith('states')
                    || n.includes('statemachine')
                    || isStateDefinition(el)
                    || hasOwnedBehavior
                    || (t.includes('exhibit') && hasOwnedBehavior);
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
                    label: buildSelectorLabel(
                        String(root?.name || `State Machine ${machineIndex + 1}`),
                        lookupElementMeta(root, machineId).packagePath
                    ),
                    packagePath: lookupElementMeta(root, machineId).packagePath,
                    container: removeCircularRefs(root),
                    states: normalizedStates,
                    transitions,
                };
            }

            let stateMachines = machineRoots
                .map((root: any, index: number) => buildMachine(root, index))
                .filter((machine: any) => {
                    const realStates = machine.states.filter((state: any) => state.kind !== 'initial' || state.name !== 'entry');
                    return realStates.length > 0;
                });

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

            stateMachines = stateMachines.sort((a: any, b: any) => {
                const stateCountDelta = b.states.length - a.states.length;
                if (stateCountDelta !== 0) return stateCountDelta;
                const transitionCountDelta = b.transitions.length - a.transitions.length;
                if (transitionCountDelta !== 0) return transitionCountDelta;
                return String(a.label || a.name).localeCompare(String(b.label || b.name));
            });

            const flatStates = stateMachines.flatMap((machine: any) => machine.states);
            const flatTransitions = stateMachines.flatMap((machine: any) => machine.transitions);

            return {
                ...data,
                stateMachines,
                stateMachineCandidates: stateMachines.map((machine: any) => ({
                    id: machine.id,
                    name: machine.name,
                    label: machine.label,
                    packagePath: machine.packagePath,
                    stateCount: countWithFallback(machine.states),
                    transitionCount: countWithFallback(machine.transitions),
                })),
                states: flatStates,
                transitions: flatTransitions,
            };
        }

        default:
            return data;
    }
}

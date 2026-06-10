/**
 * Build element trees from graph DTOs (nodes + edges).
 * Used by Model Explorer and legacy element-tree consumers.
 */
/* eslint-disable @typescript-eslint/no-explicit-any */

import type { GraphPayloadDto } from './visualizationTypes';

function isBuilderDiagnosticNode(n: { type?: string; element_type?: string }): boolean {
    const kind = (n.type || n.element_type || '').toLowerCase();
    return kind === 'diagnostic';
}

export function graphToElementTree(graph: GraphPayloadDto | null | undefined): any[] {
    if (!graph?.nodes?.length) return [];
    const nodes = graph.nodes.filter((n: any) => !isBuilderDiagnosticNode(n));
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
            children: [] as any[],
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

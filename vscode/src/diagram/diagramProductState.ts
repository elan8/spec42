export type DiagramProductState = {
  completeness: { status: string };
  projection: { nodes: unknown[] };
};

export function isEmptyIncompleteDiagramProduct(product: DiagramProductState): boolean {
  return product.projection.nodes.length === 0 && product.completeness.status === "incomplete";
}

/** Renderer-owned state; node ids address one product, section keys remain opaque here.
 * Browser expandedNodeIds records collapsed rows (empty means all expanded). */
export interface DisclosureState {
  expandedNodeIds: string[];
  sectionStates: Array<{ nodeId: string; sectionKey: string; expanded: boolean }>;
}

type PresentationProduct = { projection: { nodes: Array<{ presentationKey: string }> } };

/** Transfer state only by authority-supplied occurrence keys, never by node order or labels.
 * The caller owns selected-view continuity. Unknown/duplicate keys cannot carry state. */
export function remapDisclosureState(previous: PresentationProduct | undefined, next: PresentationProduct,
  state: DisclosureState | undefined): DisclosureState | undefined {
  if (!previous || !state) return undefined;
  const uniqueKeys = (product: PresentationProduct): Map<string, string | undefined> => {
    const keys = new Map<string, string | undefined>();
    product.projection.nodes.forEach((node, index) => {
      if (node.presentationKey) keys.set(node.presentationKey, keys.has(node.presentationKey) ? undefined : `n:${index}`);
    });
    return keys;
  };
  const oldKeys = uniqueKeys(previous);
  const newKeys = uniqueKeys(next);
  const ids = new Map<string, string>();
  for (const [key, oldId] of oldKeys) {
    const newId = newKeys.get(key);
    if (oldId !== undefined && newId !== undefined) ids.set(oldId, newId);
  }
  return {
    expandedNodeIds: [...new Set(state.expandedNodeIds.flatMap(id => ids.has(id) ? [ids.get(id)!] : []))],
    sectionStates: state.sectionStates.flatMap(section => {
      const nodeId = ids.get(section.nodeId);
      return nodeId === undefined ? [] : [{ ...section, nodeId }];
    }),
  };
}

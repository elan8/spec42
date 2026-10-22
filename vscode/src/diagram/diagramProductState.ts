export type DiagramProductState = {
  completeness: { status: string };
  projection: { nodes: unknown[] };
};

export function isEmptyIncompleteDiagramProduct(product: DiagramProductState): boolean {
  return product.projection.nodes.length === 0 && product.completeness.status === "incomplete";
}

export type DiagramProductNodes = {
  modelDigest: string;
  references: unknown[];
  projection: { nodes: unknown[] };
};

/**
 * Whether a disclosure state (expanded node ids such as `n:3`) taken from `previous` still names
 * the same nodes in `next`. Node ids are indexes into one product's projection, so they carry over
 * only between products of one model state that project the same nodes: loading a browser branch
 * keeps the model digest but projects more nodes, and shifts their indexes.
 */
export function sameProjectedNodes(previous: DiagramProductNodes | undefined, next: DiagramProductNodes): boolean {
  return previous !== undefined &&
    previous.modelDigest === next.modelDigest &&
    previous.projection.nodes.length === next.projection.nodes.length &&
    JSON.stringify([previous.projection.nodes, previous.references]) ===
      JSON.stringify([next.projection.nodes, next.references]);
}

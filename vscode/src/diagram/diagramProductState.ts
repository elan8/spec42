export type DiagramProductState = {
  completeness: { status: string };
  projection: { nodes: unknown[] };
};

export function isEmptyIncompleteDiagramProduct(product: DiagramProductState): boolean {
  return product.projection.nodes.length === 0 && product.completeness.status === "incomplete";
}

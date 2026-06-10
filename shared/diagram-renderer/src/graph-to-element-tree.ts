/** Build element trees from graph DTOs (nodes + edges). */

export interface GraphPayloadDto {
  nodes?: Array<Record<string, unknown>>;
  edges?: Array<Record<string, unknown>>;
}

function isBuilderDiagnosticNode(node: { type?: string; element_type?: string }): boolean {
  const kind = String(node.type || node.element_type || "").toLowerCase();
  return kind === "diagnostic";
}

export function graphToElementTree(graph: GraphPayloadDto | null | undefined): Record<string, unknown>[] {
  if (!graph?.nodes?.length) return [];
  const nodes = graph.nodes.filter((node) => !isBuilderDiagnosticNode(node as { type?: string; element_type?: string }));
  const edges = graph.edges || [];
  const nodeMap = new Map<string, Record<string, unknown>>();
  nodes.forEach((node) => {
    nodeMap.set(String(node.id), {
      id: node.id,
      name: node.name,
      type: node.type || node.element_type,
      range: node.range,
      attributes: node.attributes || {},
      relationships: [] as unknown[],
      children: [] as unknown[],
    });
  });
  const getEdgeType = (edge: Record<string, unknown>) => String(edge.type || edge.rel_type || "").toLowerCase();
  edges.forEach((edge) => {
    if (getEdgeType(edge) === "contains" && edge.source && edge.target) {
      const parent = nodeMap.get(String(edge.source));
      const child = nodeMap.get(String(edge.target));
      if (parent && child) {
        (parent.children as unknown[]).push(child);
      }
    }
    const relTypes = ["typing", "specializes", "connection", "bind", "allocate", "transition", "satisfy", "verify", "subject"];
    if (relTypes.includes(getEdgeType(edge))) {
      const src = nodeMap.get(String(edge.source));
      if (src) {
        (src.relationships as unknown[]).push({
          source: edge.source,
          target: edge.target,
          type: edge.type,
          name: edge.name,
        });
      }
    }
  });
  const targetsOfContains = new Set(
    edges.filter((edge) => getEdgeType(edge) === "contains").map((edge) => String(edge.target)),
  );
  return nodes
    .filter((node) => !targetsOfContains.has(String(node.id)))
    .map((node) => nodeMap.get(String(node.id)))
    .filter((node): node is Record<string, unknown> => Boolean(node));
}

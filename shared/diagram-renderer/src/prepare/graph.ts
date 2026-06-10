import { isOverviewVisualElementType, normalizeEdgeKind } from "../graph-normalization";
import { isDefinitionKind, isReferenceKind } from "../node-notation";
import type { PreparedNode, PreparedView, UnknownRecord, VisualizationPayload } from "./types";
import { asArray, asRecord, asString, elementTypeOf, isPackage, isSyntheticPackage, nodeUri } from "./util";

function isGeneralViewDiagramNode(node: UnknownRecord): boolean {
  if (isSyntheticPackage(node)) {
    return false;
  }
  return isOverviewVisualElementType(elementTypeOf(node));
}

function buildGeneralPackageContainerGroups(nodes: PreparedNode[]): UnknownRecord[] {
  const byPackage = new Map<string, string[]>();
  for (const node of nodes) {
    const qn = asString(asRecord(node.attributes).qualifiedName);
    const sep = qn.indexOf("::");
    if (sep <= 0) continue;
    const pkg = qn.slice(0, sep);
    const members = byPackage.get(pkg) ?? [];
    members.push(node.id);
    byPackage.set(pkg, members);
  }
  if (byPackage.size < 2) return [];
  return [...byPackage.entries()].map(([name, memberIds]) => ({
    id: `package:${name}`,
    name,
    memberIds,
  }));
}

export function prepareGraph(graphInput: unknown, visualization: VisualizationPayload): PreparedView {
  const graph = asRecord(graphInput);
  const rawNodes = asArray(graph.nodes).map(asRecord);
  const sourceNodes = rawNodes.filter((node) => isGeneralViewDiagramNode(node));
  const nodeIds = new Set(sourceNodes.map((node) => asString(node.id)));
  const nodes = sourceNodes.map((node) => ({
    id: asString(node.id),
    label: asString(node.name ?? node.qualifiedName ?? node.id, "Unnamed"),
    kind: elementTypeOf(node) || "Element",
    sourcePath: asString(node.sourcePath ?? node.source_path) || null,
    uri: nodeUri(node),
    range: (node.range as { start?: { line?: number } } | null | undefined) ?? null,
    attributes: {
      ...asRecord(node.attributes),
      qualifiedName: asString(node.qualifiedName ?? asRecord(node.attributes).qualifiedName),
      isPackage: isPackage(node),
      isDefinition: isDefinitionKind(asString(node.type ?? node.element_type, "")),
      isReference: isReferenceKind(asString(node.type ?? node.element_type, "")),
    },
  }));
  const edges = asArray(graph.edges)
    .map(asRecord)
    .filter((edge) => nodeIds.has(asString(edge.source)) && nodeIds.has(asString(edge.target)))
    .map((edge, index) => {
      const relationType = asString(edge.type ?? edge.rel_type ?? edge.relationType ?? edge.name, "");
      const label = asString(edge.name ?? edge.label ?? edge.type ?? edge.rel_type, "");
      return {
        id: asString(edge.id, `edge-${index}`),
        source: asString(edge.source),
        target: asString(edge.target),
        label,
        edgeKind: normalizeEdgeKind(relationType),
        attributes: {
          ...asRecord(edge.attributes),
          relationType: normalizeEdgeKind(relationType),
        },
      };
    });
  const packageContainerGroups = buildGeneralPackageContainerGroups(nodes);
  return {
    title: visualization?.selectedViewName || "SysML View",
    view: visualization?.view || "general-view",
    nodes,
    edges,
    meta: packageContainerGroups.length > 0 ? { packageContainerGroups } : undefined,
  };
}

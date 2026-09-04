import { normalizeEdgeKind } from "../graph-normalization";
import { legacyNotationRole } from "./legacy-notation";
import type {
  InterconnectionPreparedEdge,
  InterconnectionPreparedNode,
  InterconnectionPreparedView,
  InterconnectionSceneDto,
  InterconnectionScenePortDto,
  VisualizationPayload,
} from "./types";
import { asString } from "./util";

function portsForNode(
  ownerNodeId: string,
  ports: InterconnectionScenePortDto[],
): InterconnectionScenePortDto[] {
  return ports.filter((port) => port.ownerNodeId === ownerNodeId);
}

function mapPortDetail(port: InterconnectionScenePortDto) {
  return {
    id: port.id,
    name: port.name,
    direction: port.direction,
    conjugated: port.conjugated,
    semanticId: port.semanticId,
    multiplicity: port.multiplicity ?? "[1]",
    portType: port.typeName,
    portSide: port.sideHint === "west" ? "left" : port.sideHint === "east" ? "right" : undefined,
    uri: port.uri,
    range: port.range,
    attributes: {
      parentId: port.ownerNodeId,
      scenePortId: port.id,
      sideHint: port.sideHint,
    },
  };
}

function rootCoverage(rootId: string, scene: InterconnectionSceneDto): number {
  const prefix = `${rootId}.`;
  return scene.nodes.filter((node) => node.id === rootId || node.id.startsWith(prefix)).length;
}

function selectRoot(scene: InterconnectionSceneDto): string | null {
  return [...scene.view.rootIds].sort((left, right) => {
    const coverageDelta = rootCoverage(right, scene) - rootCoverage(left, scene);
    if (coverageDelta !== 0) return coverageDelta;
    const depthDelta = left.split(".").length - right.split(".").length;
    if (depthDelta !== 0) return depthDelta;
    return left.localeCompare(right);
  })[0] ?? null;
}

export function prepareInterconnectionScene(
  scene: InterconnectionSceneDto,
  visualization: VisualizationPayload,
): InterconnectionPreparedView {
  const selectedRoot = selectRoot(scene);
  const selectedRootHasCoverage = selectedRoot !== null && rootCoverage(selectedRoot, scene) > 0;
  const nodeIds = new Set(scene.nodes.map((node) => node.id));
  const nodes: InterconnectionPreparedNode[] = scene.nodes.map((node) => {
    const nodePorts = portsForNode(node.id, scene.ports);
    const portDetails = nodePorts.map(mapPortDetail);
    return {
      id: node.id,
      label: node.name,
      kind: node.kind === "ref" ? "part" : "part",
      uri: node.uri,
      range: node.range,
      attributes: {
        containerId: node.parentId ?? null,
        qualifiedName: node.qualifiedName,
        semanticId: node.semanticId,
        definitionId: node.definitionId,
        partType: node.typeName,
        ports: portDetails.map((port) => port.name),
        portDetails,
        notationRole: legacyNotationRole(node.kind),
        sceneNodeId: node.id,
      },
    };
  });

  for (const container of scene.containers) {
    const inSelectedScope = !selectedRootHasCoverage
      || selectedRoot === null
      || container.id === selectedRoot
      || container.id.startsWith(`${selectedRoot}.`);
    if (nodeIds.has(container.id) || !inSelectedScope) continue;
    nodes.push({
      id: container.id,
      label: container.label,
      kind: "package",
      attributes: {
        isSyntheticContainer: true,
        containerId: container.parentId ?? null,
        qualifiedName: container.label,
        memberNodeIds: container.memberNodeIds,
        layoutDepth: container.depth,
      },
    });
    nodeIds.add(container.id);
  }

  const edges: InterconnectionPreparedEdge[] = scene.edges
    .filter((edge) => nodeIds.has(edge.sourceNodeId) && nodeIds.has(edge.targetNodeId))
    .map((edge) => ({
      id: edge.id,
      source: edge.sourceNodeId,
      target: edge.targetNodeId,
      label: edge.label ?? edge.kind,
      edgeKind: normalizeEdgeKind(edge.kind),
      attributes: {
        sourceId: edge.sourcePortId,
        targetId: edge.targetPortId,
        sourcePortId: edge.sourcePortId,
        targetPortId: edge.targetPortId,
        sourceNodeId: edge.sourceNodeId,
        targetNodeId: edge.targetNodeId,
        semanticId: edge.semanticId,
        sourceExpression: edge.sourceExpression,
        targetExpression: edge.targetExpression,
        relationType: edge.kind,
        canonicalScene: true,
      },
    }));

  return {
    title: scene.view.name || asString(visualization.selectedViewName) || "Interconnection View",
    view: "interconnection-view",
    nodes,
    edges,
    meta: {
      canonicalScene: true,
      schemaVersion: scene.schemaVersion,
      selectedRoot,
      rootCandidates: scene.view.rootIds,
      diagnostics: scene.diagnostics,
    },
  };
}

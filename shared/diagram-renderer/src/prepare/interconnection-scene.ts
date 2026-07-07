import { normalizeEdgeKind } from "../graph-normalization";
import { isDefinitionKind, isReferenceKind } from "../node-notation";
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

export function prepareInterconnectionScene(
  scene: InterconnectionSceneDto,
  visualization: VisualizationPayload,
): InterconnectionPreparedView {
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
        isDefinition: isDefinitionKind(node.kind),
        isReference: isReferenceKind(node.kind) || node.kind === "ref",
        sceneNodeId: node.id,
      },
    };
  });

  for (const container of scene.containers) {
    if (nodeIds.has(container.id)) continue;
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
      selectedRoot: scene.view.rootIds[0] ?? null,
      rootCandidates: scene.view.rootIds,
      diagnostics: scene.diagnostics,
    },
  };
}

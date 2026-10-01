import { normalizeEdgeKind } from "../graph-normalization";
import type { DiagramEdge } from "../../../src/generated/diagram-product/DiagramEdge";
import type { DiagramNode } from "../../../src/generated/diagram-product/DiagramNode";
import type { InterconnectionMetadata } from "../../../src/generated/diagram-product/InterconnectionMetadata";
import type { SemanticReference } from "../../../src/generated/diagram-product/SemanticReference";
import { prepareInterconnectionScene } from "./interconnection-scene";
import type {
  InterconnectionSceneDto,
  InterconnectionSceneEdgeDto,
  InterconnectionSceneNodeDto,
  InterconnectionScenePortDto,
  PreparedView,
} from "./types";

type Navigation = (index: unknown) => {
  uri: string | null;
  range: {
    start?: { line?: number; character?: number };
    end?: { line?: number; character?: number };
  };
};

/**
 * SysML 8.2.3.11 interconnection notation: parts (and part-refs) are nested nodes,
 * ports sit on the node boundary, connections are edges between those ports.
 * Schema-5 projections publish that classification as `metadata.parts` / `ports` /
 * `connectors` plus owner indexes; this adapter is presentation, not a second
 * semantic model.
 */
export function prepareInterconnectionFromTypedProjection(input: {
  name: string;
  nodes: DiagramNode[];
  edges: DiagramEdge[];
  exposedRoots: number[];
  metadata: InterconnectionMetadata;
  references: SemanticReference[];
  navigation: Navigation;
}): PreparedView {
  const scene = interconnectionSceneFromTypedProjection(input);
  return prepareInterconnectionScene(scene, { selectedViewName: input.name });
}

function interconnectionSceneFromTypedProjection(input: {
  name: string;
  nodes: DiagramNode[];
  edges: DiagramEdge[];
  exposedRoots: number[];
  metadata: InterconnectionMetadata;
  references: SemanticReference[];
  navigation: Navigation;
}): InterconnectionSceneDto {
  const rawNodes = input.nodes;
  const indexSet = (value: number[]): Set<number> =>
    new Set(value.filter((index) => rawNodes[index] !== undefined));
  const partIndexes = indexSet(input.metadata.parts);
  const portIndexes = indexSet(input.metadata.ports);
  if (partIndexes.size === 0) {
    rawNodes.forEach((element, index) => {
      if (isPartMetaclass(element.metaclass)) partIndexes.add(index);
    });
  }
  if (portIndexes.size === 0) {
    rawNodes.forEach((element, index) => {
      if (isPortMetaclass(element.metaclass)) portIndexes.add(index);
    });
  }

  const idFor = (index: number) => `n:${index}`;
  const qualifiedName = (index: number): string => {
    const element = rawNodes[index];
    const reference = input.references[element.reference];
    if (reference?.kind === "qualified-name" && reference.qualifiedName) return reference.qualifiedName;
    return element.name ?? idFor(index);
  };
  const typeName = (element: DiagramNode): string | undefined => {
    const labels = element.typing.status === "resolved" || element.typing.status === "partial"
      ? element.typing.types.map((type) => type.label)
      : [];
    return labels.length > 0 ? labels.join(" & ") : undefined;
  };
  const location = (element: DiagramNode) => {
    const source = input.navigation(element.source);
    const range = source.range;
    const start = range.start;
    const end = range.end;
    return {
      uri: source.uri ?? undefined,
      range:
        typeof start?.line === "number" &&
        typeof start.character === "number" &&
        typeof end?.line === "number" &&
        typeof end.character === "number"
          ? {
              start: { line: start.line, character: start.character },
              end: { line: end.line, character: end.character },
            }
          : undefined,
    };
  };
  const sceneKind = (element: DiagramNode): string => {
    const role = element.notationRole;
    if (role === "reference-usage") return "ref";
    if (role === "definition") return "def";
    return "part";
  };

  const nodes: InterconnectionSceneNodeDto[] = [...partIndexes]
    .sort((left, right) => left - right)
    .map((index) => {
      const element = rawNodes[index];
      const owner = element.owner ?? undefined;
      const parentId = owner !== undefined && partIndexes.has(owner) ? idFor(owner) : undefined;
      const placed = location(element);
      return {
        id: idFor(index),
        semanticId: qualifiedName(index),
        qualifiedName: qualifiedName(index),
        name: element.name ?? idFor(index),
        kind: sceneKind(element),
        typeName: typeName(element),
        parentId,
        uri: placed.uri,
        range: placed.range,
      };
    });

  const ports: InterconnectionScenePortDto[] = [...portIndexes]
    .sort((left, right) => left - right)
    .flatMap((index) => {
      const element = rawNodes[index];
      const owner = element.owner ?? undefined;
      if (owner === undefined || !partIndexes.has(owner)) return [];
      const placed = location(element);
      return [
        {
          id: idFor(index),
          semanticId: qualifiedName(index),
          ownerNodeId: idFor(owner),
          name: element.name ?? idFor(index),
          typeName: typeName(element),
          // Authored, resolved facts (never derived from `name` or `typeName`): the query
          // reports `direction: null` / `conjugated: false` rather than omitting them, so an
          // absent authored direction is a fact this adapter forwards, not a gap it fills in.
          direction: element.direction ?? undefined,
          conjugated: element.conjugated === true,
          sideHint: "",
          uri: placed.uri,
          range: placed.range,
        },
      ];
    });

  const portOwner = new Map(ports.map((port) => [port.id, port.ownerNodeId]));
  const partIds = new Set(nodes.map((node) => node.id));
  const endpoint = (index: unknown): { nodeId: string; portId: string } | undefined => {
    if (typeof index !== "number") return undefined;
    const id = idFor(index);
    const owner = portOwner.get(id);
    if (owner) return { nodeId: owner, portId: id };
    if (partIds.has(id)) return { nodeId: id, portId: "" };
    return undefined;
  };

  const edges: InterconnectionSceneEdgeDto[] = input.edges
    .flatMap((edge, index) => {
      const kind = edge.kind;
      if (!isInterconnectionEdgeKind(kind)) return [];
      const source = endpoint(edge.source);
      const target = endpoint(edge.target);
      if (!source || !target) return [];
      return [
        {
          id: `e:${index}`,
          kind,
          sourcePortId: source.portId,
          targetPortId: target.portId,
          sourceNodeId: source.nodeId,
          targetNodeId: target.nodeId,
        },
      ];
    });

  const rootIds = input.exposedRoots
    .filter((index) => partIndexes.has(index))
    .map(idFor);

  return {
    schemaVersion: 2,
    view: {
      id: input.name,
      name: input.name,
      type: "InterconnectionView",
      rootIds,
    },
    nodes,
    ports,
    edges,
    containers: [],
    diagnostics: [],
  };
}

function isPartMetaclass(metaclass: string): boolean {
  const normalized = metaclass.toLowerCase();
  return normalized === "partdefinition" || normalized === "partusage";
}

function isPortMetaclass(metaclass: string): boolean {
  const normalized = metaclass.toLowerCase();
  return (
    normalized === "portusage" ||
    normalized === "portdefinition" ||
    normalized === "conjugatedportdefinition"
  );
}

/**
 * The generator emits an edge kind as `connector` / `flow` / `containment` or a
 * `DiagramRelationshipKind` spelling (`binding-connector`, `interface-connection`, …).
 * Normalize before matching so binding and interface connectors are not silently dropped —
 * the same normalization `interconnection-scene` applies to the resulting DTO.
 */
const INTERCONNECTION_EDGE_KINDS = new Set(["connection", "flow", "bind", "interface"]);

function isInterconnectionEdgeKind(kind: string): boolean {
  return INTERCONNECTION_EDGE_KINDS.has(normalizeEdgeKind(kind));
}

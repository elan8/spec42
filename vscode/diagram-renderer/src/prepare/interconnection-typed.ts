import { prepareInterconnectionScene } from "./interconnection-scene";
import type {
  InterconnectionSceneDto,
  InterconnectionSceneEdgeDto,
  InterconnectionSceneNodeDto,
  InterconnectionScenePortDto,
  PreparedView,
  UnknownRecord,
} from "./types";
import { asArray, asRecord, asString } from "./util";

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
  nodes: unknown[];
  edges: unknown[];
  exposedRoots: unknown;
  metadata: UnknownRecord;
  references: unknown[];
  navigation: Navigation;
}): PreparedView {
  const scene = interconnectionSceneFromTypedProjection(input);
  return prepareInterconnectionScene(scene, { selectedViewName: input.name });
}

function interconnectionSceneFromTypedProjection(input: {
  name: string;
  nodes: unknown[];
  edges: unknown[];
  exposedRoots: unknown;
  metadata: UnknownRecord;
  references: unknown[];
  navigation: Navigation;
}): InterconnectionSceneDto {
  const rawNodes = input.nodes.map(asRecord);
  const indexSet = (value: unknown): Set<number> =>
    new Set(
      asArray(value).filter((index): index is number =>
        typeof index === "number" && rawNodes[index] !== undefined,
      ),
    );
  const partIndexes = indexSet(input.metadata.parts);
  const portIndexes = indexSet(input.metadata.ports);
  if (partIndexes.size === 0) {
    rawNodes.forEach((element, index) => {
      if (isPartMetaclass(asString(element.metaclass))) partIndexes.add(index);
    });
  }
  if (portIndexes.size === 0) {
    rawNodes.forEach((element, index) => {
      if (isPortMetaclass(asString(element.metaclass))) portIndexes.add(index);
    });
  }

  const idFor = (index: number) => `n:${index}`;
  const qualifiedName = (index: number): string => {
    const element = rawNodes[index];
    const reference = typeof element.reference === "number" ? input.references[element.reference] : undefined;
    const qualified = asString(asRecord(reference).qualifiedName);
    if (qualified) return qualified;
    return asString(element.name, idFor(index));
  };
  const typeName = (element: UnknownRecord): string | undefined => {
    const typing = asRecord(element.typing);
    const labels = asArray(typing.types)
      .map(asRecord)
      .map((type) => type.label)
      .filter((label): label is string => typeof label === "string");
    return labels.length > 0 ? labels.join(" & ") : undefined;
  };
  const location = (element: UnknownRecord) => {
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
  const sceneKind = (element: UnknownRecord): string => {
    const role = asString(element.notationRole);
    if (role === "reference-usage") return "ref";
    if (role === "definition") return "def";
    return "part";
  };

  const nodes: InterconnectionSceneNodeDto[] = [...partIndexes]
    .sort((left, right) => left - right)
    .map((index) => {
      const element = rawNodes[index];
      const owner = typeof element.owner === "number" ? element.owner : undefined;
      const parentId = owner !== undefined && partIndexes.has(owner) ? idFor(owner) : undefined;
      const placed = location(element);
      return {
        id: idFor(index),
        semanticId: qualifiedName(index),
        qualifiedName: qualifiedName(index),
        name: asString(element.name, idFor(index)),
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
      const owner = typeof element.owner === "number" ? element.owner : undefined;
      if (owner === undefined || !partIndexes.has(owner)) return [];
      const placed = location(element);
      return [
        {
          id: idFor(index),
          semanticId: qualifiedName(index),
          ownerNodeId: idFor(owner),
          name: asString(element.name, idFor(index)),
          typeName: typeName(element),
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

  const edges: InterconnectionSceneEdgeDto[] = asArray(input.edges)
    .map(asRecord)
    .flatMap((edge, index) => {
      const kind = asString(edge.kind);
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

  const rootIds = asArray(input.exposedRoots)
    .filter((index): index is number => typeof index === "number" && partIndexes.has(index))
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

function isInterconnectionEdgeKind(kind: string): boolean {
  const normalized = kind.toLowerCase();
  return (
    normalized === "connector" ||
    normalized === "connection" ||
    normalized === "flow" ||
    normalized === "bind" ||
    normalized === "binding" ||
    normalized === "interface"
  );
}

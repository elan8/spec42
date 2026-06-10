import { normalizeEdgeKind } from "../graph-normalization";
import { isDefinitionKind, isReferenceKind } from "../node-notation";
import type { PreparedNode, PreparedView, UnknownRecord, VisualizationPayload } from "./types";
import { asArray, asRecord, asString, firstPresent } from "./util";

function ibdConnectorKind(connector: UnknownRecord): string {
  const type = asString(connector.type ?? connector.relationType ?? connector.rel_type).trim();
  const name = asString(connector.name ?? connector.label).trim();
  const itemType = asString(connector.itemType).trim();
  const interfaceName = asString(connector.interfaceName ?? connector.interfaceType ?? connector.interfaceDefinition).trim();
  const source = `${type} ${name}`.toLowerCase();
  if (source.includes("binding") || source.includes("bind")) return "binding";
  if (source.includes("reference") || source.includes("ref")) return "reference";
  if (source.includes("interface") || interfaceName) return "interface";
  if (source.includes("flow") || itemType) return "flow";
  return type || "connection";
}

function ibdConnectorLabel(connector: UnknownRecord, type: string): string {
  const name = asString(connector.name ?? connector.label).trim();
  const interfaceName = asString(connector.interfaceName ?? connector.interfaceType ?? connector.interfaceDefinition).trim();
  const itemType = asString(connector.itemType).trim();
  const normalized = type.toLowerCase();
  if (normalized.includes("flow") && itemType) return itemType;
  if (normalized.includes("interface") && interfaceName) return interfaceName;
  return name || type || "connection";
}

function findScopedRootPart(nodes: PreparedNode[], selectedRoot: string): PreparedNode | undefined {
  const normalized = selectedRoot.trim().toLowerCase();
  if (!normalized) return undefined;
  return nodes.find((node) => {
    if (asRecord(node.attributes).isSyntheticContainer) return false;
    const label = node.label.trim().toLowerCase();
    if (label === normalized || node.id.trim().toLowerCase() === normalized) return true;
    const qualifiedName = asString(asRecord(node.attributes).qualifiedName).replace(/::/g, ".").toLowerCase();
    return qualifiedName === normalized || qualifiedName.endsWith(`.${normalized}`);
  });
}

function filterContainerGroupsForScopedRoot(
  containerGroups: UnknownRecord[],
  baseNodes: PreparedNode[],
  scopedName: string,
): UnknownRecord[] {
  if (!scopedName.trim()) return containerGroups;
  const root = findScopedRootPart(baseNodes, scopedName);
  if (!root) return containerGroups;
  const rootQualifiedName = asString(asRecord(root.attributes).qualifiedName).replace(/::/g, ".");
  const packagePrefix = rootQualifiedName.includes(".") ? rootQualifiedName.split(".")[0] : "";
  if (!packagePrefix) return containerGroups;
  return containerGroups.filter((group) => {
    const groupQualifiedName = asString(group.qualifiedName ?? group.label).replace(/::/g, ".");
    const groupLabel = asString(group.label ?? group.name);
    return !(groupQualifiedName === packagePrefix || groupLabel === packagePrefix);
  });
}

function collapseRedundantOuterBoundaries(nodes: PreparedNode[], selectedRoot: string): PreparedNode[] {
  const root = findScopedRootPart(nodes, selectedRoot);
  if (!root) return nodes;

  const removedSyntheticIds = new Set<string>();
  let parentId = asString(asRecord(root.attributes).containerId);
  while (parentId) {
    const parent = nodes.find((node) => node.id === parentId);
    if (!parent || !asRecord(parent.attributes).isSyntheticContainer) break;
    removedSyntheticIds.add(parentId);
    parentId = asString(asRecord(parent.attributes).containerId);
  }
  if (removedSyntheticIds.size === 0) {
    root.attributes = { ...asRecord(root.attributes), isDiagramRoot: true };
    return nodes;
  }

  const rootQualifiedName = asString(asRecord(root.attributes).qualifiedName).replace(/::/g, ".");
  const resolveContainerId = (node: PreparedNode): string | null => {
    const attrs = asRecord(node.attributes);
    const current = asString(attrs.containerId);
    if (!current || !removedSyntheticIds.has(current)) return current || null;
    if (node.id === root.id) return null;
    const qualifiedName = asString(attrs.qualifiedName).replace(/::/g, ".");
    if (rootQualifiedName && (qualifiedName === rootQualifiedName || qualifiedName.startsWith(`${rootQualifiedName}.`))) {
      return root.id;
    }
    return null;
  };

  return nodes
    .filter((node) => !removedSyntheticIds.has(node.id))
    .map((node) => {
      const attrs = asRecord(node.attributes);
      const nextContainerId = resolveContainerId(node);
      const nextAttributes: Record<string, unknown> = {
        ...attrs,
        containerId: nextContainerId,
      };
      delete nextAttributes._fallbackContainerId;
      if (node.id === root.id) {
        nextAttributes.isDiagramRoot = true;
      }
      return { ...node, attributes: nextAttributes };
    });
}

function groupMemberIds(group: UnknownRecord): string[] {
  return asArray(group.memberPartIds ?? group.memberIds ?? group.nodeIds)
    .map((value) => asString(value))
    .filter(Boolean);
}

function resolveInterconnectionContainerIds(nodes: PreparedNode[]): void {
  const aliases = new Map<string, string>();
  const addAlias = (alias: unknown, id: string) => {
    const text = asString(alias).trim();
    if (!text) return;
    aliases.set(text, id);
    aliases.set(text.replace(/::/g, "."), id);
    aliases.set(text.replace(/\./g, "::"), id);
  };
  for (const node of nodes) {
    const attrs = asRecord(node.attributes);
    addAlias(node.id, node.id);
    addAlias(node.label, node.id);
    addAlias(attrs.qualifiedName, node.id);
  }

  for (const node of nodes) {
    const attrs = asRecord(node.attributes);
    const rawParent = asString(attrs.containerId);
    const fallbackParent = asString(attrs._fallbackContainerId);
    const resolved = aliases.get(rawParent) ?? aliases.get(rawParent.replace(/::/g, ".")) ?? aliases.get(fallbackParent);
    if (resolved && resolved !== node.id) {
      node.attributes = { ...attrs, containerId: resolved };
    } else if (rawParent) {
      node.attributes = { ...attrs, containerId: null };
    }
  }
}

function pruneEmptySyntheticContainers(nodes: PreparedNode[]): PreparedNode[] {
  let current = nodes;
  let changed = true;
  while (changed) {
    changed = false;
    const childCount = new Map<string, number>();
    for (const node of current) {
      const parentId = asString(asRecord(node.attributes).containerId);
      if (parentId) childCount.set(parentId, (childCount.get(parentId) ?? 0) + 1);
    }
    const next = current.filter((node) => {
      const attrs = asRecord(node.attributes);
      const emptySynthetic = Boolean(attrs.isSyntheticContainer) && !childCount.has(node.id);
      if (emptySynthetic) changed = true;
      return !emptySynthetic;
    });
    if (changed) {
      const ids = new Set(next.map((node) => node.id));
      for (const node of next) {
        const attrs = asRecord(node.attributes);
        const parentId = asString(attrs.containerId);
        if (parentId && !ids.has(parentId)) {
          node.attributes = { ...attrs, containerId: null };
        }
      }
    }
    current = next;
  }
  return current;
}

function synthesizeInterconnectionContainers(
  baseNodes: PreparedNode[],
  containerGroups: UnknownRecord[],
  packageContainerGroups: UnknownRecord[],
): PreparedNode[] {
  const byId = new Map(baseNodes.map((node) => [node.id, node]));
  const nodes = [...baseNodes];
  const resolveMember = (memberId: string): PreparedNode | undefined => {
    const normalized = memberId.replace(/::/g, ".");
    return (
      byId.get(memberId) ??
      baseNodes.find((node) => {
        const attrs = asRecord(node.attributes);
        const qualifiedName = asString(attrs.qualifiedName).replace(/::/g, ".");
        return node.id === memberId || node.label === memberId || qualifiedName === normalized;
      })
    );
  };
  const addGroup = (group: UnknownRecord, packageGroup: boolean) => {
    const id = asString(group.id || group.qualifiedPackage || group.qualifiedName || group.label || group.name);
    if (!id || byId.has(id)) return;
    const label = asString(group.label || group.name || group.qualifiedPackage || group.qualifiedName || id, "package");
    const memberIds = groupMemberIds(group);
    const parentId = asString(group.parentId);
    const node: PreparedNode = {
      id,
      label,
      kind: "package",
      attributes: {
        isSyntheticContainer: true,
        isPackageContainer: packageGroup,
        containerId: parentId || null,
        memberIds,
        qualifiedName: asString(group.qualifiedPackage || group.qualifiedName || id),
      },
    };
    byId.set(id, node);
    nodes.push(node);
    for (const memberId of memberIds) {
      const member = resolveMember(memberId);
      if (!member) continue;
      const attrs = member.attributes ?? {};
      member.attributes = {
        ...attrs,
        containerId: attrs.containerId || id,
        _fallbackContainerId: id,
      };
    }
  };
  packageContainerGroups.forEach((group) => addGroup(group, true));
  containerGroups.forEach((group) => addGroup(group, false));
  resolveInterconnectionContainerIds(nodes);
  return pruneEmptySyntheticContainers(nodes);
}

function portsForPart(ports: UnknownRecord[], part: UnknownRecord): Array<Record<string, unknown> & { name: string }> {
  const id = asString(part.id ?? part.name);
  const name = asString(part.name);
  const qualifiedName = asString(part.qualifiedName).replace(/::/g, ".");
  return ports
    .filter((port) => {
      const parent = asString(port.partId ?? port.ownerId ?? port.containerId ?? port.parentId, "").replace(/::/g, ".");
      return parent === id || parent === name || parent === qualifiedName;
    })
    .map((port) => ({
      ...port,
      name: asString(port.name ?? port.id),
      id: asString(port.id ?? port.name),
      parentId: asString(port.parentId ?? port.partId ?? port.ownerId ?? port.containerId),
    }))
    .filter((port) => Boolean(port.name));
}

export function prepareInterconnection(visualization: VisualizationPayload): PreparedView {
  const ibd = asRecord(visualization.ibd);
  const selectedName = asString(visualization.selectedViewName ?? ibd.defaultRoot);
  const rootViews = asRecord(ibd.rootViews);
  const rootKeys = Object.keys(rootViews);
  const scopedName =
    selectedName && rootViews[selectedName]
      ? selectedName
      : rootKeys.length > 0
        ? rootKeys[0]
        : selectedName;
  const scoped = scopedName && rootViews[scopedName] ? asRecord(rootViews[scopedName]) : ibd;
  const parts = asArray(scoped.parts ?? ibd.parts).map(asRecord);
  const ports = asArray(scoped.ports ?? ibd.ports).map(asRecord);
  const connectors = asArray(scoped.connectors ?? ibd.connectors).map(asRecord);
  const containerGroups = asArray(scoped.containerGroups ?? ibd.containerGroups).map(asRecord);
  const packageContainerGroups = asArray(scoped.packageContainerGroups ?? ibd.packageContainerGroups).map(asRecord);
  const baseNodes = parts.map((part) => {
    const partId = asString(part.id ?? part.name);
    const parent = asString(part.containerId ?? part.parentId, "");
    const portDetails = portsForPart(ports, part);
    const partKind = asString(part.type, "part");
    return {
      id: partId,
      label: asString(part.name ?? part.id, "Unnamed"),
      kind: partKind,
      sourcePath: asString(part.sourcePath) || null,
      range: (part.range as { start?: { line?: number } } | null | undefined) ?? null,
      attributes: {
        ...asRecord(part.attributes),
        containerId: parent || null,
        qualifiedName: asString(part.qualifiedName),
        partType: firstPresent(
          asRecord(part.attributes).partType,
          asRecord(part.attributes).type,
          asRecord(part.attributes).typedBy,
          part.partType,
        ),
        children: asArray(part.children),
        ports: portDetails.map((port) => port.name),
        portDetails,
        isDefinition: isDefinitionKind(partKind),
        isReference: isReferenceKind(partKind),
      },
    };
  });
  const scopedRootPart = scopedName ? findScopedRootPart(baseNodes, scopedName) : undefined;
  const scopedContainerGroups = scopedRootPart
    ? filterContainerGroupsForScopedRoot(containerGroups, baseNodes, scopedName)
    : containerGroups;
  const scopedPackageGroups = scopedRootPart ? [] : packageContainerGroups;
  let nodes = synthesizeInterconnectionContainers(baseNodes, scopedContainerGroups, scopedPackageGroups);
  nodes = scopedRootPart ? collapseRedundantOuterBoundaries(nodes, scopedName) : nodes;
  const nodeIds = new Set(nodes.map((node) => node.id));
  const nodeById = new Map(nodes.map((node) => [node.id, node]));
  const concreteNodes = nodes.filter((node) => !asRecord(node.attributes).isSyntheticContainer);
  const resolveEndpointPartId = (explicit: unknown, endpoint: unknown): string => {
    const explicitText = asString(explicit).replace(/::/g, ".").trim();
    if (explicitText) {
      const directById = concreteNodes.find((node) => {
        const attrs = asRecord(node.attributes);
        const aliases = [node.id, node.label, asString(attrs.qualifiedName)]
          .filter(Boolean)
          .map((alias) => alias.replace(/::/g, "."));
        return aliases.includes(explicitText);
      });
      if (directById) return directById.id;
      if (nodeIds.has(explicitText)) return explicitText;
    }
    const endpointText = asString(endpoint).replace(/::/g, ".").trim();
    if (!endpointText) return explicitText;
    const direct = concreteNodes.find((node) => {
      const attrs = asRecord(node.attributes);
      return [node.id, node.label, asString(attrs.qualifiedName).replace(/::/g, ".")]
        .filter(Boolean)
        .includes(endpointText);
    });
    if (direct) return direct.id;
    const best = concreteNodes
      .map((node) => {
        const qn = asString(asRecord(node.attributes).qualifiedName, node.label).replace(/::/g, ".").trim();
        const aliases = [qn, node.label, node.id].filter(Boolean);
        const matched = aliases
          .filter((alias) => endpointText === alias || endpointText.startsWith(`${alias}.`))
          .sort((a, b) => b.length - a.length)[0];
        return matched ? { node, score: matched.length } : null;
      })
      .filter((value): value is { node: (typeof nodes)[number]; score: number } => Boolean(value))
      .sort((a, b) => b.score - a.score)[0];
    return best?.node.id ?? explicitText;
  };
  const edges = connectors
    .map((connector, index) => {
      const sourceEndpoint = firstPresent(connector.sourceId, connector.source);
      const targetEndpoint = firstPresent(connector.targetId, connector.target);
      const source = resolveEndpointPartId(firstPresent(connector.sourcePartId, connector.sourcePortPartId), sourceEndpoint);
      const target = resolveEndpointPartId(firstPresent(connector.targetPartId, connector.targetPortPartId), targetEndpoint);
      const type = ibdConnectorKind(connector);
      const label = ibdConnectorLabel(connector, type);
      return {
        id: asString(connector.id, `connector-${index}`),
        source,
        target,
        label,
        edgeKind: normalizeEdgeKind(type),
        attributes: {
          ...asRecord(connector.attributes),
          sourceId: asString(sourceEndpoint),
          targetId: asString(targetEndpoint),
          itemType: asString(connector.itemType),
          interfaceName: asString(connector.interfaceName ?? connector.interfaceType ?? connector.interfaceDefinition),
          relationType: type,
        },
      };
    })
    .filter((edge) => nodeById.has(edge.source) && nodeById.has(edge.target));
  const rootCandidates = asArray(ibd.rootCandidates).map((value) => asString(value)).filter(Boolean);
  return {
    title: scopedName || selectedName || "Interconnection View",
    view: "interconnection-view",
    nodes,
    edges,
    meta: {
      selectedRoot: scopedName || null,
      rootCandidates,
      containerGroups,
      packageContainerGroups,
    },
  };
}

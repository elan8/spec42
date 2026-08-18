import type { PreparedNode } from "../types";
import { asRecord, asString } from "../util";

/** Registers every alias a node can be referenced by (id, label, qualified name — dotted or
 * `::`-joined, and its last segment) so edge endpoints written in any of those forms resolve to
 * the same node id. Shared by activity and state-machine preparation. */
export function buildActivityNodeAliasMap(nodes: PreparedNode[]): Map<string, string> {
  const aliases = new Map<string, string>();
  const register = (alias: unknown, nodeId: string) => {
    const key = asString(alias).trim();
    if (!key) return;
    if (!aliases.has(key)) aliases.set(key, nodeId);
    const normalized = key.replace(/::/g, ".");
    if (!aliases.has(normalized)) aliases.set(normalized, nodeId);
    const lastSegment = normalized.split(".").filter(Boolean).pop();
    if (lastSegment && !aliases.has(lastSegment)) aliases.set(lastSegment, nodeId);
  };
  for (const node of nodes) {
    register(node.id, node.id);
    register(node.label, node.id);
    register(asRecord(node.attributes).qualifiedName, node.id);
  }
  return aliases;
}

export function resolveActivityNodeRef(value: unknown, aliases: Map<string, string>): string {
  const key = asString(value).trim();
  if (!key) return "";
  const normalized = key.replace(/::/g, ".");
  const segments = normalized.split(".").filter(Boolean);
  const last = segments[segments.length - 1] || "";
  const first = segments[0] || "";
  return (
    aliases.get(key) ??
    aliases.get(normalized) ??
    (last ? aliases.get(last) : undefined) ??
    (first ? aliases.get(first) : undefined) ??
    key
  );
}

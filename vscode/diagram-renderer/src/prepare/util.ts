import { isPackageElementType } from "../graph-normalization";
import type { PreparedNode, UnknownRecord } from "./types";

export function asRecord(value: unknown): UnknownRecord {
  return value && typeof value === "object" ? (value as UnknownRecord) : {};
}

export function asArray(value: unknown): unknown[] {
  return Array.isArray(value) ? value : [];
}

export function asString(value: unknown, fallback = ""): string {
  if (typeof value === "string") return value;
  if (typeof value === "number" || typeof value === "boolean") return String(value);
  return fallback;
}

export function elementTypeOf(node: UnknownRecord): string {
  const attrs = asRecord(node.attributes);
  return asString(
    node.type ??
      node.element_type ??
      node.element_kind ??
      attrs.element_type ??
      attrs.element_kind ??
      attrs.elementKind,
  );
}

export function isPackage(node: UnknownRecord): boolean {
  return isPackageElementType(elementTypeOf(node));
}

export function nodeUri(node: UnknownRecord): string | null {
  return asString(node.uri ?? node.sourcePath ?? node.source_path) || null;
}

export function nodeRange(node: UnknownRecord): PreparedNode["range"] {
  return (node.range as PreparedNode["range"]) ?? null;
}

export function buildBehaviorNode(
  node: UnknownRecord,
  index: number,
  defaults: { id: string; label: string; kind: string },
): PreparedNode {
  const attrs = asRecord(node.attributes);
  const qualifiedName = asString(node.qualifiedName ?? attrs.qualifiedName ?? node.id);
  return {
    id: asString(node.id ?? node.name, defaults.id),
    label: asString(node.name ?? node.label ?? node.id, defaults.label),
    kind: defaults.kind,
    sourcePath: nodeUri(node),
    uri: nodeUri(node),
    range: nodeRange(node),
    attributes: {
      ...attrs,
      ...(qualifiedName ? { qualifiedName } : {}),
      ...(node.parentId != null ? { parentId: node.parentId } : {}),
      ...(node.parent != null ? { parent: node.parent } : {}),
    },
  };
}

export function isSyntheticPackage(node: UnknownRecord): boolean {
  if (!isPackage(node)) return false;
  const attrs = asRecord(node.attributes);
  return Boolean(node.synthetic ?? node.isSynthetic ?? attrs.synthetic ?? attrs.isSyntheticContainer);
}

export function firstPresent(...values: unknown[]): unknown {
  return values.find((value) => value != null && asString(value).trim() !== "");
}

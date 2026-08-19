import type { NodeNotationRole } from "../node-notation";

/** Compatibility decoding for pre-schema-v2 payloads. Never used for typed diagram products. */
export function legacyNotationRole(kind: string): NodeNotationRole {
  const normalized = kind.trim().toLowerCase();
  if (normalized === "ref" || normalized.endsWith("-ref") || normalized.endsWith(" ref")) {
    return "reference-usage";
  }
  if (normalized.includes(" def") || normalized.includes("_def") || normalized.includes("definition")) {
    return "definition";
  }
  if (normalized === "package") return "namespace";
  return "usage";
}

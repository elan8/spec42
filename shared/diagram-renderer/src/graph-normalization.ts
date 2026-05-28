export type NormalizedEdgeKind =
  | "relationship"
  | "connection"
  | "satisfy"
  | "verify"
  | "typing"
  | "specializes"
  | "bind"
  | "allocate"
  | "transition"
  | "hierarchy"
  | string;

export interface ArchitectureLikeElement {
  qualified_name?: string | null;
  source_file?: string | null;
}

export function normalizeEdgeKind(relationshipType: string): NormalizedEdgeKind {
  const type = relationshipType.trim().toLowerCase();
  if (!type) return "relationship";
  if (type.includes("connection") || type === "connect") return "connection";
  if (type === "satisfy") return "satisfy";
  if (type === "verify") return "verify";
  if (type === "typing") return "typing";
  if (type === "specializes" || type === "specialization") return "specializes";
  if (type === "bind" || type === "binding") return "bind";
  if (type === "allocate" || type === "allocation") return "allocate";
  if (type === "transition") return "transition";
  if (type === "hierarchy" || type === "contains" || type === "owns") return "hierarchy";
  return type.replace(/[^a-z0-9_-]+/g, "_") || "relationship";
}

export function isPackageElementType(elementType: string): boolean {
  const normalized = elementType.trim().toLowerCase();
  return (
    !normalized ||
    normalized === "package" ||
    normalized.endsWith("_package") ||
    normalized.includes("package_def")
  );
}

export function isNonDiagramSemanticElementType(elementType: string): boolean {
  const normalized = elementType.trim().toLowerCase();
  if (!normalized) return true;
  return normalized === "import" || normalized === "diagnostic" || normalized.includes("diagnostic");
}

export function isOverviewVisualElementType(elementType: string): boolean {
  return !isPackageElementType(elementType) && !isNonDiagramSemanticElementType(elementType);
}

export function isArchitectureElement(element: ArchitectureLikeElement): boolean {
  const qn = element.qualified_name ?? "";
  if (qn.startsWith("Architecture::") || qn.includes("::Architecture::")) {
    return true;
  }
  const source = element.source_file?.trim();
  return source === "Architecture.sysml" || source?.endsWith("/Architecture.sysml") === true;
}

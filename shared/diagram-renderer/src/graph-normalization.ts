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
  if (type.includes("item_flow") || type.includes("item flow") || type === "flow" || type.includes("flow")) return "flow";
  if (type.includes("interface-connection") || type.includes("interface connection")) return "interface";
  if (type.includes("interface")) return "interface";
  if (type.includes("binding-connection") || type.includes("binding connection")) return "bind";
  if (type.includes("connection") || type === "connect") return "connection";
  if (type.includes("reference") || type === "ref") return "reference";
  if (type.includes("satisfy")) return "satisfy";
  if (type.includes("verify")) return "verify";
  if (type.includes("derivation") || type.includes("derive")) return "derivation";
  if (type === "typing" || type === "defined_by" || type === "defined by" || type === "definition") return "typing";
  if (type === "dependency" || type.includes("depend") || type.includes("binary-dependency")) return "dependency";
  if (type === "usage" || type === "usage-relationship") return "usage";
  if (type.includes("redefin")) return "redefinition";
  if (type === "specializes" || type === "specialization") return "specializes";
  if (type === "bind" || type === "binding") return "bind";
  if (type === "allocate" || type === "allocation") return "allocate";
  if (type === "transition") return "transition";
  if (type === "composition") return "composition";
  if (type === "hierarchy" || type === "contains" || type === "owns" || type === "ownership" || type === "containment") return "hierarchy";
  return type.replace(/[^a-z0-9_-]+/g, "_") || "relationship";
}

export function isPackageElementType(elementType: string): boolean {
  const normalized = elementType.trim().toLowerCase();
  return (
    !normalized ||
    normalized === "package" ||
    normalized === "library package" ||
    normalized.endsWith("_package") ||
    normalized.includes("package_def")
  );
}

export function isNonDiagramSemanticElementType(elementType: string): boolean {
  const normalized = elementType.trim().toLowerCase();
  if (!normalized) return true;
  // Documentation/comments annotate their owner (SysML §7.4); Spec42 ships them as
  // compartment text only — never as standalone General View boxes ("Unnamed").
  return (
    normalized === "import" ||
    normalized === "documentation" ||
    normalized === "comment" ||
    normalized === "diagnostic" ||
    normalized.includes("diagnostic")
  );
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

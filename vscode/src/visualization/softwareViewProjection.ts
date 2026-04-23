import type {
  GraphNodeDTO,
  GraphEdgeDTO,
  SoftwareWorkspaceModelDTO,
  SysMLGraphDTO,
  WorkspaceModelDTO,
} from "../providers/sysmlModelTypes";

export type SoftwareViewId = "software-module-view" | "software-dependency-view";

export interface SoftwareProjectionResult {
  graph: SysMLGraphDTO;
  workspaceModel: WorkspaceModelDTO;
}

function zeroRange() {
  return {
    start: { line: 0, character: 0 },
    end: { line: 0, character: 0 },
  };
}

export function projectSoftwareWorkspaceModel(
  model: SoftwareWorkspaceModelDTO,
  view: SoftwareViewId
): SoftwareProjectionResult {
  const nodes: GraphNodeDTO[] = model.architecture.components.map((component) => ({
    id: component.id,
    type: component.kind,
    name: component.name,
    uri: component.anchors[0]?.filePath ? toFileUri(component.anchors[0].filePath) : undefined,
    parentId: view === "software-module-view" ? component.parentId : undefined,
    range: component.anchors[0]?.range ?? zeroRange(),
    attributes: {
      crateName: component.crateName,
      modulePath: component.modulePath,
      isExternal: component.isExternal,
      kind: component.kind,
    },
  }));

  const edges: GraphEdgeDTO[] = view === "software-module-view"
    ? model.architecture.components
        .filter((component) => !!component.parentId)
        .map((component) => ({
          source: component.parentId!,
          target: component.id,
          type: "contains",
        }))
    : dedupeDependencyEdges(
        model.architecture.dependencies.map((dependency) => ({
          source: dependency.from,
          target: dependency.to,
          type: dependency.kind,
        }))
      );

  return {
    graph: { nodes, edges },
    workspaceModel: {
      files: [],
      semantic: [],
      summary: {
        scannedFiles: model.architecture.components.length,
        loadedFiles: model.architecture.components.length,
        failures: 0,
        truncated: false,
      },
    },
  };
}

function toFileUri(filePath: string): string {
  const normalized = filePath.replace(/\\/g, "/");
  return normalized.startsWith("file:///")
    ? normalized
    : `file:///${normalized.replace(/^([A-Za-z]):/, (_, drive) => `${drive}:`)}`;
}

function dedupeDependencyEdges(edges: GraphEdgeDTO[]): GraphEdgeDTO[] {
  const seen = new Set<string>();
  return edges.filter((edge) => {
    const key = `${edge.source}|${edge.target}|${edge.type ?? edge.rel_type ?? ""}`;
    if (seen.has(key)) {
      return false;
    }
    seen.add(key);
    return true;
  });
}

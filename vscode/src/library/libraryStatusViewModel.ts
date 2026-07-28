export type LibrarySourceKind = "standard" | "domain" | "custom";

export type LibrarySearchItemLike = {
  name: string;
  kind: string;
  container?: string;
  uri: string;
  range: unknown;
  score?: number;
  source: LibrarySourceKind;
  path?: string;
};

export type LibrarySearchPackageLike = {
  name: string;
  path: string;
  source: LibrarySourceKind;
  symbols: LibrarySearchItemLike[];
};

export type LibrarySearchSourceLike = {
  source: LibrarySourceKind;
  packages: LibrarySearchPackageLike[];
};

export type LibrarySearchResultLike = {
  sources: LibrarySearchSourceLike[];
  symbolTotal?: number;
  total?: number;
};

export type SysandStatusViewModel = {
  installed: boolean;
  version?: string;
  executablePath?: string;
  projectRoot?: string;
  manifestPresent: boolean;
  lockPresent: boolean;
  dependencyRoots: string[];
  warnings: string[];
};

export type KparLibraryHeading = {
  id: string;
  displayName: string;
  pinnedVersion: string;
  format: string;
};

export type KparLibraryRuntimeStatus = {
  id: string;
  displayName?: string;
  resolvedPath?: string;
  sourceKind: string;
  pinnedVersion?: string;
  installedVersion?: string;
  isInstalled?: boolean;
  versionMatches?: boolean;
};

/** @deprecated Prefer {@link KparLibraryRuntimeStatus}. */
export type KparLibraryDoctorStatus = KparLibraryRuntimeStatus;

/** @deprecated Prefer {@link KparLibraryStatusViewModel}; kept for domain-specific callers. */
export type DomainLibrariesStatusViewModel = KparLibraryStatusViewModel;

export type KparLibraryStatusViewModel = {
  id: string;
  displayName: string;
  pinnedVersion: string;
  installedVersion?: string;
  format: string;
  available: boolean;
  versionMatches: boolean;
  resolvedPath?: string;
  sourceKind: string;
  /** Search-index counts only apply to the domain bucket today. */
  packageCount?: number;
  symbolCount?: number;
};

export type LibraryDashboardStatus = {
  stdlib: {
    pinnedVersion: string;
    format: string;
    available: boolean;
    packageCount: number;
    symbolCount: number;
  };
  kparLibraries: KparLibraryStatusViewModel[];
  /** Convenience alias for the `domain` KPAR entry (if present). */
  domain: KparLibraryStatusViewModel;
  custom: {
    configuredPaths: string[];
    missingPaths: string[];
    packageCount: number;
    symbolCount: number;
  };
  sysand: SysandStatusViewModel;
};

export type LibrarySummary = {
  standardPackages: number;
  standardSymbols: number;
  domainPackages: number;
  domainSymbols: number;
  customPackages: number;
  customSymbols: number;
  totalSymbols: number;
};

export type LibraryResultRow = {
  name: string;
  kind: string;
  packageName: string;
  container?: string;
  source: LibrarySourceKind;
  path?: string;
  uri: string;
  range: unknown;
  qualifiedName: string;
  importStatement: string;
  score: number;
};

export function summarizeLibrarySearch(result: LibrarySearchResultLike): LibrarySummary {
  const summary: LibrarySummary = {
    standardPackages: 0,
    standardSymbols: 0,
    domainPackages: 0,
    domainSymbols: 0,
    customPackages: 0,
    customSymbols: 0,
    totalSymbols: 0,
  };
  for (const source of result.sources ?? []) {
    for (const pkg of source.packages ?? []) {
      const symbolCount = pkg.symbols?.length ?? 0;
      if (source.source === "standard") {
        summary.standardPackages += 1;
        summary.standardSymbols += symbolCount;
      } else if (source.source === "domain") {
        summary.domainPackages += 1;
        summary.domainSymbols += symbolCount;
      } else {
        summary.customPackages += 1;
        summary.customSymbols += symbolCount;
      }
      summary.totalSymbols += symbolCount;
    }
  }
  return summary;
}

export function classifySysandStatus(status: SysandStatusViewModel): {
  label: string;
  severity: "ok" | "info" | "warning";
  details: string[];
} {
  const details: string[] = [];
  if (status.version) {
    details.push(status.version);
  }
  if (status.projectRoot) {
    details.push(`project: ${status.projectRoot}`);
  } else {
    details.push("no project manifest");
  }
  details.push(`${status.dependencyRoots.length} dependency root(s)`);
  if (status.lockPresent) {
    details.push("lockfile present");
  }

  if (!status.installed && status.manifestPresent) {
    return {
      label: "Project detected, Sysand not installed",
      severity: "warning",
      details,
    };
  }
  if (status.warnings.length > 0) {
    return {
      label: status.installed ? "Sysand needs attention" : "Sysand optional",
      severity: "warning",
      details,
    };
  }
  if (status.installed) {
    return {
      label: status.projectRoot ? "Sysand project ready" : "Sysand installed",
      severity: "ok",
      details,
    };
  }
  return {
    label: "Sysand not installed",
    severity: "info",
    details,
  };
}

function normalizeQuery(value: string): string {
  return value.trim().toLowerCase();
}

function rowRank(name: string, query: string, serverScore: number | undefined): number {
  if (!query) {
    return serverScore ?? 0;
  }
  const lower = name.toLowerCase();
  if (lower === query) {
    return 1_000_000 + (serverScore ?? 0);
  }
  if (lower.startsWith(query)) {
    return 500_000 + (serverScore ?? 0);
  }
  if (lower.includes(query)) {
    return 100_000 + (serverScore ?? 0);
  }
  return serverScore ?? 0;
}

export function flattenLibrarySearchResults(
  result: LibrarySearchResultLike,
  query = ""
): LibraryResultRow[] {
  const normalizedQuery = normalizeQuery(query);
  const rows: LibraryResultRow[] = [];
  for (const source of result.sources ?? []) {
    for (const pkg of source.packages ?? []) {
      for (const item of pkg.symbols ?? []) {
        const qualifiedName = item.container
          ? `${item.container}::${item.name}`
          : `${pkg.name}::${item.name}`;
        rows.push({
          name: item.name,
          kind: item.kind,
          packageName: pkg.name,
          container: item.container,
          source: item.source ?? source.source,
          path: item.path ?? pkg.path,
          uri: item.uri,
          range: item.range,
          qualifiedName,
          importStatement: `public import ${pkg.name}::${item.name};`,
          score: rowRank(item.name, normalizedQuery, item.score),
        });
      }
    }
  }
  return rows.sort((a, b) => {
    if (b.score !== a.score) {
      return b.score - a.score;
    }
    return a.name.localeCompare(b.name);
  });
}

export function buildKparLibraryStatusViewModel(
  heading: KparLibraryHeading,
  runtime?: KparLibraryRuntimeStatus
): KparLibraryStatusViewModel {
  const pinnedVersion = runtime?.pinnedVersion || heading.pinnedVersion;
  const installedVersion = runtime?.installedVersion;
  const sourceKind = runtime?.sourceKind ?? "none";
  const resolvedPath = runtime?.resolvedPath;
  const versionMatches =
    runtime?.versionMatches ??
    (!installedVersion || installedVersion === pinnedVersion);
  const available =
    runtime?.isInstalled === true ||
    !!resolvedPath ||
    sourceKind === "bundled" ||
    sourceKind === "canonical-managed" ||
    sourceKind === "override";

  return {
    id: heading.id,
    displayName: runtime?.displayName || heading.displayName,
    pinnedVersion,
    installedVersion,
    format: heading.format,
    available,
    versionMatches,
    resolvedPath,
    sourceKind,
  };
}

function emptyDomainFallback(format: string): KparLibraryStatusViewModel {
  return {
    id: "domain",
    displayName: "Domain libraries",
    pinnedVersion: "unknown",
    format,
    available: false,
    versionMatches: false,
    sourceKind: "none",
    packageCount: 0,
    symbolCount: 0,
  };
}

export function buildLibraryDashboardStatus(params: {
  pinnedVersion: string;
  format: string;
  kparHeadings: KparLibraryHeading[];
  kparStatuses: KparLibraryRuntimeStatus[];
  configuredPaths: string[];
  missingPaths: string[];
  summary: LibrarySummary;
  sysand: SysandStatusViewModel;
}): LibraryDashboardStatus {
  const statusById = new Map(
    params.kparStatuses.map((status) => [status.id, status] as const)
  );
  const kparLibraries = params.kparHeadings.map((heading) => {
    const status = buildKparLibraryStatusViewModel(heading, statusById.get(heading.id));
    if (heading.id === "domain") {
      return {
        ...status,
        packageCount: params.summary.domainPackages,
        symbolCount: params.summary.domainSymbols,
      };
    }
    return status;
  });

  const domain =
    kparLibraries.find((library) => library.id === "domain") ??
    emptyDomainFallback(params.format);

  return {
    stdlib: {
      pinnedVersion: params.pinnedVersion,
      format: params.format,
      available: true,
      packageCount: params.summary.standardPackages,
      symbolCount: params.summary.standardSymbols,
    },
    kparLibraries,
    domain,
    custom: {
      configuredPaths: params.configuredPaths,
      missingPaths: params.missingPaths,
      packageCount: params.summary.customPackages,
      symbolCount: params.summary.customSymbols,
    },
    sysand: params.sysand,
  };
}

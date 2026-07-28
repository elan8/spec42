import * as assert from "assert";
import {
  buildLibraryDashboardStatus,
  classifySysandStatus,
  flattenLibrarySearchResults,
  summarizeLibrarySearch,
} from "../../library/libraryStatusViewModel";
import { KPAR_LIBRARIES_DEFAULTS, kparLibraryDefaults } from "../../generated/kparLibrariesDefaults";
import { STANDARD_LIBRARY_DEFAULTS } from "../../generated/standardLibraryDefaults";

const DOMAIN_LIBRARIES_DEFAULTS = kparLibraryDefaults("domain")!;
const METHOD_LIBRARIES_DEFAULTS = kparLibraryDefaults("method")!;

describe("libraryStatusViewModel", () => {
  it("summarizes standard, domain, and custom library packages", () => {
    const summary = summarizeLibrarySearch({
      sources: [
        {
          source: "standard",
          packages: [
            { name: "ScalarValues", path: "stdlib", source: "standard", symbols: [
              { name: "Real", kind: "part def", uri: "file:///stdlib.sysml", range: {}, source: "standard" },
            ] },
          ],
        },
        {
          source: "domain",
          packages: [
            { name: "RequirementMetadata", path: "domain", source: "domain", symbols: [
              { name: "RequirementMetadata", kind: "metadata def", uri: "file:///domain.sysml", range: {}, source: "domain" },
            ] },
          ],
        },
        {
          source: "custom",
          packages: [
            { name: "Domain", path: "domain", source: "custom", symbols: [
              { name: "Vehicle", kind: "part def", uri: "file:///domain.sysml", range: {}, source: "custom" },
              { name: "Wheel", kind: "part def", uri: "file:///domain.sysml", range: {}, source: "custom" },
            ] },
          ],
        },
      ],
    });

    assert.strictEqual(summary.standardPackages, 1);
    assert.strictEqual(summary.standardSymbols, 1);
    assert.strictEqual(summary.domainPackages, 1);
    assert.strictEqual(summary.domainSymbols, 1);
    assert.strictEqual(summary.customPackages, 1);
    assert.strictEqual(summary.customSymbols, 2);
  });

  it("classifies Sysand project without executable as warning", () => {
    const classified = classifySysandStatus({
      installed: false,
      projectRoot: "C:/project",
      manifestPresent: true,
      lockPresent: false,
      dependencyRoots: [],
      warnings: ["Sysand executable was not found on PATH"],
    });

    assert.strictEqual(classified.severity, "warning");
    assert.strictEqual(classified.label, "Project detected, Sysand not installed");
  });

  it("sorts exact and prefix search rows above broad matches", () => {
    const rows = flattenLibrarySearchResults(
      {
        sources: [
          {
            source: "custom",
            packages: [
              {
                name: "Domain",
                path: "domain",
                source: "custom",
                symbols: [
                  { name: "MyVehicle", kind: "part def", uri: "file:///a.sysml", range: {}, source: "custom", score: 10 },
                  { name: "VehicleController", kind: "part def", uri: "file:///a.sysml", range: {}, source: "custom", score: 1 },
                  { name: "Vehicle", kind: "part def", uri: "file:///a.sysml", range: {}, source: "custom", score: 0 },
                ],
              },
            ],
          },
        ],
      },
      "Vehicle"
    );

    assert.deepStrictEqual(rows.map((row) => row.name), [
      "Vehicle",
      "VehicleController",
      "MyVehicle",
    ]);
    assert.strictEqual(rows[0].importStatement, "public import Domain::Vehicle;");
  });

  it("builds dashboard status with domain and method KPAR sections", () => {
    const status = buildLibraryDashboardStatus({
      pinnedVersion: STANDARD_LIBRARY_DEFAULTS.version,
      format: STANDARD_LIBRARY_DEFAULTS.format,
      kparHeadings: KPAR_LIBRARIES_DEFAULTS.map((library) => ({
        id: library.id,
        displayName: library.displayName,
        pinnedVersion: library.version,
        format: library.format,
      })),
      kparStatuses: [
        {
          id: "domain",
          displayName: DOMAIN_LIBRARIES_DEFAULTS.displayName,
          resolvedPath: "C:/data/kpar-libraries/domain/versions/0.2.0",
          sourceKind: "bundled",
          pinnedVersion: DOMAIN_LIBRARIES_DEFAULTS.version,
          installedVersion: DOMAIN_LIBRARIES_DEFAULTS.version,
          isInstalled: true,
          versionMatches: true,
        },
        {
          id: "method",
          displayName: METHOD_LIBRARIES_DEFAULTS.displayName,
          resolvedPath: "C:/data/kpar-libraries/method/versions/0.1.1",
          sourceKind: "bundled",
          pinnedVersion: METHOD_LIBRARIES_DEFAULTS.version,
          installedVersion: METHOD_LIBRARIES_DEFAULTS.version,
          isInstalled: true,
          versionMatches: true,
        },
      ],
      configuredPaths: ["C:/libs/domain"],
      missingPaths: ["C:/libs/missing"],
      summary: {
        standardPackages: 1,
        standardSymbols: 10,
        domainPackages: 3,
        domainSymbols: 12,
        customPackages: 2,
        customSymbols: 5,
        totalSymbols: 27,
      },
      sysand: {
        installed: true,
        manifestPresent: true,
        lockPresent: true,
        dependencyRoots: ["C:/project/.sysand/packages"],
        warnings: [],
      },
    });

    assert.strictEqual(status.stdlib.available, true);
    assert.strictEqual(status.stdlib.format, STANDARD_LIBRARY_DEFAULTS.format);
    assert.strictEqual(status.stdlib.packageCount, 1);
    assert.strictEqual(status.stdlib.symbolCount, 10);
    assert.strictEqual(status.kparLibraries.length, 2);
    assert.strictEqual(status.kparLibraries[0].id, "domain");
    assert.strictEqual(status.kparLibraries[0].pinnedVersion, DOMAIN_LIBRARIES_DEFAULTS.version);
    assert.strictEqual(status.kparLibraries[0].available, true);
    assert.strictEqual(status.kparLibraries[0].packageCount, 3);
    assert.strictEqual(status.kparLibraries[1].id, "method");
    assert.strictEqual(status.kparLibraries[1].pinnedVersion, METHOD_LIBRARIES_DEFAULTS.version);
    assert.strictEqual(status.kparLibraries[1].available, true);
    assert.strictEqual(status.domain.pinnedVersion, DOMAIN_LIBRARIES_DEFAULTS.version);
    assert.strictEqual(status.custom.packageCount, 2);
    assert.deepStrictEqual(status.custom.missingPaths, ["C:/libs/missing"]);
    assert.strictEqual(status.sysand.lockPresent, true);
  });
});

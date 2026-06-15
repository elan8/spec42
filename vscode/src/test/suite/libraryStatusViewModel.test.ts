import * as assert from "assert";
import {
  buildLibraryDashboardStatus,
  classifySysandStatus,
  flattenLibrarySearchResults,
  summarizeLibrarySearch,
} from "../../library/libraryStatusViewModel";
import { DOMAIN_LIBRARIES_DEFAULTS } from "../../generated/domainLibrariesDefaults";
import { STANDARD_LIBRARY_DEFAULTS } from "../../generated/standardLibraryDefaults";

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

  it("builds dashboard status with domain section and missing custom paths", () => {
    const status = buildLibraryDashboardStatus({
      pinnedVersion: STANDARD_LIBRARY_DEFAULTS.version,
      format: STANDARD_LIBRARY_DEFAULTS.format,
      domainPinnedVersion: DOMAIN_LIBRARIES_DEFAULTS.version,
      domainFormat: DOMAIN_LIBRARIES_DEFAULTS.format,
      domainResolvedPath: "C:/data/domain-libraries/versions/dc378a9/tree",
      domainSourceKind: "bundled",
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
    assert.strictEqual(status.domain.pinnedVersion, DOMAIN_LIBRARIES_DEFAULTS.version);
    assert.strictEqual(status.domain.format, DOMAIN_LIBRARIES_DEFAULTS.format);
    assert.strictEqual(status.domain.available, true);
    assert.strictEqual(status.domain.packageCount, 3);
    assert.strictEqual(status.domain.symbolCount, 12);
    assert.strictEqual(status.custom.packageCount, 2);
    assert.deepStrictEqual(status.custom.missingPaths, ["C:/libs/missing"]);
    assert.strictEqual(status.sysand.lockPresent, true);
  });
});

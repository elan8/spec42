import * as assert from "assert";
import {
  buildFeatureInspectorViewModel,
  type FeatureInspectorViewModel,
} from "../../inspector/featureInspectorViewModel";
import type {
  FeatureInspectorElement,
  FeatureInspectorResult,
} from "../../providers/lspModelProvider";

function element(
  name: string,
  role: FeatureInspectorElement["role"],
  overrides: Partial<FeatureInspectorElement> = {}
): FeatureInspectorElement {
  return {
    id: `P::${name}`,
    name,
    qualifiedName: `P::${name}`,
    type: role === "definition" ? "part def" : "attribute",
    role,
    declaration: role === "definition" ? `part def ${name};` : `attribute ${name};`,
    uri: "file:///model.sysml",
    range: {
      start: { line: 1, character: 2 },
      end: { line: 1, character: 20 },
    },
    evaluation: { state: "notApplicable" },
    analysis: { state: "notApplicable" },
    typing: { status: "notApplicable", targets: [] },
    specialization: { status: "notApplicable", targets: [] },
    incomingRelationships: [],
    outgoingRelationships: [],
    ...overrides,
  };
}

function result(
  selection: FeatureInspectorResult["selection"],
  containingElement?: FeatureInspectorElement
): FeatureInspectorResult {
  return {
    version: 1,
    sourceUri: "file:///model.sysml",
    requestedPosition: { line: 1, character: 2 },
    selection,
    containingElement,
    referenced: { status: "none" },
  };
}

function assertMode(
  viewModel: FeatureInspectorViewModel,
  mode: FeatureInspectorViewModel["mode"]
): void {
  assert.strictEqual(viewModel.mode, mode);
}

describe("FeatureInspectorViewModel", () => {
  it("renders a keyword as language help only", () => {
    const input = result({ kind: "keyword", text: "part" }, element("RobotLidar", "definition"));
    input.languageHelp = {
      keyword: "part",
      description: "Defines or uses a part.",
      syntax: "part def Vehicle; or part vehicle : Vehicle;",
    };

    const viewModel = buildFeatureInspectorViewModel(input);

    assertMode(viewModel, "language");
    assert.deepStrictEqual(viewModel.sections, ["Language construct"]);
    assert.strictEqual(viewModel.languageHelp?.keyword, "part");
    assert.strictEqual(viewModel.primaryElement, undefined);
  });

  it("uses the fixed element-section order and the server role", () => {
    const target = element("RobotLidar", "definition", {
      specialization: { status: "resolved", targets: [] },
      incomingRelationships: [
        {
          type: "specializes",
          peer: {
            id: "P::Child",
            name: "Child",
            qualifiedName: "P::Child",
            type: "part def",
            uri: "file:///model.sysml",
            range: {
              start: { line: 3, character: 2 },
              end: { line: 3, character: 10 },
            },
          },
          provenance: "authored",
        },
      ],
    });

    const viewModel = buildFeatureInspectorViewModel(
      result({ kind: "element", text: "RobotLidar" }, target)
    );

    assertMode(viewModel, "element");
    assert.strictEqual(viewModel.primaryElement?.role, "definition");
    assert.deepStrictEqual(viewModel.sections, [
      "Model element",
      "Declaration",
      "Identity",
      "Type/specialization",
      "Relationships",
      "Source location",
    ]);
  });

  it("adds resolved semantic, inherited-feature, documentation, and metadata sections", () => {
    const base = element("Vehicle", "definition");
    const target = element("Rover", "definition", {
      documentation: "A specialized vehicle.",
      subsetting: { status: "notApplicable", targets: [] },
      redefinition: { status: "resolved", targets: [base] },
      inheritedFeatures: [{ feature: element("command", "usage"), declaredIn: base }],
      metadata: [element("SafetyCritical", "usage")],
    });

    const viewModel = buildFeatureInspectorViewModel(
      result({ kind: "element", text: "Rover" }, target)
    );

    assert.deepStrictEqual(viewModel.sections, [
      "Model element",
      "Declaration",
      "Identity",
      "Subsetting/redefinition",
      "Inherited features",
      "Documentation",
      "Metadata",
      "Source location",
    ]);
  });

  it("makes the resolved reference primary and preserves compact source context", () => {
    const source = element("RobotLidar", "definition", {
      declaration: "part def RobotLidar :> RPLIDARC1;",
    });
    const target = element("RPLIDARC1", "definition");
    const input = result({ kind: "reference", text: "RPLIDARC1" }, source);
    input.referenced = { status: "resolved", element: target };

    const viewModel = buildFeatureInspectorViewModel(input);

    assertMode(viewModel, "reference");
    assert.strictEqual(viewModel.primaryElement, target);
    assert.strictEqual(viewModel.referencedFrom, source);
  });

  it("creates one focused value card without relationship sections or duplicate fields", () => {
    const usage = element("scanRate", "usage", {
      declaration: "attribute scanRate : FrequencyValue = 10 [Hz]",
      evaluation: { state: "literal", value: 10, unit: "Hz" },
      typing: {
        status: "resolved",
        targets: [
          {
            id: "ISQ::FrequencyValue",
            name: "FrequencyValue",
            qualifiedName: "FrequencyValue",
            type: "AttributeDefinition",
            uri: "file:///library.sysml",
            range: {
              start: { line: 1, character: 0 },
              end: { line: 1, character: 14 },
            },
          },
        ],
      },
      outgoingRelationships: [
        {
          type: "typing",
          peer: {
            id: "ISQ::FrequencyValue",
            name: "FrequencyValue",
            qualifiedName: "ISQ::FrequencyValue",
            type: "attribute def",
            uri: "file:///library.sysml",
            range: {
              start: { line: 1, character: 0 },
              end: { line: 1, character: 14 },
            },
          },
          provenance: "authored",
        },
      ],
    });

    for (const [kind, text] of [
      ["value", "10"],
      ["unit", "Hz"],
    ] as const) {
      const viewModel = buildFeatureInspectorViewModel(
        result({ kind, text }, usage)
      );
      assertMode(viewModel, "value");
      assert.deepStrictEqual(viewModel.sections, ["Value"]);
      assert.strictEqual(viewModel.value?.declaredValue, "10 [Hz]");
      assert.strictEqual(viewModel.value?.evaluatedValue, "10 [Hz]");
      assert.strictEqual(viewModel.value?.unit, "Hz");
      assert.strictEqual(viewModel.value?.quantityType, "FrequencyValue");
    }
  });
});

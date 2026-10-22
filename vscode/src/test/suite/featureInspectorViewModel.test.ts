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
  it("leads with the qualified name, readable kind and original documentation", () => {
    const documentation = "Authorizes configuration changes.\n\n  Preserve this indentation.\n<script>plain text</script>";
    const target = element("configurationChange", "usage", {
      qualifiedName: "Elevators::OperationalScenarios::configurationChange", type: "ActionUsage", documentation,
    });
    const viewModel = buildFeatureInspectorViewModel(result({ kind: "element" }, target));
    assert.deepStrictEqual(viewModel.elementHeading, {
      name: target.qualifiedName, kind: "Action usage", documentation, hasDocumentation: true,
    });
    assert.deepStrictEqual(viewModel.sections.slice(0, 3), ["Model element", "Documentation", "Identity"]);
    assert.ok(!viewModel.sections.includes("Declaration"));
  });

  it("shows missing documentation and falls back to the supplied local or unnamed name", () => {
    for (const documentation of [undefined, "", "  \n "]) {
      const target = element("Vehicle", "definition", { qualifiedName: "", documentation });
      const heading = buildFeatureInspectorViewModel(result({ kind: "element" }, target)).elementHeading;
      assert.deepStrictEqual(heading, { name:"Vehicle", kind:"Part definition", documentation:"No documentation.", hasDocumentation:false });
      target.name = "";
      assert.strictEqual(buildFeatureInspectorViewModel(result({ kind:"element" }, target)).elementHeading?.name, "(unnamed)");
    }
  });

  it("shows the resolved target's heading and documentation for reference selections", () => {
    const input = result({kind:"reference"}, element("caller", "usage"));
    input.referenced = {status:"resolved", element:element("Operation", "definition", {documentation:"Target documentation."})};
    const heading = buildFeatureInspectorViewModel(input).elementHeading;
    assert.strictEqual(heading?.name, "P::Operation");
    assert.strictEqual(heading?.documentation, "Target documentation.");
  });

  it("keeps immediate features visible and ancestor features available for expansion", () => {
    const base = element("Base", "definition");
    const ancestor = element("Ancestor", "definition");
    const direct = { feature: element("command", "usage"), declaredIn: base, isDirect: true };
    const indirect = { feature: element("status", "usage"), declaredIn: ancestor, isDirect: false };
    const target = element("Derived", "definition", { inheritedFeatures: [direct, indirect] });
    const viewModel = buildFeatureInspectorViewModel(result({ kind: "element" }, target));
    assert.deepStrictEqual(viewModel.inheritedFeatures, { direct: [direct], indirect: [indirect] });

    const referenced = result({ kind: "reference" });
    referenced.referenced = { status: "resolved", element: target };
    assert.deepStrictEqual(buildFeatureInspectorViewModel(referenced).inheritedFeatures,
      { direct: [direct], indirect: [indirect] });
    target.inheritedFeatures = [indirect];
    const ancestorOnly = buildFeatureInspectorViewModel(result({ kind: "element" }, target));
    assert.deepStrictEqual(ancestorOnly.inheritedFeatures, { direct: [], indirect: [indirect] });
    assert.ok(ancestorOnly.sections.includes("Inherited features"));
  });

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
      "Documentation",
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
      inheritedFeatures: [{ feature: element("command", "usage"), declaredIn: base, isDirect: true }],
      metadata: [element("SafetyCritical", "usage")],
    });

    const viewModel = buildFeatureInspectorViewModel(
      result({ kind: "element", text: "Rover" }, target)
    );

    assert.deepStrictEqual(viewModel.sections, [
      "Model element",
      "Documentation",
      "Identity",
      "Subsetting/redefinition",
      "Inherited features",
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

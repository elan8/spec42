import type {
  FeatureInspectorElement,
  FeatureInspectorLanguageHelp,
  FeatureInspectorResult,
  FeatureInspectorSelectionKind,
} from "../providers/lspModelProvider";

export type FeatureInspectorMode =
  | "language"
  | "element"
  | "reference"
  | "value"
  | "empty";

export interface FeatureInspectorValueContext {
  selectionKind: "value" | "unit";
  selectionText?: string;
  declaration?: string;
  declaredValue?: string;
  evaluatedValue?: string;
  unit?: string;
  quantityType?: string;
}

export interface FeatureInspectorViewModel {
  mode: FeatureInspectorMode;
  sections: string[];
  languageHelp?: FeatureInspectorLanguageHelp;
  primaryElement?: FeatureInspectorElement;
  referencedFrom?: FeatureInspectorElement;
  value?: FeatureInspectorValueContext;
}

function attributeText(
  attributes: Record<string, unknown>,
  ...keys: string[]
): string | undefined {
  for (const key of keys) {
    const value = attributes[key];
    if (value === undefined || value === null || value === "") {
      continue;
    }
    if (Array.isArray(value)) {
      const text = value.filter(Boolean).join(", ");
      if (text) {
        return text;
      }
    } else if (typeof value !== "object") {
      return String(value);
    }
  }
  return undefined;
}

function elementSections(element: FeatureInspectorElement): string[] {
  const sections = ["Model element"];
  if (element.declaration) {
    sections.push("Declaration");
  }
  sections.push("Identity");
  if (
    element.typing.status !== "notApplicable" ||
    element.specialization.status !== "notApplicable"
  ) {
    sections.push("Type/specialization");
  }
  if (
    attributeText(
      element.attributes,
      "value",
      "defaultValue",
      "evaluatedValue",
      "evaluatedUnit"
    )
  ) {
    sections.push("Value");
  }
  if (
    element.incomingRelationships.length > 0 ||
    element.outgoingRelationships.length > 0
  ) {
    sections.push("Relationships");
  }
  sections.push("Source location");
  return sections;
}

function valueContext(
  result: FeatureInspectorResult,
  selectionKind: "value" | "unit",
  element: FeatureInspectorElement
): FeatureInspectorValueContext {
  const typingTarget = element.typing.targets[0];
  return {
    selectionKind,
    selectionText: result.selection.text,
    declaration: element.declaration,
    declaredValue: attributeText(element.attributes, "value", "defaultValue"),
    evaluatedValue: attributeText(element.attributes, "evaluatedValue"),
    unit: attributeText(element.attributes, "evaluatedUnit", "unit"),
    quantityType:
      typingTarget?.qualifiedName ||
      attributeText(
        element.attributes,
        "attributeType",
        "itemType",
        "partType",
        "refType"
      ),
  };
}

export function buildFeatureInspectorViewModel(
  result: FeatureInspectorResult | null | undefined
): FeatureInspectorViewModel {
  if (!result) {
    return { mode: "empty", sections: [] };
  }

  if (result.selection.kind === "keyword" && result.languageHelp) {
    return {
      mode: "language",
      sections: ["Language construct"],
      languageHelp: result.languageHelp,
    };
  }

  if (result.selection.kind === "reference" && result.referencedElement) {
    return {
      mode: "reference",
      sections: elementSections(result.referencedElement),
      primaryElement: result.referencedElement,
      referencedFrom: result.containingElement,
    };
  }

  if (
    (result.selection.kind === "value" || result.selection.kind === "unit") &&
    result.containingElement
  ) {
    const selectionKind = result.selection.kind as Extract<
      FeatureInspectorSelectionKind,
      "value" | "unit"
    >;
    return {
      mode: "value",
      sections: ["Value"],
      primaryElement: result.containingElement,
      value: valueContext(result, selectionKind, result.containingElement),
    };
  }

  if (result.containingElement) {
    return {
      mode: "element",
      sections: elementSections(result.containingElement),
      primaryElement: result.containingElement,
    };
  }

  return { mode: "empty", sections: [] };
}

import type {
  FeatureInspectorAnalysis,
  FeatureInspectorElement,
  FeatureInspectorEvaluation,
  FeatureInspectorLanguageHelp,
  FeatureInspectorResult,
  FeatureInspectorSelectionKind,
} from "../providers/lspModelProvider";
import { resolvedReference } from "../providers/lspModelProvider";

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

/** The evaluated value with its unit, when the publication settled on one.
 *
 * Only the states that carry a value produce text here; the others are shown by their state
 * rather than as an empty value, so a failed or non-constant expression is never rendered as a
 * successful blank. */
export function evaluatedValueText(
  evaluation: FeatureInspectorEvaluation | undefined
): string | undefined {
  if (
    !evaluation ||
    (evaluation.state !== "literal" && evaluation.state !== "evaluated")
  ) {
    return undefined;
  }
  const value = String(evaluation.value);
  return evaluation.unit ? `${value} [${evaluation.unit}]` : value;
}

/** A short label for an evaluation state that carries no value. */
export function evaluationStateLabel(
  evaluation: FeatureInspectorEvaluation | undefined
): string | undefined {
  switch (evaluation?.state) {
    case "notRun":
      return "not evaluated";
    case "nonConstant":
      return "not constant";
    case "cyclic":
      return "cyclic value";
    case "unsupported":
      return "unsupported expression";
    case "failed":
      return `evaluation failed (${evaluation.reason})`;
    default:
      return undefined;
  }
}

/** A short label for the verdict channel, absent when the element states no verdict. */
export function analysisLabel(
  analysis: FeatureInspectorAnalysis | undefined
): string | undefined {
  switch (analysis?.state) {
    case "verdict":
      return analysis.passed ? "passed" : "failed";
    case "computed":
      return analysis.unit
        ? `${String(analysis.value)} [${analysis.unit}]`
        : String(analysis.value);
    case "notRun":
      return "not evaluated";
    case "unsettled":
      return `not settled (${analysis.evaluation})`;
    default:
      return undefined;
  }
}

function elementSections(element: FeatureInspectorElement): string[] {
  const sections = ["Model element"];
  if (element.declaration) {
    sections.push("Declaration");
  }
  sections.push("Identity");
  if (
    element.typing.status !== "notApplicable" ||
    (element.effectiveTyping !== undefined &&
      element.effectiveTyping.status !== "notApplicable") ||
    element.specialization.status !== "notApplicable"
  ) {
    sections.push("Type/specialization");
  }
  if (
    (element.subsetting !== undefined &&
      element.subsetting.status !== "notApplicable") ||
    (element.redefinition !== undefined &&
      element.redefinition.status !== "notApplicable")
  ) {
    sections.push("Subsetting/redefinition");
  }
  if (element.inheritedFeatures?.length) {
    sections.push("Inherited features");
  }
  if (element.documentation) {
    sections.push("Documentation");
  }
  if (element.metadata?.length) {
    sections.push("Metadata");
  }
  if (
    evaluatedValueText(element.evaluation) ||
    evaluationStateLabel(element.evaluation) ||
    analysisLabel(element.analysis)
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
  // Only a family the publication resolved names a type. An ambiguous or unresolved one has no
  // single target, and picking a candidate here would make the view claim a resolution the
  // publication refused to make.
  const typingTarget =
    element.typing.status === "resolved" ? element.typing.targets[0] : undefined;
  const evaluated = evaluatedValueText(element.evaluation);
  return {
    selectionKind,
    selectionText: result.selection.text,
    declaration: element.declaration,
    declaredValue:
      element.evaluation.state === "literal" ? evaluated : undefined,
    evaluatedValue: evaluated ?? evaluationStateLabel(element.evaluation),
    unit:
      element.evaluation.state === "literal" ||
      element.evaluation.state === "evaluated"
        ? element.evaluation.unit
        : undefined,
    quantityType: typingTarget?.qualifiedName,
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

  const referenced = resolvedReference(result);
  if (result.selection.kind === "reference" && referenced) {
    return {
      mode: "reference",
      sections: elementSections(referenced),
      primaryElement: referenced,
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

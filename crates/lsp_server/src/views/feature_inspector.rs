//! sysml/featureInspector request parsing and response building.
//!
//! A protocol adapter over one immutable publication. Every semantic fact it reports -- what an
//! element is, what it declares, what those declarations resolved to, what it inherits and from
//! where, and what its expression settled to -- is read from `PublishedModel::element_details`.
//! Nothing here resolves a name, walks a hierarchy, infers a type or manufactures a status; the
//! only thing it decides is how a settled fact is spelled on the wire.

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{Position, Url};

use crate::common::util;
use crate::language::position_to_byte_offset;
use crate::views::dto::{
    range_to_dto, PositionDto, SysmlFeatureInspectorAnalysisDto, SysmlFeatureInspectorElementDto,
    SysmlFeatureInspectorElementRefDto, SysmlFeatureInspectorEvaluationDto,
    SysmlFeatureInspectorInheritedFeatureDto, SysmlFeatureInspectorParamsDto,
    SysmlFeatureInspectorReferenceDto, SysmlFeatureInspectorRelationshipDto,
    SysmlFeatureInspectorResolutionDto, SysmlFeatureInspectorResultDto,
    SysmlFeatureInspectorSelectionDto,
};
use sysml_query::resolved_slice::{
    AnalysisEvaluation, AnnotationForm, ConnectedElement, ElementDetails, ElementDetailsAt,
    ElementEvaluation, ElementKind, ElementModifier, EvaluatedScalar, EvaluationFailure,
    EvaluationState, FeatureDirection, MultiplicityBound, MultiplicityFacts, PortionKind,
    PublishedModel, QueryOutcome, ReferencedDetails, RelationshipFamily, RelationshipOutcome,
    RelationshipProvenance, SymbolEntry, SymbolId, SymbolToken, TextPosition, TextRange,
};

/// The document a declaration's authored text is read from: the source, paired with the tree the
/// syntax service parsed for it.
///
/// Both, because the publication's declaration range covers the body as well as the header, and
/// only the grammar knows where the header ends. A borrowed pair rather than a struct: nothing
/// here owns a tree, it reads the one the host already holds.
pub type DeclarationSource<'a> = (&'a str, &'a sysml_query::syntax::ParsedSource);

/// The settled details a position identifies, whatever completeness they were published under.
///
/// A recovery or unsupported publication still answers; only a non-converged one has nothing to
/// report, and that is an absent answer rather than an empty one.
pub(crate) fn details_at(
    model: &PublishedModel,
    uri: &Url,
    position: Position,
) -> Option<ElementDetailsAt> {
    match model.inspection().element_details_at(
        uri.as_str(),
        TextPosition {
            line: position.line,
            character: position.character,
        },
    ) {
        QueryOutcome::Resolved(at)
        | QueryOutcome::Recovered(at)
        | QueryOutcome::UnsupportedWith(at) => Some(at),
        _ => None,
    }
}

/// The protocol form of one element reference.
///
/// The published handle is a `SymbolId`, valid only inside this publication; the DTO crosses a
/// process boundary, so the identity materialises here as a `SymbolToken` and nowhere earlier.
fn element_ref(model: &PublishedModel, entry: &SymbolEntry) -> SysmlFeatureInspectorElementRefDto {
    SysmlFeatureInspectorElementRefDto {
        id: symbol_token_text(model, entry.identity),
        name: entry.name.as_deref().unwrap_or_default().to_string(),
        qualified_name: entry.qualified_name.to_string(),
        element_type: entry.kind.as_str().to_string(),
        uri: entry.location.document.to_string(),
        range: range_to_dto(entry.location.range),
    }
}

/// The coarse role a client groups elements by.
///
/// A total match over the published kind vocabulary, so a kind added to the publication fails to
/// compile here until someone decides which group it belongs to. That compile error is the point:
/// a fall-through arm is how every new usage kind would silently become a "definition".
fn semantic_role(kind: ElementKind) -> &'static str {
    match kind {
        ElementKind::Namespace | ElementKind::Package | ElementKind::LibraryPackage => "namespace",

        ElementKind::PartDefinition
        | ElementKind::AttributeDefinition
        | ElementKind::EnumerationDefinition
        | ElementKind::ItemDefinition
        | ElementKind::PortDefinition
        | ElementKind::OccurrenceDefinition
        | ElementKind::IndividualDefinition
        | ElementKind::ConnectionDefinition
        | ElementKind::InterfaceDefinition
        | ElementKind::AllocationDefinition
        | ElementKind::FlowConnectionDefinition
        | ElementKind::ActionDefinition
        | ElementKind::StateDefinition
        | ElementKind::CalculationDefinition
        | ElementKind::ConstraintDefinition
        | ElementKind::RequirementDefinition
        | ElementKind::ConcernDefinition
        | ElementKind::CaseDefinition
        | ElementKind::AnalysisCaseDefinition
        | ElementKind::VerificationCaseDefinition
        | ElementKind::UseCaseDefinition
        | ElementKind::ViewDefinition
        | ElementKind::ViewpointDefinition
        | ElementKind::RenderingDefinition
        | ElementKind::MetadataDefinition
        | ElementKind::Definition
        | ElementKind::Type
        | ElementKind::Classifier
        | ElementKind::Class
        | ElementKind::Structure
        | ElementKind::Association
        | ElementKind::AssociationStructure
        | ElementKind::DataType
        | ElementKind::Metaclass
        | ElementKind::Behavior
        | ElementKind::Function
        | ElementKind::Predicate
        | ElementKind::Interaction
        | ElementKind::Multiplicity => "definition",

        ElementKind::ConnectionUsage
        | ElementKind::InterfaceUsage
        | ElementKind::AllocationUsage
        | ElementKind::FlowConnectionUsage
        | ElementKind::SuccessionAsUsage
        | ElementKind::SatisfyRequirementUsage
        | ElementKind::BindingConnectorAsUsage
        | ElementKind::Import
        | ElementKind::Expose
        | ElementKind::Alias
        | ElementKind::Dependency
        | ElementKind::Connector
        | ElementKind::BindingConnector => "relationship",

        ElementKind::PartUsage
        | ElementKind::AttributeUsage
        | ElementKind::EnumerationUsage
        | ElementKind::ItemUsage
        | ElementKind::PortUsage
        | ElementKind::OccurrenceUsage
        | ElementKind::ActionUsage
        | ElementKind::StateUsage
        | ElementKind::CalculationUsage
        | ElementKind::ConstraintUsage
        | ElementKind::AssertConstraintUsage
        | ElementKind::RequirementUsage
        | ElementKind::ConcernUsage
        | ElementKind::CaseUsage
        | ElementKind::AnalysisCaseUsage
        | ElementKind::VerificationCaseUsage
        | ElementKind::UseCaseUsage
        | ElementKind::ViewUsage
        | ElementKind::ViewpointUsage
        | ElementKind::RenderingUsage
        | ElementKind::MetadataUsage
        | ElementKind::ReferenceUsage
        | ElementKind::AcceptActionUsage
        | ElementKind::PerformActionUsage
        | ElementKind::TransitionUsage
        | ElementKind::AssignmentActionUsage
        | ElementKind::IfActionUsage
        | ElementKind::WhileLoopActionUsage
        | ElementKind::ForLoopActionUsage
        | ElementKind::ForLoopVariable
        | ElementKind::DecisionNode
        | ElementKind::MergeNode
        | ElementKind::ForkNode
        | ElementKind::JoinNode
        | ElementKind::FinalState
        | ElementKind::Feature
        | ElementKind::Step
        | ElementKind::Expression
        | ElementKind::BooleanExpression
        | ElementKind::Invariant => "usage",
    }
}

/// The wire spelling of a published relationship outcome.
fn resolution_status(outcome: RelationshipOutcome) -> &'static str {
    match outcome {
        RelationshipOutcome::NotApplicable => "notApplicable",
        RelationshipOutcome::Resolved => "resolved",
        RelationshipOutcome::Partial => "partial",
        RelationshipOutcome::Unresolved => "unresolved",
        RelationshipOutcome::Ambiguous => "ambiguous",
        RelationshipOutcome::Unsupported => "unsupported",
    }
}

fn resolution(
    model: &PublishedModel,
    family: &RelationshipFamily,
) -> SysmlFeatureInspectorResolutionDto {
    SysmlFeatureInspectorResolutionDto {
        status: resolution_status(family.outcome).to_string(),
        targets: family
            .targets
            .iter()
            .map(|entry| element_ref(model, entry))
            .collect(),
        candidates: family
            .candidates
            .iter()
            .map(|entry| element_ref(model, entry))
            .collect(),
    }
}

fn relationship(
    model: &PublishedModel,
    entry: &ConnectedElement,
) -> SysmlFeatureInspectorRelationshipDto {
    SysmlFeatureInspectorRelationshipDto {
        rel_type: entry.kind.to_string(),
        peer: element_ref(model, &entry.peer),
        provenance: match entry.provenance {
            RelationshipProvenance::Authored => "authored",
            RelationshipProvenance::Implied => "implied",
        }
        .to_string(),
    }
}

fn inherited_feature(
    model: &PublishedModel,
    feature: &sysml_query::resolved_slice::InheritedFeature,
) -> SysmlFeatureInspectorInheritedFeatureDto {
    SysmlFeatureInspectorInheritedFeatureDto {
        feature: element_ref(model, &feature.feature),
        declared_in: element_ref(model, &feature.declared_in),
    }
}

/// The inspector is a JSON transport boundary; the published scalar keeps its closed
/// representation until here.
///
/// A quantity's magnitude and unit are returned separately, because the unit is not part of the
/// number and folding them into one string would make a client parse it back out.
/// One element handle, materialised for the protocol.
///
/// `SymbolToken` is the only form of an element identity that may leave the process; a handle that
/// this publication no longer knows has no token, and the DTO carries an empty identity for it
/// exactly as the unresolvable case always did.
fn symbol_token_text(model: &PublishedModel, symbol: SymbolId) -> String {
    model
        .symbol_token(symbol)
        .map(SymbolToken::into_string)
        .unwrap_or_default()
}

fn scalar_json(scalar: &EvaluatedScalar) -> (serde_json::Value, Option<String>) {
    match scalar {
        EvaluatedScalar::Boolean(value) => (serde_json::Value::Bool(*value), None),
        EvaluatedScalar::Integer(value) => (serde_json::Value::Number((*value).into()), None),
        EvaluatedScalar::Real(value) => (
            serde_json::Number::from_f64(*value)
                .map(serde_json::Value::Number)
                // Preserve an out-of-range scalar visibly rather than presenting it as a
                // successful JSON number. The evaluator itself only publishes finite values.
                .unwrap_or_else(|| serde_json::Value::String(value.to_string())),
            None,
        ),
        EvaluatedScalar::String(value) => (serde_json::Value::String(value.to_string()), None),
        EvaluatedScalar::Quantity { magnitude, unit } => {
            let (value, _) = scalar_json(magnitude);
            (value, Some(unit.to_string()))
        }
    }
}

fn evaluation(evaluation: &ElementEvaluation) -> SysmlFeatureInspectorEvaluationDto {
    match &evaluation.state {
        EvaluationState::NotApplicable => SysmlFeatureInspectorEvaluationDto::NotApplicable,
        EvaluationState::NotRun => SysmlFeatureInspectorEvaluationDto::NotRun,
        EvaluationState::Literal(scalar) => {
            let (value, unit) = scalar_json(scalar);
            SysmlFeatureInspectorEvaluationDto::Literal { value, unit }
        }
        EvaluationState::Evaluated(scalar) => {
            let (value, unit) = scalar_json(scalar);
            SysmlFeatureInspectorEvaluationDto::Evaluated { value, unit }
        }
        EvaluationState::NonConstant => SysmlFeatureInspectorEvaluationDto::NonConstant,
        EvaluationState::Cyclic => SysmlFeatureInspectorEvaluationDto::Cyclic,
        EvaluationState::Unsupported => SysmlFeatureInspectorEvaluationDto::Unsupported,
        EvaluationState::Failed(failure) => SysmlFeatureInspectorEvaluationDto::Failed {
            reason: match failure {
                EvaluationFailure::DivisionByZero => "divisionByZero",
                EvaluationFailure::TypeMismatch => "typeMismatch",
                EvaluationFailure::UnresolvedOperand => "unresolvedOperand",
            }
            .to_string(),
        },
    }
}

fn analysis(analysis: &AnalysisEvaluation) -> SysmlFeatureInspectorAnalysisDto {
    match analysis {
        AnalysisEvaluation::NotApplicable => SysmlFeatureInspectorAnalysisDto::NotApplicable,
        AnalysisEvaluation::NotRun => SysmlFeatureInspectorAnalysisDto::NotRun,
        AnalysisEvaluation::Verdict(passed) => {
            SysmlFeatureInspectorAnalysisDto::Verdict { passed: *passed }
        }
        AnalysisEvaluation::Computed(scalar) => {
            let (value, unit) = scalar_json(scalar);
            SysmlFeatureInspectorAnalysisDto::Computed { value, unit }
        }
        AnalysisEvaluation::Unsettled(state) => SysmlFeatureInspectorAnalysisDto::Unsettled {
            evaluation: state.as_str().to_string(),
        },
    }
}

fn bound_text(bound: MultiplicityBound) -> String {
    match bound {
        MultiplicityBound::Unbounded => "*".to_string(),
        MultiplicityBound::Literal(value) => value.to_string(),
        // The author wrote a non-literal bound. Rendering a number here would invent one.
        MultiplicityBound::Expression => "…".to_string(),
    }
}

fn multiplicity_text(multiplicity: MultiplicityFacts) -> Option<String> {
    match multiplicity {
        MultiplicityFacts::Absent => None,
        MultiplicityFacts::Declared { lower, upper, .. } => {
            let lower = bound_text(lower);
            let upper = bound_text(upper);
            Some(if lower == upper {
                lower
            } else {
                format!("{lower}..{upper}")
            })
        }
    }
}

fn modifiers(details: &ElementDetails) -> Vec<String> {
    let mut modifiers = details
        .inspection
        .modifiers
        .iter()
        .map(|modifier| ElementModifier::as_str(*modifier).to_string())
        .collect::<Vec<_>>();
    if let Some(portion) = details.inspection.portion_kind {
        modifiers.push(
            match portion {
                PortionKind::Snapshot => "snapshot",
                PortionKind::Timeslice => "timeslice",
            }
            .to_string(),
        );
    }
    modifiers
}

fn documentation(details: &ElementDetails) -> Option<String> {
    let text = details
        .inspection
        .documentation
        .iter()
        .filter(|entry| entry.form == AnnotationForm::Documentation)
        .map(|entry| entry.text.trim().to_string())
        .filter(|text| !text.is_empty())
        .collect::<Vec<_>>()
        .join("\n\n");
    (!text.is_empty()).then_some(text)
}

/// The declaration as the author wrote it, taken from the source the publication was built from.
///
/// Source text, not a reconstruction: a signature rebuilt from published facts would have to guess
/// the keyword the author used, and a keyword is a syntax fact rather than a semantic one. The
/// range is the publication's own declaration range, so the slice is exactly the declaration.
fn declaration_text(details: &ElementDetails, source: Option<DeclarationSource<'_>>) -> String {
    let fallback = || {
        format!(
            "{} {}",
            details.inspection.kind.as_str(),
            details.inspection.name.as_deref().unwrap_or_default()
        )
        .trim_end()
        .to_string()
    };
    let Some((text, parsed)) = source else {
        return fallback();
    };
    // A body is not part of the declaration a reader wants to see, and where it begins is the
    // grammar's answer: the syntax service publishes the declaration's head range. Cutting the
    // publication's range at the first `{` used to truncate a declaration whose header carried
    // one in a string or a comment, and kept the whole body when it carried none.
    let declaration_range = details.inspection.declaration_range;
    let head_range = parsed
        .enclosing_declarations(declaration_range.start.line)
        .into_iter()
        .rev()
        .find(|declaration| declaration.range.start_line == declaration_range.start.line)
        .map(|declaration| {
            let head = declaration.head_range;
            TextRange::new(
                TextPosition::new(head.start_line, head.start_character),
                TextPosition::new(head.end_line, head.end_character),
            )
        })
        .unwrap_or(declaration_range);
    let Some(text) = slice_range(text, head_range) else {
        return fallback();
    };
    let collapsed = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let collapsed = collapsed.trim_end_matches(';').trim().to_string();
    if collapsed.is_empty() {
        fallback()
    } else {
        collapsed
    }
}

/// The `text` covered by a published range.
///
/// Characters are counted as Unicode scalar values, which is what the publication's own column
/// numbering counts. A range that does not land on a character boundary yields `None` rather than
/// a slice taken from somewhere else, and the caller falls back.
fn slice_range(text: &str, range: TextRange) -> Option<&str> {
    let start = position_to_byte_offset(text, range.start.line, range.start.character)?;
    let end = position_to_byte_offset(text, range.end.line, range.end.character)?;
    text.get(start..end)
}

/// Whether the publication places `position` on this element's own name.
///
/// Read from the published name range rather than by comparing the token under the cursor with the
/// element's name: two elements in one declaration can share a spelling, and a text comparison
/// cannot tell the declaration from a reference to something else with the same name.
pub(crate) fn covers_name(details: &ElementDetails, position: Position) -> bool {
    let range = details.inspection.location.range;
    let position = (position.line, position.character);
    (range.start.line, range.start.character) <= position
        && position <= (range.end.line, range.end.character)
}

pub fn parse_sysml_feature_inspector_params(v: &serde_json::Value) -> Result<(Url, Position)> {
    // vscode-jsonrpc versions can encode `sendRequest(method, params, undefined)`
    // as `[params, null]`. Accept that transition artifact at the protocol boundary
    // while clients migrate to omitting the absent cancellation-token argument.
    let normalized = match v.as_array().map(Vec::as_slice) {
        Some([params]) if params.is_object() => params,
        Some([params, trailing]) if params.is_object() && trailing.is_null() => params,
        _ => v,
    };
    let params: SysmlFeatureInspectorParamsDto = serde_json::from_value(normalized.clone())
        .map_err(|error| tower_lsp::jsonrpc::Error::invalid_params(error.to_string()))?;
    let uri_text = params
        .text_document
        .map(|document| document.uri)
        .or(params.uri)
        .ok_or_else(|| {
            tower_lsp::jsonrpc::Error::invalid_params(
                "sysml/featureInspector: expected textDocument.uri",
            )
        })?;
    let uri = Url::parse(&uri_text).map_err(|_| {
        tower_lsp::jsonrpc::Error::invalid_params("sysml/featureInspector: invalid URI")
    })?;
    let uri = util::normalize_file_uri(&uri);
    let position = Position::new(params.position.line, params.position.character);
    Ok((uri, position))
}

pub fn empty_feature_inspector_response(
    uri: &Url,
    position: Position,
) -> SysmlFeatureInspectorResultDto {
    SysmlFeatureInspectorResultDto {
        version: 2,
        source_uri: uri.to_string(),
        requested_position: PositionDto {
            line: position.line,
            character: position.character,
        },
        selection: SysmlFeatureInspectorSelectionDto {
            kind: "other".to_string(),
            text: None,
            range: None,
        },
        language_help: None,
        containing_element: None,
        referenced: SysmlFeatureInspectorReferenceDto::None,
    }
}

pub(crate) fn feature_inspector_element(
    model: &PublishedModel,
    details: &ElementDetails,
    source: Option<DeclarationSource<'_>>,
) -> SysmlFeatureInspectorElementDto {
    let inspection = &details.inspection;
    SysmlFeatureInspectorElementDto {
        id: symbol_token_text(model, inspection.identity),
        name: inspection.name.as_deref().unwrap_or_default().to_string(),
        qualified_name: inspection.qualified_name.to_string(),
        element_type: inspection.kind.as_str().to_string(),
        role: semantic_role(inspection.kind).to_string(),
        declaration: declaration_text(details, source),
        uri: inspection.location.document.to_string(),
        range: range_to_dto(inspection.declaration_range),
        parent: details
            .owner
            .as_ref()
            .map(|entry| element_ref(model, entry)),
        documentation: documentation(details),
        multiplicity: multiplicity_text(inspection.multiplicity),
        direction: inspection.direction.map(|direction| {
            match direction {
                FeatureDirection::In => "in",
                FeatureDirection::Out => "out",
                FeatureDirection::InOut => "inout",
            }
            .to_string()
        }),
        modifiers: modifiers(details),
        evaluation: evaluation(&details.evaluation),
        analysis: analysis(&details.analysis),
        typing: resolution(model, &details.typing),
        effective_typing: SysmlFeatureInspectorResolutionDto {
            status: resolution_status(details.effective_typing.outcome).to_string(),
            targets: details
                .effective_typing
                .types
                .iter()
                .map(|entry| element_ref(model, &entry.element))
                .collect(),
            candidates: Vec::new(),
        },
        specialization: resolution(model, &details.specialization),
        subsetting: resolution(model, &details.subsetting),
        redefinition: resolution(model, &details.redefinition),
        inherited_features: details
            .inherited_features
            .iter()
            .map(|feature| inherited_feature(model, feature))
            .collect(),
        metadata: details
            .metadata
            .iter()
            .map(|entry| element_ref(model, entry))
            .collect(),
        incoming_relationships: details
            .incoming
            .iter()
            .map(|entry| relationship(model, entry))
            .collect(),
        outgoing_relationships: details
            .outgoing
            .iter()
            .map(|entry| relationship(model, entry))
            .collect(),
    }
}

pub(crate) fn referenced_dto(
    model: &PublishedModel,
    referenced: &ReferencedDetails,
    source: Option<DeclarationSource<'_>>,
) -> SysmlFeatureInspectorReferenceDto {
    match referenced {
        ReferencedDetails::None => SysmlFeatureInspectorReferenceDto::None,
        ReferencedDetails::Resolved(details) => SysmlFeatureInspectorReferenceDto::Resolved {
            element: Box::new(feature_inspector_element(model, details, source)),
        },
        ReferencedDetails::Ambiguous(candidates) => SysmlFeatureInspectorReferenceDto::Ambiguous {
            candidates: candidates
                .iter()
                .map(|details| feature_inspector_element(model, details, source))
                .collect(),
        },
        ReferencedDetails::Unresolved => SysmlFeatureInspectorReferenceDto::Unresolved,
        ReferencedDetails::Unsupported => SysmlFeatureInspectorReferenceDto::Unsupported,
        ReferencedDetails::Incomplete => SysmlFeatureInspectorReferenceDto::Incomplete,
    }
}

/// The protocol answer for one already-settled position.
///
/// Separate from [`build_sysml_feature_inspector_response`] so the request handler, which also
/// needs the settled details to classify the selection, queries the publication once.
pub(crate) fn feature_inspector_response(
    model: &PublishedModel,
    uri: &Url,
    position: Position,
    at: &ElementDetailsAt,
    source: Option<DeclarationSource<'_>>,
) -> SysmlFeatureInspectorResultDto {
    let mut response = empty_feature_inspector_response(uri, position);
    response.containing_element = at
        .containing
        .as_ref()
        .map(|details| feature_inspector_element(model, details, source));
    response.referenced = referenced_dto(model, &at.referenced, source);
    response
}

pub fn build_sysml_feature_inspector_response(
    model: &PublishedModel,
    uri: &Url,
    position: Position,
    source: Option<DeclarationSource<'_>>,
) -> SysmlFeatureInspectorResultDto {
    match details_at(model, uri, position) {
        Some(at) => feature_inspector_response(model, uri, position, &at, source),
        None => empty_feature_inspector_response(uri, position),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_query::source::{SourceKind, SourceService};

    fn inspect(source: &str, line: u32, character: u32) -> SysmlFeatureInspectorResultDto {
        let uri = Url::parse("file:///inspector.sysml").expect("uri");
        let services = sysml_query::Services::new();
        let document = SourceService::new().admit_url(uri.clone(), source, SourceKind::Workspace);
        let parsed = services.syntax.parse(&document);
        let model = services
            .publication
            .publish(&[document], [])
            .expect("publication");
        build_sysml_feature_inspector_response(
            &model,
            &uri,
            Position::new(line, character),
            Some((source, &parsed)),
        )
    }

    #[test]
    fn evaluation_projects_the_published_state_without_inventing_a_value() {
        let response = inspect(
            "package Demo { attribute value = 1; part def Empty; }",
            0,
            25,
        );
        let element = response.containing_element.expect("value element");
        assert!(
            matches!(
                element.evaluation,
                SysmlFeatureInspectorEvaluationDto::Literal { ref value, unit: None }
                    if value.as_i64() == Some(1)
            ),
            "{:?}",
            element.evaluation
        );
        assert!(matches!(
            element.analysis,
            SysmlFeatureInspectorAnalysisDto::NotApplicable
        ));
    }

    #[test]
    fn an_element_with_no_expression_is_not_applicable_rather_than_valueless() {
        let response = inspect(
            "package Demo { attribute value = 1; part def Empty; }",
            0,
            45,
        );
        let element = response.containing_element.expect("Empty element");
        assert_eq!(element.name, "Empty");
        assert!(matches!(
            element.evaluation,
            SysmlFeatureInspectorEvaluationDto::NotApplicable
        ));
    }

    /// The verdict channel and the value channel are projected separately, so a constraint that
    /// evaluated to `false` reports a failing verdict rather than an absent value.
    #[test]
    fn a_constraint_reports_its_verdict_beside_its_value() {
        let response = inspect("package Demo { constraint fails { false } }", 0, 27);
        let element = response.containing_element.expect("constraint element");
        assert!(matches!(
            element.analysis,
            SysmlFeatureInspectorAnalysisDto::Verdict { passed: false }
        ));
        assert!(matches!(
            element.evaluation,
            SysmlFeatureInspectorEvaluationDto::Literal { .. }
        ));
    }

    /// The declaration line is the author's own text, cut at the body.
    #[test]
    fn the_declaration_is_the_authored_text_of_the_declaration_range() {
        let response = inspect(
            "package Demo {\n  part def Rover :> Base {\n    part wheel;\n  }\n}",
            1,
            11,
        );
        let element = response.containing_element.expect("Rover element");
        assert_eq!(element.declaration, "part def Rover :> Base");
    }

    #[test]
    fn multiplicity_renders_the_published_bounds() {
        let response = inspect("package Demo { part def W; part w[0..*] : W; }", 0, 32);
        assert_eq!(
            response
                .containing_element
                .expect("w element")
                .multiplicity
                .as_deref(),
            Some("0..*")
        );
    }
}

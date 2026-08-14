//! Owner-side canonical rendering for the private resolved semantic model.
//!
//! The writer accepts only the resolved owner and a caller-provided `fmt::Write`. It does not
//! expose storage collections or return projections that could become a second semantic model.

use std::fmt;

use super::*;

pub(super) fn write_semantic(
    model: &ResolvedSemanticModel,
    source_digest: &source_identity::RootDigest,
    semantic_contract_version: &str,
    output: &mut dyn fmt::Write,
) -> fmt::Result {
    writeln!(output, "(semantic-model")?;
    write_metadata(model, source_digest, semantic_contract_version, output)?;
    write_declarations(model, output)?;
    write_references(model, output)?;
    write_relationships(model, output)?;
    write_evaluation(model, output)?;
    write!(output, ")")
}

pub(super) fn write_navigation_only(
    model: &ResolvedSemanticModel,
    output: &mut dyn fmt::Write,
) -> fmt::Result {
    write_navigation(model, output)
}

pub(super) fn write_diagnostics(
    model: &ResolvedSemanticModel,
    output: &mut dyn fmt::Write,
) -> fmt::Result {
    // TODO(diagnostic-presentation-boundary): Publish one presentation-neutral typed diagnostic
    // contract containing the semantic code/category, severity, primary provenance, related
    // provenance, and structured message arguments. Keep localization, prose, styling, protocol
    // conversion, and layout in domain adapters. This function should then be only the canonical
    // snapshot S-expression adapter; CLI, LSP, Markdown, and HTML must consume the same typed
    // diagnostics without accessing resolver storage or reconstructing semantic meaning.
    let semantic_records = model.diagnostic_records().map_err(|_| fmt::Error)?;
    writeln!(output, "(fixture-diagnostics")?;
    for document_index in canonical_document_indices(model) {
        let document = &model.storage.documents[document_index];
        let document_id = DocumentId(document_index as u32);
        let mut records = Vec::new();
        for error in &document.parse_errors {
            let range = parse_error_range(&document.parsed, error).ok_or(fmt::Error)?;
            records.push(CanonicalDiagnostic::Parser { error, range });
        }
        for record in model
            .storage
            .unsupported
            .iter()
            .filter(|record| record.document == document_id)
        {
            let range = document_range(&model.storage, document_id, &record.span)
                .map_err(|_| fmt::Error)?;
            records.push(CanonicalDiagnostic::Unsupported { record, range });
        }
        for record in semantic_records.iter().filter(|record| {
            model
                .storage
                .declaration(record.source)
                .is_some_and(|source| source.document == document_id)
        }) {
            if diagnostic_label(record.kind, &record.outcome).is_some() {
                records.push(CanonicalDiagnostic::Semantic(record));
            }
        }
        records.sort_by(|left, right| {
            let left_range = left.range();
            let right_range = right.range();
            (
                left_range.start.line,
                left_range.start.character,
                left_range.end.line,
                left_range.end.character,
            )
                .cmp(&(
                    right_range.start.line,
                    right_range.start.character,
                    right_range.end.line,
                    right_range.end.character,
                ))
                .then_with(|| left.code().cmp(right.code()))
        });
        writeln!(output, "  (document {:?}", document.identity)?;
        writeln!(output, "    (diagnostics")?;
        for record in records {
            write_diagnostic(model, record, output)?;
        }
        writeln!(output, "    )\n  )")?;
    }
    write!(output, ")")
}

enum CanonicalDiagnostic<'a> {
    Parser {
        error: &'a ParseError,
        range: TextRange,
    },
    Unsupported {
        record: &'a UnsupportedRecord,
        range: TextRange,
    },
    Semantic(&'a DiagnosticRecord),
}

impl CanonicalDiagnostic<'_> {
    fn range(&self) -> TextRange {
        match self {
            Self::Parser { range, .. } | Self::Unsupported { range, .. } => *range,
            Self::Semantic(record) => record.range,
        }
    }

    fn code(&self) -> &str {
        match self {
            Self::Parser { error, .. } => error.code.as_deref().unwrap_or("parse_error"),
            Self::Unsupported { record, .. } => unsupported_code(record.family),
            Self::Semantic(record) => diagnostic_label(record.kind, &record.outcome)
                .map_or("semantic_diagnostic", |(_, code)| code),
        }
    }
}

fn write_diagnostic(
    model: &ResolvedSemanticModel,
    diagnostic: CanonicalDiagnostic<'_>,
    output: &mut dyn fmt::Write,
) -> fmt::Result {
    let (severity, source) = match &diagnostic {
        CanonicalDiagnostic::Parser { error, .. } => (
            match error.severity {
                Some(sysml_v2_parser_next::DiagnosticSeverity::Warning) => "warning",
                Some(sysml_v2_parser_next::DiagnosticSeverity::Error) | None => "error",
            },
            "parser",
        ),
        CanonicalDiagnostic::Unsupported { .. } => ("warning", "semantic"),
        CanonicalDiagnostic::Semantic(record) => {
            let (severity, _) = diagnostic_label(record.kind, &record.outcome).ok_or(fmt::Error)?;
            (severity, "semantic")
        }
    };
    let range = diagnostic.range();
    writeln!(output, "      (diagnostic")?;
    writeln!(output, "        (severity {severity})")?;
    write!(output, "        (code ")?;
    write_quoted(output, diagnostic.code())?;
    writeln!(output, ")")?;
    writeln!(output, "        (source \"{source}\")")?;
    writeln!(
        output,
        "        (range (start {} {}) (end {} {}))",
        range.start.line, range.start.character, range.end.line, range.end.character,
    )?;
    if let CanonicalDiagnostic::Semantic(DiagnosticRecord {
        outcome: DiagnosticOutcome::Ambiguous { candidates },
        ..
    }) = diagnostic
    {
        writeln!(output, "        (related-information")?;
        for candidate in candidates {
            let declaration = model
                .storage
                .declaration(candidate.target)
                .ok_or(fmt::Error)?;
            let range = document_range(&model.storage, declaration.document, &declaration.span)
                .map_err(|_| fmt::Error)?;
            writeln!(output, "          (related")?;
            writeln!(
                output,
                "            (uri {:?})",
                document_identity(model, declaration.document)
            )?;
            writeln!(
                output,
                "            (range (start {} {}) (end {} {}))",
                range.start.line, range.start.character, range.end.line, range.end.character,
            )?;
            writeln!(output, "          )")?;
        }
        writeln!(output, "        )")?;
    }
    writeln!(output, "      )")
}

fn parse_error_range(document: &ParsedDocument, error: &ParseError) -> Option<TextRange> {
    let start_offset = error.offset?;
    let end_offset = start_offset.checked_add(error.length.unwrap_or(1))?;
    let start = document.source.position_at(start_offset)?;
    let end = document.source.position_at(end_offset).unwrap_or(start);
    Some(TextRange {
        start: TextPosition {
            line: start.line.saturating_sub(1),
            character: u32::try_from(start.column.saturating_sub(1)).ok()?,
        },
        end: TextPosition {
            line: end.line.saturating_sub(1),
            character: u32::try_from(end.column.saturating_sub(1)).ok()?,
        },
    })
}

fn unsupported_code(family: UnsupportedFamily) -> &'static str {
    match family {
        UnsupportedFamily::PackageMember => "unsupported_package_member",
        UnsupportedFamily::PartDefinitionMember => "unsupported_part_definition_member",
        UnsupportedFamily::PartUsageMember => "unsupported_part_usage_member",
        UnsupportedFamily::AttributeMember => "unsupported_attribute_member",
        UnsupportedFamily::RequirementDefinitionMember => {
            "unsupported_requirement_definition_member"
        }
        UnsupportedFamily::PortDefinitionMember => "unsupported_port_definition_member",
        UnsupportedFamily::PortUsageMember => "unsupported_port_usage_member",
        UnsupportedFamily::ActionDefinitionMember => "unsupported_action_definition_member",
        UnsupportedFamily::ActionUsageMember => "unsupported_action_usage_member",
        UnsupportedFamily::StateDefinitionMember => "unsupported_state_definition_member",
        UnsupportedFamily::ConnectionDefinitionMember => "unsupported_connection_definition_member",
        UnsupportedFamily::InterfaceDefinitionMember => "unsupported_interface_definition_member",
        UnsupportedFamily::ViewDefinitionMember => "unsupported_view_definition_member",
        UnsupportedFamily::ConstraintDefinitionMember => "unsupported_constraint_definition_member",
        UnsupportedFamily::CalcDefinitionMember => "unsupported_calc_definition_member",
        UnsupportedFamily::RenderingDefinitionMember => "unsupported_rendering_definition_member",
        UnsupportedFamily::OccurrenceDefinitionMember => "unsupported_occurrence_definition_member",
        UnsupportedFamily::AnalysisCaseDefinitionMember => {
            "unsupported_analysis_case_definition_member"
        }
        UnsupportedFamily::CaseDefinitionMember => "unsupported_case_definition_member",
        UnsupportedFamily::VerificationCaseDefinitionMember => {
            "unsupported_verification_case_definition_member"
        }
        UnsupportedFamily::UseCaseDefinitionMember => "unsupported_use_case_definition_member",
        UnsupportedFamily::ReferenceUsageMember => "unsupported_reference_usage_member",
        UnsupportedFamily::ParserUnsupported => "unsupported_parser_construct",
    }
}

fn diagnostic_label(
    kind: ReferenceKind,
    outcome: &DiagnosticOutcome,
) -> Option<(&'static str, &'static str)> {
    match outcome {
        DiagnosticOutcome::Resolved => None,
        DiagnosticOutcome::Unresolved => Some((
            "warning",
            match kind {
                ReferenceKind::FeatureTyping => "unresolved_type_reference",
                ReferenceKind::Subclassification => "unresolved_specializes_reference",
                ReferenceKind::NamespaceImport | ReferenceKind::MembershipImport => {
                    "unresolved_import_target"
                }
                _ => "unresolved_reference",
            },
        )),
        DiagnosticOutcome::Unsupported => Some((
            "warning",
            match kind {
                ReferenceKind::NamespaceImport
                | ReferenceKind::MembershipImport
                | ReferenceKind::FilterImport => "unsupported_filtered_import",
                _ => "unsupported_reference",
            },
        )),
        DiagnosticOutcome::NonConverged => Some(("error", "non_converged_resolution")),
        DiagnosticOutcome::Ambiguous { .. } => Some((
            "error",
            match kind {
                ReferenceKind::NamespaceImport | ReferenceKind::MembershipImport => {
                    "ambiguous_import_target"
                }
                _ => "ambiguous_reference",
            },
        )),
    }
}

fn write_metadata(
    model: &ResolvedSemanticModel,
    source_digest: &source_identity::RootDigest,
    semantic_contract_version: &str,
    output: &mut dyn fmt::Write,
) -> fmt::Result {
    let phase = match model.metadata.phase {
        PublicationPhase::Resolved => "resolved",
    };
    let completeness = match model.metadata.completeness {
        PublicationCompleteness::Complete => "complete",
        PublicationCompleteness::ParseRecovery => "parse-recovery",
        PublicationCompleteness::UnsupportedSyntax => "unsupported-syntax",
        PublicationCompleteness::NonConverged => "non-converged",
    };
    write!(
        output,
        "  (publication (phase {phase}) (completeness {completeness}) (has-evaluation {}) (source-digest ",
        model.metadata.has_evaluation
    )?;
    write_quoted(output, &source_digest.to_string())?;
    write!(output, ") (contract-version ")?;
    write_quoted(output, semantic_contract_version)?;
    writeln!(output, "))")
}

fn write_declarations(model: &ResolvedSemanticModel, output: &mut dyn fmt::Write) -> fmt::Result {
    writeln!(output, "  (declarations")?;
    for index in canonical_declaration_indices(model) {
        let declaration = &model.storage.declarations[index];
        write!(output, "    (declaration (id ")?;
        write_node_identity(model, DeclarationId(index as u32), output)?;
        write!(output, ") (kind {})", declaration_kind(declaration.kind))?;
        if let Some(membership) = model
            .storage
            .memberships
            .iter()
            .find(|membership| membership.member == DeclarationId(index as u32))
        {
            write!(
                output,
                " (membership (kind {}) (visibility {}))",
                membership_kind(membership.kind),
                visibility(membership.visibility),
            )?;
        }
        write_declaration_facts(model, DeclarationId(index as u32), output)?;
        write_documentation(model, DeclarationId(index as u32), output)?;
        write_feature_values(model, DeclarationId(index as u32), output)?;
        write_authored(model, DeclarationId(index as u32), output)?;
        writeln!(output, ")")?;
    }
    writeln!(output, "  )")
}

/// Renders the authored short name, modifiers, portion kind, direction, and multiplicity of one
/// declaration.
///
/// Only facts that are actually present are emitted, so a declaration whose parser node carries
/// none of them renders exactly as it did before this fact family existed.
fn write_declaration_facts(
    model: &ResolvedSemanticModel,
    declaration: DeclarationId,
    output: &mut dyn fmt::Write,
) -> fmt::Result {
    let Some(facts) = model.storage.declaration_facts(declaration) else {
        return Ok(());
    };
    let modifiers = declaration_modifier_names(&facts.modifiers);
    if facts.short_name.is_none()
        && modifiers.is_empty()
        && facts.portion_kind.is_none()
        && facts.direction.is_none()
        && facts.multiplicity.is_none()
    {
        return Ok(());
    }
    output.write_str(" (facts")?;
    if let Some(short_name) = facts.short_name {
        let text = model.storage.symbol(short_name).ok_or(fmt::Error)?;
        write!(output, " (short-name {text:?})")?;
    }
    if !modifiers.is_empty() {
        output.write_str(" (modifiers")?;
        for name in modifiers {
            write!(output, " {name}")?;
        }
        output.write_char(')')?;
    }
    if let Some(portion_kind) = facts.portion_kind {
        write!(output, " (portion {})", portion_kind_name(portion_kind))?;
    }
    if let Some(direction) = facts.direction {
        write!(output, " (direction {})", parameter_direction(direction))?;
    }
    if let Some(multiplicity) = &facts.multiplicity {
        output.write_str(" (multiplicity (lower ")?;
        write_multiplicity_bound(multiplicity.lower, output)?;
        output.write_str(") (upper ")?;
        write_multiplicity_bound(multiplicity.upper, output)?;
        output.write_str("))")?;
    }
    output.write_char(')')
}

fn write_multiplicity_bound(bound: MultiplicityBound, output: &mut dyn fmt::Write) -> fmt::Result {
    match bound {
        MultiplicityBound::Unbounded => output.write_str("unbounded"),
        MultiplicityBound::Literal(value) => write!(output, "{value}"),
        MultiplicityBound::Expression => output.write_str("expression"),
    }
}

/// The present modifier names in a fixed canonical order, so snapshot output never depends on
/// field or hash ordering.
fn declaration_modifier_names(modifiers: &DeclarationModifiers) -> Vec<&'static str> {
    let candidates = [
        (modifiers.is_abstract, "abstract"),
        (modifiers.variation, "variation"),
        (modifiers.individual, "individual"),
        (modifiers.derived, "derived"),
        (modifiers.end, "end"),
        (modifiers.reference, "reference"),
        (modifiers.constant, "constant"),
        (modifiers.event, "event"),
        (modifiers.standard, "standard"),
        (modifiers.all, "all"),
        (modifiers.composite, "composite"),
        (modifiers.portion, "portion"),
        (modifiers.var, "var"),
        (modifiers.member, "member"),
        (modifiers.ordered, "ordered"),
        (modifiers.nonunique, "nonunique"),
    ];
    candidates
        .into_iter()
        .filter_map(|(present, name)| present.then_some(name))
        .collect()
}

fn portion_kind_name(kind: PortionKind) -> &'static str {
    match kind {
        PortionKind::Snapshot => "snapshot",
        PortionKind::Timeslice => "timeslice",
    }
}

fn annotation_form_name(form: AnnotationForm) -> &'static str {
    match form {
        AnnotationForm::Documentation => "doc",
        AnnotationForm::Comment => "comment",
        AnnotationForm::TextualRepresentation => "rep",
    }
}

/// Renders the `doc`/`comment`/`rep` annotations bound to one declaration, in authored order.
fn write_documentation(
    model: &ResolvedSemanticModel,
    declaration: DeclarationId,
    output: &mut dyn fmt::Write,
) -> fmt::Result {
    let mut wrote_header = false;
    for record in model
        .storage
        .documentation
        .iter()
        .filter(|record| record.declaration == declaration)
    {
        if !wrote_header {
            output.write_str(" (documentation")?;
            wrote_header = true;
        }
        write!(output, " ({}", annotation_form_name(record.form))?;
        if let Some(locale) = record.locale {
            let text = model.storage.symbol(locale).ok_or(fmt::Error)?;
            write!(output, " (locale {text:?})")?;
        }
        if let Some(language) = record.language {
            let text = model.storage.symbol(language).ok_or(fmt::Error)?;
            write!(output, " (language {text:?})")?;
        }
        let text = model.storage.symbol(record.text).ok_or(fmt::Error)?;
        write!(output, " (text {text:?}))")?;
    }
    if wrote_header {
        output.write_char(')')?;
    }
    Ok(())
}

/// Renders the authored feature-value spelling(s) of one declaration.
fn write_feature_values(
    model: &ResolvedSemanticModel,
    declaration: DeclarationId,
    output: &mut dyn fmt::Write,
) -> fmt::Result {
    for record in model
        .storage
        .feature_values
        .iter()
        .filter(|record| record.declaration == declaration)
    {
        let kind = match record.kind {
            FeatureValueKind::Bind => "bind",
            FeatureValueKind::Assign => "assign",
        };
        // Deliberately not `(value ...)`: that head is already the evaluation section's computed
        // value, and this is the authored spelling of the value clause.
        write!(output, " (feature-value (kind {kind})")?;
        if record.is_default {
            output.write_str(" (default true)")?;
        }
        if !record.has_operator {
            output.write_str(" (operator false)")?;
        }
        output.write_char(')')?;
    }
    Ok(())
}

fn write_references(model: &ResolvedSemanticModel, output: &mut dyn fmt::Write) -> fmt::Result {
    writeln!(output, "  (references")?;
    for index in canonical_reference_indices(model) {
        let reference = &model.storage.references[index];
        let id = AuthoredReferenceId(index as u32);
        write!(output, "    (reference (id (source ")?;
        write_node_identity(model, reference.source, output)?;
        writeln!(
            output,
            ") (kind {}) (ordinal {}))",
            reference_kind(reference.kind),
            reference.ordinal,
        )?;
        write!(output, "      (authored-target ")?;
        write_reference_path(model, reference.path, output)?;
        writeln!(output, ")")?;
        write!(output, "      ")?;
        write_outcome(model, id, output)?;
        writeln!(output, ")")?;
    }
    writeln!(output, "  )")
}

fn write_relationships(model: &ResolvedSemanticModel, output: &mut dyn fmt::Write) -> fmt::Result {
    writeln!(output, "  (relationships")?;
    for index in canonical_reference_indices(model) {
        let reference = &model.storage.references[index];
        let id = AuthoredReferenceId(index as u32);
        let Some(ResolutionStatus::Resolved(target)) = model.resolution.outcome(id) else {
            continue;
        };
        let Some(kind) = relationship_kind(reference.kind) else {
            continue;
        };
        write!(output, "    (relationship (kind {kind})")?;
        if reference.flags.conjugated {
            write!(output, " (conjugated true)")?;
        }
        if reference.flags.variation {
            write!(output, " (variation true)")?;
        }
        if let Some(direction) = reference.flags.direction {
            write!(output, " (direction {})", parameter_direction(direction))?;
        }
        write!(output, " (source ")?;
        write_node_identity(model, reference.source, output)?;
        write!(output, ") (target ")?;
        write_node_identity(model, target, output)?;
        write!(
            output,
            ") (provenance authored) (authored-reference (source "
        )?;
        write_node_identity(model, reference.source, output)?;
        writeln!(
            output,
            ") (kind {}) (ordinal {})))",
            reference_kind(reference.kind),
            reference.ordinal,
        )?;
    }
    let mut implied: Vec<&ImpliedRelationship> =
        model.resolution.implied_relationships.iter().collect();
    implied.sort_by_key(|relationship| {
        (
            declaration_path_key(model, relationship.source),
            declaration_path_key(model, relationship.target),
        )
    });
    for relationship in implied {
        let Some(kind) = relationship_kind(relationship.kind) else {
            continue;
        };
        write!(output, "    (relationship (kind {kind}) (source ")?;
        write_node_identity(model, relationship.source, output)?;
        write!(output, ") (target ")?;
        write_node_identity(model, relationship.target, output)?;
        writeln!(output, ") (provenance implied))")?;
    }
    writeln!(output, "  )")
}

fn write_evaluation(model: &ResolvedSemanticModel, output: &mut dyn fmt::Write) -> fmt::Result {
    writeln!(output, "  (evaluation")?;
    for index in canonical_evaluation_indices(model) {
        let fact = &model.evaluation[index];
        write!(output, "    (evaluated (declaration ")?;
        write_node_identity(model, fact.declaration, output)?;
        write!(output, ") ")?;
        write_evaluated_value(&fact.outcome, output)?;
        writeln!(output, ")")?;
    }
    writeln!(output, "  )")
}

fn write_evaluated_value(value: &EvaluatedValue, output: &mut dyn fmt::Write) -> fmt::Result {
    match value {
        EvaluatedValue::Boolean(value) => {
            write!(output, "(value (kind boolean) (boolean {value}))")
        }
        EvaluatedValue::Integer(value) => {
            write!(output, "(value (kind integer) (integer {value}))")
        }
        EvaluatedValue::Real(value) => write!(output, "(value (kind real) (real {value}))"),
        EvaluatedValue::String(value) => {
            write!(output, "(value (kind string) (value {value:?}))")
        }
        EvaluatedValue::Quantity(magnitude, unit) => {
            write!(output, "(value (kind quantity) (magnitude ")?;
            write_evaluated_value(magnitude, output)?;
            write!(output, ") (unit {unit:?}))")
        }
        EvaluatedValue::NotEvaluated => write!(output, "(value (kind not-evaluated))"),
        EvaluatedValue::UnresolvedOperand => write!(output, "(value (kind unresolved-operand))"),
        EvaluatedValue::NonConstant => write!(output, "(value (kind non-constant))"),
        EvaluatedValue::NonConverged => write!(output, "(value (kind non-converged))"),
        EvaluatedValue::DivisionByZero => write!(output, "(value (kind division-by-zero))"),
        EvaluatedValue::TypeMismatch => write!(output, "(value (kind type-mismatch))"),
    }
}

fn canonical_evaluation_indices(model: &ResolvedSemanticModel) -> Vec<usize> {
    let mut indices = (0..model.evaluation.len()).collect::<Vec<_>>();
    indices.sort_by(|left, right| {
        let left_fact = &model.evaluation[*left];
        let right_fact = &model.evaluation[*right];
        let left_document = model
            .storage
            .declaration(left_fact.declaration)
            .map(|declaration| document_identity(model, declaration.document))
            .unwrap_or("<invalid-document>");
        let right_document = model
            .storage
            .declaration(right_fact.declaration)
            .map(|declaration| document_identity(model, declaration.document))
            .unwrap_or("<invalid-document>");
        left_document
            .cmp(right_document)
            .then_with(|| {
                declaration_path_key(model, left_fact.declaration)
                    .cmp(&declaration_path_key(model, right_fact.declaration))
            })
            .then_with(|| left.cmp(right))
    });
    indices
}

fn write_navigation(model: &ResolvedSemanticModel, output: &mut dyn fmt::Write) -> fmt::Result {
    writeln!(output, "(navigation")?;
    for index in canonical_reference_indices(model) {
        let reference = &model.storage.references[index];
        let id = AuthoredReferenceId(index as u32);
        let source = model
            .storage
            .declaration(reference.source)
            .ok_or(fmt::Error)?;
        let range = document_range(&model.storage, source.document, &reference.span)
            .map_err(|_| fmt::Error)?;
        write!(output, "  (query (document ")?;
        write_quoted(output, document_identity(model, source.document))?;
        write!(output, ") (range ")?;
        write_range(output, range)?;
        write!(
            output,
            ") (probe (position {} {}))\n    (reference (id (source ",
            range.start.line, range.start.character,
        )?;
        write_node_identity(model, reference.source, output)?;
        write!(
            output,
            ") (kind {}) (ordinal {}) (authored-target ",
            reference_kind(reference.kind),
            reference.ordinal,
        )?;
        write_reference_path(model, reference.path, output)?;
        write!(output, ")\n      ")?;
        write_outcome(model, id, output)?;
        writeln!(output, ")\n  )")?;
    }
    write!(output, ")")
}

fn write_outcome(
    model: &ResolvedSemanticModel,
    id: AuthoredReferenceId,
    output: &mut dyn fmt::Write,
) -> fmt::Result {
    match model.resolution.outcome(id).ok_or(fmt::Error)? {
        ResolutionStatus::Resolved(target) => {
            output.write_str("(outcome (status resolved) (target ")?;
            write_node_identity(model, target, output)?;
            output.write_str("))")
        }
        ResolutionStatus::Unresolved => output.write_str("(outcome (status unresolved))"),
        ResolutionStatus::Unsupported => output.write_str("(outcome (status unsupported))"),
        ResolutionStatus::NonConverged => output.write_str("(outcome (status nonConverged))"),
        ResolutionStatus::Ambiguous(range) => {
            output.write_str("(outcome (status ambiguous) (candidates")?;
            for candidate in model.resolution.ambiguous_candidates(range) {
                output.write_char(' ')?;
                write_node_identity(model, *candidate, output)?;
            }
            output.write_str("))")
        }
    }
}

fn write_authored(
    model: &ResolvedSemanticModel,
    source: DeclarationId,
    output: &mut dyn fmt::Write,
) -> fmt::Result {
    let mut references = model
        .storage
        .references
        .iter()
        .filter(|reference| reference.source == source)
        .collect::<Vec<_>>();
    if references.is_empty() {
        return Ok(());
    }
    references.sort_by_key(|reference| (reference.kind, reference.ordinal));
    output.write_str(" (authored")?;
    if let Some(membership) = model
        .storage
        .memberships
        .iter()
        .find(|membership| membership.member == source)
    {
        write!(
            output,
            " (membership (kind {}) (visibility {}))",
            membership_kind(membership.kind),
            visibility(membership.visibility),
        )?;
    }
    output.write_str(" (relationships")?;
    for reference in references {
        write!(output, " ({} (reference ", reference_kind(reference.kind))?;
        write_reference_path(model, reference.path, output)?;
        write!(output, ")")?;
        if reference.flags.conjugated {
            write!(output, " (conjugated true)")?;
        }
        if reference.flags.variation {
            write!(output, " (variation true)")?;
        }
        if let Some(direction) = reference.flags.direction {
            write!(output, " (direction {})", parameter_direction(direction))?;
        }
        if let Some(import) = reference.import {
            write_import(import, output)?;
        }
        output.write_char(')')?;
    }
    output.write_str(")")
}

fn write_import(import: AuthoredImportFacts, output: &mut dyn fmt::Write) -> fmt::Result {
    write!(
        output,
        " (import (shape {}) (recursive {}))",
        import_shape(import.shape),
        import.recursive,
    )
}

fn import_shape(shape: AuthoredImportShape) -> &'static str {
    match shape {
        AuthoredImportShape::Membership => "membership",
        AuthoredImportShape::Namespace => "namespace",
        AuthoredImportShape::Filter => "filtered-namespace",
    }
}

fn write_declaration_name(
    model: &ResolvedSemanticModel,
    id: DeclarationId,
    output: &mut dyn fmt::Write,
) -> fmt::Result {
    output.write_char('"')?;
    write_declaration_name_body(model, id, output)?;
    output.write_char('"')
}

fn write_node_identity(
    model: &ResolvedSemanticModel,
    id: DeclarationId,
    output: &mut dyn fmt::Write,
) -> fmt::Result {
    let declaration = model.storage.declaration(id).ok_or(fmt::Error)?;
    write!(output, "(node (document ")?;
    write_quoted(output, document_identity(model, declaration.document))?;
    if model.identities.allows_qualified_name_shorthand(id) {
        // The readable shorthand, used only where the qualified name recovers the whole identity:
        // every segment named, no same-named sibling, and nothing else rendering the same name.
        output.write_str(") (qualified-name ")?;
        write_declaration_name(model, id, output)?;
    } else {
        output.write_str(") ")?;
        write_declaration_path(model, id, output)?;
    }
    output.write_str("))")
}

/// Renders the explicit root-to-leaf scope path used whenever the qualified name alone would not
/// recover the identity.
///
/// Every segment carries its kind, matching the identity encoding: a `metadata def X` and the
/// `metadata X about ...` annotating it are distinct elements sharing one name, and only the kind
/// separates them.
fn write_declaration_path(
    model: &ResolvedSemanticModel,
    id: DeclarationId,
    output: &mut dyn fmt::Write,
) -> fmt::Result {
    let mut chain = vec![id];
    let mut cursor = model.storage.declaration(id).ok_or(fmt::Error)?.owner;
    while let Some(current) = cursor {
        if chain.len() > model.storage.declarations.len() {
            return Err(fmt::Error);
        }
        chain.push(current);
        cursor = model.storage.declaration(current).ok_or(fmt::Error)?.owner;
    }
    output.write_str("(path")?;
    for current in chain.iter().rev() {
        let declaration = model.storage.declaration(*current).ok_or(fmt::Error)?;
        match declaration.name {
            Some(name) => {
                write!(
                    output,
                    " (named (kind {}) (name ",
                    declaration_kind(declaration.kind)
                )?;
                write_quoted(output, model.storage.symbol(name).ok_or(fmt::Error)?)?;
                output.write_char(')')?;
                let occurrence = model
                    .identities
                    .name_occurrence(*current)
                    .ok_or(fmt::Error)?;
                if occurrence > 0 {
                    // Only a same-named sibling after the first carries this, so a later duplicate
                    // never disturbs the identity already published for the original.
                    write!(output, " (occurrence {occurrence})")?;
                }
                output.write_char(')')?;
            }
            None => write!(
                output,
                " (anonymous (kind {}) (ordinal {}))",
                declaration_kind(declaration.kind),
                declaration.anonymous_ordinal.ok_or(fmt::Error)?,
            )?,
        }
    }
    output.write_char(')')
}

fn write_declaration_name_body(
    model: &ResolvedSemanticModel,
    id: DeclarationId,
    output: &mut dyn fmt::Write,
) -> fmt::Result {
    let declaration = model.storage.declaration(id).ok_or(fmt::Error)?;
    if let Some(owner) = declaration.owner {
        write_declaration_name_body(model, owner, output)?;
        output.write_str("::")?;
    }
    if let Some(name) = declaration.name {
        write_escaped(output, model.storage.symbol(name).ok_or(fmt::Error)?)?;
    }
    Ok(())
}

fn write_reference_path(
    model: &ResolvedSemanticModel,
    id: SymbolPathId,
    output: &mut dyn fmt::Write,
) -> fmt::Result {
    let (segments, rooted) = model.storage.paths.get(id).ok_or(fmt::Error)?;
    output.write_char('"')?;
    if rooted {
        output.write_str("$::")?;
    }
    for (index, segment) in segments.iter().enumerate() {
        if index != 0 {
            output.write_str("::")?;
        }
        write_escaped(output, model.storage.symbol(*segment).ok_or(fmt::Error)?)?;
    }
    output.write_char('"')
}

fn write_quoted(output: &mut dyn fmt::Write, value: &str) -> fmt::Result {
    output.write_char('"')?;
    write_escaped(output, value)?;
    output.write_char('"')
}

fn write_escaped(output: &mut dyn fmt::Write, value: &str) -> fmt::Result {
    for character in value.chars() {
        match character {
            '\\' => output.write_str("\\\\")?,
            '"' => output.write_str("\\\"")?,
            '\n' => output.write_str("\\n")?,
            '\r' => output.write_str("\\r")?,
            '\t' => output.write_str("\\t")?,
            character => output.write_char(character)?,
        }
    }
    Ok(())
}

fn canonical_document_indices(model: &ResolvedSemanticModel) -> Vec<usize> {
    let mut indices = (0..model.storage.documents.len()).collect::<Vec<_>>();
    indices.sort_by(|left, right| {
        model.storage.documents[*left]
            .identity
            .cmp(&model.storage.documents[*right].identity)
            .then_with(|| left.cmp(right))
    });
    indices
}

fn canonical_declaration_indices(model: &ResolvedSemanticModel) -> Vec<usize> {
    let mut indices = (0..model.storage.declarations.len()).collect::<Vec<_>>();
    indices.sort_by(|left, right| {
        let left_declaration = &model.storage.declarations[*left];
        let right_declaration = &model.storage.declarations[*right];
        document_identity(model, left_declaration.document)
            .cmp(document_identity(model, right_declaration.document))
            .then_with(|| {
                declaration_path_key(model, DeclarationId(*left as u32))
                    .cmp(&declaration_path_key(model, DeclarationId(*right as u32)))
            })
            .then_with(|| left.cmp(right))
    });
    indices
}

fn canonical_reference_indices(model: &ResolvedSemanticModel) -> Vec<usize> {
    let mut indices = (0..model.storage.references.len()).collect::<Vec<_>>();
    indices.sort_by(|left, right| {
        let left_reference = &model.storage.references[*left];
        let right_reference = &model.storage.references[*right];
        let left_source = model.storage.declaration(left_reference.source);
        let right_source = model.storage.declaration(right_reference.source);
        let left_document = left_source
            .map(|source| document_identity(model, source.document))
            .unwrap_or("<invalid-document>");
        let right_document = right_source
            .map(|source| document_identity(model, source.document))
            .unwrap_or("<invalid-document>");
        left_document
            .cmp(right_document)
            .then_with(|| {
                declaration_path_key(model, left_reference.source)
                    .cmp(&declaration_path_key(model, right_reference.source))
            })
            .then_with(|| left_reference.kind.cmp(&right_reference.kind))
            .then_with(|| left_reference.ordinal.cmp(&right_reference.ordinal))
            .then_with(|| left.cmp(right))
    });
    indices
}

fn declaration_path_key(model: &ResolvedSemanticModel, id: DeclarationId) -> String {
    let mut path = String::new();
    if write_declaration_name_body(model, id, &mut path).is_err() {
        path.push('\u{fffd}');
    }
    if path.is_empty() {
        if let Some(declaration) = model.storage.declaration(id) {
            path = format!(
                "<anonymous:{}:{}>",
                declaration_kind(declaration.kind),
                declaration.anonymous_ordinal.unwrap_or(u32::MAX)
            );
        }
    }
    path
}

fn write_range(output: &mut dyn fmt::Write, range: TextRange) -> fmt::Result {
    write!(
        output,
        "(start {} {}) (end {} {})",
        range.start.line, range.start.character, range.end.line, range.end.character,
    )
}

fn document_identity(model: &ResolvedSemanticModel, id: DocumentId) -> &str {
    model
        .storage
        .document(id)
        .map_or("<invalid-document>", |document| document.identity.as_ref())
}

fn parameter_direction(direction: ParameterDirection) -> &'static str {
    match direction {
        ParameterDirection::In => "in",
        ParameterDirection::Out => "out",
        ParameterDirection::InOut => "inout",
    }
}

pub(crate) fn declaration_kind(kind: DeclarationKind) -> &'static str {
    match kind {
        DeclarationKind::Namespace => "namespace",
        DeclarationKind::Package => "package",
        DeclarationKind::LibraryPackage => "library-package",
        DeclarationKind::PartDefinition => "part-def",
        DeclarationKind::PartUsage => "part",
        DeclarationKind::AttributeDefinition => "attribute-def",
        DeclarationKind::AttributeUsage => "attribute",
        DeclarationKind::Import => "import",
        DeclarationKind::Alias => "alias",
        DeclarationKind::EnumerationDefinition => "enum-def",
        DeclarationKind::EnumerationUsage => "enum",
        DeclarationKind::EnumerationLiteral => "enum-literal",
        DeclarationKind::RequirementDefinition => "requirement-def",
        DeclarationKind::RequirementUsage => "requirement",
        DeclarationKind::PortDefinition => "port-def",
        DeclarationKind::PortUsage => "port",
        DeclarationKind::ItemDefinition => "item-def",
        DeclarationKind::ItemUsage => "item",
        DeclarationKind::ActionDefinition => "action-def",
        DeclarationKind::ActionUsage => "action",
        DeclarationKind::Succession => "succession",
        DeclarationKind::StateDefinition => "state-def",
        DeclarationKind::StateUsage => "state",
        DeclarationKind::MetadataDefinition => "metadata-def",
        DeclarationKind::MetadataUsage => "metadata",
        DeclarationKind::ConnectionDefinition => "connection-def",
        DeclarationKind::InterfaceDefinition => "interface-def",
        DeclarationKind::ConnectionUsage => "connection",
        DeclarationKind::OccurrenceDefinition => "occurrence-def",
        DeclarationKind::OccurrenceUsage => "occurrence",
        DeclarationKind::AnalysisCaseDefinition => "analysis-def",
        DeclarationKind::AnalysisCaseUsage => "analysis",
        DeclarationKind::ViewDefinition => "view-def",
        DeclarationKind::CaseDefinition => "case-def",
        DeclarationKind::CaseUsage => "case",
        DeclarationKind::VerificationCaseDefinition => "verification-def",
        DeclarationKind::UseCaseDefinition => "use-case-def",
        DeclarationKind::ViewpointDefinition => "viewpoint-def",
        DeclarationKind::RenderingDefinition => "rendering-def",
        DeclarationKind::AllocationDefinition => "allocation-def",
        DeclarationKind::FlowDefinition => "flow-def",
        DeclarationKind::ViewUsage => "view",
        DeclarationKind::RenderingUsage => "rendering",
        DeclarationKind::UseCaseUsage => "use-case",
        DeclarationKind::VerificationCaseUsage => "verification",
        DeclarationKind::ViewpointUsage => "viewpoint",
        DeclarationKind::InterfaceUsage => "interface",
        DeclarationKind::ConstraintDefinition => "constraint-def",
        DeclarationKind::ConstraintUsage => "constraint",
        DeclarationKind::AssertConstraintUsage => "assert-constraint",
        DeclarationKind::AssumeConstraintUsage => "assume-constraint",
        DeclarationKind::RequireConstraintUsage => "require-constraint",
        DeclarationKind::ConcernDefinition => "concern-def",
        DeclarationKind::ConcernUsage => "concern",
        DeclarationKind::CalcDefinition => "calc-def",
        DeclarationKind::CalcUsage => "calc",
        DeclarationKind::ClassDefinition => "class-def",
        DeclarationKind::EntryActionBinding => "entry-action-binding",
        DeclarationKind::DoActionBinding => "do-action-binding",
        DeclarationKind::ExitActionBinding => "exit-action-binding",
        DeclarationKind::InitialState => "initial-state",
        DeclarationKind::FinalState => "final-state",
        DeclarationKind::ParameterUsage => "parameter",
        DeclarationKind::SubjectUsage => "subject",
        DeclarationKind::PerformActionUsage => "perform-action",
        DeclarationKind::Transition => "transition",
        DeclarationKind::Satisfy => "satisfy",
        DeclarationKind::Allocate => "allocate",
        DeclarationKind::Bind => "bind",
        DeclarationKind::ReferenceUsage => "ref",
        DeclarationKind::Decide => "decide",
        DeclarationKind::Merge => "merge",
        DeclarationKind::Fork => "fork",
        DeclarationKind::Join => "join",
        DeclarationKind::ThenContinuation => "then-continuation",
        DeclarationKind::Flow => "flow",
        DeclarationKind::StakeholderUsage => "stakeholder",
        DeclarationKind::RequirementActor => "requirement-actor",
        DeclarationKind::CaseActor => "case-actor",
        DeclarationKind::Frame => "frame",
        DeclarationKind::VerifyRequirement => "verify-requirement",
        // One name per KerML metaclass; see `DeclarationKind`'s own doc comments for why the
        // keyword spellings are distinct metaclasses rather than one bucket.
        DeclarationKind::KermlClassifier => "kerml-classifier",
        DeclarationKind::KermlClass => "kerml-class",
        DeclarationKind::KermlStructure => "kerml-structure",
        DeclarationKind::KermlAssociation => "kerml-association",
        DeclarationKind::KermlAssociationStructure => "kerml-association-structure",
        DeclarationKind::KermlDataType => "kerml-datatype",
        DeclarationKind::KermlMetaclass => "kerml-metaclass",
        DeclarationKind::KermlBehavior => "kerml-behavior",
        DeclarationKind::KermlFunction => "kerml-function",
        DeclarationKind::KermlPredicate => "kerml-predicate",
        DeclarationKind::KermlInteraction => "kerml-interaction",
        DeclarationKind::KermlMultiplicity => "kerml-multiplicity",
        DeclarationKind::KermlStep => "kerml-step",
        DeclarationKind::KermlExpression => "kerml-expression",
        DeclarationKind::KermlBooleanExpression => "kerml-boolean-expression",
        DeclarationKind::KermlFeature => "kerml-feature",
        DeclarationKind::DefaultReferenceUsage => "default-reference",
        DeclarationKind::KermlConnector => "kerml-connector",
        DeclarationKind::KermlBinding => "kerml-binding",
        DeclarationKind::KermlInvariant => "kerml-invariant",
        DeclarationKind::KermlEnd => "kerml-end",
        DeclarationKind::Assign => "assign",
        DeclarationKind::While => "while",
        DeclarationKind::Loop => "loop",
        DeclarationKind::If => "if",
        DeclarationKind::ForLoop => "for-loop",
        DeclarationKind::ForLoopVariable => "for-loop-variable",
        DeclarationKind::Dependency => "dependency",
        DeclarationKind::ExtendedDefinition => "extended-definition",
        DeclarationKind::IndividualDefinition => "individual-definition",
        DeclarationKind::BareConnect => "bare-connect",
        DeclarationKind::PerformParameterBinding => "perform-parameter-binding",
    }
}

fn membership_kind(kind: MembershipKind) -> &'static str {
    match kind {
        MembershipKind::Owning => "owning",
        MembershipKind::Feature => "feature",
        MembershipKind::Import => "import",
        MembershipKind::Alias => "alias",
    }
}

fn visibility(value: Visibility) -> &'static str {
    match value {
        Visibility::Default => "default",
        Visibility::Public => "public",
        Visibility::Private => "private",
        Visibility::Protected => "protected",
    }
}

fn reference_kind(kind: ReferenceKind) -> &'static str {
    match kind {
        ReferenceKind::NamespaceImport => "namespaceImport",
        ReferenceKind::MembershipImport => "membershipImport",
        ReferenceKind::FilterImport => "filterImport",
        ReferenceKind::FeatureTyping => "featureTyping",
        ReferenceKind::Subclassification => "specialization",
        ReferenceKind::Subsetting => "subsetting",
        ReferenceKind::Redefinition => "redefinition",
        ReferenceKind::References => "referenceSubsetting",
        ReferenceKind::Crosses => "crossSubsetting",
        ReferenceKind::Intersects => "intersects",
        ReferenceKind::AliasBinding => "aliasBinding",
        ReferenceKind::ConnectorEnd => "connectorEnd",
        ReferenceKind::Succession => "succession",
        ReferenceKind::EntryActionBinding => "entryActionBinding",
        ReferenceKind::DoActionBinding => "doActionBinding",
        ReferenceKind::ExitActionBinding => "exitActionBinding",
        ReferenceKind::InitialState => "initialState",
        ReferenceKind::ExpressionOperand => "expressionOperand",
        ReferenceKind::TransitionSource => "transitionSource",
        ReferenceKind::TransitionTarget => "transitionTarget",
        ReferenceKind::TransitionTrigger => "transitionTrigger",
        ReferenceKind::TransitionEffect => "transitionEffect",
        ReferenceKind::MetadataAnnotation => "metadataAnnotation",
        ReferenceKind::FilterMetadataTest => "filterMetadataTest",
        ReferenceKind::SatisfySource => "satisfySource",
        ReferenceKind::SatisfyTarget => "satisfyTarget",
        ReferenceKind::AllocateSource => "allocateSource",
        ReferenceKind::AllocateTarget => "allocateTarget",
        ReferenceKind::BindSource => "bindSource",
        ReferenceKind::BindTarget => "bindTarget",
        ReferenceKind::SatisfyViewpoint => "satisfyViewpoint",
        ReferenceKind::Variant => "variant",
        ReferenceKind::IncludeUseCase => "includeUseCase",
        ReferenceKind::MemberAccessOperand => "memberAccessOperand",
        ReferenceKind::InvocationCallee => "invocationCallee",
        ReferenceKind::DecisionInput => "decisionInput",
        ReferenceKind::MergeInput => "mergeInput",
        ReferenceKind::ForkInput => "forkInput",
        ReferenceKind::JoinInput => "joinInput",
        ReferenceKind::ThenTarget => "thenTarget",
        ReferenceKind::AcceptVia => "acceptVia",
        ReferenceKind::SendTarget => "sendTarget",
        ReferenceKind::AcceptPayloadType => "acceptPayloadType",
        ReferenceKind::TerminateTarget => "terminateTarget",
        ReferenceKind::FlowSource => "flowSource",
        ReferenceKind::FlowTarget => "flowTarget",
        ReferenceKind::TypeCheckTarget => "typeCheckTarget",
        ReferenceKind::MetaCastTarget => "metaCastTarget",
        ReferenceKind::StakeholderTarget => "stakeholderTarget",
        ReferenceKind::PurposeTarget => "purposeTarget",
        ReferenceKind::VerifyRequirementTarget => "verifyRequirementTarget",
        ReferenceKind::AssignTarget => "assignTarget",
        ReferenceKind::DependencyClient => "dependencyClient",
        ReferenceKind::DependencySupplier => "dependencySupplier",
        ReferenceKind::PerformParameterTarget => "performParameterTarget",
        ReferenceKind::FlowPayloadType => "flowPayloadType",
    }
}

fn relationship_kind(kind: ReferenceKind) -> Option<&'static str> {
    match kind {
        ReferenceKind::FeatureTyping => Some("typing"),
        ReferenceKind::Subclassification => Some("specialization"),
        ReferenceKind::Subsetting => Some("subsetting"),
        ReferenceKind::Redefinition => Some("redefinition"),
        ReferenceKind::References => Some("referenceSubsetting"),
        ReferenceKind::Crosses => Some("crossSubsetting"),
        ReferenceKind::Intersects => Some("intersects"),
        ReferenceKind::AliasBinding => Some("aliasBinding"),
        ReferenceKind::ConnectorEnd => Some("connectorEnd"),
        ReferenceKind::Succession => Some("succession"),
        ReferenceKind::EntryActionBinding => Some("entryActionBinding"),
        ReferenceKind::DoActionBinding => Some("doActionBinding"),
        ReferenceKind::ExitActionBinding => Some("exitActionBinding"),
        ReferenceKind::InitialState => Some("initialState"),
        ReferenceKind::ExpressionOperand => Some("expressionOperand"),
        ReferenceKind::TransitionSource => Some("transitionSource"),
        ReferenceKind::TransitionTarget => Some("transitionTarget"),
        ReferenceKind::TransitionTrigger => Some("transitionTrigger"),
        ReferenceKind::TransitionEffect => Some("transitionEffect"),
        ReferenceKind::MetadataAnnotation => Some("metadataAnnotation"),
        ReferenceKind::FilterMetadataTest => Some("filterMetadataTest"),
        ReferenceKind::SatisfySource => Some("satisfySource"),
        ReferenceKind::SatisfyTarget => Some("satisfyTarget"),
        ReferenceKind::AllocateSource => Some("allocateSource"),
        ReferenceKind::AllocateTarget => Some("allocateTarget"),
        ReferenceKind::BindSource => Some("bindSource"),
        ReferenceKind::BindTarget => Some("bindTarget"),
        ReferenceKind::SatisfyViewpoint => Some("satisfyViewpoint"),
        ReferenceKind::Variant => Some("variant"),
        ReferenceKind::IncludeUseCase => Some("includeUseCase"),
        ReferenceKind::MemberAccessOperand => Some("memberAccessOperand"),
        ReferenceKind::InvocationCallee => Some("invocationCallee"),
        ReferenceKind::DecisionInput => Some("decisionInput"),
        ReferenceKind::MergeInput => Some("mergeInput"),
        ReferenceKind::ForkInput => Some("forkInput"),
        ReferenceKind::JoinInput => Some("joinInput"),
        ReferenceKind::ThenTarget => Some("thenTarget"),
        ReferenceKind::AcceptVia => Some("acceptVia"),
        ReferenceKind::SendTarget => Some("sendTarget"),
        ReferenceKind::AcceptPayloadType => Some("acceptPayloadType"),
        ReferenceKind::TerminateTarget => Some("terminateTarget"),
        ReferenceKind::FlowSource => Some("flowSource"),
        ReferenceKind::FlowTarget => Some("flowTarget"),
        ReferenceKind::TypeCheckTarget => Some("typeCheckTarget"),
        ReferenceKind::MetaCastTarget => Some("metaCastTarget"),
        ReferenceKind::StakeholderTarget => Some("stakeholderTarget"),
        ReferenceKind::PurposeTarget => Some("purposeTarget"),
        ReferenceKind::VerifyRequirementTarget => Some("verifyRequirementTarget"),
        ReferenceKind::AssignTarget => Some("assignTarget"),
        ReferenceKind::DependencyClient => Some("dependencyClient"),
        ReferenceKind::DependencySupplier => Some("dependencySupplier"),
        ReferenceKind::PerformParameterTarget => Some("performParameterTarget"),
        ReferenceKind::FlowPayloadType => Some("flowPayloadType"),
        ReferenceKind::NamespaceImport
        | ReferenceKind::MembershipImport
        | ReferenceKind::FilterImport => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_resolved_model_writes_all_owned_sections() {
        let storage = SemanticModelStorage {
            documents: Box::new([]),
            declarations: Box::new([]),
            declaration_facts: Box::new([]),
            memberships: Box::new([]),
            references: Box::new([]),
            documentation: Box::new([]),
            feature_values: Box::new([]),
            unsupported: Box::new([]),
            recovery: Box::new([]),
            symbols: SymbolTableBuilder::default().freeze(),
            paths: SymbolPathArenaBuilder::default().freeze(),
            evaluation_facts: Box::new([]),
        };
        let (direct_names, effective_imports, resolution) = resolve_dense(
            &storage.declarations,
            &storage.memberships,
            &storage.paths,
            &storage.references,
        )
        .unwrap();
        let evaluation = compute_evaluation(&storage, &resolution);
        let identities = IdentityIndex::build(&storage).unwrap();
        let model = ResolvedSemanticModel {
            storage,
            direct_names,
            effective_imports,
            identities,
            resolution,
            evaluation,
            metadata: PublicationMetadata {
                phase: PublicationPhase::Resolved,
                completeness: PublicationCompleteness::Complete,
                has_evaluation: false,
            },
        };
        let mut output = String::new();
        model
            .write_semantic_sexpr(
                &source_identity::RootDigest::of_bytes(b"test"),
                "test-contract",
                &mut output,
            )
            .unwrap();
        assert!(output.starts_with("(semantic-model\n"));
        assert!(output.contains("  (declarations\n  )"));
        assert!(output.contains("  (references\n  )"));
        assert!(output.contains("  (relationships\n  )"));
        assert!(output.contains("  (evaluation\n  )"));
        output.clear();
        model.write_navigation_sexpr(&mut output).unwrap();
        assert_eq!(output, "(navigation\n)");
    }
}

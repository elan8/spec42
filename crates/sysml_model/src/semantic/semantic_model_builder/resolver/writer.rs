//! Owner-side canonical rendering for the private resolved semantic model.
//!
//! The writer accepts only the resolved owner and a caller-provided `fmt::Write`. It does not
//! expose storage collections or return projections that could become a second semantic model.

use std::fmt;

use super::*;

pub(super) fn write(model: &ResolvedSemanticModel, output: &mut dyn fmt::Write) -> fmt::Result {
    writeln!(output, "(resolved-semantic-model")?;
    write_metadata(model, output)?;
    write_declarations(model, output)?;
    write_references(model, output)?;
    write_relationships(model, output)?;
    write_navigation(model, output)?;
    write!(output, ")")
}

pub(super) fn write_semantic(
    model: &ResolvedSemanticModel,
    output: &mut dyn fmt::Write,
) -> fmt::Result {
    writeln!(output, "(semantic-model")?;
    write_metadata(model, output)?;
    write_declarations(model, output)?;
    write_references(model, output)?;
    write_relationships(model, output)?;
    writeln!(output, "  (evaluation\n  )")?;
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
    let mut records = model.diagnostic_records().map_err(|_| fmt::Error)?;
    records.sort_by_key(|record| {
        (
            document_identity_for_source(model, record.source),
            record.range.start.line,
            record.range.start.character,
            record.range.end.line,
            record.range.end.character,
            record.kind,
            record.reference,
        )
    });
    writeln!(output, "(fixture-diagnostics")?;
    for document_index in canonical_document_indices(model) {
        let document = &model.storage.documents[document_index];
        writeln!(output, "  (document {:?}", document.identity)?;
        writeln!(output, "    (diagnostics")?;
        for record in records.iter().filter(|record| {
            document_identity_for_source(model, record.source) == document.identity.as_ref()
        }) {
            let Some((severity, code)) = diagnostic_label(record.kind, &record.outcome) else {
                continue;
            };
            writeln!(
                output,
                "      (diagnostic\n        (severity {severity})\n        (code \"{code}\")\n        (source \"semantic\")\n        (range (start {} {}) (end {} {}))\n      )",
                record.range.start.line,
                record.range.start.character,
                record.range.end.line,
                record.range.end.character,
            )?;
            if let DiagnosticOutcome::Ambiguous { candidates } = &record.outcome {
                writeln!(output, "        (related-information")?;
                for candidate in candidates {
                    let declaration = model
                        .storage
                        .declaration(candidate.target)
                        .ok_or(fmt::Error)?;
                    let range =
                        document_range(&model.storage, declaration.document, &declaration.span)
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
                        range.start.line,
                        range.start.character,
                        range.end.line,
                        range.end.character,
                    )?;
                    writeln!(output, "          )")?;
                }
                writeln!(output, "        )")?;
            }
        }
        writeln!(output, "    )\n  )")?;
    }
    write!(output, ")")
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

fn document_identity_for_source(model: &ResolvedSemanticModel, source: DeclarationId) -> &str {
    let declaration = model.storage.declaration(source).ok_or(()).ok();
    declaration
        .map(|declaration| document_identity(model, declaration.document))
        .unwrap_or("<invalid-document>")
}

fn write_metadata(model: &ResolvedSemanticModel, output: &mut dyn fmt::Write) -> fmt::Result {
    let phase = match model.metadata.phase {
        PublicationPhase::Resolved => "resolved",
    };
    let completeness = match model.metadata.completeness {
        PublicationCompleteness::Complete => "complete",
    };
    writeln!(
        output,
        "  (publication (phase {phase}) (completeness {completeness}) (has-evaluation {}))",
        model.metadata.has_evaluation
    )
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
        write_authored(model, DeclarationId(index as u32), output)?;
        writeln!(output, ")")?;
    }
    writeln!(output, "  )")
}

fn write_references(model: &ResolvedSemanticModel, output: &mut dyn fmt::Write) -> fmt::Result {
    writeln!(output, "  (references")?;
    for index in canonical_reference_indices(model) {
        let reference = &model.storage.references[index];
        let id = AuthoredReferenceId(index as u32);
        writeln!(output, "    (reference (id (source ",)?;
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
        writeln!(output, "    (relationship (kind {kind}) (source ",)?;
        write_node_identity(model, reference.source, output)?;
        write!(output, ") (target ")?;
        write_node_identity(model, target, output)?;
        writeln!(
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
    writeln!(output, "  )")
}

fn write_navigation(model: &ResolvedSemanticModel, output: &mut dyn fmt::Write) -> fmt::Result {
    writeln!(output, "  (navigation")?;
    for index in canonical_reference_indices(model) {
        let reference = &model.storage.references[index];
        let id = AuthoredReferenceId(index as u32);
        let source = model
            .storage
            .declaration(reference.source)
            .ok_or(fmt::Error)?;
        let range = document_range(&model.storage, source.document, &reference.span)
            .map_err(|_| fmt::Error)?;
        write!(output, "    (query (document ")?;
        write_quoted(output, document_identity(model, source.document))?;
        write!(output, ") (range ")?;
        write_range(output, range)?;
        write!(
            output,
            ") (probe (position {} {}))\n      (reference (id (source ",
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
        write!(output, ")\n        ")?;
        write_outcome(model, id, output)?;
        writeln!(output, ")\n    )")?;
    }
    writeln!(output, "  )")
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
    if declaration.name.is_some() {
        output.write_str(") (qualified-name ")?;
        write_declaration_name(model, id, output)?;
    } else {
        write!(
            output,
            ") (anonymous (kind {}) (ordinal {}))",
            declaration_kind(declaration.kind),
            id.0,
        )?;
    }
    output.write_str("))")
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
                id.0
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

fn reference_ordinal(model: &ResolvedSemanticModel, index: usize) -> usize {
    let reference = &model.storage.references[index];
    model.storage.references[..index]
        .iter()
        .filter(|candidate| {
            candidate.source == reference.source && candidate.kind == reference.kind
        })
        .count()
}

fn declaration_kind(kind: DeclarationKind) -> &'static str {
    match kind {
        DeclarationKind::Namespace => "namespace",
        DeclarationKind::Package => "package",
        DeclarationKind::LibraryPackage => "library-package",
        DeclarationKind::PartDefinition => "part-def",
        DeclarationKind::PartUsage => "part",
        DeclarationKind::AttributeDefinition => "attribute-def",
        DeclarationKind::AttributeUsage => "attribute",
        DeclarationKind::Import => "import",
    }
}

fn membership_kind(kind: MembershipKind) -> &'static str {
    match kind {
        MembershipKind::Owning => "owning",
        MembershipKind::Feature => "feature",
        MembershipKind::Import => "import",
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
            memberships: Box::new([]),
            references: Box::new([]),
            unsupported: Box::new([]),
            recovery: Box::new([]),
            symbols: SymbolTableBuilder::default().freeze(),
            paths: SymbolPathArenaBuilder::default().freeze(),
        };
        let (direct_names, effective_imports, resolution) = resolve_dense(
            &storage.declarations,
            &storage.memberships,
            &storage.paths,
            &storage.references,
        )
        .unwrap();
        let model = ResolvedSemanticModel {
            storage,
            direct_names,
            effective_imports,
            resolution,
            metadata: PublicationMetadata {
                phase: PublicationPhase::Resolved,
                completeness: PublicationCompleteness::Complete,
                has_evaluation: false,
            },
        };
        let mut output = String::new();
        model.write_debug_sexpr(&mut output).unwrap();
        assert!(output.starts_with("(resolved-semantic-model\n"));
        assert!(output.contains("  (declarations\n  )"));
        assert!(output.contains("  (references\n  )"));
        assert!(output.contains("  (relationships\n  )"));
        assert!(output.contains("  (navigation\n  )"));
    }
}

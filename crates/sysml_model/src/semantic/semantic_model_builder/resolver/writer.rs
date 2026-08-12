//! Owner-side canonical rendering for the private resolved semantic model.
//!
//! The writer accepts only the resolved owner and a caller-provided `fmt::Write`. It does not
//! expose storage collections or return projections that could become a second semantic model.

use std::fmt;

use super::*;

pub(super) fn write(model: &ResolvedSemanticModel, output: &mut dyn fmt::Write) -> fmt::Result {
    writeln!(output, "(resolved-semantic-model")?;
    write_declarations(model, output)?;
    write_references(model, output)?;
    write_relationships(model, output)?;
    write_navigation(model, output)?;
    write!(output, ")")
}

fn write_declarations(model: &ResolvedSemanticModel, output: &mut dyn fmt::Write) -> fmt::Result {
    writeln!(output, "  (declarations")?;
    for (index, declaration) in model.storage.declarations.iter().enumerate() {
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
        writeln!(output, ")")?;
    }
    writeln!(output, "  )")
}

fn write_references(model: &ResolvedSemanticModel, output: &mut dyn fmt::Write) -> fmt::Result {
    writeln!(output, "  (references")?;
    for (index, reference) in model.storage.references.iter().enumerate() {
        let id = AuthoredReferenceId(index as u32);
        writeln!(output, "    (reference (id (source ",)?;
        write_node_identity(model, reference.source, output)?;
        writeln!(
            output,
            ") (kind {}) (ordinal {}))",
            reference_kind(reference.kind),
            reference_ordinal(model, index),
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
    for (index, reference) in model.storage.references.iter().enumerate() {
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
            ") (provenance authored) (authored-reference {index}))"
        )?;
    }
    writeln!(output, "  )")
}

fn write_navigation(model: &ResolvedSemanticModel, output: &mut dyn fmt::Write) -> fmt::Result {
    writeln!(output, "  (navigation")?;
    for (index, reference) in model.storage.references.iter().enumerate() {
        let id = AuthoredReferenceId(index as u32);
        let source = model
            .storage
            .declaration(reference.source)
            .ok_or(fmt::Error)?;
        write!(output, "    (query (document ")?;
        write_quoted(output, document_identity(model, source.document))?;
        write!(output, ") (range ")?;
        write_span(output, &reference.span)?;
        write!(
            output,
            ") (probe (position {} {}))\n      (reference (id (source ",
            reference.span.line, reference.span.column,
        )?;
        write_node_identity(model, reference.source, output)?;
        write!(
            output,
            ") (kind {}) (ordinal {}) (authored-target ",
            reference_kind(reference.kind),
            reference_ordinal(model, index),
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
    output.write_str(") (qualified-name ")?;
    write_declaration_name(model, id, output)?;
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

fn write_span(output: &mut dyn fmt::Write, span: &sysml_v2_parser_next::ast::Span) -> fmt::Result {
    write!(
        output,
        "(start {} {}) (end {} {})",
        span.line,
        span.column,
        span.line,
        span.column.saturating_add(span.len)
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

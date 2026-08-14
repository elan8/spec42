//! Canonical S-expression projection of the typed semantic graph.
//!
//! This is a diagnostic projection, not a serialization format. It reads only
//! semantic graph state: parser trees, source ranges, caches, and display-oriented
//! attributes are excluded. Ordering is canonical, so construction order does not
//! affect the rendering.

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write;

use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use url::Url;

use crate::semantic::graph::{PendingExpressionRelationship, PendingRelationship, SemanticGraph};
use crate::semantic::model::{
    DeclaredExpression, DeclaredFeatureProperties, DeclaredFeatureValueKind,
    DeclaredMultiplicityBound, DerivedRelationshipResolution, DerivedRelationshipRule,
    ExpressionResultRole, ImpliedRelationshipRule, ImportOrigin, ImportShape, NodeId,
    RelationshipProvenance, SemanticEdge, SemanticNode,
};
use crate::semantic::publication::{
    ImportConformanceOutcome, ReferenceKind, ResolutionOutcome, ResolutionProvenance,
    ResolvedRelationship, SemanticModel, SemanticModelCompleteness, SemanticModelPhase,
};

const FORMAT_ROOT: &str = "semantic-graph";

struct CanonicalIdentities {
    document_labels: HashMap<Url, String>,
}

impl CanonicalIdentities {
    fn from_graph(graph: &SemanticGraph) -> Self {
        let mut document_facts: HashMap<Url, Vec<String>> = HashMap::new();
        for (uri, ids) in &graph.nodes_by_uri {
            let facts = document_facts.entry(uri.clone()).or_default();
            facts.extend(
                ids.iter()
                    .filter_map(|id| graph.get_node(id))
                    .map(|node| format!("node:{:?}", node_sort_key(node))),
            );
        }
        for pending in &graph.pending_relationships {
            let mut target_kinds = pending
                .target_kinds
                .as_ref()
                .map(|kinds| kinds.iter().map(|kind| kind.as_str()).collect::<Vec<_>>())
                .unwrap_or_default();
            target_kinds.sort();
            document_facts
                .entry(pending.uri.clone())
                .or_default()
                .push(format!(
                    "pending:{}:{:?}:{:?}:{target_kinds:?}",
                    pending.kind.as_str(),
                    pending.source_qualified,
                    pending.target_qualified,
                ));
        }
        for pending in &graph.pending_expression_relationships {
            document_facts
                .entry(pending.uri.clone())
                .or_default()
                .push(format!(
                    "pending-expression:{}:{:?}:{:?}:{:?}:{:?}:{:?}",
                    pending.kind.as_str(),
                    pending.source_expression,
                    pending.target_expression,
                    pending.container_prefix,
                    pending.is_interface_usage,
                    pending.interface_type,
                ));
        }
        let mut documents = document_facts
            .into_iter()
            .map(|(uri, mut facts)| {
                facts.sort();
                // The URI breaks ties only to assign an otherwise invisible ordinal.
                // It is never emitted; a single document therefore remains stable when
                // moved, while duplicate qualified names remain unambiguous.
                (facts, uri)
            })
            .collect::<Vec<_>>();
        documents.sort_by(|(left_key, left_uri), (right_key, right_uri)| {
            left_key
                .cmp(right_key)
                .then_with(|| left_uri.as_str().cmp(right_uri.as_str()))
        });
        let document_labels = documents
            .into_iter()
            .enumerate()
            .map(|(index, (_, uri))| (uri, format!("d{index}")))
            .collect();
        Self { document_labels }
    }

    fn document(&self, uri: &Url) -> &str {
        self.document_labels
            .get(uri)
            .map(String::as_str)
            .expect("every semantic node and pending relationship has a document label")
    }

    fn node(&self, node: &SemanticNode) -> String {
        format!(
            "(node (document {}) (qualified-name {}))",
            atom(self.document(&node.id.uri)),
            atom(&node.id.qualified_name)
        )
    }

    fn node_id(&self, id: &NodeId) -> String {
        format!(
            "(node (document {}) (qualified-name {}))",
            atom(self.document(&id.uri)),
            atom(&id.qualified_name)
        )
    }
}

impl SemanticGraph {
    /// Renders this graph using the canonical diagnostic S-expression.
    ///
    /// The projection includes containment, resolved and pending typed relationships,
    /// and selected declared/effective facts. It intentionally excludes source ranges,
    /// document paths, caches, and legacy display attributes.
    pub fn to_semantic_sexpr(&self) -> String {
        let identities = CanonicalIdentities::from_graph(self);
        let mut output = format!("({FORMAT_ROOT}\n");
        render_containment(self, &identities, &mut output);
        render_resolved_relationships(self, &identities, &mut output);
        render_pending_relationships(&identities, &self.pending_relationships, &mut output);
        render_pending_expression_relationships(
            &identities,
            &self.pending_expression_relationships,
            &mut output,
        );
        render_derived_relationship_resolutions(self, &identities, &mut output);
        output.push(')');
        output
    }
}

/// Renders the immutable semantic publication using the same diagnostic S-expression convention
/// as [`SemanticGraph::to_semantic_sexpr`].  Unlike the legacy graph renderer, this projection
/// reads only `SemanticModel`/`ResolutionView`: unresolved and ambiguous authored references are
/// retained, and no pending queues or mutable graph edges can become a second source of truth.
/// Writes the canonical replacement-model projection into a caller-owned formatter.
///
/// The public semantic contract remains `SemanticModel`/`ResolutionView`; this test-support
/// renderer is crate-private so snapshots cannot become an alternate query API.
pub(crate) fn write_semantic_model_sexpr(
    model: &SemanticModel,
    output: &mut dyn std::fmt::Write,
) -> std::fmt::Result {
    let identities = ModelCanonicalIdentities::from_model(model);
    output.write_str("(semantic-model\n")?;
    render_model_metadata(model, output)?;
    render_model_structure(model, &identities, output)?;
    render_model_references(model, &identities, output)?;
    render_model_relationships(model, &identities, output)?;
    render_model_evaluation(model, &identities, output)?;
    output.write_char(')')
}

impl SemanticModel {
    /// Streams the canonical semantic-model debug projection to a caller-owned writer.
    ///
    /// This is deliberately a rendering sink rather than an inspection API: callers receive
    /// no graph, index, fact, or resolution storage access. The projection is intended for the
    /// standalone snapshot harness and diagnostics/debug output only.
    pub fn write_debug_sexpr(&self, output: &mut dyn std::fmt::Write) -> std::fmt::Result {
        write_semantic_model_sexpr(self, output)
    }
}

struct ModelCanonicalIdentities {
    document_labels: BTreeMap<Url, String>,
}

impl ModelCanonicalIdentities {
    fn from_model(model: &SemanticModel) -> Self {
        let mut documents: BTreeMap<Url, Vec<String>> = BTreeMap::new();
        for node in model.structural_nodes_for_debug() {
            documents
                .entry(node.id.uri.clone())
                .or_default()
                .push(format!(
                    "{}:{}:{}",
                    node.id.qualified_name,
                    node.element_kind.as_str(),
                    node.name
                ));
        }
        for fact in model.resolution().facts() {
            documents
                .entry(fact.reference.source.uri.clone())
                .or_default()
                .push(format!(
                    "reference:{}:{}",
                    reference_kind(fact.reference.kind),
                    fact.reference.authored_ordinal
                ));
        }
        let mut ordered = documents.into_iter().collect::<Vec<_>>();
        ordered.iter_mut().for_each(|(_, facts)| facts.sort());
        ordered.sort_by(|(left_uri, left_facts), (right_uri, right_facts)| {
            left_facts
                .cmp(right_facts)
                .then_with(|| left_uri.as_str().cmp(right_uri.as_str()))
        });
        Self {
            document_labels: ordered
                .into_iter()
                .enumerate()
                .map(|(index, (uri, _))| (uri, format!("d{index}")))
                .collect(),
        }
    }

    fn document(&self, uri: &Url) -> &str {
        self.document_labels
            .get(uri)
            .map(String::as_str)
            .expect("every model fact belongs to an admitted document")
    }

    fn node(&self, node: &NodeId) -> String {
        format!(
            "(node (document {}) (qualified-name {}))",
            atom(self.document(&node.uri)),
            atom(&node.qualified_name)
        )
    }
}

fn render_model_metadata(
    model: &SemanticModel,
    output: &mut dyn std::fmt::Write,
) -> std::fmt::Result {
    let phase = match model.phase() {
        SemanticModelPhase::Resolved => "resolved",
        SemanticModelPhase::Evaluated => "evaluated",
    };
    let completeness = match model.completeness() {
        SemanticModelCompleteness::Complete => "complete",
        SemanticModelCompleteness::EditorRecovery => "editor-recovery",
    };
    let identity = model.identity();
    writeln!(
        output,
        "  (publication (phase {}) (completeness {}) (has-evaluation {}) (source-digest {}) (contract-version {}))",
        phase,
        completeness,
        model.has_evaluation(),
        atom(&identity.source_digest),
        atom(&identity.semantic_contract_version),
    )
}

fn render_model_structure(
    model: &SemanticModel,
    identities: &ModelCanonicalIdentities,
    output: &mut dyn std::fmt::Write,
) -> std::fmt::Result {
    let mut nodes = model.structural_nodes_for_debug();
    nodes.sort_by_key(|node| {
        (
            identities.document(&node.id.uri).to_string(),
            node.id.qualified_name.clone(),
            node.element_kind.as_str().to_string(),
        )
    });
    output.write_str("  (structure\n")?;
    for node in nodes {
        write!(
            output,
            "    (element (id {}) (kind {}) (name {})",
            identities.node(&node.id),
            atom(node.element_kind.as_str()),
            atom(&node.name),
        )?;
        if let Some(declared_name) = &node.declared_name {
            write!(output, " (declared-name {})", atom(declared_name))?;
        }
        if let Some(parent) = &node.parent_id {
            write!(output, " (parent {})", identities.node(parent))?;
        }
        render_model_declared_facts(&node.declared_facts, identities, output)?;
        output.write_str(")\n")?;
    }
    output.write_str("  )\n")
}

fn render_model_declared_facts(
    facts: &crate::semantic::model::DeclaredSemanticFacts,
    identities: &ModelCanonicalIdentities,
    output: &mut dyn std::fmt::Write,
) -> std::fmt::Result {
    let relationships = [
        ("typing", &facts.relationships.typing),
        ("specializes", &facts.relationships.specializes),
        ("subsetting", &facts.relationships.subsetting),
        ("redefinition", &facts.relationships.redefinition),
        (
            "reference-subsetting",
            &facts.relationships.reference_subsetting,
        ),
        ("cross-subsetting", &facts.relationships.cross_subsetting),
        ("subject", &facts.relationships.subject),
        ("connection", &facts.relationships.connection),
        ("bind", &facts.relationships.bind),
        ("satisfy", &facts.relationships.satisfy),
        ("allocate", &facts.relationships.allocate),
        ("flow", &facts.relationships.flow),
        ("succession-flow", &facts.relationships.succession_flow),
        ("perform", &facts.relationships.perform),
        ("transition", &facts.relationships.transition),
        ("initial-state", &facts.relationships.initial_state),
        ("reference", &facts.relationships.reference),
        ("dependency", &facts.relationships.dependency),
        ("derivation", &facts.relationships.derivation),
    ];
    let has_relationships = relationships.iter().any(|(_, targets)| !targets.is_empty());
    let has_import = facts
        .membership
        .as_ref()
        .and_then(|membership| membership.import.as_ref())
        .is_some();
    if !has_relationships && facts.expression_relationships.is_empty() && !has_import {
        return Ok(());
    }
    output.write_str(" (authored")?;
    if let Some(membership) = &facts.membership {
        write!(output, " (membership (kind {:?})", membership.kind)?;
        if let Some(visibility) = membership.visibility {
            write!(output, " (visibility {})", atom(visibility.as_str()))?;
        }
        if let Some(import) = &membership.import {
            write!(
                output,
                " (import (reference {}) (origin {:?}) (shape {:?}) (recursive {}))",
                atom(&import.target.reference),
                import.origin,
                import.shape,
                import.recursive
            )?;
        }
        output.write_char(')')?;
    }
    if has_relationships {
        output.write_str(" (relationships")?;
        for (kind, targets) in relationships {
            for target in targets {
                write!(
                    output,
                    " ({} (reference {}))",
                    kind,
                    atom(&target.reference),
                )?;
            }
        }
        output.write_char(')')?;
    }
    if !facts.expression_relationships.is_empty() {
        output.write_str(" (expression-relationships")?;
        for relationship in &facts.expression_relationships {
            write!(
                output,
                " ({} (source {}) (target {})",
                relationship.kind.as_str(),
                atom(&relationship.source_expression),
                atom(&relationship.target_expression),
            )?;
            output.write_char(')')?;
        }
        output.write_char(')')?;
    }
    output.write_char(')')?;
    let _ = identities;
    Ok(())
}

fn render_model_references(
    model: &SemanticModel,
    identities: &ModelCanonicalIdentities,
    output: &mut dyn std::fmt::Write,
) -> std::fmt::Result {
    let mut facts = model.resolution().facts().to_vec();
    facts.sort_by_key(|fact| fact.reference.clone());
    output.write_str("  (references\n")?;
    for fact in facts {
        let reference = &fact.reference;
        write!(
            output,
            "    (reference (id (source {}) (kind {}) (ordinal {})) (authored-target {}) ",
            identities.node(&reference.source),
            reference_kind(reference.kind),
            reference.authored_ordinal,
            atom(&fact.authored_target),
        )?;
        render_outcome(&fact.outcome, identities, output)?;
        if let Some(import) = &fact.import {
            render_resolution_import(import, output)?;
        }
        output.write_str(")\n")?;
    }
    output.write_str("  )\n")
}

fn render_resolution_import(
    import: &crate::semantic::publication::ResolutionImportFact,
    output: &mut dyn std::fmt::Write,
) -> std::fmt::Result {
    write!(
        output,
        " (import (origin {}) (shape {}) (recursive {})",
        import_origin(import.origin),
        import_shape(import.shape),
        import.recursive,
    )?;
    match &import.conformance {
        ImportConformanceOutcome::Valid => output.write_str(" (conformance valid)")?,
        ImportConformanceOutcome::MissingTarget => {
            output.write_str(" (conformance missing-target)")?
        }
        ImportConformanceOutcome::NotCheckedUnresolved => {
            output.write_str(" (conformance not-checked-unresolved)")?
        }
        ImportConformanceOutcome::NotCheckedAmbiguous => {
            output.write_str(" (conformance not-checked-ambiguous)")?
        }
        ImportConformanceOutcome::NotCheckedUnsupportedFiltered => {
            output.write_str(" (conformance not-checked-unsupported-filtered)")?
        }
        ImportConformanceOutcome::NamespaceKindMismatch { actual } => write!(
            output,
            " (conformance namespace-kind-mismatch (actual-kind {}))",
            atom(actual.as_str())
        )?,
        ImportConformanceOutcome::RecursiveNonNamespace { actual } => write!(
            output,
            " (conformance recursive-non-namespace (actual-kind {}))",
            atom(actual.as_str())
        )?,
    }
    output.write_char(')')
}

fn import_origin(origin: ImportOrigin) -> &'static str {
    match origin {
        ImportOrigin::Import => "import",
        ImportOrigin::Expose => "expose",
    }
}

fn import_shape(shape: ImportShape) -> &'static str {
    match shape {
        ImportShape::Membership => "membership",
        ImportShape::Namespace => "namespace",
        ImportShape::FilteredNamespace => "filtered-namespace",
    }
}

fn render_outcome(
    outcome: &ResolutionOutcome,
    identities: &ModelCanonicalIdentities,
    output: &mut dyn std::fmt::Write,
) -> std::fmt::Result {
    match outcome {
        ResolutionOutcome::Resolved { target } => {
            write!(
                output,
                "(outcome (status resolved) (target {}))",
                identities.node(target)
            )
        }
        ResolutionOutcome::Unresolved => output.write_str("(outcome (status unresolved))"),
        ResolutionOutcome::UnsupportedFiltered => {
            output.write_str("(outcome (status unsupported-filtered))")
        }
        ResolutionOutcome::Ambiguous { candidates } => {
            output.write_str("(outcome (status ambiguous) (candidates")?;
            for candidate in candidates {
                write!(output, " {}", identities.node(candidate))?;
            }
            output.write_str("))")
        }
    }
}

fn render_model_relationships(
    model: &SemanticModel,
    identities: &ModelCanonicalIdentities,
    output: &mut dyn std::fmt::Write,
) -> std::fmt::Result {
    let mut relationships = model.resolution().relationships().to_vec();
    relationships.sort_by_key(|relationship| {
        (
            relationship.source.clone(),
            relationship.kind.clone(),
            relationship.target.clone(),
            relationship.authored_reference.clone(),
        )
    });
    output.write_str("  (relationships\n")?;
    for relationship in relationships {
        render_model_relationship(&relationship, identities, output)?;
    }
    output.write_str("  )\n")
}

fn render_model_relationship(
    relationship: &ResolvedRelationship,
    identities: &ModelCanonicalIdentities,
    output: &mut dyn std::fmt::Write,
) -> std::fmt::Result {
    write!(
        output,
        "    (relationship (kind {}) (source {}) (target {})",
        relationship.kind.as_str(),
        identities.node(&relationship.source),
        identities.node(&relationship.target),
    )?;
    match relationship.provenance {
        ResolutionProvenance::Authored => output.write_str(" (provenance authored)")?,
        ResolutionProvenance::Implied(rule) => {
            write!(output, " (provenance (implied {:?}))", rule)?;
        }
        ResolutionProvenance::Derived(rule) => {
            write!(output, " (provenance (derived {:?}))", rule)?;
        }
    }
    if let Some(reference) = &relationship.authored_reference {
        write!(
            output,
            " (authored-reference (source {}) (kind {}) (ordinal {}))",
            identities.node(&reference.source),
            reference_kind(reference.kind),
            reference.authored_ordinal,
        )?;
    }
    if let Some(expression) = &relationship.expression {
        write!(
            output,
            " (expression (kind {}) (source {}) (target {})",
            expression.kind.as_str(),
            atom(&expression.source_expression),
            atom(&expression.target_expression),
        )?;
        output.write_char(')')?;
    }
    output.write_str(")\n")
}

fn render_model_evaluation(
    model: &SemanticModel,
    identities: &ModelCanonicalIdentities,
    output: &mut dyn std::fmt::Write,
) -> std::fmt::Result {
    let Some(facts) = model.evaluation_facts() else {
        return Ok(());
    };
    let mut facts = facts.iter().collect::<Vec<_>>();
    facts.sort_by_key(|(node, _)| *node);
    output.write_str("  (evaluation\n")?;
    for (node, facts) in facts {
        write!(output, "    (node {}", identities.node(node))?;
        render_node_evaluation_facts(facts, output)?;
        output.write_str(")\n")?;
    }
    output.write_str("  )\n")
}

fn render_node_evaluation_facts(
    facts: &crate::semantic::model::NodeEvaluationFacts,
    output: &mut dyn std::fmt::Write,
) -> std::fmt::Result {
    if let Some(expression) = &facts.expression {
        write!(
            output,
            " (expression (status {})",
            atom(expression.status.as_str())
        )?;
        if let Some(value) = &expression.value {
            write!(output, " (value {})", render_evaluated_value(value))?;
        }
        if let Some(unit) = &expression.unit {
            write!(output, " (unit {})", atom(unit))?;
        }
        if let Some(error) = &expression.error {
            write!(output, " (error {})", atom(error))?;
        }
        output.write_char(')')?;
    }
    if let Some(analysis) = &facts.analysis {
        write!(
            output,
            " (analysis (status {})",
            atom(analysis.expression.status.as_str())
        )?;
        if let Some(passed) = analysis.passed {
            write!(output, " (passed {passed}")?;
        }
        if let Some(value) = &analysis.computed_value {
            write!(
                output,
                " (computed-value {})",
                render_evaluated_value(value)
            )?;
        }
        if let Some(unit) = &analysis.computed_unit {
            write!(output, " (computed-unit {})", atom(unit))?;
        }
        output.write_char(')')?;
    }
    Ok(())
}

fn reference_kind(kind: ReferenceKind) -> &'static str {
    match kind {
        ReferenceKind::FeatureTyping => "featureTyping",
        ReferenceKind::Specialization => "specialization",
        ReferenceKind::Subsetting => "subsetting",
        ReferenceKind::Redefinition => "redefinition",
        ReferenceKind::ReferenceSubsetting => "referenceSubsetting",
        ReferenceKind::CrossSubsetting => "crossSubsetting",
        ReferenceKind::ConnectionSource => "connectionSource",
        ReferenceKind::ConnectionTarget => "connectionTarget",
        ReferenceKind::BindSource => "bindSource",
        ReferenceKind::BindTarget => "bindTarget",
        ReferenceKind::SatisfySource => "satisfySource",
        ReferenceKind::SatisfyTarget => "satisfyTarget",
        ReferenceKind::AllocateSource => "allocateSource",
        ReferenceKind::AllocateTarget => "allocateTarget",
        ReferenceKind::FlowSource => "flowSource",
        ReferenceKind::FlowTarget => "flowTarget",
        ReferenceKind::SuccessionFlowSource => "successionFlowSource",
        ReferenceKind::SuccessionFlowTarget => "successionFlowTarget",
        ReferenceKind::PerformSource => "performSource",
        ReferenceKind::PerformTarget => "performTarget",
        ReferenceKind::TransitionSource => "transitionSource",
        ReferenceKind::TransitionTarget => "transitionTarget",
        ReferenceKind::ReferenceSource => "referenceSource",
        ReferenceKind::ReferenceTarget => "referenceTarget",
        ReferenceKind::DependencySource => "dependencySource",
        ReferenceKind::DependencyTarget => "dependencyTarget",
        ReferenceKind::DerivationSource => "derivationSource",
        ReferenceKind::DerivationTarget => "derivationTarget",
        ReferenceKind::NamespaceImport => "namespaceImport",
        ReferenceKind::MembershipImport => "membershipImport",
        ReferenceKind::InitialStateSource => "initialStateSource",
        ReferenceKind::InitialStateTarget => "initialStateTarget",
    }
}

fn render_containment(
    graph: &SemanticGraph,
    identities: &CanonicalIdentities,
    output: &mut String,
) {
    write_indent(output, 1);
    output.push_str("(containment\n");
    let mut roots = graph
        .graph
        .node_weights()
        .filter(|node| {
            node.parent_id
                .as_ref()
                .is_none_or(|parent| graph.get_node(parent).is_none())
        })
        .collect::<Vec<_>>();
    roots.sort_by_key(|node| (node_sort_key(node), identities.node(node)));
    for root in roots {
        render_node(graph, identities, root, 2, output);
    }
    write_indent(output, 1);
    output.push_str(")\n");
}

fn render_node(
    graph: &SemanticGraph,
    identities: &CanonicalIdentities,
    node: &SemanticNode,
    depth: usize,
    output: &mut String,
) {
    write_indent(output, depth);
    let _ = write!(
        output,
        "(element (kind {}) (id {}) (name {})",
        atom(node.element_kind.as_str()),
        identities.node(node),
        atom(&node.name),
    );
    if let Some(declared_name) = &node.declared_name {
        let _ = write!(output, " (declared-name {})", atom(declared_name));
    }
    render_declared_facts(node, output);
    render_effective_facts(graph, identities, node, output);
    render_evaluation_facts(graph, node, output);

    let mut children = graph.children_of(node);
    children.sort_by_key(|child| (node_sort_key(child), identities.node(child)));
    if children.is_empty() {
        output.push_str(")\n");
        return;
    }
    output.push('\n');
    write_indent(output, depth + 1);
    output.push_str("(contains\n");
    for child in children {
        render_node(graph, identities, child, depth + 2, output);
    }
    write_indent(output, depth + 1);
    output.push_str(")\n");
    write_indent(output, depth);
    output.push_str(")\n");
}

fn render_declared_facts(node: &SemanticNode, output: &mut String) {
    let facts = &node.declared_facts;
    if facts.multiplicity.is_none()
        && facts.feature_value.is_none()
        && facts.feature_properties.is_none()
        && facts.own_expression.is_none()
    {
        return;
    }
    output.push_str(" (declared");
    if let Some(properties) = &facts.feature_properties {
        render_feature_properties(properties, output);
    }
    if let Some(multiplicity) = &facts.multiplicity {
        let bounds = multiplicity.direct_bounds();
        let _ = write!(
            output,
            " (multiplicity (lower {}) (upper {}) (ordered {})",
            render_bound(bounds.lower),
            render_bound(bounds.upper),
            multiplicity.is_ordered
        );
        if let Some(unique) = multiplicity.is_unique {
            let _ = write!(output, " (unique {unique})");
        }
        output.push_str(if multiplicity.is_implied {
            " (provenance implied))"
        } else {
            " (provenance authored))"
        });
    }
    if let Some(feature_value) = &facts.feature_value {
        let _ = write!(
            output,
            " (feature-value (kind {}) ",
            feature_value_kind(feature_value.kind)
        );
        render_expression(&feature_value.expression, output);
        output.push(')');
    }
    if let Some(expression) = &facts.own_expression {
        output.push_str(" (own-expression ");
        render_expression(expression, output);
        output.push(')');
    }
    output.push(')');
}

fn render_feature_properties(properties: &DeclaredFeatureProperties, output: &mut String) {
    let has_property = properties.direction.is_some()
        || properties.is_abstract
        || properties.is_variation
        || properties.is_individual
        || properties.is_derived
        || properties.is_constant
        || properties.is_end
        || properties.is_composite.is_some()
        || properties.is_reference.is_some()
        || properties.is_conjugated
        || properties.is_ordered.is_some()
        || properties.is_unique.is_some()
        || properties.is_portion
        || properties.portion_kind.is_some();
    if !has_property {
        return;
    }
    output.push_str(" (properties");
    if let Some(direction) = &properties.direction {
        let _ = write!(output, " (direction {})", atom(direction));
    }
    for (name, value) in [
        ("abstract", properties.is_abstract),
        ("variation", properties.is_variation),
        ("individual", properties.is_individual),
        ("derived", properties.is_derived),
        ("constant", properties.is_constant),
        ("end", properties.is_end),
        ("conjugated", properties.is_conjugated),
        ("portion", properties.is_portion),
    ] {
        if value {
            let _ = write!(output, " ({name} true)");
        }
    }
    for (name, value) in [
        ("composite", properties.is_composite),
        ("reference", properties.is_reference),
        ("ordered", properties.is_ordered),
        ("unique", properties.is_unique),
    ] {
        if let Some(value) = value {
            let _ = write!(output, " ({name} {value})");
        }
    }
    if let Some(portion_kind) = &properties.portion_kind {
        let _ = write!(output, " (portion-kind {})", atom(portion_kind));
    }
    output.push(')');
}

fn render_effective_facts(
    graph: &SemanticGraph,
    identities: &CanonicalIdentities,
    node: &SemanticNode,
    output: &mut String,
) {
    let Some(facts) = graph.effective_facts_for(node) else {
        return;
    };
    output.push_str(" (effective");
    if let Some(multiplicity) = facts.implied_multiplicity {
        let _ = write!(
            output,
            " (implied-multiplicity (lower {}) (upper {}) (ordered {})",
            multiplicity.lower,
            multiplicity
                .upper
                .map_or_else(|| "unbounded".to_string(), |upper| upper.to_string()),
            multiplicity.is_ordered
        );
        if let Some(unique) = multiplicity.is_unique {
            let _ = write!(output, " (unique {unique})");
        }
        output.push(')');
    }
    if let Some(ownership) = facts.implied_feature_ownership {
        let _ = write!(
            output,
            " (implied-feature-ownership (composite {}) (reference {}))",
            ownership.is_composite, ownership.is_reference
        );
    }
    if let Some(featuring_type) = &facts.featuring_type {
        let _ = write!(
            output,
            " (featuring-type {})",
            identities.node_id(featuring_type)
        );
    }
    if let Some(binding) = &facts.implied_feature_value_binding {
        let _ = write!(
            output,
            " (implied-feature-value-binding (owner {}) (role {}))",
            identities.node_id(&binding.expression_result.owner_id),
            expression_result_role(binding.expression_result.role)
        );
    }
    output.push(')');
}

fn render_evaluation_facts(graph: &SemanticGraph, node: &SemanticNode, output: &mut String) {
    let Some(facts) = graph.evaluation_facts_for(node) else {
        return;
    };
    output.push_str(" (evaluation");
    if let Some(expression) = &facts.expression {
        let _ = write!(
            output,
            " (expression (status {})",
            atom(expression.status.as_str())
        );
        if let Some(value) = &expression.value {
            let _ = write!(output, " (value {})", render_evaluated_value(value));
        }
        if let Some(unit) = &expression.unit {
            let _ = write!(output, " (unit {})", atom(unit));
        }
        if let Some(error) = &expression.error {
            let _ = write!(output, " (error {})", atom(error));
        }
        output.push(')');
    }
    if let Some(analysis) = &facts.analysis {
        let _ = write!(
            output,
            " (analysis (status {})",
            atom(analysis.expression.status.as_str())
        );
        if let Some(passed) = analysis.passed {
            let _ = write!(output, " (passed {passed})");
        }
        if let Some(value) = &analysis.computed_value {
            let _ = write!(
                output,
                " (computed-value {})",
                render_evaluated_value(value)
            );
        }
        if let Some(unit) = &analysis.computed_unit {
            let _ = write!(output, " (computed-unit {})", atom(unit));
        }
        output.push(')');
    }
    output.push(')');
}

fn render_resolved_relationships(
    graph: &SemanticGraph,
    identities: &CanonicalIdentities,
    output: &mut String,
) {
    let mut relationships = graph
        .graph
        .edge_references()
        .filter_map(|edge| {
            let source = graph.graph.node_weight(edge.source())?;
            let target = graph.graph.node_weight(edge.target())?;
            Some((source, target, edge.weight()))
        })
        .map(|(source, target, edge)| {
            render_resolved_relationship(identities, source, target, edge)
        })
        .collect::<Vec<_>>();
    relationships.sort();
    write_indent(output, 1);
    output.push_str("(relationships\n");
    for relationship in relationships {
        write_indent(output, 2);
        output.push_str(&relationship);
        output.push('\n');
    }
    write_indent(output, 1);
    output.push_str(")\n");
}

fn render_resolved_relationship(
    identities: &CanonicalIdentities,
    source: &SemanticNode,
    target: &SemanticNode,
    edge: &SemanticEdge,
) -> String {
    let mut output = format!(
        "({} (status resolved) (from {}) (to {})",
        edge.kind.as_str(),
        identities.node(source),
        identities.node(target)
    );
    if let Some(connect) = &edge.connect {
        output.push_str(" (connect");
        let _ = write!(
            output,
            " (source-expression {}) (target-expression {})",
            atom(&connect.source_expression),
            atom(&connect.target_expression)
        );
        if let Some(prefix) = &connect.container_prefix {
            let _ = write!(output, " (container-prefix {})", atom(prefix));
        }
        if connect.is_interface_usage {
            output.push_str(" (interface-usage true)");
        }
        if let Some(interface_type) = &connect.interface_type {
            let _ = write!(output, " (interface-type {})", atom(interface_type));
        }
        output.push(')');
    }
    if let Some(flow) = &edge.flow {
        output.push_str(" (flow");
        for (name, value) in [
            ("payload-expression", flow.payload_expression.as_ref()),
            ("source-expression", flow.source_expression.as_ref()),
            ("target-expression", flow.target_expression.as_ref()),
            ("payload-type-id", flow.payload_type_id.as_ref()),
        ] {
            if let Some(value) = value {
                let _ = write!(output, " ({name} {})", atom(value));
            }
        }
        output.push(')');
    }
    match edge.provenance {
        RelationshipProvenance::Authored => output.push_str(" (provenance authored)"),
        RelationshipProvenance::Implied(rule) => {
            let rule = match rule {
                ImpliedRelationshipRule::UniversalStandardLibraryRelationship => {
                    "universal-standard-library-relationship"
                }
                ImpliedRelationshipRule::MetadataRedefinitionEntailsSubsetting => {
                    "metadata-redefinition-entails-subsetting"
                }
            };
            let _ = write!(output, " (provenance (implied (rule {rule})))");
        }
        RelationshipProvenance::Derived(rule) => {
            let rule = match rule {
                DerivedRelationshipRule::CaseSubjectFromTypedSubject => {
                    "case-subject-from-typed-subject"
                }
            };
            let _ = write!(output, " (provenance (derived (rule {rule})))");
        }
    }
    output.push(')');
    output
}

fn render_derived_relationship_resolutions(
    graph: &SemanticGraph,
    identities: &CanonicalIdentities,
    output: &mut String,
) {
    let mut resolutions = graph
        .derived_relationship_resolution_by_source_id
        .iter()
        .map(|(source, resolution)| (identities.node_id(source), resolution))
        .collect::<Vec<_>>();
    resolutions.sort_by(|(left, _), (right, _)| left.cmp(right));
    if resolutions.is_empty() {
        return;
    }
    write_indent(output, 1);
    output.push_str("(derived-relationship-resolutions\n");
    for (source, resolution) in resolutions {
        write_indent(output, 2);
        let _ = write!(
            output,
            "(universal-standard-library-relationship (from {source}) "
        );
        match resolution {
            DerivedRelationshipResolution::NotRun => output.push_str("(status not-run)"),
            DerivedRelationshipResolution::NotApplicable => {
                output.push_str("(status not-applicable)")
            }
            DerivedRelationshipResolution::Resolved { target } => {
                let _ = write!(
                    output,
                    "(status resolved) (to {})",
                    identities.node_id(target)
                );
            }
            DerivedRelationshipResolution::MissingPrerequisite { target } => {
                let _ = write!(
                    output,
                    "(status missing-prerequisite) (target {})",
                    atom(target.qualified_name())
                );
            }
            DerivedRelationshipResolution::Ambiguous { candidates } => {
                output.push_str("(status ambiguous) (candidates");
                for candidate in candidates {
                    let _ = write!(output, " {}", identities.node_id(candidate));
                }
                output.push(')');
            }
            DerivedRelationshipResolution::SelfTargetSuppressed { target } => {
                let _ = write!(
                    output,
                    "(status self-target-suppressed) (target {})",
                    identities.node_id(target)
                );
            }
        }
        output.push_str(")\n");
    }
    write_indent(output, 1);
    output.push_str(")\n");
}

fn render_pending_relationships(
    identities: &CanonicalIdentities,
    pending: &[PendingRelationship],
    output: &mut String,
) {
    let mut pending = pending
        .iter()
        .map(|relationship| {
            let target_kinds = relationship
                .target_kinds
                .as_ref()
                .map(|kinds| {
                    let mut kinds = kinds
                        .iter()
                        .map(|kind| atom(kind.as_str()))
                        .collect::<Vec<_>>();
                    kinds.sort();
                    format!(" (target-kinds {})", kinds.join(" "))
                })
                .unwrap_or_default();
            format!(
                "({} (status pending) (document {}) (source-qualified {}) (target-qualified {}){})",
                relationship.kind.as_str(),
                atom(identities.document(&relationship.uri)),
                atom(&relationship.source_qualified),
                atom(&relationship.target_qualified),
                target_kinds
            )
        })
        .collect::<Vec<_>>();
    pending.sort();
    render_block("pending-relationships", &pending, output);
}

fn render_evaluated_value(value: &crate::semantic::model::EvaluatedValue) -> String {
    use crate::semantic::model::EvaluatedValue;

    match value {
        EvaluatedValue::Integer(value) => format!("(integer {value})"),
        EvaluatedValue::Real(value) => format!("(real {value})"),
        EvaluatedValue::Boolean(value) => format!("(boolean {value})"),
        EvaluatedValue::String(value) => format!("(string {})", atom(value)),
    }
}

fn render_pending_expression_relationships(
    identities: &CanonicalIdentities,
    pending: &[PendingExpressionRelationship],
    output: &mut String,
) {
    let mut pending = pending
        .iter()
        .map(|relationship| {
            let mut output = format!(
                "({} (status pending-expression) (document {}) (source-expression {}) (target-expression {})",
                relationship.kind.as_str(),
                atom(identities.document(&relationship.uri)),
                atom(&relationship.source_expression),
                atom(&relationship.target_expression),
            );
            if let Some(prefix) = &relationship.container_prefix {
                let _ = write!(output, " (container-prefix {})", atom(prefix));
            }
            if relationship.is_interface_usage {
                output.push_str(" (interface-usage true)");
            }
            if let Some(interface_type) = &relationship.interface_type {
                let _ = write!(output, " (interface-type {})", atom(interface_type));
            }
            output.push(')');
            output
        })
        .collect::<Vec<_>>();
    pending.sort();
    render_block("pending-expression-relationships", &pending, output);
}

fn render_block(name: &str, entries: &[String], output: &mut String) {
    write_indent(output, 1);
    let _ = writeln!(output, "({name}");
    for entry in entries {
        write_indent(output, 2);
        output.push_str(entry);
        output.push('\n');
    }
    write_indent(output, 1);
    output.push_str(")\n");
}

fn render_expression(expression: &DeclaredExpression, output: &mut String) {
    let _ = write!(
        output,
        "(expression (kind {})",
        atom(expression.kind.as_str())
    );
    if let Some(literal) = &expression.literal {
        let _ = write!(output, " (literal {})", render_declared_literal(literal));
    }
    if let Some(reference) = &expression.reference {
        let _ = write!(output, " (reference {})", atom(reference));
    }
    if let Some(operator) = &expression.operator {
        let _ = write!(output, " (operator {})", atom(operator.as_str()));
    }
    if !expression.children.is_empty() {
        output.push_str(" (children");
        for child in &expression.children {
            output.push(' ');
            render_expression(child, output);
        }
        output.push(')');
    }
    if !expression.arguments.is_empty() {
        output.push_str(" (arguments");
        for argument in &expression.arguments {
            output.push_str(" (argument");
            if let Some(name) = &argument.name {
                let _ = write!(output, " (name {})", atom(name));
            }
            output.push(' ');
            render_expression(&argument.value, output);
            output.push(')');
        }
        output.push(')');
    }
    output.push(')');
}

fn render_declared_literal(literal: &crate::semantic::model::DeclaredLiteral) -> String {
    use crate::semantic::model::DeclaredLiteral;

    match literal {
        DeclaredLiteral::Integer(value) => format!("(integer {value})"),
        DeclaredLiteral::Real(value) => format!("(real {})", atom(value)),
        DeclaredLiteral::String(value) => format!("(string {})", atom(value)),
        DeclaredLiteral::Boolean(value) => format!("(boolean {value})"),
    }
}

fn render_bound(bound: DeclaredMultiplicityBound) -> String {
    match bound {
        DeclaredMultiplicityBound::Unbounded => "unbounded".to_string(),
        DeclaredMultiplicityBound::Integer(value) => value.to_string(),
        DeclaredMultiplicityBound::NonIntegerLiteral => "non-integer-literal".to_string(),
        DeclaredMultiplicityBound::Unevaluated => "unevaluated".to_string(),
    }
}

fn feature_value_kind(kind: DeclaredFeatureValueKind) -> &'static str {
    match kind {
        DeclaredFeatureValueKind::Default => "default",
        DeclaredFeatureValueKind::Initial => "initial",
        DeclaredFeatureValueKind::Bound => "bound",
        DeclaredFeatureValueKind::Override => "override",
    }
}

fn expression_result_role(role: ExpressionResultRole) -> &'static str {
    match role {
        ExpressionResultRole::FeatureValue => "feature-value",
    }
}

fn node_sort_key(node: &SemanticNode) -> (String, String, String, String) {
    let mut declared = String::new();
    render_declared_facts(node, &mut declared);
    (
        node.id.qualified_name.clone(),
        node.element_kind.as_str().to_string(),
        node.name.clone(),
        declared,
    )
}

fn atom(value: &str) -> String {
    serde_json::to_string(value).expect("strings serialize")
}

fn write_indent(output: &mut String, depth: usize) {
    output.push_str(&"  ".repeat(depth));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic::model::{DeclaredLiteral, ElementKind, EvaluatedValue};
    use crate::semantic::publication::ResolutionImportFact;

    #[test]
    fn renders_declared_and_evaluated_scalars_with_structural_type_tags() {
        assert_eq!(
            render_declared_literal(&DeclaredLiteral::Integer(1)),
            "(integer 1)"
        );
        assert_eq!(
            render_declared_literal(&DeclaredLiteral::Real("1.0".into())),
            r#"(real "1.0")"#
        );
        assert_eq!(
            render_declared_literal(&DeclaredLiteral::String("1.0".into())),
            r#"(string "1.0")"#
        );
        assert_eq!(
            render_evaluated_value(&EvaluatedValue::Boolean(true)),
            "(boolean true)"
        );
        assert_eq!(
            render_evaluated_value(&EvaluatedValue::String("ok".into())),
            r#"(string "ok")"#
        );
    }

    #[test]
    fn renders_import_resolution_facts_with_stable_typed_atoms() {
        let cases = [
            (
                ResolutionImportFact {
                    origin: ImportOrigin::Import,
                    shape: ImportShape::Namespace,
                    recursive: false,
                    conformance: ImportConformanceOutcome::Valid,
                },
                " (import (origin import) (shape namespace) (recursive false) (conformance valid))",
            ),
            (
                ResolutionImportFact {
                    origin: ImportOrigin::Expose,
                    shape: ImportShape::FilteredNamespace,
                    recursive: true,
                    conformance: ImportConformanceOutcome::NotCheckedUnsupportedFiltered,
                },
                " (import (origin expose) (shape filtered-namespace) (recursive true) (conformance not-checked-unsupported-filtered))",
            ),
            (
                ResolutionImportFact {
                    origin: ImportOrigin::Import,
                    shape: ImportShape::Namespace,
                    recursive: false,
                    conformance: ImportConformanceOutcome::NamespaceKindMismatch {
                        actual: ElementKind::PartDef,
                    },
                },
                " (import (origin import) (shape namespace) (recursive false) (conformance namespace-kind-mismatch (actual-kind \"part def\")))",
            ),
        ];
        for (fact, expected) in cases {
            let mut rendered = String::new();
            render_resolution_import(&fact, &mut rendered).expect("render import fact");
            assert_eq!(rendered, expected);
        }
    }
}

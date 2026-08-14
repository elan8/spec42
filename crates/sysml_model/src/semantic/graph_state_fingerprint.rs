//! Test-only canonical graph-state equivalence oracle (the `GraphStateFingerprint` contract).
//!
//! [`crate::semantic::graph_sexpr`]'s `to_semantic_sexpr()` is the human-readable semantic
//! parity oracle and is deliberately left untouched by this module: it intentionally excludes
//! source ranges, document paths, and publication/construction-ownership state (see its own doc
//! comment). [`GraphStateFingerprint`] is the complementary
//! machine-comparable oracle that *does* cover those fields -- every authoritative record field
//! listed in `planning/ROUNDTRIP_SEMGRAPH_PREREQS.md` §2.2: node identity/kind/names/URI/range/parent/
//! typed facts, edges with endpoint identity/kind/provenance/construction owner/connect-flow
//! detail, source roles, pending relationships, effective/derived/evaluation facts and
//! evaluation publication, and the graph's own [`SemanticPublication`].
//!
//! This is deliberately `#[cfg(test)]`-only. It is an oracle for differential tests, not a
//! second persistence path: `planning/ROUNDTRIP_SEMGRAPH_PREREQS.md` B5 forbids a competing runtime
//! serialization authority alongside the eventual `SemanticGraphRecordV1`, and this type is
//! never constructed outside `cargo test`.
//!
//! Deliberately excluded, matching B9's scope boundary: [`crate::semantic::model::SemanticNode`]'s
//! untyped `attributes: HashMap<String, serde_json::Value>` bag. Every field this module reads
//! instead goes through the typed `declared_facts`/`source_text`/`expression_text` facts, which
//! contain no hash-keyed maps, so their `Debug` rendering is deterministic (field-declaration
//! order) and safe to use as a canonical content digest for fields that do not otherwise derive
//! `PartialEq`. When a real serde round-trip becomes possible (after B9 removes `attributes`),
//! it can be dropped in as an additional input to the same fingerprint comparisons used here --
//! nothing about this oracle's shape assumes the input came from a live build.

use std::collections::BTreeSet;

use url::Url;

use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{
    ConstructionOwner, ElementKind, EvaluationPublicationState, NodeId, RelationshipProvenance,
};
use crate::semantic::publication::{SemanticCompleteness, SemanticContractVersion, SemanticPhase};
use crate::semantic::text_span::TextRange;

/// Canonical per-node fingerprint. Every field is either a value with real `PartialEq`
/// (`NodeId`'s `Ord` supplies the canonical §6 ordering) or a `Debug`-rendered digest of a typed
/// fact struct that does not itself derive `PartialEq` (never of `SemanticNode::attributes`,
/// which this module never reads).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeFingerprint {
    pub id: NodeId,
    pub element_kind: ElementKind,
    pub declared_name: Option<String>,
    pub name: String,
    pub range: TextRange,
    pub parent_id: Option<NodeId>,
    pub source_role_debug: String,
    pub declared_facts_debug: String,
    pub source_text_debug: String,
    pub expression_text_debug: String,
}

/// Canonical per-edge fingerprint, keyed by endpoint `NodeId`s (never petgraph indices) so
/// ordering and equality are independent of construction/insertion order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeFingerprint {
    pub source: NodeId,
    pub target: NodeId,
    pub kind_debug: String,
    pub provenance: RelationshipProvenanceKey,
    pub owner: ConstructionOwner,
    pub connect_debug: String,
    pub flow_debug: String,
}

/// `RelationshipProvenance` does not derive `Ord`, so this wraps its `Debug` rendering (a
/// finite, closed enum -- no floats, no maps, so rendering is a stable total order) for
/// canonical sorting of [`EdgeFingerprint`]s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationshipProvenanceKey(pub String);

impl From<&RelationshipProvenance> for RelationshipProvenanceKey {
    fn from(value: &RelationshipProvenance) -> Self {
        RelationshipProvenanceKey(format!("{value:?}"))
    }
}

/// The complete, canonically ordered fingerprint of one graph's authoritative state.
#[derive(Debug, Clone, PartialEq)]
pub struct GraphStateFingerprint {
    /// Sorted by [`NodeId`]'s canonical `Ord` (`planning/ROUNDTRIP_SEMGRAPH_PREREQS.md` §6).
    pub nodes: Vec<NodeFingerprint>,
    /// Sorted by `(source, target, kind_debug, ...)`, i.e. `EdgeFingerprint`'s derived `Ord`,
    /// which starts with endpoint `NodeId`s.
    pub edges: Vec<EdgeFingerprint>,
    /// The complete Workspace/StandardLibrary/Library/External classification
    /// (`SemanticGraph::source_origins_sorted`, itself already canonically ordered).
    pub source_origins: Vec<(Url, source_identity::SourceRole)>,
    pub standard_library_uris: Vec<String>,
    /// Order-independent: pending relationships have no serde-stable canonical order today, so
    /// this oracle treats the pending set as a set, not a sequence — content equality is what
    /// this fingerprint verifies, not incidental accumulation order.
    pub pending_relationships: BTreeSet<String>,
    pub pending_expression_relationships: BTreeSet<String>,
    /// Keyed by `NodeId`, sorted canonically; value is a `Debug` digest of the typed fact
    /// (`EffectiveSemanticFacts` derives `PartialEq` but not `Ord`, so `Debug` is used uniformly
    /// here for a single canonicalization strategy across every non-`Ord` fact type).
    pub effective_facts: Vec<(NodeId, String)>,
    pub derived_relationship_resolutions: Vec<(NodeId, String)>,
    pub evaluation_facts: Vec<(NodeId, String)>,
    pub evaluation_publication: EvaluationPublicationState,
    /// `RootDigest` has no `Debug` impl; its raw bytes are the canonical identity anyway.
    pub publication_root_digest_bytes: [u8; 32],
    pub publication_phase: SemanticPhase,
    pub publication_completeness: SemanticCompleteness,
    pub publication_semantic_contract: SemanticContractVersion,
}

impl GraphStateFingerprint {
    /// Computes the canonical fingerprint of `graph`'s current authoritative state.
    ///
    /// Never reads `node_index_by_id`, `children_by_parent_id`, `document_dependency_targets`,
    /// `document_dependents`, `cross_document_edges_by_source_uri`, `nodes_by_uri`,
    /// `node_ids_by_qualified_name`, or any query/shape/import cache -- those are disposable
    /// derived indexes explicitly excluded by §2.2, whose prerequisites (nodes, edges, and
    /// `ConstructionOwner`) are exactly what this fingerprint does cover.
    pub fn capture(graph: &SemanticGraph) -> Self {
        let mut nodes: Vec<NodeFingerprint> = graph
            .graph
            .node_indices()
            .filter_map(|idx| graph.graph.node_weight(idx))
            .map(|node| NodeFingerprint {
                id: node.id.clone(),
                element_kind: node.element_kind.clone(),
                declared_name: node.declared_name.clone(),
                name: node.name.clone(),
                range: node.range,
                parent_id: node.parent_id.clone(),
                source_role_debug: format!("{:?}", graph.source_role_for_uri(&node.id.uri)),
                declared_facts_debug: format!("{:?}", node.declared_facts),
                source_text_debug: format!("{:?}", node.source_text),
                expression_text_debug: format!("{:?}", node.expression_text),
            })
            .collect();
        nodes.sort_by(|a, b| {
            a.id.cmp(&b.id).then_with(|| {
                (
                    &a.declared_facts_debug,
                    &a.source_text_debug,
                    &a.expression_text_debug,
                )
                    .cmp(&(
                        &b.declared_facts_debug,
                        &b.source_text_debug,
                        &b.expression_text_debug,
                    ))
            })
        });

        let mut edges: Vec<EdgeFingerprint> = graph
            .graph
            .edge_indices()
            .filter_map(|idx| {
                let (src_idx, tgt_idx) = graph.graph.edge_endpoints(idx)?;
                let weight = graph.graph.edge_weight(idx)?;
                let source = graph.graph.node_weight(src_idx)?.id.clone();
                let target = graph.graph.node_weight(tgt_idx)?.id.clone();
                Some(EdgeFingerprint {
                    source,
                    target,
                    kind_debug: format!("{:?}", weight.kind),
                    provenance: RelationshipProvenanceKey::from(&weight.provenance),
                    owner: weight.owner,
                    connect_debug: format!("{:?}", weight.connect),
                    flow_debug: format!("{:?}", weight.flow),
                })
            })
            .collect();
        edges.sort_by(|a, b| {
            (
                &a.source,
                &a.target,
                &a.kind_debug,
                &a.provenance.0,
                a.owner as u8,
                &a.connect_debug,
                &a.flow_debug,
            )
                .cmp(&(
                    &b.source,
                    &b.target,
                    &b.kind_debug,
                    &b.provenance.0,
                    b.owner as u8,
                    &b.connect_debug,
                    &b.flow_debug,
                ))
        });

        let mut standard_library_uris: Vec<String> = graph
            .standard_library_uris
            .iter()
            .map(|uri| uri.to_string())
            .collect();
        standard_library_uris.sort();

        let pending_relationships: BTreeSet<String> = graph
            .pending_relationships
            .iter()
            .map(|pending| format!("{pending:?}"))
            .collect();
        let pending_expression_relationships: BTreeSet<String> = graph
            .pending_expression_relationships
            .iter()
            .map(|pending| format!("{pending:?}"))
            .collect();

        let mut effective_facts: Vec<(NodeId, String)> = graph
            .effective_facts_by_node_id
            .iter()
            .map(|(id, facts)| (id.clone(), format!("{facts:?}")))
            .collect();
        effective_facts.sort_by(|(a, _), (b, _)| a.cmp(b));

        let mut derived_relationship_resolutions: Vec<(NodeId, String)> = graph
            .derived_relationship_resolution_by_source_id
            .iter()
            .map(|(id, resolution)| (id.clone(), format!("{resolution:?}")))
            .collect();
        derived_relationship_resolutions.sort_by(|(a, _), (b, _)| a.cmp(b));

        let mut evaluation_facts: Vec<(NodeId, String)> = graph
            .evaluation_facts_by_node_id
            .iter()
            .map(|(id, facts)| (id.clone(), format!("{facts:?}")))
            .collect();
        evaluation_facts.sort_by(|(a, _), (b, _)| a.cmp(b));

        let publication = graph.publication;

        GraphStateFingerprint {
            nodes,
            edges,
            source_origins: graph.source_origins_sorted(),
            standard_library_uris,
            pending_relationships,
            pending_expression_relationships,
            effective_facts,
            derived_relationship_resolutions,
            evaluation_facts,
            evaluation_publication: graph.evaluation_publication,
            publication_root_digest_bytes: *publication.root_digest().as_bytes(),
            publication_phase: publication.phase(),
            publication_completeness: publication.completeness(),
            publication_semantic_contract: publication.semantic_contract(),
        }
    }

    /// Resets the publication-identity fields to a fixed placeholder, leaving every other field
    /// (nodes, edges, facts, evaluation publication, roles) untouched. Used only where a test
    /// deliberately compares graph *construction-path* state (whole vs. parallel vs. raw
    /// document-at-a-time patching) without also expecting the surrounding caller to have
    /// stamped a real `SemanticPublication` -- `patch_graph_for_document`/
    /// `patch_graph_for_document_scoped` never set `root_digest`/`completeness` themselves,
    /// that is the owning workspace layer's job (see `pipeline::build_and_link_graph`'s own
    /// `graph.publication = SemanticPublication::new(...)` call, which raw per-document patch
    /// calls deliberately do not replicate). Publication identity/phase/completeness is
    /// tested directly in `publication.rs`; this neutralization exists solely so this suite's
    /// determinism assertions are not a false negative about publication-stamping, which is out
    /// of scope for these particular comparisons.
    #[cfg(test)]
    pub(crate) fn with_neutralized_publication_identity(mut self) -> Self {
        self.publication_root_digest_bytes = [0u8; 32];
        self.publication_completeness = SemanticCompleteness::Complete;
        self
    }
}

// --- Differential test suites ---
//
// Neither `GraphStateFingerprint` nor `to_semantic_sexpr()` alone proves observable behavior
// (the `GraphStateFingerprint` contract own required resolution). These suites are the actual
// deliverable: (a) determinism, (b) sensitivity, (c) post-edit differential parity, (d) public
// query/S-expression differential parity.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic::model::{ConstructionOwner, RelationshipKind};
    use crate::semantic::pipeline::{
        build_and_link_graph, build_and_link_graph_parallel, patch_graph_for_document_scoped,
    };
    use crate::semantic::publication::SemanticPhase;
    use crate::semantic::source::{SysmlDocument, SysmlDocumentSourceKind};

    fn memory_doc(name: &str, content: &str, kind: SysmlDocumentSourceKind) -> SysmlDocument {
        SysmlDocument::from_memory_path("b11", name, content.to_string(), kind, None, None)
            .expect("uri")
    }

    fn workspace_doc(name: &str, content: &str) -> SysmlDocument {
        memory_doc(name, content, SysmlDocumentSourceKind::Workspace)
    }

    fn fp(graph: &SemanticGraph) -> GraphStateFingerprint {
        GraphStateFingerprint::capture(graph)
    }

    fn typing_fixture() -> (SysmlDocument, SysmlDocument) {
        (
            workspace_doc("A.sysml", "package A { part def Thing; }"),
            workspace_doc(
                "B.sysml",
                "package B { private import A::*; part x : Thing; }",
            ),
        )
    }

    // --- (a) Determinism suite ---

    #[test]
    fn identical_sources_built_twice_produce_identical_fingerprints() {
        let (a, b) = typing_fixture();
        let (g1, _) = build_and_link_graph(&[a.clone(), b.clone()]).expect("build 1");
        let (g2, _) = build_and_link_graph(&[a, b]).expect("build 2");
        assert_eq!(fp(&g1), fp(&g2));
    }

    #[test]
    fn whole_and_parallel_builds_agree() {
        let (a, b) = typing_fixture();
        let (whole, _) = build_and_link_graph(&[a.clone(), b.clone()]).expect("whole");
        let (parallel, _) = build_and_link_graph_parallel(&[a, b]);
        assert_eq!(fp(&whole), fp(&parallel));
    }

    #[test]
    fn whole_and_incremental_document_at_a_time_builds_agree() {
        let (a, b) = typing_fixture();
        let (whole, _) = build_and_link_graph(&[a.clone(), b.clone()]).expect("whole");

        // `patch_graph_for_document_scoped` calls `remove_nodes_for_uri` internally before
        // merging fresh content, which (correctly, matching `standard_library_uris`'s existing
        // documented behavior) clears any previously recorded `source_origins` entry for that
        // URI. `source_origins` is a caller-stamped fact, not something the raw per-document
        // patch functions maintain on their own -- so the caller must (re-)stamp it *after* the
        // patch, exactly like `build_and_link_graph_parallel`'s callers already do for a fresh
        // build.
        let mut incremental = SemanticGraph::new();
        let parsed_a = sysml_v2_parser::parse(&a.content).expect("parse a");
        patch_graph_for_document_scoped(&mut incremental, &a.uri, Some(&parsed_a), true);
        incremental.set_source_origin(a.uri.clone(), source_identity::SourceRole::Workspace);
        let parsed_b = sysml_v2_parser::parse(&b.content).expect("parse b");
        patch_graph_for_document_scoped(&mut incremental, &b.uri, Some(&parsed_b), true);
        incremental.set_source_origin(b.uri.clone(), source_identity::SourceRole::Workspace);

        assert_eq!(
            fp(&whole).with_neutralized_publication_identity(),
            fp(&incremental).with_neutralized_publication_identity(),
            "graph state must agree across construction paths regardless of publication \
             identity stamping, which is the caller's job, not the pipeline patch functions'"
        );
    }

    #[test]
    fn forward_and_reverse_document_order_agree() {
        let (a, b) = typing_fixture();
        let (forward, _) = build_and_link_graph(&[a.clone(), b.clone()]).expect("forward");
        let (reverse, _) = build_and_link_graph(&[b, a]).expect("reverse");
        assert_eq!(fp(&forward), fp(&reverse));
    }

    #[test]
    fn decoded_graph_matches_live_graph() {
        let (a, b) = typing_fixture();
        let (built, _) = build_and_link_graph(&[a, b]).expect("build");
        let mut data = built.clone().into_data();
        data.simulate_decode_reset_for_test();
        let decoded = SemanticGraph::from_data(data);
        assert_eq!(fp(&built), fp(&decoded));
    }

    // --- (b) Sensitivity suite ---
    //
    // A fingerprint that never differs proves nothing: each case below
    // perturbs exactly one authoritative field and asserts the fingerprint changes.

    #[test]
    fn sensitivity_changed_source_byte() {
        let a = workspace_doc("A.sysml", "package A { part def Thing; }");
        let a_edited = workspace_doc("A.sysml", "package A { part def Thingy; }");
        let (g1, _) = build_and_link_graph(&[a]).expect("build 1");
        let (g2, _) = build_and_link_graph(&[a_edited]).expect("build 2");
        assert_ne!(
            fp(&g1),
            fp(&g2),
            "renaming a declared element must change the fingerprint"
        );
    }

    #[test]
    fn sensitivity_changed_uri() {
        let content = "package A { part def Thing; }";
        let a1 = workspace_doc("A.sysml", content);
        let a2 = workspace_doc("A2.sysml", content);
        let (g1, _) = build_and_link_graph(&[a1]).expect("build 1");
        let (g2, _) = build_and_link_graph(&[a2]).expect("build 2");
        assert_ne!(
            fp(&g1),
            fp(&g2),
            "a different document URI must change every NodeId"
        );
    }

    #[test]
    fn sensitivity_changed_source_role() {
        let content = "package A { part def Thing; }";
        let workspace = memory_doc("A.sysml", content, SysmlDocumentSourceKind::Workspace);
        let library = memory_doc("A.sysml", content, SysmlDocumentSourceKind::Library);
        let (g1, _) = build_and_link_graph(&[workspace]).expect("build 1");
        let (g2, _) = build_and_link_graph(&[library]).expect("build 2");
        assert_ne!(
            fp(&g1),
            fp(&g2),
            "reclassifying a source's role must change the fingerprint even with identical bytes"
        );
    }

    #[test]
    fn sensitivity_changed_range() {
        let a1 = workspace_doc("A.sysml", "package A { part def Thing; }");
        // Leading whitespace shifts every subsequent byte offset without changing any
        // declared/effective semantic fact.
        let a2 = workspace_doc("A.sysml", "package A {   part def Thing; }");
        let (g1, _) = build_and_link_graph(&[a1]).expect("build 1");
        let (g2, _) = build_and_link_graph(&[a2]).expect("build 2");
        assert_ne!(
            fp(&g1),
            fp(&g2),
            "a shifted source range must change the fingerprint even when semantics are identical"
        );
    }

    #[test]
    fn sensitivity_changed_relationship_kind() {
        let (a, _) = typing_fixture();
        let b_typing = workspace_doc(
            "B.sysml",
            "package B { private import A::*; part x : Thing; }",
        );
        let b_specializes = workspace_doc(
            "B.sysml",
            "package B { private import A::*; part def x :> Thing; }",
        );
        let (g1, _) = build_and_link_graph(&[a.clone(), b_typing]).expect("build 1");
        let (g2, _) = build_and_link_graph(&[a, b_specializes]).expect("build 2");
        assert_ne!(
            fp(&g1),
            fp(&g2),
            "Typing vs Specializes must be distinguishable in the fingerprint"
        );
    }

    #[test]
    fn sensitivity_changed_construction_owner() {
        let (a, b) = typing_fixture();
        let (mut graph, _) = build_and_link_graph(&[a, b]).expect("build");
        let baseline = fp(&graph);

        // Retag one edge's owner in place -- a mechanical fact distinct from provenance --
        // without touching any other field.
        let edge_idx = graph
            .graph
            .edge_indices()
            .find(|idx| graph.graph[*idx].owner == ConstructionOwner::WorkspaceCrossDocumentLinking)
            .expect("a cross-document edge must exist in this fixture");
        graph.graph[edge_idx].owner = ConstructionOwner::PendingResolution;

        assert_ne!(
            baseline,
            fp(&graph),
            "a changed construction owner must change the fingerprint"
        );
    }

    #[test]
    fn sensitivity_changed_provenance() {
        let (a, b) = typing_fixture();
        let (mut graph, _) = build_and_link_graph(&[a, b]).expect("build");
        let baseline = fp(&graph);

        let edge_idx = graph
            .graph
            .edge_indices()
            .find(|idx| graph.graph[*idx].kind == RelationshipKind::Typing)
            .expect("a typing edge must exist in this fixture");
        graph.graph[edge_idx].provenance = crate::semantic::model::RelationshipProvenance::Implied(
            crate::semantic::model::ImpliedRelationshipRule::UniversalStandardLibraryRelationship,
        );

        assert_ne!(
            baseline,
            fp(&graph),
            "authored vs implied provenance must change the fingerprint"
        );
    }

    #[test]
    fn sensitivity_changed_publication_phase_and_completeness() {
        let (a, b) = typing_fixture();
        let (mut graph, _) = build_and_link_graph(&[a, b]).expect("build");
        let baseline = fp(&graph);

        let mut degraded = graph.clone();
        degraded
            .publication
            .set_completeness(SemanticCompleteness::EditorRecovery);
        assert_ne!(
            baseline,
            fp(&degraded),
            "a changed completeness must change the fingerprint"
        );

        graph.publication = crate::semantic::publication::SemanticPublication::new(
            graph.publication.root_digest(),
            graph.publication.completeness(),
        );
        assert_ne!(
            baseline.publication_phase,
            fp(&graph).publication_phase,
            "resetting phase back to Parsed must change the fingerprint's phase"
        );
        assert_eq!(fp(&graph).publication_phase, SemanticPhase::Parsed);
    }

    #[test]
    fn sensitivity_changed_evaluation_publication() {
        let a = workspace_doc(
            "A.sysml",
            "package A { part def Rocket { attribute mass = 1 + 2; } }",
        );
        let (mut graph, _) = build_and_link_graph(&[a]).expect("build");
        let baseline = fp(&graph);
        graph.invalidate_evaluation_facts();
        assert_ne!(
            baseline,
            fp(&graph),
            "clearing evaluation facts/publication must change the fingerprint"
        );
    }
}

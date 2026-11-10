//! The published whole-model projection contract.
//!
//! Every other query group answers one question about one element. This one composes them into a
//! single deterministic answer for a headless consumer -- CI, a script, an agent, the read-only
//! Python SDK -- that needs the resolved *structure* of a workspace, not just its diagnostics.
//! `spec42 model-export --format json` serialises it.
//!
//! It adds no analysis and holds nothing new. Each element carries the [`ElementDetails`] the
//! `inspection().element_details()` query already publishes, plus its resolved expression tree
//! and metadata annotations (the `#84` families); the `connect` / `interface` topology (the
//! third `#84` family) is on the projection as a whole. It is the composition, canonically
//! ordered, with a [`schema_version`] a consumer can branch on.
//!
//! [schema_version]: PublishedModelProjection::schema_version
//!
//! What it deliberately is **not**: a write surface (no element mutation -- that is the Babel42
//! model-repository track), and not the OMG Systems Modeling API JSON (bidirectional,
//! element-CRUD, standards-compliance -- a separate concern). This projection is read-only and
//! Spec42-shaped.
//!
//! Like the other boundary answers ([`PublishedExpression`], [`PublishedConnector`]) it is an
//! in-process value: elements are named by [`SymbolId`], and a consumer that has to cross a
//! process or protocol boundary materialises a [`crate::SymbolToken`] for each through
//! `PublishedModel`. The serde JSON form is owned by the host that emits it, not by this crate.

use source_identity::{PublicationModelDigest, RootDigest};

use crate::connection_query::PublishedConnector;
use crate::details::ElementDetails;
use crate::expression::PublishedExpression;
use crate::metadata_query::PublishedMetadataAnnotation;
use crate::{ElementSource, PublicationCompleteness, SymbolId};

/// The schema version of [`PublishedModelProjection`] and everything it carries.
///
/// Bumped when a field is removed or retyped, including the closed expression-node vocabulary;
/// adding optional fields alone does not bump it. Schema 2 adds invocation, sequence and meta-cast
/// nodes. A consumer that pins a major version can refuse an unknown one.
pub const MODEL_PROJECTION_SCHEMA_VERSION: u32 = 2;

/// A read-only, deterministic projection of one publication's workspace-authored structure.
///
/// [`Self::elements`] is in the publication's canonical order (document order, then source
/// range); [`Self::connectors`] likewise. Only workspace-authored elements are enumerated --
/// admitted libraries are reported as counts on [`ProjectionEnvelope::admitted`].
#[derive(Debug, Clone, PartialEq)]
pub struct PublishedModelProjection {
    /// Always [`MODEL_PROJECTION_SCHEMA_VERSION`]; carried so a serialised copy is self-describing.
    pub schema_version: u32,
    pub envelope: ProjectionEnvelope,
    /// Every workspace-authored element, canonically ordered, each with its full details and
    /// composed resolved facts. Truncated to [`ProjectionTruncation::elements_returned`].
    pub elements: Box<[ProjectedElement]>,
    /// Every workspace `connect` / `interface` connector, canonically ordered. The whole-workspace
    /// form of `inspection().connections(root)`.
    pub connectors: Box<[PublishedConnector]>,
    pub truncation: ProjectionTruncation,
}

/// Whether the projection lists every workspace element or a bounded prefix of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectionTruncation {
    /// Workspace-authored elements in the publication.
    pub elements_total: usize,
    /// Elements actually carried in [`PublishedModelProjection::elements`].
    pub elements_returned: usize,
    /// Elements within the `max_nodes` bound whose details did not resolve (a non-converged
    /// publication) and were therefore omitted from [`PublishedModelProjection::elements`]
    /// without being subject to `max_nodes`. Distinct from truncation: raising `max_nodes`
    /// cannot recover these -- see [`ProjectionEnvelope::completeness`] for why.
    pub elements_incomplete: usize,
}

impl ProjectionTruncation {
    /// Whether elements beyond `max_nodes` exist that were never attempted. `false` when every
    /// workspace element was attempted, even if some of those were
    /// [`Self::elements_incomplete`] -- raising `max_nodes` would not change the result.
    pub fn is_truncated(&self) -> bool {
        self.elements_returned + self.elements_incomplete < self.elements_total
    }
}

/// The publication a projection was taken from, enough to tell a complete build from a recovered
/// one without a second query.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectionEnvelope {
    pub phase: ProjectionPhase,
    /// [`PublicationCompleteness::is_complete`], or the obstacles that stopped it.
    pub completeness: PublicationCompleteness,
    /// Whether constant evaluation ran for this publication.
    pub has_evaluation: bool,
    /// Dependency-complete digest of every admitted source.
    pub source_digest: RootDigest,
    /// Dependency-complete digest of every input that can change a published answer.
    pub model_digest: PublicationModelDigest,
    pub admitted: AdmittedSourceCounts,
}

/// The lifecycle phase a publication reached.
///
/// Mirrors the authority's internal phase enum so this contract publishes no implementation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionPhase {
    /// Lowering and resolution completed; the only phase a consumer ever observes.
    Resolved,
}

impl ProjectionPhase {
    /// A stable kebab-case name, for snapshot output.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Resolved => "resolved",
        }
    }
}

/// How many documents took part in resolution, by provenance.
///
/// The snapshot renderer still reports only the non-workspace counts. JSON export includes
/// `workspace` as well, so a publication that admitted no workspace documents is visible without
/// inferring it from an empty element list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AdmittedSourceCounts {
    pub workspace: usize,
    pub standard_library: usize,
    pub library: usize,
    pub external: usize,
}

/// Which non-workspace elements a model projection includes.
///
/// The default is the workspace publication. `Referenced` adds elements named by workspace
/// typing, specialization, subsetting, or redefinition, recursively including relationship
/// targets and owned members of library and external elements. `All` adds every admitted
/// library and external element. Standard-library elements stay out of `All`; a standard-library
/// element appears in `Referenced` only when a selected element's relationship names it, and its
/// owned members are not pulled in with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LibraryElementInclusion {
    #[default]
    None,
    Referenced,
    All,
}

/// One workspace element, with everything the publication settled about it.
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectedElement {
    /// Stable within this publication; materialise a [`crate::SymbolToken`] through
    /// `PublishedModel` to cross a boundary.
    pub identity: SymbolId,
    /// [`ElementSource::Workspace`] unless the caller asked for library elements.
    pub source: ElementSource,
    /// The same answer `inspection().element_details()` publishes: kind, name, location,
    /// membership, modifiers, multiplicity, direction, documentation, the four relationship
    /// families, effective typing, inherited features, metadata bindings, and incoming / outgoing
    /// relationships.
    pub details: ElementDetails,
    /// Settled effective names, separate from the declaration's authored identification.
    /// An absent value means no complete effective name is available.
    pub effective_name: Option<Box<str>>,
    pub effective_qualified_name: Option<Box<str>>,
    /// Authored bound expressions; a single-bound range has only an upper expression.
    pub multiplicity_expressions: Option<ProjectedMultiplicity>,
    pub exposure: Option<ProjectedExposure>,
    /// The element's resolved constraint / calc / value expression, or
    /// [`crate::ExpressionOutcome::NotApplicable`] when it authored none.
    pub expression: PublishedExpression,
    /// Inherited expressions interpreted with this element's effective bindings.
    pub contextual_evaluations: Box<[ProjectedContextualEvaluation]>,
    /// Every metadata annotation bound to the element, in canonical order.
    pub metadata_annotations: Box<[PublishedMetadataAnnotation]>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectedContextualEvaluation {
    pub element: SymbolId,
    pub authored_element: SymbolId,
    pub effective_element: SymbolId,
    pub outcome: ContextualEvaluationOutcome,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualEvaluationOutcome {
    Resolved(crate::EvaluationState),
    Unresolved,
    Ambiguous(Box<[SymbolId]>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectedMultiplicity {
    pub range: SymbolId,
    pub lower: Option<SymbolId>,
    pub upper: Option<SymbolId>,
}

/// Authored exposure shape, read from the canonical ViewExpose relationship flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectedExposure {
    pub namespace: bool,
    pub recursive: bool,
    pub filtered: bool,
}

//! The published element-details contract.
//!
//! One cohesive answer per element, for a consumer that shows everything this publication knows
//! about one declaration at once. It exists because assembling that view from the individual
//! services -- inspection, types, evaluation, relationships -- forced the consumer to decide how
//! they relate to each other, and every consumer decided differently.
//!
//! Nothing here is derivable by a consumer from another field. In particular an empty target list
//! never means "nothing was authored": [`RelationshipOutcome`] says which of the two it is, and a
//! family that resolved partially cannot present itself as fully resolved.

use crate::evaluation::{AnalysisEvaluation, ElementEvaluation};
use crate::inspection::{ElementInspection, RelationshipProvenance, SymbolEntry};
use crate::type_query::EffectiveTypeOrigin;
use crate::SourceLocation;

pub use sysml_contract::RelationshipOutcome;

/// One authored relationship family of an element.
///
/// `targets` and `candidates` are separate channels because they answer different questions. A
/// target is what the publication settled on; a candidate is one of several the publication
/// refused to choose between. Merging them would let a consumer render an ambiguous reference as
/// if it had resolved to each of its candidates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationshipFamily {
    pub outcome: RelationshipOutcome,
    /// The distinct settled targets, in canonical order.
    pub targets: Box<[SymbolEntry]>,
    /// Every candidate of every ambiguous reference in the family, in canonical order.
    pub candidates: Box<[SymbolEntry]>,
}

impl RelationshipFamily {
    /// The family of an element that authors nothing of this kind.
    pub fn not_applicable() -> Self {
        Self {
            outcome: RelationshipOutcome::NotApplicable,
            targets: Box::default(),
            candidates: Box::default(),
        }
    }
}

/// One type a feature effectively has, with the element that carries it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveTypeEntry {
    pub element: SymbolEntry,
    pub origin: EffectiveTypeOrigin,
}

/// The types a feature has, directly or along its subsetting/redefinition chain.
///
/// The outcome is a fact about the *declaration*, not about the list: a feature that declares a
/// typing which did not resolve is [`RelationshipOutcome::Unresolved`] with no types, and a
/// feature that declares nothing to inherit a type along is
/// [`RelationshipOutcome::NotApplicable`]. Those are different answers, and an empty list alone
/// cannot tell them apart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveTyping {
    pub outcome: RelationshipOutcome,
    /// The effective types, in canonical order.
    pub types: Box<[EffectiveTypeEntry]>,
    /// Candidate effective types retained when an authored typing or inherited typing path is
    /// ambiguous. Candidates are never promoted to settled types.
    pub candidates: Box<[EffectiveTypeEntry]>,
}

/// One feature an element has through a type or supertype rather than by declaring it.
///
/// `declared_in` is the type that actually declares the feature, which is the provenance an
/// inspector shows and a consumer cannot recover from the feature alone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InheritedFeature {
    pub feature: SymbolEntry,
    pub declared_in: SymbolEntry,
}

/// The candidate-dependent result of applying all effective conditions of a view to one element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewSelectionOutcome {
    Included,
    Excluded,
    Indeterminate(Box<[ViewSelectionObstacle]>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ViewSelectionObstacle {
    UnresolvedPredicate,
    AmbiguousPredicate(Box<[crate::SymbolIdentity]>),
    UnsupportedPredicate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewSelection {
    pub view: crate::SymbolIdentity,
    pub candidate: crate::SymbolIdentity,
    pub outcome: ViewSelectionOutcome,
}

/// One settled relationship between an element and a peer, in one direction.
///
/// `kind` is the relationship's canonical name (`typing`, `specialization`, `subsetting`, ...),
/// which is the relationship channel rather than the authored reference channel: an inspector
/// showing "this is typed by Engine" is naming the relationship, not the production that wrote it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectedElement {
    pub kind: &'static str,
    pub provenance: RelationshipProvenance,
    pub peer: SymbolEntry,
    /// Where the reference that produced this relationship was written; `None` when the resolver
    /// implied it and no text exists.
    pub location: Option<SourceLocation>,
}

/// Everything one publication settled about one element, as one coherent answer.
///
/// Assembled from the same settled facts the individual services read, so a consumer of this and a
/// consumer of [`crate::PublishedResolution::inspect`] cannot disagree.
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDetails {
    pub inspection: ElementInspection,
    /// The element's owning declaration, when it has one.
    pub owner: Option<SymbolEntry>,
    /// The typings the element declares itself.
    pub typing: RelationshipFamily,
    /// The types it has once inherited typing is taken into account.
    pub effective_typing: EffectiveTyping,
    pub specialization: RelationshipFamily,
    /// `subsets`, `references` and `crosses` together: all three are subsetting in KerML.
    pub subsetting: RelationshipFamily,
    pub redefinition: RelationshipFamily,
    /// Features the element has without declaring them, nearest owner first.
    pub inherited_features: Box<[InheritedFeature]>,
    /// Metadata annotations bound to this element, in canonical order.
    pub metadata: Box<[SymbolEntry]>,
    /// Relationships whose target is this element, in canonical order.
    pub incoming: Box<[ConnectedElement]>,
    /// Relationships this element is the source of, in canonical order.
    pub outgoing: Box<[ConnectedElement]>,
    pub evaluation: ElementEvaluation,
    pub analysis: AnalysisEvaluation,
}

/// What a reference at a source position resolves to, in full detail.
///
/// The same shape as [`crate::inspection::ReferenceAt`] and for the same reason: "no reference
/// here" and "a reference here that did not resolve" are different answers.
#[derive(Debug, Clone, PartialEq)]
pub enum ReferencedDetails {
    /// No authored reference covers the position.
    None,
    Resolved(Box<ElementDetails>),
    /// The reference resolved to several candidates, retained in canonical order.
    Ambiguous(Box<[ElementDetails]>),
    /// A reference is here, and resolution found no target for it.
    Unresolved,
    /// A reference is here, and its form is outside the supported resolution slice.
    Unsupported,
    /// A reference is here, and the publication did not converge, so it has no settled answer.
    Incomplete,
}

/// The two elements a position identifies, in full detail.
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDetailsAt {
    /// The innermost element whose declaration contains the position.
    pub containing: Option<ElementDetails>,
    /// What a reference at the position resolves to, with its own outcome.
    pub referenced: ReferencedDetails,
}

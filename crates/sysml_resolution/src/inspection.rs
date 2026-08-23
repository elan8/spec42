//! The published element-inspection contract.
//!
//! One typed answer per element, assembled from facts this crate already owns. Nothing here is
//! recovered by re-reading authored text, and nothing is a generic attribute map: every field has
//! a typed producer behind it, and a fact the parser cannot express is absent or explicitly
//! not-representable rather than defaulted.
//!
//! The shape follows the OMG Pilot's separation of concerns rather than its presentation: the
//! Pilot's hover returns a pre-formatted markdown string carrying a kind label, a name and one
//! type name, which throws away everything its own model knows. Rendering is a consumer's job.

use crate::evaluation::EvaluationState;
use crate::{ElementKind, MembershipRole, SourceLocation, SymbolId, TextId, TextRange};

pub use sysml_contract::{
    AnnotationForm, AuthoredValue, ElementDerivedDocumentationCollection, ElementModifier,
    FeatureDirection, MembershipFacts, MembershipKind, MultiplicityBound, MultiplicityFacts,
    PortionKind, RelationshipProvenance, ValueKind, Visibility, VisibilityProvenance,
};

/// One documentation, comment or textual-representation annotation.
///
/// `text` is the raw content between the comment delimiters. The parser performs no leading-`*`
/// stripping and no dedent, so this crate does not either -- normalising here would make two
/// consumers disagree about what the author wrote.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Documentation {
    pub form: AnnotationForm,
    pub locale: Option<Box<str>>,
    /// The `rep` language; always `None` for the other two forms.
    pub language: Option<Box<str>>,
    /// The body, as a handle into the publication's interned text.
    ///
    /// Not a copy: a documentation body is often the longest string on an element, and an
    /// inspection is produced per element in bulk answers. Read it with
    /// [`PublishedResolution::text`](crate::PublishedResolution::text).
    pub text: TextId,
}

/// The exact derived `Element::owner` value.
///
/// `NoOwner` is a settled `null`, not an unresolved source identity. Query-level uncertainty is
/// represented by [`crate::QueryOutcome`], so callers cannot mistake a root element for a failed
/// ownership derivation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DerivedElementOwner {
    NoOwner,
    Owner(SymbolId),
}

/// What resolution concluded about one authored reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationshipTarget {
    Resolved(SymbolId),
    /// Resolution found more than one candidate, all retained in canonical order.
    Ambiguous(Box<[SymbolId]>),
    Unresolved,
    /// The reference's form is outside the supported resolution slice.
    Unsupported,
}

/// One relationship of an element, in the direction it was authored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementRelationship {
    /// The reference kind, as its canonical name (`featureTyping`, `subsetting`, ...).
    pub kind: &'static str,
    pub provenance: RelationshipProvenance,
    /// The path text as written, for a consumer rendering the source form.
    ///
    /// `None` for an implied relationship, which by definition was never written.
    pub authored: Option<Box<str>>,
    pub target: RelationshipTarget,
    /// Where the reference was written; `None` for an implied relationship.
    pub location: Option<SourceLocation>,
}

/// Everything this crate knows about one element.
#[derive(Debug, Clone, PartialEq)]
pub struct ElementInspection {
    /// The element this inspection is about.
    ///
    /// The `::`-joined display path is not carried: it is a copy of text the publication already
    /// stores, and an inspection is produced per element in queries that return many. Read it
    /// with `PublishedResolution::qualified_name`, which borrows from the settled blob.
    pub identity: SymbolId,
    pub kind: ElementKind,
    /// The role this element plays in its owner, where the OMG carries it on the membership.
    pub role: Option<MembershipRole>,
    /// The authored name, absent for an anonymous element.
    pub name: Option<Box<str>>,
    pub short_name: Option<Box<str>>,
    /// The element's name range, or its declaration range when it has no name.
    pub location: SourceLocation,
    /// The whole declaration's range.
    pub declaration_range: TextRange,
    pub owner: Option<SymbolId>,
    pub membership: MembershipFacts,
    pub documentation: Box<[Documentation]>,
    pub multiplicity: MultiplicityFacts,
    /// The authored modifiers, in a stable order.
    pub modifiers: Box<[ElementModifier]>,
    pub portion_kind: Option<PortionKind>,
    pub direction: Option<FeatureDirection>,
    pub value: Option<AuthoredValue>,
    pub evaluation: EvaluationState,
    /// Outgoing relationships, in canonical reference order.
    pub relationships: Box<[ElementRelationship]>,
}

/// What a reference at a source position resolves to.
///
/// Not an `Option`: collapsing "no reference here" together with "a reference here did not
/// resolve" would lose exactly the distinction an inspector exists to show, and would break this
/// crate's rule that unresolved, ambiguous and unsupported outcomes stay explicit.
#[derive(Debug, Clone, PartialEq)]
pub enum ReferenceAt {
    /// No authored reference covers the position.
    None,
    Resolved(Box<ElementInspection>),
    /// The reference resolved to several candidates, retained in canonical order.
    Ambiguous(Box<[ElementInspection]>),
    /// A reference is here, and resolution found no target for it.
    Unresolved,
    /// A reference is here, and its form is outside the supported resolution slice.
    Unsupported,
    /// A reference is here, and the publication did not converge, so it has no settled answer.
    Incomplete,
}

/// The two elements a position identifies.
///
/// A feature inspector needs both: the element whose declaration encloses the cursor, and what a
/// reference under the cursor points at. They are usually different, and collapsing them would
/// make the inspector show the wrong one half the time.
#[derive(Debug, Clone, PartialEq)]
pub struct ElementInspectionAt {
    /// The innermost element whose declaration contains the position.
    pub containing: Option<ElementInspection>,
    /// What a reference at the position resolves to, with its own outcome.
    pub referenced: ReferenceAt,
}

/// One entry of a document's symbol outline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolEntry {
    pub identity: SymbolId,
    pub kind: ElementKind,
    pub name: Option<Box<str>>,
    pub owner: Option<SymbolId>,
    pub location: SourceLocation,
    pub declaration_range: TextRange,
}

impl SymbolEntry {
    /// What to call this element in text a person reads.
    ///
    /// The authored name when there is one, and the qualified name otherwise. A presentation
    /// default, decided here so every renderer shows the same string for the same element rather
    /// than each choosing its own fallback: it never substitutes for `identity`, and an absent
    /// `name` stays absent in the published fact.
    pub fn display_label<'a>(&'a self, qualified_name: &'a str) -> &'a str {
        self.name.as_deref().unwrap_or(qualified_name)
    }
}

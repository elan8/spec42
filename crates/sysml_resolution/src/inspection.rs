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

use crate::element_kind::{ElementKind, MembershipRole};
use crate::evaluation::EvaluationState;
use crate::{SourceLocation, SymbolIdentity, TextRange};

/// How an element is owned by its namespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MembershipKind {
    /// An owned member.
    Owning,
    /// A feature of the owning type.
    Feature,
    /// An import.
    Import,
    /// An alias.
    Alias,
}

/// Membership visibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

/// Where a membership's effective visibility came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VisibilityProvenance {
    /// A `public`/`private`/`protected` keyword was written.
    Authored,
    /// No keyword was written, so the KerML default for this membership's context applies.
    Default,
}

/// The membership facts of one element.
///
/// Visibility lives here rather than on the element because the OMG models it on the membership
/// (KerML §7.3.3.1), and the sibling compiler makes the same choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MembershipFacts {
    pub kind: MembershipKind,
    pub visibility: Visibility,
    pub provenance: VisibilityProvenance,
}

/// Which annotation production a documentation entry came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnnotationForm {
    /// `doc /* ... */`.
    Documentation,
    /// `comment /* ... */`.
    Comment,
    /// `rep <language> "..." /* ... */`.
    TextualRepresentation,
}

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
    pub text: Box<str>,
}

/// One authored multiplicity bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplicityBound {
    /// No bound authored on this side: unbounded.
    Unbounded,
    /// A bound that folds to a literal integer.
    Literal(i64),
    /// A bound authored as a non-literal expression, published as an explicit non-literal fact
    /// rather than guessed at.
    Expression,
}

/// The authored multiplicity and collection modifiers of one element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplicityFacts {
    /// No `[...]` was authored.
    ///
    /// Distinct from `[*]`, which authors an unbounded multiplicity.
    Absent,
    Declared {
        lower: MultiplicityBound,
        upper: MultiplicityBound,
        ordered: bool,
        nonunique: bool,
    },
}

/// A modifier prefix that was authored on an element.
///
/// A closed set: every variant has exactly one parser field behind it. Modifiers the pinned parser
/// cannot express -- SysML `readonly`, SysML `variable`, `unique`, the bare `portion` prefix --
/// are absent from this set by construction rather than reported as `false`; see
/// `UPSTREAM_PARSER_GAPS.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ElementModifier {
    Abstract,
    Variation,
    Individual,
    Derived,
    End,
    Reference,
    Constant,
    Event,
    Standard,
    All,
    Composite,
    Portion,
    Var,
    Member,
    Ordered,
    Nonunique,
}

impl ElementModifier {
    /// The authored keyword.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Abstract => "abstract",
            Self::Variation => "variation",
            Self::Individual => "individual",
            Self::Derived => "derived",
            Self::End => "end",
            Self::Reference => "ref",
            Self::Constant => "constant",
            Self::Event => "event",
            Self::Standard => "standard",
            Self::All => "all",
            Self::Composite => "composite",
            Self::Portion => "portion",
            Self::Var => "var",
            Self::Member => "member",
            Self::Ordered => "ordered",
            Self::Nonunique => "nonunique",
        }
    }
}

/// The `snapshot`/`timeslice` portion prefix of an occurrence usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PortionKind {
    Snapshot,
    Timeslice,
}

/// A directed feature's direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FeatureDirection {
    In,
    Out,
    InOut,
}

/// Which operator introduced an authored value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValueKind {
    /// `=`.
    Bind,
    /// `:=`.
    Assign,
}

/// The authored spelling of an element's value clause.
///
/// Keeps all five spellings apart: `= e`, `:= e`, `default = e`, `default := e`, and the
/// operator-less `default e`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthoredValue {
    pub kind: ValueKind,
    pub is_default: bool,
    /// `false` only for the operator-less bare `default e` spelling, so a renderer does not
    /// fabricate an `=` the author never wrote.
    pub has_operator: bool,
}

/// Whether a relationship was authored or derived by the resolver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RelationshipProvenance {
    /// Written in the source.
    Authored,
    /// Synthesized by the resolver from a rule, such as an implied redefinition.
    Implied,
}

/// What resolution concluded about one authored reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationshipTarget {
    Resolved(SymbolIdentity),
    /// Resolution found more than one candidate, all retained in canonical order.
    Ambiguous(Box<[SymbolIdentity]>),
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
    pub identity: SymbolIdentity,
    pub kind: ElementKind,
    /// The role this element plays in its owner, where the OMG carries it on the membership.
    pub role: Option<MembershipRole>,
    /// The authored name, absent for an anonymous element.
    pub name: Option<Box<str>>,
    pub short_name: Option<Box<str>>,
    /// The `::`-joined owner path. Anonymous ancestors contribute no segment, so this is a display
    /// convenience and not an identity -- use [`ElementInspection::identity`] for that.
    pub qualified_name: Box<str>,
    /// The element's name range, or its declaration range when it has no name.
    pub location: SourceLocation,
    /// The whole declaration's range.
    pub declaration_range: TextRange,
    pub owner: Option<SymbolIdentity>,
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

/// The two elements a position identifies.
///
/// A feature inspector needs both: the element whose declaration encloses the cursor, and the
/// element a reference under the cursor resolves to. They are usually different, and collapsing
/// them would make the inspector show the wrong one half the time.
#[derive(Debug, Clone, PartialEq)]
pub struct ElementInspectionAt {
    /// The innermost element whose declaration contains the position.
    pub containing: Option<ElementInspection>,
    /// The element a reference at the position resolves to.
    pub referenced: Option<ElementInspection>,
}

/// One entry of a document's symbol outline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolEntry {
    pub identity: SymbolIdentity,
    pub kind: ElementKind,
    pub name: Option<Box<str>>,
    pub qualified_name: Box<str>,
    pub owner: Option<SymbolIdentity>,
    pub location: SourceLocation,
    pub declaration_range: TextRange,
}

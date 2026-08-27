//! What an element is, how it is owned, and what its declaration says about it.
//!
//! Membership, visibility, multiplicity, direction, modifiers and the value spelling are all
//! facts of one declaration, published as closed enums so a consumer matches them exhaustively
//! instead of parsing keywords back out of text. [`ElementKind`] and [`MembershipRole`], the two
//! channels the OMG keeps apart, live beside them in [`crate::element_kind`].

use crate::ElementKind;

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

/// One exact derived `Element` documentation collection selected by a pinned manifest rule.
///
/// The values themselves remain `Documentation` facts from the canonical publication. The
/// selector distinguishes the OMG's `Documentation` and `TextualRepresentation` metaclasses
/// without reducing either to rendered source text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ElementDerivedDocumentationCollection {
    Documentation,
    TextualRepresentation,
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
    Parallel,
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
            Self::Parallel => "parallel",
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

/// Which authored source domain an element search may observe.
///
/// This is deliberately provenance-based. Consumers must not infer library ownership from a
/// document URI or qualified name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementSource {
    Workspace,
    StandardLibrary,
    Library,
    External,
}

/// A typed, bounded search over declarations in one immutable publication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementSearch {
    pub kind: ElementKind,
    pub source: ElementSource,
}

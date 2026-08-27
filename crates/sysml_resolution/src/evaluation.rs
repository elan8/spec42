//! The published evaluation contract.
//!
//! Two channels, deliberately separate, for the same reason the element kind and its membership
//! role are separate: [`EvaluatedScalar`] is *what the value is*, [`EvaluationState`] is *what
//! happened*. Folding internally needs one lattice type whose failure variants absorb -- a
//! division by zero anywhere in a tree must propagate out of it -- but a consumer asking "does
//! this element have a value" should not have to pattern-match a failure out of the value type.
//!
//! Every state is explicit. The sibling compiler collapses non-constant, unsupported,
//! depth-exceeded, cyclic and failed-lookup into a single `.unevaluable`
//! (`src/analyse/Evaluator.zig`), and expresses "not run" as absence from a memo table. Absence is
//! not a state a consumer can reason about: it cannot tell "there was nothing to evaluate" from
//! "evaluation was not run" from "evaluation ran and found nothing".

use std::fmt;

use crate::{SourceLocation, SymbolId, TextId};

pub use sysml_contract::EvaluationFailure;

/// A computed constant value.
///
/// `PartialEq` but not `Eq`, because [`EvaluatedScalar::Real`] carries an `f64`.
#[derive(Debug, Clone, PartialEq)]
pub enum EvaluatedScalar {
    Boolean(bool),
    Integer(i64),
    Real(f64),
    String(Box<str>),
    /// A magnitude with an authored unit token, e.g. `10 [kg]`.
    ///
    /// The unit is the raw authored text (`kg`, `SI::s`, `m/s^2`), never a resolved declaration:
    /// units may contain operators, so the parser hands them over as opaque text rather than as a
    /// qualified reference, and this layer does not invent a resolution the parser cannot support.
    Quantity {
        magnitude: Box<EvaluatedScalar>,
        unit: Box<str>,
    },
}

/// What evaluation produced for one element.
#[derive(Debug, Clone, PartialEq)]
pub enum EvaluationState {
    /// The element carries no expression, so there is nothing to evaluate.
    NotApplicable,
    /// The build's [`EvaluationPolicy`] excluded evaluation.
    ///
    /// Distinct from every other state: it says nothing about the expression, only that no attempt
    /// was made.
    NotRun,
    /// The authored expression is itself a literal value.
    Literal(EvaluatedScalar),
    /// The expression folded to a constant.
    ///
    /// Includes an expression over literals alone (`2 + 3`), which needs folding even though it
    /// needs no resolution -- the distinction from [`EvaluationState::Literal`] is whether a value
    /// was written or computed.
    Evaluated(EvaluatedScalar),
    /// The expression is a supported shape, but a resolved operand has no constant value of its
    /// own, so the expression is correctly not a constant.
    NonConstant,
    /// The value depends on itself, directly or through other declarations.
    ///
    /// A property of the model rather than a failure of the evaluator, and never a fabricated
    /// value, an infinite loop, or a panic.
    Cyclic,
    /// The expression's syntactic shape is outside the supported evaluation slice.
    ///
    /// Explicit rather than absent, so a consumer can tell "this crate does not evaluate this
    /// shape yet" from "this expression has no constant value".
    Unsupported,
    /// Evaluation was attempted and could not produce a value.
    Failed(EvaluationFailure),
}

impl EvaluationState {
    /// The computed value, when there is one.
    pub fn value(&self) -> Option<&EvaluatedScalar> {
        match self {
            Self::Literal(value) | Self::Evaluated(value) => Some(value),
            _ => None,
        }
    }

    /// A stable kebab-case name for this state, for debug rendering and snapshot output.
    ///
    /// Names the state only; the value, where there is one, is rendered separately.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotApplicable => "not-applicable",
            Self::NotRun => "not-run",
            Self::Literal(_) => "literal",
            Self::Evaluated(_) => "evaluated",
            Self::NonConstant => "non-constant",
            Self::Cyclic => "cyclic",
            Self::Unsupported => "unsupported",
            Self::Failed(EvaluationFailure::DivisionByZero) => "division-by-zero",
            Self::Failed(EvaluationFailure::TypeMismatch) => "type-mismatch",
            Self::Failed(EvaluationFailure::UnresolvedOperand) => "unresolved-operand",
        }
    }
}

impl fmt::Display for EvaluationState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// One unit declaration the publication's unit catalog admits.
///
/// `dimensions` is the measurement-reference type or types the unit is an instance of -- `ISQ::
/// MassUnit` for `SI::kilogram`. It is the dimension as a *typed identity*, never a name: two
/// dimensions are the same one when they are the same declaration, and compatibility between them
/// is the ordinary specialization question, answered by the same closure every other type query
/// uses.
///
/// It is a list because a unit may be an instance of several: the SI kelvin is declared
/// `attribute <K> kelvin : ThermodynamicTemperatureUnit, TemperatureDifferenceUnit`, and reporting
/// one of the two would make a correct use of the other look wrong. Non-empty by construction: a
/// declaration with no measurement-reference type is not a unit and never enters the catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedUnit {
    /// The unit declaration itself, such as `SI::kilogram`.
    pub unit: SymbolId,
    /// The measurement-reference types it is an instance of, in canonical order.
    pub dimensions: Box<[SymbolId]>,
}

/// What the publication settled for one authored unit token.
///
/// Every state is its own variant because they mean different things to a consumer and to a rule.
/// "This publication admits no unit catalog" is not "this symbol is unknown", and "the parser hands
/// this layer an opaque token it cannot resolve" is neither.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnitResolution {
    /// The token names exactly one unit in the admitted catalog.
    Resolved(ResolvedUnit),
    /// A catalog is admitted and no unit in it carries this symbol.
    UnknownSymbol,
    /// Several admitted units carry this symbol, so choosing one would be a guess.
    Ambiguous(Box<[SymbolId]>),
    /// The token is not a single unit symbol.
    ///
    /// The parser hands a unit over as opaque text rather than as a reference, because a unit may
    /// contain operators (`m/s^2`, `kg*m`). This layer resolves a token that is a plain name and
    /// says so explicitly for one that is not, rather than reimplementing a unit-expression
    /// grammar over text the parser declined to model.
    UnsupportedExpression,
    /// No unit catalog was admitted to this publication, so no token can be looked up in one.
    ///
    /// Distinct from [`UnitResolution::UnknownSymbol`]: a workspace built without the measurement
    /// libraries has not written a wrong unit, it has not supplied the catalog that would settle
    /// the question.
    CatalogUnavailable,
}

/// One unit token exactly as it was authored, and what the publication settled for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoredUnit {
    /// The token as written between the brackets, never normalized to the unit's declared name.
    ///
    /// A handle, not a copy: the token is text this publication interned once, and an element
    /// with several unit literals would otherwise allocate one `Box<str>` per token for text a
    /// consumer renders only at a hover or a diagnostic. Read it with
    /// [`PublishedResolution::text`](crate::PublishedResolution::text).
    pub authored: TextId,
    /// The token's own range: the text inside the brackets, so a diagnostic about the unit points
    /// at the unit rather than at the literal that carries it.
    pub location: SourceLocation,
    pub resolution: UnitResolution,
}

/// Everything one publication settled about one element's authored expression.
///
/// The three channels stay separate on purpose. [`ElementEvaluation::state`] says what happened and
/// carries the value when there is one; [`ElementEvaluation::units`] carries the authored unit
/// tokens with their own provenance and outcomes; [`ElementEvaluation::expected_measurement`] says
/// what the element's declared type asks a unit to be. Folding any of them into another would make
/// one of the questions unanswerable.
#[derive(Debug, Clone, PartialEq)]
pub struct ElementEvaluation {
    pub element: SymbolId,
    pub state: EvaluationState,
    /// The unit tokens authored in this element's expression, in authored order.
    ///
    /// Empty for an expression with no unit token. A consumer reading a value's unit reads the
    /// value's own [`EvaluatedScalar::Quantity`] spelling; this is the resolution of every token
    /// the author wrote, which is a different and larger set.
    pub units: Box<[AuthoredUnit]>,
    pub expected_measurement: ExpectedMeasurement,
}

/// The measurement reference an element's declared type requires of its value.
///
/// A quantity-valued feature is typed by a quantity value definition -- `ISQ::MassValue` -- whose
/// measurement-reference feature is typed by the dimension its values must be measured in. That
/// requirement is a fact about the *feature*, independent of whether a value was authored, which
/// is why it is published beside the evaluation rather than inside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpectedMeasurement {
    /// The element is not typed by a quantity value, so no measurement reference is required.
    ///
    /// An affirmative answer: this publication admits the library that says what a quantity value
    /// is, and this element is not one. [`ExpectedMeasurement::Unavailable`] is the case where
    /// that prerequisite is missing, and the two must not be confused -- suppressing a unit rule
    /// because a fact was ruled out is not the same as suppressing it because it was never
    /// decidable.
    NotApplicable,
    /// No admitted library declares what a quantity value is, so whether this element is one is
    /// unknown.
    Unavailable,
    /// The element is typed by a quantity value whose measurement reference is not settled: the
    /// type resolved but its measurement-reference feature has no type this publication can read.
    Indeterminate,
    /// The element's values must be measured in one of these measurement-reference types.
    Required(Box<[SymbolId]>),
}

/// What one publication settled about a verdict-bearing element's expression.
///
/// Deliberately a second channel beside [`ElementEvaluation`] rather than a field of it. An
/// ordinary feature's expression *is* its value; an analysis case's, a verification case's, a
/// requirement's or a constraint's expression *states an outcome about the model*. Folding the two
/// would make "this attribute is `false`" and "this constraint failed" the same published fact.
///
/// The value channel is the same settled [`EvaluationState`] the element publishes, so the verdict
/// and the value can never disagree; what this adds is which of the two questions the element's
/// kind makes it an answer to.
#[derive(Debug, Clone, PartialEq)]
pub enum AnalysisEvaluation {
    /// The element's kind does not make its expression a verdict or a computed result.
    ///
    /// An affirmative answer, distinct from [`AnalysisEvaluation::NotRun`] and from an element
    /// that is verdict-bearing but whose expression did not settle.
    NotApplicable,
    /// The element is verdict-bearing and the build's [`EvaluationPolicy`] excluded evaluation.
    NotRun,
    /// The expression settled to a boolean, which for this element is a pass/fail verdict.
    Verdict(bool),
    /// The expression settled to a value that is not a verdict, such as an analysis result.
    ///
    /// Carries the unit when the author wrote one, because the magnitude alone is not the result.
    Computed(EvaluatedScalar),
    /// The element is verdict-bearing and its expression did not settle to a value.
    ///
    /// Carries the state that says why, so an unsupported shape, a cycle and a failed fold stay
    /// distinguishable and none of them can be read as a failing verdict.
    Unsettled(EvaluationState),
}

impl AnalysisEvaluation {
    /// A stable kebab-case name for this outcome, for debug rendering and snapshot output.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotApplicable => "not-applicable",
            Self::NotRun => "not-run",
            Self::Verdict(_) => "verdict",
            Self::Computed(_) => "computed",
            Self::Unsettled(_) => "unsettled",
        }
    }
}

/// Whether a build evaluates constant expressions.
///
/// Makes [`EvaluationState::NotRun`] a declared outcome rather than an empty table that a consumer
/// cannot distinguish from "nothing to evaluate".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EvaluationPolicy {
    /// Evaluate every supported expression.
    #[default]
    Evaluate,
    /// Publish resolution only; every element with an expression reports
    /// [`EvaluationState::NotRun`].
    Skip,
}

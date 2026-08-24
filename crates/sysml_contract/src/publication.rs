//! What a whole publication settled, said once rather than inferred from its contents.
//!
//! A consumer needs to know whether the answers it is about to read are complete before it trusts
//! any one of them, and needs to know which branch of a conditional library rule was taken without
//! reading a stringly anchor name. Both are single scalars, both are the same for every query
//! against the publication, and neither carries a model identity.

/// How complete this publication's answers are.
///
/// Not a boolean and not an error. A publication built from recovered parse trees or containing
/// syntax outside the supported slice still answers queries; what changes is what an absent fact
/// means. Collapsing these three into "incomplete" would leave a consumer unable to say whether a
/// missing answer is a modelling gap or a tooling one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationCompleteness {
    Complete,
    ParseRecovery,
    UnsupportedSyntax,
    NonConverged,
}

/// Which canonical anchor branch a generated conditional library-specialization rule selects.
///
/// Most rules own only [`Self::Default`]. Exact XMI `if … then … else … endif` contracts publish
/// both branches atomically, and consumers select the predicate-true branch without encoding a
/// stringly anchor convention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LibrarySpecializationAnchorBranch {
    Default,
    PredicateTrue,
}

/// Why an attempted constant evaluation produced no value.
///
/// Distinct from "not constant" and "cyclic", which are answers rather than failures: an
/// expression over a feature with no constant value is correctly not constant, and a value cycle
/// is a property of the model. These three are cases where the expression could not be folded at
/// all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EvaluationFailure {
    /// A constant `/` or `%` whose divisor folded to zero.
    ///
    /// Intercepted before the operation runs: integer division would panic and float division
    /// would silently yield an infinity, and neither is an honest published value.
    DivisionByZero,
    /// An operand folded to a type the operator cannot take, such as a boolean in an arithmetic
    /// position.
    TypeMismatch,
    /// An operand reference that resolution left unresolved, ambiguous, unsupported or
    /// non-converged, so what it would evaluate to is unknown.
    UnresolvedOperand,
}

impl EvaluationFailure {
    /// A stable kebab-case name for this failure, for diagnostic text and snapshot output.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DivisionByZero => "division-by-zero",
            Self::TypeMismatch => "type-mismatch",
            Self::UnresolvedOperand => "unresolved-operand",
        }
    }
}

/// What executing one verification case settled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationOutcome {
    /// Execution/evaluation of verification cases is not owned by the immutable model yet.
    Unsupported,
}

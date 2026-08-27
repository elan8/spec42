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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PublicationObstacle {
    ParseRecovery,
    UnsupportedSyntax,
    NonConverged,
}

/// The complete set of obstacles encountered while constructing one publication.
///
/// The representation is private so callers cannot manufacture unknown states. Iteration is in
/// canonical enum order and therefore does not depend on discovery or phase scheduling order.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PublicationCompleteness(u8);

impl PublicationCompleteness {
    #[allow(non_upper_case_globals)]
    pub const Complete: Self = Self(0);
    #[allow(non_upper_case_globals)]
    pub const ParseRecovery: Self = Self(1 << 0);
    #[allow(non_upper_case_globals)]
    pub const UnsupportedSyntax: Self = Self(1 << 1);
    #[allow(non_upper_case_globals)]
    pub const NonConverged: Self = Self(1 << 2);

    pub const fn is_complete(self) -> bool {
        self.0 == 0
    }

    pub const fn contains(self, obstacle: PublicationObstacle) -> bool {
        self.0 & Self::for_obstacle(obstacle).0 != 0
    }

    pub const fn with(self, obstacle: PublicationObstacle) -> Self {
        Self(self.0 | Self::for_obstacle(obstacle).0)
    }

    pub fn obstacles(self) -> impl Iterator<Item = PublicationObstacle> {
        [
            PublicationObstacle::ParseRecovery,
            PublicationObstacle::UnsupportedSyntax,
            PublicationObstacle::NonConverged,
        ]
        .into_iter()
        .filter(move |obstacle| self.contains(*obstacle))
    }

    const fn for_obstacle(obstacle: PublicationObstacle) -> Self {
        match obstacle {
            PublicationObstacle::ParseRecovery => Self::ParseRecovery,
            PublicationObstacle::UnsupportedSyntax => Self::UnsupportedSyntax,
            PublicationObstacle::NonConverged => Self::NonConverged,
        }
    }
}

impl std::fmt::Debug for PublicationCompleteness {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_set().entries(self.obstacles()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::{PublicationCompleteness, PublicationObstacle};

    #[test]
    fn completeness_retains_every_obstacle_in_canonical_order() {
        let completeness = PublicationCompleteness::Complete
            .with(PublicationObstacle::NonConverged)
            .with(PublicationObstacle::ParseRecovery)
            .with(PublicationObstacle::UnsupportedSyntax);

        assert_eq!(
            completeness.obstacles().collect::<Vec<_>>(),
            vec![
                PublicationObstacle::ParseRecovery,
                PublicationObstacle::UnsupportedSyntax,
                PublicationObstacle::NonConverged,
            ]
        );
        assert!(!completeness.is_complete());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryOutcome<T> {
    pub completeness: PublicationCompleteness,
    pub answer: QueryAnswer<T>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryAnswer<T> {
    Resolved(T),
    Unresolved,
    Ambiguous(Box<[T]>),
    Unsupported,
    Recovery,
    Incomplete,
}

impl<T> QueryOutcome<T> {
    pub const fn new(completeness: PublicationCompleteness, answer: QueryAnswer<T>) -> Self {
        Self {
            completeness,
            answer,
        }
    }
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

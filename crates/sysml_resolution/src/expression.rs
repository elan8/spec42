//! The published resolved-expression contract.
//!
//! [`crate::EvaluationState`] is *terminal*: a constraint body with a free variable is
//! [`crate::EvaluationState::NonConstant`] and there is no way to ask what its structure is. This
//! contract is the other half -- the resolved shape of the same authored expression, for a
//! consumer that has to *interpret* a model (assemble equations, emit interface stubs, drive an
//! external solver) rather than fold it to a scalar.
//!
//! It adds no analysis. Resolution already builds and resolves these trees to fold constants; the
//! only change is that the structure is now retained and handed back settled: every feature
//! reference paired with the specific inherited or redefined feature it names, every operator kept,
//! and a shape outside the published slice reported as [`ExpressionNodeKind::Unsupported`] rather
//! than dropped -- the same precedent [`crate::EvaluationState::Unsupported`] sets.
//!
//! The tree is a flat arena. A `Box`-recursive owned tree would allocate per node on every
//! keystroke and pin the authority's node layout; instead [`PublishedExpression::nodes`] is one
//! borrowed slice and a child is named by its `u32` index into it, the same shape the scope and
//! type indexes use.

use crate::{SourceLocation, SymbolId};

/// One element's authored expression, resolved and settled.
///
/// [`Self::outcome`] says whether there is a tree to read; when it is
/// [`ExpressionOutcome::Resolved`], [`Self::root`] indexes the top node of [`Self::nodes`].
#[derive(Debug, Clone, PartialEq)]
pub struct PublishedExpression {
    pub element: SymbolId,
    pub outcome: ExpressionOutcome,
    /// The expression's nodes in a flat arena, children named by index. Empty unless
    /// [`Self::outcome`] is [`ExpressionOutcome::Resolved`].
    pub nodes: Box<[ExpressionNode]>,
    /// The index of the root node in [`Self::nodes`], or `None` when there is no tree.
    pub root: Option<u32>,
}

/// Whether an element has a resolved expression tree, and why not when it does not.
///
/// "This element authored no expression" is a different fact from "this element authored an
/// expression whose shape is outside the published slice", so the two are separate variants. A
/// publication that did not converge is reported by the enclosing [`crate::QueryOutcome`]'s
/// completeness, not here: an unresolved operand still has a place in the tree with no target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpressionOutcome {
    /// The element carries no constraint, calc, or value expression.
    NotApplicable,
    /// The element authored an expression and its resolved tree is in [`PublishedExpression::nodes`].
    Resolved,
    /// The element authored an expression whose syntactic shape is outside the published slice, so
    /// no tree is offered. Matches [`crate::EvaluationState::Unsupported`].
    Unsupported,
}

impl ExpressionOutcome {
    /// A stable kebab-case name, for snapshot output.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not-applicable",
            Self::Resolved => "resolved",
            Self::Unsupported => "unsupported",
        }
    }
}

/// One node of a resolved expression tree.
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionNode {
    pub kind: ExpressionNodeKind,
    /// The node's own source span, so a consumer can point a message at exactly this operator,
    /// literal, or reference.
    pub location: SourceLocation,
}

/// What one resolved expression node is.
///
/// This first slice structures the shapes constant folding already recognises: literals, feature
/// references, and the arithmetic / comparison / boolean / prefix operators. A shape it does not
/// model yet -- an invocation, a `select` / `collect` body, a constructor, an index expression, a
/// meta cast, a type check -- is an [`Self::Unsupported`] node that still lists its subtree's
/// resolved nodes, so a consumer sees *what* an unmodelled operator ranges over without this crate
/// claiming a structure it has not settled.
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNodeKind {
    /// A literal value written in the source (`3`, `true`, `1.5`, `"x"`, `10 [kg]`).
    Literal(crate::EvaluatedScalar),
    /// A reference to a feature, resolved to the specific one it names.
    ///
    /// `symbol` is `None` when the operand reference did not resolve, is ambiguous, or is
    /// unsupported -- the reference exists but its target is not a single settled feature.
    /// `authored` is the qualified name exactly as written; it is materialised here because a
    /// `PublishedExpression` is a boundary answer, not the authority's storage.
    FeatureReference {
        symbol: Option<SymbolId>,
        authored: Box<str>,
    },
    /// A prefix or infix operator over its operands: `[left, right]` for an infix operator,
    /// `[operand]` for a prefix one. Operand order is authored order.
    Operator {
        operator: ExpressionOperator,
        operands: Box<[u32]>,
    },
    /// A shape this slice does not structure. `children` lists the subtree's nodes (its resolved
    /// feature references and any operators over them), so nothing resolved is lost even though
    /// the shape itself is not yet named.
    Unsupported { children: Box<[u32]> },
}

/// A resolved expression operator, mirrored from the parser's operator vocabulary so the facade
/// publishes no parser type. Spellings match the authored source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpressionOperator {
    // Comparison
    Equal,
    NotEqual,
    IdenticalTo,
    NotIdenticalTo,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    // Boolean
    And,
    Or,
    Xor,
    Implies,
    // Prefix
    Negate,
    Not,
}

impl ExpressionOperator {
    /// The operator's authored spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Equal => "==",
            Self::NotEqual => "!=",
            Self::IdenticalTo => "===",
            Self::NotIdenticalTo => "!==",
            Self::Less => "<",
            Self::LessOrEqual => "<=",
            Self::Greater => ">",
            Self::GreaterOrEqual => ">=",
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::Modulo => "%",
            Self::Power => "^",
            Self::And => "and",
            Self::Or => "or",
            Self::Xor => "xor",
            Self::Implies => "implies",
            Self::Negate => "-",
            Self::Not => "not",
        }
    }
}

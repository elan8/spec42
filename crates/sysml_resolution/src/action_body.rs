//! The published body of one action.
//!
//! Entry, do, and effect actions own a sequence of steps (`assign`, `if`, `while`,
//! `accept`, `send`). Those steps are already lowered. This projects them in source order,
//! with each condition and assigned value as the expression the publication already
//! settled. It does not execute the action, and it does not decide which statements
//! belong to an `if` branch when lowering left that unmarked.

use crate::expression::{
    ExpressionNode, ExpressionNodeKind, ExpressionOutcome, ExpressionResultTyping,
    PublishedExpression,
};
use crate::inspection::RelationshipTarget;
use crate::lower::facts::ThenSource;
use crate::model::resolver::ResolvedSemanticModel;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::ReferenceKind;
use crate::state_query::PublishedTrigger;
use crate::QueryAnswer;
use crate::QueryOutcome;
use crate::SymbolId;

/// The steps one action usage owns, in source order.
#[derive(Debug, Clone, PartialEq)]
pub struct PublishedActionBody {
    pub identity: SymbolId,
    pub name: Option<Box<str>>,
    pub steps: Box<[PublishedActionStep]>,
}

/// One step of an action body.
#[derive(Debug, Clone, PartialEq)]
pub enum PublishedActionStep {
    Assign {
        identity: SymbolId,
        target: RelationshipTarget,
        /// The expression after `:=`.
        value: PublishedExpression,
        /// The step this `then assign` succeeds, when one was authored.
        then_source: Option<SymbolId>,
    },
    If {
        identity: SymbolId,
        condition: PublishedExpression,
        then_branch: PublishedActionBranch,
        /// `None` when the `if` has no `else`.
        else_branch: Option<PublishedActionBranch>,
    },
    While {
        identity: SymbolId,
        condition: PublishedExpression,
        body: Box<[PublishedActionStep]>,
    },
    Accept {
        identity: SymbolId,
        trigger: PublishedTrigger,
        /// The trigger argument expression (`1 [s]` in `accept after 1 [s]`).
        argument: PublishedExpression,
        then_source: Option<SymbolId>,
    },
    /// `send <payload> to <receiver>`. `receiver` is absent when no `to` clause was written.
    ///
    /// `send new Signal()` with no arguments publishes `Signal` as the payload reference.
    /// The expression slice leaves constructors unstructured, and the signal type is already
    /// resolved as the constructor's callee. A constructor that carries arguments is not
    /// rewritten into that reference.
    Send {
        identity: SymbolId,
        payload: PublishedExpression,
        receiver: Option<RelationshipTarget>,
        then_source: Option<SymbolId>,
    },
    /// A nested action usage and the steps it owns.
    Action {
        identity: SymbolId,
        name: Option<Box<str>>,
        steps: Box<[PublishedActionStep]>,
    },
    /// A step was authored and this query does not structure it. The identity is kept.
    Unsupported { identity: SymbolId },
}

/// One branch of an `if`.
///
/// `Unmarked` means the branch was authored and the owned actions were not marked as
/// that branch. The steps are not placed in the other branch instead.
#[derive(Debug, Clone, PartialEq)]
pub enum PublishedActionBranch {
    Steps(Box<[PublishedActionStep]>),
    Unmarked,
}

impl ResolvedSemanticModel {
    /// The steps `symbol` owns, or `None` when it is not an action usage or definition.
    pub(crate) fn action_body(
        &self,
        symbol: SymbolId,
    ) -> QueryOutcome<Option<PublishedActionBody>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(record) = self.storage.declaration(declaration) else {
            return self.query_outcome(QueryAnswer::Unresolved);
        };
        if !is_action_body(record.kind) {
            return self.resolved_outcome(None);
        }
        let Ok(steps) = self.action_steps(declaration) else {
            return self.query_outcome(QueryAnswer::Unresolved);
        };
        self.resolved_outcome(Some(PublishedActionBody {
            identity: symbol,
            name: self.symbol_name(symbol).map(Into::into),
            steps,
        }))
    }

    fn action_steps(&self, owner: DeclarationId) -> Result<Box<[PublishedActionStep]>, ()> {
        let mut steps = Vec::new();
        for child in self.child_declarations(owner).iter().copied() {
            if let Some(step) = self.project_step(child)? {
                steps.push(step);
            }
        }
        Ok(steps.into())
    }

    fn project_step(&self, declaration: DeclarationId) -> Result<Option<PublishedActionStep>, ()> {
        let Some(kind) = self.declaration_kind(declaration) else {
            return Ok(None);
        };
        let Some(identity) = self.symbol_id(declaration) else {
            return Err(());
        };
        let then_source = self.then_source_symbol(declaration);
        Ok(Some(match kind {
            DeclarationKind::Assign => PublishedActionStep::Assign {
                identity,
                target: self.reference_target(declaration, ReferenceKind::AssignTarget),
                value: self.expression_of(declaration, identity)?,
                then_source,
            },
            DeclarationKind::If => {
                let has_else = self
                    .storage
                    .declaration_facts(declaration)
                    .and_then(|facts| facts.has_else_action)
                    == Some(true);
                let (then_branch, else_branch) = self.if_branches(declaration, has_else)?;
                PublishedActionStep::If {
                    identity,
                    condition: self.expression_of(declaration, identity)?,
                    then_branch,
                    else_branch,
                }
            }
            DeclarationKind::While | DeclarationKind::Loop => PublishedActionStep::While {
                identity,
                condition: self.expression_of(declaration, identity)?,
                body: self.action_steps(declaration)?,
            },
            DeclarationKind::AcceptActionUsage => {
                let (trigger, argument) = self.accept_trigger(declaration)?;
                PublishedActionStep::Accept {
                    identity,
                    trigger,
                    argument,
                    then_source,
                }
            }
            DeclarationKind::SendActionUsage => {
                let has_receiver = self
                    .storage
                    .declaration_facts(declaration)
                    .and_then(|facts| facts.send_has_receiver_argument)
                    == Some(true);
                PublishedActionStep::Send {
                    identity,
                    payload: self.send_payload(declaration, identity)?,
                    receiver: has_receiver
                        .then(|| self.reference_target(declaration, ReferenceKind::SendTarget)),
                    then_source,
                }
            }
            DeclarationKind::ActionUsage | DeclarationKind::PerformActionUsage => {
                PublishedActionStep::Action {
                    identity,
                    name: self.symbol_name(identity).map(Into::into),
                    steps: self.action_steps(declaration)?,
                }
            }
            DeclarationKind::TerminateActionUsage
            | DeclarationKind::ForLoop
            | DeclarationKind::Decide
            | DeclarationKind::Merge
            | DeclarationKind::Fork
            | DeclarationKind::Join => PublishedActionStep::Unsupported { identity },
            _ => return Ok(None),
        }))
    }

    /// `then` is position 2 and `else` is position 3, the marks lowering records when a
    /// branch owns one action. A branch with several unmarked actions stays
    /// [`PublishedActionBranch::Unmarked`].
    fn if_branches(
        &self,
        statement: DeclarationId,
        has_else: bool,
    ) -> Result<(PublishedActionBranch, Option<PublishedActionBranch>), ()> {
        let mut then_steps = Vec::new();
        let mut else_steps = Vec::new();
        let mut unmarked = false;
        for child in self.child_declarations(statement).iter().copied() {
            if !is_step_kind(self.declaration_kind(child)) {
                continue;
            }
            match self
                .storage
                .declaration_facts(child)
                .and_then(|facts| facts.action_input_parameter_position)
            {
                Some(2) => then_steps.push(self.project_step(child)?.ok_or(())?),
                Some(3) if has_else => else_steps.push(self.project_step(child)?.ok_or(())?),
                _ => unmarked = true,
            }
        }
        if unmarked {
            return Ok((
                PublishedActionBranch::Unmarked,
                has_else.then_some(PublishedActionBranch::Unmarked),
            ));
        }
        Ok((
            PublishedActionBranch::Steps(then_steps.into()),
            has_else.then(|| PublishedActionBranch::Steps(else_steps.into())),
        ))
    }

    fn accept_trigger(
        &self,
        accept: DeclarationId,
    ) -> Result<(PublishedTrigger, PublishedExpression), ()> {
        for child in self.child_declarations(accept).iter().copied() {
            let Some(facts) = self.storage.declaration_facts(child) else {
                continue;
            };
            let Some(kind) = facts.trigger_kind else {
                continue;
            };
            let Some(argument) = facts.trigger_argument_text.clone() else {
                return Ok((
                    PublishedTrigger::Unsupported,
                    self.expression_of(child, self.symbol_id(child).ok_or(())?)?,
                ));
            };
            let Some(target) = self.symbol_id(child) else {
                return Err(());
            };
            return Ok((
                PublishedTrigger::Accept {
                    label: format!("{} {argument}", kind.as_str()).into(),
                    target,
                },
                self.expression_of(child, target)?,
            ));
        }
        let absent = self.expression_of(accept, self.symbol_id(accept).ok_or(())?)?;
        Ok((PublishedTrigger::Unresolved, absent))
    }

    fn expression_of(
        &self,
        declaration: DeclarationId,
        element: SymbolId,
    ) -> Result<PublishedExpression, ()> {
        self.published_expression(declaration, element).ok_or(())
    }

    /// The payload of a send step.
    ///
    /// A settled expression is published as itself. `send new Signal()` is a constructor, which
    /// the expression slice does not structure, so the send declaration has no expression row.
    /// The constructor's callee is the signal type. When that constructor has no arguments, the
    /// payload is a reference to that type. Arguments stay unpublished rather than being dropped.
    fn send_payload(
        &self,
        declaration: DeclarationId,
        element: SymbolId,
    ) -> Result<PublishedExpression, ()> {
        let expression = self.expression_of(declaration, element)?;
        if expression.outcome == ExpressionOutcome::Resolved {
            return Ok(expression);
        }
        let Some(payload) = self.signal_constructor(declaration, element) else {
            return Ok(expression);
        };
        Ok(payload)
    }

    fn signal_constructor(
        &self,
        declaration: DeclarationId,
        element: SymbolId,
    ) -> Option<PublishedExpression> {
        let expressions = self
            .child_declarations(declaration)
            .iter()
            .copied()
            .filter(|child| {
                self.storage
                    .declaration(*child)
                    .is_some_and(|declaration| declaration.kind == DeclarationKind::KermlExpression)
            })
            .collect::<Vec<_>>();
        let [expression] = expressions.as_slice() else {
            return None;
        };
        if self.owns_argument(*expression) {
            return None;
        }
        let relationships =
            self.relationships_of_kinds(*expression, &[ReferenceKind::InvocationCallee]);
        let [relationship] = relationships.as_ref() else {
            return None;
        };
        let RelationshipTarget::Resolved(symbol) = relationship.target else {
            return None;
        };
        let location = relationship.location?;
        let authored = relationship.authored.clone().unwrap_or_else(|| {
            self.symbol_name(symbol)
                .unwrap_or("<signal>")
                .to_owned()
                .into_boxed_str()
        });
        Some(PublishedExpression {
            element,
            outcome: ExpressionOutcome::Resolved,
            result_typing: ExpressionResultTyping::NotApplicable,
            nodes: Box::new([ExpressionNode {
                kind: ExpressionNodeKind::FeatureReference {
                    symbol: Some(symbol),
                    authored,
                },
                location,
            }]),
            root: Some(0),
        })
    }

    fn owns_argument(&self, declaration: DeclarationId) -> bool {
        let mut pending = self.child_declarations(declaration).to_vec();
        while let Some(child) = pending.pop() {
            if self
                .storage
                .declaration_facts(child)
                .is_some_and(|facts| facts.instantiation_argument.is_some())
            {
                return true;
            }
            pending.extend(self.child_declarations(child).iter().copied());
        }
        false
    }

    fn then_source_symbol(&self, declaration: DeclarationId) -> Option<SymbolId> {
        let source = self
            .storage
            .declaration_facts(declaration)
            .and_then(|facts| facts.then_source.as_ref())?;
        let source = match source {
            ThenSource::Declaration(source) | ThenSource::InitialNode(source) => *source,
        };
        self.symbol_id(source)
    }
}

fn is_action_body(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::ActionUsage
            | DeclarationKind::ActionDefinition
            | DeclarationKind::PerformActionUsage
    )
}

fn is_step_kind(kind: Option<DeclarationKind>) -> bool {
    matches!(
        kind,
        Some(
            DeclarationKind::Assign
                | DeclarationKind::If
                | DeclarationKind::While
                | DeclarationKind::Loop
                | DeclarationKind::AcceptActionUsage
                | DeclarationKind::ActionUsage
                | DeclarationKind::PerformActionUsage
                | DeclarationKind::SendActionUsage
                | DeclarationKind::TerminateActionUsage
                | DeclarationKind::ForLoop
                | DeclarationKind::Decide
                | DeclarationKind::Merge
                | DeclarationKind::Fork
                | DeclarationKind::Join
        )
    )
}

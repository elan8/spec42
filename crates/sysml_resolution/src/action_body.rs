//! The published body of one action.
//!
//! Entry, do, and effect actions own a sequence of steps (`assign`, `if`, `while`,
//! `accept`). Those steps are already lowered. This projects them in source order,
//! with each condition and assigned value as the expression the publication already
//! settled. It does not execute the action, and it does not decide which statements
//! belong to an `if` branch when lowering left that unmarked.

use crate::expression::PublishedExpression;
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
            DeclarationKind::ActionUsage | DeclarationKind::PerformActionUsage => {
                PublishedActionStep::Action {
                    identity,
                    name: self.symbol_name(identity).map(Into::into),
                    steps: self.action_steps(declaration)?,
                }
            }
            DeclarationKind::SendActionUsage
            | DeclarationKind::TerminateActionUsage
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

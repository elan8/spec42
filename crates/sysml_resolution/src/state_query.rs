//! The published state-machine contract.
//!
//! A consumer that has to *interpret* behaviour (a discrete runtime, a verification
//! scenario, a firmware trace) needs the states and transitions of one state usage or
//! state definition as resolved structure. A state-transition view is a rendering of
//! that structure, not the structure itself: the kitchen-timer countdown is a `state`
//! usage, and it has no `StateTransitionView`.
//!
//! This adds no analysis. Sources, targets, triggers, guards and effects are facts
//! resolution already settled. An unresolved, ambiguous or unsupported reference stays
//! that outcome; it is never dropped, and it is never reported as "there is no such
//! part of the transition".

use crate::inspection::RelationshipTarget;
use crate::SymbolId;

/// One state usage or state definition and the states and transitions it owns directly.
///
/// Nested states are their own machines: query the nested usage to read what it owns.
/// `initial` is `None` when no `then <state>;` initial succession was authored. When one
/// was authored, the target is that succession's outcome, including unresolved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedStateMachine {
    pub identity: SymbolId,
    pub name: Option<Box<str>>,
    /// The `then <state>;` initial succession's target.
    pub initial: Option<RelationshipTarget>,
    /// Directly owned states, in source order.
    pub states: Box<[PublishedState]>,
    /// Directly owned transitions, in source order.
    pub transitions: Box<[PublishedTransition]>,
}

/// One state owned by a [`PublishedStateMachine`].
///
/// `entry`, `do_action` and `exit` are the actions the state itself owns in those
/// roles. An inherited library action is not reported here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedState {
    pub identity: SymbolId,
    pub name: Option<Box<str>>,
    pub entry: Option<SymbolId>,
    pub do_action: Option<SymbolId>,
    pub exit: Option<SymbolId>,
}

/// One transition owned by a [`PublishedStateMachine`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedTransition {
    pub identity: SymbolId,
    pub name: Option<Box<str>>,
    /// `first <state>`, or the implicit source of a target-only shorthand.
    pub source: RelationshipTarget,
    /// `then <state>`.
    pub target: RelationshipTarget,
    pub trigger: PublishedTrigger,
    /// `if <expr>` as written, or `else`. `None` when the transition has no guard.
    pub guard: Option<PublishedGuard>,
    pub effect: PublishedEffect,
}

/// The accepting feature of a transition.
///
/// `Absent` means the transition was written without `accept`. The other variants are
/// a trigger that was written and could not be settled to one feature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublishedTrigger {
    Absent,
    /// `accept Name` or `accept when` / `at` / `after`.
    ///
    /// `label` is the payload type name, or the trigger keyword plus its argument as
    /// written (`when remaining == 0 [s]`). `target` is the accepting feature.
    Accept {
        label: Box<str>,
        target: SymbolId,
    },
    Unresolved,
    Ambiguous,
    Unsupported,
}

/// A transition guard, as authored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublishedGuard {
    /// The `if` expression with its white space collapsed. Not an evaluated value.
    Condition(Box<str>),
    /// The `else` succession.
    Else,
}

/// A transition's `do` effect.
///
/// `Absent` means no effect was authored. A written effect whose target did not settle
/// is not `Absent`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublishedEffect {
    Absent,
    Resolved(SymbolId),
    Unresolved,
    Ambiguous,
    Unsupported,
}

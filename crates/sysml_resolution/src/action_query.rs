//! Closed exact Systems::Actions derived-property query vocabulary.

use crate::SymbolIdentity;
pub use spec42_constraint_manifest::ActionDerivedFactKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActionDerivedFactCollection {
    ActionDefinitionAction,
    AssignmentValueExpression,
    AssignmentTargetArgument,
    AssignmentReferent,
    ForLoopVariable,
    ForLoopSeqArgument,
    LoopBodyAction,
    TerminateOccurrenceArgument,
    AcceptPayloadArgument,
    AcceptPayloadParameter,
    AcceptReceiverArgument,
    WhileArgument,
    UntilArgument,
    SendSenderArgument,
    SendReceiverArgument,
    SendPayloadArgument,
    IfThenAction,
    IfElseAction,
    IfArgument,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionDerivedFactPrerequisite {
    EffectiveUsageClosure,
    OrderedActionArgumentIdentity,
    OrderedInputParameterIdentity,
    OwnedMembershipIdentity,
    OrderedOwnedFeatureIdentity,
    ActionMetaclassIdentity,
    RuleNotPublished,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionDerivedFactOutcome {
    Values(Box<[SymbolIdentity]>),
    Unsupported {
        prerequisite: ActionDerivedFactPrerequisite,
    },
}

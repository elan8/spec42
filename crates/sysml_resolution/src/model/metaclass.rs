//! Canonical SysML definition/usage metaclass hierarchy, shared by construction and checks.

use crate::model::DeclarationKind;

/// The SysML metaclass family a declaration belongs to.
///
/// One variant per definition/usage pair the language provides. Two declarations in the same
/// family are the definition and usage halves of one concept (`part def` / `part`), which is what
/// makes a family the right key for every rule below.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Family {
    Part,
    Attribute,
    Enumeration,
    Item,
    Occurrence,
    Action,
    State,
    Port,
    Requirement,
    Concern,
    UseCase,
    Case,
    AnalysisCase,
    VerificationCase,
    View,
    Viewpoint,
    Rendering,
    Metadata,
    Connection,
    Interface,
    Flow,
    Allocation,
    Constraint,
    Calc,
}

/// Which half of its family a declaration is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Role {
    Definition,
    Usage,
}

/// The family and role of one declaration, or `None` when it is outside the SysML metaclass space.
///
/// Exhaustive by construction: a new `DeclarationKind` fails to compile until it is classified, so
/// a kind cannot silently acquire "no conformance rule applies" by omission.
pub(crate) fn classify(kind: DeclarationKind) -> Option<(Family, Role)> {
    use DeclarationKind as K;
    use Family as F;
    use Role::{Definition, Usage};
    Some(match kind {
        K::PartDefinition => (F::Part, Definition),
        K::PartUsage => (F::Part, Usage),
        K::AttributeDefinition => (F::Attribute, Definition),
        K::AttributeUsage => (F::Attribute, Usage),
        K::EnumerationDefinition => (F::Enumeration, Definition),
        K::EnumerationUsage => (F::Enumeration, Usage),
        K::ItemDefinition => (F::Item, Definition),
        K::ItemUsage => (F::Item, Usage),
        K::OccurrenceDefinition => (F::Occurrence, Definition),
        K::OccurrenceUsage => (F::Occurrence, Usage),
        K::ActionDefinition => (F::Action, Definition),
        K::ActionUsage => (F::Action, Usage),
        K::AcceptActionUsage | K::SendActionUsage | K::TerminateActionUsage => (F::Action, Usage),
        K::StateDefinition => (F::State, Definition),
        K::StateUsage => (F::State, Usage),
        K::PortDefinition => (F::Port, Definition),
        K::PortUsage => (F::Port, Usage),
        K::RequirementDefinition => (F::Requirement, Definition),
        K::RequirementUsage => (F::Requirement, Usage),
        K::ConcernDefinition => (F::Concern, Definition),
        K::ConcernUsage => (F::Concern, Usage),
        K::UseCaseDefinition => (F::UseCase, Definition),
        K::UseCaseUsage => (F::UseCase, Usage),
        K::CaseDefinition => (F::Case, Definition),
        K::CaseUsage => (F::Case, Usage),
        K::AnalysisCaseDefinition => (F::AnalysisCase, Definition),
        K::AnalysisCaseUsage => (F::AnalysisCase, Usage),
        K::VerificationCaseDefinition => (F::VerificationCase, Definition),
        K::VerificationCaseUsage => (F::VerificationCase, Usage),
        K::ViewDefinition => (F::View, Definition),
        K::ViewUsage => (F::View, Usage),
        K::ViewpointDefinition => (F::Viewpoint, Definition),
        K::ViewpointUsage => (F::Viewpoint, Usage),
        K::RenderingDefinition => (F::Rendering, Definition),
        K::RenderingUsage => (F::Rendering, Usage),
        K::MetadataDefinition => (F::Metadata, Definition),
        K::MetadataUsage => (F::Metadata, Usage),
        K::ConnectionDefinition => (F::Connection, Definition),
        K::ConnectionUsage => (F::Connection, Usage),
        K::InterfaceDefinition => (F::Interface, Definition),
        K::InterfaceUsage => (F::Interface, Usage),
        K::FlowDefinition => (F::Flow, Definition),
        K::Flow => (F::Flow, Usage),
        K::AllocationDefinition => (F::Allocation, Definition),
        K::Allocate => (F::Allocation, Usage),
        K::ConstraintDefinition => (F::Constraint, Definition),
        K::ConstraintUsage => (F::Constraint, Usage),
        K::CalcDefinition => (F::Calc, Definition),
        K::CalcUsage => (F::Calc, Usage),

        // Outside the definition/usage kind space this family of rules describes.
        //
        // Namespaces and their members are not typed. The KerML metaclasses are `Type`s, but the
        // SysML family tables do not describe them, and the SysML families bottom out in them --
        // reporting against a table that does not model KerML would flag most of the standard
        // library. The remaining entries are usage forms whose metaclass is fixed by their
        // syntax (`subject`, `actor`, a constraint assertion), anonymous scopes the lowering mints
        // to give nested references somewhere to resolve, and control-flow members that carry
        // references rather than a typing of their own.
        K::Namespace
        | K::Package
        | K::LibraryPackage
        | K::Import
        | K::Expose
        | K::Alias
        | K::EnumerationLiteral
        | K::ClassDefinition
        | K::ExtendedDefinition
        | K::Succession
        | K::EntryActionBinding
        | K::DoActionBinding
        | K::ExitActionBinding
        | K::InitialState
        | K::FinalState
        | K::ParameterUsage
        | K::SubjectUsage
        | K::PerformActionUsage
        | K::Transition
        | K::Satisfy
        | K::Bind
        | K::ReferenceUsage
        | K::Decide
        | K::Merge
        | K::Fork
        | K::Join
        | K::ThenContinuation
        | K::StakeholderUsage
        | K::RequirementActor
        | K::CaseActor
        | K::Frame
        | K::VerifyRequirement
        | K::AssertConstraintUsage
        | K::AssumeConstraintUsage
        | K::RequireConstraintUsage
        | K::DefaultReferenceUsage
        | K::ExtendedUsage
        | K::Assign
        | K::While
        | K::Loop
        | K::If
        | K::ForLoop
        | K::ForLoopVariable
        | K::Dependency
        | K::BareConnect
        | K::PerformParameterBinding
        | K::KermlType
        | K::KermlClassifier
        | K::KermlStructure
        | K::KermlAssociation
        | K::KermlAssociationStructure
        | K::KermlDataType
        | K::KermlMetaclass
        | K::KermlBehavior
        | K::KermlFunction
        | K::KermlPredicate
        | K::KermlInteraction
        | K::KermlMultiplicity
        | K::KermlFeature
        | K::KermlStep
        | K::KermlExpression
        | K::KermlBooleanExpression
        | K::KermlConnector
        | K::KermlBinding
        | K::KermlInvariant
        | K::KermlEnd
        | K::CommentUsage => return None,
    })
}

/// The family a family specialises in the SysML metamodel, or `None` at a root.
///
/// This is the metamodel's own generalization hierarchy -- SysML §7's `PartUsage :> ItemUsage :>
/// OccurrenceUsage`, `StateUsage :> ActionUsage`, `RequirementUsage :> ConstraintUsage` and so on.
/// It is a static property of the language, not of any admitted library, so reading it here is not
/// a name lookup: nothing consults what `Occurrence` happens to be called or where it is declared.
///
/// A flat per-family allowlist could not express it. `action substates : StateAction[0..*];` types
/// an action usage with a state definition, which is well-formed precisely because a state *is* an
/// action; a list that did not happen to name `State` under `Action` reported it as a violation.
pub(crate) fn parent(family: Family) -> Option<Family> {
    use Family as F;
    Some(match family {
        F::Occurrence | F::Attribute => return None,
        F::Item => F::Occurrence,
        F::Part => F::Item,
        F::Action => F::Occurrence,
        F::State => F::Action,
        F::Calc => F::Action,
        F::Case => F::Calc,
        F::UseCase => F::Case,
        F::AnalysisCase => F::Case,
        F::VerificationCase => F::Case,
        F::Port => F::Occurrence,
        F::Connection => F::Part,
        F::Interface => F::Connection,
        F::Flow => F::Action,
        F::Allocation => F::Connection,
        F::View => F::Part,
        F::Viewpoint => F::Requirement,
        F::Rendering => F::Part,
        F::Metadata => F::Item,
        F::Enumeration => F::Attribute,
        F::Constraint => F::Occurrence,
        F::Requirement => F::Constraint,
        F::Concern => F::Requirement,
    })
}

/// Whether `family` is an occurrence family: `Occurrence` itself or one specialising it.
///
/// SysML's flow payload rule is about occurrences, and the occurrence families are exactly those
/// under `Occurrence` in the hierarchy above -- part, item, action, state, connection and the rest.
/// Asking the hierarchy keeps the rule from naming a library type.
pub(crate) fn descends_from_occurrence(family: Family) -> bool {
    descends_from(family, Family::Occurrence)
}

/// Whether `descendant` is `ancestor` or specialises it, transitively.
pub(crate) fn descends_from(descendant: Family, ancestor: Family) -> bool {
    let mut cursor = Some(descendant);
    while let Some(current) = cursor {
        if current == ancestor {
            return true;
        }
        cursor = parent(current);
    }
    false
}

/// Whether the canonical metaclass is an OccurrenceDefinition or a specialization of it.
pub(crate) fn is_occurrence_definition(kind: DeclarationKind) -> bool {
    classify(kind)
        .is_some_and(|(family, role)| role == Role::Definition && descends_from_occurrence(family))
}

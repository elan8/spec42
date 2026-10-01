//! Phase 2 lowering — state machines: state definitions and usages, transitions, entry/do/exit actions.

use crate::lower::facts::definition_prefix_node_modifiers;
use crate::lower::facts::direction_fact;
use crate::lower::facts::multiplicity_facts;
use crate::lower::facts::DeclarationFacts;
use crate::lower::facts::DeclarationModifiers;
use crate::lower::facts::ParameterDirection;
use crate::lower::facts::PendingReference;
use crate::lower::facts::RelationshipFlags;
use crate::lower::facts::TransitionFeatureRole;
use crate::lower::facts::UnsupportedFamily;
use crate::lower::SemanticModelBuilder;
use crate::model::ConstructionError;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::DocumentIdx;
use crate::model::MembershipKind;
use crate::model::ReferenceKind;
use crate::model::Visibility;
use crate::MembershipRole;
use crate::StateSubactionKind;
use sysml_v2_parser::ast::{
    DeclarationName, DoAction, EntryAction, ExhibitState as ParserExhibitState, ExitAction,
    Expression, FinalState, MembershipKind as ParserMembershipKind, Node, QualifiedReferenceId,
    Span, StateBodyModifier, StateDef, StateDefBody, StateDefBodyElement,
    StateUsage as ParserStateUsage, SubsettingRelationship, ThenStmt, Transition, TransitionAccept,
    TransitionEffect,
};

/// `StateDefinition::isParallel` / `StateUsage::isParallel` from the authored body modifier.
fn state_body_is_parallel(modifier: Option<&Node<StateBodyModifier>>) -> bool {
    matches!(modifier, Some(node) if node.value == StateBodyModifier::Parallel)
}

impl SemanticModelBuilder {
    /// Publishes the part of a state body modifier that is not a SysML fact. `parallel` is
    /// `isParallel` and is recorded as a declaration modifier by the caller; the pinned parser's
    /// `initial` body modifier has no production in `StateDefBody`/`StateUsageBody` (SysML BNF
    /// 1192: `( isParallel ?= 'parallel' )?`), so it is kept visible as unsupported syntax rather
    /// than invented into a semantic fact or silently dropped.
    pub(crate) fn lower_state_body_modifier(
        &mut self,
        document: DocumentIdx,
        family: UnsupportedFamily,
        modifier: Option<&Node<StateBodyModifier>>,
    ) {
        if let Some(node) = modifier {
            if node.value == StateBodyModifier::Initial {
                self.push_unsupported(document, family, node.span);
            }
        }
    }

    /// Lowers a `state def` (BNF StateDefinition), mirroring `lower_action_def`: ownership,
    /// membership, an optional `:>` specialization relationship, and owned declarations.
    /// State-machine-specific semantics (entry/do/exit action bindings, transitions, exclusive/
    /// parallel substates, history) are explicitly out of scope; unrecognized body elements fall
    /// through to `unsupported_state_definition_member` via `lower_state_def_body`.
    pub(crate) fn lower_state_def(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<StateDef>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.identification.name)?;
        let short_name = self.intern_short_name(document, node.identification.short_name)?;
        let (is_abstract, variation) =
            definition_prefix_node_modifiers(node.value.definition_prefix.as_ref());
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::StateDefinition,
            name,
            node.span,
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract,
                    variation,
                    individual: node.value.is_individual,
                    parallel: state_body_is_parallel(node.value.body_modifier.as_ref()),
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.lower_state_body_modifier(
            document,
            UnsupportedFamily::StateDefinitionMember,
            node.value.body_modifier.as_ref(),
        );
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span,
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_state_def_body(document, declaration, &node.value.body)
    }

    /// Lowers the `StateDefBody` shared by `state def` and by a `state` usage's own owned
    /// members (BNF `StateDefBodyElement`): nested state/requirement usages, entry/do/exit action
    /// bindings, `then`/`final` state markers, `ref` bindings, and transitions are all lowered.
    /// `StateDefBodyElement` also carries `AttributeUsage`/`ActionUsage`/`AssertConstraint`/
    /// `SuccessionUsage` variants, each dispatching to its existing lowering.
    pub(crate) fn lower_state_def_body(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        body: &StateDefBody,
    ) -> Result<(), ConstructionError> {
        let StateDefBody::Brace { elements, .. } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                StateDefBodyElement::Error(error) => {
                    self.push_recovery(document, error.span);
                }
                StateDefBodyElement::PartUsage(node) => {
                    // New upstream member kind: kept visible as unsupported rather than dropped.
                    self.push_unsupported(
                        document,
                        UnsupportedFamily::StateDefinitionMember,
                        node.span,
                    );
                }
                StateDefBodyElement::ConstraintUsage(node) => {
                    // New upstream member kind: kept visible as unsupported rather than dropped.
                    self.push_unsupported(
                        document,
                        UnsupportedFamily::StateDefinitionMember,
                        node.span,
                    );
                }
                StateDefBodyElement::StateUsage(state_usage) => {
                    self.lower_state_usage(document, Some(owner), state_usage)?;
                }
                StateDefBodyElement::RequirementUsage(requirement_usage) => {
                    self.lower_requirement_usage(document, Some(owner), requirement_usage)?;
                }
                StateDefBodyElement::Annotating(member) => {
                    self.lower_annotating_member(
                        document,
                        Some(owner),
                        UnsupportedFamily::StateDefinitionMember,
                        member,
                    )?;
                }
                StateDefBodyElement::Entry(entry) => {
                    self.lower_state_entry_action(document, owner, entry)?;
                }
                StateDefBodyElement::Do(action) => {
                    self.lower_state_do_action(document, owner, action)?;
                }
                StateDefBodyElement::Exit(exit) => {
                    self.lower_state_exit_action(document, owner, exit)?;
                }
                StateDefBodyElement::Then(then) => {
                    self.lower_state_then_stmt(document, owner, then)?;
                }
                StateDefBodyElement::Transition(transition) => {
                    self.lower_transition(document, owner, transition)?;
                }
                StateDefBodyElement::InOutDecl(param) => {
                    self.lower_parameter_declaration(
                        document,
                        Some(owner),
                        UnsupportedFamily::StateDefinitionMember,
                        param,
                    )?;
                }
                StateDefBodyElement::Ref(node) => {
                    self.lower_ref_decl(document, Some(owner), node)?;
                }
                StateDefBodyElement::FinalState(node) => {
                    self.lower_final_state(document, owner, node)?;
                }
                StateDefBodyElement::AttributeUsage(node) => {
                    self.lower_attribute_usage(document, Some(owner), node)?;
                }
                StateDefBodyElement::ActionUsage(node) => {
                    self.lower_action_usage(document, Some(owner), node)?;
                }
                StateDefBodyElement::AssertConstraint(node) => {
                    self.lower_assert_constraint_member(document, owner, node)?
                }
                StateDefBodyElement::SuccessionUsage(node) => self.lower_succession_usage(
                    document,
                    owner,
                    UnsupportedFamily::StateDefinitionMember,
                    node,
                )?,
                StateDefBodyElement::MetadataKeywordUsage(_) => self.push_unsupported(
                    document,
                    UnsupportedFamily::StateDefinitionMember,
                    element.span,
                ),
                // `first <node>;` / `then <target>;` action-flow statements, produced only for an
                // `entry`/`do`/`exit` action body (a SysML `ActionBody`), never a plain `state`
                // body (Apollo 11 `state def PrepareForMissionPhase`'s `do action`). Lowered
                // through the same succession machinery the action-def/action-usage bodies use.
                StateDefBodyElement::FirstStmt(node) => {
                    self.lower_first_stmt(
                        document,
                        owner,
                        UnsupportedFamily::StateDefinitionMember,
                        node,
                    )?;
                }
                StateDefBodyElement::ThenAction(node) => {
                    self.lower_then_action(
                        document,
                        owner,
                        UnsupportedFamily::StateDefinitionMember,
                        node,
                    )?;
                }
            }
        }
        Ok(())
    }

    /// Lowers a state def/usage's `entry` body element (BNF `EntryActionMember`).
    pub(crate) fn lower_state_entry_action(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        node: &Node<EntryAction>,
    ) -> Result<(), ConstructionError> {
        let EntryAction {
            action_reference,
            declared_name,
            type_name,
            redefines,
            effect,
            body,
            ..
        } = &node.value;
        self.lower_state_subaction(
            document,
            owner,
            StateSubactionKind::Entry,
            StateSubactionSyntax {
                action_reference: *action_reference,
                declared_name: *declared_name,
                type_name: *type_name,
                redefines: redefines.as_ref(),
                effect: effect.as_ref(),
                body,
            },
            node.span,
        )
    }

    /// Lowers a state def/usage's `do` body element (BNF `DoActionMember`).
    pub(crate) fn lower_state_do_action(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        node: &Node<DoAction>,
    ) -> Result<(), ConstructionError> {
        let DoAction {
            action_reference,
            declared_name,
            type_name,
            redefines,
            effect,
            body,
            ..
        } = &node.value;
        self.lower_state_subaction(
            document,
            owner,
            StateSubactionKind::Do,
            StateSubactionSyntax {
                action_reference: *action_reference,
                declared_name: *declared_name,
                type_name: *type_name,
                redefines: redefines.as_ref(),
                effect: effect.as_ref(),
                body,
            },
            node.span,
        )
    }

    /// Lowers a state def/usage's `exit` body element (BNF `ExitActionMember`).
    pub(crate) fn lower_state_exit_action(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        node: &Node<ExitAction>,
    ) -> Result<(), ConstructionError> {
        let ExitAction {
            action_reference,
            declared_name,
            type_name,
            redefines,
            effect,
            body,
            ..
        } = &node.value;
        self.lower_state_subaction(
            document,
            owner,
            StateSubactionKind::Exit,
            StateSubactionSyntax {
                action_reference: *action_reference,
                declared_name: *declared_name,
                type_name: *type_name,
                redefines: redefines.as_ref(),
                effect: effect.as_ref(),
                body,
            },
            node.span,
        )
    }

    /// Lowers the StateActionUsage of one `entry`/`do`/`exit` member (SysML BNF
    /// `StateActionUsage`) as the member of a StateSubactionMembership of `kind`, owned by the
    /// enclosing state `owner`:
    ///
    /// - the declaration form (`entry action entryAction :>> 'entry';`) introduces a named action
    ///   ([`Self::lower_state_declared_action`]);
    /// - the reference form (`entry action <path>;`) binds an existing action through an
    ///   anonymous binding declaration whose reference resolves in the state's own scope, the same
    ///   shared lexical lookup as `AliasBinding`/`Succession`;
    /// - the effect forms (`entry assign ...;`, `do send ...;`, `do accept ...;`) publish the
    ///   AssignmentActionUsage, SendActionUsage or AcceptActionUsage itself, whose membership
    ///   carries the role; the effect's operands are not lowered and stay explicitly unsupported,
    ///   as a transition effect's are;
    /// - a bare `entry;` is the `EmptyActionUsage` alternative, an anonymous ActionUsage.
    ///
    /// Every form therefore publishes its occupant of the StateSubactionMembership, which
    /// `checkActionUsageStateActionRedefinition` enumerates. A body with owned members on a
    /// reference or bare form has no field in this typed AST shape and stays an explicit
    /// unsupported diagnostic.
    fn lower_state_subaction(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        kind: StateSubactionKind,
        syntax: StateSubactionSyntax<'_>,
        span: Span,
    ) -> Result<(), ConstructionError> {
        let (binding_kind, reference_kind) = match kind {
            StateSubactionKind::Entry => (
                DeclarationKind::EntryActionBinding,
                ReferenceKind::EntryActionBinding,
            ),
            StateSubactionKind::Do => (
                DeclarationKind::DoActionBinding,
                ReferenceKind::DoActionBinding,
            ),
            StateSubactionKind::Exit => (
                DeclarationKind::ExitActionBinding,
                ReferenceKind::ExitActionBinding,
            ),
        };
        if let Some(declared_name) = syntax.declared_name {
            return self.lower_state_declared_action(
                document,
                owner,
                binding_kind,
                declared_name,
                syntax.type_name,
                syntax.redefines,
                syntax.body,
                span,
            );
        }
        if let Some(effect) = syntax.effect {
            let (effect_kind, name) = match effect {
                TransitionEffect::Perform { name, .. } => (
                    DeclarationKind::PerformActionUsage,
                    self.intern_declaration_name(document, *name)?,
                ),
                TransitionEffect::Accept { .. } => (DeclarationKind::AcceptActionUsage, None),
                TransitionEffect::Send { .. } => (DeclarationKind::SendActionUsage, None),
                TransitionEffect::Assign { .. } => (DeclarationKind::Assign, None),
                TransitionEffect::Expression(_) => (DeclarationKind::ActionUsage, None),
            };
            let action = self.push_typed_declaration(
                document,
                Some(owner),
                effect_kind,
                name,
                span,
                DeclarationFacts {
                    is_trigger_action: (effect_kind == DeclarationKind::AcceptActionUsage)
                        .then_some(false),
                    ..DeclarationFacts::none()
                },
            )?;
            self.push_role_membership(
                action,
                MembershipKind::Feature,
                Visibility::Default,
                MembershipRole::StateSubaction(kind),
                span,
            )?;
            self.push_unsupported(document, UnsupportedFamily::StateDefinitionMember, span);
            return Ok(());
        }
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            binding_kind,
            None,
            span,
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            span,
        )?;
        if state_action_body_has_content(syntax.body) {
            self.push_unsupported(document, UnsupportedFamily::StateDefinitionMember, span);
        }
        match syntax.action_reference {
            Some(target) => {
                self.push_action_binding_reference(document, declaration, reference_kind, target)
            }
            None => Ok(()),
        }
    }

    /// Lowers the *declaration* form of an `entry`/`do`/`exit` action -- `do action
    /// prepareForMissionPhaseOperations { first start; then action ...; then done; }` (Apollo 11
    /// `Purpose/MissionPhasesPackage.sysml`; spec42#100 form 4), or `entry action entryAction :>>
    /// 'entry';` (Systems Library `States.sysml`; spec42 Gap 43) -- as opposed to the reference
    /// form (`do myAction;`) the callers handle above. The leading token is a `declared_name`, not
    /// a semantic target, so this introduces a genuine new nested action rather than binding an
    /// existing one: it is pushed as the same name-bearing `Entry`/`Do`/`ExitActionBinding`
    /// declaration kind (so its owning membership still carries the `StateSubaction` role), and its
    /// own body -- the SysML `ActionBody` the parser models as a nested `StateDefBody` -- recurses
    /// through `lower_state_def_body` so its `first`/`then`/`then action` flow and any nested
    /// action usages resolve against the action's own scope (where downstream feature chains like
    /// `phase.prepareForMissionPhaseOperations.transferCrewToVehicle` land).
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn lower_state_declared_action(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        kind: DeclarationKind,
        declared_name: DeclarationName,
        type_name: Option<QualifiedReferenceId>,
        redefines: Option<&Node<SubsettingRelationship>>,
        body: &StateDefBody,
        span: Span,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, Some(declared_name))?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            kind,
            name,
            span,
            // `ast::{Entry,Do,Exit}Action` carries no declaration facts of its own beyond the
            // name and the specialization clauses lowered as references below.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            span,
        )?;
        if let Some(type_name) = type_name {
            // `do action doAction : Action :>> 'do';` -- the `: Type` clause types the nested
            // action, mirroring `lower_first_stmt`'s bare-`QualifiedReferenceId` typing branch.
            let type_span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span: type_span,
                import: None,
            })?;
        }
        if let Some(relationship) = redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_state_def_body(document, declaration, body)
    }

    /// Lowers a state def/usage's `then <target>;` initial-state body element (BNF `ThenStmt`,
    /// the bare initial-state marker -- distinct from a full `transition ... then ...;`
    /// construct, which stays out of scope) as an anonymous `DeclarationKind::InitialState`
    /// feature owned by the enclosing state `owner` declaration, mirroring
    /// `lower_state_entry_action`. `ThenStmt.state_reference` is already a structured
    /// `QualifiedReferenceId`, so it always resolves through the same shared lexical lookup.
    pub(crate) fn lower_state_then_stmt(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        node: &Node<ThenStmt>,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::InitialState,
            None,
            node.span,
            // A synthesized scope for the `then <state>` initial-state reference.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span,
        )?;
        self.push_action_binding_reference(
            document,
            declaration,
            ReferenceKind::InitialState,
            node.value.state_reference,
        )
    }

    /// Lowers a state def/usage's `final <name>;`/`final state <name>;` body element (BNF
    /// `FinalState`) as a named `DeclarationKind::FinalState` feature owned by the enclosing state
    /// `owner` declaration, mirroring `lower_state_usage`'s plain named-declaration shape.
    /// `FinalState.state_name` is always a non-empty declared name per the grammar (`final` is
    /// always followed by a mandatory `name`), so this declares a genuine new nested state rather
    /// than referencing an existing one -- unlike `lower_state_then_stmt`'s `InitialState`, there
    /// is no target reference to resolve.
    pub(crate) fn lower_final_state(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        node: &Node<FinalState>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, Some(node.value.state_name))?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::FinalState,
            name,
            node.span,
            // `ast::FinalState` carries only its state name.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span,
        )?;
        Ok(())
    }

    /// Shared helper for `lower_state_entry_action`/`lower_state_do_action`/
    /// `lower_state_exit_action`/`lower_state_then_stmt`: pushes an authored reference of `kind`
    /// sourced at `declaration` for an already-structured `QualifiedReferenceId` target, mirroring
    /// `lower_alias_def`'s reference-push shape.
    pub(crate) fn push_action_binding_reference(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
        kind: ReferenceKind,
        target: QualifiedReferenceId,
    ) -> Result<(), ConstructionError> {
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span;
        self.push_reference(PendingReference {
            source: declaration,
            kind,
            document,
            local: target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    /// Lowers a `transition ...;` body element (BNF `Transition`, `ast::Transition`) found inside
    /// a state def/usage body as an anonymous `DeclarationKind::Transition` feature owned by the
    /// enclosing state `owner` declaration, mirroring `lower_first_stmt`/`lower_state_entry_
    /// action`'s nested-declaration shape so `source`/`target`/`guard`/`accept`/`effect` all
    /// resolve against the state's own scope (where sibling states/actions are declared), not
    /// the state's enclosing scope. Picks up the full construct explicitly deferred by
    /// `4762b875`; see `DeclarationKind::Transition`'s doc comment for the exact sub-piece scope
    /// boundary.
    pub(crate) fn lower_transition(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        node: &Node<Transition>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Transition,
            name,
            node.span,
            // `ast::Transition` carries no modifier, multiplicity, direction, or short name; its
            // source/target/trigger/guard/effect facts are lowered as references.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span,
        )?;
        if let Some(source) = &node.value.source {
            self.lower_transition_end(
                document,
                declaration,
                ReferenceKind::TransitionSource,
                source,
            )?;
        }
        self.lower_transition_end(
            document,
            declaration,
            ReferenceKind::TransitionTarget,
            &node.value.target,
        )?;
        self.lower_transition_succession(document, declaration, node)?;
        self.lower_transition_source_parameter(document, declaration, node.span)?;
        self.lower_transition_link(document, declaration, node.span)?;
        if let Some(guard) = &node.value.guard {
            let guard_expression = self.push_typed_declaration(
                document,
                Some(declaration),
                DeclarationKind::KermlBooleanExpression,
                None,
                guard.span,
                DeclarationFacts {
                    transition_feature_role: Some(TransitionFeatureRole::Guard),
                    ..DeclarationFacts::none()
                },
            )?;
            self.push_membership(
                guard_expression,
                MembershipKind::Feature,
                Visibility::Default,
                guard.span,
            )?;
            self.push_evaluation_fact(
                guard_expression,
                self.constraint_expression_site(document, &guard.value),
            );
            self.lower_constraint_expression(
                document,
                guard_expression,
                UnsupportedFamily::StateDefinitionMember,
                guard,
            )?;
        }
        let trigger_action = node
            .value
            .accept
            .as_ref()
            .map(|accept| {
                self.lower_transition_trigger_action(document, declaration, node.span, accept)
            })
            .transpose()?;
        // `TransitionUsage` authors an `EmptyParameterMember` payload parameter before every
        // `TriggerActionMember`, and the trigger's `AcceptParameterPart` always owns a payload
        // parameter, whatever the trigger form; only the `accept x : T` form names and types them.
        if let Some(trigger_action) = trigger_action {
            let clause = match &node.value.accept {
                Some(TransitionAccept::Payload(clause, _)) => Some(clause),
                _ => None,
            };
            self.lower_transition_payload_chain(
                document,
                declaration,
                trigger_action,
                clause,
                node.span,
            )?;
        }
        match &node.value.accept {
            None => {}
            Some(TransitionAccept::Shorthand(expression, _via)) => {
                self.lower_transition_end(
                    document,
                    declaration,
                    ReferenceKind::TransitionTrigger,
                    expression,
                )?;
            }
            Some(TransitionAccept::TimeTrigger(kind, expression)) => {
                // Mirrors `lower_then_accept`'s `TimeTrigger` arm: the `at`/`when`/`after`
                // trigger (e.g. `accept at vehicle.maintenanceTime`) is its own
                // TriggerInvocationExpression, owned by the transition's trigger action.
                self.lower_trigger_invocation(
                    document,
                    trigger_action.expect("an authored accept always creates its trigger action"),
                    UnsupportedFamily::StateDefinitionMember,
                    *kind,
                    expression,
                    node.span,
                )?;
            }
            Some(TransitionAccept::Payload(_clause, via)) => {
                let trigger_action =
                    trigger_action.expect("an authored accept always creates its trigger action");
                if let Some(via) = via {
                    self.lower_satisfy_operand(
                        document,
                        trigger_action,
                        UnsupportedFamily::StateDefinitionMember,
                        ReferenceKind::AcceptVia,
                        via,
                    )?;
                }
            }
        }
        let effect_action = node
            .value
            .effect
            .as_ref()
            .map(|effect| {
                self.lower_transition_effect_action(document, declaration, node.span, effect)
            })
            .transpose()?;
        match &node.value.effect {
            None => {}
            Some(TransitionEffect::Perform {
                type_name: Some(type_name),
                ..
            }) => {
                self.push_action_binding_reference(
                    document,
                    effect_action.expect("an authored effect always creates its action"),
                    ReferenceKind::TransitionEffect,
                    *type_name,
                )?;
            }
            Some(TransitionEffect::Expression(expression)) => {
                self.lower_transition_end(
                    document,
                    declaration,
                    ReferenceKind::TransitionEffect,
                    expression,
                )?;
            }
            Some(TransitionEffect::Perform {
                type_name: None, ..
            })
            | Some(TransitionEffect::Accept { .. })
            | Some(TransitionEffect::Send { .. })
            | Some(TransitionEffect::Assign { .. }) => {
                self.push_unsupported(
                    document,
                    UnsupportedFamily::StateDefinitionMember,
                    node.span,
                );
            }
        }
        Ok(())
    }

    /// Publishes the SuccessionAsUsage that the transition grammar authors through its implicit
    /// source end and explicit `then` end. TransitionUsage keeps its own derived source/target
    /// references, while this distinct owned member carries the succession endpoints used by the
    /// normative `succession.sourceFeature = source` contract.
    fn lower_transition_succession(
        &mut self,
        document: DocumentIdx,
        transition: DeclarationId,
        node: &Node<Transition>,
    ) -> Result<DeclarationId, ConstructionError> {
        let succession = self.push_typed_declaration(
            document,
            Some(transition),
            DeclarationKind::Succession,
            None,
            node.span,
            DeclarationFacts {
                is_transition_succession: true,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            succession,
            MembershipKind::Owning,
            Visibility::Default,
            node.span,
        )?;
        if let Some(source) = &node.value.source {
            self.lower_succession_end(
                document,
                succession,
                UnsupportedFamily::StateDefinitionMember,
                ReferenceKind::Succession,
                source,
            )?;
        } else {
            self.reserve_reference_ordinal(succession, ReferenceKind::Succession)?;
        }
        self.lower_succession_end(
            document,
            succession,
            UnsupportedFamily::StateDefinitionMember,
            ReferenceKind::Succession,
            &node.value.target,
        )?;
        Ok(succession)
    }

    /// Publishes a TransitionUsage's first input parameter.
    ///
    /// The transition grammar always authors an `EmptyParameterMember` after the source member
    /// (and before the payload parameter of an accepting transition), so every TransitionUsage
    /// owns this anonymous `in` parameter; `checkTransitionUsageSourceBindingConnector` binds it
    /// to the transition's `source`.
    fn lower_transition_source_parameter(
        &mut self,
        document: DocumentIdx,
        transition: DeclarationId,
        span: Span,
    ) -> Result<DeclarationId, ConstructionError> {
        let parameter = self.push_typed_declaration(
            document,
            Some(transition),
            DeclarationKind::ParameterUsage,
            None,
            span,
            DeclarationFacts {
                direction: Some(ParameterDirection::In),
                is_transition_source_parameter: true,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            parameter,
            MembershipKind::Feature,
            Visibility::Default,
            span,
        )?;
        Ok(parameter)
    }

    /// Publishes a TransitionUsage's `transitionLink` feature.
    ///
    /// Every TransitionUsage owns a succession, so `TransitionUsageAdapter.
    /// computeTransitionLinkConnectors` adds this anonymous ReferenceUsage to each one;
    /// `checkTransitionUsageSuccessionBindingConnector` binds it to that succession.
    fn lower_transition_link(
        &mut self,
        document: DocumentIdx,
        transition: DeclarationId,
        span: Span,
    ) -> Result<DeclarationId, ConstructionError> {
        let link = self.push_typed_declaration(
            document,
            Some(transition),
            DeclarationKind::ReferenceUsage,
            None,
            span,
            DeclarationFacts {
                is_transition_link: true,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(link, MembershipKind::Feature, Visibility::Default, span)?;
        Ok(link)
    }

    /// Publishes the `AcceptActionUsage` owned through a transition's typed trigger membership.
    ///
    /// The parser represents this grammar branch as `TransitionAccept` on the transition rather
    /// than a standalone action-usage node, but the OMG metamodel gives it a distinct
    /// `AcceptActionUsage` element. Keeping that distinction at lowering is what makes the exact
    /// `isTriggerAction()` specialization contract consume a canonical fact rather than inspect a
    /// transition's syntax downstream.
    pub(crate) fn lower_transition_trigger_action(
        &mut self,
        document: DocumentIdx,
        transition: DeclarationId,
        span: Span,
        accept: &TransitionAccept,
    ) -> Result<DeclarationId, ConstructionError> {
        let (has_payload, has_receiver) = match accept {
            TransitionAccept::Payload(_, via) => (true, via.is_some()),
            TransitionAccept::Shorthand(_, via) => (false, via.is_some()),
            TransitionAccept::TimeTrigger(_, _) => (false, false),
        };
        let declaration = self.push_typed_declaration(
            document,
            Some(transition),
            DeclarationKind::AcceptActionUsage,
            None,
            span,
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    composite: true,
                    ..DeclarationModifiers::default()
                },
                is_trigger_action: Some(true),
                transition_feature_role: Some(TransitionFeatureRole::Trigger),
                accept_has_payload_argument: Some(has_payload),
                accept_has_receiver_argument: Some(has_receiver),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            span,
        )?;
        Ok(declaration)
    }

    /// Publishes the ActionUsage owned by a transition's effect membership. Detailed lowering for
    /// the individual perform/accept/send/assign forms remains with their existing branches; this
    /// common declaration is the canonical identity consumed by transition-feature derivation and
    /// specialization.
    fn lower_transition_effect_action(
        &mut self,
        document: DocumentIdx,
        transition: DeclarationId,
        span: Span,
        effect: &TransitionEffect,
    ) -> Result<DeclarationId, ConstructionError> {
        let (kind, name) = match effect {
            TransitionEffect::Perform { name, .. } => (
                DeclarationKind::PerformActionUsage,
                self.intern_declaration_name(document, *name)?,
            ),
            TransitionEffect::Accept { .. } => (DeclarationKind::AcceptActionUsage, None),
            TransitionEffect::Send { .. } => (DeclarationKind::SendActionUsage, None),
            TransitionEffect::Assign { .. } => (DeclarationKind::Assign, None),
            TransitionEffect::Expression(_) => (DeclarationKind::ActionUsage, None),
        };
        let action = self.push_typed_declaration(
            document,
            Some(transition),
            kind,
            name,
            span,
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    composite: true,
                    ..DeclarationModifiers::default()
                },
                is_trigger_action: (kind == DeclarationKind::AcceptActionUsage).then_some(false),
                transition_feature_role: Some(TransitionFeatureRole::Effect),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(action, MembershipKind::Feature, Visibility::Default, span)?;
        Ok(action)
    }

    /// Lowers the two distinct parameters every triggered TransitionUsage owns: the trigger
    /// AcceptActionUsage's payload parameter and the transition's second input parameter
    /// (`accept signal : Signal` names and types them; `accept when c`, `accept at t` and
    /// `accept S` leave them anonymous and untyped). Explicit role facts let resolution publish
    /// `subsetsChain(triggerAction, triggerPayloadParameter())` without rediscovering either
    /// endpoint from syntax, names, or child order.
    fn lower_transition_payload_chain(
        &mut self,
        document: DocumentIdx,
        transition: DeclarationId,
        trigger_action: DeclarationId,
        clause: Option<&sysml_v2_parser::ast::PayloadClause>,
        span: Span,
    ) -> Result<(), ConstructionError> {
        let trigger_payload_parameter = self.push_typed_declaration(
            document,
            Some(trigger_action),
            DeclarationKind::ParameterUsage,
            None,
            span,
            DeclarationFacts {
                direction: Some(ParameterDirection::InOut),
                is_trigger_payload_parameter: true,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            trigger_payload_parameter,
            MembershipKind::Feature,
            Visibility::Default,
            span,
        )?;
        if let Some(type_name) = clause.and_then(|clause| clause.type_name) {
            let type_span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
            self.push_reference(PendingReference {
                source: trigger_payload_parameter,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags {
                    direction: Some(ParameterDirection::InOut),
                    ..RelationshipFlags::default()
                },
                span: type_span,
                import: None,
            })?;
        }

        let name = self.intern_declaration_name(document, clause.map(|clause| clause.name))?;
        let transition_payload_parameter = self.push_typed_declaration(
            document,
            Some(transition),
            DeclarationKind::ParameterUsage,
            name,
            span,
            DeclarationFacts {
                direction: Some(ParameterDirection::In),
                is_transition_payload_parameter: true,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            transition_payload_parameter,
            MembershipKind::Feature,
            Visibility::Default,
            span,
        )?;
        Ok(())
    }

    /// Lowers one `Transition` operand (`source`/`target`/shorthand `accept`/`Expression`
    /// effect): its path expression is a structured `Expression` (not a flattened string), so a
    /// simple/qualified name (`Expression::FeatureRef`) resolves as an authored reference of
    /// `kind` through the same shared `DeclarationDomain::Any` lexical lookup as
    /// `lower_succession_end`. Any other expression shape is left as an explicit unsupported-
    /// member diagnostic, mirroring `lower_succession_end`'s scope boundary.
    pub(crate) fn lower_transition_end(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        kind: ReferenceKind,
        node: &Node<Expression>,
    ) -> Result<(), ConstructionError> {
        match &node.value {
            Expression::FeatureRef(target) => {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(*target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span;
                self.push_reference(PendingReference {
                    source: owner,
                    kind,
                    document,
                    local: *target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
            }
            Expression::MemberAccess { .. } => {
                if self
                    .push_member_access_expression(owner, document, node)?
                    .is_none()
                {
                    self.push_unsupported(
                        document,
                        UnsupportedFamily::StateDefinitionMember,
                        node.span,
                    );
                }
            }
            _ => self.push_unsupported(
                document,
                UnsupportedFamily::StateDefinitionMember,
                node.span,
            ),
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `state` feature member (BNF StateUsage), e.g.
    /// `state s;` or `state s : SomeState;`, mirroring `lower_action_usage`. `StateUsage`'s
    /// typing is a structured `TypingRelationship` (like `ActionUsage.typing`), not a bare
    /// `QualifiedReferenceId`. Behavioral clauses (`entry`/`do`/`exit`, transitions,
    /// abstract/reference/individual prefixes) are explicitly out of scope; owned members lower
    /// through the same `lower_state_def_body` as a `state def`'s body (both share
    /// `StateDefBody`/`StateDefBodyElement`).
    pub(crate) fn lower_state_usage(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<ParserStateUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::StateUsage,
            name,
            node.span,
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    individual: node.value.is_individual,
                    derived: node.value.is_derived,
                    reference: node.value.is_reference,
                    parallel: state_body_is_parallel(node.value.body_modifier.as_ref()),
                    ..DeclarationModifiers::default()
                },
                direction: direction_fact(node.value.direction.as_ref()),
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.lower_state_body_modifier(
            document,
            UnsupportedFamily::StateDefinitionMember,
            node.value.body_modifier.as_ref(),
        );
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span,
        )?;
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_state_def_body(document, declaration, &node.value.body)
    }

    /// Lowers an `exhibit` member (BNF `ExhibitStateUsage`) as an ExhibitStateUsage: the
    /// declared `exhibit state name : Type` form with its typed state-usage facts, or the
    /// `exhibit <path>;` form, whose exhibited state is the `OwnedReferenceSubsetting`
    /// alternative (SysML BNF `ExhibitStateUsage`, as `perform <path>;` is for a
    /// PerformActionUsage) and so a `::>` reference-subsetting target --
    /// the reference `validateExhibitStateUsageReference` reads.
    pub(crate) fn lower_exhibit_state(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        unsupported_family: UnsupportedFamily,
        node: &Node<ParserExhibitState>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ExhibitStateUsage,
            name,
            node.span,
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    individual: node.value.is_individual,
                    derived: node.value.is_derived,
                    reference: node.value.is_reference,
                    parallel: state_body_is_parallel(node.value.body_modifier.as_ref()),
                    ..DeclarationModifiers::default()
                },
                direction: direction_fact(node.value.direction.as_ref()),
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.lower_state_body_modifier(
            document,
            unsupported_family,
            node.value.body_modifier.as_ref(),
        );
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span,
        )?;
        if let Some(state) = node.value.state_reference {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(state)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::References,
                document,
                local: state,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_state_def_body(document, declaration, &node.value.body)
    }
}

/// The authored parts of one `entry`/`do`/`exit` member shared by the three typed AST nodes.
struct StateSubactionSyntax<'a> {
    action_reference: Option<QualifiedReferenceId>,
    declared_name: Option<DeclarationName>,
    type_name: Option<QualifiedReferenceId>,
    redefines: Option<&'a Node<SubsettingRelationship>>,
    effect: Option<&'a TransitionEffect>,
    body: &'a StateDefBody,
}

/// True when a state def/usage's `entry`/`do`/`exit` action body (BNF `StateDefBody`, shared by
/// `EntryAction`/`DoAction`/`ExitAction.body`) carries actual owned members, as opposed to a bare
/// `;` terminator or an empty `{ }` -- both of which are legal no-op markers with nothing to
/// represent when the action also has no bound `action_reference` (see
/// `lower_state_entry_action`'s doc comment). Used to distinguish that genuinely-empty case from
/// an inline `entry { <members> }` anonymous action body, which does carry content this typed AST
/// shape has no field for and so stays an explicit unsupported diagnostic.
pub(crate) fn state_action_body_has_content(body: &StateDefBody) -> bool {
    matches!(body, StateDefBody::Brace { elements, .. } if !elements.is_empty())
}

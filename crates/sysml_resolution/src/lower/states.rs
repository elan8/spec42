//! Phase 2 lowering — state machines: state definitions and usages, transitions, entry/do/exit actions.

use crate::evaluate::classify::flatten_member_access_chain;
use crate::lower::facts::direction_fact;
use crate::lower::facts::multiplicity_facts;
use crate::lower::facts::DeclarationFacts;
use crate::lower::facts::DeclarationModifiers;
use crate::lower::facts::PendingReference;
use crate::lower::facts::RelationshipFlags;
use crate::lower::facts::UnsupportedFamily;
use crate::lower::SemanticModelBuilder;
use crate::model::ConstructionError;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::DocumentId;
use crate::model::MembershipKind;
use crate::model::ReferenceKind;
use crate::model::Visibility;
use sysml_v2_parser::ast::{
    DoAction, EntryAction, ExhibitState as ParserExhibitState, ExitAction, Expression, FinalState,
    MembershipKind as ParserMembershipKind, Node, QualifiedReferenceId, Span, StateDef,
    StateDefBody, StateDefBodyElement, StateUsage as ParserStateUsage, ThenStmt, Transition,
    TransitionAccept, TransitionEffect,
};

impl SemanticModelBuilder {
    /// Lowers a `state def` (BNF StateDefinition), mirroring `lower_action_def`: ownership,
    /// membership, an optional `:>` specialization relationship, and owned declarations.
    /// State-machine-specific semantics (entry/do/exit action bindings, transitions, exclusive/
    /// parallel substates, history) are explicitly out of scope; unrecognized body elements fall
    /// through to `unsupported_state_definition_member` via `lower_state_def_body`.
    pub(crate) fn lower_state_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<StateDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::StateDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    individual: node.value.is_individual,
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
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
    /// `SuccessionUsage` variants; the first three dispatch to their existing lowerings here, and
    /// a `succession` member stays on `unsupported_state_definition_member`.
    pub(crate) fn lower_state_def_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &StateDefBody,
    ) -> Result<(), ConstructionError> {
        let StateDefBody::Brace { elements, .. } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                StateDefBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                StateDefBodyElement::PartUsage(node) => {
                    // New upstream member kind: kept visible as unsupported rather than dropped.
                    self.push_unsupported(
                        document,
                        UnsupportedFamily::StateDefinitionMember,
                        node.span.clone(),
                    );
                }
                StateDefBodyElement::ConstraintUsage(node) => {
                    // New upstream member kind: kept visible as unsupported rather than dropped.
                    self.push_unsupported(
                        document,
                        UnsupportedFamily::StateDefinitionMember,
                        node.span.clone(),
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
                StateDefBodyElement::AssertConstraint(node) => self
                    .lower_assert_constraint_member(
                        document,
                        owner,
                        UnsupportedFamily::StateDefinitionMember,
                        node,
                    )?,
                StateDefBodyElement::SuccessionUsage(_)
                | StateDefBodyElement::MetadataKeywordUsage(_) => self.push_unsupported(
                    document,
                    UnsupportedFamily::StateDefinitionMember,
                    element.span.clone(),
                ),
            }
        }
        Ok(())
    }

    /// Lowers a state def/usage's `entry action <path> ...;` body element (BNF `EntryAction`) as
    /// an anonymous `DeclarationKind::EntryActionBinding` feature owned by the enclosing state
    /// `owner` declaration, mirroring `lower_first_stmt`'s nested-declaration shape so the bound
    /// action reference resolves against the state's own scope (where sibling actions are
    /// declared), not the state's enclosing scope. `EntryAction.action_reference` is already a
    /// structured `QualifiedReferenceId` (not a flattened string), so it resolves through the
    /// same shared lexical lookup as `AliasBinding`/`Succession`. A plain `entry` with no bound
    /// action (`action_reference: None`) has no reference to lower: a bare `entry;`/empty `entry
    /// { }` (no owned members) is a legal no-op marker with genuinely nothing to represent, so it
    /// is silently recognized rather than reported (pervasive in the training/validation corpus,
    /// e.g. `24_state_actions.md`'s `entry; then off;`); an inline `entry { <members> }` body with
    /// actual owned content has no representation in this typed AST shape (no field carries it)
    /// and stays an explicit unsupported diagnostic.
    pub(crate) fn lower_state_entry_action(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<EntryAction>,
    ) -> Result<(), ConstructionError> {
        let Some(target) = node.value.action_reference else {
            if state_action_body_has_content(&node.value.body) {
                self.push_unsupported(
                    document,
                    UnsupportedFamily::StateDefinitionMember,
                    node.span.clone(),
                );
            }
            return Ok(());
        };
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::EntryActionBinding,
            None,
            node.span.clone(),
            // A synthesized scope for the state's entry-action binding reference.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.push_action_binding_reference(
            document,
            declaration,
            ReferenceKind::EntryActionBinding,
            target,
        )
    }

    /// Same as `lower_state_entry_action`, for a `do action <path> ...;` body element
    /// (`DoAction.action_reference`).
    pub(crate) fn lower_state_do_action(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<DoAction>,
    ) -> Result<(), ConstructionError> {
        let Some(target) = node.value.action_reference else {
            if state_action_body_has_content(&node.value.body) {
                self.push_unsupported(
                    document,
                    UnsupportedFamily::StateDefinitionMember,
                    node.span.clone(),
                );
            }
            return Ok(());
        };
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::DoActionBinding,
            None,
            node.span.clone(),
            // A synthesized scope for the state's do-action binding reference.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.push_action_binding_reference(
            document,
            declaration,
            ReferenceKind::DoActionBinding,
            target,
        )
    }

    /// Same as `lower_state_entry_action`, for an `exit action <path> ...;` body element
    /// (`ExitAction.action_reference`).
    pub(crate) fn lower_state_exit_action(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<ExitAction>,
    ) -> Result<(), ConstructionError> {
        let Some(target) = node.value.action_reference else {
            if state_action_body_has_content(&node.value.body) {
                self.push_unsupported(
                    document,
                    UnsupportedFamily::StateDefinitionMember,
                    node.span.clone(),
                );
            }
            return Ok(());
        };
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::ExitActionBinding,
            None,
            node.span.clone(),
            // A synthesized scope for the state's exit-action binding reference.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.push_action_binding_reference(
            document,
            declaration,
            ReferenceKind::ExitActionBinding,
            target,
        )
    }

    /// Lowers a state def/usage's `then <target>;` initial-state body element (BNF `ThenStmt`,
    /// the bare initial-state marker -- distinct from a full `transition ... then ...;`
    /// construct, which stays out of scope) as an anonymous `DeclarationKind::InitialState`
    /// feature owned by the enclosing state `owner` declaration, mirroring
    /// `lower_state_entry_action`. `ThenStmt.state_reference` is already a structured
    /// `QualifiedReferenceId`, so it always resolves through the same shared lexical lookup.
    pub(crate) fn lower_state_then_stmt(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<ThenStmt>,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::InitialState,
            None,
            node.span.clone(),
            // A synthesized scope for the `then <state>` initial-state reference.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
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
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<FinalState>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.state_name)?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::FinalState,
            name,
            node.span.clone(),
            // `ast::FinalState` carries only its state name.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        Ok(())
    }

    /// Shared helper for `lower_state_entry_action`/`lower_state_do_action`/
    /// `lower_state_exit_action`/`lower_state_then_stmt`: pushes an authored reference of `kind`
    /// sourced at `declaration` for an already-structured `QualifiedReferenceId` target, mirroring
    /// `lower_alias_def`'s reference-push shape.
    pub(crate) fn push_action_binding_reference(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        kind: ReferenceKind,
        target: QualifiedReferenceId,
    ) -> Result<(), ConstructionError> {
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
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
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<Transition>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Transition,
            name,
            node.span.clone(),
            // `ast::Transition` carries no modifier, multiplicity, direction, or short name; its
            // source/target/trigger/guard/effect facts are lowered as references.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
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
        if let Some(guard) = &node.value.guard {
            self.push_evaluation_fact(
                declaration,
                self.constraint_expression_site(document, &guard.value),
            );
            self.lower_constraint_expression(
                document,
                declaration,
                UnsupportedFamily::StateDefinitionMember,
                guard,
            )?;
        }
        if node.value.accept.is_some() {
            self.lower_transition_trigger_action(document, declaration, node.span.clone())?;
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
            Some(TransitionAccept::TimeTrigger(_kind, expression)) => {
                // Mirrors `lower_then_accept`'s `TimeTrigger` arm: the `at`/`when`/`after`
                // trigger expression (e.g. `accept at vehicle.maintenanceTime`) is lowered
                // through the general constraint-expression dispatch (`FeatureRef`/
                // `MemberAccess`/`Invocation`/`Constructor`), the same as a `Transition`'s
                // `guard` clause, rather than `lower_transition_end`'s narrower reference-only
                // dispatch (which `Shorthand` above uses for a bare accepted-signal name). The
                // `TriggerKind` (`at`/`when`/`after`) distinction is not yet represented.
                self.lower_constraint_expression(
                    document,
                    declaration,
                    UnsupportedFamily::StateDefinitionMember,
                    expression,
                )?;
            }
            Some(TransitionAccept::Payload(_, _)) => {
                self.push_unsupported(
                    document,
                    UnsupportedFamily::StateDefinitionMember,
                    node.span.clone(),
                );
            }
        }
        match &node.value.effect {
            None => {}
            Some(TransitionEffect::Perform {
                type_name: Some(type_name),
                ..
            }) => {
                self.push_action_binding_reference(
                    document,
                    declaration,
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
                    node.span.clone(),
                );
            }
        }
        Ok(())
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
        document: DocumentId,
        transition: DeclarationId,
        span: Span,
    ) -> Result<DeclarationId, ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(transition),
            DeclarationKind::AcceptActionUsage,
            None,
            span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    composite: true,
                    ..DeclarationModifiers::default()
                },
                is_trigger_action: Some(true),
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

    /// Lowers one `Transition` operand (`source`/`target`/shorthand `accept`/`Expression`
    /// effect): its path expression is a structured `Expression` (not a flattened string), so a
    /// simple/qualified name (`Expression::FeatureRef`) resolves as an authored reference of
    /// `kind` through the same shared `DeclarationDomain::Any` lexical lookup as
    /// `lower_succession_end`. Any other expression shape is left as an explicit unsupported-
    /// member diagnostic, mirroring `lower_succession_end`'s scope boundary.
    pub(crate) fn lower_transition_end(
        &mut self,
        document: DocumentId,
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
                    .span
                    .clone();
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
                if let Some(chain) = flatten_member_access_chain(node) {
                    self.push_member_access_reference(owner, document, &chain, node.span.clone())?;
                } else {
                    self.push_unsupported(
                        document,
                        UnsupportedFamily::StateDefinitionMember,
                        node.span.clone(),
                    );
                }
            }
            _ => self.push_unsupported(
                document,
                UnsupportedFamily::StateDefinitionMember,
                node.span.clone(),
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
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserStateUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::StateUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    individual: node.value.is_individual,
                    derived: node.value.is_derived,
                    reference: node.value.is_reference,
                    ..DeclarationModifiers::default()
                },
                direction: direction_fact(node.value.direction.as_ref()),
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
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

    /// Lowers the declaration-shaped `exhibit state name : Type` form using the parser's typed
    /// state-usage facts. The reference-only `exhibit qualified::state` form denotes a distinct
    /// exhibit relationship which this publication does not yet own, so it remains explicitly
    /// unsupported rather than being misrepresented as feature typing.
    pub(crate) fn lower_exhibit_state(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        unsupported_family: UnsupportedFamily,
        node: &Node<ParserExhibitState>,
    ) -> Result<(), ConstructionError> {
        if node.value.state_reference.is_some() {
            self.push_unsupported(document, unsupported_family, node.span.clone());
            return Ok(());
        }
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::StateUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    individual: node.value.is_individual,
                    derived: node.value.is_derived,
                    reference: node.value.is_reference,
                    ..DeclarationModifiers::default()
                },
                direction: direction_fact(node.value.direction.as_ref()),
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
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

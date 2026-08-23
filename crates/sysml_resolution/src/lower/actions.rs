//! Phase 2 lowering — behaviour: action definitions and usages, control nodes, flows, performs.

use crate::evaluate::classify::flatten_member_access_chain;
use crate::lower::facts::definition_prefix_modifiers;
use crate::lower::facts::definition_prefix_node_modifiers;
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
use crate::model::NameId;
use crate::model::Visibility;
use std::sync::Arc;
use sysml_v2_parser::ast::{
    ActionBranchBody, ActionDef, ActionDefBody, ActionDefBodyElement,
    ActionUsage as ParserActionUsage, ActionUsageBody, ActionUsageBodyElement, AssignStmt,
    ControlNodeDeclaration, DefinitionBody, DefinitionBodyElement, Expression, FirstMergeBody,
    FirstMergeBodyElement, FirstStmt, FlowDeclaration, FlowDef, FlowUsage, ForLoop,
    GuardedSuccession, IfStmt, MembershipKind as ParserMembershipKind, Node,
    Perform as ParserPerform, PerformActionTarget, PerformBody, PerformBodyElement,
    PerformInOutBinding, SendPayload, Span, TerminateStmt, ThenAction, ThenTarget,
    TransitionAccept,
};

impl SemanticModelBuilder {
    /// Lowers an `action def` (BNF ActionDefinition), mirroring `lower_part_def`: ownership,
    /// membership, an optional `:>` specialization relationship, and owned declarations.
    /// Behavioral/control-flow body elements (parameters, succession, decision/merge/fork/join,
    /// accept/send, perform, assign, loops) are explicitly out of scope; unrecognized body
    /// elements fall through to `unsupported_action_definition_member` via
    /// `lower_action_def_body`.
    pub(crate) fn lower_action_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ActionDef>,
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
            DeclarationKind::ActionDefinition,
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
        self.lower_action_def_body(document, declaration, &node.value.body)
    }

    /// Lowers the `ActionDefBody` shared by `action def` and by an `action` usage's own owned
    /// members (BNF `ActionDefBodyElement`): recognized owned members are nested action usages
    /// and `item` usages (BNF `StructureUsageMember` shape, see `crate::ast::ItemUsage`);
    /// everything else -- in/out parameters, `first`/`then` succession, decision/merge/fork/join,
    /// accept/send, perform, assign, loops -- falls through to
    /// `unsupported_action_definition_member`. This is the genuinely out-of-scope
    /// behavioral/control-flow surface for this slice.
    pub(crate) fn lower_action_def_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &ActionDefBody,
    ) -> Result<(), ConstructionError> {
        let ActionDefBody::Brace { elements, .. } = body else {
            return Ok(());
        };
        for element in elements {
            self.lower_action_def_body_element(document, owner, element)?;
        }
        Ok(())
    }

    /// Lowers one `ActionDefBodyElement`, wherever the grammar puts one: the members of an
    /// `ActionDefBody`, and the single brace-less member an `if` branch may be written as
    /// (`ActionBranchBody::Shorthand`). See `lower_action_def_body`'s doc comment for the per-arm
    /// recognized/unsupported shape.
    pub(crate) fn lower_action_def_body_element(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        element: &Node<ActionDefBodyElement>,
    ) -> Result<(), ConstructionError> {
        match &element.value {
            ActionDefBodyElement::Error(error) => {
                self.push_recovery(document, error.span.clone());
            }
            ActionDefBodyElement::Import(node) => {
                // New upstream member kind: kept visible as unsupported rather than dropped.
                self.push_unsupported(
                    document,
                    UnsupportedFamily::ActionDefinitionMember,
                    node.span.clone(),
                );
            }
            ActionDefBodyElement::VariantUsage(node) => {
                // New upstream member kind: kept visible as unsupported rather than dropped.
                self.push_unsupported(
                    document,
                    UnsupportedFamily::ActionDefinitionMember,
                    node.span.clone(),
                );
            }
            ActionDefBodyElement::ActionUsage(action_usage) => {
                self.lower_action_usage(document, Some(owner), action_usage)?;
            }
            ActionDefBodyElement::ItemUsage(item_usage) => {
                self.lower_item_usage(document, Some(owner), item_usage)?;
            }
            ActionDefBodyElement::MetadataUsage(metadata_usage) => {
                self.lower_metadata_usage(document, Some(owner), metadata_usage)?;
            }
            ActionDefBodyElement::StateUsage(state_usage) => {
                self.lower_state_usage(document, Some(owner), state_usage)?;
            }
            ActionDefBodyElement::OccurrenceUsage(occurrence_usage) => {
                self.lower_occurrence_usage(document, Some(owner), occurrence_usage)?;
            }
            ActionDefBodyElement::PartUsage(part_usage) => {
                self.lower_part_usage(document, Some(owner), part_usage)?;
            }
            ActionDefBodyElement::GuardedSuccession(guarded) => {
                self.lower_guarded_succession(
                    document,
                    owner,
                    UnsupportedFamily::ActionDefinitionMember,
                    guarded,
                )?;
            }
            ActionDefBodyElement::FirstStmt(first_stmt) => {
                self.lower_first_stmt(
                    document,
                    owner,
                    UnsupportedFamily::ActionDefinitionMember,
                    first_stmt,
                )?;
            }
            ActionDefBodyElement::InOutDecl(param) => {
                self.lower_parameter_declaration(
                    document,
                    Some(owner),
                    UnsupportedFamily::ActionDefinitionMember,
                    param,
                )?;
            }
            ActionDefBodyElement::Perform(perform) => {
                self.lower_perform(document, Some(owner), perform)?;
            }
            ActionDefBodyElement::Annotating(member) => {
                self.lower_annotating_member(
                    document,
                    Some(owner),
                    UnsupportedFamily::ActionDefinitionMember,
                    member,
                )?;
            }
            ActionDefBodyElement::Bind(node) => {
                self.lower_bind(
                    document,
                    owner,
                    UnsupportedFamily::ActionDefinitionMember,
                    node,
                )?;
            }
            ActionDefBodyElement::AssertConstraint(node) => self.lower_assert_constraint_member(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                node,
            )?,
            ActionDefBodyElement::RefDecl(node) => {
                self.lower_ref_decl(document, Some(owner), node)?;
            }
            ActionDefBodyElement::MergeStmt(node) => self.lower_first_merge_stmt(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                DeclarationKind::Merge,
                node.span.clone(),
                &node.value.declaration,
                &node.value.body,
            )?,
            ActionDefBodyElement::DecisionStmt(node) => self.lower_first_merge_stmt(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                DeclarationKind::Decide,
                node.span.clone(),
                &node.value.declaration,
                &node.value.body,
            )?,
            ActionDefBodyElement::JoinStmt(node) => self.lower_first_merge_stmt(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                DeclarationKind::Join,
                node.span.clone(),
                &node.value.declaration,
                &node.value.body,
            )?,
            ActionDefBodyElement::ForkStmt(node) => self.lower_first_merge_stmt(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                DeclarationKind::Fork,
                node.span.clone(),
                &node.value.declaration,
                &node.value.body,
            )?,
            ActionDefBodyElement::ThenAction(node) => {
                self.lower_then_action(
                    document,
                    owner,
                    UnsupportedFamily::ActionDefinitionMember,
                    node,
                )?;
            }
            ActionDefBodyElement::FlowUsage(node) => self.lower_flow_usage(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                node,
            )?,
            ActionDefBodyElement::TerminateStmt(node) => self.lower_terminate_stmt(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                node,
            )?,
            ActionDefBodyElement::DefaultReferenceUsage(node) => {
                self.lower_default_reference_usage(
                    document,
                    Some(owner),
                    UnsupportedFamily::ActionDefinitionMember,
                    node,
                )?;
            }
            ActionDefBodyElement::WhileStmt(node) => self.lower_while_or_loop_stmt(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                DeclarationKind::While,
                node.span.clone(),
                Some(&node.value.condition),
                &node.value.body.body,
            )?,
            ActionDefBodyElement::LoopStmt(node) => self.lower_while_or_loop_stmt(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                DeclarationKind::Loop,
                node.span.clone(),
                None,
                &node.value.body.body,
            )?,
            ActionDefBodyElement::IfStmt(node) => self.lower_if_stmt(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                node.span.clone(),
                &node.value,
            )?,
            ActionDefBodyElement::Assign(node) => self.lower_assign_stmt(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                node.span.clone(),
                &node.value,
            )?,
            ActionDefBodyElement::ForLoop(node) => self.lower_for_loop(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                node.span.clone(),
                &node.value,
            )?,
            ActionDefBodyElement::AttributeUsage(node) => {
                self.lower_attribute_usage(document, Some(owner), node)?;
            }
            ActionDefBodyElement::CalcUsage(node) => {
                self.lower_calc_usage(document, Some(owner), node)?;
            }
            ActionDefBodyElement::ActionDef(node) => {
                self.lower_action_def(document, Some(owner), node)?;
            }
            ActionDefBodyElement::Dependency(node) => {
                self.lower_dependency(document, Some(owner), node)?;
            }
            ActionDefBodyElement::MetadataKeywordUsage(_) => self.push_unsupported(
                document,
                UnsupportedFamily::ActionDefinitionMember,
                element.span.clone(),
            ),
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `action` feature member (BNF ActionUsage), e.g.
    /// `action validateRoute;` or `action a : SomeAction;`, mirroring `lower_part_usage`.
    /// `ActionUsage`'s typing is a structured `TypingRelationship` (like `PartUsage.typing`), not
    /// a bare `QualifiedReferenceId`. Its typed `accept` suffix is retained as an
    /// `AcceptActionUsage` metaclass and its payload/via facts lower below; owned members lower
    /// through the same `lower_action_def_body` as an `action def`'s body (BNF `ActionUsageBody`
    /// is a structurally near-identical production, differing only in the extra `VariantUsage`
    /// alternative, which is itself out of scope and so folds into the same unsupported family).
    pub(crate) fn lower_action_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserActionUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let short_name = self.intern_short_name(node.value.short_name.as_ref())?;
        let is_accept_action = node.value.accept.is_some();
        let declaration = self.push_typed_declaration(
            document,
            owner,
            if is_accept_action {
                DeclarationKind::AcceptActionUsage
            } else {
                DeclarationKind::ActionUsage
            },
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    variation: node.value.is_variation,
                    individual: node.value.is_individual,
                    reference: node.value.is_reference,
                    // ActionUsage has no standalone `composite` token: its BNF's `ref action`
                    // branch is the non-composite alternative, so the parser's explicit
                    // reference fact is the authoritative source for the derived composite
                    // state needed by the normative `isComposite` predicates.
                    composite: !node.value.is_reference,
                    ..DeclarationModifiers::default()
                },
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                is_trigger_action: is_accept_action.then_some(false),
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
        self.lower_accept_send_clauses(document, declaration, node)?;
        // An action usage is one of the two constructs whose body may not be written at all --
        // `action a accept M via v;` ends at the statement after it -- so an absent body is a
        // distinct upstream state from `;`, and neither owns members.
        match &node.value.body {
            Some(body) => self.lower_action_usage_body(document, declaration, body),
            None => Ok(()),
        }
    }

    /// Lowers the accept/send-suffix facts an `ActionUsage` may carry (BNF `AcceptParameterPart`/
    /// `SenderReceiverPart`, GH-86): a standalone control-node statement's typed `accept name :
    /// Type` payload (`ActionUsage.accept`), a `send`-suffixed usage's optional payload
    /// (`ActionUsage.send`, either a typed-name clause like `accept`'s or a general expression
    /// e.g. `send new Publish(...)`), and the optional trailing `via <port>`/`to <target>` clauses
    /// shared by both forms. Only the payload TYPE reference (`AcceptPayloadType`) and the via/to
    /// operand references (`AcceptVia`/`SendTarget`) are resolved; the payload's own declared NAME
    /// is not a reference target (mirrors `InOutDecl`'s own scope boundary -- the name introduces a
    /// binding, it does not reference one). Sourced directly at `declaration` (the `ActionUsage`'s
    /// own declaration), not an anonymous nested one: unlike `Bind`/`Allocate`, each `ActionUsage`
    /// already has its own unique declaration to source these facts at.
    pub(crate) fn lower_accept_send_clauses(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        node: &Node<ParserActionUsage>,
    ) -> Result<(), ConstructionError> {
        let family = UnsupportedFamily::ActionUsageMember;
        if let Some(accept) = &node.value.accept {
            self.lower_accept_trigger(document, declaration, family, accept)?;
        }
        if let Some(send) = &node.value.send {
            match send {
                SendPayload::Typed(clause) => {
                    self.lower_payload_clause_type(document, declaration, clause)?;
                }
                SendPayload::Expression(expr) => {
                    self.lower_constraint_expression(document, declaration, family, expr)?;
                }
            }
        }
        if let Some(via) = &node.value.via {
            self.lower_satisfy_operand(
                document,
                declaration,
                family,
                ReferenceKind::AcceptVia,
                via,
            )?;
        }
        if let Some(to) = &node.value.to {
            self.lower_satisfy_operand(
                document,
                declaration,
                family,
                ReferenceKind::SendTarget,
                to,
            )?;
        }
        Ok(())
    }

    /// Resolves a `PayloadClause`'s optional `: Type` suffix (`accept name : Type`/`send name :
    /// Type`) as an `AcceptPayloadType` reference, resolved through the same
    /// Subclassification/FeatureTyping `DeclarationDomain::Type` lexical lookup fixed point as
    /// `FeatureTyping` -- the payload names a type, exactly like an ordinary parameter's type
    /// annotation.
    pub(crate) fn lower_payload_clause_type(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        clause: &sysml_v2_parser::ast::PayloadClause,
    ) -> Result<(), ConstructionError> {
        let Some(type_name) = clause.type_name else {
            return Ok(());
        };
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(type_name)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
        self.push_reference(PendingReference {
            source: declaration,
            kind: ReferenceKind::AcceptPayloadType,
            document,
            local: type_name,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    /// Lowers the `ActionUsageBody` owned by an `action` usage (BNF `ActionUsageBodyElement`):
    /// see `lower_action_def_body` for the shared recognized/unsupported shape. The one
    /// additional alternative here, `VariantUsage`, wraps the same `ast::VariantUsage` node
    /// `lower_variant_usage` already lowers for `PartUsageBodyElement::VariantUsage`/
    /// `PerformBodyElement::Variant`, so it dispatches there rather than staying unconditionally
    /// unsupported. Delegates each element to `lower_action_usage_body_element`, which
    /// `PerformBodyElement::Action` (BNF `PerformBodyElement`, the identical body-element shape an
    /// anonymous `perform action { ... }` owns) also calls directly, since a `perform action`'s
    /// own body is typed `ActionUsageBodyElement` too, not a distinct enum.
    pub(crate) fn lower_action_usage_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &ActionUsageBody,
    ) -> Result<(), ConstructionError> {
        let ActionUsageBody::Brace { elements, .. } = body else {
            return Ok(());
        };
        for element in elements {
            self.lower_action_usage_body_element(document, owner, element)?;
        }
        Ok(())
    }

    /// Lowers one `ActionUsageBodyElement` (see `lower_action_usage_body`'s doc comment for why
    /// this is a standalone per-element function rather than inlined into that loop).
    pub(crate) fn lower_action_usage_body_element(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        element: &Node<ActionUsageBodyElement>,
    ) -> Result<(), ConstructionError> {
        match &element.value {
            ActionUsageBodyElement::Error(error) => {
                self.push_recovery(document, error.span.clone());
            }
            ActionUsageBodyElement::Import(node) => {
                // New upstream member kind: kept visible as unsupported rather than dropped.
                self.push_unsupported(
                    document,
                    UnsupportedFamily::ActionUsageMember,
                    node.span.clone(),
                );
            }
            ActionUsageBodyElement::ActionUsage(action_usage) => {
                self.lower_action_usage(document, Some(owner), action_usage)?;
            }
            ActionUsageBodyElement::ItemUsage(item_usage) => {
                self.lower_item_usage(document, Some(owner), item_usage)?;
            }
            ActionUsageBodyElement::MetadataUsage(metadata_usage) => {
                self.lower_metadata_usage(document, Some(owner), metadata_usage)?;
            }
            ActionUsageBodyElement::StateUsage(state_usage) => {
                self.lower_state_usage(document, Some(owner), state_usage)?;
            }
            ActionUsageBodyElement::OccurrenceUsage(occurrence_usage) => {
                self.lower_occurrence_usage(document, Some(owner), occurrence_usage)?;
            }
            ActionUsageBodyElement::PartUsage(part_usage) => {
                self.lower_part_usage(document, Some(owner), part_usage)?;
            }
            ActionUsageBodyElement::GuardedSuccession(guarded) => {
                self.lower_guarded_succession(
                    document,
                    owner,
                    UnsupportedFamily::ActionUsageMember,
                    guarded,
                )?;
            }
            ActionUsageBodyElement::FirstStmt(first_stmt) => {
                self.lower_first_stmt(
                    document,
                    owner,
                    UnsupportedFamily::ActionUsageMember,
                    first_stmt,
                )?;
            }
            ActionUsageBodyElement::InOutDecl(param) => {
                self.lower_parameter_declaration(
                    document,
                    Some(owner),
                    UnsupportedFamily::ActionUsageMember,
                    param,
                )?;
            }
            ActionUsageBodyElement::Annotating(member) => {
                self.lower_annotating_member(
                    document,
                    Some(owner),
                    UnsupportedFamily::ActionUsageMember,
                    member,
                )?;
            }
            ActionUsageBodyElement::Bind(node) => {
                self.lower_bind(document, owner, UnsupportedFamily::ActionUsageMember, node)?;
            }
            ActionUsageBodyElement::AssertConstraint(node) => self.lower_assert_constraint_member(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                node,
            )?,
            ActionUsageBodyElement::RefDecl(node) => {
                self.lower_ref_decl(document, Some(owner), node)?;
            }
            ActionUsageBodyElement::MergeStmt(node) => self.lower_first_merge_stmt(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                DeclarationKind::Merge,
                node.span.clone(),
                &node.value.declaration,
                &node.value.body,
            )?,
            ActionUsageBodyElement::DecisionStmt(node) => self.lower_first_merge_stmt(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                DeclarationKind::Decide,
                node.span.clone(),
                &node.value.declaration,
                &node.value.body,
            )?,
            ActionUsageBodyElement::JoinStmt(node) => self.lower_first_merge_stmt(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                DeclarationKind::Join,
                node.span.clone(),
                &node.value.declaration,
                &node.value.body,
            )?,
            ActionUsageBodyElement::ForkStmt(node) => self.lower_first_merge_stmt(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                DeclarationKind::Fork,
                node.span.clone(),
                &node.value.declaration,
                &node.value.body,
            )?,
            ActionUsageBodyElement::ThenAction(node) => {
                self.lower_then_action(
                    document,
                    owner,
                    UnsupportedFamily::ActionUsageMember,
                    node,
                )?;
            }
            ActionUsageBodyElement::FlowUsage(node) => {
                self.lower_flow_usage(document, owner, UnsupportedFamily::ActionUsageMember, node)?
            }
            ActionUsageBodyElement::TerminateStmt(node) => self.lower_terminate_stmt(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                node,
            )?,
            ActionUsageBodyElement::DefaultReferenceUsage(node) => {
                self.lower_default_reference_usage(
                    document,
                    Some(owner),
                    UnsupportedFamily::ActionUsageMember,
                    node,
                )?;
            }
            ActionUsageBodyElement::WhileStmt(node) => self.lower_while_or_loop_stmt(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                DeclarationKind::While,
                node.span.clone(),
                Some(&node.value.condition),
                &node.value.body.body,
            )?,
            ActionUsageBodyElement::LoopStmt(node) => self.lower_while_or_loop_stmt(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                DeclarationKind::Loop,
                node.span.clone(),
                None,
                &node.value.body.body,
            )?,
            ActionUsageBodyElement::IfStmt(node) => self.lower_if_stmt(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                node.span.clone(),
                &node.value,
            )?,
            ActionUsageBodyElement::Assign(node) => self.lower_assign_stmt(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                node.span.clone(),
                &node.value,
            )?,
            ActionUsageBodyElement::ForLoop(node) => self.lower_for_loop(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                node.span.clone(),
                &node.value,
            )?,
            ActionUsageBodyElement::VariantUsage(node) => {
                self.lower_variant_usage(
                    document,
                    owner,
                    UnsupportedFamily::ActionUsageMember,
                    node,
                )?;
            }
            ActionUsageBodyElement::AttributeUsage(node) => {
                self.lower_attribute_usage(document, Some(owner), node)?;
            }
            ActionUsageBodyElement::CalcUsage(node) => {
                self.lower_calc_usage(document, Some(owner), node)?;
            }
            ActionUsageBodyElement::ActionDef(node) => {
                self.lower_action_def(document, Some(owner), node)?;
            }
            ActionUsageBodyElement::Dependency(node) => {
                self.lower_dependency(document, Some(owner), node)?;
            }
            ActionUsageBodyElement::MetadataKeywordUsage(_) => self.push_unsupported(
                document,
                UnsupportedFamily::ActionUsageMember,
                element.span.clone(),
            ),
        }
        Ok(())
    }

    /// Lowers a `first X then Y;` control-flow succession statement (BNF `FirstStmt`) found
    /// inside an action def/usage body as its own anonymous `DeclarationKind::Succession`
    /// feature owned by the enclosing action def/usage `owner` declaration, mirroring
    /// `lower_end_decl`'s nested-declaration shape: both ends are lowered as authored
    /// `Succession` references sourced at this new anonymous declaration (not at `owner`
    /// directly), so lexical lookup starts in `owner`'s own scope -- where `X`/`Y` are actually
    /// declared as sibling actions -- rather than `owner`'s enclosing scope. The `first` end is
    /// always lowered; the `then` end is `None` for the standalone initial-node marker
    /// `first start;` (§6 G13), which is left as-is (no reference to lower). The named/typed
    /// `succession` keyword prefix and any braced body content are out of scope.
    pub(crate) fn lower_first_stmt(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<FirstStmt>,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Succession,
            None,
            node.span.clone(),
            DeclarationFacts {
                // The succession feature's own multiplicity (`succession [n] first ... then ...`).
                // The per-end `first_multiplicity`/`then_multiplicity` belong to the ends, which
                // are lowered as references rather than declarations, so they are not facts here.
                multiplicity: multiplicity_facts(node.value.succession_multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_succession_end(
            document,
            declaration,
            family,
            ReferenceKind::Succession,
            &node.value.first,
        )?;
        if let Some(then) = &node.value.then {
            self.lower_succession_end(
                document,
                declaration,
                family,
                ReferenceKind::Succession,
                then,
            )?;
        }
        Ok(())
    }

    /// Lowers an action-only `GuardedSuccession` body element (`('succession' UsageDeclaration)?
    /// 'first' <feature-chain> 'if' <guard> 'then' <connector-end>`), mirroring `lower_first_stmt`:
    /// an anonymous `DeclarationKind::Succession` feature owned by `owner`, whose `first` source
    /// and `then` target are lowered as `ReferenceKind::Succession` references (both are
    /// grammar-owned member productions upstream, so neither is an expression), and whose guard
    /// is lowered through the same constraint-expression dispatch a `Transition`'s guard uses.
    /// The optional `succession` declaration contributes the succession feature's own
    /// multiplicity; its own body is not lowered, matching `lower_first_stmt`.
    pub(crate) fn lower_guarded_succession(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<GuardedSuccession>,
    ) -> Result<(), ConstructionError> {
        let succession = node.value.succession.as_ref();
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Succession,
            None,
            node.span.clone(),
            DeclarationFacts {
                multiplicity: multiplicity_facts(
                    succession.and_then(|decl| decl.declaration.value.multiplicity.as_ref()),
                ),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(node.value.first)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
        self.push_reference(PendingReference {
            source: declaration,
            kind: ReferenceKind::Succession,
            document,
            local: node.value.first,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        self.lower_constraint_expression(document, owner, family, &node.value.guard)?;
        self.lower_kerml_connector_end(
            document,
            declaration,
            ReferenceKind::Succession,
            &node.value.target,
        )
    }

    /// Lowers one succession end (the `first` or `then` operand of a `FirstStmt`, or -- reused
    /// verbatim by `AssignTarget`/`Decide`/`Merge`/`Fork`/`Join` -- any other paired/single-operand
    /// control-flow reference): its path expression is a structured `Expression` (not a flattened
    /// string), so a simple/qualified name (`Expression::FeatureRef`) resolves as an authored
    /// reference through the same shared lexical lookup as `ConnectorEnd`. A dotted feature-chain
    /// path -- either nested `Expression::MemberAccess` nodes or a single `Expression::
    /// FeatureChainRef` (the shape the parser actually produces for a dotted path with no
    /// intervening non-name segments, e.g. `assign a.b := ...;`'s target, mirroring
    /// `lower_satisfy_operand`'s identical `MemberAccess`/`FeatureChainRef` pairing) -- resolves as
    /// a `MemberAccessOperand` reference through `flatten_member_access_chain`/
    /// `push_member_access_reference`. The bare `start`/`done` pseudo-action markers parse as an
    /// ordinary `FeatureRef` that legitimately fails to resolve because no such declaration is
    /// synthesized; any other expression shape is left as an explicit unsupported-member
    /// diagnostic.
    pub(crate) fn lower_succession_end(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
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
            Expression::MemberAccess { .. } | Expression::FeatureChainRef(_) => {
                if let Some(chain) = flatten_member_access_chain(node) {
                    self.push_member_access_reference(owner, document, &chain, node.span.clone())?;
                } else {
                    self.push_unsupported(document, family, node.span.clone());
                }
            }
            _ => self.push_unsupported(document, family, node.span.clone()),
        }
        Ok(())
    }

    /// Lowers a `decide`/`merge`/`fork`/`join` control node (BNF `DecisionStmt`/`MergeStmt`/
    /// `ForkStmt`/`JoinStmt`, which all share the identical `<keyword> <expr> <FirstMergeBody>`
    /// shape) as its own anonymous nested-declaration feature owned by `owner`, mirroring
    /// `lower_first_stmt`'s `Succession` shape: the required operand expression is lowered as a
    /// `kind` reference through `lower_succession_end`'s exact `FeatureRef`/`MemberAccess`
    /// dispatch, and a braced body's members recurse through `lower_first_merge_body`.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn lower_first_merge_stmt(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        decl_kind: DeclarationKind,
        span: Span,
        control_declaration: &ControlNodeDeclaration,
        body: &FirstMergeBody,
    ) -> Result<(), ConstructionError> {
        // `ControlNodeDeclaration` is the node's own declaration, not a reference to another
        // element: `merge continue;` declares a MergeNode named `continue`, while `merge;`
        // declares an anonymous one. An unsupported declaration surface stays visible as an
        // unsupported member rather than being lowered as though it named something.
        let name = match control_declaration {
            ControlNodeDeclaration::Anonymous => None,
            ControlNodeDeclaration::Named(expression) => {
                match self.control_node_declared_name(document, expression)? {
                    Some(name) => Some(name),
                    None => {
                        self.push_unsupported(document, family, expression.span.clone());
                        None
                    }
                }
            }
        };
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            decl_kind,
            name,
            span.clone(),
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            span,
        )?;
        self.lower_first_merge_body(document, declaration, family, body)
    }

    /// The declared name of a control node, when its declaration is the simple identifier the
    /// SysML control-node surface admits. A qualified or computed expression is not a declaration
    /// this lowering can honor, and returns `None` so the caller can report it explicitly.
    pub(crate) fn control_node_declared_name(
        &mut self,
        document: DocumentId,
        expression: &Node<Expression>,
    ) -> Result<Option<NameId>, ConstructionError> {
        let Expression::FeatureRef(target) = &expression.value else {
            return Ok(None);
        };
        let parsed = Arc::clone(&self.documents[document.index()].parsed);
        let reference = parsed
            .qualified_reference(*target)
            .ok_or(ConstructionError::InvalidParserReference)?;
        if reference.segments.len() != 1 || reference.metadata.is_absolute {
            return Ok(None);
        }
        let decoded = reference
            .segment_decoded_text(0)
            .ok_or(ConstructionError::InvalidParserReference)?;
        self.intern_declared_name(decoded.as_ref())
    }

    /// Lowers a `decide`/`merge`/`fork`/`join` node's optional braced body (BNF
    /// `FirstMergeBody::Brace`): each retained member is an ordinary `ActionDefBodyElement`, so
    /// the common nested shapes actually authored -- `in`/`out` parameter declarations (a fork's
    /// output flows, e.g. `fork F { in a; out b1; out b2; }`), nested action usages, and further
    /// `then <target>;` continuations -- recurse through the same lowering functions an ordinary
    /// action def/usage body uses. Anything else falls through to the existing
    /// unsupported-member diagnostic, unchanged in kind from prior behavior.
    pub(crate) fn lower_first_merge_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        body: &FirstMergeBody,
    ) -> Result<(), ConstructionError> {
        let FirstMergeBody::Brace { elements, .. } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                FirstMergeBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                FirstMergeBodyElement::Unsupported(_) => {
                    self.push_unsupported(document, family, element.span.clone());
                }
                FirstMergeBodyElement::Member(member) => match &member.value {
                    ActionDefBodyElement::ActionUsage(action_usage) => {
                        self.lower_action_usage(document, Some(owner), action_usage)?;
                    }
                    ActionDefBodyElement::InOutDecl(param) => {
                        self.lower_parameter_declaration(document, Some(owner), family, param)?;
                    }
                    ActionDefBodyElement::ThenAction(then_action) => {
                        self.lower_then_action(document, owner, family, then_action)?;
                    }
                    ActionDefBodyElement::Annotating(member) => {
                        self.lower_annotating_member(document, Some(owner), family, member)?;
                    }
                    ActionDefBodyElement::MergeStmt(node) => self.lower_first_merge_stmt(
                        document,
                        owner,
                        family,
                        DeclarationKind::Merge,
                        node.span.clone(),
                        &node.value.declaration,
                        &node.value.body,
                    )?,
                    ActionDefBodyElement::DecisionStmt(node) => self.lower_first_merge_stmt(
                        document,
                        owner,
                        family,
                        DeclarationKind::Decide,
                        node.span.clone(),
                        &node.value.declaration,
                        &node.value.body,
                    )?,
                    ActionDefBodyElement::JoinStmt(node) => self.lower_first_merge_stmt(
                        document,
                        owner,
                        family,
                        DeclarationKind::Join,
                        node.span.clone(),
                        &node.value.declaration,
                        &node.value.body,
                    )?,
                    ActionDefBodyElement::ForkStmt(node) => self.lower_first_merge_stmt(
                        document,
                        owner,
                        family,
                        DeclarationKind::Fork,
                        node.span.clone(),
                        &node.value.declaration,
                        &node.value.body,
                    )?,
                    _ => self.push_unsupported(document, family, element.span.clone()),
                },
            }
        }
        Ok(())
    }

    /// Lowers a `then <target>;` continuation statement (BNF `ThenAction`) found either as a
    /// direct action def/usage body element or nested inside a `decide`/`merge`/`fork`/`join`
    /// node's braced body: dispatches on `ThenTarget` to whichever existing lowering function
    /// already handles that target's own AST shape (an inline `action`/`perform` declaration, a
    /// nested `merge`/`fork`/`decide` control node, or a bare feature reference to an
    /// already-declared sibling node), so no new resolution machinery is written here. The
    /// `Accept` shorthand-trigger target is deliberately out of scope for this slice (mirroring
    /// `Transition`'s own `TransitionAccept::Payload`/`TimeTrigger` deferral) and falls through to
    /// the existing unsupported-member diagnostic.
    pub(crate) fn lower_then_action(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<ThenAction>,
    ) -> Result<(), ConstructionError> {
        match &node.value.target {
            // `then action ...;` and `then send ... to ...;` are the same `ActionUsage` shape
            // upstream -- the send form is the one carrying `send`/`via`/`to` clauses, which
            // `lower_action_usage` already reads off the usage itself.
            ThenTarget::Action(action_usage) | ThenTarget::Send(action_usage) => {
                self.lower_action_usage(document, Some(owner), action_usage)?;
            }
            ThenTarget::Perform(perform) => {
                self.lower_perform(document, Some(owner), perform)?;
            }
            ThenTarget::Merge(merge_stmt) => self.lower_first_merge_stmt(
                document,
                owner,
                family,
                DeclarationKind::Merge,
                merge_stmt.span.clone(),
                &merge_stmt.value.declaration,
                &merge_stmt.value.body,
            )?,
            ThenTarget::Fork(fork_stmt) => self.lower_first_merge_stmt(
                document,
                owner,
                family,
                DeclarationKind::Fork,
                fork_stmt.span.clone(),
                &fork_stmt.value.declaration,
                &fork_stmt.value.body,
            )?,
            ThenTarget::Join(join_stmt) => self.lower_first_merge_stmt(
                document,
                owner,
                family,
                DeclarationKind::Join,
                join_stmt.span.clone(),
                &join_stmt.value.declaration,
                &join_stmt.value.body,
            )?,
            ThenTarget::Decide(decision_stmt) => self.lower_first_merge_stmt(
                document,
                owner,
                family,
                DeclarationKind::Decide,
                decision_stmt.span.clone(),
                &decision_stmt.value.declaration,
                &decision_stmt.value.body,
            )?,
            // `then if <condition> { ... }` -- an inline conditional action node, lowered
            // through the same owner the standalone `if` body element uses.
            ThenTarget::If(if_stmt) => self.lower_if_stmt(
                document,
                owner,
                family,
                if_stmt.span.clone(),
                &if_stmt.value,
            )?,
            ThenTarget::Feature(expression) => {
                let declaration = self.push_typed_declaration(
                    document,
                    Some(owner),
                    DeclarationKind::ThenContinuation,
                    None,
                    node.span.clone(),
                    // A synthesized scope for the `then <feature>` continuation target.
                    DeclarationFacts::none(),
                )?;
                self.push_membership(
                    declaration,
                    MembershipKind::Feature,
                    Visibility::Default,
                    node.span.clone(),
                )?;
                self.lower_succession_end(
                    document,
                    declaration,
                    family,
                    ReferenceKind::ThenTarget,
                    expression,
                )?;
            }
            ThenTarget::Accept(accept) => {
                self.lower_then_accept(document, owner, family, accept)?;
            }
        }
        Ok(())
    }

    /// Lowers an `assign <target> := <value>;` reassignment statement (BNF `AssignStmt`, `ast::
    /// AssignStmt`; `is_then` is not modeled as a distinct fact -- both the plain and `then assign
    /// ...;` spellings resolve identically) as its own anonymous `DeclarationKind::Assign` feature
    /// owned by `owner`, mirroring `lower_bind`'s "statement, not a new named usage" shape: `lhs`
    /// is lowered as an `AssignTarget` reference through the exact `lower_succession_end` `FeatureRef`/
    /// `MemberAccess` dispatch every other paired/single-operand control-flow reference kind uses,
    /// and `rhs` is lowered through the shared value-assignment pipeline (`classify_constraint_
    /// expression`/`lower_constraint_expression`), publishing its own evaluation fact exactly like
    /// an attribute default value.
    pub(crate) fn lower_assign_stmt(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        span: Span,
        node: &AssignStmt,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Assign,
            None,
            span.clone(),
            // A synthesized scope for the assignment's target/value references.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            span,
        )?;
        self.lower_succession_end(
            document,
            declaration,
            family,
            ReferenceKind::AssignTarget,
            &node.lhs,
        )?;
        self.push_evaluation_fact(
            declaration,
            self.constraint_expression_site(document, &node.rhs.value),
        );
        self.lower_constraint_expression(document, declaration, family, &node.rhs)
    }

    /// Lowers a `while <condition> { ... }` (BNF `WhileStmt`) or bare `loop { ... }` (BNF
    /// `LoopStmt`, no condition) control node as its own anonymous nested-declaration feature
    /// owned by `owner`, mirroring `lower_first_merge_stmt`'s shape: an optional boolean
    /// `condition` is lowered through the same `classify_expression`/
    /// `lower_constraint_expression` machinery already used for `decide`'s branch guards/
    /// transition guards/filter conditions (a loop condition is a genuine boolean expression, not
    /// a control-node reference, unlike `decide`/`merge`/`fork`/`join`'s own operand), and the
    /// body's nested statements recurse through `lower_action_def_body` -- the same dispatch this
    /// helper is itself reached from, since a nested body is always typed `ActionDefBody`
    /// regardless of whether the enclosing action is a def or a usage.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn lower_while_or_loop_stmt(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        decl_kind: DeclarationKind,
        span: Span,
        condition: Option<&Node<Expression>>,
        body: &ActionDefBody,
    ) -> Result<(), ConstructionError> {
        // A synthesized control-flow scope with no authored declaration syntax of its own.
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            decl_kind,
            None,
            span.clone(),
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            span,
        )?;
        if let Some(condition) = condition {
            self.push_evaluation_fact(
                declaration,
                self.constraint_expression_site(document, &condition.value),
            );
            self.lower_constraint_expression(document, declaration, family, condition)?;
        }
        self.lower_action_def_body(document, declaration, body)
    }

    /// Lowers an `if <condition> { ... } (else { ... })?` control node (BNF `IfStmt`) as its own
    /// anonymous nested-declaration feature owned by `owner`, same condition handling as
    /// `lower_while_or_loop_stmt`. Both `then_body` and `else_body` (when present) recurse through
    /// `lower_action_def_body`, owned by this one `If` declaration -- branch bodies are not
    /// distinguished from one another as separate declaration scopes, mirroring how `decide`'s own
    /// braced body is a single undifferentiated scope.
    pub(crate) fn lower_if_stmt(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        span: Span,
        node: &IfStmt,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::If,
            None,
            span.clone(),
            // A synthesized scope for the conditional's guard and branches. `else_body` is a
            // typed parser distinction, so publish its presence once at this lowering boundary.
            DeclarationFacts {
                has_else_action: Some(node.else_body.is_some()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            span,
        )?;
        self.push_evaluation_fact(
            declaration,
            self.constraint_expression_site(document, &node.condition.value),
        );
        self.lower_constraint_expression(document, declaration, family, &node.condition)?;
        self.lower_action_branch_body(document, declaration, &node.then_body)?;
        if let Some(else_body) = &node.else_body {
            self.lower_action_branch_body(document, declaration, else_body)?;
        }
        Ok(())
    }

    /// Lowers one branch of an `if` control node (`ast::ActionBranchBody`). The grammar offers two
    /// spellings -- a braced action body, or a single member written without braces (`if x then
    /// y;`) -- which are different authored syntax and so different states upstream. They own the
    /// same members either way, so both dispatch through the same walker; the branch keeps no
    /// declaration scope of its own (see `lower_if_stmt`).
    pub(crate) fn lower_action_branch_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &ActionBranchBody,
    ) -> Result<(), ConstructionError> {
        match body {
            ActionBranchBody::Braced(body) => self.lower_action_def_body(document, owner, body),
            ActionBranchBody::Shorthand(element) => {
                self.lower_action_def_body_element(document, owner, element)
            }
        }
    }

    /// Lowers a `for <var> in <range> { ... }` loop control node (BNF `ForLoop`) as its own
    /// anonymous `DeclarationKind::ForLoop` nested-declaration feature owned by `owner`, mirroring
    /// `lower_while_or_loop_stmt`'s shape: the `range` collection expression is lowered through
    /// the same `classify_expression`/`lower_constraint_expression` machinery as
    /// `while`'s condition, sourced at this `ForLoop` declaration (the range is evaluated once per
    /// loop, not once per iteration binding). `var` (a bare, untyped `String` -- the parser
    /// records no type/multiplicity for it) is lowered as a named `DeclarationKind::
    /// ForLoopVariable` feature owned by this same `ForLoop` declaration -- introducing a binding,
    /// not a reference, mirroring `InOutDecl`'s own scope boundary -- so it is a visible sibling
    /// through the shared `DeclarationDomain::Any` lexical lookup the body's own statements use.
    /// The body then recurses through `lower_action_def_body`, owned by this `ForLoop`
    /// declaration. No type inference from `range`'s element type is performed; this is
    /// reference-resolution scope only, not iteration-execution semantics.
    pub(crate) fn lower_for_loop(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        span: Span,
        node: &ForLoop,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::ForLoop,
            None,
            span.clone(),
            // A synthesized scope owning the loop variable and range expression.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            span.clone(),
        )?;
        self.push_evaluation_fact(
            declaration,
            self.constraint_expression_site(document, &node.in_parameter.expression.value),
        );
        self.lower_constraint_expression(
            document,
            declaration,
            family,
            &node.in_parameter.expression,
        )?;
        let var_name = match node.variable.value.identification.name.as_deref() {
            Some(name) => self.intern_declared_name(name)?,
            None => None,
        };
        let var_declaration = self.push_typed_declaration(
            document,
            Some(declaration),
            DeclarationKind::ForLoopVariable,
            var_name,
            span.clone(),
            // `ast::ForLoop::var` is a bare `String`; the parser records no type, multiplicity, or
            // modifier for the loop variable.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            var_declaration,
            MembershipKind::Feature,
            Visibility::Default,
            span,
        )?;
        self.lower_action_def_body(document, declaration, &node.body.body)
    }

    /// Lowers a `then accept ...;` shorthand trigger (BNF `ThenTarget::Accept`, `ast::
    /// TransitionAccept`) -- the inline accept-trigger form reused unchanged from `Transition`'s
    /// own `accept` clause, deliberately deferred by `39bd06fc`. Sourced directly at `owner` (the
    /// enclosing action def/usage declaration), not an anonymous nested declaration: an accept
    /// trigger's operand is looked up in the action's own enclosing scope (e.g. `accept sig after
    /// ...;`'s `sig`, `accept when b.f;`'s `b`), not among the action's own children the way a
    /// `Succession`/`Decide`/`Merge` end is. Dispatches on the three `TransitionAccept` shapes:
    /// `Shorthand`'s expression and `TimeTrigger`'s (`at`/`when`/`after`) expression both reuse
    /// `lower_constraint_expression` directly (picking up its `FeatureRef`/`MemberAccess`/
    /// `Invocation`/`Constructor` dispatch, e.g. `accept at new Time::Iso8601DateTime(...)`'s
    /// constructor callee/argument); `Payload`'s typed `: Type` suffix reuses
    /// `lower_payload_clause_type`. Either shape's optional trailing `via <port>` clause resolves
    /// as an `AcceptVia` reference through `lower_satisfy_operand`.
    pub(crate) fn lower_then_accept(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        accept: &Node<TransitionAccept>,
    ) -> Result<(), ConstructionError> {
        self.lower_accept_trigger(document, owner, family, &accept.value)
    }

    /// The `TransitionAccept` dispatch shared by `then accept ...;` (see `lower_then_accept`) and
    /// an `ActionUsage`'s own `accept` clause, which upstream now types as the same production so
    /// the pin-valid `accept Type via port` shorthand and its `via` target are retained.
    pub(crate) fn lower_accept_trigger(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        accept: &TransitionAccept,
    ) -> Result<(), ConstructionError> {
        match accept {
            TransitionAccept::Shorthand(expr, via) => {
                self.lower_constraint_expression(document, owner, family, expr)?;
                if let Some(via) = via {
                    self.lower_satisfy_operand(
                        document,
                        owner,
                        family,
                        ReferenceKind::AcceptVia,
                        via,
                    )?;
                }
            }
            TransitionAccept::TimeTrigger(_kind, expr) => {
                self.lower_constraint_expression(document, owner, family, expr)?;
            }
            TransitionAccept::Payload(clause, via) => {
                self.lower_payload_clause_type(document, owner, clause)?;
                if let Some(via) = via {
                    self.lower_satisfy_operand(
                        document,
                        owner,
                        family,
                        ReferenceKind::AcceptVia,
                        via,
                    )?;
                }
            }
        }
        Ok(())
    }

    /// Lowers a standalone, unnamed `flow (of <payload>)? <source> to <target>;` body element
    /// (BNF `FlowUsage`'s bare from/to shorthand, `ast::FlowUsage`) found inside an action
    /// def/usage body. Mirrors `lower_allocate`/`lower_bind`: an anonymous `DeclarationKind::Flow`
    /// feature owned by `owner`, with `from`/`to` lowered as authored `FlowSource`/`FlowTarget`
    /// references through `lower_kerml_connector_end` -- upstream types both ends as
    /// `KermlConnectorEnd`, the same connector-end shape the KerML connector, binding and
    /// succession members carry, so each end's `target` resolves through the shared
    /// `DeclarationDomain::Any` lexical lookup directly -- and the optional `of
    /// <payload>` clause's type resolved as a `FlowPayloadType` reference (mirroring
    /// `AcceptPayloadType`). A `: Type` clause on the flow itself (`type_name`) is a structurally
    /// distinct declaration form and stays unsupported, as does a *named* flow
    /// (`node.value.name.is_some()`, e.g. `flow generateToAmplify from a to b;`). The upstream
    /// misparse that made a name untrustworthy -- the canonical `flow from <a> to <b>;` shorthand
    /// consuming its own `from` keyword as the declared name -- is fixed, so an authored name is
    /// now a real one; lowering the named form is pending (planning/UPSTREAM_PARSER_GAPS.md,
    /// "Typed upstream, not yet lowered here").
    pub(crate) fn lower_flow_usage(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<FlowUsage>,
    ) -> Result<(), ConstructionError> {
        // Upstream now models `FlowDeclaration`'s two grammar alternatives: the endpoint-only
        // shorthand this lowering supports, and the declaration-led form, whose named/typed
        // shapes stay unsupported exactly as before.
        let (payload, endpoints) = match &node.value.declaration {
            FlowDeclaration::EndpointOnly { endpoints } => (None, endpoints),
            FlowDeclaration::Declared {
                declaration,
                payload,
                endpoints,
                ..
            } => {
                let declared_label = declaration.value.identification.name.is_some()
                    || declaration.value.identification.short_name.is_some();
                let Some(endpoints) = endpoints.as_ref() else {
                    self.push_unsupported(document, family, node.span.clone());
                    return Ok(());
                };
                if declared_label || declaration.value.typing.is_some() {
                    self.push_unsupported(document, family, node.span.clone());
                    return Ok(());
                }
                (payload.as_ref(), endpoints)
            }
        };
        let (from, to) = (&endpoints.from, &endpoints.to);
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Flow,
            None,
            node.span.clone(),
            // `ast::FlowUsage` carries no modifier, multiplicity, direction, or short name; its
            // payload/from/to facts are lowered as references.
            DeclarationFacts {
                owned_end_feature_count: Some(2),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let Some(payload) = payload {
            if let Some(type_name) = payload.value.type_name {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(type_name)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span
                    .clone();
                self.push_reference(PendingReference {
                    source: declaration,
                    kind: ReferenceKind::FlowPayloadType,
                    document,
                    local: type_name,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
            }
        }
        self.lower_kerml_connector_end(document, declaration, ReferenceKind::FlowSource, from)?;
        self.lower_kerml_connector_end(document, declaration, ReferenceKind::FlowTarget, to)?;
        Ok(())
    }

    /// Lowers a `terminate <target>;`/bare `terminate;` body element (BNF `TerminateStmt`, `ast::
    /// TerminateStmt`) found inside an action def/usage body. The optional `target` is resolved as
    /// a `TerminateTarget` reference through the shared `lower_satisfy_operand`
    /// `DeclarationDomain::Any` lexical lookup, sourced directly at `owner` (the enclosing action
    /// def/usage declaration) -- unlike `Succession`/`Decide`, no anonymous nested-declaration
    /// scope shift is needed because the terminated node/action is looked up in the terminate
    /// statement's own enclosing scope, where sibling action names like `terminate c1;`'s `c1` are
    /// actually declared. The bare `terminate;` form (no target) has nothing to resolve and is a
    /// legitimate no-op, not an unsupported construct.
    pub(crate) fn lower_terminate_stmt(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<TerminateStmt>,
    ) -> Result<(), ConstructionError> {
        if let Some(target) = &node.value.target {
            self.lower_satisfy_operand(
                document,
                owner,
                family,
                ReferenceKind::TerminateTarget,
                target,
            )?;
        }
        Ok(())
    }

    /// Lowers an explicit `perform action <name> : <Type>;` performance usage (BNF `Perform`)
    /// found in a part def/usage or action def/usage body, mirroring `lower_action_usage`'s
    /// shape: ownership, membership, an optional `FeatureTyping`/`Subclassification` reference to
    /// the performed action type, and `subsets`/`redefines` specialization. Only nested `part`/
    /// `item` usages inside the perform's own body are lowered; the shorthand `perform <path>;`
    /// reference form (no declaration label) and other body content are out of scope.
    pub(crate) fn lower_perform(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserPerform>,
    ) -> Result<(), ConstructionError> {
        let declared = match &node.value.target {
            PerformActionTarget::Action(declaration) => Some(declaration.as_ref()),
            PerformActionTarget::Reference { .. } => None,
        };
        let name_text = declared
            .and_then(|declaration| {
                declaration
                    .value
                    .identification
                    .name
                    .clone()
                    .or_else(|| declaration.value.identification.short_name.clone())
            })
            .unwrap_or_default();
        let name = self.intern_declared_name(&name_text)?;
        let (is_abstract, variation) =
            definition_prefix_modifiers(node.value.usage_prefix.as_ref());
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::PerformActionUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    is_abstract,
                    variation,
                    ..DeclarationModifiers::default()
                },
                multiplicity: multiplicity_facts(
                    declared.and_then(|decl| decl.value.multiplicity.as_ref()),
                ),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        // Records the authored value spelling (`=`/`:=`/`default`) for this declaration. The
        // value expression itself is not lowered here -- expression coverage for this usage
        // family is unchanged by this fact family.
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
        }
        match &node.value.target {
            PerformActionTarget::Action(usage_declaration) => {
                if let Some(relationship) = &usage_declaration.value.typing {
                    self.lower_typing_relationship(document, declaration, relationship)?;
                }
                if let Some((relationship, _)) = &usage_declaration.value.subsets {
                    self.lower_subsetting_relationship(document, declaration, relationship)?;
                }
                if let Some(relationship) = &usage_declaration.value.redefines {
                    self.lower_subsetting_relationship(document, declaration, relationship)?;
                }
            }
            PerformActionTarget::Reference { redefines, .. } => {
                if let Some(relationship) = redefines {
                    self.lower_subsetting_relationship(document, declaration, relationship)?;
                }
            }
        }
        self.lower_perform_body(document, declaration, &node.value.body)
    }

    /// Lowers the `PerformBody` owned by a `perform action` usage (BNF `PerformBodyElement`):
    /// `part`/`item`/`attribute` usages, `in`/`out` parameter-argument bindings, nested
    /// action-body content (an anonymous `perform action { ... }`'s own body, typed
    /// `ActionUsageBodyElement`, delegated to `lower_action_usage_body_element` rather than
    /// duplicating the control-flow dispatch here), and `variant` members (delegated to
    /// `lower_variant_usage`, the same function `PartUsageBodyElement::VariantUsage` uses) are all
    /// recognized.
    pub(crate) fn lower_perform_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &PerformBody,
    ) -> Result<(), ConstructionError> {
        let PerformBody::Brace { elements, .. } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                PerformBodyElement::PartUsage(part_usage) => {
                    self.lower_part_usage(document, Some(owner), part_usage)?;
                }
                PerformBodyElement::ItemUsage(item_usage) => {
                    self.lower_item_usage(document, Some(owner), item_usage)?;
                }
                PerformBodyElement::AttributeUsage(attribute) => {
                    self.lower_attribute_usage(document, Some(owner), attribute)?;
                }
                PerformBodyElement::Annotating(member) => {
                    self.lower_annotating_member(
                        document,
                        Some(owner),
                        UnsupportedFamily::ActionUsageMember,
                        member,
                    )?;
                }
                PerformBodyElement::Action(element) => {
                    self.lower_action_usage_body_element(document, owner, element)?;
                }
                PerformBodyElement::InOut(node) => {
                    self.lower_perform_inout_binding(document, owner, node)?;
                }
                PerformBodyElement::Variant(node) => {
                    self.lower_variant_usage(
                        document,
                        owner,
                        UnsupportedFamily::ActionUsageMember,
                        node,
                    )?;
                }
            }
        }
        Ok(())
    }

    /// Lowers a `perform` usage's `in`/`out <target> = <value>;` parameter-argument binding (BNF
    /// `PerformInOutBinding`, `ast::structure::PerformInOutBinding`, found only inside
    /// `PerformBody`) as its own anonymous `DeclarationKind::PerformParameterBinding` feature
    /// owned by `owner` (the enclosing `perform`'s own declaration), mirroring `lower_bind`'s
    /// shape: `target` is an authored reference to the invoked action's own declared parameter
    /// (not a new declared name, unlike `InOutDecl`'s own `in`/`out` shorthand), resolved directly
    /// as a `PerformParameterTarget` reference since it is already a structured
    /// `QualifiedReferenceId`; `value` is lowered through the ordinary
    /// `lower_constraint_expression` machinery, exactly like `Assign`'s RHS. `direction`
    /// (`in`/`out`/`inout`) is out of scope, matching `Bind`'s own "binding keyword prefix
    /// ignored" precedent.
    pub(crate) fn lower_perform_inout_binding(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<PerformInOutBinding>,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::PerformParameterBinding,
            None,
            node.span.clone(),
            DeclarationFacts {
                direction: direction_fact(Some(&node.value.direction)),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(node.value.target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
        self.push_reference(PendingReference {
            source: declaration,
            kind: ReferenceKind::PerformParameterTarget,
            document,
            local: node.value.target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        self.push_evaluation_fact(
            declaration,
            self.constraint_expression_site(document, &node.value.value.value),
        );
        self.lower_constraint_expression(
            document,
            declaration,
            UnsupportedFamily::ActionUsageMember,
            &node.value.value,
        )
    }

    /// Lowers a `flow def` (BNF FlowDefinition), mirroring `lower_allocation_def`/
    /// `lower_occurrence_def`: ownership, membership, an optional `:>` specialization
    /// relationship, and owned attribute/part/item/nested-occurrence declarations plus `end`
    /// connector-end structure via the shared `lower_occurrence_body_element` walker
    /// (`FlowDef.body` is the same `DefinitionBody`/`OccurrenceBodyElement` shape
    /// `OccurrenceDef.body`/`AllocationDef.body` use). Flow-payload (`ref :>> payload : Type;`)
    /// and succession-flow semantics are explicitly out of scope here -- see
    /// `DeclarationKind::FlowDefinition`'s doc comment.
    pub(crate) fn lower_flow_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<FlowDef>,
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
        let (is_abstract, variation) =
            definition_prefix_node_modifiers(node.value.definition_prefix.as_ref());
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::FlowDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract,
                    variation,
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
        let DefinitionBody::Brace { elements, .. } = &node.value.body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                DefinitionBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                DefinitionBodyElement::OccurrenceMember(member) => {
                    self.lower_occurrence_body_element(document, declaration, member)?;
                }
                DefinitionBodyElement::Unsupported(node) => self.push_unsupported(
                    document,
                    UnsupportedFamily::ParserUnsupported,
                    node.span.clone(),
                ),
            }
        }
        Ok(())
    }
}

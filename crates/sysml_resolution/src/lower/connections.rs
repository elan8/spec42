//! Phase 2 lowering — connectivity: ports, connections, interfaces, bindings, allocations.

use crate::evaluate::classify::flatten_member_access_chain;
use crate::lower::facts::definition_prefix_node_modifiers;
use crate::lower::facts::direction_node_fact;
use crate::lower::facts::multiplicity_facts;
use crate::lower::facts::occurrence_prefix_modifiers;
use crate::lower::facts::DeclarationFacts;
use crate::lower::facts::DeclarationModifiers;
use crate::lower::facts::PendingReference;
use crate::lower::facts::RelationshipFlags;
use crate::lower::facts::UnsupportedFamily;
use crate::model::ConstructionError;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::DocumentId;
use crate::model::MembershipKind;
use crate::model::ReferenceKind;
use crate::model::SemanticModelBuilder;
use crate::model::Visibility;
use sysml_v2_parser::ast::{
    Allocate, AllocationDef, AllocationUsage as ParserAllocationUsage, Bind, BindingConnectorUsage,
    ConnectStmt, ConnectionDef, ConnectionDefBody, ConnectionDefBodyElement, ConnectionEnd,
    ConnectionUsageMember as ParserConnectionUsage, DefinitionBody, DefinitionBodyElement, EndDecl,
    EndIdentity, Expression, InterfaceDef, InterfaceDefBody, InterfaceDefBodyElement, InterfaceEnd,
    InterfaceEndTarget, InterfacePart, InterfaceUsage as ParserInterfaceUsage,
    InterfaceUsageBodyElement, MembershipKind as ParserMembershipKind, Node, PortBody,
    PortBodyElement, PortDef, PortDefBody, PortDefBodyElement, PortUsage as ParserPortUsage,
    QualifiedReferenceId,
};

impl SemanticModelBuilder {
    /// Lowers a `port def` (BNF PortDefinition), mirroring `lower_part_def`:
    /// membership, an optional `:>` specialization relationship (participates in the shared
    /// Subclassification/FeatureTyping lexical lookup fixed point, see `DeclarationDomain::Type`
    /// in resolver.rs), and owned attribute/enum/nested-port members. Port-specific semantics
    /// (interface/flow binding, port conformance, connector-end validation) are explicitly out of
    /// scope; unrecognized body elements fall through to `unsupported_port_definition_member`.
    pub(crate) fn lower_port_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<PortDef>,
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
            DeclarationKind::PortDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
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
        if let PortDefBody::Brace { elements, .. } = &node.value.body {
            for element in elements {
                match &element.value {
                    PortDefBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    PortDefBodyElement::VariantUsage(node) => {
                        // New upstream member kind: kept visible as unsupported rather than dropped.
                        self.push_unsupported(
                            document,
                            UnsupportedFamily::PortDefinitionMember,
                            node.span.clone(),
                        );
                    }
                    PortDefBodyElement::AttributeDef(attribute) => {
                        self.lower_attribute_def(document, Some(declaration), attribute)?;
                    }
                    PortDefBodyElement::AttributeUsage(attribute) => {
                        self.lower_attribute_usage(document, Some(declaration), attribute)?;
                    }
                    PortDefBodyElement::EnumerationUsage(enum_usage) => {
                        self.lower_enum_usage(document, Some(declaration), enum_usage)?;
                    }
                    PortDefBodyElement::PortUsage(port_usage) => {
                        self.lower_port_usage(document, Some(declaration), port_usage)?;
                    }
                    PortDefBodyElement::ItemDef(item_def) => {
                        self.lower_item_def(document, Some(declaration), item_def)?;
                    }
                    PortDefBodyElement::ItemUsage(item_usage) => {
                        self.lower_item_usage(document, Some(declaration), item_usage)?;
                    }
                    PortDefBodyElement::Annotating(member) => {
                        self.lower_annotating_member(
                            document,
                            Some(declaration),
                            UnsupportedFamily::PortDefinitionMember,
                            member,
                        )?;
                    }
                    PortDefBodyElement::RefDecl(ref_decl) => {
                        self.lower_ref_decl(document, Some(declaration), ref_decl)?;
                    }
                    PortDefBodyElement::InOutDecl(param) => {
                        self.lower_parameter_declaration(
                            document,
                            Some(declaration),
                            UnsupportedFamily::PortDefinitionMember,
                            param,
                        )?;
                    }
                    PortDefBodyElement::Unsupported(node) => self.push_unsupported(
                        document,
                        UnsupportedFamily::ParserUnsupported,
                        node.span.clone(),
                    ),
                    PortDefBodyElement::MetadataKeywordUsage(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::PortDefinitionMember,
                        element.span.clone(),
                    ),
                }
            }
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `port` feature member (BNF PortUsage), mirroring
    /// `lower_part_usage`: ownership, membership, an optional `:`/`:>` typing/subclassification
    /// relationship (whose target may be conjugated, e.g. `port source : ~InputPort;` -- the
    /// polarity is carried as an explicit `RelationshipFlags::conjugated` fact via
    /// `lower_typing_relationship`, never folded into the reference target), `subsets`/
    /// `redefines`/`references`/`crosses`/`intersects` subsetting relationships, and owned
    /// attribute/nested-port members.
    pub(crate) fn lower_port_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserPortUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let short_name = self.intern_short_name(node.value.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::PortUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    ordered: node.value.multiplicity_modifiers.is_ordered(),
                    nonunique: !node.value.multiplicity_modifiers.is_unique(),
                    ..occurrence_prefix_modifiers(&node.value.prefix)
                },
                direction: direction_node_fact(
                    node.value.prefix.basic.ref_prefix.direction.as_ref(),
                ),
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
        // Records the authored value spelling (`=`/`:=`/`default`) for this declaration. The
        // value expression itself is not lowered here -- expression coverage for this usage
        // family is unchanged by this fact family.
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
        }
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let Some((relationship, _)) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.references {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.crosses {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.intersects {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let PortBody::Brace { elements, .. } = &node.value.body {
            for element in elements {
                match &element.value {
                    PortBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    PortBodyElement::OccurrenceUsage(node) => {
                        // New upstream member kind: kept visible as unsupported rather than dropped.
                        self.push_unsupported(
                            document,
                            UnsupportedFamily::PortUsageMember,
                            node.span.clone(),
                        );
                    }
                    PortBodyElement::VariantUsage(node) => {
                        // New upstream member kind: kept visible as unsupported rather than dropped.
                        self.push_unsupported(
                            document,
                            UnsupportedFamily::PortUsageMember,
                            node.span.clone(),
                        );
                    }
                    PortBodyElement::AttributeUsage(attribute) => {
                        self.lower_attribute_usage(document, Some(declaration), attribute)?;
                    }
                    PortBodyElement::PortUsage(port_usage) => {
                        self.lower_port_usage(document, Some(declaration), port_usage)?;
                    }
                    PortBodyElement::RefDecl(ref_decl) => {
                        self.lower_ref_decl(document, Some(declaration), ref_decl)?;
                    }
                    PortBodyElement::ItemUsage(item_usage) => {
                        self.lower_item_usage(document, Some(declaration), item_usage)?;
                    }
                    PortBodyElement::Annotating(member) => {
                        self.lower_annotating_member(
                            document,
                            Some(declaration),
                            UnsupportedFamily::PortDefinitionMember,
                            member,
                        )?;
                    }
                    PortBodyElement::InOutDecl(param) => {
                        self.lower_parameter_declaration(
                            document,
                            Some(declaration),
                            UnsupportedFamily::PortDefinitionMember,
                            param,
                        )?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Lowers a `connection def` (BNF ConnectionDefinition), mirroring `lower_port_def`:
    /// ownership, membership, an optional `:>` specialization relationship (participates in the
    /// shared Subclassification/FeatureTyping lexical lookup fixed point, see
    /// `DeclarationDomain::Type` in resolver.rs), and owned attribute/item/port/nested-connection
    /// members plus connector-end structure via `lower_connection_body`. Connector-end
    /// referential/multiplicity validation is explicitly out of scope; unrecognized body elements
    /// fall through to `unsupported_connection_definition_member`.
    pub(crate) fn lower_connection_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ConnectionDef>,
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
            DeclarationKind::ConnectionDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract,
                    variation,
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
        self.lower_connection_body(document, declaration, &node.value.body)
    }

    /// Lowers a package/definition/usage-level `connection` feature member (BNF ConnectionUsage),
    /// mirroring `lower_metadata_usage`: ownership, membership, an optional `:` typing reference
    /// (a bare `QualifiedReferenceId`, not a structured `TypingRelationship`),
    /// `subsets`/`redefines` subsetting relationships, an optional inline `connect from to to`
    /// clause (connector-end references), and owned attribute/item/port/nested-connection
    /// members via the same shared `lower_connection_body` as `connection def`.
    pub(crate) fn lower_connection_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserConnectionUsage>,
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
            owner,
            DeclarationKind::ConnectionUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
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
        if let Some(type_reference) = node.value.type_reference {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_reference)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_reference,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(end) = &node.value.connect_from {
            self.lower_connector_end(document, declaration, end)?;
        }
        if let Some(end) = &node.value.connect_to {
            self.lower_connector_end(document, declaration, end)?;
        }
        for end in &node.value.connect_extra_ends {
            self.lower_connector_end(document, declaration, end)?;
        }
        self.lower_connection_body(document, declaration, &node.value.body)
    }

    /// Shared body walker for `connection def`/`connection` usage bodies (both use
    /// `ConnectionDefBody`/`ConnectionDefBodyElement` -- there is no separate
    /// `ConnectionUsageBody`), mirroring `lower_state_def_body`'s single-walker pattern. `end`
    /// declarations and `connect` statements carry the connector-end structure; everything else
    /// beyond attribute/item/port/nested-part-usage members falls through to
    /// `unsupported_connection_definition_member`.
    pub(crate) fn lower_connection_body(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        body: &ConnectionDefBody,
    ) -> Result<(), ConstructionError> {
        if let ConnectionDefBody::Brace { elements, .. } = body {
            for element in elements {
                match &element.value {
                    ConnectionDefBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    ConnectionDefBodyElement::MetadataKeywordUsage(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::ConnectionDefinitionMember,
                        element.span.clone(),
                    ),
                    ConnectionDefBodyElement::Annotating(member) => {
                        self.lower_annotating_member(
                            document,
                            Some(declaration),
                            UnsupportedFamily::ConnectionDefinitionMember,
                            member,
                        )?;
                    }
                    ConnectionDefBodyElement::EndDecl(end_decl) => {
                        self.lower_end_decl(document, declaration, end_decl)?;
                    }
                    ConnectionDefBodyElement::ConnectStmt(connect_stmt) => {
                        self.lower_connect_stmt(document, declaration, connect_stmt)?;
                    }
                    ConnectionDefBodyElement::AttributeDef(attribute) => {
                        self.lower_attribute_def(document, Some(declaration), attribute)?;
                    }
                    ConnectionDefBodyElement::AttributeUsage(attribute) => {
                        self.lower_attribute_usage(document, Some(declaration), attribute)?;
                    }
                    ConnectionDefBodyElement::ItemDef(item_def) => {
                        self.lower_item_def(document, Some(declaration), item_def)?;
                    }
                    ConnectionDefBodyElement::ItemUsage(item_usage) => {
                        self.lower_item_usage(document, Some(declaration), item_usage)?;
                    }
                    ConnectionDefBodyElement::PortDef(port_def) => {
                        self.lower_port_def(document, Some(declaration), port_def)?;
                    }
                    ConnectionDefBodyElement::PortUsage(port_usage) => {
                        self.lower_port_usage(document, Some(declaration), port_usage)?;
                    }
                    ConnectionDefBodyElement::PartUsage(part_usage) => {
                        self.lower_part_usage(document, Some(declaration), part_usage)?;
                    }
                    ConnectionDefBodyElement::OccurrenceUsage(occurrence_usage) => {
                        self.lower_occurrence_usage(document, Some(declaration), occurrence_usage)?;
                    }
                    ConnectionDefBodyElement::AssertConstraint(node) => self
                        .lower_assert_constraint_member(
                            document,
                            declaration,
                            UnsupportedFamily::ConnectionDefinitionMember,
                            node,
                        )?,
                    ConnectionDefBodyElement::RefDecl(node) => {
                        self.lower_ref_decl(document, Some(declaration), node)?;
                    }
                    ConnectionDefBodyElement::SuccessionUsage(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::ConnectionDefinitionMember,
                        element.span.clone(),
                    ),
                }
            }
        }
        Ok(())
    }

    /// Lowers an `end` declaration inside a connection/interface def body (BNF `EndDecl`) as its
    /// own nested declaration: a normal declared label (or an anonymous `#original`/`#derive`
    /// derivation role), an optional `:` typing relationship, and an optional `::>`/`references`
    /// reference-subsetting relationship as an authored `ConnectorEnd` reference (resolved
    /// through the same shared lexical lookup as `AliasBinding`, see `DeclarationDomain::Any` in
    /// resolver.rs). `redefines`/`crosses`/`nested_usage` -- connector-end referential
    /// constraints, not the plain reference shape this slice covers -- are explicitly out of
    /// scope and left unlowered.
    pub(crate) fn lower_end_decl(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<EndDecl>,
    ) -> Result<(), ConstructionError> {
        let name = match &node.value.identity {
            EndIdentity::Declaration(label) => self.intern_declared_name(&label.value)?,
            EndIdentity::Derivation(_) => None,
        };
        let positional_end = self.next_positional_end_ordinal(owner)?;
        let short_name = self.intern_short_name(node.value.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::ConnectionUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                positional_end: Some(positional_end),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.references {
            for target in relationship.value.target.iter().copied() {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span
                    .clone();
                self.push_reference(PendingReference {
                    source: declaration,
                    kind: ReferenceKind::ConnectorEnd,
                    document,
                    local: target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
            }
        }
        Ok(())
    }

    /// Lowers an inline `connect from to to (, extra)*` statement (BNF `ConnectStmt`) as
    /// `ConnectorEnd` references from the owning connection def/usage declaration to each end's
    /// target. `ConnectionUsage`'s body is `UsageBody`, so a braced body owns the whole usage
    /// member set; a `connect` statement mints no declaration of its own, so those members have no
    /// element to belong to and stay explicitly unsupported (see the body walk below).
    pub(crate) fn lower_connect_stmt(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<ConnectStmt>,
    ) -> Result<(), ConstructionError> {
        self.lower_connector_end(document, owner, &node.value.from)?;
        self.lower_connector_end(document, owner, &node.value.to)?;
        for end in &node.value.extra_ends {
            self.lower_connector_end(document, owner, end)?;
        }
        // `ConnectionUsage`'s body is `UsageBody`, so a braced `connect a to b { ... }` owns the
        // whole usage member set. A `connect` statement mints no declaration of its own -- its ends
        // are lowered directly against the enclosing `owner` -- so there is nothing for those
        // members to belong to, and attributing them to `owner` would report them as its own. They
        // stay an explicit unsupported member of the enclosing connection scope until the statement
        // form owns a declaration.
        for element in node.value.body.members() {
            self.push_unsupported(
                document,
                UnsupportedFamily::ConnectionDefinitionMember,
                element.span.clone(),
            );
        }
        Ok(())
    }

    /// Lowers one connector end (`ConnectionEnd`, used by both `ConnectStmt` and
    /// `ConnectionUsageMember`'s inline `connect` clause): its path expression is a structured
    /// `Expression` (not a flattened string), so a simple/qualified name (`Expression::FeatureRef`)
    /// resolves as an authored `ConnectorEnd` reference through the same shared lexical lookup as
    /// `AliasBinding`. A dotted feature-chain path (`Expression::MemberAccess`, e.g. `t.bead`)
    /// resolves as a `ReferenceKind::MemberAccessOperand` reference instead (see its doc comment
    /// for the algorithm), through `flatten_member_access_chain`/`push_member_access_reference`.
    /// Any other expression shape is left as an explicit `unsupported_connection_definition_member`
    /// diagnostic rather than a fabricated or partial resolution.
    pub(crate) fn lower_connector_end(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<ConnectionEnd>,
    ) -> Result<(), ConstructionError> {
        match &node.value.expression.value {
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
                    kind: ReferenceKind::ConnectorEnd,
                    document,
                    local: *target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
            }
            Expression::MemberAccess { .. } | Expression::FeatureChainRef(_) => {
                if let Some(chain) = flatten_member_access_chain(&node.value.expression) {
                    self.push_member_access_reference(
                        owner,
                        document,
                        &chain,
                        node.value.expression.span.clone(),
                    )?;
                } else {
                    self.push_unsupported(
                        document,
                        UnsupportedFamily::ConnectionDefinitionMember,
                        node.span.clone(),
                    );
                }
            }
            _ => self.push_unsupported(
                document,
                UnsupportedFamily::ConnectionDefinitionMember,
                node.span.clone(),
            ),
        }
        Ok(())
    }

    /// Lowers an `interface def` (BNF InterfaceDefinition), mirroring `lower_connection_def`:
    /// ownership, membership, an optional `:>` specialization relationship (participates in the
    /// shared Subclassification/FeatureTyping lexical lookup fixed point, see
    /// `DeclarationDomain::Type` in resolver.rs), and owned attribute/item/port/flow members plus
    /// connector-end structure via `lower_interface_body`, reusing the same `end`/`connect`
    /// `ReferenceKind::ConnectorEnd` machinery `lower_connection_def` uses (interface ends are
    /// semantically the same kind of fact). `interface` usage lowering is deferred -- see
    /// `DeclarationKind::InterfaceDefinition`'s doc comment and planning/UPSTREAM_PARSER_GAPS.md #6.
    pub(crate) fn lower_interface_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<InterfaceDef>,
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
            DeclarationKind::InterfaceDefinition,
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
        self.lower_interface_body(document, declaration, &node.value.body)
    }

    /// Body walker for `interface def` bodies (`InterfaceDefBody`/`InterfaceDefBodyElement`),
    /// mirroring `lower_connection_body`. `end` declarations and `connect` statements carry the
    /// connector-end structure through the same `lower_end_decl`/`lower_connect_stmt` helpers
    /// `connection def` uses; everything else beyond attribute/item/port members falls through to
    /// `unsupported_interface_definition_member`.
    pub(crate) fn lower_interface_body(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        body: &InterfaceDefBody,
    ) -> Result<(), ConstructionError> {
        if let InterfaceDefBody::Brace { elements, .. } = body {
            for element in elements {
                match &element.value {
                    InterfaceDefBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    InterfaceDefBodyElement::ConstraintUsage(node) => {
                        // New upstream member kind: kept visible as unsupported rather than dropped.
                        self.push_unsupported(
                            document,
                            UnsupportedFamily::InterfaceDefinitionMember,
                            node.span.clone(),
                        );
                    }
                    InterfaceDefBodyElement::MetadataKeywordUsage(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::InterfaceDefinitionMember,
                        element.span.clone(),
                    ),
                    InterfaceDefBodyElement::Annotating(member) => {
                        self.lower_annotating_member(
                            document,
                            Some(declaration),
                            UnsupportedFamily::InterfaceDefinitionMember,
                            member,
                        )?;
                    }
                    InterfaceDefBodyElement::EndDecl(end_decl) => {
                        self.lower_end_decl(document, declaration, end_decl)?;
                    }
                    InterfaceDefBodyElement::ConnectStmt(connect_stmt) => {
                        self.lower_connect_stmt(document, declaration, connect_stmt)?;
                    }
                    InterfaceDefBodyElement::AttributeDef(attribute) => {
                        self.lower_attribute_def(document, Some(declaration), attribute)?;
                    }
                    InterfaceDefBodyElement::AttributeUsage(attribute) => {
                        self.lower_attribute_usage(document, Some(declaration), attribute)?;
                    }
                    InterfaceDefBodyElement::ItemDef(item_def) => {
                        self.lower_item_def(document, Some(declaration), item_def)?;
                    }
                    InterfaceDefBodyElement::ItemUsage(item_usage) => {
                        self.lower_item_usage(document, Some(declaration), item_usage)?;
                    }
                    InterfaceDefBodyElement::PortDef(port_def) => {
                        self.lower_port_def(document, Some(declaration), port_def)?;
                    }
                    InterfaceDefBodyElement::PortUsage(port_usage) => {
                        self.lower_port_usage(document, Some(declaration), port_usage)?;
                    }
                    InterfaceDefBodyElement::RefDecl(node) => {
                        self.lower_ref_decl(document, Some(declaration), node)?;
                    }
                    InterfaceDefBodyElement::FlowUsage(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::InterfaceDefinitionMember,
                        element.span.clone(),
                    ),
                }
            }
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `interface` feature member (BNF InterfaceUsage),
    /// mirroring `lower_connection_usage`: ownership, membership, an optional `:` typing target,
    /// `subsets`/`redefines` subsetting relationships, and connector-end structure (`connect`
    /// endpoints via `lower_interface_connector_expression`, reusing the same
    /// `ReferenceKind::ConnectorEnd` machinery `interface def`/`connection` usage use). Resolved
    /// upstream in `0757de13` (planning/UPSTREAM_PARSER_GAPS.md #6).
    pub(crate) fn lower_interface_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserInterfaceUsage>,
    ) -> Result<(), ConstructionError> {
        let (name, interface_type, subsets, redefines, ends, body) = match &node.value {
            ParserInterfaceUsage::TypedConnect {
                name,
                interface_type,
                subsets,
                redefines,
                part,
                body,
                ..
            } => (
                name.as_deref(),
                interface_type.as_ref(),
                subsets.as_ref(),
                redefines.as_ref(),
                Some(part),
                body,
            ),
            ParserInterfaceUsage::Connection {
                subsets,
                redefines,
                part,
                body,
                ..
            } => (
                None,
                None,
                subsets.as_ref(),
                redefines.as_ref(),
                Some(part),
                body,
            ),
            ParserInterfaceUsage::Declaration {
                name,
                interface_type,
                subsets,
                redefines,
                body,
                ..
            } => (
                name.as_deref(),
                interface_type.as_ref(),
                subsets.as_ref(),
                redefines.as_ref(),
                None,
                body,
            ),
        };
        let name = name
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::InterfaceUsage,
            name,
            node.span.clone(),
            // `ast::InterfaceUsage` is an enum of connect/declaration shapes carrying only name,
            // type, subsets/redefines, and ends -- no modifier, multiplicity, direction, or short
            // name on either variant.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let Some(type_reference) = interface_type {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(*type_reference)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: *type_reference,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(part) = ends {
            self.lower_interface_part(document, declaration, part)?;
        }
        for element in body.members() {
            match &element.value {
                InterfaceUsageBodyElement::Annotating(member) => {
                    self.lower_annotating_member(
                        document,
                        Some(declaration),
                        UnsupportedFamily::InterfaceDefinitionMember,
                        member,
                    )?;
                }
                InterfaceUsageBodyElement::EndDecl(end_decl) => {
                    self.lower_end_decl(document, declaration, end_decl.as_ref())?;
                }
                InterfaceUsageBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                InterfaceUsageBodyElement::FlowUsage(flow) => {
                    self.lower_flow_usage(
                        document,
                        declaration,
                        UnsupportedFamily::InterfaceDefinitionMember,
                        flow.as_ref(),
                    )?;
                }
                InterfaceUsageBodyElement::Perform(perform) => {
                    self.lower_perform(document, Some(declaration), perform.as_ref())?;
                }
                InterfaceUsageBodyElement::RefRedef { .. } => self.push_unsupported(
                    document,
                    UnsupportedFamily::InterfaceDefinitionMember,
                    element.span.clone(),
                ),
            }
        }
        Ok(())
    }

    /// Lowers an `InterfacePart` -- the binary `<from> to <to>` pair or the parenthesized n-ary
    /// end list upstream now models -- as `ConnectorEnd` references, one per authored endpoint.
    pub(crate) fn lower_interface_part(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        part: &Node<InterfacePart>,
    ) -> Result<(), ConstructionError> {
        match &part.value {
            InterfacePart::Binary { from, to, .. } => {
                self.lower_interface_end(document, owner, from)?;
                self.lower_interface_end(document, owner, to)?;
            }
            InterfacePart::Nary { ends, .. } => {
                for member in ends {
                    self.lower_interface_end(document, owner, &member.end)?;
                }
            }
        }
        Ok(())
    }

    /// Lowers one `InterfaceEnd` as a `ConnectorEnd` reference. The production owns a required
    /// reference subsetting, so the endpoint target is a source-backed qualified reference rather
    /// than an expression; an optional declaration label (`left ::> port`) is not itself a
    /// reference and is not lowered here.
    pub(crate) fn lower_interface_end(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        end: &Node<InterfaceEnd>,
    ) -> Result<(), ConstructionError> {
        let target = match &end.value.target {
            InterfaceEndTarget::Direct(target) => *target,
            InterfaceEndTarget::Named { target, .. } => *target,
        };
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
        self.push_reference(PendingReference {
            source: owner,
            kind: ReferenceKind::ConnectorEnd,
            document,
            local: target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    /// Lowers one `from`/`to` interface-connect endpoint expression as a `ConnectorEnd`
    /// reference, mirroring `lower_connector_end` but operating directly on a bare
    /// `Node<Expression>` (rather than the `Node<ConnectionEnd>` wrapper `connection` usage's
    /// `connect_from`/`connect_to` use).
    #[allow(dead_code)]
    pub(crate) fn lower_interface_connector_expression(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
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
                    kind: ReferenceKind::ConnectorEnd,
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
                    self.push_unsupported(
                        document,
                        UnsupportedFamily::InterfaceDefinitionMember,
                        node.span.clone(),
                    );
                }
            }
            _ => self.push_unsupported(
                document,
                UnsupportedFamily::InterfaceDefinitionMember,
                node.span.clone(),
            ),
        }
        Ok(())
    }

    /// Lowers an `allocate <source> to <target>;` body element (BNF `Allocate`, `ast::Allocate`)
    /// found inside a part def/part usage/occurrence body -- the shorthand allocation
    /// *statement* form, which asserts an allocation relationship between two already-declared
    /// elements without introducing a new named allocation usage (genuinely distinct from
    /// `AllocationDefinition`/`AllocationUsage`, the declaration forms lowered in `04274711`).
    /// Mirrors `lower_satisfy`: an anonymous `DeclarationKind::Allocate` feature owned by `owner`,
    /// with `source`/`target` lowered as authored `AllocateSource`/`AllocateTarget` references
    /// when they are a simple/qualified name (`Expression::FeatureRef`), resolved through the
    /// same `DeclarationDomain::Any` lexical lookup fixed point `Satisfy`/`Succession` use.
    /// Unlike a satisfy usage, `Allocate` has no reference/declaration alternative to gate on. Its
    /// body is `UsageBody = DefinitionBody`, the same part-usage member set `Bind`'s body uses, so
    /// members are lowered against the anonymous allocate declaration through the shared
    /// `lower_part_usage_body_element` walker.
    pub(crate) fn lower_allocate(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<Allocate>,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Allocate,
            None,
            node.span.clone(),
            // `ast::Allocate` carries only its source/target ends, lowered as references.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_satisfy_operand(
            document,
            declaration,
            family,
            ReferenceKind::AllocateSource,
            &node.value.source,
        )?;
        self.lower_satisfy_operand(
            document,
            declaration,
            family,
            ReferenceKind::AllocateTarget,
            &node.value.target,
        )?;
        for element in node.value.body.members() {
            self.lower_part_usage_body_element(document, declaration, family, element)?;
        }
        Ok(())
    }

    /// Lowers a `bind <source> = <target>;` body element (BNF `Bind`, `ast::Bind`) found inside a
    /// part def/part usage/action def/action usage body -- the shorthand binding-connector
    /// *statement* form, which asserts a binding-connector relationship between two
    /// already-declared elements without introducing a new named binding-connector usage. Mirrors
    /// `lower_allocate`: an anonymous `DeclarationKind::Bind` feature owned by `owner`, with
    /// `left`/`right` lowered as authored `BindSource`/`BindTarget` references when they are a
    /// simple/qualified name (`Expression::FeatureRef`), resolved through the same
    /// `DeclarationDomain::Any` lexical lookup fixed point `Satisfy`/`Allocate` use (reusing
    /// `lower_satisfy_operand` directly). The optional `binding <name>`/`: Type`/multiplicity
    /// prefix on either end is out of scope. `Bind`'s body (BNF `Bind`'s `UsageBody`) is typed
    /// `PartUsageBody` -- the same part-usage member set
    /// `PartUsageBody` uses (see its own doc comment) -- so each element dispatches through the
    /// shared `lower_part_usage_body_element`, owned by this `Bind`'s own anonymous declaration,
    /// rather than the blanket "every element unsupported" fallback used before that dispatcher
    /// was factored out of `lower_part_usage`.
    pub(crate) fn lower_bind(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<Bind>,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Bind,
            None,
            node.span.clone(),
            // `ast::Bind` carries only its two bound operands, lowered as references.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_satisfy_operand(
            document,
            declaration,
            family,
            ReferenceKind::BindSource,
            &node.value.left,
        )?;
        self.lower_satisfy_operand(
            document,
            declaration,
            family,
            ReferenceKind::BindTarget,
            &node.value.right,
        )?;
        for element in node.value.body.members() {
            self.lower_part_usage_body_element(document, declaration, family, element)?;
        }
        Ok(())
    }

    /// Lowers a package-level `binding ... left = right;` element (BNF `BindingConnectorUsage`,
    /// `ast::BindingConnectorUsage`) -- the keyword-less sibling of `Bind` (see its doc comment),
    /// same binding-connector-statement semantics as `lower_bind` but with `left`/`right` already
    /// structured `QualifiedReferenceId`s rather than `Expression`s, so they resolve directly
    /// through the same `DeclarationDomain::Any` lexical lookup fixed point as `AliasBinding`
    /// (mirroring `lower_alias_def`'s single-reference shape, applied twice). The `all`/name/
    /// multiplicity prefix and any real content in the braced body are out of scope, matching
    /// `Bind`'s own scope boundary.
    pub(crate) fn lower_binding_connector_usage(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<BindingConnectorUsage>,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Bind,
            None,
            node.span.clone(),
            DeclarationFacts {
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_binding_connector_operand(
            document,
            declaration,
            ReferenceKind::BindSource,
            node.value.left,
        )?;
        self.lower_binding_connector_operand(
            document,
            declaration,
            ReferenceKind::BindTarget,
            node.value.right,
        )?;
        for element in node.value.body.members() {
            self.lower_part_usage_body_element(document, declaration, family, element)?;
        }
        Ok(())
    }

    /// Lowers one `BindingConnectorUsage` operand (`left`/`right`), mirroring `lower_alias_def`'s
    /// `AliasDef::target` handling: an already-structured `QualifiedReferenceId` resolves directly
    /// as an authored reference of `kind` through the shared `DeclarationDomain::Any` lexical
    /// lookup, with no expression-shape gating (`BindingConnectorUsage`'s ends are never a general
    /// `Expression`, unlike `Bind`'s).
    pub(crate) fn lower_binding_connector_operand(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
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
            source: owner,
            kind,
            document,
            local: target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    /// Lowers an `allocation def` (BNF AllocationDefinition), mirroring `lower_occurrence_def`:
    /// ownership, membership, an optional `:>` specialization relationship, and owned
    /// attribute/part/item/nested-occurrence declarations plus `end` connector-end structure via
    /// the shared `lower_occurrence_body_element` walker (`AllocationDef.body` is the same
    /// `DefinitionBody`/`OccurrenceBodyElement` shape `OccurrenceDef.body` uses). Allocation-
    /// specific semantics (the `allocate ... to ...` binding itself) are explicitly out of scope
    /// here -- see `DeclarationKind::AllocationDefinition`'s doc comment.
    pub(crate) fn lower_allocation_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<AllocationDef>,
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
            DeclarationKind::AllocationDefinition,
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

    /// Lowers a named `allocation` usage from the parser-owned usage header and connector ends.
    /// This is distinct from the anonymous `allocate source to target` statement lowered by
    /// `lower_allocate`, but publishes the same directional endpoint kinds.
    pub(crate) fn lower_allocation_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserAllocationUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::Allocate,
            Some(name),
            node.span.clone(),
            DeclarationFacts::none(),
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
        if let Some(target) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(target)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: target,
                flags: RelationshipFlags {
                    conjugated: node.value.type_is_conjugated,
                    ..RelationshipFlags::default()
                },
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(source) = &node.value.source {
            self.lower_kerml_connector_end(
                document,
                declaration,
                ReferenceKind::AllocateSource,
                source,
            )?;
        }
        if let Some(target) = &node.value.target {
            self.lower_kerml_connector_end(
                document,
                declaration,
                ReferenceKind::AllocateTarget,
                target,
            )?;
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

    /// Lowers a keyword-less bare `connect <from> to <to> [:> <subsets>] [:>> <redefines>]
    /// { ... }` connector member (BNF `Connect`, `structure.rs` struct `Connect`, distinct from
    /// the `connect ... to ...;` sub-clause of an already-dispatched connector production modeled
    /// by `ConnectStmt`/`lower_connect_stmt`), e.g. a top-level `connect a to b;` package member.
    /// Sourced directly at the enclosing `owner` declaration (no separate declaration is
    /// synthesized), mirroring `lower_connect_stmt`'s anonymous shape: `from`/`to` resolve
    /// through the shared `lower_connector_end` walker, and an optional `:>`/`:>>` `subsets`/
    /// `redefines` clause resolves through the shared `lower_subsetting_relationship` helper.
    pub(crate) fn lower_bare_connect(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<sysml_v2_parser::ast::Connect>,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::BareConnect,
            None,
            node.span.clone(),
            // A synthesized scope giving the bare `connect a to b;` ends a lexical owner.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_connector_end(document, declaration, &node.value.from)?;
        self.lower_connector_end(document, declaration, &node.value.to)?;
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        for element in node.value.body.members() {
            self.lower_part_usage_body_element(document, declaration, family, element)?;
        }
        Ok(())
    }
}

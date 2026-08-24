//! Phase 2 lowering — metadata definitions, usages, and annotations.

use crate::lower::facts::DeclarationFacts;
use crate::lower::facts::DeclarationModifiers;
use crate::lower::facts::PendingReference;
use crate::lower::facts::RelationshipFlags;
use crate::lower::facts::UnsupportedFamily;
use crate::lower::SemanticModelBuilder;
use crate::model::ConstructionError;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::DocumentIdx;
use crate::model::MembershipKind;
use crate::model::ReferenceKind;
use crate::model::Visibility;
use sysml_v2_parser::ast::{
    MembershipKind as ParserMembershipKind, MetadataAnnotation, MetadataBody, MetadataBodyElement,
    MetadataBodyUsage, MetadataDef, MetadataUsage as ParserMetadataUsage, Node,
};

impl SemanticModelBuilder {
    /// Lowers a `metadata def` (BNF MetadataDefinition), mirroring `lower_item_def`: ownership,
    /// membership, and an optional `:>` specialization relationship. `MetadataDef`'s body is a
    /// plain `AttributeBody` (shared with `AttributeDef`/`ItemDef`), so its owned members are
    /// lowered through the existing `lower_attribute_body`.
    pub(crate) fn lower_metadata_def(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<MetadataDef>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.identification.name)?;
        let short_name = self.intern_short_name(document, node.identification.short_name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::MetadataDefinition,
            name,
            node.span,
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
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
            node.value.membership.span,
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_attribute_body(document, declaration, &node.value.body)
    }

    /// Lowers a package/definition/usage-level `metadata` feature member (BNF MetadataUsage),
    /// e.g. `metadata m : SomeMetadata;`, mirroring `lower_item_usage`. `type_reference` is a
    /// bare `QualifiedReferenceId`, so its `FeatureTyping` reference is pushed directly rather
    /// than through `lower_typing_relationship`. `MetadataUsage`'s body is a plain
    /// `AttributeBody` (see `lower_metadata_def`), so owned members are lowered through
    /// `lower_attribute_body`. The `about` clause (annotation targets) is deliberately not
    /// lowered here -- it belongs to the separate annotation-application fact family, out of
    /// scope for this slice.
    pub(crate) fn lower_metadata_usage(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<ParserMetadataUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, Some(node.value.name))?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::MetadataUsage,
            name,
            node.span,
            // `ast::MetadataUsage` carries no modifier, multiplicity, direction, or short name.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span,
        )?;
        if let Some(type_reference) = node.value.type_reference {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_reference)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
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
        self.lower_metadata_body(document, declaration, &node.value.body)
    }

    /// Lowers a `MetadataBody` (`';' | '{' MetadataBodyElement* '}'`), the body shared by
    /// `metadata` usages and `@Name { ... }` annotations. Its members are reference
    /// redefinitions (`MetadataBodyUsage`), not attribute declarations: each one names an
    /// existing feature of the annotated metadata type, optionally binds a value, and may own a
    /// nested metadata body of its own.
    pub(crate) fn lower_metadata_body(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        body: &MetadataBody,
    ) -> Result<(), ConstructionError> {
        let MetadataBody::Brace { elements, .. } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                MetadataBodyElement::Error(error) => {
                    self.push_recovery(document, error.span);
                }
                MetadataBodyElement::Annotating(member) => {
                    self.lower_annotating_member(
                        document,
                        Some(owner),
                        UnsupportedFamily::AttributeMember,
                        member,
                    )?;
                }
                MetadataBodyElement::Usage(usage) => {
                    self.lower_metadata_body_usage(document, owner, usage)?;
                }
                // `MetadataBody`'s `DefinitionMember` alternative shares `metadata def`'s member
                // dispatcher, so a nested declaration lowers exactly as it would in a `metadata
                // def` body (SysML BNF 1677).
                MetadataBodyElement::Definition(element) => {
                    self.lower_attribute_body_element(document, owner, element)?;
                }
                MetadataBodyElement::Alias(alias) => {
                    self.lower_alias_def(document, Some(owner), alias)?;
                }
                MetadataBodyElement::Import(import) => {
                    self.lower_import(document, Some(owner), import)?;
                }
            }
        }
        Ok(())
    }

    /// Lowers one `MetadataBodyUsage`: an anonymous feature owned by `owner` that redefines the
    /// named target (`totalRisk` in `@Risk { totalRisk = 0.3; }`), carries the authored value
    /// spelling, and owns any nested metadata body.
    pub(crate) fn lower_metadata_body_usage(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        node: &Node<MetadataBodyUsage>,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::AttributeUsage,
            None,
            node.span,
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span,
        )?;
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(node.value.target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span;
        self.push_reference(PendingReference {
            source: declaration,
            kind: ReferenceKind::Redefinition,
            document,
            local: node.value.target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        self.lower_attribute_default_value(document, declaration, node.value.value.as_ref())?;
        self.lower_metadata_body(document, declaration, &node.value.body)
    }

    /// Lowers an `@Name{...}`/`@Name;` metadata annotation body element (`ast::MetadataAnnotation`,
    /// see `ReferenceKind::MetadataAnnotation`), applied to `owner` -- the declaration that owns
    /// the body the annotation appears in (a part usage, action def, state def, ...). Only the
    /// annotation-target reference (`type_reference`, the production's required
    /// `OwnedFeatureTyping`, e.g. `Safety`) is resolved, sourced directly at `owner`;
    /// `about_targets` and the nested `body` (feature-value overrides) are out of scope, see the
    /// `ReferenceKind::MetadataAnnotation` doc comment.
    ///
    /// `MetadataFeatureDeclaration`'s optional `Identification ( ':' | 'typed by' )` prefix is a
    /// declared name, not a reference: `@t : Safety;` declares `t` and is typed by `Safety`, and
    /// only the latter is the annotation target. The name is carried onto the annotation's own
    /// declaration below when the annotation body mints one.
    pub(crate) fn lower_metadata_annotation(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        node: &Node<MetadataAnnotation>,
    ) -> Result<(), ConstructionError> {
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(node.value.type_reference)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span;
        self.push_reference(PendingReference {
            source: owner,
            kind: ReferenceKind::MetadataAnnotation,
            document,
            local: node.value.type_reference,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        // Widened value-assignment handling (see `lower_value_assignment`): the annotation body's
        // nested feature-value overrides (`@Safety{isMandatory = true;}`'s `isMandatory = true;`)
        // were deliberately deferred by the annotation-application slice pending exactly this
        // machinery (see the `ReferenceKind::MetadataAnnotation` doc comment). Each override is
        // typed as an ordinary `AttributeUsage` (BNF-shared `AttributeBody`, exactly like `metadata
        // m : Safety { isMandatory = true; }`'s own body), but the `@Safety{...}` annotation form
        // has no named declaration of its own to own them (unlike a named `metadata m : Safety`
        // usage) -- a `MetadataUsage`-kind declaration nested under `owner` gives the overrides a
        // real owning scope without disturbing `owner`'s own member set or the
        // `MetadataAnnotation` reference above (still sourced directly at `owner`, unchanged).
        // It is anonymous unless the author wrote `MetadataFeatureDeclaration`'s optional
        // `Identification` prefix (`@t : Safety { ... }`), whose declared name and short name are
        // the scope's own -- the annotated type is never borrowed as a stand-in for them.
        if matches!(&node.value.body, MetadataBody::Brace { elements, .. } if !elements.is_empty())
        {
            let identification = node
                .value
                .declared_name
                .as_ref()
                .map(|declared| &declared.value.identification);
            let name = self.intern_declaration_name(
                document,
                identification.and_then(|identification| identification.name),
            )?;
            let short_name = self.intern_short_name(
                document,
                identification.and_then(|identification| identification.short_name),
            )?;
            let annotation_scope = self.push_typed_declaration(
                document,
                Some(owner),
                DeclarationKind::MetadataUsage,
                name,
                node.span,
                DeclarationFacts {
                    short_name,
                    ..DeclarationFacts::none()
                },
            )?;
            self.push_membership(
                annotation_scope,
                MembershipKind::Feature,
                Visibility::Default,
                node.span,
            )?;
            self.lower_metadata_body(document, annotation_scope, &node.value.body)?;
        }
        Ok(())
    }
}

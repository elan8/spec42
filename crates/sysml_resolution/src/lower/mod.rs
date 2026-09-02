//! Phase 2: lowering. Authored facts derived from parsed trees, and nothing else.

use crate::lower::facts::definition_prefix_node_modifiers;
use crate::lower::facts::AdmittedDocument;
use crate::lower::facts::AnnotationForm;
use crate::lower::facts::AuthoredExpression;
use crate::lower::facts::AuthoredFilterCondition;
use crate::lower::facts::AuthoredImportFacts;
use crate::lower::facts::AuthoredImportShape;
use crate::lower::facts::AuthoredInvocation;
use crate::lower::facts::AuthoredReference;
use crate::lower::facts::AuthoredRelationshipDeclaration;
use crate::lower::facts::AuthoredUnitToken;
use crate::lower::facts::CanonicalDocument;
use crate::lower::facts::ConstructorExpressionRecord;
use crate::lower::facts::Declaration;
use crate::lower::facts::DeclarationFacts;
use crate::lower::facts::DeclarationModifiers;
use crate::lower::facts::DocumentationRecord;
use crate::lower::facts::ExpressionArgumentRecord;
use crate::lower::facts::ExpressionGrammar;
use crate::lower::facts::FeatureChainExpressionRecord;
use crate::lower::facts::FeatureReferenceExpressionRecord;
use crate::lower::facts::FeatureValueKind;
use crate::lower::facts::FeatureValueRecord;
use crate::lower::facts::FilterForm;
use crate::lower::facts::FilterPredicate;
use crate::lower::facts::LineIndex;
use crate::lower::facts::MemberAccessNarrowing;
use crate::lower::facts::MembershipRecord;
use crate::lower::facts::MetadataAnnotationRecord;
use crate::lower::facts::OperatorExpressionKind;
use crate::lower::facts::OperatorExpressionRecord;
use crate::lower::facts::ParameterDirection;
use crate::lower::facts::ParserReferenceId;
use crate::lower::facts::PendingEvaluationFact;
use crate::lower::facts::PendingReference;
use crate::lower::facts::RecoveryRecord;
use crate::lower::facts::RelationshipFlags;
use crate::lower::facts::UnsupportedFamily;
use crate::lower::facts::UnsupportedRecord;
use crate::lower::intern::SymbolPathArenaBuilder;
use crate::lower::intern::SymbolTableBuilder;
use crate::lower::storage::ParsedSources;
use crate::lower::storage::SemanticModelStorage;
use crate::model::AuthoredReferenceId;
use crate::model::ConstructionError;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::DocumentIdx;
use crate::model::MembershipKind;
use crate::model::NameId;
use crate::model::ReferenceKind;
use crate::model::Visibility;
use crate::MembershipRole;
use hashbrown::HashTable;
use std::hash::BuildHasher;

use source_identity::ContentDigest;
use source_identity::SourceRole;
use std::collections::hash_map::RandomState;
use std::collections::BTreeMap;
use std::sync::Arc;
use sysml_v2_parser::ast::CollectionOperator;
use sysml_v2_parser::ast::CommentBody;
use sysml_v2_parser::ast::DeclarationName;
use sysml_v2_parser::ast::ReferenceSeparator;
use sysml_v2_parser::ast::StringLiteral;
use sysml_v2_parser::ast::{
    AliasBody, AliasDef, AnnotatingMember, CommentAnnotation, Dependency, DocComment, ExposeMember,
    Expression, ExtendedDefinition, FeatureValue, FeatureValueKind as ParserFeatureValueKind,
    Import, ImportShape, LibraryPackage, Membership, MembershipKind as ParserMembershipKind,
    NamespaceDecl, Node, Package, PackageBody, PackageBodyElement, QualifiedIdentification,
    QualifiedReferenceId, RelationshipBodyElement, RootElement, Span, SubsettingKind,
    SubsettingRelationship, TextualRepresentation, TypeCheckKind, VariantTypedUsage, VariantUsage,
    VariantUsageForm, Visibility as ParserVisibility,
};
use sysml_v2_parser::{ParseError, ParsedDocument};

pub(crate) mod actions;
pub(crate) mod connections;
pub(crate) mod constraints;
pub(crate) mod document;
pub(crate) mod facts;
pub(crate) mod intern;
pub(crate) mod kerml;
pub(crate) mod memo;
pub(crate) mod metadata;
pub(crate) mod parts;
pub(crate) mod requirements;
pub(crate) mod states;
pub(crate) mod storage;
pub(crate) mod views;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FeatureValueEndpoints {
    pub(crate) expression: DeclarationId,
    pub(crate) result: DeclarationId,
}

/// One buffered body-less `#tag` prefix metadata keyword awaiting binding to the member it
/// precedes. Both fields are arena/source identities copied out of the parser node, so the buffer
/// does not borrow the parsed document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PendingPrefixMetadata {
    /// `OwnedFeatureTyping` -- the metadata type the tag refers to (`refinement`).
    pub(crate) reference: QualifiedReferenceId,
    /// The `#tag` keyword span, used both for the minted annotation's declaration and, when the
    /// prefix never binds, for its unsupported-member diagnostic.
    pub(crate) span: Span,
}

#[derive(Debug, Default)]
pub(crate) struct SemanticModelBuilder {
    pub(crate) documents: Vec<AdmittedDocument>,
    pub(crate) document_index: HashTable<DocumentIdx>,
    pub(crate) document_hash_builder: RandomState,
    pub(crate) declarations: Vec<Declaration>,
    pub(crate) declaration_facts: Vec<DeclarationFacts>,
    pub(crate) memberships: Vec<MembershipRecord>,
    pub(crate) references: Vec<AuthoredReference>,
    pub(crate) relationship_declarations: Vec<AuthoredRelationshipDeclaration>,
    pub(crate) documentation: Vec<DocumentationRecord>,
    pub(crate) feature_values: Vec<FeatureValueRecord>,
    pub(crate) operator_expressions: Vec<OperatorExpressionRecord>,
    pub(crate) expression_arguments: Vec<ExpressionArgumentRecord>,
    pub(crate) constructor_expressions: Vec<ConstructorExpressionRecord>,
    pub(crate) feature_chain_expressions: Vec<FeatureChainExpressionRecord>,
    pub(crate) feature_reference_expressions: Vec<FeatureReferenceExpressionRecord>,
    pub(crate) metadata_annotations: Vec<MetadataAnnotationRecord>,
    pub(crate) unsupported: Vec<UnsupportedRecord>,
    pub(crate) recovery: Vec<RecoveryRecord>,
    pub(crate) evaluation_facts: Vec<PendingEvaluationFact>,
    pub(crate) unit_tokens: Vec<AuthoredUnitToken>,
    pub(crate) filter_conditions: Vec<AuthoredFilterCondition>,
    pub(crate) invocations: Vec<AuthoredInvocation>,
    pub(crate) symbols: SymbolTableBuilder,
    pub(crate) paths: SymbolPathArenaBuilder,
    pub(crate) path_scratch: Vec<NameId>,
    pub(crate) next_anonymous_ordinals:
        BTreeMap<(DocumentIdx, Option<DeclarationId>, DeclarationKind), u32>,
    pub(crate) next_reference_ordinals: BTreeMap<(DeclarationId, ReferenceKind), u32>,
    /// A role supplied by an enclosing membership production and consumed by the very next
    /// membership construction. This preserves roles such as `variant` while reusing the ordinary
    /// element lowering for the owned member.
    pub(crate) next_membership_override: Option<(MembershipKind, Visibility, MembershipRole, Span)>,
    /// Body-less `#tag` prefix metadata keywords (`PrefixMetadataAnnotation`, e.g. the `#refinement`
    /// in `#refinement dependency X to Y;`) seen in a definition body but not yet bound. The parser
    /// emits them as standalone sibling `MetadataKeywordUsage` elements with no link to the member
    /// they prefix, so the enclosing body walker buffers them here and binds them to that member's
    /// declaration via [`Self::bind_pending_prefix_metadata`]. Every walker that fills this drains
    /// it before any other member and at the end of the body, so an unbound prefix stays an
    /// explicit unsupported member rather than leaking to a later declaration.
    pub(crate) pending_prefix_metadata: Vec<PendingPrefixMetadata>,
    /// Counts each owner's authored `end` members so every positional connector end carries the
    /// order it was written in. Keyed by owner alone: an owner's ends are lowered in source order
    /// by one walker, so the counter is the authored position.
    pub(crate) next_positional_end_ordinals: BTreeMap<DeclarationId, u32>,
    /// Counts each owner's authored FeatureMemberships in lowering order. The resulting one-based
    /// position is the canonical ordered `ownedFeature` fact consumed after publication.
    pub(crate) next_owned_feature_ordinals: BTreeMap<DeclarationId, u32>,
    /// Counts each declaration's authored unit tokens, so each carries the order it was written
    /// in rather than the order the table happened to be filled.
    pub(crate) next_unit_token_ordinals: BTreeMap<DeclarationId, u32>,
}

impl SemanticModelBuilder {
    pub(crate) fn admit_document(
        &mut self,
        identity: impl Into<Box<str>>,
        role: SourceRole,
        digest: ContentDigest,
        parsed: Arc<ParsedDocument>,
        parse_errors: Vec<ParseError>,
    ) -> Result<DocumentIdx, ConstructionError> {
        let identity = identity.into();
        let hash = self.document_hash_builder.hash_one(identity.as_ref());
        if self
            .document_index
            .find(hash, |candidate| {
                self.documents[candidate.index()].identity.as_ref() == identity.as_ref()
            })
            .is_some()
        {
            return Err(ConstructionError::DuplicateDocumentIdentity);
        }
        let id = DocumentIdx::from_index(self.documents.len())?;
        self.documents
            .try_reserve(1)
            .map_err(|_| ConstructionError::Capacity)?;
        let documents = &self.documents;
        let hash_builder = &self.document_hash_builder;
        self.document_index
            .try_reserve(1, |candidate| {
                hash_builder.hash_one(documents[candidate.index()].identity.as_ref())
            })
            .map_err(|_| ConstructionError::Capacity)?;
        self.documents.push(AdmittedDocument {
            identity,
            role,
            digest,
            parsed,
            parse_errors: parse_errors.into_boxed_slice(),
        });
        let documents = &self.documents;
        let hash_builder = &self.document_hash_builder;
        self.document_index.insert_unique(hash, id, |candidate| {
            hash_builder.hash_one(documents[candidate.index()].identity.as_ref())
        });
        Ok(id)
    }

    pub(crate) fn intern_name(&mut self, value: &str) -> Result<NameId, ConstructionError> {
        self.symbols.intern(value)
    }

    pub(crate) fn intern_declared_name(
        &mut self,
        value: &str,
    ) -> Result<Option<NameId>, ConstructionError> {
        (!value.is_empty())
            .then(|| self.intern_name(value))
            .transpose()
    }

    #[cfg(test)]
    pub(crate) fn push_declaration(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        name: Option<NameId>,
    ) -> Result<DeclarationId, ConstructionError> {
        self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::Package,
            name,
            Span::dummy(),
            DeclarationFacts::none(),
        )
    }

    /// Mints one declaration identity and records its authored facts in the same call.
    ///
    /// `facts` is a required parameter rather than an optional follow-up so that every present and
    /// future lowering site has to make an explicit decision about the declaration's modifiers,
    /// multiplicity, direction, and short name. A site with nothing to record passes
    /// `DeclarationFacts::none()`; a site that simply forgets does not compile.
    /// The next authored position among `owner`'s connector ends.
    pub(crate) fn next_positional_end_ordinal(
        &mut self,
        owner: DeclarationId,
    ) -> Result<u32, ConstructionError> {
        let ordinal = self.next_positional_end_ordinals.entry(owner).or_insert(0);
        let value = *ordinal;
        *ordinal = ordinal.checked_add(1).ok_or(ConstructionError::Capacity)?;
        Ok(value)
    }

    pub(crate) fn push_typed_declaration(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        kind: DeclarationKind,
        name: Option<NameId>,
        span: Span,
        facts: DeclarationFacts,
    ) -> Result<DeclarationId, ConstructionError> {
        if document.index() >= self.documents.len()
            || owner.is_some_and(|id| id.index() >= self.declarations.len())
            || name.is_some_and(|id| id.index() >= self.symbols.len())
            || facts
                .short_name
                .is_some_and(|id| id.index() >= self.symbols.len())
        {
            return Err(ConstructionError::InvalidIdentity);
        }
        let contributes_owned_end = facts.modifiers.end || facts.positional_end.is_some();
        let id = DeclarationId::from_index(self.declarations.len())?;
        let anonymous_ordinal = if name.is_none() {
            let ordinal = self
                .next_anonymous_ordinals
                .entry((document, owner, kind))
                .or_insert(0);
            let value = *ordinal;
            *ordinal = ordinal.checked_add(1).ok_or(ConstructionError::Capacity)?;
            Some(value)
        } else {
            None
        };
        self.declarations.push(Declaration {
            document,
            owner,
            name,
            anonymous_ordinal,
            kind,
            span,
        });
        self.declaration_facts.push(facts);
        if contributes_owned_end {
            if let Some(count) = owner
                .and_then(|owner| self.declaration_facts.get_mut(owner.index()))
                .and_then(|owner_facts| owner_facts.owned_end_feature_count.as_mut())
            {
                *count = count.checked_add(1).ok_or(ConstructionError::Capacity)?;
            }
        }
        debug_assert_eq!(self.declarations.len(), self.declaration_facts.len());
        Ok(id)
    }

    /// Records one `doc`/`comment`/`rep` annotation against the declaration it annotates.
    ///
    /// The parser attaches these as sibling body elements with no parent link, so the binding is
    /// made by the lowering walk rather than read off the annotated node.
    pub(crate) fn push_documentation(
        &mut self,
        declaration: DeclarationId,
        form: AnnotationForm,
        locale: Option<NameId>,
        language: Option<NameId>,
        text: NameId,
        span: Span,
    ) -> Result<(), ConstructionError> {
        if declaration.index() >= self.declarations.len()
            || text.index() >= self.symbols.len()
            || locale.is_some_and(|id| id.index() >= self.symbols.len())
            || language.is_some_and(|id| id.index() >= self.symbols.len())
        {
            return Err(ConstructionError::InvalidIdentity);
        }
        self.documentation.push(DocumentationRecord {
            declaration,
            form,
            locale,
            language,
            text,
            span,
        });
        Ok(())
    }

    /// Interns an optional authored declaration name or `<shortName>`, decoding it through the
    /// owning document (`ParsedDocument::decoded_declaration_name`): the parser carries names
    /// as source spans, and an escaped unrestricted name decodes to its `NAME` value here. An
    /// absent or empty spelling is `None`.
    pub(crate) fn intern_declaration_name(
        &mut self,
        document: DocumentIdx,
        name: Option<DeclarationName>,
    ) -> Result<Option<NameId>, ConstructionError> {
        let Some(name) = name else {
            return Ok(None);
        };
        let parsed = Arc::clone(
            &self
                .documents
                .get(document.index())
                .ok_or(ConstructionError::InvalidParserReference)?
                .parsed,
        );
        let decoded = parsed
            .decoded_declaration_name(name)
            .ok_or(ConstructionError::InvalidParserReference)?;
        self.intern_declared_name(decoded.as_ref())
    }

    /// Interns the decoded contents of an optional authored string literal through its owning
    /// document. The semantic model stores the value, while the parser remains the source-fidelity
    /// owner for quotes and escapes.
    pub(crate) fn intern_string_literal(
        &mut self,
        document: DocumentIdx,
        literal: Option<StringLiteral>,
    ) -> Result<Option<NameId>, ConstructionError> {
        let Some(literal) = literal else {
            return Ok(None);
        };
        let parsed = Arc::clone(
            &self
                .documents
                .get(document.index())
                .ok_or(ConstructionError::InvalidParserReference)?
                .parsed,
        );
        let decoded = parsed
            .decoded_string_literal(literal)
            .ok_or(ConstructionError::InvalidParserReference)?;
        self.intern_declared_name(decoded.as_ref())
    }

    /// Interns the authored bytes of a regular-comment body. Normalized comment text is a
    /// separate parser query; documentation publication has historically preserved the authored
    /// body and continues to do so across the span-backed AST migration.
    pub(crate) fn intern_comment_body(
        &mut self,
        document: DocumentIdx,
        body: CommentBody,
    ) -> Result<NameId, ConstructionError> {
        let parsed = Arc::clone(
            &self
                .documents
                .get(document.index())
                .ok_or(ConstructionError::InvalidParserReference)?
                .parsed,
        );
        let text = parsed
            .comment_body(body)
            .ok_or(ConstructionError::InvalidParserReference)?;
        self.intern_name(text)
    }

    /// Interns an optional authored `<shortName>` prefix; see `intern_declaration_name`.
    pub(crate) fn intern_short_name(
        &mut self,
        document: DocumentIdx,
        short_name: Option<DeclarationName>,
    ) -> Result<Option<NameId>, ConstructionError> {
        self.intern_declaration_name(document, short_name)
    }

    /// Records a `doc /* ... */` body element against the declaration owning that body.
    ///
    /// `declaration` is `None` only for a `doc` written at document-root scope, whose annotated
    /// element is the file root -- an element this model deliberately does not mint a declaration
    /// for -- so there is nothing to bind it to.
    pub(crate) fn record_root_doc_comment(
        &mut self,
        document: DocumentIdx,
        declaration: Option<DeclarationId>,
        node: &Node<DocComment>,
    ) -> Result<(), ConstructionError> {
        match declaration {
            Some(declaration) => self.record_doc_comment(document, declaration, node),
            None => Ok(()),
        }
    }

    /// Root-scope counterpart of `record_comment_annotation`; see `record_root_doc_comment`.
    pub(crate) fn record_root_comment_annotation(
        &mut self,
        document: DocumentIdx,
        declaration: Option<DeclarationId>,
        node: &Node<CommentAnnotation>,
    ) -> Result<(), ConstructionError> {
        match declaration {
            Some(declaration) => self.record_comment_annotation(document, declaration, node),
            None => Ok(()),
        }
    }

    /// Root-scope counterpart of `record_textual_representation`; see `record_root_doc_comment`.
    pub(crate) fn record_root_textual_representation(
        &mut self,
        document: DocumentIdx,
        declaration: Option<DeclarationId>,
        node: &Node<TextualRepresentation>,
    ) -> Result<(), ConstructionError> {
        match declaration {
            Some(declaration) => self.record_textual_representation(document, declaration, node),
            None => Ok(()),
        }
    }

    /// Lowers the grammar's whole `AnnotatingElement` production (`ast::AnnotatingMember`:
    /// `doc`, `comment`, `rep`, and the `@Name` metadata spelling), which upstream dispatches as
    /// one member in every scope that accepts all four alternatives. One production, one lowering:
    /// the alternatives keep the same per-form owners (`record_doc_comment`,
    /// `record_comment_annotation`, `record_textual_representation`,
    /// `lower_metadata_annotation`) they have wherever a scope still spells them out separately.
    ///
    /// `annotated` is `None` only where the construct owning the body mints no declaration of its
    /// own -- a `connect a to b { ... }` statement lowers its ends directly against the enclosing
    /// declaration -- so there is no element the annotation belongs to and attributing it to the
    /// enclosing type would misreport it. The three documentation forms are simply not recorded
    /// there (they are inert text with nowhere to hang); an `@Name` annotation is not, because it
    /// carries a reference whose source declaration is exactly what is missing, so it is reported
    /// as an explicit `family` unsupported member rather than dropped.
    pub(crate) fn lower_annotating_member(
        &mut self,
        document: DocumentIdx,
        annotated: Option<DeclarationId>,
        family: UnsupportedFamily,
        member: &AnnotatingMember,
    ) -> Result<(), ConstructionError> {
        match member {
            AnnotatingMember::Doc(node) => self.record_root_doc_comment(document, annotated, node),
            AnnotatingMember::Comment(node) => {
                self.record_root_comment_annotation(document, annotated, node)
            }
            AnnotatingMember::TextualRep(node) => {
                self.record_root_textual_representation(document, annotated, node)
            }
            AnnotatingMember::MetadataAnnotation(node) => match annotated {
                Some(annotated) => self.lower_metadata_annotation(document, annotated, node),
                None => {
                    self.push_unsupported(document, family, node.span);
                    Ok(())
                }
            },
        }
    }

    /// Records a `doc /* ... */` annotation against the declaration whose body it heads.
    pub(crate) fn record_doc_comment(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
        node: &Node<DocComment>,
    ) -> Result<(), ConstructionError> {
        let locale = self.intern_string_literal(document, node.value.locale)?;
        let text = self.intern_comment_body(document, node.value.body)?;
        self.push_documentation(
            declaration,
            AnnotationForm::Documentation,
            locale,
            None,
            text,
            node.span,
        )
    }

    /// Records a `comment /* ... */` annotation against the declaration whose body it heads.
    pub(crate) fn record_comment_annotation(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
        node: &Node<CommentAnnotation>,
    ) -> Result<(), ConstructionError> {
        let locale = self.intern_string_literal(document, node.value.locale)?;
        let text = self.intern_comment_body(document, node.value.body)?;
        self.push_documentation(
            declaration,
            AnnotationForm::Comment,
            locale,
            None,
            text,
            node.span,
        )
    }

    /// Records a `rep <language> "..." /* ... */` annotation against the declaration whose body it
    /// heads.
    pub(crate) fn record_textual_representation(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
        node: &Node<TextualRepresentation>,
    ) -> Result<(), ConstructionError> {
        let language = self.intern_string_literal(document, node.value.language)?;
        let text = self.intern_comment_body(document, node.value.body)?;
        self.push_documentation(
            declaration,
            AnnotationForm::TextualRepresentation,
            None,
            language,
            text,
            node.span,
        )
    }

    /// Constructs the canonical value Expression and result Feature for a `FeatureValue` clause.
    pub(crate) fn record_feature_value(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
        value: &Node<FeatureValue>,
    ) -> Result<FeatureValueEndpoints, ConstructionError> {
        let kind = match value.value.kind {
            ParserFeatureValueKind::Bind => FeatureValueKind::Bind,
            ParserFeatureValueKind::Assign => FeatureValueKind::Assign,
        };
        let expression = self.push_typed_declaration(
            document,
            Some(declaration),
            DeclarationKind::KermlExpression,
            None,
            value.value.expression.span,
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            expression,
            MembershipKind::Owning,
            Visibility::Default,
            value.value.expression.span,
        )?;
        let result = self.push_typed_declaration(
            document,
            Some(expression),
            DeclarationKind::KermlFeature,
            None,
            value.value.expression.span,
            DeclarationFacts {
                direction: Some(ParameterDirection::Out),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            result,
            MembershipKind::Feature,
            Visibility::Default,
            value.value.expression.span,
        )?;
        self.declaration_facts[expression.index()].expression_result = Some(result);
        if matches!(
            value.value.expression.value,
            Expression::MemberAccess { .. } | Expression::FeatureChainRef(_)
        ) {
            let input_parameter = self.push_typed_declaration(
                document,
                Some(expression),
                DeclarationKind::KermlFeature,
                None,
                value.value.expression.span,
                DeclarationFacts {
                    direction: Some(ParameterDirection::In),
                    ..DeclarationFacts::none()
                },
            )?;
            self.push_membership(
                input_parameter,
                MembershipKind::Feature,
                Visibility::Default,
                value.value.expression.span,
            )?;
            let source_target = self.push_typed_declaration(
                document,
                Some(input_parameter),
                DeclarationKind::KermlFeature,
                None,
                value.value.expression.span,
                DeclarationFacts::none(),
            )?;
            self.push_membership(
                source_target,
                MembershipKind::Feature,
                Visibility::Default,
                value.value.expression.span,
            )?;
            let subsetting_chain = self.push_typed_declaration(
                document,
                Some(expression),
                DeclarationKind::KermlFeature,
                None,
                value.value.expression.span,
                DeclarationFacts::none(),
            )?;
            self.push_membership(
                subsetting_chain,
                MembershipKind::Owning,
                Visibility::Default,
                value.value.expression.span,
            )?;
            self.feature_chain_expressions
                .push(FeatureChainExpressionRecord {
                    expression,
                    result,
                    input_parameter,
                    source_target,
                    subsetting_chain,
                });
        }
        if matches!(value.value.expression.value, Expression::FeatureRef(_)) {
            self.feature_reference_expressions
                .push(FeatureReferenceExpressionRecord { expression, result });
        }
        self.record_operator_expression(document, expression, result, &value.value.expression)?;
        if matches!(value.value.expression.value, Expression::Constructor { .. }) {
            self.constructor_expressions
                .push(ConstructorExpressionRecord { expression, result });
        }
        let endpoints = FeatureValueEndpoints { expression, result };
        self.push_feature_value(
            declaration,
            endpoints,
            kind,
            value.value.is_default,
            value.value.has_operator,
            value.value.span,
        )?;
        Ok(endpoints)
    }

    fn push_expression_argument(
        &mut self,
        document: DocumentIdx,
        expression: DeclarationId,
        ordinal: u32,
        span: Span,
    ) -> Result<(), ConstructionError> {
        let argument = self.push_typed_declaration(
            document,
            Some(expression),
            DeclarationKind::KermlExpression,
            None,
            span,
            DeclarationFacts::none(),
        )?;
        self.push_membership(argument, MembershipKind::Owning, Visibility::Default, span)?;
        let result = self.push_typed_declaration(
            document,
            Some(argument),
            DeclarationKind::KermlFeature,
            None,
            span,
            DeclarationFacts {
                direction: Some(ParameterDirection::Out),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(result, MembershipKind::Feature, Visibility::Default, span)?;
        self.declaration_facts[argument.index()].expression_result = Some(result);
        self.expression_arguments.push(ExpressionArgumentRecord {
            expression,
            argument,
            result,
            ordinal,
        });
        Ok(())
    }

    /// Materializes the ordered argument Expressions of the two operator metaclasses whose
    /// specialization constraints consume `arguments->first().result`.
    fn record_operator_expression(
        &mut self,
        document: DocumentIdx,
        expression: DeclarationId,
        result: DeclarationId,
        node: &Node<Expression>,
    ) -> Result<(), ConstructionError> {
        let (kind, spans) = match &node.value {
            Expression::Index { base, operands, .. } => (
                OperatorExpressionKind::Index,
                std::iter::once(base.span)
                    .chain(
                        operands
                            .value
                            .elements
                            .iter()
                            .map(|element| element.expression.span),
                    )
                    .collect::<Vec<_>>(),
            ),
            Expression::Select { base, selector } => {
                let selector_span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(*selector)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span;
                (
                    OperatorExpressionKind::Select,
                    vec![base.span, selector_span],
                )
            }
            Expression::CollectionOp {
                op: CollectionOperator::Select,
                base,
                args,
                brace_body: None,
                ..
            } => (
                OperatorExpressionKind::Select,
                std::iter::once(base.span)
                    .chain(args.iter().map(|argument| argument.value.span))
                    .collect::<Vec<_>>(),
            ),
            _ => return Ok(()),
        };
        self.operator_expressions.push(OperatorExpressionRecord {
            expression,
            result,
            kind,
        });
        for (ordinal, span) in spans.into_iter().enumerate() {
            self.push_expression_argument(
                document,
                expression,
                u32::try_from(ordinal).map_err(|_| ConstructionError::Capacity)?,
                span,
            )?;
        }
        Ok(())
    }

    /// Records the authored feature value spelling of one declaration.
    ///
    /// The record preserves both canonical endpoints and which of the five authored spellings
    /// (`=`, `:=`, `default =`, `default :=`, bare `default`) was written.
    pub(crate) fn push_feature_value(
        &mut self,
        declaration: DeclarationId,
        endpoints: FeatureValueEndpoints,
        kind: FeatureValueKind,
        is_default: bool,
        has_operator: bool,
        span: Span,
    ) -> Result<(), ConstructionError> {
        if declaration.index() >= self.declarations.len() {
            return Err(ConstructionError::InvalidIdentity);
        }
        self.feature_values.push(FeatureValueRecord {
            declaration,
            value: endpoints.expression,
            result: endpoints.result,
            kind,
            is_default,
            has_operator,
            span,
        });
        Ok(())
    }

    pub(crate) fn push_membership(
        &mut self,
        member: DeclarationId,
        kind: MembershipKind,
        visibility: Visibility,
        span: Span,
    ) -> Result<(), ConstructionError> {
        if member.index() >= self.declarations.len() {
            return Err(ConstructionError::InvalidIdentity);
        }
        let (kind, visibility, role, span) = self.next_membership_override.take().map_or(
            (kind, visibility, None, span),
            |(kind, visibility, role, span)| (kind, visibility, Some(role), span),
        );
        if kind == MembershipKind::Feature {
            if let Some(owner) = self.declarations[member.index()].owner {
                let ordinal = self.next_owned_feature_ordinals.entry(owner).or_insert(0);
                *ordinal = ordinal.checked_add(1).ok_or(ConstructionError::Capacity)?;
                self.declaration_facts[member.index()].owned_feature_position = Some(*ordinal);
            }
        }
        self.memberships.push(MembershipRecord {
            member,
            kind,
            visibility,
            role,
            span,
        });
        Ok(())
    }

    pub(crate) fn push_reference(
        &mut self,
        pending: PendingReference,
    ) -> Result<AuthoredReferenceId, ConstructionError> {
        let PendingReference {
            source,
            kind,
            document,
            local,
            flags,
            span,
            import,
        } = pending;
        if source.index() >= self.declarations.len() || document.index() >= self.documents.len() {
            return Err(ConstructionError::InvalidParserReference);
        }
        let parsed = Arc::clone(&self.documents[document.index()].parsed);
        let reference = parsed
            .qualified_reference(local)
            .ok_or(ConstructionError::InvalidParserReference)?;
        let mut segments = std::mem::take(&mut self.path_scratch);
        segments.clear();
        segments
            .try_reserve(reference.segments.len())
            .map_err(|_| ConstructionError::Capacity)?;
        let path = (|| {
            for index in 0..reference.segments.len() {
                let decoded = reference
                    .segment_decoded_text(index)
                    .ok_or(ConstructionError::InvalidParserReference)?;
                segments.push(self.intern_name(decoded.as_ref())?);
            }
            self.paths.push(&segments, reference.metadata.is_absolute)
        })();
        segments.clear();
        self.path_scratch = segments;
        let path = path?;
        let dotted = reference
            .segments
            .iter()
            .any(|segment| segment.separator_before == Some(ReferenceSeparator::Dot));
        let flags = RelationshipFlags { dotted, ..flags };
        let ordinal = self
            .next_reference_ordinals
            .entry((source, kind))
            .or_insert(0);
        let authored_ordinal = *ordinal;
        *ordinal = ordinal.checked_add(1).ok_or(ConstructionError::Capacity)?;
        let id = AuthoredReferenceId::from_index(self.references.len())?;
        self.references.push(AuthoredReference {
            source,
            kind,
            target: ParserReferenceId { document, local },
            path,
            ordinal: authored_ordinal,
            import,
            flags,
            member_access_narrowings: Box::default(),
            span,
        });
        Ok(id)
    }

    /// Reserves one authored-reference position for a grammar-owned endpoint that has no name to
    /// resolve. This keeps later positional endpoints (for example a transition succession's
    /// target when its source is implicit) at their canonical ordinal without inventing a
    /// reference or resolved semantic fact.
    pub(crate) fn reserve_reference_ordinal(
        &mut self,
        source: DeclarationId,
        kind: ReferenceKind,
    ) -> Result<(), ConstructionError> {
        let ordinal = self
            .next_reference_ordinals
            .entry((source, kind))
            .or_insert(0);
        *ordinal = ordinal.checked_add(1).ok_or(ConstructionError::Capacity)?;
        Ok(())
    }

    /// Pushes one `ReferenceKind::MemberAccessOperand` reference for a flattened dotted
    /// feature-chain: `chain` is the ordered list of
    /// parser `QualifiedReferenceId`s from the root segment outward (a bare `FeatureRef`/
    /// `FeatureChainRef` flattens to a one-entry chain). Builds one combined `SymbolPathId` by
    /// concatenating every chain entry's own segments in order -- mirroring `push_reference`'s
    /// single-reference path construction, but across multiple parser references -- so
    /// `resolve_member_access_reference` in resolver.rs can walk the whole dotted path as one
    /// path with a root-lookup first segment followed by type-directed member segments. Always
    /// non-rooted (`::`-absolute chains do not occur in dotted member-access position), matching
    /// `ConnectorEnd`/`ExpressionOperand`'s existing `DeclarationDomain::Any` shape.
    pub(crate) fn push_member_access_reference(
        &mut self,
        source: DeclarationId,
        document: DocumentIdx,
        chain: &[QualifiedReferenceId],
        span: Span,
    ) -> Result<AuthoredReferenceId, ConstructionError> {
        self.push_member_access_reference_with_kind(
            source,
            document,
            ReferenceKind::MemberAccessOperand,
            chain,
            span,
        )
    }

    /// Lowers a member-access expression without erasing authored `as Type` narrowing.
    ///
    /// The flattened path remains the canonical sequence of authored feature names. Each cast is
    /// also an ordinary typed `TypeCheckTarget` reference and the member-access fact records the
    /// exact segment boundary at which that resolved type becomes the scope for later hops.
    pub(crate) fn push_member_access_expression(
        &mut self,
        source: DeclarationId,
        document: DocumentIdx,
        node: &Node<Expression>,
    ) -> Result<Option<AuthoredReferenceId>, ConstructionError> {
        fn collect(
            node: &Node<Expression>,
            chain: &mut Vec<QualifiedReferenceId>,
            casts: &mut Vec<(usize, QualifiedReferenceId)>,
        ) -> bool {
            match &node.value {
                Expression::FeatureRef(target) | Expression::FeatureChainRef(target) => {
                    chain.push(*target);
                    true
                }
                Expression::MemberAccess { base, member, .. } => {
                    if !collect(base, chain, casts) {
                        return false;
                    }
                    chain.push(*member);
                    true
                }
                Expression::Sequence { operands, .. } => match operands.value.elements.as_slice() {
                    [only] => collect(&only.expression, chain, casts),
                    _ => false,
                },
                Expression::TypeCheck {
                    kind,
                    operand: Some(operand),
                    type_name,
                } => {
                    if !collect(operand, chain, casts) {
                        return false;
                    }
                    if *kind == TypeCheckKind::As {
                        casts.push((chain.len(), *type_name));
                    }
                    true
                }
                _ => false,
            }
        }

        let mut chain = Vec::new();
        let mut casts = Vec::new();
        if !collect(node, &mut chain, &mut casts) || chain.is_empty() {
            return Ok(None);
        }
        let parsed = Arc::clone(&self.documents[document.index()].parsed);
        let mut narrowings = Vec::with_capacity(casts.len());
        for (chain_entry_count, target) in casts {
            let segment_count =
                chain[..chain_entry_count]
                    .iter()
                    .try_fold(0usize, |count, local| {
                        let reference = parsed
                            .qualified_reference(*local)
                            .ok_or(ConstructionError::InvalidParserReference)?;
                        count
                            .checked_add(reference.segments.len())
                            .ok_or(ConstructionError::Capacity)
                    })?;
            let target_reference =
                self.push_type_check_target_reference_id(document, source, target)?;
            narrowings.push(MemberAccessNarrowing {
                segment_count: u32::try_from(segment_count)
                    .map_err(|_| ConstructionError::Capacity)?,
                target: target_reference,
            });
        }
        let reference = self.push_member_access_reference(source, document, &chain, node.span)?;
        self.references[reference.index()].member_access_narrowings = narrowings.into_boxed_slice();
        Ok(Some(reference))
    }

    pub(crate) fn push_member_access_reference_with_kind(
        &mut self,
        source: DeclarationId,
        document: DocumentIdx,
        kind: ReferenceKind,
        chain: &[QualifiedReferenceId],
        span: Span,
    ) -> Result<AuthoredReferenceId, ConstructionError> {
        if chain.is_empty()
            || source.index() >= self.declarations.len()
            || document.index() >= self.documents.len()
        {
            return Err(ConstructionError::InvalidParserReference);
        }
        let parsed = Arc::clone(&self.documents[document.index()].parsed);
        let mut segments = std::mem::take(&mut self.path_scratch);
        segments.clear();
        let path = (|| {
            for local in chain {
                let reference = parsed
                    .qualified_reference(*local)
                    .ok_or(ConstructionError::InvalidParserReference)?;
                segments
                    .try_reserve(reference.segments.len())
                    .map_err(|_| ConstructionError::Capacity)?;
                for index in 0..reference.segments.len() {
                    let decoded = reference
                        .segment_decoded_text(index)
                        .ok_or(ConstructionError::InvalidParserReference)?;
                    segments.push(self.intern_name(decoded.as_ref())?);
                }
            }
            self.paths.push(&segments, false)
        })();
        segments.clear();
        self.path_scratch = segments;
        let path = path?;
        let ordinal = self
            .next_reference_ordinals
            .entry((source, kind))
            .or_insert(0);
        let authored_ordinal = *ordinal;
        *ordinal = ordinal.checked_add(1).ok_or(ConstructionError::Capacity)?;
        let id = AuthoredReferenceId::from_index(self.references.len())?;
        self.references.push(AuthoredReference {
            source,
            kind,
            target: ParserReferenceId {
                document,
                local: *chain
                    .last()
                    .ok_or(ConstructionError::InvalidParserReference)?,
            },
            path,
            ordinal: authored_ordinal,
            import: None,
            flags: RelationshipFlags::default(),
            member_access_narrowings: Box::default(),
            span,
        });
        Ok(id)
    }

    /// Resolves the callee of an `Expression::Invocation` (e.g. `sum` in `sum(partMasses)`) as an
    /// authored `ReferenceKind::InvocationCallee` reference sourced at `declaration`. A simple/
    /// qualified name (`FeatureRef`/`FeatureChainRef`) resolves through the same
    /// `DeclarationDomain::Any` lexical lookup fixed point every other operand kind uses; a dotted
    /// chain (`MemberAccess`, e.g. a callee like `SysML::sum`) resolves through the same
    /// typed member-access lowering path `ExpressionOperand`'s own
    /// `MemberAccess` arm uses (publishing `ReferenceKind::MemberAccessOperand`, not
    /// `InvocationCallee`, matching that shared path's existing "one kind per algorithm" trade-off
    /// -- see `ReferenceKind::MemberAccessOperand`'s doc comment). Any other callee shape (e.g. an
    /// invocation whose callee is itself computed, `(a + b)(x)`) is left unresolved: this narrow
    /// helper has no `UnsupportedFamily` to publish a diagnostic against (the invocation itself is
    /// a supported shape; only this specific callee sub-shape is not), so it silently resolves
    /// nothing for that callee rather than fabricating a reference.
    ///
    /// `argument_count` and `span` describe the call site itself. They are recorded only when the
    /// callee resolves through an `InvocationCallee` reference, because an invocation whose callee
    /// this helper cannot name has nothing to compare its arguments against, and a record without a
    /// callee would be an argument count attributed to no callee at all.
    pub(crate) fn lower_invocation_callee(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
        callee: &Node<Expression>,
        argument_count: usize,
        span: Span,
    ) -> Result<(), ConstructionError> {
        match &callee.value {
            Expression::FeatureRef(target) | Expression::FeatureChainRef(target) => {
                let reference =
                    self.push_invocation_callee_reference(document, declaration, *target)?;
                self.push_invocation(declaration, document, reference, argument_count, span)
            }
            Expression::MemberAccess { .. } => {
                self.push_member_access_expression(declaration, document, callee)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Pushes one `ReferenceKind::InvocationCallee` reference for a callee/`Constructor` type name
    /// that is already a parser `QualifiedReferenceId` (an `Expression::Invocation`'s `FeatureRef`/
    /// `FeatureChainRef` callee via `lower_invocation_callee`, or an `Expression::Constructor`'s
    /// `type_name` directly).
    pub(crate) fn push_invocation_callee_reference(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
        target: QualifiedReferenceId,
    ) -> Result<AuthoredReferenceId, ConstructionError> {
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span;
        self.push_reference(PendingReference {
            source: declaration,
            kind: ReferenceKind::InvocationCallee,
            document,
            local: target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })
    }

    /// Pushes one `ReferenceKind::MetaCastTarget` reference for an `Expression::MetaCast`'s
    /// `metaclass` (e.g. `KerML::Classifier` in `Atom meta KerML::Classifier`), mirroring
    /// `push_type_check_target_reference`'s shape but its own `ReferenceKind` since a meta-cast
    /// target joins the `DeclarationDomain::Type` fixed point rather than `TypeCheckTarget`
    /// directly (kept distinct purely for query-output clarity).
    pub(crate) fn push_meta_cast_target_reference(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
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
            kind: ReferenceKind::MetaCastTarget,
            document,
            local: target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    /// Pushes one `ReferenceKind::TypeCheckTarget` reference for an `Expression::TypeCheck`'s
    /// `type_name` (e.g. `Type` in `x istype Type`), mirroring `push_invocation_callee_reference`'s
    /// shape but its own `ReferenceKind` since a type-check target joins the `DeclarationDomain::
    /// Type` fixed point rather than `InvocationCallee`'s `Any` domain.
    pub(crate) fn push_type_check_target_reference(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
        target: QualifiedReferenceId,
    ) -> Result<(), ConstructionError> {
        self.push_type_check_target_reference_id(document, declaration, target)?;
        Ok(())
    }

    fn push_type_check_target_reference_id(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
        target: QualifiedReferenceId,
    ) -> Result<AuthoredReferenceId, ConstructionError> {
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span;
        self.push_reference(PendingReference {
            source: declaration,
            kind: ReferenceKind::TypeCheckTarget,
            document,
            local: target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })
    }

    pub(crate) fn push_expression_operand_reference(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
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
            kind: ReferenceKind::ExpressionOperand,
            document,
            local: target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    /// The site of a constraint-body expression: what the author wrote, and where.
    ///
    /// Records the expression; it does not classify it. What the expression evaluates to is phase
    /// 5's answer to give, over this record.
    pub(crate) fn constraint_expression_site(
        &self,
        document: DocumentIdx,
        node: &Expression,
    ) -> AuthoredExpression {
        AuthoredExpression {
            document,
            grammar: ExpressionGrammar::Constraint,
            operand_start: 0,
            node: node.clone(),
        }
    }

    /// The site of a calculation-body expression. See [`Self::constraint_expression_site`].
    pub(crate) fn calc_expression_site(
        &self,
        document: DocumentIdx,
        node: &Expression,
    ) -> AuthoredExpression {
        AuthoredExpression {
            document,
            grammar: ExpressionGrammar::Calc,
            operand_start: 0,
            node: node.clone(),
        }
    }

    pub(crate) fn push_unsupported(
        &mut self,
        document: DocumentIdx,
        family: UnsupportedFamily,
        span: Span,
    ) {
        self.unsupported.push(UnsupportedRecord {
            document,
            family,
            span,
        });
    }

    pub(crate) fn push_recovery(&mut self, document: DocumentIdx, span: Span) {
        self.recovery.push(RecoveryRecord { document, span });
    }

    /// Records one evaluation candidate: the declaration, and the expression site it authored.
    ///
    /// Every authored expression is recorded, whatever its shape. The publication has to be able
    /// to say "an expression is here and this engine does not evaluate its shape"; dropping the
    /// record would leave the declaration indistinguishable from one that authored no expression,
    /// which is a different fact about the model. Deciding which is which is phase 5's, so the
    /// record is unconditional here.
    pub(crate) fn push_evaluation_fact(
        &mut self,
        declaration: DeclarationId,
        expression: AuthoredExpression,
    ) {
        self.evaluation_facts.push(PendingEvaluationFact {
            declaration,
            expression,
        });
    }

    /// Records one authored unit token, in lockstep with the classifier that counts them.
    pub(crate) fn push_unit_token(
        &mut self,
        declaration: DeclarationId,
        document: DocumentIdx,
        text: &str,
        span: Span,
    ) -> Result<(), ConstructionError> {
        let text = self.symbols.intern(text)?;
        let ordinal = self
            .next_unit_token_ordinals
            .entry(declaration)
            .or_insert(0);
        let assigned = *ordinal;
        *ordinal = ordinal.checked_add(1).ok_or(ConstructionError::Capacity)?;
        self.unit_tokens.push(AuthoredUnitToken {
            declaration,
            document,
            ordinal: assigned,
            text,
            span,
        });
        Ok(())
    }

    /// Records one authored `filter` condition against the declaration it was written in.
    pub(crate) fn push_filter_condition(
        &mut self,
        owner: DeclarationId,
        document: DocumentIdx,
        form: FilterForm,
        span: Span,
        expression: AuthoredExpression,
        predicate: FilterPredicate,
    ) -> Result<(), ConstructionError> {
        self.filter_conditions.push(AuthoredFilterCondition {
            owner,
            document,
            form,
            span,
            expression,
            predicate,
        });
        Ok(())
    }

    /// Records one authored invocation's argument count against the callee reference naming it.
    pub(crate) fn push_invocation(
        &mut self,
        declaration: DeclarationId,
        document: DocumentIdx,
        callee: AuthoredReferenceId,
        argument_count: usize,
        span: Span,
    ) -> Result<(), ConstructionError> {
        self.invocations.push(AuthoredInvocation {
            declaration,
            document,
            callee,
            argument_count: u32::try_from(argument_count)
                .map_err(|_| ConstructionError::Capacity)?,
            span,
        });
        Ok(())
    }

    /// How many `ExpressionOperand` references this declaration has already been given.
    ///
    /// The classifier assigns each `EvalNode::Operand` leaf the ordinal the matching reference will
    /// receive, so an expression lowered after another one at the same declaration -- a view's
    /// second `filter`, say -- must start counting where the first left off.
    pub(crate) fn expression_operand_offset(&self, declaration: DeclarationId) -> u32 {
        self.next_reference_ordinals
            .get(&(declaration, ReferenceKind::ExpressionOperand))
            .copied()
            .unwrap_or(0)
    }

    /// The phase-2 barrier: the sealed storage and, separately, the parse product the later
    /// construction phases still read.
    ///
    /// They come apart here so that nothing downstream can put a tree back into the model: the
    /// sealed `SemanticModelStorage` keeps each document's identity, role and line index, and the
    /// trees leave in a value the publication barrier drops.
    pub(crate) fn freeze(self) -> (SemanticModelStorage, ParsedSources) {
        let mut documents = Vec::with_capacity(self.documents.len());
        let mut canonical = Vec::with_capacity(self.documents.len());
        for document in self.documents {
            canonical.push(CanonicalDocument {
                identity: document.identity.clone(),
                role: document.role,
                lines: LineIndex::build(document.parsed.source.as_str()),
            });
            documents.push(document);
        }
        let storage = SemanticModelStorage {
            documents: canonical.into_boxed_slice(),
            declarations: self.declarations.into_boxed_slice(),
            declaration_facts: self.declaration_facts.into_boxed_slice(),
            memberships: self.memberships.into_boxed_slice(),
            references: self.references.into_boxed_slice(),
            relationship_declarations: self.relationship_declarations.into_boxed_slice(),
            documentation: self.documentation.into_boxed_slice(),
            feature_values: self.feature_values.into_boxed_slice(),
            operator_expressions: self.operator_expressions.into_boxed_slice(),
            expression_arguments: self.expression_arguments.into_boxed_slice(),
            constructor_expressions: self.constructor_expressions.into_boxed_slice(),
            feature_chain_expressions: self.feature_chain_expressions.into_boxed_slice(),
            feature_reference_expressions: self.feature_reference_expressions.into_boxed_slice(),
            metadata_annotations: self.metadata_annotations.into_boxed_slice(),
            unsupported: self.unsupported.into_boxed_slice(),
            recovery: self.recovery.into_boxed_slice(),
            symbols: self.symbols.freeze(),
            paths: self.paths.freeze(),
            evaluation_facts: self.evaluation_facts.into_boxed_slice(),
            unit_tokens: self.unit_tokens.into_boxed_slice(),
            filter_conditions: self.filter_conditions.into_boxed_slice(),
            invocations: self.invocations.into_boxed_slice(),
        };
        (storage, ParsedSources::new(documents))
    }

    pub(crate) fn canonicalize_document(
        &mut self,
        document: DocumentIdx,
    ) -> Result<(), ConstructionError> {
        let parsed = Arc::clone(
            &self
                .documents
                .get(document.index())
                .ok_or(ConstructionError::InvalidIdentity)?
                .parsed,
        );
        for element in &parsed.root.elements {
            self.lower_root_element(document, element)?;
        }
        Ok(())
    }

    pub(crate) fn lower_root_element(
        &mut self,
        document: DocumentIdx,
        element: &Node<RootElement>,
    ) -> Result<(), ConstructionError> {
        match &element.value {
            RootElement::Package(node) => self.lower_package(document, None, node),
            RootElement::LibraryPackage(node) => self.lower_library_package(document, None, node),
            RootElement::Namespace(node) => self.lower_namespace(document, None, node),
            RootElement::Import(node) => self.lower_import(document, None, node),
            RootElement::Member(node) => self.lower_package_element(document, None, node),
        }
    }

    pub(crate) fn lower_package(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<Package>,
    ) -> Result<(), ConstructionError> {
        let name = self.simple_name(document, &node.identification)?;
        let short_name = self.intern_short_name(document, node.identification.short_name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::Package,
            name,
            node.span,
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            Visibility::Default,
            node.span,
        )?;
        self.lower_package_body(document, Some(declaration), &node.value.body)
    }

    pub(crate) fn lower_library_package(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<LibraryPackage>,
    ) -> Result<(), ConstructionError> {
        let name = self.simple_name(document, &node.identification)?;
        let short_name = self.intern_short_name(document, node.identification.short_name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::LibraryPackage,
            name,
            node.span,
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    standard: node.is_standard,
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            Visibility::Default,
            node.span,
        )?;
        self.lower_package_body(document, Some(declaration), &node.value.body)
    }

    pub(crate) fn lower_namespace(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<NamespaceDecl>,
    ) -> Result<(), ConstructionError> {
        let name = self.simple_name(document, &node.identification)?;
        let short_name = self.intern_short_name(document, node.identification.short_name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::Namespace,
            name,
            node.span,
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            Visibility::Default,
            node.span,
        )?;
        self.lower_package_body(document, Some(declaration), &node.value.body)
    }

    pub(crate) fn simple_name(
        &mut self,
        document: DocumentIdx,
        identification: &QualifiedIdentification,
    ) -> Result<Option<NameId>, ConstructionError> {
        self.intern_declaration_name(document, identification.simple_name())
    }

    pub(crate) fn lower_package_body(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        body: &PackageBody,
    ) -> Result<(), ConstructionError> {
        if let PackageBody::Brace { elements, .. } = body {
            for element in elements {
                self.lower_package_element(document, owner, element)?;
            }
        }
        Ok(())
    }

    pub(crate) fn lower_package_element(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        element: &Node<PackageBodyElement>,
    ) -> Result<(), ConstructionError> {
        match &element.value {
            PackageBodyElement::Error(node) => {
                self.push_recovery(document, node.span);
            }
            PackageBodyElement::Unsupported(node) => {
                self.push_unsupported(document, UnsupportedFamily::ParserUnsupported, node.span);
            }
            PackageBodyElement::Annotating(member) => {
                self.lower_annotating_member(
                    document,
                    owner,
                    UnsupportedFamily::PackageMember,
                    member,
                )?;
            }
            PackageBodyElement::Filter(node) => match owner {
                Some(declaration) => self.lower_filter_condition(
                    document,
                    declaration,
                    FilterForm::PackageImport,
                    &node.value.condition,
                )?,
                None => {
                    self.push_unsupported(document, UnsupportedFamily::PackageMember, node.span);
                }
            },
            PackageBodyElement::Package(node) => self.lower_package(document, owner, node)?,
            PackageBodyElement::LibraryPackage(node) => {
                self.lower_library_package(document, owner, node)?
            }
            PackageBodyElement::Import(node) => self.lower_import(document, owner, node)?,
            PackageBodyElement::PartDef(node) => self.lower_part_def(document, owner, node)?,
            PackageBodyElement::PartUsage(node) => self.lower_part_usage(document, owner, node)?,
            PackageBodyElement::ExtendedUsage(node) => {
                self.lower_extended_usage(document, owner, node)?
            }
            PackageBodyElement::AttributeUsage(node) => {
                self.lower_attribute_usage(document, owner, node)?
            }
            PackageBodyElement::PortDef(node) => self.lower_port_def(document, owner, node)?,
            PackageBodyElement::InterfaceDef(node) => {
                self.lower_interface_def(document, owner, node)?
            }
            PackageBodyElement::AliasDef(node) => self.lower_alias_def(document, owner, node)?,
            PackageBodyElement::AttributeDef(node) => {
                self.lower_attribute_def(document, owner, node)?
            }
            PackageBodyElement::EnumDef(node) => self.lower_enum_def(document, owner, node)?,
            PackageBodyElement::EnumerationUsage(node) => {
                self.lower_enum_usage(document, owner, node)?
            }
            PackageBodyElement::ActionDef(node) => self.lower_action_def(document, owner, node)?,
            PackageBodyElement::ActionUsage(node) => {
                self.lower_action_usage(document, owner, node)?
            }
            PackageBodyElement::RequirementDef(node) => {
                self.lower_requirement_def(document, owner, node)?
            }
            PackageBodyElement::RequirementUsage(node) => {
                self.lower_requirement_usage(document, owner, node)?
            }
            PackageBodyElement::Satisfy(node) => match owner {
                Some(owner) => {
                    self.lower_satisfy(document, owner, UnsupportedFamily::PackageMember, node)?
                }
                None => {
                    self.push_unsupported(document, UnsupportedFamily::PackageMember, node.span)
                }
            },
            PackageBodyElement::UseCaseDef(node) => {
                self.lower_use_case_def(document, owner, node)?
            }
            PackageBodyElement::Actor(node) => {
                self.push_unsupported(document, UnsupportedFamily::PackageMember, node.span)
            }
            PackageBodyElement::StateDef(node) => self.lower_state_def(document, owner, node)?,
            PackageBodyElement::StateUsage(node) => {
                self.lower_state_usage(document, owner, node)?
            }
            PackageBodyElement::ItemDef(node) => self.lower_item_def(document, owner, node)?,
            PackageBodyElement::MetadataDef(node) => {
                self.lower_metadata_def(document, owner, node)?
            }
            PackageBodyElement::IndividualDef(node) => {
                self.lower_individual_def(document, owner, node)?
            }
            PackageBodyElement::ConstraintDef(node) => {
                self.lower_constraint_def(document, owner, node)?
            }
            PackageBodyElement::ConstraintUsage(node) => {
                self.lower_constraint_usage(document, owner, node)?
            }
            PackageBodyElement::CalcDef(node) => self.lower_calc_def(document, owner, node)?,
            PackageBodyElement::CalcUsage(node) => self.lower_calc_usage(document, owner, node)?,
            PackageBodyElement::ViewDef(node) => self.lower_view_def(document, owner, node)?,
            PackageBodyElement::ViewpointDef(node) => {
                self.lower_viewpoint_def(document, owner, node)?
            }
            PackageBodyElement::RenderingDef(node) => {
                self.lower_rendering_def(document, owner, node)?
            }
            PackageBodyElement::ViewUsage(node) => self.lower_view_usage(document, owner, node)?,
            PackageBodyElement::ViewpointUsage(node) => {
                self.lower_viewpoint_usage(document, owner, node)?
            }
            PackageBodyElement::RenderingUsage(node) => {
                self.lower_rendering_usage(document, owner, node)?
            }
            PackageBodyElement::Expose(node) => match owner {
                Some(owner) => self.lower_expose(document, owner, node)?,
                None => {
                    self.push_unsupported(document, UnsupportedFamily::PackageMember, node.span)
                }
            },
            PackageBodyElement::ConnectionDef(node) => {
                self.lower_connection_def(document, owner, node)?
            }
            PackageBodyElement::OccurrenceDef(node) => {
                self.lower_occurrence_def(document, owner, node)?
            }
            PackageBodyElement::OccurrenceUsage(node) => {
                self.lower_occurrence_usage(document, owner, node)?
            }
            PackageBodyElement::Dependency(node) => {
                self.lower_dependency(document, owner, node)?;
            }
            PackageBodyElement::AllocationDef(node) => {
                self.lower_allocation_def(document, owner, node)?
            }
            PackageBodyElement::AllocationUsage(node) => {
                self.lower_allocation_usage(document, owner, node)?
            }
            PackageBodyElement::FlowDef(node) => self.lower_flow_def(document, owner, node)?,
            PackageBodyElement::FlowUsage(node) => match owner {
                Some(owner) => self.lower_flow_usage(document, owner, node)?,
                None => {
                    self.push_unsupported(document, UnsupportedFamily::PackageMember, node.span)
                }
            },
            PackageBodyElement::ConcernUsage(node) => {
                self.lower_concern_usage(document, owner, node)?
            }
            PackageBodyElement::CaseDef(node) => self.lower_case_def(document, owner, node)?,
            PackageBodyElement::CaseUsage(node) => self.lower_case_usage(document, owner, node)?,
            PackageBodyElement::AnalysisCaseDef(node) => {
                self.lower_analysis_case_def(document, owner, node)?
            }
            PackageBodyElement::AnalysisCaseUsage(node) => {
                self.lower_analysis_case_usage(document, owner, node)?
            }
            PackageBodyElement::VerificationCaseDef(node) => {
                self.lower_verification_case_def(document, owner, node)?
            }
            PackageBodyElement::VerificationCaseUsage(node) => {
                self.lower_verification_case_usage(document, owner, node)?
            }
            PackageBodyElement::UseCaseUsage(node) => {
                self.lower_use_case_usage(document, owner, node)?
            }
            PackageBodyElement::FeatureDecl(node) => {
                self.push_unsupported(document, UnsupportedFamily::PackageMember, node.span)
            }
            PackageBodyElement::ClassifierDecl(node) => {
                self.push_unsupported(document, UnsupportedFamily::PackageMember, node.span)
            }
            PackageBodyElement::KermlSemanticDecl(node) => {
                self.push_unsupported(document, UnsupportedFamily::PackageMember, node.span)
            }
            PackageBodyElement::KermlFeatureDecl(node) => {
                self.push_unsupported(document, UnsupportedFamily::PackageMember, node.span)
            }
            PackageBodyElement::KermlClassifier(node) => {
                self.lower_kerml_classifier_decl(document, owner, node)?
            }
            PackageBodyElement::KermlInvariant(node) => {
                self.lower_kerml_invariant_member(document, owner, node)?
            }
            PackageBodyElement::KermlConnector(node) => match owner {
                Some(owner) => self.lower_kerml_connector_member(document, owner, node)?,
                // A connector at the root of a document has no type to be featured by, so there
                // is no owner to source its ends at; the `connect` statement arm above defers the
                // same shape for the same reason.
                None => {
                    self.push_unsupported(document, UnsupportedFamily::PackageMember, node.span)
                }
            },
            PackageBodyElement::KermlRelationship(node) => match owner {
                Some(owner) => self.lower_kerml_relationship_decl(document, owner, node)?,
                None => {
                    self.push_unsupported(document, UnsupportedFamily::PackageMember, node.span)
                }
            },
            PackageBodyElement::KermlFeature(node) => self.lower_kerml_feature_member(
                document,
                owner,
                UnsupportedFamily::PackageMember,
                node,
            )?,
            PackageBodyElement::ExtendedLibraryDecl(node) => {
                self.push_unsupported(document, UnsupportedFamily::PackageMember, node.span)
            }
            PackageBodyElement::ItemUsage(node) => self.lower_item_usage(document, owner, node)?,
            PackageBodyElement::MetadataUsage(node) => {
                self.lower_metadata_usage(document, owner, node)?
            }
            PackageBodyElement::PortUsage(node) => self.lower_port_usage(document, owner, node)?,
            PackageBodyElement::ConnectionUsage(node) => {
                self.lower_connection_usage(document, owner, node)?
            }
            PackageBodyElement::InterfaceUsage(node) => {
                self.lower_interface_usage(document, owner, node)?
            }
            PackageBodyElement::Ref(node) => self.lower_ref_decl(document, owner, node)?,
            PackageBodyElement::MetadataKeywordUsage(node) => {
                self.push_unsupported(document, UnsupportedFamily::PackageMember, node.span)
            }
            PackageBodyElement::Connect(node) => {
                if let Some(owner) = owner {
                    self.lower_bare_connect(
                        document,
                        owner,
                        UnsupportedFamily::PackageMember,
                        node,
                    )?;
                } else {
                    self.push_unsupported(document, UnsupportedFamily::PackageMember, node.span);
                }
            }
            PackageBodyElement::DefaultReferenceUsage(node) => self.lower_default_reference_usage(
                document,
                owner,
                UnsupportedFamily::PackageMember,
                node,
            )?,
            PackageBodyElement::AssertConstraint(node) => match owner {
                Some(declaration) => self.lower_assert_constraint_member(
                    document,
                    declaration,
                    UnsupportedFamily::PackageMember,
                    node,
                )?,
                None => {
                    self.push_unsupported(document, UnsupportedFamily::PackageMember, node.span)
                }
            },
            PackageBodyElement::KermlBareDeclaration(node) => {
                self.push_unsupported(document, UnsupportedFamily::PackageMember, node.span)
            }
            PackageBodyElement::PerformUsage(node) => self.lower_perform(document, owner, node)?,
            PackageBodyElement::BindingConnectorUsage(node) => match owner {
                Some(owner) => self.lower_binding_connector_usage(
                    document,
                    owner,
                    UnsupportedFamily::PackageMember,
                    node,
                )?,
                None => {
                    self.push_unsupported(document, UnsupportedFamily::PackageMember, node.span)
                }
            },
            PackageBodyElement::Succession(node) => match owner {
                Some(owner) => {
                    self.lower_first_stmt(document, owner, UnsupportedFamily::PackageMember, node)?
                }
                None => {
                    self.push_unsupported(document, UnsupportedFamily::PackageMember, node.span)
                }
            },
            PackageBodyElement::ExhibitState(node) => {
                self.lower_exhibit_state(document, owner, UnsupportedFamily::PackageMember, node)?
            }
            PackageBodyElement::IncludeUseCase(node) => {
                self.push_unsupported(document, UnsupportedFamily::PackageMember, node.span)
            }
            PackageBodyElement::ExtendedDefinition(node) => {
                self.lower_extended_definition(document, owner, node)?
            }
        }
        Ok(())
    }

    pub(crate) fn lower_import(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<Import>,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::Import,
            None,
            node.span,
            // An import declares no name, modifier, multiplicity, or direction of its own; its
            // recursive/wildcard/filter facts belong to the authored import reference below.
            DeclarationFacts::none(),
        )?;
        let membership = &node.value.membership;
        self.push_membership(
            declaration,
            MembershipKind::Import,
            self.member_visibility(membership, ParserMembershipKind::Import)?,
            membership.span,
        )?;
        let (kind, flags) = match &node.value.target.shape {
            ImportShape::Membership { recursive_suffix } => (
                ReferenceKind::MembershipImport,
                RelationshipFlags {
                    recursive: recursive_suffix.is_some(),
                    ..RelationshipFlags::default()
                },
            ),
            ImportShape::Namespace {
                recursive_suffix, ..
            } => (
                ReferenceKind::NamespaceImport,
                RelationshipFlags {
                    recursive: recursive_suffix.is_some(),
                    wildcard: true,
                    ..RelationshipFlags::default()
                },
            ),
            ImportShape::Filter {
                recursive_suffix, ..
            } => (
                ReferenceKind::FilterImport,
                RelationshipFlags {
                    recursive: recursive_suffix.is_some(),
                    ..RelationshipFlags::default()
                },
            ),
        };
        let import = Some(AuthoredImportFacts {
            shape: match &node.value.target.shape {
                ImportShape::Membership { .. } => AuthoredImportShape::Membership,
                ImportShape::Namespace { .. } => AuthoredImportShape::Namespace,
                ImportShape::Filter { .. } => AuthoredImportShape::Filter,
            },
            recursive: flags.recursive,
        });
        self.push_reference(PendingReference {
            source: declaration,
            kind,
            document,
            local: node.value.target.reference,
            flags,
            span: node.value.target.span,
            import,
        })?;
        if let Some(elements) = &node.value.body_elements {
            self.lower_relationship_body_elements(document, Some(declaration), elements)?;
        }
        Ok(())
    }

    /// Lowers a view body's `expose <target>;` member.
    ///
    /// Mirrors [`Self::lower_import`]'s shape -- the production carries the same `ImportTarget` --
    /// minus the import facts: an expose selects what a view shows rather than bringing names into
    /// a scope, so its target is an ordinary authored reference with no import conformance.
    pub(crate) fn lower_expose(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        node: &Node<ExposeMember>,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Expose,
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
        // The target is an ordinary authored reference. What a `::*` or `::**` suffix would
        // *expand* to is not a fact this publication holds -- there is no published expose
        // expansion -- so the reference states what the author named and nothing more.
        self.push_reference(PendingReference {
            source: declaration,
            kind: ReferenceKind::ViewExpose,
            document,
            local: node.value.target.reference,
            flags: RelationshipFlags::default(),
            span: node.value.target.span,
            import: None,
        })?;
        if let sysml_v2_parser::ast::Body::Brace { elements, .. } = &node.value.body {
            self.lower_relationship_body_elements(document, Some(declaration), elements)?;
        }
        Ok(())
    }

    /// Dispatches a shared KerML `RelationshipBody`-shaped element list (BNF `RelationshipBody :
    /// Relationship = ';' | '{' (ownedRelationship += OwnedAnnotation)* '}'`, `ast::
    /// RelationshipBodyElement`), used verbatim by `Import`/`Dependency`/plain `connect`
    /// statements/`alias ... for ...` bodies: recovery nodes, the whole annotating production
    /// bound to `annotated` (`lower_annotating_member`), and an owned KerML `feature` member
    /// (`dependency z to x, y { feature e; }`, the BNF's `ownedRelatedElement`), which is lowered
    /// by the same `lower_kerml_feature_member` owner every other KerML feature member uses.
    ///
    /// `annotated` is `None` only where the construct owning the body mints no declaration of its
    /// own -- a `connect a to b { ... }` statement lowers its ends directly against the enclosing
    /// declaration -- so there is no element the annotation belongs to and attributing it to the
    /// enclosing type would misreport it.
    pub(crate) fn lower_relationship_body_elements(
        &mut self,
        document: DocumentIdx,
        annotated: Option<DeclarationId>,
        elements: &[Node<RelationshipBodyElement>],
    ) -> Result<(), ConstructionError> {
        for element in elements {
            match &element.value {
                RelationshipBodyElement::Error(error) => {
                    self.push_recovery(document, error.span);
                }
                RelationshipBodyElement::Annotating(member) => {
                    self.lower_annotating_member(
                        document,
                        annotated,
                        UnsupportedFamily::RelationshipBodyMember,
                        member,
                    )?;
                }
                RelationshipBodyElement::KermlFeature(node) => self.lower_kerml_feature_member(
                    document,
                    annotated,
                    UnsupportedFamily::RelationshipBodyMember,
                    node,
                )?,
            }
        }
        Ok(())
    }

    pub(crate) fn lower_subsetting_relationship(
        &mut self,
        document: DocumentIdx,
        source: DeclarationId,
        relationship: &Node<SubsettingRelationship>,
    ) -> Result<(), ConstructionError> {
        let kind = match relationship.value.kind {
            SubsettingKind::Subsets => ReferenceKind::Subsetting,
            SubsettingKind::References => ReferenceKind::References,
            SubsettingKind::Redefines => ReferenceKind::Redefinition,
            SubsettingKind::Crosses => ReferenceKind::Crosses,
            SubsettingKind::Intersects => ReferenceKind::Intersects,
        };
        for target in relationship.value.target.iter().copied() {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(target)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
            self.push_reference(PendingReference {
                source,
                kind,
                document,
                local: target,
                flags: RelationshipFlags {
                    implied: relationship.value.is_implied,
                    ..RelationshipFlags::default()
                },
                span,
                import: None,
            })?;
        }
        Ok(())
    }

    /// Lowers the specialization clauses of a shared `UsageDeclaration` (SysML BNF 300:
    /// `Identification? FeatureSpecializationPart?`): its `:` typing and every `:>`/`:>>`/`::>`/
    /// `crosses`/`intersects` relationship, each through the same helper the keyworded usage
    /// families use. The identification and multiplicity are declaration facts the caller records
    /// when it mints the declaration, so they are not read here.
    pub(crate) fn lower_usage_declaration_clauses(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
        usage: &sysml_v2_parser::ast::UsageDeclaration,
        variation: bool,
        direction: Option<ParameterDirection>,
    ) -> Result<(), ConstructionError> {
        if let Some(relationship) = &usage.typing {
            self.lower_typing_relationship_impl(
                document,
                declaration,
                relationship,
                variation,
                direction,
            )?;
        }
        if let Some((relationship, _)) = &usage.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        for relationship in [
            usage.redefines.as_ref(),
            usage.references.as_ref(),
            usage.crosses.as_ref(),
            usage.intersects.as_ref(),
        ]
        .into_iter()
        .flatten()
        {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        Ok(())
    }

    /// Lowers a `UsageExtensionKeyword*` run (`#Tag`, SysML BNF 296: `PrefixMetadataMember`,
    /// a `MetadataUsage` typed by the annotation) as one `MetadataAnnotation` reference per
    /// keyword, sourced at the prefixed declaration -- the same reference `@Tag`
    /// (`lower_metadata_annotation`) publishes, because both productions annotate their owner
    /// with a metadata feature typed by the named metadata definition.
    pub(crate) fn lower_usage_extension_keywords(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
        keywords: &[Node<sysml_v2_parser::ast::UsageExtensionKeyword>],
    ) -> Result<(), ConstructionError> {
        for keyword in keywords {
            let annotation = self.push_typed_declaration(
                document,
                Some(declaration),
                DeclarationKind::MetadataUsage,
                None,
                keyword.span,
                DeclarationFacts::none(),
            )?;
            self.push_membership(
                annotation,
                MembershipKind::Feature,
                Visibility::Default,
                keyword.span,
            )?;
            self.metadata_annotations.push(MetadataAnnotationRecord {
                annotation,
                annotated_element: declaration,
            });
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(keyword.value.annotation)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
            self.push_reference(PendingReference {
                source: annotation,
                kind: ReferenceKind::MetadataAnnotation,
                document,
                local: keyword.value.annotation,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        Ok(())
    }

    /// Buffers a body-less `#tag` prefix metadata keyword seen in a definition body. It binds to
    /// the declaration of the member that immediately follows it (see
    /// [`Self::bind_pending_prefix_metadata`]); the parser gives no parent link, so the binding is
    /// the walker's responsibility, exactly as it is for sibling `doc`/`comment` annotations.
    pub(crate) fn buffer_prefix_metadata_keyword(
        &mut self,
        node: &Node<sysml_v2_parser::ast::MetadataKeywordUsage>,
    ) {
        self.pending_prefix_metadata.push(PendingPrefixMetadata {
            reference: node.value.reference,
            span: node.span,
        });
    }

    /// Binds every buffered body-less `#tag` prefix metadata keyword to `declaration` -- the member
    /// they prefix -- as one `MetadataAnnotation` per keyword, the same reference `#Tag` on a usage
    /// prefix (`lower_usage_extension_keywords`) and `@Tag` (`lower_metadata_annotation`) publish.
    pub(crate) fn bind_pending_prefix_metadata(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
    ) -> Result<(), ConstructionError> {
        for entry in std::mem::take(&mut self.pending_prefix_metadata) {
            let annotation = self.push_typed_declaration(
                document,
                Some(declaration),
                DeclarationKind::MetadataUsage,
                None,
                entry.span,
                DeclarationFacts::none(),
            )?;
            self.push_membership(
                annotation,
                MembershipKind::Feature,
                Visibility::Default,
                entry.span,
            )?;
            self.metadata_annotations.push(MetadataAnnotationRecord {
                annotation,
                annotated_element: declaration,
            });
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(entry.reference)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
            self.push_reference(PendingReference {
                source: annotation,
                kind: ReferenceKind::MetadataAnnotation,
                document,
                local: entry.reference,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        Ok(())
    }

    /// Reports every buffered `#tag` prefix metadata keyword that never bound to a following member
    /// as an explicit unsupported member of `family`. Called before any non-prefixable member and
    /// at the end of a body so a dangling prefix stays visible rather than leaking onto a later
    /// declaration.
    pub(crate) fn flush_pending_prefix_metadata(
        &mut self,
        document: DocumentIdx,
        family: UnsupportedFamily,
    ) {
        for entry in std::mem::take(&mut self.pending_prefix_metadata) {
            self.push_unsupported(document, family, entry.span);
        }
    }

    /// Lowers a package-level `alias X for Y;` member into a declaration plus an authored
    /// `AliasBinding` reference for `Y`, following the Subclassification/typing lowering pattern
    /// above: `target` is already a structured `QualifiedReferenceId` (not a flattened string), so
    /// it resolves through the same lexical lookup fixed point as every other authored reference.
    pub(crate) fn lower_alias_def(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<AliasDef>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.identification.name)?;
        let short_name = self.intern_short_name(document, node.identification.short_name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::Alias,
            name,
            node.span,
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Alias,
            self.member_visibility(&node.value.membership, ParserMembershipKind::Alias)?,
            node.value.membership.span,
        )?;
        let target = node.value.target;
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span;
        self.push_reference(PendingReference {
            source: declaration,
            kind: ReferenceKind::AliasBinding,
            document,
            local: target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        if let AliasBody::Brace { elements, .. } = &node.value.body {
            self.lower_relationship_body_elements(document, Some(declaration), elements)?;
        }
        Ok(())
    }

    /// Lowers an `individual def` (BNF IndividualDef), mirroring `lower_item_def`: ownership,
    /// membership, an optional `:>` specialization relationship, and owned members via the
    /// shared `lower_attribute_body` walker (`IndividualDef.body: AttributeBody` is the same
    /// shape `ItemDef`/`ClassDef` use).
    pub(crate) fn lower_individual_def(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<sysml_v2_parser::ast::IndividualDef>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.identification.name)?;
        let short_name = self.intern_short_name(document, node.identification.short_name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::IndividualDefinition,
            name,
            node.span,
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    // `individual def` is this declaration's own form; the `individual` prefix
                    // modifier belongs to the usages and definitions that carry `is_individual`.
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

    /// Lowers a `#<keyword>+ def <Name> ...` short-form definition (BNF ExtendedDefinition),
    /// mirroring `lower_package`: ownership, membership, an optional `:>` specialization
    /// relationship, and owned members through the same `lower_package_body` walker `body:
    /// PackageBody` shares with an ordinary `package { ... }`. `ExtendedDefinition` has no
    /// `Membership` node of its own (unlike `Package`, which also lowers with a synthesized
    /// `Owning`/`Default` membership for the same reason -- see `lower_package`), so membership is
    /// synthesized identically. The `#`-prefix keyword tags and `abstract`/`variation` prefix are
    /// out of scope; see `DeclarationKind::ExtendedDefinition`'s doc comment.
    pub(crate) fn lower_extended_definition(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<ExtendedDefinition>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.identification.name)?;
        let short_name = self.intern_short_name(document, node.identification.short_name)?;
        let (is_abstract, variation) =
            definition_prefix_node_modifiers(node.value.definition_prefix.as_ref());
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ExtendedDefinition,
            name,
            node.span,
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
            Visibility::Default,
            node.span,
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_package_body(document, Some(declaration), &node.value.body)
    }

    /// Lowers a `dependency` relationship declaration (BNF Dependency), mirroring `lower_satisfy`:
    /// an anonymous (or optionally named, via `Identification`) `DeclarationKind::Dependency`
    /// feature owned by the enclosing scope, with each `client`/`supplier` operand resolved as
    /// its own authored `ReferenceKind::DependencyClient`/`DependencySupplier` reference. Unlike
    /// `AliasDef`/`Import`, `Dependency` has no `membership: Membership` field of its own, so
    /// membership is always synthesized as `MembershipKind::Feature`/`Visibility::Default` at the
    /// declaration's own span (matching `lower_satisfy`'s anonymous-relationship shape).
    /// Its `RelationshipBody` members (doc/comment/metadata only) are walked through the same
    /// `lower_relationship_body_elements` helper `AliasDef`/`Import` use.
    ///
    /// Returns the minted `DeclarationKind::Dependency` identity so a definition-body walker can
    /// bind a preceding body-less `#tag` prefix (`#refinement dependency X to Y;`) to it via
    /// [`Self::bind_pending_prefix_metadata`].
    pub(crate) fn lower_dependency(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<Dependency>,
    ) -> Result<DeclarationId, ConstructionError> {
        let name = self.intern_declaration_name(
            document,
            node.value
                .identification
                .as_ref()
                .and_then(|identification| identification.name),
        )?;
        let short_name = self.intern_short_name(
            document,
            node.value
                .identification
                .as_ref()
                .and_then(|identification| identification.short_name),
        )?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::Dependency,
            name,
            node.span,
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span,
        )?;
        for target in node.value.clients.iter().copied() {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(target)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::DependencyClient,
                document,
                local: target,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        for target in node.value.suppliers.iter().copied() {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(target)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::DependencySupplier,
                document,
                local: target,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        self.lower_relationship_body_elements(
            document,
            Some(declaration),
            node.value.body.braced_elements().unwrap_or_default(),
        )?;
        Ok(declaration)
    }

    pub(crate) fn lower_typing_relationship(
        &mut self,
        document: DocumentIdx,
        source: DeclarationId,
        relationship: &Node<sysml_v2_parser::ast::TypingRelationship>,
    ) -> Result<(), ConstructionError> {
        self.lower_typing_relationship_impl(document, source, relationship, false, None)
    }

    /// Shared implementation behind `lower_typing_relationship`, with two extra flags.
    ///
    /// `variation` is set only by `lower_part_usage` (when its prefix's variance slot is
    /// `DefinitionPrefix::Variation`), mirroring the `conjugated` flag convention on a port's
    /// typing target. `direction` is set only by `lower_kerml_feature_member`, whose node absorbed
    /// the directed kinded parameter (`in expr p : Boolean`) upstream: that declaration's typing
    /// reference has always carried its direction, so it keeps doing so now that the declaration
    /// reaches this shared path instead of pushing its own reference. Every other caller goes
    /// through the `lower_typing_relationship` wrapper above.
    pub(crate) fn lower_typing_relationship_impl(
        &mut self,
        document: DocumentIdx,
        source: DeclarationId,
        relationship: &Node<sysml_v2_parser::ast::TypingRelationship>,
        variation: bool,
        direction: Option<ParameterDirection>,
    ) -> Result<(), ConstructionError> {
        let kind = match relationship.value.kind {
            sysml_v2_parser::ast::TypingKind::Typing => ReferenceKind::FeatureTyping,
            sysml_v2_parser::ast::TypingKind::Subclassification => ReferenceKind::Subclassification,
        };
        for target in relationship.value.target.iter().copied() {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(target)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
            self.push_reference(PendingReference {
                source,
                kind,
                document,
                local: target,
                flags: RelationshipFlags {
                    conjugated: relationship.value.is_conjugated,
                    implied: relationship.value.is_implied,
                    variation,
                    direction,
                    ..RelationshipFlags::default()
                },
                span,
                import: None,
            })?;
        }
        Ok(())
    }

    /// Lowers a `variant <name>;` member (BNF `VariantUsageElement`'s untyped reference form,
    /// `ast::VariantUsage`) found inside a `variation part`/`variation part def` body, mirroring
    /// `lower_purpose_member`: the referenced sibling usage is a bare `QualifiedReferenceId` (not
    /// wrapped in an `Expression`). Per the Pilot grammar, that form introduces an anonymous
    /// `ReferenceUsage` under the `VariantMembership`; its authored `Subsetting` resolves the
    /// reference usage's target through the ordinary lexical fixed point. The typed inline
    /// form (`variant part name : Type { ... }`) introduces its concrete usage directly.
    ///
    /// Every `VariantTypedUsage` kind wraps the exact same node its ordinary spelling uses, so
    /// each delegates to the lowering that already exists for it -- there is no new lowering
    /// logic, just reuse. The `body.is_none()` guard is kept on all six: `VariantUsage.body` is a
    /// second, *outer* body that the inner node's own lowering never sees, so lowering the inner
    /// declaration while silently dropping that body would publish a partial model that looks
    /// complete. The untyped form with a body, and the case where neither `reference` nor `typed`
    /// is present, stay explicit unsupported-member diagnostics.
    ///
    /// Before delegation, the enclosing production installs a one-shot `VariantMembership` role.
    /// The ordinary usage lowering consumes it when it constructs the member's canonical
    /// Membership record, so reusing element lowering does not erase the relationship metaclass.
    pub(crate) fn lower_variant_usage(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<VariantUsage>,
    ) -> Result<(), ConstructionError> {
        let visibility = self.member_visibility(
            &node.value.membership,
            ParserMembershipKind::VariantMembership,
        )?;
        if self
            .next_membership_override
            .replace((
                MembershipKind::Owning,
                visibility,
                MembershipRole::Variant,
                node.value.membership.span,
            ))
            .is_some()
        {
            return Err(ConstructionError::InvalidMembership);
        }
        // `VariantUsageForm` makes the two authored shapes exclusive: an inline typed usage, or
        // a reference to an existing element with an optional body. A body on the reference form
        // belongs to the anonymous ReferenceUsage introduced by that form.
        match &node.value.form {
            VariantUsageForm::Typed(typed) => {
                let direct_owner = Some(owner);
                match typed {
                    VariantTypedUsage::Perform(perform) => {
                        self.lower_perform(document, direct_owner, perform.as_ref())
                    }
                    VariantTypedUsage::Part(part) => {
                        self.lower_part_usage(document, direct_owner, part.as_ref())
                    }
                    VariantTypedUsage::Attribute(attribute) => {
                        self.lower_attribute_usage(document, direct_owner, attribute.as_ref())
                    }
                    VariantTypedUsage::Item(item) => {
                        self.lower_item_usage(document, direct_owner, item.as_ref())
                    }
                    VariantTypedUsage::Port(port) => {
                        self.lower_port_usage(document, direct_owner, port.as_ref())
                    }
                    VariantTypedUsage::Action(action) => {
                        self.lower_action_usage(document, direct_owner, action.as_ref())
                    }
                    VariantTypedUsage::Requirement(requirement) => {
                        self.lower_requirement_usage(document, direct_owner, requirement.as_ref())
                    }
                }?;
                if self.next_membership_override.is_some() {
                    return Err(ConstructionError::InvalidMembership);
                }
                Ok(())
            }
            VariantUsageForm::Reference { reference, body } => {
                let declaration = self.push_typed_declaration(
                    document,
                    Some(owner),
                    DeclarationKind::ReferenceUsage,
                    None,
                    node.span,
                    DeclarationFacts::none(),
                )?;
                self.push_membership(
                    declaration,
                    MembershipKind::Owning,
                    visibility,
                    node.value.membership.span,
                )?;
                if let Some(body) = body {
                    for element in body.members() {
                        self.lower_part_usage_body_element(document, declaration, family, element)?;
                    }
                }
                let target = *reference;
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span;
                self.push_reference(PendingReference {
                    source: declaration,
                    // Pilot `VariantReference = OwnedReferenceSubsetting ...`: the anonymous
                    // ReferenceUsage subsets the referenced existing usage.
                    kind: ReferenceKind::Subsetting,
                    document,
                    local: target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
                Ok(())
            }
        }
    }

    pub(crate) fn member_visibility(
        &self,
        membership: &Membership,
        expected: ParserMembershipKind,
    ) -> Result<Visibility, ConstructionError> {
        if membership.kind != expected {
            return Err(ConstructionError::InvalidMembership);
        }
        Ok(membership
            .visibility
            .map(Self::visibility)
            .unwrap_or(Visibility::Default))
    }

    pub(crate) fn visibility(value: ParserVisibility) -> Visibility {
        match value {
            ParserVisibility::Public => Visibility::Public,
            ParserVisibility::Private => Visibility::Private,
            ParserVisibility::Protected => Visibility::Protected,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_v2_parser::ast::{QualifiedReferenceArena, RootNamespace, SourceStorage};

    fn empty_document() -> Arc<ParsedDocument> {
        Arc::new(ParsedDocument {
            source: SourceStorage::default(),
            qualified_references: QualifiedReferenceArena::default(),
            root: RootNamespace {
                elements: Vec::new(),
            },
        })
    }

    #[test]
    fn canonicalization_assigns_dense_typed_slots_and_interns_names() {
        let mut builder = SemanticModelBuilder::default();
        let parsed = empty_document();
        let document = builder
            .admit_document(
                "model",
                SourceRole::Workspace,
                ContentDigest::of_bytes(&[]),
                parsed.clone(),
                Vec::new(),
            )
            .unwrap();
        let first_name = builder.intern_name("Vehicle").unwrap();
        let second_name = builder.intern_name("Vehicle").unwrap();
        assert_eq!(first_name, second_name);
        let root = builder
            .push_declaration(document, None, Some(first_name))
            .unwrap();
        let child = builder
            .push_declaration(document, Some(root), Some(second_name))
            .unwrap();

        let (model, sources) = builder.freeze();
        assert_eq!(model.document(document).unwrap().identity.as_ref(), "model");
        // The trees leave with the parse product, not with the storage: the sealed model keeps
        // only what a settled span needs.
        assert!(Arc::ptr_eq(
            &sources.into_documents()[document.index()].parsed,
            &parsed
        ));
        assert_eq!(model.declaration(root).unwrap().owner, None);
        assert_eq!(model.declaration(child).unwrap().owner, Some(root));
        assert_eq!(model.symbol(first_name), Some("Vehicle"));
        assert_eq!(model.symbols.spans.len(), 1);
    }

    #[test]
    fn document_identity_index_rejects_duplicates_after_growth_without_mutation() {
        let parsed = empty_document();
        let mut builder = SemanticModelBuilder::default();
        for index in 0..256 {
            builder
                .admit_document(
                    format!("model-{index}"),
                    SourceRole::Workspace,
                    ContentDigest::of_bytes(&[]),
                    parsed.clone(),
                    Vec::new(),
                )
                .unwrap();
        }
        let before = builder.documents.len();

        assert_eq!(
            builder
                .admit_document(
                    "model-0",
                    SourceRole::Workspace,
                    ContentDigest::of_bytes(&[]),
                    parsed,
                    Vec::new(),
                )
                .unwrap_err(),
            ConstructionError::DuplicateDocumentIdentity
        );
        assert_eq!(builder.documents.len(), before);
    }

    #[test]
    fn anonymous_ordinals_are_owner_local_and_ignore_named_declarations() {
        let parsed = empty_document();
        let mut builder = SemanticModelBuilder::default();
        let document = builder
            .admit_document(
                "model",
                SourceRole::Workspace,
                ContentDigest::of_bytes(&[]),
                parsed,
                Vec::new(),
            )
            .unwrap();
        let owner_name = builder.intern_name("Owner").unwrap();
        let owner = builder
            .push_typed_declaration(
                document,
                None,
                DeclarationKind::Package,
                Some(owner_name),
                Span::dummy(),
                DeclarationFacts::none(),
            )
            .unwrap();
        let first = builder
            .push_typed_declaration(
                document,
                Some(owner),
                DeclarationKind::Import,
                None,
                Span::dummy(),
                DeclarationFacts::none(),
            )
            .unwrap();
        let named = builder.intern_name("Named").unwrap();
        builder
            .push_typed_declaration(
                document,
                Some(owner),
                DeclarationKind::PartUsage,
                Some(named),
                Span::dummy(),
                DeclarationFacts::none(),
            )
            .unwrap();
        let second = builder
            .push_typed_declaration(
                document,
                Some(owner),
                DeclarationKind::Import,
                None,
                Span::dummy(),
                DeclarationFacts::none(),
            )
            .unwrap();

        assert_eq!(
            builder.declarations[first.index()].anonymous_ordinal,
            Some(0)
        );
        assert_eq!(
            builder.declarations[second.index()].anonymous_ordinal,
            Some(1)
        );
    }
}

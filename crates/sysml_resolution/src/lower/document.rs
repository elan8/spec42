//! Phase 2, per document: one document's authored facts, lowered in isolation and spliced.
//!
//! Lowering is a pure function of one parsed document. Nothing in the walk reads another
//! document's declarations, symbols, or paths -- every ordinal counter it keeps is keyed by a
//! document, or by a declaration that document owns. That is what makes the product below a
//! *value*: [`lower_document`] runs the ordinary lowering walk against a builder holding only that
//! one document, so every identity it mints lands in a document-local id space starting at zero.
//!
//! [`SemanticModelBuilder::splice`] then relocates that value into the whole-build arenas. The
//! relocation is total and mechanical -- every identity-typed field of every record is named here,
//! so a new field of a lowered record does not compile until it has been given a relocation -- and
//! it visits the local symbols and paths in local id order, which is the order the walk first
//! encountered them. A build that splices document by document therefore assigns exactly the
//! identities a build that lowered document by document into one shared builder would.
//!
//! Reuse follows from that: the product is keyed by content digest alone, because content is the
//! only input the walk reads.

use std::sync::Arc;

use sysml_v2_parser::ParsedDocument;

use crate::lower::facts::AuthoredExpression;
use crate::lower::facts::AuthoredFilterCondition;
use crate::lower::facts::AuthoredInvocation;
use crate::lower::facts::AuthoredReference;
use crate::lower::facts::AuthoredUnitToken;
use crate::lower::facts::Declaration;
use crate::lower::facts::DeclarationFacts;
use crate::lower::facts::DocumentationRecord;
use crate::lower::facts::FeatureValueRecord;
use crate::lower::facts::MembershipRecord;
use crate::lower::facts::PendingEvaluationFact;
use crate::lower::facts::RecoveryRecord;
use crate::lower::facts::UnsupportedRecord;
use crate::lower::intern::SymbolPathArena;
use crate::lower::intern::SymbolTable;
use crate::lower::SemanticModelBuilder;
use crate::model::AuthoredReferenceId;
use crate::model::ConstructionError;
use crate::model::DeclarationId;
use crate::model::DocumentIdx;
use crate::model::NameId;
use crate::model::SymbolPathId;

/// One document's lowering product, in a document-local identity space.
///
/// Every `DocumentIdx` inside is [`DocumentIdx::from_index(0)`]; every `DeclarationId`,
/// `AuthoredReferenceId`, `NameId` and `SymbolPathId` indexes this value's own arenas. The value
/// is immutable and shareable: a publication authority holds it behind an `Arc` and splices the
/// same value into every later build whose document has the same content.
#[derive(Debug)]
pub(crate) struct LoweredDocument {
    pub(crate) declarations: Box<[Declaration]>,
    pub(crate) declaration_facts: Box<[DeclarationFacts]>,
    pub(crate) memberships: Box<[MembershipRecord]>,
    pub(crate) references: Box<[AuthoredReference]>,
    pub(crate) documentation: Box<[DocumentationRecord]>,
    pub(crate) feature_values: Box<[FeatureValueRecord]>,
    pub(crate) unsupported: Box<[UnsupportedRecord]>,
    pub(crate) recovery: Box<[RecoveryRecord]>,
    pub(crate) evaluation_facts: Box<[PendingEvaluationFact]>,
    pub(crate) unit_tokens: Box<[AuthoredUnitToken]>,
    pub(crate) filter_conditions: Box<[AuthoredFilterCondition]>,
    pub(crate) invocations: Box<[AuthoredInvocation]>,
    /// The names this document interned, in the order the walk first interned them.
    pub(crate) symbols: SymbolTable,
    /// The qualified paths this document interned, in the order the walk first interned them.
    pub(crate) paths: SymbolPathArena,
}

/// Lowers one parsed document on its own, yielding a relocatable product.
///
/// The walk is the ordinary one: this admits the document to a fresh builder and calls the same
/// `canonicalize_document` a whole-build lowering calls, so there is no second lowering
/// implementation to drift from the first.
pub(crate) fn lower_document(
    parsed: Arc<ParsedDocument>,
) -> Result<LoweredDocument, ConstructionError> {
    let mut builder = SemanticModelBuilder::default();
    // The identity, role, digest and parse errors of the admitted document are not read by the
    // lowering walk; they belong to the whole build, which supplies its own when it splices.
    let document = builder.admit_document(
        "",
        source_identity::SourceRole::Workspace,
        source_identity::ContentDigest::of_bytes(&[]),
        parsed,
        Vec::new(),
    )?;
    builder.canonicalize_document(document)?;
    Ok(LoweredDocument {
        declarations: builder.declarations.into_boxed_slice(),
        declaration_facts: builder.declaration_facts.into_boxed_slice(),
        memberships: builder.memberships.into_boxed_slice(),
        references: builder.references.into_boxed_slice(),
        documentation: builder.documentation.into_boxed_slice(),
        feature_values: builder.feature_values.into_boxed_slice(),
        unsupported: builder.unsupported.into_boxed_slice(),
        recovery: builder.recovery.into_boxed_slice(),
        evaluation_facts: builder.evaluation_facts.into_boxed_slice(),
        unit_tokens: builder.unit_tokens.into_boxed_slice(),
        filter_conditions: builder.filter_conditions.into_boxed_slice(),
        invocations: builder.invocations.into_boxed_slice(),
        symbols: builder.symbols.freeze(),
        paths: builder.paths.freeze(),
    })
}

/// The identity translation from one document's local space into the whole build's arenas.
struct Relocation {
    document: DocumentIdx,
    declaration_base: usize,
    reference_base: usize,
    symbols: Vec<NameId>,
    paths: Vec<SymbolPathId>,
}

impl Relocation {
    fn declaration(&self, local: DeclarationId) -> Result<DeclarationId, ConstructionError> {
        DeclarationId::from_index(
            local
                .index()
                .checked_add(self.declaration_base)
                .ok_or(ConstructionError::Capacity)?,
        )
    }

    fn reference(
        &self,
        local: AuthoredReferenceId,
    ) -> Result<AuthoredReferenceId, ConstructionError> {
        AuthoredReferenceId::from_index(
            local
                .index()
                .checked_add(self.reference_base)
                .ok_or(ConstructionError::Capacity)?,
        )
    }

    fn symbol(&self, local: NameId) -> Result<NameId, ConstructionError> {
        self.symbols
            .get(local.index())
            .copied()
            .ok_or(ConstructionError::InvalidIdentity)
    }

    fn optional_symbol(&self, local: Option<NameId>) -> Result<Option<NameId>, ConstructionError> {
        local.map(|id| self.symbol(id)).transpose()
    }

    fn path(&self, local: SymbolPathId) -> Result<SymbolPathId, ConstructionError> {
        self.paths
            .get(local.index())
            .copied()
            .ok_or(ConstructionError::InvalidIdentity)
    }

    fn expression(&self, expression: &AuthoredExpression) -> AuthoredExpression {
        AuthoredExpression {
            document: self.document,
            grammar: expression.grammar,
            operand_start: expression.operand_start,
            node: expression.node.clone(),
        }
    }
}

impl SemanticModelBuilder {
    /// Splices one document's lowering product into this build's arenas under `document`.
    ///
    /// `document` must be the id `admit_document` returned for exactly the parsed tree `lowered`
    /// was produced from; the caller owns that pairing, and the memo that keys the product by
    /// content digest is what makes it hold.
    pub(crate) fn splice(
        &mut self,
        document: DocumentIdx,
        lowered: &LoweredDocument,
    ) -> Result<(), ConstructionError> {
        let mut relocation = Relocation {
            document,
            declaration_base: self.declarations.len(),
            reference_base: self.references.len(),
            symbols: Vec::new(),
            paths: Vec::new(),
        };
        // Local id order is first-interned order, so replaying it here appends to the whole-build
        // tables in exactly the order a walk of this document would have.
        relocation
            .symbols
            .try_reserve(lowered.symbols.len())
            .map_err(|_| ConstructionError::Capacity)?;
        for index in 0..lowered.symbols.len() {
            let local = NameId::from_index(index)?;
            let text = lowered
                .symbols
                .get(local)
                .ok_or(ConstructionError::InvalidIdentity)?;
            let interned = self.symbols.intern(text)?;
            relocation.symbols.push(interned);
        }
        relocation
            .paths
            .try_reserve(lowered.paths.len())
            .map_err(|_| ConstructionError::Capacity)?;
        let mut segments = Vec::new();
        for index in 0..lowered.paths.len() {
            let local = SymbolPathId::from_index(index)?;
            let (local_segments, rooted) = lowered
                .paths
                .get(local)
                .ok_or(ConstructionError::InvalidIdentity)?;
            segments.clear();
            segments
                .try_reserve(local_segments.len())
                .map_err(|_| ConstructionError::Capacity)?;
            for segment in local_segments {
                segments.push(relocation.symbol(*segment)?);
            }
            let interned = self.paths.push(&segments, rooted)?;
            relocation.paths.push(interned);
        }

        reserve(&mut self.declarations, lowered.declarations.len())?;
        reserve(&mut self.declaration_facts, lowered.declaration_facts.len())?;
        for (declaration, facts) in lowered
            .declarations
            .iter()
            .zip(lowered.declaration_facts.iter())
        {
            self.declarations.push(Declaration {
                document,
                owner: declaration
                    .owner
                    .map(|owner| relocation.declaration(owner))
                    .transpose()?,
                name: relocation.optional_symbol(declaration.name)?,
                anonymous_ordinal: declaration.anonymous_ordinal,
                kind: declaration.kind,
                span: declaration.span,
            });
            self.declaration_facts.push(DeclarationFacts {
                short_name: relocation.optional_symbol(facts.short_name)?,
                cross_feature_projection: facts
                    .cross_feature_projection
                    .map(|projection| {
                        Ok::<_, ConstructionError>(super::facts::CrossFeatureProjection {
                            cross_feature: relocation.declaration(projection.cross_feature)?,
                            owned_cross_feature: relocation
                                .declaration(projection.owned_cross_feature)?,
                        })
                    })
                    .transpose()?,
                expression_result: facts
                    .expression_result
                    .map(|result| relocation.declaration(result))
                    .transpose()?,
                ..facts.clone()
            });
        }

        reserve(&mut self.memberships, lowered.memberships.len())?;
        for membership in lowered.memberships.iter() {
            self.memberships.push(MembershipRecord {
                member: relocation.declaration(membership.member)?,
                kind: membership.kind,
                visibility: membership.visibility,
                role: membership.role,
                span: membership.span,
            });
        }

        reserve(&mut self.references, lowered.references.len())?;
        for reference in lowered.references.iter() {
            self.references.push(AuthoredReference {
                source: relocation.declaration(reference.source)?,
                kind: reference.kind,
                target: reference.target,
                path: relocation.path(reference.path)?,
                ordinal: reference.ordinal,
                import: reference.import,
                flags: reference.flags,
                span: reference.span,
            });
        }

        reserve(&mut self.documentation, lowered.documentation.len())?;
        for record in lowered.documentation.iter() {
            self.documentation.push(DocumentationRecord {
                declaration: relocation.declaration(record.declaration)?,
                form: record.form,
                locale: relocation.optional_symbol(record.locale)?,
                language: relocation.optional_symbol(record.language)?,
                text: relocation.symbol(record.text)?,
                span: record.span,
            });
        }

        reserve(&mut self.feature_values, lowered.feature_values.len())?;
        for record in lowered.feature_values.iter() {
            self.feature_values.push(FeatureValueRecord {
                declaration: relocation.declaration(record.declaration)?,
                value: relocation.declaration(record.value)?,
                result: relocation.declaration(record.result)?,
                kind: record.kind,
                is_default: record.is_default,
                has_operator: record.has_operator,
                span: record.span,
            });
        }

        reserve(&mut self.unsupported, lowered.unsupported.len())?;
        for record in lowered.unsupported.iter() {
            self.unsupported.push(UnsupportedRecord {
                document,
                family: record.family,
                span: record.span,
            });
        }

        reserve(&mut self.recovery, lowered.recovery.len())?;
        for record in lowered.recovery.iter() {
            self.recovery.push(RecoveryRecord {
                document,
                span: record.span,
            });
        }

        reserve(&mut self.evaluation_facts, lowered.evaluation_facts.len())?;
        for fact in lowered.evaluation_facts.iter() {
            self.evaluation_facts.push(PendingEvaluationFact {
                declaration: relocation.declaration(fact.declaration)?,
                expression: relocation.expression(&fact.expression),
            });
        }

        reserve(&mut self.unit_tokens, lowered.unit_tokens.len())?;
        for token in lowered.unit_tokens.iter() {
            self.unit_tokens.push(AuthoredUnitToken {
                declaration: relocation.declaration(token.declaration)?,
                document,
                ordinal: token.ordinal,
                text: relocation.symbol(token.text)?,
                span: token.span,
            });
        }

        reserve(&mut self.filter_conditions, lowered.filter_conditions.len())?;
        for condition in lowered.filter_conditions.iter() {
            self.filter_conditions.push(AuthoredFilterCondition {
                owner: relocation.declaration(condition.owner)?,
                document,
                form: condition.form,
                span: condition.span,
                expression: relocation.expression(&condition.expression),
                predicate: condition.predicate.clone(),
            });
        }

        reserve(&mut self.invocations, lowered.invocations.len())?;
        for invocation in lowered.invocations.iter() {
            self.invocations.push(AuthoredInvocation {
                declaration: relocation.declaration(invocation.declaration)?,
                document,
                callee: relocation.reference(invocation.callee)?,
                argument_count: invocation.argument_count,
                span: invocation.span,
            });
        }

        debug_assert_eq!(self.declarations.len(), self.declaration_facts.len());
        Ok(())
    }
}

fn reserve<T>(target: &mut Vec<T>, additional: usize) -> Result<(), ConstructionError> {
    target
        .try_reserve(additional)
        .map_err(|_| ConstructionError::Capacity)
}

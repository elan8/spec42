//! Phase 2 barrier: the immutable product lowering freezes into.

use crate::lower::facts::AdmittedDocument;
use crate::lower::facts::AuthoredFilterCondition;
use crate::lower::facts::AuthoredInvocation;
use crate::lower::facts::AuthoredReference;
use crate::lower::facts::AuthoredRelationshipDeclaration;
use crate::lower::facts::AuthoredUnitToken;
use crate::lower::facts::CanonicalDocument;
use crate::lower::facts::ConstructorExpressionRecord;
use crate::lower::facts::Declaration;
use crate::lower::facts::DeclarationFacts;
use crate::lower::facts::DocumentationRecord;
use crate::lower::facts::ExpressionArgumentRecord;
use crate::lower::facts::FeatureChainExpressionRecord;
use crate::lower::facts::FeatureReferenceExpressionRecord;
use crate::lower::facts::FeatureValueRecord;
use crate::lower::facts::MembershipRecord;
use crate::lower::facts::MetadataAnnotationRecord;
use crate::lower::facts::OperatorExpressionRecord;
use crate::lower::facts::OwnedEndRecord;
use crate::lower::facts::PendingEvaluationFact;
use crate::lower::facts::RecoveryRecord;
use crate::lower::facts::UnsupportedRecord;
use crate::lower::intern::SymbolPathArena;
use crate::lower::intern::SymbolTable;
use crate::model::DeclarationId;
use crate::model::DocumentIdx;
use crate::model::NameId;
use sysml_v2_parser::{ParseError, ParsedDocument};

#[derive(Debug)]
pub(crate) struct SemanticModelStorage {
    pub(crate) documents: Box<[CanonicalDocument]>,
    pub(crate) declarations: Box<[Declaration]>,
    /// Parallel to `declarations`, one entry per `DeclarationId`.
    pub(crate) declaration_facts: Box<[DeclarationFacts]>,
    pub(crate) memberships: Box<[MembershipRecord]>,
    pub(crate) references: Box<[AuthoredReference]>,
    pub(crate) relationship_declarations: Box<[AuthoredRelationshipDeclaration]>,
    pub(crate) documentation: Box<[DocumentationRecord]>,
    pub(crate) feature_values: Box<[FeatureValueRecord]>,
    pub(crate) operator_expressions: Box<[OperatorExpressionRecord]>,
    pub(crate) expression_arguments: Box<[ExpressionArgumentRecord]>,
    pub(crate) constructor_expressions: Box<[ConstructorExpressionRecord]>,
    pub(crate) feature_chain_expressions: Box<[FeatureChainExpressionRecord]>,
    pub(crate) feature_reference_expressions: Box<[FeatureReferenceExpressionRecord]>,
    pub(crate) metadata_annotations: Box<[MetadataAnnotationRecord]>,
    pub(crate) unsupported: Box<[UnsupportedRecord]>,
    pub(crate) recovery: Box<[RecoveryRecord]>,
    pub(crate) symbols: SymbolTable,
    pub(crate) paths: SymbolPathArena,
    pub(crate) evaluation_facts: Box<[PendingEvaluationFact]>,
    pub(crate) unit_tokens: Box<[AuthoredUnitToken]>,
    pub(crate) filter_conditions: Box<[AuthoredFilterCondition]>,
    pub(crate) invocations: Box<[AuthoredInvocation]>,
    /// Every `assign` and the Features of its target parameter, in lowering order.
    pub(crate) assignments: Box<[crate::lower::facts::AssignmentRecord]>,
    /// Every `accept when|at|after` TriggerInvocationExpression, in lowering order.
    pub(crate) trigger_invocations: Box<[crate::lower::facts::TriggerInvocationRecord]>,
    /// Every evaluation site whose authored expression lowering does not fully represent as
    /// Expression elements, sorted and deduplicated.
    pub(crate) unlowered_expressions: Box<[crate::lower::facts::UnloweredExpressionSite]>,
    /// Every Type's owned end Features, grouped by owner and in authored order within an owner.
    ///
    /// The one canonical representation of KerML `Type::ownedEndFeature` (and so of the owned
    /// `connectorEnd` / `associationEnd`): every end-count, end-position and related-feature
    /// consumer reads it through [`Self::owned_end_features`]. Declared ends of every form are
    /// recorded for every owner. Bare ends are recorded for every connector whose ends lower
    /// through the KerML connector-end shape (KerML `connector` / `binding` / `succession` and
    /// `flow ... from ... to`). The SysML `connect` / `bind` / `first ... then` / transition /
    /// `allocate` forms keep their bare ends as references only, so a consumer must scope itself
    /// to the metaclasses whose collection is complete.
    pub(crate) owned_end_features: Box<[OwnedEndRecord]>,
}

/// The parse product of every admitted document, held alongside the storage until the publication
/// barrier and then dropped.
///
/// `design.md`: a sealed publication holds no parse tree. Phases that must read source text -- the
/// evaluation classifier, the parse-error projection, and the barrier that settles identifier
/// ranges -- name this value explicitly, so the set of readers is the set of places this type
/// appears, and none of them is a query.
#[derive(Debug, Default)]
pub(crate) struct ParsedSources {
    documents: Box<[AdmittedDocument]>,
}

impl ParsedSources {
    pub(crate) fn new(documents: Vec<AdmittedDocument>) -> Self {
        Self {
            documents: documents.into_boxed_slice(),
        }
    }

    pub(crate) fn parsed(&self, id: DocumentIdx) -> Option<&ParsedDocument> {
        self.documents.get(id.index()).map(|d| d.parsed.as_ref())
    }

    pub(crate) fn parse_errors(&self, id: DocumentIdx) -> &[ParseError] {
        self.documents
            .get(id.index())
            .map(|d| d.parse_errors.as_ref())
            .unwrap_or_default()
    }

    pub(crate) fn any_parse_errors(&self) -> bool {
        self.documents
            .iter()
            .any(|document| !document.parse_errors.is_empty())
    }

    pub(crate) fn into_documents(self) -> Vec<AdmittedDocument> {
        self.documents.into_vec()
    }
}

impl SemanticModelStorage {
    /// Every `(owner, expression)` ResultExpressionMembership, in lowering (authored) order.
    pub(crate) fn result_expressions(
        &self,
    ) -> impl Iterator<Item = (DeclarationId, DeclarationId)> + '_ {
        self.memberships
            .iter()
            .filter(|membership| membership.role == Some(crate::MembershipRole::ResultExpression))
            .filter_map(|membership| {
                Some((
                    self.declaration(membership.member)?.owner?,
                    membership.member,
                ))
            })
    }

    /// Each Function's or Expression's result expression, indexed by owner: the first it owns
    /// (the Pilot's `getOwnedFeatureByMembershipIn(ResultExpressionMembership)`). The owner's
    /// expression body is that element's; the owner holds none of its own.
    pub(crate) fn result_expression_of(&self) -> Box<[Option<DeclarationId>]> {
        let mut bodies = vec![None; self.declarations.len()];
        for (owner, expression) in self.result_expressions() {
            if let Some(slot) = bodies.get_mut(owner.index()) {
                slot.get_or_insert(expression);
            }
        }
        bodies.into_boxed_slice()
    }

    /// The Expression that gives each declaration its value, indexed by declaration: its first
    /// FeatureValue's value Expression, else its result expression.
    ///
    /// Evaluation is keyed to that Expression element; this is the one bridge from the valued
    /// declaration to it, so no consumer evaluates an expression at its owner or picks a
    /// different one.
    pub(crate) fn value_expressions(&self) -> Box<[Option<DeclarationId>]> {
        let mut values = self.result_expression_of();
        for value in self.feature_values.iter().rev() {
            if let Some(slot) = values.get_mut(value.declaration.index()) {
                *slot = Some(value.value);
            }
        }
        values
    }

    /// The implicit multiplicities introduced by individual-definition syntax. Their existing
    /// declaration ownership and this explicit role are the sole relationship representation.
    pub(crate) fn individual_multiplicities(&self) -> impl Iterator<Item = DeclarationId> + '_ {
        self.declaration_facts
            .iter()
            .enumerate()
            .filter_map(|(index, facts)| {
                // Construction bounds the declaration arena to the DeclarationId domain.
                facts
                    .is_individual_multiplicity
                    .then_some(DeclarationId(index as u32))
            })
    }

    /// Lowering introduces exactly one such child for each individual definition. Project that
    /// canonical role without a second parent-to-child store or a persistent derived cache.
    pub(crate) fn individual_multiplicity(&self, owner: DeclarationId) -> Option<DeclarationId> {
        if !self.declaration_facts(owner)?.modifiers.individual {
            return None;
        }
        self.individual_multiplicities()
            .find(|child| self.declarations[child.index()].owner == Some(owner))
    }

    pub(crate) fn document(&self, id: DocumentIdx) -> Option<&CanonicalDocument> {
        self.documents.get(id.index())
    }

    pub(crate) fn declaration(&self, id: DeclarationId) -> Option<&Declaration> {
        self.declarations.get(id.index())
    }

    pub(crate) fn declaration_facts(&self, id: DeclarationId) -> Option<&DeclarationFacts> {
        self.declaration_facts.get(id.index())
    }

    /// `Type::multiplicity` (KerML 8.3.3.1.10, `deriveTypeMultiplicity`): the first
    /// `Multiplicity` `owner` owns, in `ownedMember` order. A header `[m..n]` and the empty
    /// multiplicity of an `individual` definition precede every body member, so source position
    /// is that order.
    pub(crate) fn type_multiplicity(&self, owner: DeclarationId) -> Option<DeclarationId> {
        self.declarations
            .iter()
            .enumerate()
            .filter(|(_, candidate)| {
                candidate.owner == Some(owner)
                    && matches!(
                        candidate.kind,
                        crate::model::DeclarationKind::KermlMultiplicity
                            | crate::model::DeclarationKind::KermlMultiplicityRange
                    )
            })
            .min_by_key(|(index, candidate)| (candidate.span.offset, *index))
            .and_then(|(index, _)| DeclarationId::from_index(index).ok())
    }

    pub(crate) fn symbol(&self, id: NameId) -> Option<&str> {
        self.symbols.get(id)
    }

    /// `owner`'s owned end Features in authored order (KerML `Type::ownedEndFeature`).
    pub(crate) fn owned_end_features(&self, owner: DeclarationId) -> &[OwnedEndRecord] {
        let start = self
            .owned_end_features
            .partition_point(|record| record.owner < owner);
        let end = self
            .owned_end_features
            .partition_point(|record| record.owner <= owner);
        &self.owned_end_features[start..end]
    }
}

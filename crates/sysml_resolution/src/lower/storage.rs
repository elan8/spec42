//! Phase 2 barrier: the immutable product lowering freezes into.

use crate::lower::facts::AdmittedDocument;
use crate::lower::facts::AuthoredFilterCondition;
use crate::lower::facts::AuthoredInvocation;
use crate::lower::facts::AuthoredReference;
use crate::lower::facts::AuthoredUnitToken;
use crate::lower::facts::CanonicalDocument;
use crate::lower::facts::ConstructorExpressionRecord;
use crate::lower::facts::Declaration;
use crate::lower::facts::DeclarationFacts;
use crate::lower::facts::DocumentationRecord;
use crate::lower::facts::ExpressionArgumentRecord;
use crate::lower::facts::FeatureChainExpressionRecord;
use crate::lower::facts::FeatureValueRecord;
use crate::lower::facts::MembershipRecord;
use crate::lower::facts::MetadataAnnotationRecord;
use crate::lower::facts::OperatorExpressionRecord;
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
    pub(crate) documentation: Box<[DocumentationRecord]>,
    pub(crate) feature_values: Box<[FeatureValueRecord]>,
    pub(crate) operator_expressions: Box<[OperatorExpressionRecord]>,
    pub(crate) expression_arguments: Box<[ExpressionArgumentRecord]>,
    pub(crate) constructor_expressions: Box<[ConstructorExpressionRecord]>,
    pub(crate) feature_chain_expressions: Box<[FeatureChainExpressionRecord]>,
    pub(crate) metadata_annotations: Box<[MetadataAnnotationRecord]>,
    pub(crate) unsupported: Box<[UnsupportedRecord]>,
    pub(crate) recovery: Box<[RecoveryRecord]>,
    pub(crate) symbols: SymbolTable,
    pub(crate) paths: SymbolPathArena,
    pub(crate) evaluation_facts: Box<[PendingEvaluationFact]>,
    pub(crate) unit_tokens: Box<[AuthoredUnitToken]>,
    pub(crate) filter_conditions: Box<[AuthoredFilterCondition]>,
    pub(crate) invocations: Box<[AuthoredInvocation]>,
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
    pub(crate) fn document(&self, id: DocumentIdx) -> Option<&CanonicalDocument> {
        self.documents.get(id.index())
    }

    pub(crate) fn declaration(&self, id: DeclarationId) -> Option<&Declaration> {
        self.declarations.get(id.index())
    }

    pub(crate) fn declaration_facts(&self, id: DeclarationId) -> Option<&DeclarationFacts> {
        self.declaration_facts.get(id.index())
    }

    pub(crate) fn symbol(&self, id: NameId) -> Option<&str> {
        self.symbols.get(id)
    }
}
